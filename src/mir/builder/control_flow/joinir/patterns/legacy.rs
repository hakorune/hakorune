//! Legacy routing fallback (plan normalizer path).
//! Legacy is entry-compatibility only; semantics live in plan/recipe/parts.

use crate::mir::builder::control_flow::joinir::patterns::router::LoopPatternContext;
use crate::mir::builder::control_flow::plan::DomainPlan;
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;

/// Phase 286 P2.2: Common helper for Plan line lowering.
///
/// Extracts the common 3-line pattern used by Plan-based routing:
/// 1. Normalize DomainPlan → CorePlan
/// 2. Verify CorePlan invariants (fail-fast)
/// 3. Lower CorePlan → MIR
pub(in crate::mir::builder) fn lower_via_plan(
    _builder: &mut MirBuilder,
    domain_plan: DomainPlan,
    _ctx: &LoopPatternContext,
) -> Result<Option<ValueId>, String> {
    // Phase 29bq P2.x: LoopCondContinueWithReturn not implemented via legacy path
    // Use the recipe-first pipeline instead
    match domain_plan {
        _ => return Ok(None),
    }
}
