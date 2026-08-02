//! Scan with init facts extraction

use super::loop_types::ScanWithInitFacts;
use super::scan_shapes::{
    loop_var_from_profile, match_scan_with_init_shape, scan_condition_observation,
    step_delta_from_profile, ConditionShape, StepShape,
};
use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::builder::control_flow::facts::stmt_view::{
    LoopSourceBodySiteV1, LoopSourceProjectionV1,
};
use crate::mir::builder::control_flow::plan::planner::Freeze;

/// Opaque coordinates for the two whole statements observed by ScanWithInit.
///
/// The matcher permits unrelated prefix/sibling statements, so this records
/// neither a whole-body schedule nor any inner expression authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct ScanWithInitSourceTopologyV1 {
    matched_if_return: LoopSourceBodySiteV1,
    final_step: LoopSourceBodySiteV1,
}

impl ScanWithInitSourceTopologyV1 {
    pub(in crate::mir::builder) fn has_scope_box_lineage(&self) -> bool {
        !self.matched_if_return.scope_box_children().is_empty()
            || !self.final_step.scope_box_children().is_empty()
    }
}

struct ScanIfReturnMatch {
    haystack: String,
    needle: String,
    dynamic_needle: bool,
    matched_index: usize,
}

pub(super) fn try_extract_scan_with_init_facts(
    condition: &ASTNode,
    body: &[ASTNode],
    condition_shape: &ConditionShape,
    step_shape: &StepShape,
) -> Result<Option<ScanWithInitFacts>, Freeze> {
    try_extract_scan_with_init_facts_with_projection(
        condition,
        body,
        condition_shape,
        step_shape,
        &LoopSourceProjectionV1::default(),
    )
}

pub(super) fn try_extract_scan_with_init_facts_with_projection(
    condition: &ASTNode,
    body: &[ASTNode],
    condition_shape: &ConditionShape,
    step_shape: &StepShape,
    source_projection: &LoopSourceProjectionV1,
) -> Result<Option<ScanWithInitFacts>, Freeze> {
    let mut idx_var: Option<String> = None;
    let mut expected_haystack: Option<String> = None;
    let mut step_lit = 0;
    let mut shape_dynamic_needle = false;
    let mut shape_needle_var: Option<String> = None;

    let observation = scan_condition_observation(condition_shape, step_shape);
    let profile_loop_var = loop_var_from_profile(&observation.cond_profile);
    let candidate_idx_var = profile_loop_var.as_deref();
    let (candidate_expected_haystack, candidate_dynamic_needle, expected_step) =
        match condition_shape {
            ConditionShape::VarLessLength { haystack_var, .. } => {
                (Some(haystack_var.as_str()), false, Some(1))
            }
            ConditionShape::VarLessLiteral { .. } => (None, false, None),
            ConditionShape::VarCompareBound { .. } => (None, false, None),
            ConditionShape::VarLessEqualLengthMinusNeedle { haystack_var, .. } => {
                (Some(haystack_var.as_str()), true, Some(1))
            }
            ConditionShape::VarGreaterEqualZero { .. } => (None, false, Some(-1)),
            ConditionShape::Unknown => (None, false, None),
        };
    let candidate_needle_var = None; // Derived from CondProfile, not shape
    if let (Some(idx_var), Some(expected_step)) = (candidate_idx_var, expected_step) {
        let step_lit_candidate = match step_shape {
            StepShape::AssignAddConst { k, .. } => *k,
            StepShape::Unknown => 0,
        };
        if find_scan_if_return(
            body,
            idx_var,
            candidate_expected_haystack,
            candidate_dynamic_needle,
            candidate_needle_var,
            step_lit_candidate,
            true,
        )
        .is_some()
        {
            match step_shape {
                StepShape::Unknown => {
                    return Err(Freeze::contract(
                        "[joinir/phase29ab/scan_with_init/contract] scan-with-init contract: missing step update",
                    ));
                }
                StepShape::AssignAddConst { var, k } => {
                    if var != idx_var || *k != expected_step {
                        return Err(Freeze::contract(
                            "[joinir/phase29ab/scan_with_init/contract] scan-with-init contract: invalid step update",
                        ));
                    }
                }
            }
        }
    }
    if let Some(shape) = match_scan_with_init_shape(
        &observation.condition_shape,
        &observation.step_shape,
        &observation.cond_profile,
    ) {
        idx_var = Some(shape.idx_var.clone());
        expected_haystack = shape.haystack_var.clone();
        step_lit = shape.step_lit;
        shape_dynamic_needle = shape.dynamic_needle;
        shape_needle_var = shape.needle_var.clone();
    } else if let StepShape::AssignAddConst { var, .. } = step_shape {
        // CondProfile must have LoopVar - no fallback
        let Some(profile_var) = profile_loop_var else {
            return Ok(None); // Skip if CondProfile has no LoopVar
        };
        if profile_var != *var {
            return Ok(None); // Skip if mismatch
        }
        if step_delta_from_profile(&observation.cond_profile) == Some(1)
            && match_index_of_bound(condition, var)
        {
            idx_var = Some(profile_var);
            expected_haystack = None;
            step_lit = 1;
            shape_dynamic_needle = true;
            shape_needle_var = None;
        }
    }

    let Some(idx_var) = idx_var else {
        return Ok(None);
    };
    let idx_var = idx_var.as_str();

    if let Some(scan_match) = find_scan_if_return(
        body,
        idx_var,
        expected_haystack.as_deref(),
        shape_dynamic_needle,
        shape_needle_var.as_deref(),
        step_lit,
        false,
    ) {
        return Ok(Some(ScanWithInitFacts {
            loop_var: idx_var.to_string(),
            haystack: scan_match.haystack,
            needle: scan_match.needle,
            step_lit,
            dynamic_needle: scan_match.dynamic_needle,
            source_topology: source_topology_for(body, source_projection, scan_match.matched_index),
        }));
    }

    Ok(None)
}

