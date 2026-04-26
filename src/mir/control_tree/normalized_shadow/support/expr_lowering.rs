//! Shared expression lowering facade for normalized-shadow routes.
//!
//! Route lowerers import this module instead of the legacy entry owner. The
//! implementation still delegates during the physical split, but the semantic
//! dependency now points at a support owner.

use std::collections::BTreeMap;

use crate::ast::ASTNode;
use crate::mir::control_tree::normalized_shadow::legacy::LegacyLowerer;
use crate::mir::control_tree::step_tree::AstNodeHandle;
use crate::mir::join_ir::{CompareOp, JoinInst};
use crate::mir::ValueId;

pub(crate) fn lower_assign_stmt(
    target: &Option<String>,
    value_ast: &Option<AstNodeHandle>,
    body: &mut Vec<JoinInst>,
    next_value_id: &mut u32,
    env: &mut BTreeMap<String, ValueId>,
) -> Result<(), String> {
    LegacyLowerer::lower_assign_stmt(target, value_ast, body, next_value_id, env)
}

pub(crate) fn parse_minimal_compare(ast: &ASTNode) -> Result<(String, CompareOp, i64), String> {
    LegacyLowerer::parse_minimal_compare(ast)
}
