//! Exit-if mapping feature for loop-cond bodies.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::parts;
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::MirBuilder;
use std::collections::BTreeMap;

/// Delegate to parts::if_exit (SSOT for exit-if lowering).
pub(in crate::mir::builder) fn lower_if_exit_stmt(
    builder: &mut MirBuilder,
    current_bindings: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    condition: &ASTNode,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    parts::if_exit::lower_if_exit_stmt(
        builder,
        current_bindings,
        carrier_step_phis,
        condition,
        then_body,
        else_body,
        error_prefix,
    )
}

/// Delegate to parts::if_exit (SSOT for exit-if lowering with break PHI args).
pub(in crate::mir::builder) fn lower_if_exit_stmt_with_break_phi_args(
    builder: &mut MirBuilder,
    current_bindings: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    condition: &ASTNode,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    parts::if_exit::lower_if_exit_stmt_with_break_phi_args(
        builder,
        current_bindings,
        carrier_step_phis,
        break_phi_dsts,
        condition,
        then_body,
        else_body,
        error_prefix,
    )
}
