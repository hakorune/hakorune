//! Phase 29aj P4: loop_continue_only facts

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::builder::control_flow::facts::extractors::common_helpers::{
    count_control_flow, extract_loop_increment_plan, has_break_statement, ControlFlowDetector,
};
use crate::mir::builder::control_flow::facts::stmt_view::{
    LoopSourceBodySiteV1, LoopSourceProjectionV1,
};
use crate::mir::builder::control_flow::generic_loop_canon::canon_update_for_loop_var;
use crate::mir::builder::control_flow::plan::planner::Freeze;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct LoopContinueOnlyFacts {
    pub loop_var: String,
    pub condition: ASTNode,
    pub continue_condition: ASTNode,
    pub carrier_updates: BTreeMap<String, ASTNode>,
    pub loop_increment: ASTNode,
    pub source_topology: Option<LoopContinueOnlySourceTopologyV1>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct LoopContinueOnlySourceTopologyV1 {
    schedule: Box<[LoopContinueOnlyObservedStmtV1]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct LoopContinueOnlyObservedStmtV1 {
    site: LoopSourceBodySiteV1,
    role: LoopContinueOnlyObservedStmtRoleV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum LoopContinueOnlyObservedStmtRoleV1 {
    ContinueIf,
    CarrierUpdate { carrier: String },
    Step,
}

pub(in crate::mir::builder) fn try_extract_loop_continue_only_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<LoopContinueOnlyFacts>, Freeze> {
    try_extract_loop_continue_only_facts_with_projection(
        condition,
        body,
        &LoopSourceProjectionV1::default(),
    )
}

pub(in crate::mir::builder) fn try_extract_loop_continue_only_facts_with_projection(
    condition: &ASTNode,
    body: &[ASTNode],
    source_projection: &LoopSourceProjectionV1,
) -> Result<Option<LoopContinueOnlyFacts>, Freeze> {
    let Some(loop_var) = extract_loop_var_for_subset(condition) else {
        return Ok(None);
    };

    let continue_count = count_control_flow(body, ControlFlowDetector::default()).continue_count;
    if continue_count != 1 {
        return Ok(None);
    }

    if has_break_statement(body) {
        return Ok(None);
    }

    let (continue_if_idx, continue_condition) = match find_continue_condition(body) {
        Some(found) => found,
        None => return Ok(None),
    };
    if continue_if_idx != 0 {
        return Ok(None);
    }

    let carrier_updates = extract_carrier_updates(body, &loop_var);
    if carrier_updates.is_empty() {
        return Ok(None);
    }

    let loop_increment = match extract_loop_increment_plan(body, &loop_var) {
        Ok(Some(inc)) => inc,
        _ => return Ok(None),
    };
    let source_topology = source_topology_for(body, source_projection, &loop_var);

    Ok(Some(LoopContinueOnlyFacts {
        loop_var,
        condition: condition.clone(),
        continue_condition,
        carrier_updates,
        loop_increment,
        source_topology,
    }))
}

fn source_topology_for(
    body: &[ASTNode],
    projection: &LoopSourceProjectionV1,
    loop_var: &str,
) -> Option<LoopContinueOnlySourceTopologyV1> {
    if projection.flattened_body_len() != Some(body.len()) || body.len() < 3 {
        return None;
    }
    let mut schedule = Vec::with_capacity(body.len());
    let mut carriers = std::collections::BTreeSet::new();
    for (index, stmt) in body.iter().enumerate() {
        let role = if index == 0 && find_continue_condition(&body[..1]).is_some() {
            LoopContinueOnlyObservedStmtRoleV1::ContinueIf
        } else if index + 1 == body.len() && canon_update_for_loop_var(stmt, loop_var).is_some() {
            LoopContinueOnlyObservedStmtRoleV1::Step
        } else if let Some(carrier) = carrier_update_name(stmt, loop_var) {
            if !carriers.insert(carrier.clone()) {
                return None;
            }
            LoopContinueOnlyObservedStmtRoleV1::CarrierUpdate { carrier }
        } else {
            return None;
        };
        schedule.push(LoopContinueOnlyObservedStmtV1 {
            site: projection.site_for_flattened_index(index)?.clone(),
            role,
        });
    }
    matches!(
        schedule.first()?.role,
        LoopContinueOnlyObservedStmtRoleV1::ContinueIf
    )
    .then_some(LoopContinueOnlySourceTopologyV1 {
        schedule: schedule.into(),
    })
}

fn carrier_update_name(stmt: &ASTNode, loop_var: &str) -> Option<String> {
    let ASTNode::Assignment { target, value, .. } = stmt else {
        return None;
    };
    let ASTNode::Variable { name, .. } = target.as_ref() else {
        return None;
    };
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left,
        ..
    } = value.as_ref()
    else {
        return None;
    };
    (name != loop_var
        && matches!(left.as_ref(), ASTNode::Variable { name: lhs, .. } if lhs == name))
    .then(|| name.clone())
}

fn extract_loop_var_for_subset(condition: &ASTNode) -> Option<String> {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left,
        right,
        ..
    } = condition
    else {
        return None;
    };

    let ASTNode::Variable { name, .. } = left.as_ref() else {
        return None;
    };

    if !matches!(
        right.as_ref(),
        ASTNode::Literal {
            value: LiteralValue::Integer(_),
            ..
        }
    ) {
        return None;
    }

    Some(name.clone())
}

fn find_continue_condition(body: &[ASTNode]) -> Option<(usize, ASTNode)> {
    for (idx, stmt) in body.iter().enumerate() {
        let ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } = stmt
        else {
            continue;
        };

        let then_has_continue = then_body
            .iter()
            .any(|n| matches!(n, ASTNode::Continue { .. }));
        let else_has_continue = else_body.as_ref().map_or(false, |b| {
            b.iter().any(|n| matches!(n, ASTNode::Continue { .. }))
        });

        if else_body.is_some() && (then_has_continue || else_has_continue) {
            return None;
        }

        if then_has_continue {
            return Some((idx, condition.as_ref().clone()));
        }
    }

    None
}

