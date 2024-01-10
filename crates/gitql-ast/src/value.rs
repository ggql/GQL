use std::cmp::Ordering;
use std::ops::Mul;

use crate::date_utils::time_stamp_to_date;
use crate::date_utils::time_stamp_to_date_time;
use crate::types::DataType;

#[derive(Clone)]
pub enum Value {
    Integer(i64),
    Float(f64),
    Text(String),
    Boolean(bool),
    DateTime(i64),
    Date(i64),
    Time(String),
    Null,
}

impl Value {
    pub fn equals(&self, other: &Self) -> bool {
        if self.data_type() != other.data_type() {
            return false;
        }

        match self.data_type() {
            DataType::Any => true,
            DataType::Text => self.as_text() == other.as_text(),
            DataType::Integer => self.as_int() == other.as_int(),
            DataType::Float => self.as_float() == other.as_float(),
            DataType::Boolean => self.as_bool() == other.as_bool(),
            DataType::DateTime => self.as_date() == other.as_date(),
            DataType::Date => self.as_date() == other.as_date(),
            DataType::Time => self.as_date() == other.as_date(),
            DataType::Undefined => true,
            DataType::Null => true,
        }
    }

    pub fn compare(&self, other: &Self) -> Ordering {
        let self_type = self.data_type();
        let other_type = other.data_type();

        if self_type.is_type(DataType::Integer) && other_type.is_type(DataType::Integer) {
            return other.as_int().cmp(&self.as_int());
        }

        if self_type.is_type(DataType::Float) && other_type.is_type(DataType::Float) {
            return other.as_float().total_cmp(&self.as_float());
        }

        if self_type.is_type(DataType::Text) && other_type.is_type(DataType::Text) {
            return other.as_text().cmp(&self.as_text());
        }

        if self_type.is_type(DataType::DateTime) && other_type.is_type(DataType::DateTime) {
            return other.as_date_time().cmp(&self.as_date_time());
        }

        if self_type.is_type(DataType::Date) && other_type.is_type(DataType::Date) {
            return other.as_date().cmp(&self.as_date());
        }

        if self_type.is_type(DataType::Time) && other_type.is_type(DataType::Time) {
            return other.as_time().cmp(&self.as_time());
        }

        Ordering::Equal
    }

    pub fn plus(&self, other: &Value) -> Value {
        let self_type = self.data_type();
        let other_type = other.data_type();

        if self_type == DataType::Integer && other_type == DataType::Integer {
            return Value::Integer(self.as_int() + other.as_int());
        }

        if self_type == DataType::Float && other_type == DataType::Float {
            return Value::Float(self.as_float() + other.as_float());
        }

        if self_type == DataType::Integer && other_type == DataType::Float {
            return Value::Float((self.as_int() as f64) + other.as_float());
        }

        if self_type == DataType::Float && other_type == DataType::Integer {
            return Value::Float(self.as_float() + (other.as_int() as f64));
        }

        Value::Integer(0)
    }

    pub fn minus(&self, other: &Value) -> Value {
        let self_type = self.data_type();
        let other_type = other.data_type();

        if self_type == DataType::Integer && other_type == DataType::Integer {
            return Value::Integer(self.as_int() - other.as_int());
        }

        if self_type == DataType::Float && other_type == DataType::Float {
            return Value::Float(self.as_float() - other.as_float());
        }

        if self_type == DataType::Integer && other_type == DataType::Float {
            return Value::Float((self.as_int() as f64) - other.as_float());
        }

        if self_type == DataType::Float && other_type == DataType::Integer {
            return Value::Float(self.as_float() - (other.as_int() as f64));
        }

        Value::Integer(0)
    }

