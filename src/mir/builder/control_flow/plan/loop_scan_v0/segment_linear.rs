use crate::mir::builder::control_flow::plan::facts::exit_only_block::ExitAllowedBlockRecipe;
use crate::mir::builder::control_flow::plan::parts;
use crate::mir::builder::control_flow::plan::recipe_tree::verified::VerifiedRecipeBlock;
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::MirBuilder;
use std::collections::BTreeMap;

const LOOP_SCAN_ERR: &str = "[normalizer] loop_scan_v0";

fn verify_loop_scan_v0_linear_segment<'a>(
    exit_allowed: &'a ExitAllowedBlockRecipe,
    current_bindings: &BTreeMap<String, crate::mir::ValueId>,
) -> Result<VerifiedRecipeBlock<'a>, String> {
    parts::entry::verify_exit_allowed_block_with_pre(
        &exit_allowed.arena,
        &exit_allowed.block,
        LOOP_SCAN_ERR,
        Some(current_bindings),
    )
}

fn lower_loop_scan_v0_linear_segment_verified(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    verified: VerifiedRecipeBlock<'_>,
) -> Result<Vec<LoweredRecipe>, String> {
    parts::entry::lower_exit_allowed_block_verified(
        builder,
        current_bindings,
        carrier_step_phis,
        break_phi_dsts,
        verified,
        LOOP_SCAN_ERR,
    )
}

pub(in crate::mir::builder) fn lower_loop_scan_v0_linear_segment(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    exit_allowed: &ExitAllowedBlockRecipe,
) -> Result<Vec<LoweredRecipe>, String> {
    let verified = verify_loop_scan_v0_linear_segment(exit_allowed, current_bindings)?;
    lower_loop_scan_v0_linear_segment_verified(
        builder,
        current_bindings,
        carrier_step_phis,
        break_phi_dsts,
        verified,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, LiteralValue, Span};
    use crate::mir::builder::control_flow::plan::facts::exit_only_block::try_build_exit_allowed_block_recipe;
    use crate::mir::builder::control_flow::plan::recipe_tree::BlockContractKind;

    fn span() -> Span {
        Span::unknown()
    }

    #[test]
    fn loop_scan_v0_linear_segment_verifies_simple_exit_allowed_slice() {
        let current_bindings = BTreeMap::new();
        let stmts = vec![ASTNode::Return {
            value: Some(Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: span(),
            })),
            span: span(),
        }];
        let exit_allowed =
            try_build_exit_allowed_block_recipe(&stmts, true).expect("exit-allowed recipe");

        let verified = verify_loop_scan_v0_linear_segment(&exit_allowed, &current_bindings)
            .expect("simple exit-allowed slice should verify");

        assert_eq!(verified.kind(), BlockContractKind::ExitAllowed);
    }

    #[test]
    fn loop_scan_v0_linear_segment_lowers_simple_exit_allowed_slice() {
        let mut builder = MirBuilder::new();
        let mut current_bindings = BTreeMap::new();
        let carrier_step_phis = BTreeMap::new();
        let break_phi_dsts = BTreeMap::new();
        let stmts = vec![ASTNode::Return {
            value: Some(Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: span(),
            })),
            span: span(),
        }];
        let exit_allowed =
            try_build_exit_allowed_block_recipe(&stmts, true).expect("exit-allowed recipe");

        let plans = lower_loop_scan_v0_linear_segment(
            &mut builder,
            &mut current_bindings,
            &carrier_step_phis,
            &break_phi_dsts,
            &exit_allowed,
        )
        .expect("simple exit-allowed slice should lower");

        assert!(!plans.is_empty());
    }
}
