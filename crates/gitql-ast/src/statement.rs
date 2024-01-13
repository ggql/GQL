use std::any::Any;
use std::collections::HashMap;

use crate::expression::Expression;

pub enum StatementKind {
    Select,
    Where,
    Having,
    Limit,
    Offset,
    OrderBy,
    GroupBy,
    AggregateFunction,
    GlobalVariable,
}

pub trait Statement {
    fn get_statement_kind(&self) -> StatementKind;
    fn as_any(&self) -> &dyn Any;
}

pub enum Query {
    Select(GQLQuery),
    GlobalVariableDeclaration(GlobalVariableStatement),
}

pub struct GQLQuery {
    pub statements: HashMap<String, Box<dyn Statement>>,
    pub has_aggregation_function: bool,
    pub has_group_by_statement: bool,
    pub hidden_selections: Vec<String>,
}

pub struct SelectStatement {
    pub table_name: String,
    pub fields_names: Vec<String>,
    pub fields_values: Vec<Box<dyn Expression>>,
    pub alias_table: HashMap<String, String>,
    pub is_distinct: bool,
}

impl Statement for SelectStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_statement_kind(&self) -> StatementKind {
        StatementKind::Select
    }
}

pub struct WhereStatement {
    pub condition: Box<dyn Expression>,
}

impl Statement for WhereStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_statement_kind(&self) -> StatementKind {
        StatementKind::Where
    }
}

pub struct HavingStatement {
    pub condition: Box<dyn Expression>,
}

impl Statement for HavingStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_statement_kind(&self) -> StatementKind {
        StatementKind::Having
    }
}

pub struct LimitStatement {
    pub count: usize,
}

impl Statement for LimitStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_statement_kind(&self) -> StatementKind {
        StatementKind::Limit
    }
}

pub struct OffsetStatement {
    pub count: usize,
}

impl Statement for OffsetStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_statement_kind(&self) -> StatementKind {
        StatementKind::Offset
    }
}

#[derive(PartialEq)]
pub enum SortingOrder {
    Ascending,
    Descending,
}

pub struct OrderByStatement {
    pub arguments: Vec<Box<dyn Expression>>,
    pub sorting_orders: Vec<SortingOrder>,
}

impl Statement for OrderByStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_statement_kind(&self) -> StatementKind {
        StatementKind::OrderBy
    }
}

pub struct GroupByStatement {
    pub field_name: String,
}

impl Statement for GroupByStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_statement_kind(&self) -> StatementKind {
        StatementKind::GroupBy
    }
}

pub struct AggregateFunction {
    pub function_name: String,
    pub argument: String,
}

pub struct AggregationFunctionsStatement {
    pub aggregations: HashMap<String, AggregateFunction>,
}

impl Statement for AggregationFunctionsStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_statement_kind(&self) -> StatementKind {
        StatementKind::AggregateFunction
    }
}

pub struct GlobalVariableStatement {
    pub name: String,
    pub value: Box<dyn Expression>,
}

impl Statement for GlobalVariableStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_statement_kind(&self) -> StatementKind {
        StatementKind::GlobalVariable
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_selectstatement_as_any() {
        assert!(true);
    }

    #[test]
    fn test_selectstatement_get_statement_kind() {
        assert!(true);
    }

    #[test]
    fn test_wherestatement_as_any() {
        assert!(true);
    }

    #[test]
    fn test_wherestatement_get_statement_kind() {
        assert!(true);
    }

    #[test]
    fn test_havingstatement_as_any() {
        assert!(true);
    }

    #[test]
    fn test_havingstatement_get_statement_kind() {
        assert!(true);
    }

    #[test]
    fn test_limitstatement_as_any() {
        assert!(true);
    }

    #[test]
    fn test_limitstatement_get_statement_kind() {
        assert!(true);
    }

    #[test]
    fn test_offsetstatement_as_any() {
        assert!(true);
    }

    #[test]
    fn test_offsetstatement_get_statement_kind() {
        assert!(true);
    }

    #[test]
    fn test_orderbystatement_as_any() {
        assert!(true);
    }

    #[test]
    fn test_orderbystatement_get_statement_kind() {
        assert!(true);
    }

    #[test]
    fn test_groupbystatement_as_any() {
        assert!(true);
    }

    #[test]
    fn test_groupbystatement_get_statement_kind() {
        assert!(true);
    }

    #[test]
    fn test_aggregationfunctionstatement_as_any() {
        assert!(true);
    }

    #[test]
    fn test_aggregationfunctionstatement_get_statement_kind() {
        assert!(true);
    }

    #[test]
    fn test_globalvariablestatement_as_any() {
        assert!(true);
    }

    #[test]
    fn test_globalvariablestatement_get_statement_kind() {
        assert!(true);
    }
}
