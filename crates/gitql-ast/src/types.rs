use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt;

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

/// Represent the data types for values to be used in type checker
#[derive(Clone)]
pub enum DataType {
    /// Represent general type so can be equal to any other type
    Any,
    /// Represent String Type
    Text,
    /// Represent Integer 64 bit type
    Integer,
    /// Represent Float 64 bit type
    Float,
    /// Represent Boolean (true | false) type
    Boolean,
    /// Represent Date type
    Date,
    /// Represent Time type
    Time,
    /// Represent Date with Time type
    DateTime,
    /// Represent `Undefined` value
    Undefined,
    /// Represent `NULL` value
    Null,
    /// Represent a set of valid variant of types
    Variant(Vec<DataType>),
    /// Represent an optional type so it can passed or not, must be last parameter
    Optional(Box<DataType>),
    /// Represent variable arguments so can pass 0 or more value with spastic type, must be last parameter
    Varargs(Box<DataType>),
}

impl PartialEq for DataType {
    fn eq(&self, other: &Self) -> bool {
        if self.is_any() || other.is_any() {
            return true;
        }

        if let DataType::Variant(types) = self {
            for data_type in types {
                if data_type == other {
                    return true;
                }
            }
            return false;
        }

        if let DataType::Variant(types) = other {
            for data_type in types {
                if data_type == self {
                    return true;
                }
            }
            return false;
        }

        if let DataType::Optional(optional_type) = self {
            return optional_type.as_ref() == other;
        }

        if let DataType::Optional(optional_type) = other {
            return optional_type.as_ref() == self;
        }

        if let DataType::Varargs(data_type) = self {
            return data_type.as_ref() == other;
        }

        if let DataType::Varargs(data_type) = other {
            return data_type.as_ref() == self;
        }

        if self.is_bool() && other.is_bool() {
            return true;
        }

        if self.is_int() && other.is_int() {
            return true;
        }

        if self.is_float() && other.is_float() {
            return true;
        }

        if self.is_number() && other.is_number() {
            return true;
        }

        if self.is_text() && other.is_text() {
            return true;
        }

        if self.is_date() && other.is_date() {
            return true;
        }

        if self.is_time() && other.is_time() {
            return true;
        }

        if self.is_datetime() && other.is_datetime() {
            return true;
        }

        if self.is_null() && other.is_null() {
            return true;
        }

        if self.is_undefined() && other.is_undefined() {
            return true;
        }

        false
    }
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DataType::Any => write!(f, "Any"),
            DataType::Text => write!(f, "Text"),
            DataType::Integer => write!(f, "Integer"),
            DataType::Float => write!(f, "Float"),
            DataType::Boolean => write!(f, "Boolean"),
            DataType::Date => write!(f, "Date"),
            DataType::Time => write!(f, "Time"),
            DataType::DateTime => write!(f, "DateTime"),
            DataType::Undefined => write!(f, "Undefined"),
            DataType::Null => write!(f, "Null"),
            DataType::Variant(types) => {
                write!(f, "[")?;
                for (pos, data_type) in types.iter().enumerate() {
                    write!(f, "{}", data_type)?;
                    if pos != types.len() - 1 {
                        write!(f, " | ")?;
                    }
                }
                write!(f, "]")
            }
            DataType::Optional(data_type) => {
                write!(f, "{}?", data_type)
            }
            DataType::Varargs(data_type) => {
                write!(f, "...{}", data_type)
            }
        }
    }
}