fn extract_carrier_updates(body: &[ASTNode], loop_var: &str) -> BTreeMap<String, ASTNode> {
    let mut updates = BTreeMap::new();

    for stmt in body {
        if let ASTNode::Assignment { target, value, .. } = stmt {
            let ASTNode::Variable { name, .. } = target.as_ref() else {
                continue;
            };
            if name == loop_var {
                continue;
            }
            let ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left,
                ..
            } = value.as_ref()
            else {
                continue;
            };
            if matches!(left.as_ref(), ASTNode::Variable { name: lhs, .. } if lhs == name) {
                updates.insert(name.clone(), value.as_ref().clone());
            }
        }
    }

    updates
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Span;

    fn v(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    fn lit_int(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    fn condition_lt(var: &str, bound: i64) -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(v(var)),
            right: Box::new(lit_int(bound)),
            span: Span::unknown(),
        }
    }

    fn increment(var: &str) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(v(var)),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(v(var)),
                right: Box::new(lit_int(1)),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }
    }

    fn carrier_update(var: &str, rhs: ASTNode) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(v(var)),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(v(var)),
                right: Box::new(rhs),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }
    }

    fn if_continue(cond: ASTNode) -> ASTNode {
        ASTNode::If {
            condition: Box::new(cond),
            then_body: vec![ASTNode::Continue {
                span: Span::unknown(),
            }],
            else_body: None,
            span: Span::unknown(),
        }
    }

    #[test]
    fn facts_extracts_loop_continue_only_success() {
        let condition = condition_lt("i", 6);
        let body = vec![
            if_continue(v("skip")),
            carrier_update("sum", v("i")),
            increment("i"),
        ];

        let facts = try_extract_loop_continue_only_facts(&condition, &body).expect("Ok");
        let facts = facts.expect("Some");
        assert_eq!(facts.loop_var, "i");
        assert_eq!(facts.carrier_updates.len(), 1);
    }

    #[test]
    fn facts_rejects_break() {
        let condition = condition_lt("i", 6);
        let body = vec![
            ASTNode::If {
                condition: Box::new(v("done")),
                then_body: vec![ASTNode::Break {
                    span: Span::unknown(),
                }],
                else_body: None,
                span: Span::unknown(),
            },
            if_continue(v("skip")),
            carrier_update("sum", v("i")),
            increment("i"),
        ];

        let facts = try_extract_loop_continue_only_facts(&condition, &body).expect("Ok");
        assert!(facts.is_none());
    }

    #[test]
    fn facts_rejects_missing_continue() {
        let condition = condition_lt("i", 6);
        let body = vec![carrier_update("sum", v("i")), increment("i")];

        let facts = try_extract_loop_continue_only_facts(&condition, &body).expect("Ok");
        assert!(facts.is_none());
    }

    #[test]
    fn facts_rejects_multiple_continue_sites() {
        let condition = condition_lt("i", 6);
        let body = vec![
            if_continue(v("skip_one")),
            if_continue(v("skip_two")),
            carrier_update("sum", v("i")),
            increment("i"),
        ];

        let facts = try_extract_loop_continue_only_facts(&condition, &body).expect("Ok");
        assert!(facts.is_none());
    }
}
