//! Phase 29ap P10: nested_loop_minimal facts (SSOT)

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::builder::control_flow::facts::stmt_view::{
    LoopSourceBodySiteV1, LoopSourceProjectionV1,
};
use crate::mir::builder::control_flow::plan::facts::accum_const_loop_facts::try_extract_accum_const_loop_facts;
use crate::mir::builder::control_flow::plan::facts::loop_condition_shape::try_extract_condition_shape;
use crate::mir::builder::control_flow::plan::facts::loop_step_shape::try_extract_step_shape;
use crate::mir::builder::control_flow::plan::facts::scan_shapes::{
    scan_condition_observation, ConditionShape, StepShape,
};
use crate::mir::builder::control_flow::plan::planner::Freeze;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct NestedLoopMinimalFacts {
    pub outer_loop_var: String,
    pub outer_condition: ASTNode,
    pub outer_increment: ASTNode,
    pub inner_loop_var: String,
    pub inner_condition: ASTNode,
    pub inner_increment: ASTNode,
    pub acc_var: String,
    pub acc_update: ASTNode,
    pub inner_init_lit: i64,
    pub source_topology: Option<NestedLoopMinimalSourceTopologyV1>,
}

/// Opaque, complete outer-body schedule observed by the accepted extractor.
///
/// The nested Loop is deliberately one whole-statement site. Its condition and
/// body remain a transferred boundary, not source authority for this route.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct NestedLoopMinimalSourceTopologyV1 {
    schedule: Box<[NestedLoopMinimalObservedStmtV1]>,
}

