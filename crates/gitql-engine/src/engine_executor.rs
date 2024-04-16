use std::cmp;
use std::cmp::Ordering;
use std::collections::HashMap;

use gitql_ast::aggregation::AGGREGATIONS;
use gitql_ast::environment::Environment;
use gitql_ast::object::GitQLObject;
use gitql_ast::object::Group;
use gitql_ast::object::Row;
use gitql_ast::statement::AggregateValue;
use gitql_ast::statement::AggregationsStatement;
use gitql_ast::statement::GlobalVariableStatement;
use gitql_ast::statement::GroupByStatement;
use gitql_ast::statement::HavingStatement;
use gitql_ast::statement::LimitStatement;
use gitql_ast::statement::OffsetStatement;
use gitql_ast::statement::OrderByStatement;
use gitql_ast::statement::SelectStatement;
use gitql_ast::statement::SortingOrder;
use gitql_ast::statement::Statement;
use gitql_ast::statement::StatementKind::*;
use gitql_ast::statement::WhereStatement;
use gitql_ast::value::Value;

use crate::engine_evaluator::evaluate_expression;
use crate::engine_function::get_column_name;
use crate::engine_function::select_gql_objects;

#[allow(clippy::borrowed_box)]
pub fn execute_statement(
    env: &mut Environment,
    statement: &Box<dyn Statement>,
    repo: &gix::Repository,
    gitql_object: &mut GitQLObject,
    alias_table: &mut HashMap<String, String>,
    hidden_selection: &Vec<String>,
) -> Result<(), String> {
    match statement.kind() {
        Select => {
            let statement = statement
                .as_any()
                .downcast_ref::<SelectStatement>()
                .unwrap();

            // Copy alias table to be last later for Aggregations functions
            for alias in &statement.alias_table {
                alias_table.insert(alias.0.to_string(), alias.1.to_string());
            }

            execute_select_statement(env, statement, repo, gitql_object, hidden_selection)
        }
        Where => {
            let statement = statement.as_any().downcast_ref::<WhereStatement>().unwrap();
            execute_where_statement(env, statement, gitql_object)
        }
        Having => {
            let statement = statement
                .as_any()
                .downcast_ref::<HavingStatement>()
                .unwrap();
            execute_having_statement(env, statement, gitql_object)
        }
        Limit => {
            let statement = statement.as_any().downcast_ref::<LimitStatement>().unwrap();
            execute_limit_statement(statement, gitql_object)
        }
        Offset => {
            let statement = statement
                .as_any()
                .downcast_ref::<OffsetStatement>()
                .unwrap();
            execute_offset_statement(statement, gitql_object)
        }
        OrderBy => {
            let statement = statement
                .as_any()
                .downcast_ref::<OrderByStatement>()
                .unwrap();
            execute_order_by_statement(env, statement, gitql_object)
        }
        GroupBy => {
            let statement = statement
                .as_any()
                .downcast_ref::<GroupByStatement>()
                .unwrap();
            execute_group_by_statement(statement, gitql_object)
        }
        AggregateFunction => {
            let statement = statement
                .as_any()
                .downcast_ref::<AggregationsStatement>()
                .unwrap();
            execute_aggregation_function_statement(env, statement, gitql_object, alias_table)
        }
        GlobalVariable => {
            let statement = statement
                .as_any()
                .downcast_ref::<GlobalVariableStatement>()
                .unwrap();
            execute_global_variable_statement(env, statement)
        }
    }
}

fn execute_select_statement(
    env: &mut Environment,
    statement: &SelectStatement,
    repo: &gix::Repository,
    gitql_object: &mut GitQLObject,
    hidden_selections: &Vec<String>,
) -> Result<(), String> {
    // Append hidden selection to the selected fields names
    let mut fields_names = statement.fields_names.to_owned();
    if !statement.table_name.is_empty() {
        for hidden in hidden_selections {
            if !fields_names.contains(hidden) {
                fields_names.push(hidden.to_string());
            }
        }
    }

    // Calculate list of titles once
    for field_name in &fields_names {
        gitql_object
            .titles
            .push(get_column_name(&statement.alias_table, field_name));
    }

    // Select objects from the target table
    let mut objects = select_gql_objects(
        env,
        repo,
        statement.table_name.to_string(),
        &fields_names,
        &gitql_object.titles,
        &statement.fields_values,
    )?;

    // Push the selected elements as a first group
    if gitql_object.is_empty() {
        gitql_object.groups.push(objects);
    } else {
        gitql_object.groups[0].rows.append(&mut objects.rows);
    }

    Ok(())
}

