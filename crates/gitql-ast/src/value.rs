use std::cmp::Ordering;
use std::fmt;
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

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Integer(i64) => write!(f, "{}", i64),
            Value::Float(f64) => write!(f, "{}", f64),
            Value::Text(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::DateTime(dt) => write!(f, "{}", time_stamp_to_date_time(*dt)),
            Value::Date(d) => write!(f, "{}", time_stamp_to_date(*d)),
            Value::Time(t) => write!(f, "{}", t),
            Value::Null => write!(f, "Null"),
        }
    }
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
            DataType::DateTime => self.as_date_time() == other.as_date_time(),
            DataType::Date => self.as_date() == other.as_date(),
            DataType::Time => self.as_time() == other.as_time(),
            DataType::Undefined => true,
            DataType::Null => true,
            _ => false,
        }
    }

    pub fn compare(&self, other: &Self) -> Ordering {
        let self_type = self.data_type();
        let other_type = other.data_type();

        if self_type.is_int() && other_type.is_int() {
            return other.as_int().cmp(&self.as_int());
        }

        if self_type.is_float() && other_type.is_float() {
            return other.as_float().total_cmp(&self.as_float());
        }

        if self_type.is_text() && other_type.is_text() {
            return other.as_text().cmp(&self.as_text());
        }

        if self_type.is_datetime() && other_type.is_datetime() {
            return other.as_date_time().cmp(&self.as_date_time());
        }

        if self_type.is_date() && other_type.is_date() {
            return other.as_date().cmp(&self.as_date());
        }

        if self_type.is_time() && other_type.is_time() {
            return other.as_time().cmp(&self.as_time());
        }

        Ordering::Equal
    }

    pub fn plus(&self, other: &Value) -> Result<Value, String> {
        let self_type = self.data_type();
        let other_type = other.data_type();

        if self_type.is_int() && other_type.is_int() {
            let lhs = self.as_int();
            let rhs = other.as_int();

            if let Some(sub) = lhs.checked_add(rhs) {
                return Ok(Value::Integer(sub));
            }

            return Err(format!(
                "Attempt to compute `{} + {}`, which would overflow",
                lhs, rhs
            ));
        }

        if self_type.is_float() && other_type.is_float() {
            return Ok(Value::Float(self.as_float() + other.as_float()));
        }

        if self_type.is_int() && other_type.is_float() {
            return Ok(Value::Float((self.as_int() as f64) + other.as_float()));
        }

        if self_type.is_float() && other_type.is_int() {
            return Ok(Value::Float(self.as_float() + (other.as_int() as f64)));
        }

        Ok(Value::Integer(0))
    }

    pub fn minus(&self, other: &Value) -> Result<Value, String> {
        let self_type = self.data_type();
        let other_type = other.data_type();

        if self_type.is_int() && other_type.is_int() {
            let lhs = self.as_int();
            let rhs = other.as_int();

            if let Some(sub) = lhs.checked_sub(rhs) {
                return Ok(Value::Integer(sub));
            }

            return Err(format!(
                "Attempt to compute `{} - {}`, which would overflow",
                lhs, rhs
            ));
        }

        if self_type.is_float() && other_type.is_float() {
            return Ok(Value::Float(self.as_float() - other.as_float()));
        }

        if self_type.is_int() && other_type.is_float() {
            return Ok(Value::Float((self.as_int() as f64) - other.as_float()));
        }

        if self_type.is_float() && other_type.is_int() {
            return Ok(Value::Float(self.as_float() - (other.as_int() as f64)));
        }

        Ok(Value::Integer(0))
    }

    pub fn mul(&self, other: &Value) -> Result<Value, String> {
        let self_type = self.data_type();
        let other_type = other.data_type();

        if self_type.is_int() && other_type.is_int() {
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

        if self_type.is_float() && other_type.is_float() {
            return Ok(Value::Float(self.as_float() * other.as_float()));
        }

        if self_type.is_int() && other_type.is_float() {
            return Ok(Value::Float(other.as_float().mul(self.as_int() as f64)));
        }

        if self_type.is_float() && other_type.is_int() {
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
                return Err(format!("Attempt to divide `{}` by zero", self));
            }
        }

        if self_type.is_int() && other_type.is_int() {
            return Ok(Value::Integer(self.as_int() / other.as_int()));
        }

        if self_type.is_float() && other_type.is_float() {
            return Ok(Value::Float(self.as_float() / other.as_float()));
        }

        if self_type.is_int() && other_type.is_float() {
            return Ok(Value::Float(self.as_int() as f64 / other.as_float()));
        }

        if self_type.is_float() && other_type.is_int() {
            return Ok(Value::Float(self.as_float() / other.as_int() as f64));
        }

        Ok(Value::Integer(0))
    }

    pub fn modulus(&self, other: &Value) -> Result<Value, String> {
        let self_type = self.data_type();
        let other_type = other.data_type();

        if other_type.is_int() {
            let other = other.as_int();
            if other == 0 {
                return Err(format!(
                    "Attempt to calculate the remainder of `{}` with a divisor of zero",
                    self
                ));
            }
        }

        if self_type.is_int() && other_type.is_int() {
            return Ok(Value::Integer(self.as_int() % other.as_int()));
        }

        if self_type.is_float() && other_type.is_float() {
            return Ok(Value::Float(self.as_float() % other.as_float()));
        }

        if self_type.is_int() && other_type.is_float() {
            return Ok(Value::Float(self.as_int() as f64 % other.as_float()));
        }

        if self_type.is_float() && other_type.is_int() {
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
    fn test_value_fmt() {
        let value = Value::Integer(1);
        assert_eq!(format!("{}", value), "1");

        let value = Value::Float(1f64);
        assert_eq!(format!("{}", value), "1");

        let value = Value::Text("hello".to_string());
        assert_eq!(format!("{}", value), "hello");

        let value = Value::Boolean(false);
        assert_eq!(format!("{}", value), "false");

        let value = Value::DateTime(1704890191);
        println!("{}", value);
        assert!(true);

        let value = Value::Date(1704890191);
        println!("{}", value);
        assert!(true);

        let value = Value::Time("12:36:31".to_string());
        println!("{}", value);
        assert!(true);

        let value = Value::Null;
        assert_eq!(format!("{}", value), "Null");
    }

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

        let value = Value::Float(1.0);
        let other = Value::Float(1.0);
        let ret = value.equals(&other);
        assert_eq!(ret, true);

        let value = Value::Boolean(true);
        let other = Value::Boolean(true);
        let ret = value.equals(&other);
        assert_eq!(ret, true);

        let value = Value::DateTime(1704890191);
        let other = Value::DateTime(1704890191);
        let ret = value.equals(&other);
        assert_eq!(ret, true);

        let value = Value::Date(1704890191);
        let other = Value::Date(1704890191);
        let ret = value.equals(&other);
        assert_eq!(ret, true);

        let value = Value::Time("12:36:31".to_string());
        let other = Value::Time("12:36:31".to_string());
        let ret = value.equals(&other);
        assert_eq!(ret, true);

        let value = Value::Null;
        let other = Value::Null;
        let ret = value.equals(&other);
        assert_eq!(ret, true);
    }

    #[test]
    fn test_value_compare() {
        let value = Value::Integer(1);
        let other = Value::Null;
        let ret = value.compare(&other);
        assert_eq!(ret, Ordering::Equal);

        let value = Value::Integer(1);
        let other = Value::Integer(1);
        let ret = value.compare(&other);
        assert_eq!(ret, Ordering::Equal);

        let value = Value::Integer(1);
        let other = Value::Integer(2);
        let ret = value.compare(&other);
        assert_eq!(ret, Ordering::Greater);

        let value = Value::Float(1.0);
        let other = Value::Float(1.0);
        let ret = value.compare(&other);
        assert_eq!(ret, Ordering::Equal);

        let value = Value::Float(1.0);
        let other = Value::Float(2.0);
        let ret = value.compare(&other);
        assert_eq!(ret, Ordering::Greater);

        let value = Value::Text("hello".to_string());
        let other = Value::Text("hello".to_string());
        let ret = value.compare(&other);
        assert_eq!(ret, Ordering::Equal);

        let value = Value::Text("hello".to_string());
        let other = Value::Text("world".to_string());
        let ret = value.compare(&other);
        assert_eq!(ret, Ordering::Greater);

        let value = Value::Boolean(true);
        let other = Value::Boolean(true);
        let ret = value.compare(&other);
        assert_eq!(ret, Ordering::Equal);

        let value = Value::DateTime(1704890191);
        let other = Value::DateTime(1704890191);
        let ret = value.compare(&other);
        assert_eq!(ret, Ordering::Equal);

        let value = Value::DateTime(1704890191);
        let other = Value::DateTime(1704890192);
        let ret = value.compare(&other);
        assert_eq!(ret, Ordering::Greater);

        let value = Value::Date(1704890191);
        let other = Value::Date(1704890191);
        let ret = value.compare(&other);
        assert_eq!(ret, Ordering::Equal);

        let value = Value::Date(1704890191);
        let other = Value::Date(1704890192);
        let ret = value.compare(&other);
        assert_eq!(ret, Ordering::Greater);

        let value = Value::Time("12:36:31".to_string());
        let other = Value::Time("12:36:31".to_string());
        let ret = value.compare(&other);
        assert_eq!(ret, Ordering::Equal);

        let value = Value::Time("12:36:31".to_string());
        let other = Value::Time("12:36:32".to_string());
        let ret = value.compare(&other);
        assert_eq!(ret, Ordering::Greater);

        let value = Value::Null;
        let other = Value::Null;
        let ret = value.compare(&other);
        assert_eq!(ret, Ordering::Equal);
    }

    #[test]
    fn test_value_plus() {
        let value = Value::Integer(1);
        let other = Value::Null;
        let ret = value.plus(&other);
        assert_eq!(ret.as_int(), 0);

        let value = Value::Integer(1);
        let other = Value::Integer(1);
        let ret = value.plus(&other);
        assert_eq!(ret.as_int(), 2);

        let value = Value::Float(1.0);
        let other = Value::Float(1.0);
        let ret = value.plus(&other);
        assert_eq!(ret.as_float(), 2.0);

        let value = Value::Integer(1);
        let other = Value::Float(1.0);
        let ret = value.plus(&other);
        assert_eq!(ret.as_float(), 2.0);

        let value = Value::Float(1.0);
        let other = Value::Integer(1);
        let ret = value.plus(&other);
        assert_eq!(ret.as_float(), 2.0);
    }

    #[test]
    fn test_value_minus() {
        let value = Value::Integer(1);
        let other = Value::Null;
        let ret = value.minus(&other);
        assert_eq!(ret.as_int(), 0);

        let value = Value::Integer(1);
        let other = Value::Integer(1);
        let ret = value.minus(&other);
        assert_eq!(ret.as_int(), 0);

        let value = Value::Float(1.0);
        let other = Value::Float(1.0);
        let ret = value.minus(&other);
        assert_eq!(ret.as_float(), 0.0);

        let value = Value::Integer(2);
        let other = Value::Float(1.0);
        let ret = value.minus(&other);
        assert_eq!(ret.as_float(), 1.0);

        let value = Value::Float(1.0);
        let other = Value::Integer(1);
        let ret = value.minus(&other);
        assert_eq!(ret.as_float(), 0.0);
    }

    #[test]
    fn test_value_mul() {
        let value = Value::Integer(1);
        let other = Value::Null;
        if let Ok(ret) = value.mul(&other) {
            assert_eq!(ret.as_int(), 0);
        } else {
            assert!(false);
        }

        let value = Value::Integer(1);
        let other = Value::Integer(2);
        if let Ok(ret) = value.mul(&other) {
            assert_eq!(ret.as_int(), 2);
        } else {
            assert!(false);
        }

        let value = Value::Float(1.0);
        let other = Value::Float(2.0);
        if let Ok(ret) = value.mul(&other) {
            assert_eq!(ret.as_float(), 2.0);
        } else {
            assert!(false);
        }

        let value = Value::Integer(2);
        let other = Value::Float(1.0);
        if let Ok(ret) = value.mul(&other) {
            assert_eq!(ret.as_float(), 2.0);
        } else {
            assert!(false);
        }

        let value = Value::Float(1.0);
        let other = Value::Integer(2);
        if let Ok(ret) = value.mul(&other) {
            assert_eq!(ret.as_float(), 2.0);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_value_div() {
        let value = Value::Integer(1);
        let other = Value::Null;
        if let Ok(ret) = value.div(&other) {
            assert_eq!(ret.as_int(), 0);
        } else {
            assert!(false);
        }

        let value = Value::Integer(1);
        let other = Value::Integer(0);
        if let Ok(_ret) = value.div(&other) {
            assert!(false);
        } else {
            assert!(true);
        }

        let value = Value::Integer(2);
        let other = Value::Integer(2);
        if let Ok(ret) = value.div(&other) {
            assert_eq!(ret.as_int(), 1);
        } else {
            assert!(false);
        }

        let value = Value::Float(2.0);
        let other = Value::Float(2.0);
        if let Ok(ret) = value.div(&other) {
            assert_eq!(ret.as_float(), 1.0);
        } else {
            assert!(false);
        }

        let value = Value::Integer(2);
        let other = Value::Float(2.0);
        if let Ok(ret) = value.div(&other) {
            assert_eq!(ret.as_float(), 1.0);
        } else {
            assert!(false);
        }

        let value = Value::Float(2.0);
        let other = Value::Integer(2);
        if let Ok(ret) = value.div(&other) {
            assert_eq!(ret.as_float(), 1.0);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_value_modulus() {
        let value = Value::Integer(1);
        let other = Value::Null;
        if let Ok(ret) = value.modulus(&other) {
            assert_eq!(ret.as_int(), 0);
        } else {
            assert!(false);
        }

        let value = Value::Integer(1);
        let other = Value::Integer(0);
        if let Ok(_ret) = value.modulus(&other) {
            assert!(false);
        } else {
            assert!(true);
        }

        let value = Value::Integer(5);
        let other = Value::Integer(3);
        if let Ok(ret) = value.modulus(&other) {
            assert_eq!(ret.as_int(), 2);
        } else {
            assert!(false);
        }

        let value = Value::Float(5.0);
        let other = Value::Float(3.0);
        if let Ok(ret) = value.modulus(&other) {
            assert_eq!(ret.as_float(), 2.0);
        } else {
            assert!(false);
        }

        let value = Value::Integer(5);
        let other = Value::Float(3.0);
        if let Ok(ret) = value.modulus(&other) {
            assert_eq!(ret.as_float(), 2.0);
        } else {
            assert!(false);
        }

        let value = Value::Float(5.0);
        let other = Value::Integer(3);
        if let Ok(ret) = value.modulus(&other) {
            assert_eq!(ret.as_float(), 2.0);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_value_data_type() {
        let value = Value::Integer(1);
        let ret = value.data_type();
        assert_eq!(ret.is_int(), true);

        let value = Value::Float(1.0);
        let ret = value.data_type();
        assert_eq!(ret.is_float(), true);

        let value = Value::Text("hello".to_string());
        let ret = value.data_type();
        assert_eq!(ret.is_text(), true);

        let value = Value::Boolean(false);
        let ret = value.data_type();
        assert_eq!(ret.is_bool(), true);

        let value = Value::DateTime(1704890191);
        let ret = value.data_type();
        assert_eq!(ret.is_datetime(), true);

        let value = Value::Date(1704890191);
        let ret = value.data_type();
        assert_eq!(ret.is_date(), true);

        let value = Value::Time("12:36:31".to_string());
        let ret = value.data_type();
        assert_eq!(ret.is_time(), true);

        let value = Value::Null;
        let ret = value.data_type();
        assert_eq!(ret.is_null(), true);
    }

    #[test]
    fn test_value_as_int() {
        let value = Value::Integer(1);
        let ret = value.as_int();
        assert_eq!(ret, 1);

        let value = Value::Null;
        let ret = value.as_int();
        assert_eq!(ret, 0);
    }

    #[test]
    fn test_value_as_float() {
        let value = Value::Float(1.0);
        let ret = value.as_float();
        assert_eq!(ret, 1.0);

        let value = Value::Null;
        let ret = value.as_float();
        assert_eq!(ret, 0f64);
    }

    #[test]
    fn test_value_as_text() {
        let value = Value::Text("hello".to_string());
        let ret = value.as_text();
        assert_eq!(ret, "hello");

        let value = Value::Null;
        let ret = value.as_text();
        assert_eq!(ret, "");
    }

    #[test]
    fn test_value_as_bool() {
        let value = Value::Boolean(true);
        let ret = value.as_bool();
        assert_eq!(ret, true);

        let value = Value::Null;
        let ret = value.as_bool();
        assert_eq!(ret, false);
    }

    #[test]
    fn test_value_as_date_time() {
        let value = Value::DateTime(1704890191);
        let ret = value.as_date_time();
        assert_eq!(ret, 1704890191);

        let value = Value::Null;
        let ret = value.as_date_time();
        assert_eq!(ret, 0);
    }

    #[test]
    fn test_value_as_date() {
        let value = Value::Date(1704890191);
        let ret = value.as_date();
        assert_eq!(ret, 1704890191);

        let value = Value::Null;
        let ret = value.as_date();
        assert_eq!(ret, 0);
    }

    #[test]
    fn test_value_as_time() {
        let value = Value::Time("12:36:31".to_string());
        let ret = value.as_time();
        assert_eq!(ret, "12:36:31");

        let value = Value::Null;
        let ret = value.as_time();
        assert_eq!(ret, "");
    }
}
