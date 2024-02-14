use gitql_ast::date_utils::is_valid_date_format;
use gitql_ast::date_utils::is_valid_datetime_format;
use gitql_ast::date_utils::is_valid_time_format;
use gitql_ast::environment::Environment;
use gitql_ast::expression::Expression;
use gitql_ast::expression::ExpressionKind;
use gitql_ast::expression::StringExpression;
use gitql_ast::expression::StringValueType;
use gitql_ast::types::DataType;

use crate::diagnostic::Diagnostic;

/// The return result after performing types checking with implicit casting option
pub enum TypeCheckResult {
    /// Both right and left hand sides types are equals without implicit casting
    Equals,
    /// Both right and left hand sides types are not equals and can't perform implicit casting
    NotEqualAndCantImplicitCast,
    /// Not Equals and can't perform implicit casting with error message provided
    Error(Box<Diagnostic>),
    /// Right hand side type will match the left side after implicit casting
    RightSideCasted(Box<dyn Expression>),
    /// Left hand side type will match the right side after implicit casting
    LeftSideCasted(Box<dyn Expression>),
}

/// Check if expression type and data type are equals
/// If not then check if one can be implicit casted to the other
#[allow(clippy::borrowed_box)]
pub fn is_expression_type_equals(
    scope: &Environment,
    expr: &Box<dyn Expression>,
    data_type: &DataType,
) -> TypeCheckResult {
    let expr_type = expr.expr_type(scope);

    // Both types are already equals without need for implicit casting
    if expr_type == *data_type {
        return TypeCheckResult::Equals;
    }

    // Cast expr type from Text literal to time
    if data_type.is_time() && expr_type.is_text() && expr.kind() == ExpressionKind::String {
        let literal = expr.as_any().downcast_ref::<StringExpression>().unwrap();
        let string_literal_value = &literal.value;
        if !is_valid_time_format(string_literal_value) {
            return TypeCheckResult::Error(
                Diagnostic::error(&format!(
                    "Can't compare Time and Text `{}` because it can't be implicitly casted to Time",
                    string_literal_value
                )).add_help("A valid Time format must match `HH:MM:SS` or `HH:MM:SS.SSS`")
                .add_help("You can use `MAKETIME(hour, minute, second)` function to create date value")
                .as_boxed(),
            );
        }

        return TypeCheckResult::RightSideCasted(Box::new(StringExpression {
            value: string_literal_value.to_owned(),
            value_type: StringValueType::Time,
        }));
    }

    // Cast expr type from Text literal to Date
    if data_type.is_date() && expr_type.is_text() && expr.kind() == ExpressionKind::String {
        let literal = expr.as_any().downcast_ref::<StringExpression>().unwrap();
        let string_literal_value = &literal.value;
        if !is_valid_date_format(string_literal_value) {
            return TypeCheckResult::Error(
                Diagnostic::error(&format!(
                    "Can't compare Date and Text `{}` because it can't be implicitly casted to Date",
                    string_literal_value
                )).add_help("A valid Date format must match `YYYY-MM-DD`")
                .add_help("You can use `MAKEDATE(year, dayOfYear)` function to a create date value")
                .as_boxed(),
            );
        }

        return TypeCheckResult::RightSideCasted(Box::new(StringExpression {
            value: string_literal_value.to_owned(),
            value_type: StringValueType::Date,
        }));
    }

    // Cast right hand side type from Text literal to DateTime
    if data_type.is_datetime() && expr_type.is_text() && expr.kind() == ExpressionKind::String {
        let literal = expr.as_any().downcast_ref::<StringExpression>().unwrap();
        let string_literal_value = &literal.value;
        if !is_valid_datetime_format(string_literal_value) {
            return TypeCheckResult::Error(
                Diagnostic::error(&format!(
                    "Can't compare DateTime and Text `{}` because it can't be implicitly casted to DateTime",
                    string_literal_value
                )).add_help("A valid DateTime format must match `YYYY-MM-DD HH:MM:SS` or `YYYY-MM-DD HH:MM:SS.SSS`")
                .as_boxed(),
            );
        }

        return TypeCheckResult::RightSideCasted(Box::new(StringExpression {
            value: string_literal_value.to_owned(),
            value_type: StringValueType::DateTime,
        }));
    }

    TypeCheckResult::NotEqualAndCantImplicitCast
}