fn execute_where_statement(
    env: &mut Environment,
    statement: &WhereStatement,
    gitql_object: &mut GitQLObject,
) -> Result<(), String> {
    if gitql_object.is_empty() {
        return Ok(());
    }

    // Perform where command only on the first group
    // because group by command not executed yet
    let mut filtered_group: Group = Group { rows: vec![] };
    let first_group = gitql_object.groups.first().unwrap().rows.iter();
    for object in first_group {
        let eval_result = evaluate_expression(
            env,
            &statement.condition,
            &gitql_object.titles,
            &object.values,
        );
        if eval_result.is_err() {
            return Err(eval_result.err().unwrap());
        }

        if eval_result.ok().unwrap().as_bool() {
            filtered_group.rows.push(Row {
                values: object.values.clone(),
            });
        }
    }

    // Update the main group with the filtered data
    gitql_object.groups.remove(0);
    gitql_object.groups.push(filtered_group);

    Ok(())
}

fn execute_having_statement(
    env: &mut Environment,
    statement: &HavingStatement,
    gitql_object: &mut GitQLObject,
) -> Result<(), String> {
    if gitql_object.is_empty() {
        return Ok(());
    }

    if gitql_object.len() > 1 {
        gitql_object.flat()
    }

    // Perform where command only on the first group
    // because groups are already merged
    let mut filtered_group: Group = Group { rows: vec![] };
    let first_group = gitql_object.groups.first().unwrap().rows.iter();
    for object in first_group {
        let eval_result = evaluate_expression(
            env,
            &statement.condition,
            &gitql_object.titles,
            &object.values,
        );
        if eval_result.is_err() {
            return Err(eval_result.err().unwrap());
        }

        if eval_result.ok().unwrap().as_bool() {
            filtered_group.rows.push(Row {
                values: object.values.clone(),
            });
        }
    }

    // Update the main group with the filtered data
    gitql_object.groups.remove(0);
    gitql_object.groups.push(filtered_group);

    Ok(())
}

fn execute_limit_statement(
    statement: &LimitStatement,
    gitql_object: &mut GitQLObject,
) -> Result<(), String> {
    if gitql_object.is_empty() {
        return Ok(());
    }

    if gitql_object.len() > 1 {
        gitql_object.flat()
    }

    let main_group: &mut Group = &mut gitql_object.groups[0];
    if statement.count <= main_group.len() {
        main_group.rows.drain(statement.count..main_group.len());
    }

    Ok(())
}

fn execute_offset_statement(
    statement: &OffsetStatement,
    gitql_object: &mut GitQLObject,
) -> Result<(), String> {
    if gitql_object.is_empty() {
        return Ok(());
    }

    if gitql_object.len() > 1 {
        gitql_object.flat()
    }

    let main_group: &mut Group = &mut gitql_object.groups[0];
    main_group
        .rows
        .drain(0..cmp::min(statement.count, main_group.len()));

    Ok(())
}

fn execute_order_by_statement(
    env: &mut Environment,
    statement: &OrderByStatement,
    gitql_object: &mut GitQLObject,
) -> Result<(), String> {
    if gitql_object.is_empty() {
        return Ok(());
    }

    if gitql_object.len() > 1 {
        gitql_object.flat();
    }

    let main_group: &mut Group = &mut gitql_object.groups[0];
    if main_group.is_empty() {
        return Ok(());
    }

    main_group.rows.sort_by(|a, b| {
        // The default ordering
        let mut ordering = Ordering::Equal;

        for i in 0..statement.arguments.len() {
            let argument = &statement.arguments[i];
            // No need to compare if the ordering argument is constants
            if argument.is_const() {
                continue;
            }

            // Compare the two set of attributes using the current argument
            let first = &evaluate_expression(env, argument, &gitql_object.titles, &a.values)
                .unwrap_or(Value::Null);
            let other = &evaluate_expression(env, argument, &gitql_object.titles, &b.values)
                .unwrap_or(Value::Null);

            let current_ordering = first.compare(other);

            // If comparing result still equal, check the next argument
            if current_ordering == Ordering::Equal {
                continue;
            }

            // Reverse the order if its not ASC order
            ordering = if statement.sorting_orders[i] == SortingOrder::Descending {
                current_ordering
            } else {
                current_ordering.reverse()
            };
            break;
        }

        ordering
    });

    Ok(())
}

