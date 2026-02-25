//! Scan with init facts extraction

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use super::scan_shapes::{
    loop_var_from_profile, match_scan_with_init_shape, scan_condition_observation,
    step_delta_from_profile, ConditionShape, StepShape,
};
use super::loop_types::ScanWithInitFacts;
use crate::mir::builder::control_flow::plan::planner::Freeze;

pub(super) fn try_extract_scan_with_init_facts(
    condition: &ASTNode,
    body: &[ASTNode],
    condition_shape: &ConditionShape,
    step_shape: &StepShape,
) -> Result<Option<ScanWithInitFacts>, Freeze> {
    let mut idx_var: Option<String> = None;
    let mut expected_haystack: Option<String> = None;
    let mut step_lit = 0;
    let mut shape_dynamic_needle = false;
    let mut shape_needle_var: Option<String> = None;

    let observation = scan_condition_observation(condition_shape, step_shape);
    let profile_loop_var = loop_var_from_profile(&observation.cond_profile);
    let candidate_idx_var = profile_loop_var.as_deref();
    let (candidate_expected_haystack, candidate_dynamic_needle, expected_step) = match condition_shape {
        ConditionShape::VarLessLength {
            haystack_var,
            ..
        } => (Some(haystack_var.as_str()), false, Some(1)),
        ConditionShape::VarLessLiteral { .. } => (None, false, None),
        ConditionShape::VarLessEqualLengthMinusNeedle {
            haystack_var,
            ..
        } => (Some(haystack_var.as_str()), true, Some(1)),
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
                        "[joinir/phase29ab/pattern6/contract] scan-with-init contract: missing step update",
                    ));
                }
                StepShape::AssignAddConst { var, k } => {
                    if var != idx_var || *k != expected_step {
                        return Err(Freeze::contract(
                            "[joinir/phase29ab/pattern6/contract] scan-with-init contract: invalid step update",
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

    if let Some((haystack_var, needle, dynamic_needle)) = find_scan_if_return(
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
            haystack: haystack_var,
            needle,
            step_lit,
            dynamic_needle,
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
) -> Option<(String, String, bool)> {
    // Find `if s.substring(i, i + 1) == ch { return i }` anywhere except the last step.
    let stmts: Box<dyn Iterator<Item = &ASTNode>> = if include_tail {
        Box::new(body.iter())
    } else {
        Box::new(body.iter().take(body.len().saturating_sub(1)))
    };
    for stmt in stmts {
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

                let ASTNode::Variable { name: needle_var, .. } = right.as_ref() else {
                    continue;
                };
                Some((
                    resolved_haystack,
                    needle_var.clone(),
                    is_dynamic,
                    len_var,
                ))
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
                let ASTNode::Variable { name: needle_var, .. } = &arguments[2] else {
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
            method,
            arguments,
            ..
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
            let ASTNode::Variable { name: needle_var, .. } = &arguments[2] else {
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

        return Some((haystack_var, needle, dynamic_needle));
    }

    None
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