/// Check if two expressions types are equals
/// If not then check if one can be implicit casted to the other
#[allow(clippy::borrowed_box)]
pub fn are_types_equals(
    scope: &Environment,
    lhs: &Box<dyn Expression>,
    rhs: &Box<dyn Expression>,
) -> TypeCheckResult {
    let lhs_type = lhs.expr_type(scope);
    let rhs_type = rhs.expr_type(scope);

    // Both types are already equals without need for implicit casting
    if lhs_type == rhs_type {
        return TypeCheckResult::Equals;
    }

    // Cast right hand side type from Text literal to time
    if lhs_type.is_time() && rhs_type.is_text() && rhs.kind() == ExpressionKind::String {
        let expr = rhs.as_any().downcast_ref::<StringExpression>().unwrap();
        let string_literal_value = &expr.value;
        if !is_valid_time_format(string_literal_value) {
            return TypeCheckResult::Error(
                Diagnostic::error(&format!(
                    "Can't compare Time and Text `{}` because it can't be implicitly casted to Time",
                    string_literal_value
                )).add_help("A valid Time format must match `HH:MM:SS` or `HH:MM:SS.SSS`")
                .add_help("You can use `MAKETIME(hour, minute, second)` function to a create date value")
                .as_boxed(),
            );
        }

        return TypeCheckResult::RightSideCasted(Box::new(StringExpression {
            value: string_literal_value.to_owned(),
            value_type: StringValueType::Time,
        }));
    }

    // Cast left hand side type from Text literal to time
    if lhs_type.is_text() && rhs_type.is_time() && lhs.kind() == ExpressionKind::String {
        let expr = lhs.as_any().downcast_ref::<StringExpression>().unwrap();
        let string_literal_value = &expr.value;
        if !is_valid_time_format(string_literal_value) {
            return TypeCheckResult::Error(
                Diagnostic::error(&format!(
                    "Can't compare Text `{}` and Time because it can't be implicitly casted to Time",
                    string_literal_value
                )).add_help("A valid Time format must match `HH:MM:SS` or `HH:MM:SS.SSS`")
                .add_help("You can use `MAKETIME(hour, minute, second)` function to a create date value")
                .as_boxed(),
            );
        }

        return TypeCheckResult::LeftSideCasted(Box::new(StringExpression {
            value: string_literal_value.to_owned(),
            value_type: StringValueType::Time,
        }));
    }

    // Cast right hand side type from Text literal to Date
    if lhs_type.is_date() && rhs_type.is_text() && rhs.kind() == ExpressionKind::String {
        let expr = rhs.as_any().downcast_ref::<StringExpression>().unwrap();
        let string_literal_value = &expr.value;
        if !is_valid_date_format(string_literal_value) {
            return TypeCheckResult::Error(
                Diagnostic::error(&format!(
                    "Can't compare Date and Text(`{}`) because Text can't be implicitly casted to Date",
                    string_literal_value
                )).add_help("A valid Date format should be matching `YYYY-MM-DD`")
                .add_help("You can use `MAKEDATE(year, dayOfYear)` function to a create date value")
                .as_boxed(),
            );
        }

        return TypeCheckResult::RightSideCasted(Box::new(StringExpression {
            value: string_literal_value.to_owned(),
            value_type: StringValueType::Date,
        }));
    }

    // Cast left hand side type from Text literal to Date
    if lhs_type.is_text() && rhs_type.is_date() && lhs.kind() == ExpressionKind::String {
        let expr = lhs.as_any().downcast_ref::<StringExpression>().unwrap();
        let string_literal_value = &expr.value;
        if !is_valid_date_format(string_literal_value) {
            return TypeCheckResult::Error(
                Diagnostic::error(&format!(
                    "Can't compare Text(`{}`) and Date because Text can't be implicitly casted to Date",
                    string_literal_value
                )).add_help("A valid Date format should be matching `YYYY-MM-DD`")
                .add_help("You can use `MAKEDATE(year, dayOfYear)` function to a create date value")
                .as_boxed(),
            );
        }

        return TypeCheckResult::LeftSideCasted(Box::new(StringExpression {
            value: string_literal_value.to_owned(),
            value_type: StringValueType::Date,
        }));
    }

    // Cast right hand side type from Text literal to DateTime
    if lhs_type.is_datetime() && rhs_type.is_text() && rhs.kind() == ExpressionKind::String {
        let expr = rhs.as_any().downcast_ref::<StringExpression>().unwrap();
        let string_literal_value = &expr.value;
        if !is_valid_datetime_format(string_literal_value) {
            return TypeCheckResult::Error(
                Diagnostic::error(&format!(
                    "Can't compare DateTime and Text `{}` because it can't be implicitly casted to DateTime",
                    string_literal_value
                )).add_help("A valid DateTime format must match `YYYY-MM-DD HH:MM:SS` or `YYYY-MM-DD HH:MM:SS.SSS`")
                .as_boxed(),
            );
        }

        return TypeCheckResult::RightSideCasted(Box::new(StringExpression {
            value: string_literal_value.to_owned(),
            value_type: StringValueType::DateTime,
        }));
    }

    // Cast Left hand side type from Text literal to DateTime
    if lhs_type.is_text() && rhs_type.is_datetime() && lhs.kind() == ExpressionKind::String {
        let expr = lhs.as_any().downcast_ref::<StringExpression>().unwrap();
        let string_literal_value = &expr.value;
        if !is_valid_datetime_format(string_literal_value) {
            return TypeCheckResult::Error(
                Diagnostic::error(&format!(
                    "Can't compare Text `{}` and DateTime because it can't be implicitly casted to DateTime",
                    string_literal_value
                )).add_help("A valid DateTime format must match `YYYY-MM-DD HH:MM:SS` or `YYYY-MM-DD HH:MM:SS.SSS`")
                .as_boxed(),
            );
        }

        return TypeCheckResult::LeftSideCasted(Box::new(StringExpression {
            value: string_literal_value.to_owned(),
            value_type: StringValueType::DateTime,
        }));
    }

    TypeCheckResult::NotEqualAndCantImplicitCast
}