fn execute_group_by_statement(
    statement: &GroupByStatement,
    gitql_object: &mut GitQLObject,
) -> Result<(), String> {
    if gitql_object.is_empty() {
        return Ok(());
    }

    let main_group: Group = gitql_object.groups.remove(0);
    if main_group.is_empty() {
        return Ok(());
    }

    // Mapping each unique value to it group index
    let mut groups_map: HashMap<String, usize> = HashMap::new();

    // Track current group index
    let mut next_group_index = 0;

    for object in main_group.rows.into_iter() {
        let field_index = gitql_object
            .titles
            .iter()
            .position(|r| r.eq(&statement.field_name))
            .unwrap();

        let field_value = &object.values[field_index];

        // If there is an existing group for this value, append current object to it
        if let std::collections::hash_map::Entry::Vacant(e) =
            groups_map.entry(field_value.as_text())
        {
            e.insert(next_group_index);
            next_group_index += 1;
            gitql_object.groups.push(Group { rows: vec![object] });
        }
        // Push a new group for this unique value and update the next index
        else {
            let index = *groups_map.get(&field_value.as_text()).unwrap();
            let target_group = &mut gitql_object.groups[index];
            target_group.rows.push(object);
        }
    }

    Ok(())
}

fn execute_aggregation_function_statement(
    env: &mut Environment,
    statement: &AggregationsStatement,
    gitql_object: &mut GitQLObject,
    alias_table: &HashMap<String, String>,
) -> Result<(), String> {
    // Make sure you have at least one aggregation function to calculate
    let aggregations_map = &statement.aggregations;
    if aggregations_map.is_empty() {
        return Ok(());
    }

    // Used to determine if group by statement is executed before or not
    let groups_count = gitql_object.len();

    // We should run aggregation function for each group
    for group in &mut gitql_object.groups {
        // No need to apply all aggregation if there is no selected elements
        if group.is_empty() {
            continue;
        }

        // Resolve all aggregations functions first
        for aggregation in aggregations_map {
            if let AggregateValue::Function(function, argument) = aggregation.1 {
                // Get alias name if exists or column name by default

                let result_column_name = aggregation.0;
                let column_name = get_column_name(alias_table, result_column_name);

                let column_index = gitql_object
                    .titles
                    .iter()
                    .position(|r| r.eq(&column_name))
                    .unwrap();

                // Get the target aggregation function
                let aggregation_function = AGGREGATIONS.get(function.as_str()).unwrap();
                let result =
                    &aggregation_function(&argument.to_string(), &gitql_object.titles, group);

                // Insert the calculated value in the group objects
                for object in &mut group.rows {
                    if column_index < object.values.len() {
                        object.values[column_index] = result.clone();
                    } else {
                        object.values.push(result.clone());
                    }
                }
            }
        }

        // Resolve aggregations expressions
        for aggregation in aggregations_map {
            if let AggregateValue::Expression(expr) = aggregation.1 {
                // Get alias name if exists or column name by default
                let result_column_name = aggregation.0;
                let column_name = get_column_name(alias_table, result_column_name);

                let column_index = gitql_object
                    .titles
                    .iter()
                    .position(|r| r.eq(&column_name))
                    .unwrap();

                // Insert the calculated value in the group objects
                for object in group.rows.iter_mut() {
                    let result =
                        evaluate_expression(env, expr, &gitql_object.titles, &object.values)?;
                    if column_index < object.values.len() {
                        object.values[column_index] = result.clone();
                    } else {
                        object.values.push(result.clone());
                    }
                }
            }
        }

        // In case of group by statement is executed
        // Remove all elements expect the first one
        if groups_count > 1 {
            group.rows.drain(1..);
        }
    }

    Ok(())
}

