//! # Loop Route JoinIR Lowering Router
//!
//! **Phase 33-12 Modularization**: Extracted from `mod.rs` (lines 424-511)
//!
//! ## Responsibility
//! Routes loop route families to appropriate JoinIR lowering strategies.
//! This is the **main entry point** for loop → JoinIR lowering.
//!
//! ## Route Dispatch
//! Routes to:
//! - `LoopSimpleWhile`: `loop_routes::simple_while` (no break/continue)
//! - `LoopBreak`: `loop_routes::with_break` (conditional break)
//! - `IfPhiJoin`: `loop_routes::with_if_phi` (if + PHI merging)
//! - `LoopContinueOnly`: `loop_routes::with_continue` (deferred to Phase 195)
//! - `LoopTrueEarlyExit`: planned route family for `loop(true)` + early exit
//!
//! ## Why Separate Router?
//! See `if_lowering_router.rs` for rationale.
//! Summary: Orthogonal concerns, easier to maintain/extend.
//!
//! # Routing Strategy
//!
//! This router uses structure-based route classification (Phase 194):
//! 1. Extract CFG features from LoopForm
//! 2. Classify into a route family using `loop_route_detection::classify`
//! 3. Route to the appropriate lowerer
//!
//! # Phase 183: Unified Detection
//!
//! This router shares route classification logic with
//! `crate::mir::builder::control_flow::joinir::route_entry::router`.
//! Both use `loop_route_detection::classify()` for consistent classification.
//!
//! # Route Dispatch Order
//!
//! `loop_route_detection::classify()` chooses one route family, then this
//! router dispatches to that lowerer. The current match order is:
//! - `NestedLoopMinimal`
//! - `LoopContinueOnly`
//! - `IfPhiJoin`
//! - `LoopBreak`
//! - `LoopSimpleWhile`
//! - `LoopTrueEarlyExit` (stub / fallback)
//!
//! # Integration Points
//!
//! Called from:
//! - `loop_to_join::LoopToJoinLowerer::lower_loop()`
//! - `loop_form_intake.rs::handle_loop_form()`

use crate::mir::join_ir::JoinInst;
use crate::mir::loop_form::LoopForm;
use crate::runtime::get_global_ring0;

