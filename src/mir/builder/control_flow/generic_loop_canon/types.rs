use crate::ast::BinaryOperator;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct UpdateCanon {
    pub op: BinaryOperator,
    pub step: i64,
    pub commuted: bool,
}