    pub fn mul(&self, other: &Value) -> Result<Value, String> {
        let self_type = self.data_type();
        let other_type = other.data_type();

        if self_type == DataType::Integer && other_type == DataType::Integer {
            let lhs = self.as_int();
            let rhs = other.as_int();
            let multi_result = lhs.overflowing_mul(rhs);
            if multi_result.1 {
                return Err(format!(
                    "Attempt to compute `{} * {}`, which would overflow",
                    lhs, rhs
                ));
            }
            return Ok(Value::Integer(multi_result.0));
        }

        if self_type == DataType::Float && other_type == DataType::Float {
            return Ok(Value::Float(self.as_float() * other.as_float()));
        }

        if self_type == DataType::Integer && other_type == DataType::Float {
            return Ok(Value::Float(other.as_float().mul(self.as_int() as f64)));
        }

        if self_type == DataType::Float && other_type == DataType::Integer {
            return Ok(Value::Float(self.as_float().mul(other.as_int() as f64)));
        }

        Ok(Value::Integer(0))
    }

    pub fn div(&self, other: &Value) -> Result<Value, String> {
        let self_type = self.data_type();
        let other_type = other.data_type();

        if other_type == DataType::Integer {
            let other = other.as_int();
            if other == 0 {
                return Err(format!("Attempt to divide `{}` by zero", self.literal()));
            }
        }

        if self_type == DataType::Integer && other_type == DataType::Integer {
            return Ok(Value::Integer(self.as_int() / other.as_int()));
        }

        if self_type == DataType::Float && other_type == DataType::Float {
            return Ok(Value::Float(self.as_float() / other.as_float()));
        }

        if self_type == DataType::Integer && other_type == DataType::Float {
            return Ok(Value::Float(self.as_int() as f64 / other.as_float()));
        }

        if self_type == DataType::Float && other_type == DataType::Integer {
            return Ok(Value::Float(self.as_float() / other.as_int() as f64));
        }

        Ok(Value::Integer(0))
    }

    pub fn modulus(&self, other: &Value) -> Result<Value, String> {
        let self_type = self.data_type();
        let other_type = other.data_type();

        if other_type == DataType::Integer {
            let other = other.as_int();
            if other == 0 {
                return Err(format!(
                    "Attempt to calculate the remainder of `{}` with a divisor of zero",
                    self.literal()
                ));
            }
        }

        if self_type == DataType::Integer && other_type == DataType::Integer {
            return Ok(Value::Integer(self.as_int() % other.as_int()));
        }

        if self_type == DataType::Float && other_type == DataType::Float {
            return Ok(Value::Float(self.as_float() % other.as_float()));
        }

        if self_type == DataType::Integer && other_type == DataType::Float {
            return Ok(Value::Float(self.as_int() as f64 % other.as_float()));
        }

        if self_type == DataType::Float && other_type == DataType::Integer {
            return Ok(Value::Float(self.as_float() % other.as_int() as f64));
        }

        Ok(Value::Integer(0))
    }

    pub fn data_type(&self) -> DataType {
        match self {
            Value::Integer(_) => DataType::Integer,
            Value::Float(_) => DataType::Float,
            Value::Text(_) => DataType::Text,
            Value::Boolean(_) => DataType::Boolean,
            Value::DateTime(_) => DataType::DateTime,
            Value::Date(_) => DataType::Date,
            Value::Time(_) => DataType::Time,
            Value::Null => DataType::Null,
        }
    }

