use crate::mir::builder::control_flow::plan::parts;
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::MirBuilder;
use std::collections::BTreeMap;

use super::recipe::LinearBlockRecipe;

const LOOP_SCAN_METHODS_BLOCK_ERR: &str = "[normalizer] loop_scan_methods_block_v0";

pub(in crate::mir::builder) fn lower_loop_scan_methods_block_linear_segment(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    linear: &LinearBlockRecipe,
) -> Result<Vec<LoweredRecipe>, String> {
    match linear {
        LinearBlockRecipe::NoExit(recipe) => {
            let verified = parts::entry::verify_no_exit_block_with_pre(
                &recipe.arena,
                &recipe.block,
                LOOP_SCAN_METHODS_BLOCK_ERR,
                Some(current_bindings),
            )?;
            parts::entry::lower_no_exit_block_verified(
                builder,
                current_bindings,
                carrier_step_phis,
                Some(break_phi_dsts),
                verified,
                LOOP_SCAN_METHODS_BLOCK_ERR,
            )
        }
        LinearBlockRecipe::ExitAllowed(recipe) => {
            let verified = parts::entry::verify_exit_allowed_block_with_pre(
                &recipe.arena,
                &recipe.block,
                LOOP_SCAN_METHODS_BLOCK_ERR,
                Some(current_bindings),
            )?;
            parts::entry::lower_exit_allowed_block_verified(
                builder,
                current_bindings,
                carrier_step_phis,
                break_phi_dsts,
                verified,
                LOOP_SCAN_METHODS_BLOCK_ERR,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, LiteralValue, Span};
    use crate::mir::builder::control_flow::plan::facts::exit_only_block::try_build_exit_allowed_block_recipe;

    fn span() -> Span {
        Span::unknown()
    }

    #[test]
    fn loop_scan_methods_block_linear_segment_lowers_simple_exit_allowed_slice() {
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
        let recipe =
            try_build_exit_allowed_block_recipe(&stmts, true).expect("exit-allowed block recipe");

        let plans = lower_loop_scan_methods_block_linear_segment(
            &mut builder,
            &mut current_bindings,
            &carrier_step_phis,
            &break_phi_dsts,
            &LinearBlockRecipe::ExitAllowed(recipe),
        )
        .expect("simple exit-allowed slice should lower");

        assert!(!plans.is_empty());
    }
}
