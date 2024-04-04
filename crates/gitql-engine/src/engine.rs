use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::hash::Hasher;
use std::vec;

use gitql_ast::environment::Environment;
use gitql_ast::object::GitQLObject;
use gitql_ast::object::Group;
use gitql_ast::object::Row;
use gitql_ast::statement::GQLQuery;
use gitql_ast::statement::Query;
use gitql_ast::statement::SelectStatement;

use crate::engine_executor::execute_global_variable_statement;
use crate::engine_executor::execute_statement;

const GQL_COMMANDS_IN_ORDER: [&str; 8] = [
    "select",
    "where",
    "group",
    "aggregation",
    "having",
    "order",
    "offset",
    "limit",
];

pub enum EvaluationResult {
    SelectedGroups(GitQLObject, Vec<std::string::String>),
    SetGlobalVariable,
}

pub fn evaluate(
    env: &mut Environment,
    repos: &[gix::Repository],
    query: Query,
) -> Result<EvaluationResult, String> {
    match query {
        Query::Select(gql_query) => evaluate_select_query(env, repos, gql_query),
        Query::GlobalVariableDeclaration(global_variable) => {
            execute_global_variable_statement(env, &global_variable)?;
            Ok(EvaluationResult::SetGlobalVariable)
        }
    }
}

pub fn evaluate_select_query(
    env: &mut Environment,
    repos: &[gix::Repository],
    query: GQLQuery,
) -> Result<EvaluationResult, String> {
    let mut gitql_object = GitQLObject::default();
    let mut alias_table: HashMap<String, String> = HashMap::new();

    let hidden_selections = query.hidden_selections;
    let mut statements_map = query.statements;
    let first_repo = repos.first().unwrap();

    for gql_command in GQL_COMMANDS_IN_ORDER {
        if statements_map.contains_key(gql_command) {
            let statement = statements_map.get_mut(gql_command).unwrap();

            match gql_command {
                "select" => {
                    // Select statement should be performed on all repositories, can be executed in parallel
                    let select_statement = statement
                        .as_any()
                        .downcast_ref::<SelectStatement>()
                        .unwrap();

                    // If table name is empty no need to perform it on each repository
                    if select_statement.table_name.is_empty() {
                        execute_statement(
                            env,
                            statement,
                            &repos[0],
                            &mut gitql_object,
                            &mut alias_table,
                            &hidden_selections,
                        )?;

                        // If the main group is empty, no need to perform other statements
                        if gitql_object.is_empty() || gitql_object.groups[0].is_empty() {
                            return Ok(EvaluationResult::SelectedGroups(
                                gitql_object,
                                hidden_selections,
                            ));
                        }

                        continue;
                    }

                    // If table name is not empty, must perform it on each repository
                    for repo in repos {
                        execute_statement(
                            env,
                            statement,
                            repo,
                            &mut gitql_object,
                            &mut alias_table,
                            &hidden_selections,
                        )?;
                    }

                    // If the main group is empty, no need to perform other statements
                    if gitql_object.is_empty() || gitql_object.groups[0].is_empty() {
                        return Ok(EvaluationResult::SelectedGroups(
                            gitql_object,
                            hidden_selections,
                        ));
                    }

                    // If Select statement has table name and distinct flag, keep only unique values
                    if !select_statement.table_name.is_empty() && select_statement.is_distinct {
                        apply_distinct_on_objects_group(&mut gitql_object, &hidden_selections);
                    }
                }
                _ => {
                    // Any other statement can be performed on first or non repository
                    execute_statement(
                        env,
                        statement,
                        first_repo,
                        &mut gitql_object,
                        &mut alias_table,
                        &hidden_selections,
                    )?;
                }
            }
        }
    }

    // If there are many groups that mean group by is executed before.
    // must merge each group into only one element
    if gitql_object.len() > 1 {
        for group in gitql_object.groups.iter_mut() {
            if group.len() > 1 {
                group.rows.drain(1..);
            }
        }
    }
    // If it a single group but it select only aggregations function,
    // should return only first element in the group
    else if gitql_object.len() == 1
        && !query.has_group_by_statement
        && query.has_aggregation_function
    {
        let group: &mut Group = &mut gitql_object.groups[0];
        if group.len() > 1 {
            group.rows.drain(1..);
        }
    }

    // Return the groups and hidden selections to be used later in GUI or TUI ...etc
    Ok(EvaluationResult::SelectedGroups(
        gitql_object,
        hidden_selections,
    ))
}

