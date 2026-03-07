//! Loop Canonicalization Entry Point
//!
//! This module provides the main canonicalization logic that converts
//! AST loops into normalized LoopSkeleton structures.

use crate::ast::{ASTNode, LiteralValue};
use crate::mir::loop_route_detection::LoopRouteKind;
use crate::mir::policies::balanced_depth_scan;
use crate::mir::policies::PolicyDecision;

use super::capability_guard::{CapabilityTag, RoutingDecision};
use super::route_shape_recognizer::{
    try_extract_continue_shape, try_extract_escape_skip_shape, try_extract_parse_number_shape,
    try_extract_parse_string_shape, try_extract_read_digits_loop_true_shape,
    try_extract_skip_whitespace_shape,
};
use super::skeleton_types::{
    CarrierRole, CarrierSlot, ExitContract, LoopSkeleton, SkeletonStep, UpdateKind,
};

// ============================================================================
// Canonicalization Entry Point
// ============================================================================

/// Canonicalize a loop AST into LoopSkeleton
///
/// Phase 143-P2: Now supports parse_array route shape in addition to parse_string, skip_whitespace, parse_number, and continue
///
/// Supported route shapes:
/// 1. Skip whitespace (break in ELSE clause):
/// ```
/// loop(cond) {
///     // ... optional body statements
///     if check_cond {
///         carrier = carrier + step
///     } else {
///         break
///     }
/// }
/// ```
///
/// 2. Parse number (break in THEN clause):
/// ```
/// loop(cond) {
///     // ... optional body statements (ch, digit_pos computation)
///     if invalid_cond {
///         break
///     }
///     // ... rest statements (result append)
///     carrier = carrier + step
/// }
/// ```
///
/// 3. Continue route shape:
/// ```
/// loop(cond) {
///     // ... optional body statements
///     if skip_cond {
///         carrier = carrier + step  // Optional
///         continue
///     }
///     // ... rest statements
///     carrier = carrier + step
/// }
/// ```
///
/// 4. Parse string/array (both continue AND return):
/// ```
/// loop(cond) {
///     // ... body statements
///     if stop_cond {            // quote for string, ']' for array
///         return result
///     }
///     if separator_cond {       // escape for string, ',' for array
///         // ... separator handling
///         carrier = carrier + step
///         continue
///     }
///     carrier = carrier + step
/// }
/// ```
///
/// Note: parse_string and parse_array share the same structural route shape
/// (continue + return exits) and are recognized by the same detector.
///
/// All other route shapes return Fail-Fast with detailed reasoning.
///
/// # Arguments
/// - `loop_expr`: The loop AST node (must be `ASTNode::Loop`)
///
/// # Returns
/// - `Ok((skeleton, decision))`: Successfully extracted skeleton and routing decision
/// - `Err(String)`: Malformed AST or internal error
pub fn canonicalize_loop_expr(
    loop_expr: &ASTNode,
) -> Result<(LoopSkeleton, RoutingDecision), String> {
    // Extract loop components
    let (condition, body, span) = match loop_expr {
        ASTNode::Loop {
            condition,
            body,
            span,
        } => (condition.as_ref(), body, span.clone()),
        _ => return Err(format!("Expected Loop node, got: {:?}", loop_expr)),
    };

    // Phase 29bq: Align canonicalizer with balanced depth-scan policy routing.
    match balanced_depth_scan::decide(condition, body) {
        PolicyDecision::Use(_) => {
            let decision = RoutingDecision::success(LoopRouteKind::LoopBreak);
            return Ok((LoopSkeleton::new(span), decision));
        }
        PolicyDecision::Reject(_) => {
            if crate::config::env::joinir_dev::strict_enabled() {
                let decision = RoutingDecision::success(LoopRouteKind::LoopBreak);
                return Ok((LoopSkeleton::new(span), decision));
            }
        }
        PolicyDecision::None => {}
    }

    // Phase 143-P1/P2: Try to extract parse_string/parse_array route shape first (most specific)
    // Note: Both parse_string and parse_array share the same structure (continue + return)
    if let Some((carrier_name, delta, body_stmts)) = try_extract_parse_string_shape(body) {
        // Build skeleton for parse_string/parse_array route shape
        let mut skeleton = LoopSkeleton::new(span);

        // Step 1: Header condition
        skeleton.steps.push(SkeletonStep::HeaderCond {
            expr: Box::new(condition.clone()),
        });

        // Step 2: Body statements (if any)
        if !body_stmts.is_empty() {
            skeleton
                .steps
                .push(SkeletonStep::Body { stmts: body_stmts });
        }

        // Step 3: Update step
        skeleton.steps.push(SkeletonStep::Update {
            carrier_name: carrier_name.clone(),
            update_kind: UpdateKind::ConstStep { delta },
        });

        // Add carrier slot
        skeleton.carriers.push(CarrierSlot {
            name: carrier_name,
            role: CarrierRole::Counter,
            update_kind: UpdateKind::ConstStep { delta },
        });

        // Set exit contract for parse_string/parse_array route shape
        skeleton.exits = ExitContract {
            has_break: false,
            has_continue: true,
            has_return: true,
            break_has_value: false,
        };

        // Phase 143-P1: Route to LoopContinueOnly (has both continue and return)
        let decision = RoutingDecision::success(LoopRouteKind::LoopContinueOnly);
        return Ok((skeleton, decision));
    }

    // Phase 142-P1: Try to extract continue route shape
    if let Some((carrier_name, delta, body_stmts, rest_stmts)) = try_extract_continue_shape(body)
    {
        // Build skeleton for continue route shape
        let mut skeleton = LoopSkeleton::new(span);

        // Step 1: Header condition
        skeleton.steps.push(SkeletonStep::HeaderCond {
            expr: Box::new(condition.clone()),
        });

        // Step 2: Body statements (if any)
        if !body_stmts.is_empty() {
            skeleton
                .steps
                .push(SkeletonStep::Body { stmts: body_stmts });
        }

        // Step 3: Rest statements (if any, excluding carrier update)
        // For now, we include all rest_stmts in Body
        // The actual carrier update is implicit in the Update step
        if !rest_stmts.is_empty() {
            // Remove the last statement (carrier update) from rest_stmts
            let mut rest_body = rest_stmts;
            if !rest_body.is_empty() {
                rest_body.pop();
            }
            if !rest_body.is_empty() {
                skeleton.steps.push(SkeletonStep::Body { stmts: rest_body });
            }
        }

        // Step 4: Update step
        skeleton.steps.push(SkeletonStep::Update {
            carrier_name: carrier_name.clone(),
            update_kind: UpdateKind::ConstStep { delta },
        });

        // Add carrier slot
        skeleton.carriers.push(CarrierSlot {
            name: carrier_name,
            role: CarrierRole::Counter,
            update_kind: UpdateKind::ConstStep { delta },
        });

        // Set exit contract for continue route shape
        skeleton.exits = ExitContract {
            has_break: false,
            has_continue: true,
            has_return: false,
            break_has_value: false,
        };

        // Phase 142-P1: Route to LoopContinueOnly
        let decision = RoutingDecision::success(LoopRouteKind::LoopContinueOnly);
        return Ok((skeleton, decision));
    }

    // Phase 104: loop(true) + break-only digits (read_digits_from)
    //
    // Shape (JsonCursorBox.read_digits_from / MiniJsonLoader.read_digits_from):
    // - loop(true)
    // - last statement is `if is_digit { ... i = i + 1 } else { break }`
    // - may have `if ch == "" { break }` and substring read before it
    if matches!(
        condition,
        ASTNode::Literal {
            value: crate::ast::LiteralValue::Bool(true),
            ..
        }
    ) {
        if let Some((carrier_name, delta, body_stmts)) = try_extract_read_digits_loop_true_shape(body) {
            let mut skeleton = LoopSkeleton::new(span);

            skeleton.steps.push(SkeletonStep::HeaderCond {
                expr: Box::new(condition.clone()),
            });

            if !body_stmts.is_empty() {
                skeleton.steps.push(SkeletonStep::Body { stmts: body_stmts });
            }

            skeleton.steps.push(SkeletonStep::Update {
                carrier_name: carrier_name.clone(),
                update_kind: UpdateKind::ConstStep { delta },
            });

            skeleton.carriers.push(CarrierSlot {
                name: carrier_name,
                role: CarrierRole::Counter,
                update_kind: UpdateKind::ConstStep { delta },
            });

            skeleton.exits = ExitContract {
                has_break: true,
                has_continue: false,
                has_return: false,
                break_has_value: false,
            };

            let decision = RoutingDecision::success(LoopRouteKind::LoopBreak);
            return Ok((skeleton, decision));
        }
    }

    // Phase 143-P0: Try to extract parse_number route shape (break in THEN clause)
    if let Some((carrier_name, delta, body_stmts, rest_stmts)) =
        try_extract_parse_number_shape(body)
    {
        // Build skeleton for parse_number route shape
        let mut skeleton = LoopSkeleton::new(span);

        // Step 1: Header condition
        skeleton.steps.push(SkeletonStep::HeaderCond {
            expr: Box::new(condition.clone()),
        });

        // Step 2: Body statements before break check (if any)
        if !body_stmts.is_empty() {
            skeleton
                .steps
                .push(SkeletonStep::Body { stmts: body_stmts });
        }

        // Step 3: Rest statements after break check (if any, excluding carrier update)
        // The carrier update is implicit in the Update step
        if !rest_stmts.is_empty() {
            // Remove the last statement (carrier update) from rest_stmts
            let mut rest_body = rest_stmts;
            if !rest_body.is_empty() {
                rest_body.pop();
            }
            if !rest_body.is_empty() {
                skeleton.steps.push(SkeletonStep::Body { stmts: rest_body });
            }
        }

        // Step 4: Update step
        skeleton.steps.push(SkeletonStep::Update {
            carrier_name: carrier_name.clone(),
            update_kind: UpdateKind::ConstStep { delta },
        });

        // Add carrier slot
        skeleton.carriers.push(CarrierSlot {
            name: carrier_name,
            role: CarrierRole::Counter,
            update_kind: UpdateKind::ConstStep { delta },
        });

        // Set exit contract for parse_number route shape
        skeleton.exits = ExitContract {
            has_break: true,
            has_continue: false,
            has_return: false,
            break_has_value: false,
        };

        // Phase 143-P0: Route to LoopBreak (has_break=true)
        let decision = RoutingDecision::success(LoopRouteKind::LoopBreak);
        return Ok((skeleton, decision));
    }

    // Phase 3: Try to extract skip_whitespace route shape
    if let Some((carrier_name, delta, body_stmts)) = try_extract_skip_whitespace_shape(body) {
        // Build skeleton for skip_whitespace route shape
        let mut skeleton = LoopSkeleton::new(span);

        // Step 1: Header condition
        skeleton.steps.push(SkeletonStep::HeaderCond {
            expr: Box::new(condition.clone()),
        });

        // Step 2: Body statements (if any)
        if !body_stmts.is_empty() {
            skeleton
                .steps
                .push(SkeletonStep::Body { stmts: body_stmts });
        }

        // Step 3: Update step
        skeleton.steps.push(SkeletonStep::Update {
            carrier_name: carrier_name.clone(),
            update_kind: UpdateKind::ConstStep { delta },
        });

        // Add carrier slot
        skeleton.carriers.push(CarrierSlot {
            name: carrier_name,
            role: CarrierRole::Counter,
            update_kind: UpdateKind::ConstStep { delta },
        });

        // Set exit contract
        skeleton.exits = ExitContract {
            has_break: true,
            has_continue: false,
            has_return: false,
            break_has_value: false,
        };

        // Phase 137-5: Decision policy SSOT - ExitContract determines route choice
        // Since has_break=true, this should route to LoopBreak (not IfPhiJoin)
        // IfPhiJoin is for if-else PHI *without* break statements
        let decision = RoutingDecision::success(LoopRouteKind::LoopBreak);
        return Ok((skeleton, decision));
    }

    // ========================================================================
    // Phase 91 P5b: Escape Sequence Handling Route Shape
    // ========================================================================
    // Position: After skip_whitespace (post-existing route shapes)
    // Purpose: Recognize escape sequence handling in string parsers
    // Chosen: LoopBreak (same as skip_whitespace, but with richer Skeleton)
    // Notes: Added for parity/observability, lowering deferred to Phase 92

    // Phase 92 P0-3: Now also extracts escape_cond for JoinIR Select generation
    if let Some((counter_name, normal_delta, escape_delta, _quote_char, _escape_char, body_stmts, escape_cond)) =
        try_extract_escape_skip_shape(body)
    {
        // Build skeleton for escape skip route shape (P5b)
        let mut skeleton = LoopSkeleton::new(span);

        // Step 1: Header condition
        skeleton.steps.push(SkeletonStep::HeaderCond {
            expr: Box::new(condition.clone()),
        });

        // Step 2: Body statements (if any)
        if !body_stmts.is_empty() {
            skeleton
                .steps
                .push(SkeletonStep::Body { stmts: body_stmts });
        }

        // Step 3: Update step with ConditionalStep (escape_delta vs normal_delta)
        // Route shape: normal i = i + 1, escape i = i + escape_delta (e.g., +2)
        // Represented as UpdateKind::ConditionalStep with both deltas and condition
        // Phase 92 P0-3: Now includes escape_cond for JoinIR Select generation
        skeleton.steps.push(SkeletonStep::Update {
            carrier_name: counter_name.clone(),
            update_kind: UpdateKind::ConditionalStep {
                cond: escape_cond.clone(),  // Phase 92 P0-3: Condition for Select
                then_delta: escape_delta,    // Escape branch: +2 or other
                else_delta: normal_delta,    // Normal branch: +1
            },
        });

        // Add carrier slot with conditional step update
        skeleton.carriers.push(CarrierSlot {
            name: counter_name,
            role: CarrierRole::Counter,
            update_kind: UpdateKind::ConditionalStep {
                cond: escape_cond,           // Phase 92 P0-3: Condition for Select
                then_delta: escape_delta,
                else_delta: normal_delta,
            },
        });

        // Set exit contract (P5b has break for string boundary detection)
        skeleton.exits = ExitContract {
            has_break: true,
            has_continue: false,
            has_return: false,
            break_has_value: false,
        };

        // Phase 91 P5b Decision Policy:
        // Same as skip_whitespace (LoopBreak)
        // P5b is a "detailed version" of LoopBreak, not a separate chosen route
        // Notes field would record escape-specific details (Phase 91 MVP: omitted)
        let decision = RoutingDecision::success(LoopRouteKind::LoopBreak);
        return Ok((skeleton, decision));
    }

    // Phase 29bq: loop(true) break-only scan (parse_term2 family).
    if matches_loop_true_break_only_body(condition, body) {
        let decision = RoutingDecision::success(LoopRouteKind::LoopBreak);
        return Ok((LoopSkeleton::new(span), decision));
    }

    // Route shape not recognized - fail fast
    Ok((
        LoopSkeleton::new(span),
        RoutingDecision::fail_fast(
            vec![CapabilityTag::ConstStep],
            "Phase 143-P2: Loop does not match read_digits(loop(true)), skip_whitespace, parse_number, continue, parse_string, or parse_array route shape"
                .to_string(),
        ),
    ))
}

fn matches_loop_true_break_only_body(condition: &ASTNode, body: &[ASTNode]) -> bool {
    if !matches!(
        condition,
        ASTNode::Literal {
            value: LiteralValue::Bool(true),
            ..
        }
    ) {
        return false;
    }
    if body.len() != 7 {
        return false;
    }
    if !matches!(body[0], ASTNode::Assignment { .. }) {
        return false;
    }
    if !is_break_only_if(&body[1]) {
        return false;
    }
    if !matches!(body[2], ASTNode::Local { .. }) {
        return false;
    }
    if !is_break_only_if(&body[3]) {
        return false;
    }
    if !matches!(body[4], ASTNode::Local { .. }) {
        return false;
    }
    if !matches!(body[5], ASTNode::Assignment { .. }) {
        return false;
    }
    if !matches!(body[6], ASTNode::Assignment { .. }) {
        return false;
    }
    true
}

fn is_break_only_if(stmt: &ASTNode) -> bool {
    let ASTNode::If {
        then_body,
        else_body,
        ..
    } = stmt
    else {
        return false;
    };
    if else_body.is_some() || then_body.len() != 1 {
        return false;
    }
    matches!(then_body[0], ASTNode::Break { .. })
}