fn find_scan_if_return(
    body: &[ASTNode],
    idx_var: &str,
    expected_haystack: Option<&str>,
    shape_dynamic_needle: bool,
    shape_needle_var: Option<&str>,
    step_lit: i64,
    include_tail: bool,
) -> Option<ScanIfReturnMatch> {
    // Find `if s.substring(i, i + 1) == ch { return i }` anywhere except the last step.
    let stmts: Box<dyn Iterator<Item = &ASTNode>> = if include_tail {
        Box::new(body.iter())
    } else {
        Box::new(body.iter().take(body.len().saturating_sub(1)))
    };
    for (matched_index, stmt) in stmts.enumerate() {
        let ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } = stmt
        else {
            continue;
        };
        if else_body.is_some() {
            continue;
        }

        let extracted = if let ASTNode::BinaryOp {
            operator: BinaryOperator::Equal,
            left,
            right,
            ..
        } = condition.as_ref()
        {
            let ASTNode::MethodCall {
                object,
                method,
                arguments,
                ..
            } = left.as_ref()
            else {
                continue;
            };
            if method == "substring" && arguments.len() == 2 {
                let ASTNode::Variable { name: obj, .. } = object.as_ref() else {
                    continue;
                };
                let resolved_haystack = match expected_haystack {
                    Some(expected) => {
                        if obj != expected {
                            continue;
                        }
                        expected.to_string()
                    }
                    None => obj.clone(),
                };

                // substring(i, i + 1) or substring(i, i + needle.length())
                let (start, end) = (&arguments[0], &arguments[1]);
                match start {
                    ASTNode::Variable { name, .. } if name == idx_var => {}
                    _ => continue,
                }
                let ASTNode::BinaryOp {
                    operator: BinaryOperator::Add,
                    left: end_left,
                    right: end_right,
                    ..
                } = end
                else {
                    continue;
                };
                match end_left.as_ref() {
                    ASTNode::Variable { name, .. } if name == idx_var => {}
                    _ => continue,
                }
                let (is_dynamic, len_var) = match end_right.as_ref() {
                    ASTNode::Literal {
                        value: LiteralValue::Integer(1),
                        ..
                    } => (false, None),
                    ASTNode::MethodCall {
                        object,
                        method,
                        arguments,
                        ..
                    } if arguments.is_empty()
                        && (method == "length" || method == "size")
                        && matches!(object.as_ref(), ASTNode::Variable { .. }) =>
                    {
                        let ASTNode::Variable { name, .. } = object.as_ref() else {
                            continue;
                        };
                        (true, Some(name.to_string()))
                    }
                    _ => continue,
                };

                let ASTNode::Variable {
                    name: needle_var, ..
                } = right.as_ref()
                else {
                    continue;
                };
                Some((resolved_haystack, needle_var.clone(), is_dynamic, len_var))
            } else if method == "starts_with"
                && arguments.len() == 3
                && matches!(
                    right.as_ref(),
                    ASTNode::Literal {
                        value: LiteralValue::Integer(1),
                        ..
                    }
                )
            {
                let ASTNode::Variable { name: haystack, .. } = &arguments[0] else {
                    continue;
                };
                let ASTNode::Variable { name: idx_name, .. } = &arguments[1] else {
                    continue;
                };
                let ASTNode::Variable {
                    name: needle_var, ..
                } = &arguments[2]
                else {
                    continue;
                };
                if idx_name != idx_var {
                    continue;
                }
                let resolved_haystack = match expected_haystack {
                    Some(expected) => {
                        if haystack != expected {
                            continue;
                        }
                        expected.to_string()
                    }
                    None => haystack.clone(),
                };
                Some((
                    resolved_haystack,
                    needle_var.clone(),
                    true,
                    Some(needle_var.clone()),
                ))
            } else {
                None
            }
        } else if let ASTNode::MethodCall {
            method, arguments, ..
        } = condition.as_ref()
        {
            if method != "starts_with" || arguments.len() != 3 {
                continue;
            }
            let ASTNode::Variable { name: haystack, .. } = &arguments[0] else {
                continue;
            };
            let ASTNode::Variable { name: idx_name, .. } = &arguments[1] else {
                continue;
            };
            let ASTNode::Variable {
                name: needle_var, ..
            } = &arguments[2]
            else {
                continue;
            };
            if idx_name != idx_var {
                continue;
            }
            let resolved_haystack = match expected_haystack {
                Some(expected) => {
                    if haystack != expected {
                        continue;
                    }
                    expected.to_string()
                }
                None => haystack.clone(),
            };
            Some((
                resolved_haystack,
                needle_var.clone(),
                true,
                Some(needle_var.clone()),
            ))
        } else {
            None
        };

        let Some((haystack_var, needle, dynamic_needle, needle_len_var)) = extracted else {
            continue;
        };
        if dynamic_needle {
            let len_name = needle_len_var.as_deref();
            if len_name != Some(needle.as_str()) {
                continue;
            }
        }
        if dynamic_needle != shape_dynamic_needle && !(step_lit == -1 && dynamic_needle) {
            continue;
        }
        if let Some(shape_needle) = shape_needle_var {
            if shape_needle != needle {
                continue;
            }
        }

        // then-body must contain `return i` (minimal)
        if !then_body.iter().any(|n| {
            matches!(
                n,
                ASTNode::Return {
                    value: Some(v),
                    ..
                } if matches!(v.as_ref(), ASTNode::Variable { name, .. } if name == idx_var)
            )
        }) {
            continue;
        }

        return Some(ScanIfReturnMatch {
            haystack: haystack_var,
            needle,
            dynamic_needle,
            matched_index,
        });
    }

    None
}

