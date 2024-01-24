use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::types::DataType;
use crate::value::Value;

lazy_static! {
    pub static ref TABLES_FIELDS_NAMES: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("refs", vec!["name", "full_name", "type", "repo"]);
        map.insert(
            "commits",
            vec![
                "commit_id",
                "title",
                "message",
                "name",
                "email",
                "datetime",
                "repo",
            ],
        );
        map.insert(
            "branches",
            vec!["name", "commit_count", "is_head", "is_remote", "repo"],
        );
        map.insert(
            "diffs",
            vec![
                "commit_id",
                "name",
                "email",
                "insertions",
                "deletions",
                "files_changed",
                "repo",
            ],
        );
        map.insert("tags", vec!["name", "repo"]);
        map
    };
}

#[derive(Default)]
pub struct Environment {
    /// All Global Variables values that can life for this program session
    pub globals: HashMap<String, Value>,
    /// All Global Variables Types that can life for this program session
    pub globals_types: HashMap<String, DataType>,
    /// Local variables types in the current scope, later will be multi layer scopes
    pub scopes: HashMap<String, DataType>,
}

impl Environment {
    /// Define in the current scope
    pub fn define(&mut self, str: String, data_type: DataType) {
        self.scopes.insert(str, data_type);
    }

    /// Define in the global scope
    pub fn define_global(&mut self, str: String, data_type: DataType) {
        self.globals_types.insert(str, data_type);
    }

    /// Returns true if local or global scopes has contains field
    pub fn contains(&self, str: &String) -> bool {
        self.scopes.contains_key(str) || self.globals_types.contains_key(str)
    }

    /// Resolve Global or Local type using symbol name
    pub fn resolve_type(&self, str: &String) -> Option<&DataType> {
        if str.starts_with('@') {
            return self.globals_types.get(str);
        }
        return self.scopes.get(str);
    }

    /// Clear all locals scopes and only save globals
    pub fn clear_session(&mut self) {
        self.scopes.clear()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_define() {
        let mut env = Environment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };

        env.define("field1".to_string(), DataType::Text);
        if env.scopes["field1"] == DataType::Text {
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_define_global() {
        let mut env = Environment{
            globals: Default::default (),
            globals_types: Default::default (),
            scopes: Default::default (),
        };

        env.define_global("field1".to_string(), DataType::Text);
        if env.globals_types["field1"] == DataType::Text {
            assert!(true);
        } else {
            assert!(false);
        }
}

    #[test]
    fn test_contains() {
        let mut env = Environment{
            globals: Default::default (),
            globals_types: Default::default (),
            scopes: Default::default (),
        };

        env.define("field1".to_string(), DataType::Text);
        env.define_global("field2".to_string(), DataType::Integer);

        let ret = env.contains(&"field1".to_string());
        assert_eq!(ret, true);

        let ret = env.contains(&"field2".to_string());
        assert_eq!(ret, true);

        let ret = env.contains(&"invalid".to_string());
        assert_eq!(ret, false);
    }

    #[test]
    fn test_resolve_type() {
        let mut env = Environment{
            globals: Default::default (),
            globals_types: Default::default (),
            scopes: Default::default (),
        };

        env.define("field1".to_string(), DataType::Text);
        env.define_global("@field2".to_string(), DataType::Integer);

        if let Some(v) = env.resolve_type(&"field1".to_string()) {
            if *v == DataType::Text {
                assert!(true);
            } else {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        if let Some(v) = env.resolve_type(&"@field2".to_string()) {
            if *v == DataType::Integer {
                assert!(true);
            } else {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        if let Some(_) = env.resolve_type(&"invalid".to_string()) {
            assert!(false);
        } else {
            assert!(true);
        }
    }

    #[test]
    fn test_clear_session() {
        let mut env = Environment{
            globals: Default::default (),
            globals_types: Default::default (),
            scopes: Default::default (),
        };

        env.define("field1".to_string(), DataType::Text);

        env.clear_session();
        assert_eq!(env.scopes.len(), 0);
    }
}