    pub fn literal(&self) -> String {
        match self {
            Value::Integer(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::Text(s) => s.to_string(),
            Value::Boolean(b) => b.to_string(),
            Value::DateTime(dt) => time_stamp_to_date_time(*dt),
            Value::Date(d) => time_stamp_to_date(*d),
            Value::Time(t) => t.to_string(),
            Value::Null => "Null".to_string(),
        }
    }

    pub fn as_int(&self) -> i64 {
        if let Value::Integer(n) = self {
            return *n;
        }
        0
    }

    pub fn as_float(&self) -> f64 {
        if let Value::Float(n) = self {
            return *n;
        }
        0f64
    }

    pub fn as_text(&self) -> String {
        if let Value::Text(s) = self {
            return s.to_string();
        }
        "".to_owned()
    }

    pub fn as_bool(&self) -> bool {
        if let Value::Boolean(b) = self {
            return *b;
        }
        false
    }

    pub fn as_date_time(&self) -> i64 {
        if let Value::DateTime(d) = self {
            return *d;
        }
        0
    }

    pub fn as_date(&self) -> i64 {
        if let Value::Date(d) = self {
            return *d;
        }
        0
    }

    pub fn as_time(&self) -> String {
        if let Value::Time(d) = self {
            return d.to_string();
        }
        "".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_equals() {
        let value = Value::Integer(1);
        let other = Value::Null;

        let ret = value.equals(&other);
        assert_eq!(ret, false);

        let value = Value::Integer(1);
        let other = Value::Integer(1);
        let ret = value.equals(&other);
        assert_eq!(ret, true);

        let value = Value::Integer(1);
        let other = Value::Integer(2);
        let ret = value.equals(&other);
        assert_eq!(ret, false);

        let value = Value::Float(1.0);
        let other = Value::Float(1.0);
        let ret = value.equals(&other);
        assert_eq!(ret, true);

        let value = Value::Float(1.0);
        let other = Value::Float(2.0);
        let ret = value.equals(&other);
        assert_eq!(ret, false);

        let value = Value::Text("hello".to_string());
        let other = Value::Text("hello".to_string());
        let ret = value.equals(&other);
        assert_eq!(ret, true);

        let value = Value::Text("hello".to_string());
        let other = Value::Text("world".to_string());
        let ret = value.equals(&other);
        assert_eq!(ret, false);

        let value = Value::Boolean(true);
        let other = Value::Boolean(true);
        let ret = value.equals(&other);
        assert_eq!(ret, true);

        let value = Value::Boolean(true);
        let other = Value::Boolean(false);
        let ret = value.equals(&other);
        assert_eq!(ret, false);

        let value = Value::DateTime(1704890191);
        let other = Value::DateTime(1704890191);
        let ret = value.equals(&other);
        assert_eq!(ret, true);

        let value = Value::DateTime(1704890191);
        let other = Value::DateTime(1704890192);
        let ret = value.equals(&other);
        assert_eq!(ret, false);

        let value = Value::Date(1704890191);
        let other = Value::Date(1704890191);
        let ret = value.equals(&other);
        assert_eq!(ret, true);

        let value = Value::Date(1704890191);
        let other = Value::Date(1704890192);
        let ret = value.equals(&other);
        assert_eq!(ret, false);

        let value = Value::Time("12:36:31".to_string());
        let other = Value::Time("12:36:31".to_string());
        let ret = value.equals(&other);
        assert_eq!(ret, true);

        let value = Value::Time("12:36:31".to_string());
        let other = Value::Time("12:36:32".to_string());
        let ret = value.equals(&other);
        assert_eq!(ret, false);

        let value = Value::Null;
        let other = Value::Null;
        let ret = value.equals(&other);
        assert_eq!(ret, true);
    }

    #[test]
    fn test_value_compare() {
    }

    #[test]
    fn test_value_plus() {
    }

    #[test]
    fn test_value_minus() {
    }

    #[test]
    fn test_value_mul() {
    }

    #[test]
    fn test_value_div() {
    }

    #[test]
    fn test_value_modulus() {
    }

    #[test]
    fn test_value_data_type() {
    }

    #[test]
    fn test_value_literal() {
    }

    #[test]
    fn test_value_as_int() {
    }

    #[test]
    fn test_value_as_float() {
    }

    #[test]
    fn test_value_as_text() {
    }

    #[test]
    fn test_value_as_bool() {
    }

    #[test]
    fn test_value_as_date_time() {
    }

    #[test]
    fn test_value_as_date() {
    }

    #[test]
    fn test_value_as_time() {
    }
}