fn source_topology_for(
    body: &[ASTNode],
    projection: &LoopSourceProjectionV1,
    matched_index: usize,
) -> Option<ScanWithInitSourceTopologyV1> {
    let final_step_index = body.len().checked_sub(1)?;
    if matched_index >= final_step_index || projection.flattened_body_len() != Some(body.len()) {
        return None;
    }
    Some(ScanWithInitSourceTopologyV1 {
        matched_if_return: projection.site_for_flattened_index(matched_index)?.clone(),
        final_step: projection
            .site_for_flattened_index(final_step_index)?
            .clone(),
    })
}

pub(super) fn match_index_of_bound(condition: &ASTNode, idx_var: &str) -> bool {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::LessEqual,
        left,
        right,
        ..
    } = condition
    else {
        return false;
    };
    if !matches!(right.as_ref(), ASTNode::Variable { .. }) {
        return false;
    }
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left: add_left,
        right: add_right,
        ..
    } = left.as_ref()
    else {
        return false;
    };
    let (a, b) = match (add_left.as_ref(), add_right.as_ref()) {
        (ASTNode::Variable { name: a, .. }, ASTNode::Variable { name: b, .. }) => (a, b),
        _ => return false,
    };
    a == idx_var || b == idx_var
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Span;
    use crate::mir::builder::control_flow::plan::facts::loop_builder::try_build_loop_facts;
    use crate::mir::builder::control_flow::plan::facts::loop_condition_shape::try_extract_condition_shape;
    use crate::mir::builder::control_flow::plan::facts::loop_step_shape::try_extract_step_shape;

    fn v(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    fn scan_condition() -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(v("i")),
            right: Box::new(ASTNode::MethodCall {
                object: Box::new(v("s")),
                method: "length".to_string(),
                arguments: vec![],
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }
    }

    fn matched_if_return() -> ASTNode {
        ASTNode::If {
            condition: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Equal,
                left: Box::new(ASTNode::MethodCall {
                    object: Box::new(v("s")),
                    method: "substring".to_string(),
                    arguments: vec![
                        v("i"),
                        ASTNode::BinaryOp {
                            operator: BinaryOperator::Add,
                            left: Box::new(v("i")),
                            right: Box::new(ASTNode::Literal {
                                value: LiteralValue::Integer(1),
                                span: Span::unknown(),
                            }),
                            span: Span::unknown(),
                        },
                    ],
                    span: Span::unknown(),
                }),
                right: Box::new(v("ch")),
                span: Span::unknown(),
            }),
            then_body: vec![ASTNode::Return {
                value: Some(Box::new(v("i"))),
                span: Span::unknown(),
            }],
            else_body: None,
            span: Span::unknown(),
        }
    }

    fn final_step() -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(v("i")),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(v("i")),
                right: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }
    }

    fn topology_for(raw_body: Vec<ASTNode>) -> ScanWithInitSourceTopologyV1 {
        try_build_loop_facts(&scan_condition(), &raw_body)
            .expect("facts extraction")
            .expect("loop facts")
            .scan_with_init
            .expect("scan facts")
            .source_topology
            .expect("aligned scan topology")
    }

    #[test]
    fn source_topology_keeps_only_matched_if_return_and_final_step() {
        let topology = topology_for(vec![matched_if_return(), final_step()]);

        assert_eq!(topology.matched_if_return.raw_body_index(), 0);
        assert_eq!(topology.final_step.raw_body_index(), 1);
        assert!(!topology.has_scope_box_lineage());
    }

    #[test]
    fn source_topology_retains_scope_box_lineage_without_borrowing() {
        let topology = topology_for(vec![ASTNode::ScopeBox {
            body: vec![matched_if_return(), final_step()],
            span: Span::unknown(),
        }]);

        assert_eq!(topology.matched_if_return.raw_body_index(), 0);
        assert_eq!(topology.matched_if_return.scope_box_children(), &[0]);
        assert_eq!(topology.final_step.raw_body_index(), 0);
        assert_eq!(topology.final_step.scope_box_children(), &[1]);
        assert!(topology.has_scope_box_lineage());
    }

    #[test]
    fn source_topology_keeps_prefix_sibling_outside_observed_pair() {
        let prefix = ASTNode::Local {
            variables: vec!["unused".to_string()],
            initial_values: vec![None],
            declared_type_names: vec![],
            span: Span::unknown(),
        };
        let topology = topology_for(vec![prefix, matched_if_return(), final_step()]);

        assert_eq!(topology.matched_if_return.raw_body_index(), 1);
        assert_eq!(topology.final_step.raw_body_index(), 2);
        assert!(!topology.has_scope_box_lineage());
    }

    #[test]
    fn legacy_extractor_keeps_source_topology_absent() {
        let condition = scan_condition();
        let body = vec![matched_if_return(), final_step()];
        let condition_shape = try_extract_condition_shape(&condition)
            .expect("condition shape")
            .expect("known condition shape");
        let step_shape = try_extract_step_shape(&body)
            .expect("step shape")
            .expect("known step shape");

        let facts =
            try_extract_scan_with_init_facts(&condition, &body, &condition_shape, &step_shape)
                .expect("facts extraction")
                .expect("scan facts");

        assert!(facts.source_topology.is_none());
    }
}