fn apply_distinct_on_objects_group(gitql_object: &mut GitQLObject, hidden_selections: &[String]) {
    if gitql_object.is_empty() {
        return;
    }

    let titles: Vec<&String> = gitql_object
        .titles
        .iter()
        .filter(|s| !hidden_selections.contains(s))
        .collect();

    let titles_count = titles.len();

    let objects = &gitql_object.groups[0].rows;
    let mut new_objects: Group = Group { rows: vec![] };
    let mut values_set: HashSet<u64> = HashSet::new();

    for object in objects {
        // Build row of the selected only values
        let mut row_values: Vec<String> = Vec::with_capacity(titles_count);
        for index in 0..titles.len() {
            row_values.push(object.values.get(index).unwrap().to_string());
        }

        // Compute the hash for row of values
        let mut hash = DefaultHasher::new();
        row_values.hash(&mut hash);
        let values_hash = hash.finish();

        // If this hash is unique, insert the row
        if values_set.insert(values_hash) {
            new_objects.rows.push(Row {
                values: object.values.clone(),
            });
        }
    }

    // If number of total rows is changed, update the main group rows
    if objects.len() != new_objects.len() {
        gitql_object.groups[0].rows.clear();
        gitql_object.groups[0].rows.append(&mut new_objects.rows);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gitql_ast::value::Value;
    use gitql_parser::{parser, tokenizer};

    fn test_new_repo(path: String) -> Result<(), String> {
        let mut repo = gix::init_bare(path).expect("failed to init bare");
        let mut tree = gix::objs::Tree::empty();
        let object = repo
            .write_object(&tree)
            .expect("failed to write object")
            .detach();

        let mut config = repo.config_snapshot_mut();
        config
            .set_raw_value("author", None, "name", "name")
            .expect("failed to set name");
        config
            .set_raw_value("author", None, "email", "name@example.com")
            .expect("failed to set email");

        let repo = config
            .commit_auto_rollback()
            .expect("failed to commit auto rollback");
        let commit = repo
            .commit("HEAD", "initial commit", object, gix::commit::NO_PARENT_IDS)
            .expect("failed to commit");

        let blob = repo
            .write_blob("hello world")
            .expect("faile to write blob")
            .into();
        let entry = gix::objs::tree::Entry {
            mode: gix::objs::tree::EntryKind::Blob.into(),
            oid: blob,
            filename: "hello.txt".into(),
        };

        tree.entries.push(entry);
        let object = repo.write_object(&tree).expect("failed to write object");

        let _ = repo
            .commit("HEAD", "hello commit", object, [commit])
            .expect("failed to commit");

        Ok(())
    }

    fn test_delete_repo(path: String) -> Result<(), String> {
        std::fs::remove_dir_all(path).expect("failed to remove dir");
        Ok(())
    }

    #[test]
    fn test_evaluate() {
        let mut env = Environment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };

        let path = "test-evaluate";
        test_new_repo(path.to_string()).expect("failed to new repo");

        let buf = gix::open(path);
        let repos = &vec![buf.ok().unwrap()];

        let query = "SELECT * FROM commits";
        let result = tokenizer::tokenize(query.to_string());
        let tokens = result.ok().unwrap();
        let result = parser::parse_gql(tokens, &mut env);
        let query = result.ok().unwrap();

        let ret = evaluate(&mut env, &repos, query);
        if ret.is_err() {
            test_delete_repo(path.to_string()).expect("failed to delete repo");
            assert!(false);
        }

        let query = "SET @STRING = \"GitQL\"";
        let result = tokenizer::tokenize(query.to_string());
        let tokens = result.ok().unwrap();
        let result = parser::parse_gql(tokens, &mut env);
        let query = result.ok().unwrap();

        let ret = evaluate(&mut env, &repos, query);
        if ret.is_err() {
            test_delete_repo(path.to_string()).expect("failed to delete repo");
            assert!(false);
        }

        test_delete_repo(path.to_string()).expect("failed to delete repo");
    }

    #[test]
    fn test_evaluate_select_query() {
        let mut env = Environment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };

        let path = "test-evaluate-select-query";
        test_new_repo(path.to_string()).expect("failed to new repo");

        let buf = gix::open(path);
        let repos = &vec![buf.ok().unwrap()];

        let query = "SELECT * FROM commits";
        let result = tokenizer::tokenize(query.to_string());
        let tokens = result.ok().unwrap();
        let result = parser::parse_gql(tokens, &mut env);
        let query = result.ok().unwrap();

        match query {
            Query::Select(q) => {
                let ret = evaluate_select_query(&mut env, &repos, q);
                if ret.is_err() {
                    test_delete_repo(path.to_string()).expect("failed to delete repo");
                    assert!(false);
                }
            }
            _ => {
                test_delete_repo(path.to_string()).expect("failed to delete repo");
                assert!(false);
            }
        };

        test_delete_repo(path.to_string()).expect("failed to delete repo");
    }

    #[test]
    fn test_apply_distinct_on_objects_group() {
        let mut object = GitQLObject {
            titles: vec!["title1".to_string(), "title2".to_string()],
            groups: vec![Group {
                rows: vec![
                    Row {
                        values: vec![Value::Integer(1), Value::Integer(2)],
                    },
                    Row {
                        values: vec![Value::Integer(3), Value::Integer(4)],
                    },
                ],
            }],
        };

        let selections = vec!["".to_string()];

        apply_distinct_on_objects_group(&mut object, &selections);
        assert_eq!(object.groups[0].rows.len(), 2);

        let mut object = GitQLObject {
            titles: vec!["title1".to_string(), "title2".to_string()],
            groups: vec![Group {
                rows: vec![
                    Row {
                        values: vec![Value::Integer(1), Value::Integer(2)],
                    },
                    Row {
                        values: vec![Value::Integer(1), Value::Integer(2)],
                    },
                ],
            }],
        };

        let selections = vec!["".to_string()];

        apply_distinct_on_objects_group(&mut object, &selections);
        assert_eq!(object.groups[0].rows.len(), 1);
    }
}
