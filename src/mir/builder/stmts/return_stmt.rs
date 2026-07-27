//! Return statement handling module
//!
//! This module owns Return completion, Match-return optimization, and the
//! exact no-value Return leaf.
//!
//! # Purpose
//! - Return statement execution with defer mechanism
//! - Match-return CorePlan composition and adoption
//! - Plan system integration (verify → lower)
//!
//! # Responsibilities
//! - `build_void_return_statement`: Exact `return;` lowering
//! - `try_apply_match_return_optimization`: Value-bearing Match probe
//! - `emit_return_from_value`: Shared Return/defer completion
//! - `adopt_match_return_coreplan`: **Private** helper for match-return optimization
//!   - CorePlan composition via `compose_match_return_branchn`
//!   - CorePlan verification
//!   - Flowbox tag emission for observability
//!   - CorePlan lowering to MIR
//!
//! # Match-Return Optimization
//! Optimizes patterns like:
//! ```hako
//! return match x {
//!     1 => "one",
//!     2 => "two",
//!     _ => "other"
//! }
//! ```
//!
//! Into efficient control flow using CorePlan system.
//!
//! # Defer Mechanism
//! When `return_defer_active` is true:
//! - Copies return value to `return_defer_slot`
//! - Jumps to `return_defer_target` (cleanup block)
//! - Allows cleanup code execution before actual return
//!
//! # Related
//! - CorePlan system: `src/mir/builder/control_flow/plan/`
//! - Match-return facts: `control_flow/plan/facts/match_return_facts.rs`
//! - Match-return composer: `control_flow/plan/composer/match_return_branchn.rs`

use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::lower::PlanLowerer;
use crate::mir::builder::control_flow::plan::composer::{
    compose_match_return_branchn, MatchReturnPlan,
};
use crate::mir::builder::control_flow::plan::facts::match_return_facts::{
    try_extract_match_return_facts, MatchReturnFacts,
};
use crate::mir::builder::control_flow::verify::observability::flowbox_tags::{self, FlowboxVia};
use crate::mir::builder::control_flow::verify::PlanVerifier;
use crate::mir::{MirBuilder, MirInstruction, ValueId};

/// Preserve the existing cleanup-block Return prohibition before any value
/// observation or lowering effect.
pub(in crate::mir::builder) fn ensure_return_allowed(builder: &MirBuilder) -> Result<(), String> {
    if builder.function_state.in_cleanup_block && !builder.function_state.cleanup_allow_return {
        return Err("return is not allowed inside cleanup block (enable NYASH_CLEANUP_ALLOW_RETURN=1 to permit)".to_string());
    }
    Ok(())
}

/// Adopt match-return CorePlan optimization
///
/// Private implementation of `try_apply_match_return_optimization`.
///
/// # Process
/// 1. Compose CorePlan from match-return facts
/// 2. Verify CorePlan invariants
/// 3. Emit flowbox tag for observability
/// 4. Lower CorePlan to MIR instructions
///
/// # Arguments
/// * `builder` - MIR builder context
/// * `facts` - Match-return pattern facts
/// * `emit_tag` - Whether to emit observability tags
///
/// # Returns
/// `Ok(Some(ValueId))` - Return value after optimization
/// `Ok(None)` - Optimization not applicable
/// `Err(String)` - CorePlan composition/verification/lowering error
fn adopt_match_return_coreplan(
    builder: &mut MirBuilder,
    facts: &MatchReturnFacts,
    emit_tag: bool,
) -> Result<Option<ValueId>, String> {
    let MatchReturnPlan {
        core_plan,
        return_value,
    } = compose_match_return_branchn(builder, facts)?;

    PlanVerifier::verify(&core_plan)?;
    flowbox_tags::emit_flowbox_adopt_tag_for_coreplan(
        emit_tag,
        &core_plan,
        None,
        &["return"],
        FlowboxVia::Shadow,
    );

    let cond = ASTNode::Literal {
        value: LiteralValue::Bool(true),
        span: Span::unknown(),
    };
    let body: Vec<ASTNode> = Vec::new();
    let func_name = builder
        .function_state
        .current_function
        .as_ref()
        .map(|func| func.signature.name.clone())
        .unwrap_or_else(|| "<unknown>".to_string());
    let ctx = LoopRouteContext::new(&cond, &body, &func_name, false, false);

    PlanLowerer::lower(builder, core_plan, &ctx)?;
    Ok(Some(return_value))
}

/// Try to apply the match-return optimization before lowering the return value.
///
/// This is the shared shell used by ordinary lowering and fastmem lowering.
pub(in crate::mir::builder) fn try_apply_match_return_optimization(
    builder: &mut MirBuilder,
    value: Option<&ASTNode>,
    emit_tag: bool,
) -> Result<Option<ValueId>, String> {
    if builder.function_state.return_defer_active {
        return Ok(None);
    }

    let Some(expr) = value else {
        return Ok(None);
    };

    let strict_or_dev = crate::config::env::joinir_dev::strict_enabled();
    if strict_or_dev {
        match try_extract_match_return_facts(expr, true) {
            Ok(Some(facts)) => {
                if let Some(return_value) =
                    adopt_match_return_coreplan(builder, &facts, emit_tag && strict_or_dev)?
                {
                    return Ok(Some(return_value));
                }
            }
            Ok(None) => {}
            Err(freeze) => return Err(freeze.to_string()),
        }
    } else if let Ok(Some(facts)) = try_extract_match_return_facts(expr, false) {
        if let Ok(Some(return_value)) =
            adopt_match_return_coreplan(builder, &facts, emit_tag && strict_or_dev)
        {
            return Ok(Some(return_value));
        }
    }

    Ok(None)
}

/// Emit the final return instruction from an already-evaluated value.
///
/// This is the shared shell used by ordinary lowering and fastmem lowering.
pub(in crate::mir::builder) fn emit_return_from_value(
    builder: &mut MirBuilder,
    return_value: ValueId,
) -> Result<ValueId, String> {
    if builder.function_state.return_defer_active {
        // Defer: copy into slot and jump to target
        if let (Some(slot), Some(target)) = (
            builder.function_state.return_defer_slot,
            builder.function_state.return_defer_target,
        ) {
            builder.function_state.return_deferred_emitted = true;
            builder.emit_instruction(MirInstruction::Copy {
                dst: slot,
                src: return_value,
            })?;
            crate::mir::builder::metadata::propagate::propagate(builder, return_value, slot);
            if !builder.is_current_block_terminated() {
                crate::mir::builder::emission::branch::emit_jump(builder, target)?;
            }
            Ok(return_value)
        } else {
            // Fallback: no configured slot/target; emit a real return
            builder.emit_instruction(MirInstruction::Return {
                value: Some(return_value),
            })?;
            Ok(return_value)
        }
    } else {
        // Normal return
        builder.emit_instruction(MirInstruction::Return {
            value: Some(return_value),
        })?;
        Ok(return_value)
    }
}

/// Lower the exact `return;` source shape.
///
/// Value-bearing Return is owned by `drive_value_return_statement_v1`.
pub(in crate::mir::builder) fn build_void_return_statement(
    builder: &mut MirBuilder,
) -> Result<ValueId, String> {
    ensure_return_allowed(builder)?;
    let return_value = crate::mir::builder::emission::constant::emit_void(builder)?;
    emit_return_from_value(builder, return_value)
}