impl DataType {
    pub fn is_any(&self) -> bool {
        matches!(self, DataType::Any)
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, DataType::Boolean)
    }

    pub fn is_int(&self) -> bool {
        matches!(self, DataType::Integer)
    }

    pub fn is_float(&self) -> bool {
        matches!(self, DataType::Float)
    }

    pub fn is_number(&self) -> bool {
        self.is_int() || self.is_float()
    }

    pub fn is_text(&self) -> bool {
        matches!(self, DataType::Text)
    }

    pub fn is_time(&self) -> bool {
        matches!(self, DataType::Time)
    }

    pub fn is_date(&self) -> bool {
        matches!(self, DataType::Date)
    }

    pub fn is_datetime(&self) -> bool {
        matches!(self, DataType::DateTime)
    }

    pub fn is_null(&self) -> bool {
        matches!(self, DataType::Null)
    }

    pub fn is_undefined(&self) -> bool {
        matches!(self, DataType::Undefined)
    }

    pub fn is_variant(&self) -> bool {
        matches!(self, DataType::Variant(_))
    }

    pub fn is_optional(&self) -> bool {
        matches!(self, DataType::Optional(_))
    }

    pub fn is_varargs(&self) -> bool {
        matches!(self, DataType::Varargs(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partialeq_eq() {
        let partialeq = DataType::Any;
        let other = DataType::Any;

        let ret = partialeq.eq(&other);
        assert_eq!(ret, true);

        let partialeq = DataType::Variant(vec![DataType::Text, DataType::Integer]);
        let other = DataType::Text;

        let ret = partialeq.eq(&other);
        assert_eq!(ret, true);

        let partialeq = DataType::Text;
        let other = DataType::Variant(vec![DataType::Text, DataType::Integer]);

        let ret = partialeq.eq(&other);
        assert_eq!(ret, true);

        let partialeq = DataType::Optional(Box::new(DataType::Text));
        let other = DataType::Text;

        let ret = partialeq.eq(&other);
        assert_eq!(ret, true);

        let partialeq = DataType::Text;
        let other = DataType::Optional(Box::new(DataType::Text));

        let ret = partialeq.eq(&other);
        assert_eq!(ret, true);

        let partialeq = DataType::Varargs(Box::new(DataType::Text));
        let other = DataType::Text;

        let ret = partialeq.eq(&other);
        assert_eq!(ret, true);

        let partialeq = DataType::Text;
        let other = DataType::Varargs(Box::new(DataType::Text));

        let ret = partialeq.eq(&other);
        assert_eq!(ret, true);

        let partialeq = DataType::Boolean;
        let other = DataType::Boolean;

        let ret = partialeq.eq(&other);
        assert_eq!(ret, true);

        let partialeq = DataType::Integer;
        let other = DataType::Integer;

        let ret = partialeq.eq(&other);
        assert_eq!(ret, true);

        let partialeq = DataType::Float;
        let other = DataType::Float;

        let ret = partialeq.eq(&other);
        assert_eq!(ret, true);

        let partialeq = DataType::Integer;
        let other = DataType::Integer;

        let ret = partialeq.eq(&other);
        assert_eq!(ret, true);

        let partialeq = DataType::Text;
        let other = DataType::Text;

        let ret = partialeq.eq(&other);
        assert_eq!(ret, true);

        let partialeq = DataType::Date;
        let other = DataType::Date;

        let ret = partialeq.eq(&other);
        assert_eq!(ret, true);

        let partialeq = DataType::Time;
        let other = DataType::Time;

        let ret = partialeq.eq(&other);
        assert_eq!(ret, true);

        let partialeq = DataType::DateTime;
        let other = DataType::DateTime;

        let ret = partialeq.eq(&other);
        assert_eq!(ret, true);

        let partialeq = DataType::Null;
        let other = DataType::Null;

        let ret = partialeq.eq(&other);
        assert_eq!(ret, true);

        let partialeq = DataType::Undefined;
        let other = DataType::Undefined;

        let ret = partialeq.eq(&other);
        assert_eq!(ret, true);
    }

    #[test]
    fn test_datatype_fmt() {
        let dtype = DataType::Any;
        assert_eq!(format!("{}", dtype), "Any");

        let dtype = DataType::Text;
        assert_eq!(format!("{}", dtype), "Text");

        let dtype = DataType::Integer;
        assert_eq!(format!("{}", dtype), "Integer");

        let dtype = DataType::Float;
        assert_eq!(format!("{}", dtype), "Float");

        let dtype = DataType::Boolean;
        assert_eq!(format!("{}", dtype), "Boolean");

        let dtype = DataType::Date;
        assert_eq!(format!("{}", dtype), "Date");

        let dtype = DataType::Time;
        assert_eq!(format!("{}", dtype), "Time");

        let dtype = DataType::DateTime;
        assert_eq!(format!("{}", dtype), "DateTime");

        let dtype = DataType::Undefined;
        assert_eq!(format!("{}", dtype), "Undefined");

        let dtype = DataType::Null;
        assert_eq!(format!("{}", dtype), "Null");

        let dtype = DataType::Variant(vec![DataType::Text, DataType::Integer]);
        assert_eq!(format!("{}", dtype), "[Text | Integer]");

        let dtype = DataType::Optional(Box::new(DataType::Text));
        assert_eq!(format!("{}", dtype), "Text?");

        let dtype = DataType::Varargs(Box::new(DataType::Text));
        assert_eq!(format!("{}", dtype), "...Text");
    }

    #[test]
    fn test_datatype_is_any() {
        let dtype = DataType::Any;

        let ret = dtype.is_any();
        assert_eq!(ret, true);
    }

    #[test]
    fn test_datatype_is_bool() {
        let dtype = DataType::Boolean;

        let ret = dtype.is_bool();
        assert_eq!(ret, true);
    }

    #[test]
    fn test_datatype_is_int() {
        let dtype = DataType::Integer;

        let ret = dtype.is_int();
        assert_eq!(ret, true);
    }

    #[test]
    fn test_datatype_is_float() {
        let dtype = DataType::Float;

        let ret = dtype.is_float();
        assert_eq!(ret, true);
    }

    #[test]
    fn test_datatype_is_number() {
        let dtype = DataType::Integer;

        let ret = dtype.is_number();
        assert_eq!(ret, true);

        let dtype = DataType::Float;

        let ret = dtype.is_number();
        assert_eq!(ret, true);
    }

    #[test]
    fn test_datatype_is_text() {
        let dtype = DataType::Text;

        let ret = dtype.is_text();
        assert_eq!(ret, true);
    }

    #[test]
    fn test_datatype_is_time() {
        let dtype = DataType::Time;

        let ret = dtype.is_time();
        assert_eq!(ret, true);
    }

    #[test]
    fn test_datatype_is_date() {
        let dtype = DataType::Date;

        let ret = dtype.is_date();
        assert_eq!(ret, true);
    }

    #[test]
    fn test_datatype_is_datetime() {
        let dtype = DataType::DateTime;

        let ret = dtype.is_datetime();
        assert_eq!(ret, true);
    }

    #[test]
    fn test_datatype_is_null() {
        let dtype = DataType::Null;

        let ret = dtype.is_null();
        assert_eq!(ret, true);
    }

    #[test]
    fn test_datatype_is_undefined() {
        let dtype = DataType::Undefined;

        let ret = dtype.is_undefined();
        assert_eq!(ret, true);
    }

    #[test]
    fn test_datatype_is_variant() {
        let dtype = DataType::Variant(vec![DataType::Text, DataType::Integer]);

        let ret = dtype.is_variant();
        assert_eq!(ret, true);
    }

    #[test]
    fn test_datatype_is_optional() {
        let dtype = DataType::Optional(Box::new(DataType::Text));

        let ret = dtype.is_optional();
        assert_eq!(ret, true);
    }

    #[test]
    fn test_datatype_is_varargs() {
        let dtype = DataType::Varargs(Box::new(DataType::Text));

        let ret = dtype.is_varargs();
        assert_eq!(ret, true);
    }
}
