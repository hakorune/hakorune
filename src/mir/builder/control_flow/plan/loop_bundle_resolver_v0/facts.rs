//! Facts for loop_bundle_resolver_v0 (one-shape, planner-required only).

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::planner::Freeze;

use super::facts_helpers::release_enabled;
use super::facts_recipe_builder::try_build_loop_bundle_resolver_v0_facts;
use super::facts_shape_routes::try_match_loop_bundle_resolver_v0_shape_pins;
use super::facts_types::LoopBundleResolverV0Facts;

pub(in crate::mir::builder) fn try_extract_loop_bundle_resolver_v0_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<LoopBundleResolverV0Facts>, Freeze> {
    let debug = crate::config::env::joinir_dev::debug_enabled();
    let debug_reject = |reason: &str| {
        if debug {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[plan/reject_detail] box=loop_bundle_resolver_v0 reason={}",
                reason
            ));
        }
    };

    if !release_enabled() {
        debug_reject("planner_required_off");
        return Ok(None);
    }

    let Some(shape_pins) =
        try_match_loop_bundle_resolver_v0_shape_pins(condition, body, &debug_reject)
    else {
        return Ok(None);
    };

    Ok(try_build_loop_bundle_resolver_v0_facts(
        condition,
        body,
        shape_pins,
        &debug_reject,
    ))
}

#[cfg(test)]
mod tests {
    use super::try_extract_loop_bundle_resolver_v0_facts;
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};

    fn var(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    fn int(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    fn binop(operator: BinaryOperator, left: ASTNode, right: ASTNode) -> ASTNode {
        ASTNode::BinaryOp {
            operator,
            left: Box::new(left),
            right: Box::new(right),
            span: Span::unknown(),
        }
    }

    fn assign(target: ASTNode, value: ASTNode) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(target),
            value: Box::new(value),
            span: Span::unknown(),
        }
    }

    fn local(name: &str, init: Option<ASTNode>) -> ASTNode {
        ASTNode::Local {
            variables: vec![name.to_string()],
            initial_values: vec![init.map(Box::new)],
            span: Span::unknown(),
        }
    }

    #[test]
    fn extracts_exit_allowed_recipe_for_loop_bundle_resolver_v0() {
        std::env::set_var("NYASH_JOINIR_DEV", "1");
        std::env::set_var("HAKO_JOINIR_PLANNER_REQUIRED", "1");

        let condition = binop(BinaryOperator::Less, var("i"), var("n"));
        let body = vec![
            local("next_i", Some(int(0))),
            ASTNode::If {
                condition: Box::new(binop(BinaryOperator::Less, var("i"), int(1))),
                then_body: vec![ASTNode::Return {
                    value: Some(Box::new(int(0))),
                    span: Span::unknown(),
                }],
                else_body: None,
                span: Span::unknown(),
            },
            assign(var("i"), var("next_i")),
        ];

        let facts = try_extract_loop_bundle_resolver_v0_facts(&condition, &body)
            .expect("extract ok")
            .expect("facts");

        assert_eq!(facts.recipe.step_var, "next_i");
    }
}
