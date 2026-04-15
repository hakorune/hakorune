//! Facts for loop_collect_using_entries_v0 (one-shape, planner-required only).

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::loop_collect_using_entries_v0_helpers::{
    is_loop_cond_var_lt_var, release_enabled,
};
use crate::mir::builder::control_flow::facts::loop_collect_using_entries_v0_recipe_builder::try_build_loop_collect_using_entries_v0_recipe;
use crate::mir::builder::control_flow::facts::loop_collect_using_entries_v0_shape_routes::try_match_loop_collect_using_entries_v0_shape;
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::builder::control_flow::recipes::loop_collect_using_entries_v0::LoopCollectUsingEntriesV0Recipe;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct LoopCollectUsingEntriesV0Facts {
    pub loop_var: String,
    pub limit_var: String,
    pub condition: ASTNode,
    pub recipe: LoopCollectUsingEntriesV0Recipe,
}

pub(in crate::mir::builder) fn try_extract_loop_collect_using_entries_v0_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<LoopCollectUsingEntriesV0Facts>, Freeze> {
    let debug = crate::config::env::joinir_dev::debug_enabled();
    let debug_reject = |reason: &str| {
        if debug {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[plan/reject_detail] box=loop_collect_using_entries_v0 reason={}",
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

    match try_match_loop_collect_using_entries_v0_shape(body, &loop_var) {
        Ok(()) => {}
        Err(reason) => {
            debug_reject(&reason);
            return Ok(None);
        }
    };

    let recipe = match try_build_loop_collect_using_entries_v0_recipe(body) {
        Some(recipe) => recipe,
        None => {
            debug_reject("no_exit_recipe_failed");
            return Ok(None);
        }
    };

    Ok(Some(LoopCollectUsingEntriesV0Facts {
        loop_var,
        limit_var,
        condition: condition.clone(),
        recipe,
    }))
}

#[cfg(test)]
mod tests {
    use super::try_extract_loop_collect_using_entries_v0_facts;
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
    fn accepts_loop_collect_using_entries_v0_shape() {
        std::env::set_var("NYASH_JOINIR_DEV", "1");
        std::env::set_var("HAKO_JOINIR_PLANNER_REQUIRED", "1");

        let condition = binop(BinaryOperator::Less, var("pos"), var("n"));
        let body = vec![
            local("next_pos", Some(int(0))),
            local("entry", Some(int(0))),
            ASTNode::If {
                condition: Box::new(binop(BinaryOperator::Less, var("pos"), int(1))),
                then_body: vec![
                    assign(var("entry"), int(1)),
                    assign(var("next_pos"), var("pos")),
                ],
                else_body: Some(vec![
                    assign(var("entry"), int(2)),
                    assign(var("next_pos"), var("pos")),
                ]),
                span: Span::unknown(),
            },
            assign(var("pos"), var("next_pos")),
        ];

        let facts = try_extract_loop_collect_using_entries_v0_facts(&condition, &body)
            .expect("extract ok")
            .expect("facts");

        assert_eq!(facts.loop_var, "pos");
        assert_eq!(facts.limit_var, "n");
    }

    #[test]
    fn rejects_loop_collect_using_entries_v0_without_top_level_if_else() {
        std::env::set_var("NYASH_JOINIR_DEV", "1");
        std::env::set_var("HAKO_JOINIR_PLANNER_REQUIRED", "1");

        let condition = binop(BinaryOperator::Less, var("pos"), var("n"));
        let body = vec![
            local("next_pos", Some(int(0))),
            local("entry", Some(int(0))),
            assign(var("entry"), int(1)),
            assign(var("pos"), var("next_pos")),
        ];

        let facts =
            try_extract_loop_collect_using_entries_v0_facts(&condition, &body).expect("extract ok");

        assert!(facts.is_none());
    }
}