impl NestedLoopMinimalSourceTopologyV1 {
    pub(in crate::mir::builder) fn has_scope_box_lineage(&self) -> bool {
        self.schedule
            .iter()
            .any(|observed| !observed.site.scope_box_children().is_empty())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct NestedLoopMinimalObservedStmtV1 {
    site: LoopSourceBodySiteV1,
    role: NestedLoopMinimalObservedStmtRoleV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum NestedLoopMinimalObservedStmtRoleV1 {
    InnerLocal { initializes: bool },
    InnerInitAssignment,
    OuterWrite,
    InnerLoop,
    OuterStep,
}

struct OuterBodyScanV1 {
    inner_init_lit: i64,
    outer_increment: ASTNode,
    schedule: Vec<NestedLoopMinimalObservedStmtRoleV1>,
}

pub(in crate::mir::builder) fn try_extract_nested_loop_minimal_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<NestedLoopMinimalFacts>, Freeze> {
    try_extract_nested_loop_minimal_facts_with_projection(
        condition,
        body,
        &LoopSourceProjectionV1::default(),
    )
}

pub(in crate::mir::builder) fn try_extract_nested_loop_minimal_facts_with_projection(
    condition: &ASTNode,
    body: &[ASTNode],
    source_projection: &LoopSourceProjectionV1,
) -> Result<Option<NestedLoopMinimalFacts>, Freeze> {
    let Some(outer_loop_var) = extract_loop_var_for_subset(condition) else {
        return Ok(None);
    };

    let (inner_idx, inner_loop) = match find_single_inner_loop(body) {
        Some(loop_pair) => loop_pair,
        None => return Ok(None),
    };

    let ASTNode::Loop {
        condition: inner_condition,
        body: inner_body,
        ..
    } = inner_loop
    else {
        return Ok(None);
    };

    let inner_condition_shape =
        try_extract_condition_shape(inner_condition)?.unwrap_or(ConditionShape::Unknown);
    let inner_step_shape = try_extract_step_shape(inner_body)?.unwrap_or(StepShape::Unknown);
    let inner_observation = scan_condition_observation(&inner_condition_shape, &inner_step_shape);
    let Some(inner_facts) =
        try_extract_accum_const_loop_facts(inner_condition, inner_body, &inner_observation)?
    else {
        return Ok(None);
    };

    if inner_facts.loop_var == outer_loop_var
        || inner_facts.acc_var == outer_loop_var
        || inner_facts.acc_var == inner_facts.loop_var
    {
        return Ok(None);
    }

    let Some(inner_step) =
        extract_increment_step_one(&inner_facts.loop_increment, &inner_facts.loop_var)
    else {
        return Ok(None);
    };

    let Some(acc_step) = extract_accum_add_const(&inner_facts.acc_update, &inner_facts.acc_var)
    else {
        return Ok(None);
    };

    if acc_step != 1 {
        return Ok(None);
    }

    let outer_scan = match scan_outer_body(body, inner_idx, &outer_loop_var, &inner_facts.loop_var)
    {
        Some(values) => values,
        None => return Ok(None),
    };

    if outer_scan.inner_init_lit != 0 {
        return Ok(None);
    }

    if extract_increment_step_one(&outer_scan.outer_increment, &outer_loop_var).is_none() {
        return Ok(None);
    }

    let source_topology = source_topology_for(&outer_scan.schedule, source_projection);

    Ok(Some(NestedLoopMinimalFacts {
        outer_loop_var,
        outer_condition: condition.clone(),
        outer_increment: outer_scan.outer_increment,
        inner_loop_var: inner_facts.loop_var,
        inner_condition: inner_facts.condition,
        inner_increment: inner_step,
        acc_var: inner_facts.acc_var,
        acc_update: inner_facts.acc_update,
        inner_init_lit: outer_scan.inner_init_lit,
        source_topology,
    }))
}

fn source_topology_for(
    schedule: &[NestedLoopMinimalObservedStmtRoleV1],
    projection: &LoopSourceProjectionV1,
) -> Option<NestedLoopMinimalSourceTopologyV1> {
    if projection.flattened_body_len() != Some(schedule.len()) {
        return None;
    }
    schedule
        .iter()
        .enumerate()
        .map(|(index, role)| {
            Some(NestedLoopMinimalObservedStmtV1 {
                site: projection.site_for_flattened_index(index)?.clone(),
                role: role.clone(),
            })
        })
        .collect::<Option<Vec<_>>>()
        .map(|schedule| NestedLoopMinimalSourceTopologyV1 {
            schedule: schedule.into(),
        })
}

fn find_single_inner_loop(body: &[ASTNode]) -> Option<(usize, &ASTNode)> {
    let mut found = None;
    for (idx, stmt) in body.iter().enumerate() {
        if matches!(stmt, ASTNode::Loop { .. }) {
            if found.is_some() {
                return None;
            }
            found = Some((idx, stmt));
        }
    }
    found
}

fn scan_outer_body(
    body: &[ASTNode],
    inner_idx: usize,
    outer_loop_var: &str,
    inner_loop_var: &str,
) -> Option<OuterBodyScanV1> {
    let mut inner_init_lit = None;
    let mut outer_increment = None;
    let mut outer_increment_idx = None;
    let mut schedule = Vec::with_capacity(body.len());

    for (idx, stmt) in body.iter().enumerate() {
        if matches!(stmt, ASTNode::Loop { .. }) {
            if idx != inner_idx {
                return None;
            }
            schedule.push(NestedLoopMinimalObservedStmtRoleV1::InnerLoop);
            continue;
        }

        match stmt {
            ASTNode::Local {
                variables,
                initial_values,
                ..
            } => {
                if variables.len() != 1 || variables[0] != inner_loop_var {
                    return None;
                }
                if idx > inner_idx {
                    return None;
                }
                let initializes = initial_values.get(0).is_some_and(Option::is_some);
                if let Some(Some(init)) = initial_values.get(0) {
                    let lit = extract_int_literal(init)?;
                    if inner_init_lit.replace(lit).is_some() {
                        return None;
                    }
                }
                schedule.push(NestedLoopMinimalObservedStmtRoleV1::InnerLocal { initializes });
            }
            ASTNode::Assignment { target, value, .. } => {
                let ASTNode::Variable { name, .. } = target.as_ref() else {
                    return None;
                };
                if name == inner_loop_var {
                    if idx > inner_idx {
                        return None;
                    }
                    let lit = extract_int_literal(value)?;
                    if inner_init_lit.replace(lit).is_some() {
                        return None;
                    }
                    schedule.push(NestedLoopMinimalObservedStmtRoleV1::InnerInitAssignment);
                } else if name == outer_loop_var {
                    outer_increment_idx = Some(idx);
                    outer_increment = Some(value.as_ref().clone());
                    schedule.push(NestedLoopMinimalObservedStmtRoleV1::OuterWrite);
                } else {
                    return None;
                }
            }
            _ => return None,
        }
    }

    let inner_init_lit = inner_init_lit?;
    let outer_increment = outer_increment?;
    let outer_increment_idx = outer_increment_idx?;

    if outer_increment_idx <= inner_idx {
        return None;
    }
    if outer_increment_idx + 1 != body.len() {
        return None;
    }
    if schedule.len() != body.len() {
        return None;
    }
    schedule[outer_increment_idx] = NestedLoopMinimalObservedStmtRoleV1::OuterStep;

    Some(OuterBodyScanV1 {
        inner_init_lit,
        outer_increment,
        schedule,
    })
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

fn extract_int_literal(node: &ASTNode) -> Option<i64> {
    match node {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            ..
        } => Some(*value),
        _ => None,
    }
}

fn extract_increment_step_one(value: &ASTNode, loop_var: &str) -> Option<ASTNode> {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left,
        right,
        ..
    } = value
    else {
        return None;
    };

    let ASTNode::Variable { name, .. } = left.as_ref() else {
        return None;
    };
    if name != loop_var {
        return None;
    }

    if !matches!(
        right.as_ref(),
        ASTNode::Literal {
            value: LiteralValue::Integer(1),
            ..
        }
    ) {
        return None;
    }

    Some(value.clone())
}

fn extract_accum_add_const(update: &ASTNode, acc_var: &str) -> Option<i64> {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left,
        right,
        ..
    } = update
    else {
        return None;
    };

    let ASTNode::Variable { name, .. } = left.as_ref() else {
        return None;
    };
    if name != acc_var {
        return None;
    }

    extract_int_literal(right)
}

#[cfg(test)]
mod tests {
    use super::{
        try_extract_nested_loop_minimal_facts,
        try_extract_nested_loop_minimal_facts_with_projection, NestedLoopMinimalObservedStmtRoleV1,
    };
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
    use crate::mir::builder::control_flow::facts::stmt_view::flatten_scope_boxes_with_projection;

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

