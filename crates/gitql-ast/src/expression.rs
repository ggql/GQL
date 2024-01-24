use std::any::Any;

use crate::environment::Environment;
use crate::function::PROTOTYPES;
use crate::types::{DataType, TABLES_FIELDS_TYPES};
use crate::value::Value;

#[derive(PartialEq)]
pub enum ExpressionKind {
    Assignment,
    String,
    Symbol,
    GlobalVariable,
    Number,
    Boolean,
    PrefixUnary,
    Arithmetic,
    Comparison,
    Like,
    Glob,
    Logical,
    Bitwise,
    Call,
    Between,
    Case,
    In,
    IsNull,
    Null,
}

pub trait Expression {
    fn kind(&self) -> ExpressionKind;
    fn expr_type(&self, scope: &Environment) -> DataType;
    fn as_any(&self) -> &dyn Any;
}

impl dyn Expression {
    pub fn is_const(&self) -> bool {
        matches!(
            self.kind(),
            ExpressionKind::Number | ExpressionKind::Boolean | ExpressionKind::String
        )
    }
}

pub struct AssignmentExpression {
    pub symbol: String,
    pub value: Box<dyn Expression>,
}

impl Expression for AssignmentExpression {
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::Assignment
    }

    fn expr_type(&self, scope: &Environment) -> DataType {
        self.value.expr_type(scope)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub enum StringValueType {
    Text,
    Time,
    Date,
    DateTime,
}

pub struct StringExpression {
    pub value: String,
    pub value_type: StringValueType,
}

impl Expression for StringExpression {
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::String
    }

    fn expr_type(&self, _scope: &Environment) -> DataType {
        match self.value_type {
            StringValueType::Text => DataType::Text,
            StringValueType::Time => DataType::Time,
            StringValueType::Date => DataType::Date,
            StringValueType::DateTime => DataType::DateTime,
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct SymbolExpression {
    pub value: String,
}

impl Expression for SymbolExpression {
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::Symbol
    }

    fn expr_type(&self, scope: &Environment) -> DataType {
        // Search in symbol table
        if scope.contains(&self.value) {
            return scope.scopes[self.value.as_str()].clone();
        }

        // Search in static table fields types
        if TABLES_FIELDS_TYPES.contains_key(&self.value.as_str()) {
            return TABLES_FIELDS_TYPES[&self.value.as_str()].clone();
        }

        DataType::Undefined
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct GlobalVariableExpression {
    pub name: String,
}

impl Expression for GlobalVariableExpression {
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::GlobalVariable
    }

    fn expr_type(&self, scope: &Environment) -> DataType {
        if scope.globals_types.contains_key(&self.name) {
            return scope.globals_types[self.name.as_str()].clone();
        }
        DataType::Undefined
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct NumberExpression {
    pub value: Value,
}

impl Expression for NumberExpression {
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::Number
    }

    fn expr_type(&self, _scope: &Environment) -> DataType {
        self.value.data_type()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct BooleanExpression {
    pub is_true: bool,
}

impl Expression for BooleanExpression {
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::Boolean
    }

    fn expr_type(&self, _scope: &Environment) -> DataType {
        DataType::Boolean
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(PartialEq)]
pub enum PrefixUnaryOperator {
    Minus,
    Bang,
}

pub struct PrefixUnary {
    pub right: Box<dyn Expression>,
    pub op: PrefixUnaryOperator,
}

impl Expression for PrefixUnary {
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::PrefixUnary
    }

    fn expr_type(&self, _scope: &Environment) -> DataType {
        if self.op == PrefixUnaryOperator::Bang {
            DataType::Boolean
        } else {
            DataType::Integer
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(PartialEq)]
pub enum ArithmeticOperator {
    Plus,
    Minus,
    Star,
    Slash,
    Modulus,
}

pub struct ArithmeticExpression {
    pub left: Box<dyn Expression>,
    pub operator: ArithmeticOperator,
    pub right: Box<dyn Expression>,
}

impl Expression for ArithmeticExpression {
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::Arithmetic
    }

    fn expr_type(&self, scope: &Environment) -> DataType {
        if self.left.expr_type(scope).is_int() && self.right.expr_type(scope).is_int() {
            return DataType::Integer;
        }
        DataType::Float
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(PartialEq)]
pub enum ComparisonOperator {
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Equal,
    NotEqual,
    NullSafeEqual,
}

pub struct ComparisonExpression {
    pub left: Box<dyn Expression>,
    pub operator: ComparisonOperator,
    pub right: Box<dyn Expression>,
}

impl Expression for ComparisonExpression {
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::Comparison
    }

    fn expr_type(&self, _scope: &Environment) -> DataType {
        if self.operator == ComparisonOperator::NullSafeEqual {
            DataType::Integer
        } else {
            DataType::Boolean
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct LikeExpression {
    pub input: Box<dyn Expression>,
    pub pattern: Box<dyn Expression>,
}

impl Expression for LikeExpression {
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::Like
    }

    fn expr_type(&self, _scope: &Environment) -> DataType {
        DataType::Boolean
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct GlobExpression {
    pub input: Box<dyn Expression>,
    pub pattern: Box<dyn Expression>,
}

impl Expression for GlobExpression {
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::Glob
    }

    fn expr_type(&self, _scope: &Environment) -> DataType {
        DataType::Boolean
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(PartialEq)]
pub enum LogicalOperator {
    Or,
    And,
    Xor,
}

pub struct LogicalExpression {
    pub left: Box<dyn Expression>,
    pub operator: LogicalOperator,
    pub right: Box<dyn Expression>,
}

impl Expression for LogicalExpression {
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::Logical
    }

    fn expr_type(&self, _scope: &Environment) -> DataType {
        DataType::Boolean
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(PartialEq)]
pub enum BitwiseOperator {
    Or,
    And,
    RightShift,
    LeftShift,
}

pub struct BitwiseExpression {
    pub left: Box<dyn Expression>,
    pub operator: BitwiseOperator,
    pub right: Box<dyn Expression>,
}

impl Expression for BitwiseExpression {
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::Bitwise
    }

    fn expr_type(&self, _scope: &Environment) -> DataType {
        DataType::Integer
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct CallExpression {
    pub function_name: String,
    pub arguments: Vec<Box<dyn Expression>>,
    pub is_aggregation: bool,
}

impl Expression for CallExpression {
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::Call
    }

    fn expr_type(&self, _scope: &Environment) -> DataType {
        let prototype = PROTOTYPES.get(&self.function_name.as_str()).unwrap();
        prototype.result.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct BetweenExpression {
    pub value: Box<dyn Expression>,
    pub range_start: Box<dyn Expression>,
    pub range_end: Box<dyn Expression>,
}

impl Expression for BetweenExpression {
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::Between
    }

    fn expr_type(&self, _scope: &Environment) -> DataType {
        DataType::Boolean
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct CaseExpression {
    pub conditions: Vec<Box<dyn Expression>>,
    pub values: Vec<Box<dyn Expression>>,
    pub default_value: Option<Box<dyn Expression>>,
    pub values_type: DataType,
}

impl Expression for CaseExpression {
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::Case
    }

    fn expr_type(&self, _scope: &Environment) -> DataType {
        self.values_type.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct InExpression {
    pub argument: Box<dyn Expression>,
    pub values: Vec<Box<dyn Expression>>,
    pub values_type: DataType,
    pub has_not_keyword: bool,
}

impl Expression for InExpression {
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::In
    }

    fn expr_type(&self, _scope: &Environment) -> DataType {
        self.values_type.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct IsNullExpression {
    pub argument: Box<dyn Expression>,
    pub has_not: bool,
}

impl Expression for IsNullExpression {
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::IsNull
    }

    fn expr_type(&self, _scope: &Environment) -> DataType {
        DataType::Boolean
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct NullExpression {}

impl Expression for NullExpression {
    fn kind(&self) -> ExpressionKind {
        ExpressionKind::Null
    }

    fn expr_type(&self, _scope: &Environment) -> DataType {
        DataType::Null
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expression_is_const() {
        assert!(true);
    }

    #[test]
    fn test_assignmentexpression_kind() {
        assert!(true);
    }

    #[test]
    fn test_assignmentexpression_expr_type() {
        let expr = AssignmentExpression{ symbol: "".to_string(),
            value: Box::new(StringExpression{ value: "".to_string(), value_type: StringValueType::Text }) };

        let scope = Environment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };

        let ret = expr.expr_type(&scope);
        assert_eq!(ret.is_text(), true);
    }

    #[test]
    fn test_stringexpression_kind() {
        assert!(true);
    }

    #[test]
    fn test_stringexpression_expr_type() {
        let expr = StringExpression {
            value: "".to_string(),
            value_type: StringValueType::Text,
        };

        let scope = Environment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };

        let ret = expr.expr_type(&scope);
        assert_eq!(ret.is_text(), true);
    }

    #[test]
    fn test_symbolexpression_kind() {
        assert!(true);
    }

    #[test]
    fn test_symbolexpression_expr_type() {
        let expr = SymbolExpression {
            value: "field1".to_string(),
        };

        let mut scope = Environment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };

        scope.scopes.insert("field1".to_string(), DataType::Text);

        let ret = expr.expr_type(&scope);
        assert_eq!(ret.is_text(), true);

        let expr = SymbolExpression {
            value: "title".to_string(),
        };

        let ret = expr.expr_type(&scope);
        assert_eq!(ret.is_text(), true);

        let expr = SymbolExpression {
            value: "invalid".to_string(),
        };

        let ret = expr.expr_type(&scope);
        assert_eq!(ret.is_undefined(), true);
    }

    #[test]
    fn test_globalvariableexpression_kind() {
        assert!(true);
    }

    #[test]
    fn test_globalvariableexpression_expr_type() {
        let expr = GlobalVariableExpression {
            name: "field1".to_string(),
        };

        let mut scope = Environment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };

        scope
            .globals_types
            .insert("field1".to_string(), DataType::Text);

        let ret = expr.expr_type(&scope);
        assert_eq!(ret.is_text(), true);

        let expr = GlobalVariableExpression {
            name: "invalid".to_string(),
        };

        let ret = expr.expr_type(&scope);
        assert_eq!(ret.is_undefined(), true);
    }

    #[test]
    fn test_numberexpression_kind() {
        assert!(true);
    }

    #[test]
    fn test_numberexpression_expr_type() {
        let expr = NumberExpression {
            value: Value::Text("field".to_string()),
        };

        let scope = Environment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };

        let ret = expr.expr_type(&scope);
        assert_eq!(ret.is_text(), true);
    }

    #[test]
    fn test_booleanexpression_kind() {
        assert!(true);
    }

    #[test]
    fn test_booleanexpression_expr_type() {
        let expr = BooleanExpression { is_true: false };

        let scope = Environment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };

        let ret = expr.expr_type(&scope);
        assert_eq!(ret.is_bool(), true);
    }

    #[test]
    fn test_prefixunaryexpression_kind() {
        assert!(true);
    }

    #[test]
    fn test_prefixunaryexpression_expr_type() {
        let expr = PrefixUnary {
            right: Box::new(NumberExpression { value: Value::Null }),
            op: PrefixUnaryOperator::Minus,
        };

        let scope = Environment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };

        let ret = expr.expr_type(&scope);
        assert_eq!(ret.is_int(), true);

        let expr = PrefixUnary {
            right: Box::new(NumberExpression { value: Value::Null }),
            op: PrefixUnaryOperator::Bang,
        };

        let ret = expr.expr_type(&scope);
        assert_eq!(ret.is_bool(), true);
    }

    #[test]
    fn test_arithmeticexpression_kind() {
        assert!(true);
    }

    #[test]
    fn test_arithmeticexpression_expr_type() {
        let expr = ArithmeticExpression {
            left: Box::new(NumberExpression {
                value: Value::Integer(1),
            }),
            operator: ArithmeticOperator::Plus,
            right: Box::new(NumberExpression {
                value: Value::Integer(1),
            }),
        };

        let scope = Environment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };

        let ret = expr.expr_type(&scope);
        assert_eq!(ret.is_int(), true);

        let expr = ArithmeticExpression {
            left: Box::new(NumberExpression {
                value: Value::Integer(1),
            }),
            operator: ArithmeticOperator::Plus,
            right: Box::new(NumberExpression {
                value: Value::Float(1.0),
            }),
        };

        let ret = expr.expr_type(&scope);
        assert_eq!(ret.is_float(), true);
    }

    #[test]
    fn test_comparisionexpression_kind() {
        assert!(true);
    }

    #[test]
    fn test_comparisionexpression_expr_type() {
        let expr = ComparisonExpression {
            left: Box::new(NumberExpression {
                value: Value::Integer(1),
            }),
            operator: ComparisonOperator::NullSafeEqual,
            right: Box::new(NumberExpression {
                value: Value::Integer(1),
            }),
        };

        let scope = Environment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };

        let ret = expr.expr_type(&scope);
        assert_eq!(ret.is_int(), true);

        let expr = ComparisonExpression {
            left: Box::new(NumberExpression {
                value: Value::Integer(1),
            }),
            operator: ComparisonOperator::NotEqual,
            right: Box::new(NumberExpression {
                value: Value::Integer(1),
            }),
        };

        let ret = expr.expr_type(&scope);
        assert_eq!(ret.is_bool(), true);
    }

    #[test]
    fn test_likeexpression_kind() {
        assert!(true);
    }

    #[test]
    fn test_likeexpression_expr_type() {
        let expr = LikeExpression {
            input: Box::new(NumberExpression {
                value: Value::Integer(1),
            }),
            pattern: Box::new(NumberExpression {
                value: Value::Integer(1),
            }),
        };

        let scope = Environment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };

        let ret = expr.expr_type(&scope);
        assert_eq!(ret.is_bool(), true);
    }

    #[test]
    fn test_globalexpression_kind() {
        assert!(true);
    }

    #[test]
    fn test_globalexpression_expr_type() {
        let expr = GlobExpression {
            input: Box::new(NumberExpression {
                value: Value::Integer(1),
            }),
            pattern: Box::new(NumberExpression {
                value: Value::Integer(1),
            }),
        };

        let scope = Environment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };

        let ret = expr.expr_type(&scope);
        assert_eq!(ret.is_bool(), true);
    }

    #[test]
    fn test_logicalexpression_kind() {
        assert!(true);
    }

    #[test]
    fn test_logicalexpression_expr_type() {
        let expr = LogicalExpression {
            left: Box::new(NumberExpression {
                value: Value::Integer(1),
            }),
            operator: LogicalOperator::Or,
            right: Box::new(NumberExpression {
                value: Value::Integer(1),
            }),
        };

        let scope = Environment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };

        let ret = expr.expr_type(&scope);
        assert_eq!(ret.is_bool(), true);
    }

    #[test]
    fn test_bitwiseexpression_kind() {
        assert!(true);
    }

    #[test]
    fn test_bitwiseexpression_expr_type() {
        let expr = BitwiseExpression {
            left: Box::new(NumberExpression {
                value: Value::Integer(1),
            }),
            operator: BitwiseOperator::Or,
            right: Box::new(NumberExpression {
                value: Value::Integer(1),
            }),
        };

        let scope = Environment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };

        let ret = expr.expr_type(&scope);
        assert_eq!(ret.is_int(), true);
    }

    #[test]
    fn test_callexpression_kind() {
        assert!(true);
    }

    #[test]
    fn test_callexpression_expr_type() {
        let expr = CallExpression {
            function_name: "lower".to_string(),
            arguments: vec![Box::new(NumberExpression {
                value: Value::Integer(1),
            })],
            is_aggregation: false,
        };

        let scope = Environment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };

        let ret = expr.expr_type(&scope);
        assert_eq!(ret.is_text(), true);
    }

    #[test]
    fn test_betweenexpression_kind() {
        assert!(true);
    }

    #[test]
    fn test_betweenexpression_expr_type() {
        let expr = BetweenExpression {
            value: Box::new(NumberExpression {
                value: Value::Integer(1),
            }),
            range_start: Box::new(NumberExpression {
                value: Value::Integer(1),
            }),
            range_end: Box::new(NumberExpression {
                value: Value::Integer(1),
            }),
        };

        let scope = Environment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };

        let ret = expr.expr_type(&scope);
        assert_eq!(ret.is_bool(), true);
    }

    #[test]
    fn test_caseexpression_kind() {
        assert!(true);
    }

    #[test]
    fn test_caseexpression_expr_type() {
        let expr = CaseExpression {
            conditions: vec![],
            values: vec![],
            default_value: None,
            values_type: DataType::Text,
        };

        let scope = Environment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };

        let ret = expr.expr_type(&scope);
        assert_eq!(ret.is_text(), true);
    }

    #[test]
    fn test_inexpression_kind() {
        assert!(true);
    }

    #[test]
    fn test_inexpression_expr_type() {
        let expr = InExpression {
            argument: Box::new(NumberExpression {
                value: Value::Integer(1),
            }),
            values: vec![],
            values_type: DataType::Text,
            has_not_keyword: false,
        };

        let scope = Environment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };

        let ret = expr.expr_type(&scope);
        assert_eq!(ret.is_text(), true);
    }

    #[test]
    fn test_isnullexpression_kind() {
        assert!(true);
    }

    #[test]
    fn test_isnullexpression_expr_type() {
        let expr = IsNullExpression {
            argument: Box::new(NumberExpression {
                value: Value::Integer(1),
            }),
            has_not: false,
        };

        let scope = Environment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };

        let ret = expr.expr_type(&scope);
        assert_eq!(ret.is_bool(), true);
    }

    #[test]
    fn test_nullexpression_kind() {
        assert!(true);
    }

    #[test]
    fn test_nullexpression_expr_type() {
        let expr = NullExpression {};

        let scope = Environment {
            globals: Default::default(),
            globals_types: Default::default(),
            scopes: Default::default(),
        };

        let ret = expr.expr_type(&scope);
        assert_eq!(ret.is_null(), true);
    }
}
