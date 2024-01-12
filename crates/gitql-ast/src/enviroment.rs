use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::{types::DataType, value::Value};

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
pub struct Enviroment {
    /// All Global Variables values that can life for this program session
    pub globals: HashMap<String, Value>,
    /// All Global Variables Types that can life for this program session
    pub globals_types: HashMap<String, DataType>,
    /// Local variables types in the current scope, later will be multi layer scopes
    pub scopes: HashMap<String, DataType>,
}

impl Enviroment {
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
    fn test_enviroment_define() {
        let mut env = Enviroment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };

        let s = "field".to_string();
        let t = DataType::Integer;

        env.define(s.to_owned(), t);
        assert_eq!(env.scopes[&s.to_owned()].is_int(), true);
    }

    #[test]
    fn test_enviroment_define_global() {
        let mut env = Enviroment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };

        let s = "field".to_string();
        let t = DataType::Integer;

        env.define_global(s.to_owned(), t);
        assert_eq!(env.globals_types[&s.to_owned()].is_int(), true);
    }

    #[test]
    fn test_enviroment_contains() {
        let mut env = Enviroment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };

        let s = "field".to_string();
        let t = DataType::Integer;

        env.define(s.to_owned(), t.to_owned());
        env.define_global(s.to_owned(), t.to_owned());

        let ret = env.contains(&s.to_owned());
        assert_eq!(ret, true);

        let ret = env.contains(&"invalid".to_string());
        assert_eq!(ret, false);
    }

    #[test]
    fn test_enviroment_resolve_type() {
        let mut env = Enviroment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };

        let s = "field1".to_string();
        let t = DataType::Integer;

        env.define(s.to_owned(), t.to_owned());

        let s = "@field2".to_string();
        let t = DataType::Integer;

        env.define_global(s.to_owned(), t.to_owned());

        if let Some(ret) = env.resolve_type(&"@field2".to_string()) {
            assert_eq!(ret.is_int(), true);
        } else {
            assert!(false);
        }

        if let Some(ret) = env.resolve_type(&"field1".to_string()) {
            assert_eq!(ret.is_int(), true);
        } else {
            assert!(false);
        }

        if let None = env.resolve_type(&"@field1".to_string()) {
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_enviroment_clear_session() {
        let mut env = Enviroment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };

        let s = "field".to_string();
        let t = DataType::Integer;

        env.define(s.to_owned(), t);

        env.clear_session();
        assert_eq!(env.scopes.len(), 0);
    }
}