    fn condition_lt(loop_var: &str, bound: i64) -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(v(loop_var)),
            right: Box::new(lit_int(bound)),
            span: Span::unknown(),
        }
    }

    fn increment(loop_var: &str, step: i64) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(v(loop_var)),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(v(loop_var)),
                right: Box::new(lit_int(step)),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }
    }

    fn accum_const(acc_var: &str, step: i64) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(v(acc_var)),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(v(acc_var)),
                right: Box::new(lit_int(step)),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }
    }

    fn inner_loop() -> ASTNode {
        ASTNode::Loop {
            condition: Box::new(condition_lt("j", 3)),
            body: vec![accum_const("sum", 1), increment("j", 1)],
            span: Span::unknown(),
        }
    }

    fn variable_outer_schedule() -> Vec<ASTNode> {
        vec![
            ASTNode::Local {
                variables: vec!["j".to_string()],
                initial_values: vec![None],
                declared_type_names: Vec::new(),
                span: Span::unknown(),
            },
            ASTNode::Local {
                variables: vec!["j".to_string()],
                initial_values: vec![None],
                declared_type_names: Vec::new(),
                span: Span::unknown(),
            },
            ASTNode::Assignment {
                target: Box::new(v("j")),
                value: Box::new(lit_int(0)),
                span: Span::unknown(),
            },
            ASTNode::Assignment {
                target: Box::new(v("i")),
                value: Box::new(lit_int(0)),
                span: Span::unknown(),
            },
            inner_loop(),
            increment("i", 1),
        ]
    }

    #[test]
    fn facts_extracts_nested_minimal_subset() {
        let body = vec![
            ASTNode::Local {
                variables: vec!["j".to_string()],
                initial_values: vec![None],
                declared_type_names: Vec::new(),
                span: Span::unknown(),
            },
            ASTNode::Assignment {
                target: Box::new(v("j")),
                value: Box::new(lit_int(0)),
                span: Span::unknown(),
            },
            inner_loop(),
            increment("i", 1),
        ];
        let condition = condition_lt("i", 3);

        let facts = try_extract_nested_loop_minimal_facts(&condition, &body).expect("Ok");
        let facts = facts.expect("Some");

        assert_eq!(facts.outer_loop_var, "i");
        assert_eq!(facts.inner_loop_var, "j");
        assert_eq!(facts.acc_var, "sum");
        assert_eq!(facts.inner_init_lit, 0);
        assert!(facts.source_topology.is_none());
    }

    #[test]
    fn facts_rejects_missing_inner_init() {
        let body = vec![inner_loop(), increment("i", 1)];
        let condition = condition_lt("i", 3);

        let facts = try_extract_nested_loop_minimal_facts(&condition, &body).expect("Ok");
        assert!(facts.is_none());
    }

    #[test]
    fn facts_retains_variable_direct_outer_schedule() {
        let condition = condition_lt("i", 3);
        let body = variable_outer_schedule();
        let (flat_body, projection) = flatten_scope_boxes_with_projection(&body).into_parts();

        let facts = try_extract_nested_loop_minimal_facts_with_projection(
            &condition,
            &flat_body,
            &projection,
        )
        .expect("Ok")
        .expect("nested facts");
        let topology = facts.source_topology.expect("aligned topology");

        assert!(!topology.has_scope_box_lineage());
        assert_eq!(topology.schedule.len(), 6);
        assert_eq!(
            topology
                .schedule
                .iter()
                .map(|observed| observed.role.clone())
                .collect::<Vec<_>>(),
            vec![
                NestedLoopMinimalObservedStmtRoleV1::InnerLocal { initializes: false },
                NestedLoopMinimalObservedStmtRoleV1::InnerLocal { initializes: false },
                NestedLoopMinimalObservedStmtRoleV1::InnerInitAssignment,
                NestedLoopMinimalObservedStmtRoleV1::OuterWrite,
                NestedLoopMinimalObservedStmtRoleV1::InnerLoop,
                NestedLoopMinimalObservedStmtRoleV1::OuterStep,
            ]
        );
        assert!(topology
            .schedule
            .iter()
            .enumerate()
            .all(|(index, observed)| observed.site.raw_body_index() == index as u32));
    }

    #[test]
    fn facts_retains_scope_box_lineage_for_variable_schedule() {
        let condition = condition_lt("i", 3);
        let raw_body = vec![ASTNode::ScopeBox {
            body: variable_outer_schedule(),
            span: Span::unknown(),
        }];
        let (flat_body, projection) = flatten_scope_boxes_with_projection(&raw_body).into_parts();

        let facts = try_extract_nested_loop_minimal_facts_with_projection(
            &condition,
            &flat_body,
            &projection,
        )
        .expect("Ok")
        .expect("nested facts");
        let topology = facts.source_topology.expect("aligned topology");

        assert!(topology.has_scope_box_lineage());
        assert!(topology
            .schedule
            .iter()
            .enumerate()
            .all(|(index, observed)| {
                observed.site.raw_body_index() == 0
                    && observed.site.scope_box_children() == [index as u32]
            }));
    }
}