/// Phase 188: Try to lower loop to JoinIR using route-family-based dispatch
///
/// This function routes loop lowering to specific route handlers based on
/// loop structure characteristics. `loop_route_detection::classify()` decides
/// the route family, and this function dispatches that result to the matching
/// lowerer.
///
/// # Arguments
///
/// * `loop_form` - The loop structure to lower
/// * `lowerer` - The LoopToJoinLowerer builder (provides ValueId allocation, etc.)
///
/// # Returns
///
/// * `Some(JoinInst)` - Successfully lowered to JoinIR
/// * `None` - No route matched (fallback to existing lowering)
///
/// # Route Selection Strategy
///
/// If the selected route lowerer returns `None`, this function returns `None`
/// and the caller may fall back to the existing lowering path.
///
/// # Integration Point
///
/// This function should be called from loop lowering entry points:
/// - `loop_to_join::LoopToJoinLowerer::lower_loop()`
/// - `loop_form_intake.rs::handle_loop_form()`
///
/// # Example Usage
///
/// ```rust,ignore
/// use crate::mir::join_ir::lowering::try_lower_loop_route_to_joinir;
///
/// // In loop lowering entry point:
/// if let Some(joinir_inst) = try_lower_loop_route_to_joinir(&loop_form, &mut lowerer) {
///     // Route matched, use JoinIR
///     return Some(joinir_inst);
/// }
/// // No route matched, use existing lowering
/// existing_loop_lowering(&loop_form, &mut lowerer)
/// ```
///
/// # Reference
///
/// See design.md for complete pattern specifications and transformation rules:
/// `docs/private/roadmap2/phases/phase-188-joinir-loop-pattern-expansion/design.md`
pub fn try_lower_loop_route_to_joinir(
    loop_form: &LoopForm,
    lowerer: &mut crate::mir::join_ir::lowering::LoopToJoinLowerer,
) -> Option<JoinInst> {
    // Phase 194: Structure-based route classification
    // Tries routes based on CFG structure, not function names

    use crate::mir::loop_route_detection::{classify, extract_features, LoopRouteKind};

    // Step 1: Extract features from LoopForm (no LoopScope needed for now)
    let features = extract_features(loop_form);

    // Step 2: Classify route family based on structure
    let route_kind = classify(&features);

    // Step 3: Route to the appropriate lowerer
    match route_kind {
        LoopRouteKind::NestedLoopMinimal => {
            // Phase 188.2: NestedLoopMinimal lowering stub (infrastructure only)
            // classify() can reach NestedLoopMinimal when depth/features confirm
            // a minimal nested-loop route shape, but the lowerer is still a stub.
            #[cfg(debug_assertions)]
            if crate::config::env::joinir_dev::debug_enabled() {
                get_global_ring0().log.debug(
                    "[try_lower_loop_route] ℹ️ NestedLoopMinimal reached (lowerer still stubbed)",
                );
            }

            if let Some(inst) =
                super::loop_routes::lower_nested_loop_minimal_to_joinir(loop_form, lowerer)
            {
                return Some(inst);
            }
            // Stub returns None - fallback to existing lowering
        }
        LoopRouteKind::LoopContinueOnly => {
            if let Some(inst) =
                super::loop_routes::lower_loop_with_continue_to_joinir(loop_form, lowerer)
            {
                if crate::config::env::joinir_dev::debug_enabled() {
                    get_global_ring0()
                        .log
                        .debug("[try_lower_loop_route] ✅ LoopContinueOnly matched");
                }
                return Some(inst);
            }
        }
        LoopRouteKind::IfPhiJoin => {
            if let Some(inst) =
                super::loop_routes::lower_loop_with_conditional_phi_to_joinir(loop_form, lowerer)
            {
                if crate::config::env::joinir_dev::debug_enabled() {
                    get_global_ring0()
                        .log
                        .debug("[try_lower_loop_route] ✅ IfPhiJoin matched");
                }
                return Some(inst);
            }
        }
        LoopRouteKind::LoopBreak => {
            if let Some(inst) =
                super::loop_routes::lower_loop_with_break_to_joinir(loop_form, lowerer)
            {
                if crate::config::env::joinir_dev::debug_enabled() {
                    get_global_ring0()
                        .log
                        .debug("[try_lower_loop_route] ✅ LoopBreak matched");
                }
                return Some(inst);
            }
        }
        LoopRouteKind::LoopSimpleWhile => {
            if let Some(inst) = super::loop_routes::lower_simple_while_to_joinir(loop_form, lowerer)
            {
                if crate::config::env::joinir_dev::debug_enabled() {
                    get_global_ring0()
                        .log
                        .debug("[try_lower_loop_route] ✅ LoopSimpleWhile matched");
                }
                return Some(inst);
            }
        }
        LoopRouteKind::LoopTrueEarlyExit => {
            // Phase 131-11: Not implemented yet in LoopForm-based router
            if crate::config::env::joinir_dev::debug_enabled() {
                get_global_ring0().log.debug(
                    "[try_lower_loop_route] ⚠️ LoopTrueEarlyExit not implemented in LoopForm router",
                );
            }
        }
        LoopRouteKind::Unknown => {
            // Phase 188.1: Check for explicit rejection reasons (depth > 2)
            if features.max_loop_depth > 2 {
                if crate::config::env::joinir_dev::debug_enabled() {
                    let ring0 = get_global_ring0();
                    ring0.log.debug(&format!(
                        "[try_lower_loop_route] ❌ EXPLICIT ERROR: max_loop_depth={} exceeds limit (max=2)",
                        features.max_loop_depth
                    ));
                    ring0.log.debug(
                        "[try_lower_loop_route]   Hint: Nested loops with depth > 2 not supported in Phase 188.1",
                    );
                }
                // Fallback will trigger error (no silent Ok(None))
            } else {
                if crate::config::env::joinir_dev::debug_enabled() {
                    get_global_ring0().log.debug(
                        "[try_lower_loop_route] ❌ Unknown route, fallback to existing lowering",
                    );
                }
            }
        }
    }

    // No route matched (fallback to existing lowering)
    if crate::config::env::joinir_dev::debug_enabled() {
        get_global_ring0().log.debug(
            "[try_lower_loop_route] ❌ Route lowering failed, fallback to existing lowering",
        );
    }
    None
}
