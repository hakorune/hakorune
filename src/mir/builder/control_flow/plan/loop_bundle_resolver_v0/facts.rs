//! Facts for loop_bundle_resolver_v0 (one-shape, planner-required only).

use crate::ast::{ASTNode, BinaryOperator};
use crate::mir::builder::control_flow::plan::facts::exit_only_block::try_build_exit_allowed_block_recipe;
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::policies::BodyLoweringPolicy;

use super::recipe::LoopBundleResolverV0Recipe;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct LoopBundleResolverV0Facts {
    pub loop_var: String,
    pub limit_var: String,
    pub condition: ASTNode,
    pub body_lowering_policy: BodyLoweringPolicy,
    pub recipe: LoopBundleResolverV0Recipe,
}

fn release_enabled() -> bool {
    true
}

fn as_var_name(ast: &ASTNode) -> Option<&str> {
    match ast {
        ASTNode::Variable { name, .. } => Some(name),
        _ => None,
    }
}

fn is_loop_cond_var_lt_var(ast: &ASTNode) -> Option<(String, String)> {
    match ast {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left,
            right,
            ..
        } => Some((
            as_var_name(left.as_ref())?.to_string(),
            as_var_name(right.as_ref())?.to_string(),
        )),
        _ => None,
    }
}

fn declares_local_var(stmt: &ASTNode, name: &str) -> bool {
    let ASTNode::Local { variables, .. } = stmt else {
        return false;
    };
    variables.iter().any(|v| v == name)
}

fn extract_step_var_from_tail(stmt: &ASTNode, loop_var: &str) -> Option<String> {
    let ASTNode::Assignment { target, value, .. } = stmt else {
        return None;
    };
    if as_var_name(target.as_ref()) != Some(loop_var) {
        return None;
    }
    Some(as_var_name(value.as_ref())?.to_string())
}

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

    let Some((loop_var, limit_var)) = is_loop_cond_var_lt_var(condition) else {
        debug_reject("cond_not_var_lt_var");
        return Ok(None);
    };

    if body.len() < 2 {
        debug_reject("body_too_short");
        return Ok(None);
    }

    let Some(last) = body.last() else {
        debug_reject("body_last_missing");
        return Ok(None);
    };
    let Some(step_var) = extract_step_var_from_tail(last, &loop_var) else {
        debug_reject("tail_not_loopvar_eq_stepvar");
        return Ok(None);
    };

    // One-shape pin: step var must be introduced as a loop-local (typically the first stmt).
    let Some(first) = body.first() else {
        debug_reject("body_first_missing");
        return Ok(None);
    };
    if !declares_local_var(first, &step_var) {
        debug_reject("step_var_not_declared_as_first_local");
        return Ok(None);
    }

    // Distinguish from scan loops: this shape is exit-bearing (nested return).
    if !body.iter().any(ASTNode::contains_return_stmt) {
        debug_reject("no_return_in_body");
        return Ok(None);
    }

    let Some(body_exit_allowed) = try_build_exit_allowed_block_recipe(body, true) else {
        debug_reject("exit_allowed_recipe_failed");
        return Ok(None);
    };

    Ok(Some(LoopBundleResolverV0Facts {
        loop_var,
        limit_var,
        condition: condition.clone(),
        body_lowering_policy: BodyLoweringPolicy::ExitAllowed {
            allow_join_if: false,
        },
        recipe: LoopBundleResolverV0Recipe {
            step_var,
            body_exit_allowed,
        },
    }))
}

#[cfg(test)]
mod tests {
    use super::try_extract_loop_bundle_resolver_v0_facts;
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
    use crate::mir::policies::BodyLoweringPolicy;

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
    fn policy_exit_allowed_for_loop_bundle_resolver_v0() {
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

        assert!(matches!(
            facts.body_lowering_policy,
            BodyLoweringPolicy::ExitAllowed { .. }
        ));
    }
}
