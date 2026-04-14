use crate::mir::builder::control_flow::plan::facts::no_exit_block::NoExitBlockRecipe;
use crate::mir::builder::control_flow::plan::parts;
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::MirBuilder;
use std::collections::BTreeMap;

const LOOP_SCAN_METHODS_ERR: &str = "[normalizer] loop_scan_methods_v0";

pub(in crate::mir::builder) fn lower_loop_scan_methods_linear_segment(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    no_exit: &NoExitBlockRecipe,
) -> Result<Vec<LoweredRecipe>, String> {
    let verified = parts::entry::verify_no_exit_block_with_pre(
        &no_exit.arena,
        &no_exit.block,
        LOOP_SCAN_METHODS_ERR,
        Some(current_bindings),
    )?;
    parts::entry::lower_no_exit_block_verified(
        builder,
        current_bindings,
        carrier_step_phis,
        Some(break_phi_dsts),
        verified,
        LOOP_SCAN_METHODS_ERR,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, LiteralValue, Span};
    use crate::mir::builder::control_flow::plan::facts::no_exit_block::try_build_no_exit_block_recipe;

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
            &no_exit,
        )
        .expect("simple no-exit slice should lower");

        assert_eq!(plans.len(), 1);
    }
}