/// Checks if all values has the same type
/// If they has the same type, return it or return None
pub fn check_all_values_are_same_type(
    env: &mut Environment,
    arguments: &Vec<Box<dyn Expression>>,
) -> Option<DataType> {
    let arguments_count = arguments.len();
    if arguments_count == 0 {
        return Some(DataType::Any);
    }

    let data_type = arguments[0].expr_type(env);
    for argument in arguments.iter().take(arguments_count).skip(1) {
        let expr_type = argument.expr_type(env);
        if data_type != expr_type {
            return None;
        }
    }

    Some(data_type)
}

#[cfg(test)]
mod tests {
    use super::*;
    use gitql_ast::expression::NumberExpression;
    use gitql_ast::value::Value;

    #[test]
    fn test_is_expression_type_equals() {
        // Cast equal
        let scope = Environment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };
        let expr: Box<dyn Expression> = Box::new(StringExpression {
            value: "name".to_string(),
            value_type: StringValueType::Text,
        });
        let data_type = DataType::Text;

        match is_expression_type_equals(&scope, &expr, &data_type) {
            TypeCheckResult::Equals => {
                assert!(true);
            }
            _ => {
                assert!(false);
            }
        }

        // Cast DataType::Text to DataType::Time
        let scope = Environment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };
        let expr: Box<dyn Expression> = Box::new(StringExpression {
            value: "12:36:31".to_string(),
            value_type: StringValueType::Text,
        });
        let data_type = DataType::Time;

        match is_expression_type_equals(&scope, &expr, &data_type) {
            TypeCheckResult::RightSideCasted(_) => {
                assert!(true);
            }
            TypeCheckResult::Error(_) => {
                assert!(false);
            }
            _ => {
                assert!(false);
            }
        }

        // Cast DataType::Text to DataType::Date
        let scope = Environment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };
        let expr: Box<dyn Expression> = Box::new(StringExpression {
            value: "2024-01-10".to_string(),
            value_type: StringValueType::Text,
        });
        let data_type = DataType::Date;

        match is_expression_type_equals(&scope, &expr, &data_type) {
            TypeCheckResult::RightSideCasted(_) => {
                assert!(true);
            }
            TypeCheckResult::Error(_) => {
                assert!(false);
            }
            _ => {
                assert!(false);
            }
        }

        // Cast DataType::Text to DataType::DateTime
        let scope = Environment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };
        let expr: Box<dyn Expression> = Box::new(StringExpression {
            value: "2024-01-10 12:36:31".to_string(),
            value_type: StringValueType::Text,
        });
        let data_type = DataType::DateTime;

        match is_expression_type_equals(&scope, &expr, &data_type) {
            TypeCheckResult::RightSideCasted(_) => {
                assert!(true);
            }
            TypeCheckResult::Error(_) => {
                assert!(false);
            }
            _ => {
                assert!(false);
            }
        }

        // Cast not equal
        let scope = Environment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };
        let expr: Box<dyn Expression> = Box::new(StringExpression {
            value: "invalid".to_string(),
            value_type: StringValueType::Text,
        });
        let data_type = DataType::Integer;

        match is_expression_type_equals(&scope, &expr, &data_type) {
            TypeCheckResult::NotEqualAndCantImplicitCast => {
                assert!(true);
            }
            _ => {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_are_types_equals() {
        // Cast equal
        let scope = Environment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };
        let lhs: Box<dyn Expression> = Box::new(StringExpression {
            value: "name".to_string(),
            value_type: StringValueType::Text,
        });
        let rhs: Box<dyn Expression> = Box::new(StringExpression {
            value: "name".to_string(),
            value_type: StringValueType::Text,
        });

        match are_types_equals(&scope, &lhs, &rhs) {
            TypeCheckResult::Equals => {
                assert!(true);
            }
            _ => {
                assert!(false);
            }
        }

        // Cast DataType::Text to DataType::Time for rhs
        let scope = Environment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };
        let lhs: Box<dyn Expression> = Box::new(StringExpression {
            value: "name".to_string(),
            value_type: StringValueType::Time,
        });
        let rhs: Box<dyn Expression> = Box::new(StringExpression {
            value: "12:36:31".to_string(),
            value_type: StringValueType::Text,
        });

        match are_types_equals(&scope, &lhs, &rhs) {
            TypeCheckResult::RightSideCasted(_) => {
                assert!(true);
            }
            TypeCheckResult::Error(_) => {
                assert!(false);
            }
            _ => {
                assert!(false);
            }
        }

        // Cast DataType::Text to DataType::Time for lhs
        let scope = Environment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };
        let lhs: Box<dyn Expression> = Box::new(StringExpression {
            value: "12:36:31".to_string(),
            value_type: StringValueType::Text,
        });
        let rhs: Box<dyn Expression> = Box::new(StringExpression {
            value: "name".to_string(),
            value_type: StringValueType::Time,
        });

        match are_types_equals(&scope, &lhs, &rhs) {
            TypeCheckResult::LeftSideCasted(_) => {
                assert!(true);
            }
            TypeCheckResult::Error(_) => {
                assert!(false);
            }
            _ => {
                assert!(false);
            }
        }

        // Cast DataType::Text to DataType::Date for rhs
        let scope = Environment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };
        let lhs: Box<dyn Expression> = Box::new(StringExpression {
            value: "name".to_string(),
            value_type: StringValueType::Date,
        });
        let rhs: Box<dyn Expression> = Box::new(StringExpression {
            value: "2024-01-10".to_string(),
            value_type: StringValueType::Text,
        });

        match are_types_equals(&scope, &lhs, &rhs) {
            TypeCheckResult::RightSideCasted(_) => {
                assert!(true);
            }
            TypeCheckResult::Error(_) => {
                assert!(false);
            }
            _ => {
                assert!(false);
            }
        }

        // Cast DataType::Text to DataType::Date for lhs
        let scope = Environment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };
        let lhs: Box<dyn Expression> = Box::new(StringExpression {
            value: "2024-01-10".to_string(),
            value_type: StringValueType::Text,
        });
        let rhs: Box<dyn Expression> = Box::new(StringExpression {
            value: "name".to_string(),
            value_type: StringValueType::Date,
        });

        match are_types_equals(&scope, &lhs, &rhs) {
            TypeCheckResult::LeftSideCasted(_) => {
                assert!(true);
            }
            TypeCheckResult::Error(_) => {
                assert!(false);
            }
            _ => {
                assert!(false);
            }
        }

        // Cast DataType::Text to DataType::DateTime for rhs
        let scope = Environment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };
        let lhs: Box<dyn Expression> = Box::new(StringExpression {
            value: "name".to_string(),
            value_type: StringValueType::DateTime,
        });
        let rhs: Box<dyn Expression> = Box::new(StringExpression {
            value: "2024-01-10 12:36:31".to_string(),
            value_type: StringValueType::Text,
        });

        match are_types_equals(&scope, &lhs, &rhs) {
            TypeCheckResult::RightSideCasted(_) => {
                assert!(true);
            }
            TypeCheckResult::Error(_) => {
                assert!(false);
            }
            _ => {
                assert!(false);
            }
        }

        // Cast DataType::Text to DataType::DateTime for lhs
        let scope = Environment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };
        let lhs: Box<dyn Expression> = Box::new(StringExpression {
            value: "2024-01-10 12:36:31".to_string(),
            value_type: StringValueType::Text,
        });
        let rhs: Box<dyn Expression> = Box::new(StringExpression {
            value: "name".to_string(),
            value_type: StringValueType::DateTime,
        });

        match are_types_equals(&scope, &lhs, &rhs) {
            TypeCheckResult::LeftSideCasted(_) => {
                assert!(true);
            }
            TypeCheckResult::Error(_) => {
                assert!(false);
            }
            _ => {
                assert!(false);
            }
        }

        // Cast not equal
        let scope = Environment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };
        let lhs: Box<dyn Expression> = Box::new(NumberExpression {
            value: Value::Integer(1),
        });
        let rhs: Box<dyn Expression> = Box::new(NumberExpression {
            value: Value::Float(1.0),
        });

        match are_types_equals(&scope, &lhs, &rhs) {
            TypeCheckResult::NotEqualAndCantImplicitCast => {
                assert!(true);
            }
            _ => {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_check_all_values_are_same_type() {
        // Check null type
        let mut env = Environment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };
        let arguments: Vec<Box<dyn Expression>> = vec![];

        let result = check_all_values_are_same_type(&mut env, &arguments);
        if result.is_some() {
            if !result.unwrap().is_any() {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        // Check different type
        let mut env = Environment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };
        let arg1: Box<dyn Expression> = Box::new(StringExpression {
            value: "name".to_string(),
            value_type: StringValueType::Text,
        });
        let arg2: Box<dyn Expression> = Box::new(NumberExpression {
            value: Value::Integer(1),
        });
        let arguments: Vec<Box<dyn Expression>> = vec![arg1, arg2];

        let result = check_all_values_are_same_type(&mut env, &arguments);
        if result.is_none() {
            assert!(true);
        } else {
            assert!(false);
        }

        // Check the same type
        let mut env = Environment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };
        let arg1: Box<dyn Expression> = Box::new(StringExpression {
            value: "name".to_string(),
            value_type: StringValueType::Text,
        });
        let arg2: Box<dyn Expression> = Box::new(StringExpression {
            value: "name".to_string(),
            value_type: StringValueType::Text,
        });
        let arguments: Vec<Box<dyn Expression>> = vec![arg1, arg2];

        let result = check_all_values_are_same_type(&mut env, &arguments);
        if result.is_some() {
            assert!(true);
        } else {
            assert!(false);
        }
    }
}
