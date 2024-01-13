use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(PartialEq, Clone)]
pub enum DataType {
    Any,
    Text,
    Integer,
    Float,
    Boolean,
    Date,
    Time,
    DateTime,
    Undefined,
    Null,
}

impl DataType {
    pub fn is_type(&self, data_type: DataType) -> bool {
        *self == data_type
    }

    pub fn is_int(&self) -> bool {
        self.is_type(DataType::Integer)
    }

    pub fn is_float(&self) -> bool {
        self.is_type(DataType::Float)
    }

    pub fn is_number(&self) -> bool {
        self.is_int() || self.is_float()
    }

    pub fn is_text(&self) -> bool {
        self.is_type(DataType::Text)
    }

    pub fn is_time(&self) -> bool {
        self.is_type(DataType::Time)
    }

    pub fn is_date(&self) -> bool {
        self.is_type(DataType::Date)
    }

    pub fn is_datetime(&self) -> bool {
        self.is_type(DataType::DateTime)
    }

    pub fn is_undefined(&self) -> bool {
        self.is_type(DataType::Undefined)
    }

    pub fn literal(&self) -> &'static str {
        match self {
            DataType::Any => "Any",
            DataType::Text => "Text",
            DataType::Integer => "Integer",
            DataType::Float => "Float",
            DataType::Boolean => "Boolean",
            DataType::Date => "Date",
            DataType::Time => "Time",
            DataType::DateTime => "DateTime",
            DataType::Undefined => "Undefined",
            DataType::Null => "Null",
        }
    }
}

lazy_static! {
    pub static ref TABLES_FIELDS_TYPES: HashMap<&'static str, DataType> = {
        let mut map = HashMap::new();
        map.insert("commit_id", DataType::Text);
        map.insert("title", DataType::Text);
        map.insert("message", DataType::Text);
        map.insert("name", DataType::Text);
        map.insert("full_name", DataType::Text);
        map.insert("insertions", DataType::Integer);
        map.insert("deletions", DataType::Integer);
        map.insert("files_changed", DataType::Integer);
        map.insert("email", DataType::Text);
        map.insert("type", DataType::Text);
        map.insert("datetime", DataType::DateTime);
        map.insert("is_head", DataType::Boolean);
        map.insert("is_remote", DataType::Boolean);
        map.insert("commit_count", DataType::Integer);
        map.insert("repo", DataType::Text);
        map
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_datatype_is_type() {
        let data_type: DataType = DataType::Text;

        let ret = data_type.is_type(DataType::Any);
        assert_eq!(ret, false);

        let ret = data_type.is_type(DataType::Text);
        assert_eq!(ret, true);
    }

    #[test]
    fn test_datatype_is_int() {
        let data_type: DataType = DataType::Any;
        let ret = data_type.is_int();
        assert_eq!(ret, false);

        let data_type: DataType = DataType::Integer;
        let ret = data_type.is_int();
        assert_eq!(ret, true);
    }

    #[test]
    fn test_datatype_is_float() {
        let data_type: DataType = DataType::Any;
        let ret = data_type.is_float();
        assert_eq!(ret, false);

        let data_type: DataType = DataType::Float;
        let ret = data_type.is_float();
        assert_eq!(ret, true);
    }

    #[test]
    fn test_datatype_is_number() {
        let data_type: DataType = DataType::Any;
        let ret = data_type.is_number();
        assert_eq!(ret, false);

        let data_type: DataType = DataType::Integer;
        let ret = data_type.is_number();
        assert_eq!(ret, true);

        let data_type: DataType = DataType::Float;
        let ret = data_type.is_number();
        assert_eq!(ret, true);
    }

    #[test]
    fn test_datatype_is_text() {
        let data_type: DataType = DataType::Any;
        let ret = data_type.is_text();
        assert_eq!(ret, false);

        let data_type: DataType = DataType::Text;
        let ret = data_type.is_text();
        assert_eq!(ret, true);
    }

    #[test]
    fn test_datatype_is_time() {
        let data_type: DataType = DataType::Any;
        let ret = data_type.is_time();
        assert_eq!(ret, false);

        let data_type: DataType = DataType::Time;
        let ret = data_type.is_time();
        assert_eq!(ret, true);
    }

    #[test]
    fn test_datatype_is_date() {
        let data_type: DataType = DataType::Any;
        let ret = data_type.is_date();
        assert_eq!(ret, false);

        let data_type: DataType = DataType::Date;
        let ret = data_type.is_date();
        assert_eq!(ret, true);
    }

    #[test]
    fn test_datatype_is_datetime() {
        let data_type: DataType = DataType::Any;
        let ret = data_type.is_datetime();
        assert_eq!(ret, false);

        let data_type: DataType = DataType::DateTime;
        let ret = data_type.is_datetime();
        assert_eq!(ret, true);
    }

    #[test]
    fn test_datatype_is_undefined() {
        let data_type: DataType = DataType::Any;
        let ret = data_type.is_undefined();
        assert_eq!(ret, false);

        let data_type: DataType = DataType::Undefined;
        let ret = data_type.is_undefined();
        assert_eq!(ret, true);
    }

    #[test]
    fn test_datatype_literal() {
        let data_type: DataType = DataType::Any;
        let ret = data_type.literal();
        assert_eq!(ret, "Any");

        let data_type: DataType = DataType::Text;
        let ret = data_type.literal();
        assert_eq!(ret, "Text");
    }
}
