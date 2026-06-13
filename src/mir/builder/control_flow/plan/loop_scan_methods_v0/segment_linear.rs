use crate::mir::builder::control_flow::plan::parts;
use crate::mir::builder::control_flow::plan::recipe_tree::VerifiedRecipeBlock;
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::control_flow::recipes::loop_scan_methods_v0::LinearBlockRecipe;
use crate::mir::builder::MirBuilder;
use std::collections::BTreeMap;

const LOOP_SCAN_METHODS_ERR: &str = "[normalizer] loop_scan_methods_v0";

fn verify_loop_scan_methods_linear_segment<'a>(
    linear: &'a LinearBlockRecipe,
    current_bindings: &BTreeMap<String, crate::mir::ValueId>,
) -> Result<VerifiedRecipeBlock<'a>, String> {
    match linear {
        LinearBlockRecipe::NoExit(recipe) => parts::entry::verify_no_exit_block_with_pre(
            &recipe.arena,
            &recipe.block,
            LOOP_SCAN_METHODS_ERR,
            Some(current_bindings),
        ),
        LinearBlockRecipe::ExitAllowed(recipe) => parts::entry::verify_exit_allowed_block_with_pre(
            &recipe.arena,
            &recipe.block,
            LOOP_SCAN_METHODS_ERR,
            Some(current_bindings),
        ),
    }
}

fn lower_loop_scan_methods_linear_segment_verified(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    verified: VerifiedRecipeBlock<'_>,
) -> Result<Vec<LoweredRecipe>, String> {
    match verified.kind() {
        crate::mir::builder::control_flow::plan::recipe_tree::BlockContractKind::NoExit => {
            parts::entry::lower_no_exit_block_verified(
                builder,
                current_bindings,
                carrier_step_phis,
                Some(break_phi_dsts),
                verified,
                LOOP_SCAN_METHODS_ERR,
            )
        }
        crate::mir::builder::control_flow::plan::recipe_tree::BlockContractKind::ExitAllowed => {
            parts::entry::lower_exit_allowed_block_verified(
                builder,
                current_bindings,
                carrier_step_phis,
                break_phi_dsts,
                verified,
                LOOP_SCAN_METHODS_ERR,
            )
        }
        other => Err(format!(
            "[freeze:contract][loop_scan_methods_v0] unexpected linear block kind {:?}: ctx={}",
            other, LOOP_SCAN_METHODS_ERR
        )),
    }
}

pub(in crate::mir::builder) fn lower_loop_scan_methods_linear_segment(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    linear: &LinearBlockRecipe,
) -> Result<Vec<LoweredRecipe>, String> {
    let verified = verify_loop_scan_methods_linear_segment(linear, current_bindings)?;
    lower_loop_scan_methods_linear_segment_verified(
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
    use crate::mir::builder::control_flow::facts::no_exit_block::try_build_no_exit_block_recipe;
    use crate::mir::builder::control_flow::plan::facts::exit_only_block::try_build_exit_allowed_block_recipe;
    use crate::mir::builder::control_flow::plan::recipe_tree::BlockContractKind;

    #[test]
    fn loop_scan_methods_linear_segment_verifies_simple_no_exit_slice() {
        let current_bindings = BTreeMap::new();
        let stmts = vec![ASTNode::Assignment {
            target: Box::new(ASTNode::Variable {
                name: "x".to_string(),
                span: Span::unknown(),
            }),
            value: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }];
        let no_exit = try_build_no_exit_block_recipe(&stmts, true).expect("no-exit recipe");
        let linear = LinearBlockRecipe::NoExit(no_exit);

        let verified = verify_loop_scan_methods_linear_segment(&linear, &current_bindings)
            .expect("simple no-exit slice should verify");

        assert_eq!(verified.kind(), BlockContractKind::NoExit);
    }

    #[test]
    fn loop_scan_methods_linear_segment_lowers_simple_no_exit_slice() {
        let mut builder = MirBuilder::new();
        let mut current_bindings = BTreeMap::new();
        let carrier_step_phis = BTreeMap::new();
        let break_phi_dsts = BTreeMap::new();
        let stmts = vec![ASTNode::Assignment {
            target: Box::new(ASTNode::Variable {
                name: "x".to_string(),
                span: Span::unknown(),
            }),
            value: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }];
        let no_exit = try_build_no_exit_block_recipe(&stmts, true).expect("no-exit recipe");

        let plans = lower_loop_scan_methods_linear_segment(
            &mut builder,
            &mut current_bindings,
            &carrier_step_phis,
            &break_phi_dsts,
            &LinearBlockRecipe::NoExit(no_exit),
        )
        .expect("simple no-exit slice should lower");

        assert_eq!(plans.len(), 1);
    }

    #[test]
    fn loop_scan_methods_linear_segment_lowers_simple_exit_allowed_slice() {
        let mut builder = MirBuilder::new();
        let mut current_bindings = BTreeMap::new();
        let carrier_step_phis = BTreeMap::new();
        let break_phi_dsts = BTreeMap::new();
        let stmts = vec![ASTNode::Return {
            value: Some(Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            })),
            span: Span::unknown(),
        }];
        let recipe =
            try_build_exit_allowed_block_recipe(&stmts, true).expect("exit-allowed recipe");

        let plans = lower_loop_scan_methods_linear_segment(
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
