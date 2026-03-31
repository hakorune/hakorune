use crate::mir::builder::control_flow::plan::recipe_tree::{RecipeBlock, RecipeBodies};
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::MirBuilder;
use std::collections::BTreeMap;

use super::LoopBodyContractKind;
use super::super::{entry, stmt as parts_stmt, verify};

pub(in crate::mir::builder) fn lower_loop_with_body_block(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    arena: &RecipeBodies,
    body_block: &RecipeBlock,
    contract: LoopBodyContractKind,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    lower_loop_with_body_block_internal(
        builder,
        current_bindings,
        carrier_step_phis,
        None, // break_phi_dsts
        arena,
        body_block,
        contract,
        error_prefix,
    )
}

pub(in crate::mir::builder) fn lower_loop_with_body_block_with_break_phi_dsts(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    arena: &RecipeBodies,
    body_block: &RecipeBlock,
    contract: LoopBodyContractKind,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    lower_loop_with_body_block_internal(
        builder,
        current_bindings,
        carrier_step_phis,
        Some(break_phi_dsts),
        arena,
        body_block,
        contract,
        error_prefix,
    )
}

fn lower_loop_with_body_block_internal(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: Option<&BTreeMap<String, crate::mir::ValueId>>,
    arena: &RecipeBodies,
    body_block: &RecipeBlock,
    contract: LoopBodyContractKind,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    match contract {
        LoopBodyContractKind::StmtOnly => {
            let verified = entry::verify_stmt_only_block_with_pre(
                arena,
                body_block,
                error_prefix,
                Some(current_bindings),
            )?;
            entry::lower_stmt_only_block_verified(
                builder,
                current_bindings,
                carrier_step_phis,
                break_phi_dsts,
                verified,
                error_prefix,
                |builder, bindings, carrier_step_phis, break_phi_dsts, stmt, error_prefix| {
                    parts_stmt::lower_return_prelude_stmt(
                        builder,
                        bindings,
                        carrier_step_phis,
                        break_phi_dsts,
                        stmt,
                        error_prefix,
                    )
                },
            )
        }
        LoopBodyContractKind::NoExit => {
            verify::verify_no_exit_block_contract_if_enabled(arena, body_block, error_prefix)?;
            let verified = entry::verify_no_exit_block_with_pre(
                arena,
                body_block,
                error_prefix,
                Some(current_bindings),
            )?;
            entry::lower_no_exit_block_verified(
                builder,
                current_bindings,
                carrier_step_phis,
                break_phi_dsts,
                verified,
                error_prefix,
            )
        }
        LoopBodyContractKind::ExitAllowed => {
            let Some(break_phi_dsts) = break_phi_dsts else {
                return Err(format!(
                    "[freeze:contract][recipe] loop_body_contract_requires_break_phi_dsts: ctx={}",
                    error_prefix
                ));
            };
            let verified = entry::verify_exit_allowed_block_with_pre(
                arena,
                body_block,
                error_prefix,
                Some(current_bindings),
            )?;
            entry::lower_exit_allowed_block_verified(
                builder,
                current_bindings,
                carrier_step_phis,
                break_phi_dsts,
                verified,
                error_prefix,
            )
        }
        LoopBodyContractKind::ExitOnly => {
            let Some(break_phi_dsts) = break_phi_dsts else {
                return Err(format!(
                    "[freeze:contract][recipe] loop_body_contract_requires_break_phi_dsts: ctx={}",
                    error_prefix
                ));
            };
            let verified = entry::verify_exit_only_block_with_pre(
                arena,
                body_block,
                error_prefix,
                Some(current_bindings),
            )?;
            entry::lower_exit_only_block_verified(
                builder,
                current_bindings,
                carrier_step_phis,
                break_phi_dsts,
                verified,
                error_prefix,
            )
        }
    }
}

/// Lower an exit-only `RecipeBlock` in loop context.
///
/// This is a thin Parts entry (BoxShape): producers should pass an already-built exit-only block.
pub(in crate::mir::builder) fn lower_loop_with_exit_only_body_block(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    arena: &RecipeBodies,
    body_block: &RecipeBlock,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    let verified = entry::verify_exit_only_block_with_pre(
        arena,
        body_block,
        error_prefix,
        Some(current_bindings),
    )?;
    entry::lower_exit_only_block_verified(
        builder,
        current_bindings,
        carrier_step_phis,
        break_phi_dsts,
        verified,
        error_prefix,
    )
}

