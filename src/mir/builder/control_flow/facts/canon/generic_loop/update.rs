use crate::ast::{ASTNode, BinaryOperator, LiteralValue};

use super::types::UpdateCanon;

mod literal_match;
mod literal_step;

#[derive(Debug, Clone, PartialEq)]
pub(super) struct UpdateLiteralMatch {
    pub op: BinaryOperator,
    pub literal: LiteralValue,
    pub commuted: bool,
}

pub(crate) fn canon_update_for_loop_var(stmt: &ASTNode, loop_var: &str) -> Option<UpdateCanon> {
    let matched = literal_match::match_update_literal(stmt, loop_var)?;
    literal_step::build_update_canon(matched)
}
