//! Return statement handling module
//!
//! This module provides return statement lowering with match-return optimization support.
//!
//! # Purpose
//! - Return statement execution with defer mechanism
//! - Match-return CorePlan composition and adoption
//! - Plan system integration (verify → lower)
//!
//! # Responsibilities
//! - `build_return_statement`: Main return statement builder
//!   - Match-return facts extraction and optimization
//!   - Return value evaluation
//!   - Defer mechanism handling (slot copy + jump)
//!   - Normal return emission
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
//! - Match-return facts: `control_flow/facts/`
//! - Match-return composer: `control_flow/plan/composer/match_return_branchn.rs`

use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::builder::control_flow::facts::{try_extract_match_return_facts, MatchReturnFacts};
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::composer::{
    compose_match_return_branchn, MatchReturnPlan,
};
use crate::mir::builder::control_flow::lower::PlanLowerer;
use crate::mir::builder::control_flow::verify::observability::flowbox_tags::{self, FlowboxVia};
use crate::mir::builder::control_flow::verify::PlanVerifier;
use crate::mir::{MirBuilder, MirInstruction, ValueId};

/// Adopt match-return CorePlan optimization
///
/// **Private helper** - only called from `build_return_statement`.
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
        .scope_ctx
        .current_function
        .as_ref()
        .map(|func| func.signature.name.clone())
        .unwrap_or_else(|| "<unknown>".to_string());
    let ctx = LoopRouteContext::new(&cond, &body, &func_name, false, false);

    PlanLowerer::lower(builder, core_plan, &ctx)?;
    Ok(Some(return_value))
}

/// Build return statement with match-return optimization
///
/// # Process
/// 1. **Match-return optimization** (if enabled and pattern matches):
///    - Extract match-return facts from return expression
///    - Adopt CorePlan optimization via `adopt_match_return_coreplan`
/// 2. **Return value evaluation**:
///    - Evaluate return expression or default to void
/// 3. **Defer mechanism** (if `return_defer_active`):
///    - Copy return value to `return_defer_slot`
///    - Jump to `return_defer_target` for cleanup
///    - Set `return_deferred_emitted` flag
/// 4. **Normal return**:
///    - Emit `MirInstruction::Return` with evaluated value
///
/// # Arguments
/// * `builder` - MIR builder context
/// * `value` - Optional return expression
///
/// # Returns
/// `Ok(ValueId)` - Return value (evaluated or void)
/// `Err(String)` - Compilation error
///
/// # Examples
/// ```hako
/// // Simple return
/// return 42
///
/// // Match-return (optimized)
/// return match x {
///     1 => "one",
///     _ => "other"
/// }
///
/// // Void return
/// return
/// ```
pub(in crate::mir::builder) fn build_return_statement(
    builder: &mut MirBuilder,
    value: Option<Box<ASTNode>>,
) -> Result<ValueId, String> {
    // Enforce cleanup policy
    if builder.in_cleanup_block && !builder.cleanup_allow_return {
        return Err("return is not allowed inside cleanup block (enable NYASH_CLEANUP_ALLOW_RETURN=1 to permit)".to_string());
    }

    let strict_or_dev = crate::config::env::joinir_dev::strict_enabled();
    if !builder.return_defer_active {
        if let Some(expr) = value.as_deref() {
            if strict_or_dev {
                match try_extract_match_return_facts(expr, true) {
                    Ok(Some(facts)) => {
                        if let Some(return_value) =
                            adopt_match_return_coreplan(builder, &facts, true)?
                        {
                            return Ok(return_value);
                        }
                    }
                    Ok(None) => {}
                    Err(freeze) => return Err(freeze.to_string()),
                }
            } else if let Ok(Some(facts)) = try_extract_match_return_facts(expr, false) {
                if let Ok(Some(return_value)) = adopt_match_return_coreplan(builder, &facts, false)
                {
                    return Ok(return_value);
                }
            }
        }
    }

    let return_value = if let Some(expr) = value {
        builder.build_expression(*expr)?
    } else {
        crate::mir::builder::emission::constant::emit_void(builder)?
    };

    if builder.return_defer_active {
        // Defer: copy into slot and jump to target
        if let (Some(slot), Some(target)) = (builder.return_defer_slot, builder.return_defer_target)
        {
            builder.return_deferred_emitted = true;
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