pub fn execute_global_variable_statement(
    env: &mut Environment,
    statement: &GlobalVariableStatement,
) -> Result<(), String> {
    let value = evaluate_expression(env, &statement.value, &[], &vec![])?;
    env.globals.insert(statement.name.to_string(), value);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use gitql_ast::expression::NumberExpression;

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
    fn test_execute_statement() {
        let mut env = Environment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };

        let statement: Box<dyn Statement> = Box::new(SelectStatement {
            table_name: "commits".to_string(),
            fields_names: vec![
                "commit_id".to_string(),
                "title".to_string(),
                "message".to_string(),
                "name".to_string(),
                "email".to_string(),
                "datetime".to_string(),
                "repo".to_string(),
            ],
            fields_values: vec![],
            alias_table: Default::default(),
            is_distinct: false,
        });

        let path = "test-execute-statement";
        test_new_repo(path.to_string()).expect("failed to new repo");

        let buf = gix::open(path);
        let repo = buf.ok().unwrap();

        let mut object = GitQLObject::default();
        let mut table: HashMap<String, String> = HashMap::new();
        let selection = vec!["".to_string()];

        let ret = execute_statement(
            &mut env,
            &statement,
            &repo,
            &mut object,
            &mut table,
            &selection,
        );
        if ret.is_ok() {
            assert!(true);
        } else {
            assert!(false);
        }

        test_delete_repo(path.to_string()).expect("failed to delete repo");
    }

    #[test]
    fn test_execute_select_statement() {
        let mut env = Environment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };

        let statement = SelectStatement {
            table_name: "commits".to_string(),
            fields_names: vec![
                "commit_id".to_string(),
                "title".to_string(),
                "message".to_string(),
                "name".to_string(),
                "email".to_string(),
                "datetime".to_string(),
                "repo".to_string(),
            ],
            fields_values: vec![],
            alias_table: Default::default(),
            is_distinct: false,
        };

        let path = "test-execute-select-statement";
        test_new_repo(path.to_string()).expect("failed to new repo");

        let buf = gix::open(path);
        let repo = buf.ok().unwrap();

        let mut object = GitQLObject::default();
        let selections = vec!["".to_string()];

        let ret = execute_select_statement(&mut env, &statement, &repo, &mut object, &selections);
        if ret.is_ok() {
            assert!(true);
        } else {
            assert!(false);
        }

        test_delete_repo(path.to_string()).expect("failed to delete repo");
    }

    #[test]
    fn test_execute_where_statement() {
        let mut env = Environment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };

        let statement = WhereStatement {
            condition: Box::new(NumberExpression {
                value: Value::Integer(1),
            }),
        };

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

        let ret = execute_where_statement(&mut env, &statement, &mut object);
        if ret.is_ok() {
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_execute_having_statement() {
        let mut env = Environment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };

        let statement = HavingStatement {
            condition: Box::new(NumberExpression {
                value: Value::Integer(1),
            }),
        };

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

        let ret = execute_having_statement(&mut env, &statement, &mut object);
        if ret.is_ok() {
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_execute_limit_statement() {
        let statement = LimitStatement { count: 0 };

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

        let ret = execute_limit_statement(&statement, &mut object);
        if ret.is_ok() {
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_execute_offset_statement() {
        let statement = OffsetStatement { count: 0 };

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

        let ret = execute_offset_statement(&statement, &mut object);
        if ret.is_ok() {
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_execute_order_by_statement() {
        let mut env = Environment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };

        let statement = OrderByStatement {
            arguments: vec![Box::new(NumberExpression {
                value: Value::Integer(5),
            })],
            sorting_orders: vec![SortingOrder::Ascending],
        };

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

        let ret = execute_order_by_statement(&mut env, &statement, &mut object);
        if ret.is_ok() {
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_execute_group_by_statement() {
        let statement = GroupByStatement {
            field_name: "title1".to_string(),
        };

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

        let ret = execute_group_by_statement(&statement, &mut object);
        if ret.is_ok() {
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_execute_aggregation_function_statement() {
        let mut env = Environment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };

        let mut statement = AggregationsStatement {
            aggregations: Default::default(),
        };

        statement.aggregations.insert(
            "title".to_string(),
            AggregateValue::Function("max".to_string(), "title1".to_string()),
        );
        statement.aggregations.insert(
            "title".to_string(),
            AggregateValue::Expression(Box::new(NumberExpression {
                value: Value::Integer(5),
            })),
        );

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

        let mut table: HashMap<String, String> = HashMap::new();
        table.insert("title".to_string(), "title1".to_string());

        let ret = execute_aggregation_function_statement(&mut env, &statement, &mut object, &table);
        if ret.is_ok() {
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_execute_global_variable_statement() {
        let mut env = Environment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };

        let statement = GlobalVariableStatement {
            name: "name".to_string(),
            value: Box::new(NumberExpression {
                value: Value::Integer(1),
            }),
        };

        let ret = execute_global_variable_statement(&mut env, &statement);
        if ret.is_ok() {
            assert!(true);
        } else {
            assert!(false);
        }
    }
}
