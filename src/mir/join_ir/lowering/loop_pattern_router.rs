//! # Loop Pattern JoinIR Lowering Router
//!
//! **Phase 33-12 Modularization**: Extracted from `mod.rs` (lines 424-511)
//!
//! ## Responsibility
//! Routes loop patterns to appropriate JoinIR lowering strategies.
//! This is the **main entry point** for loop → JoinIR lowering.
//!
//! ## Route Dispatch
//! Routes to:
//! - Pattern 1 / `LoopSimpleWhile`: `loop_patterns::simple_while` (no break/continue)
//! - `LoopBreak`: `loop_patterns::with_break` (conditional break)
//! - `IfPhiJoin`: `loop_patterns::with_if_phi` (if + PHI merging)
//! - `LoopContinueOnly`: `loop_patterns::with_continue` (deferred to Phase 195)
//!
//! ## Why Separate Router?
//! See `if_lowering_router.rs` for rationale.
//! Summary: Orthogonal concerns, easier to maintain/extend.
//!
//! # Routing Strategy
//!
//! This router uses structure-based pattern classification (Phase 194):
//! 1. Extract CFG features from LoopForm
//! 2. Classify into pattern kind (1-4 or Unknown) using `loop_pattern_detection::classify`
//! 3. Route to appropriate pattern lowerer
//!
//! # Phase 183: Unified Detection
//!
//! This router shares pattern detection logic with `patterns/router.rs`.
//! Both use `loop_pattern_detection::classify()` for consistent classification.
//!
//! # Route Priority (Phase 188)
//!
//! Route families are tried in complexity order:
//! - **LoopContinueOnly** (highest complexity)
//! - **IfPhiJoin** (leverages If lowering)
//! - **LoopBreak** (medium complexity)
//! - **Pattern 1 / LoopSimpleWhile** (foundational, easiest)
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
/// This function routes loop lowering to specific pattern handlers based on
/// loop structure characteristics. It tries route families in order of complexity:
///
/// 1. **Pattern 1 / LoopSimpleWhile** (foundational, easiest)
/// 2. **LoopBreak** (medium complexity)
/// 3. **IfPhiJoin** (leverages existing If lowering)
/// 4. **LoopContinueOnly** (highest complexity)
///
/// # Arguments
///
/// * `loop_form` - The loop structure to lower
/// * `lowerer` - The LoopToJoinLowerer builder (provides ValueId allocation, etc.)
///
/// # Returns
///
/// * `Some(JoinInst)` - Successfully lowered to JoinIR
/// * `None` - No pattern matched (fallback to existing lowering)
///
/// # Route Selection Strategy
///
/// Route families are tried sequentially. First matching route wins.
/// If no route matches, returns `None` to trigger fallback.
///
/// ## Pattern 1: Simple While Loop
/// - **Condition**: Empty break/continue targets, single latch
/// - **Handler**: `loop_patterns::lower_simple_while_to_joinir()`
/// - **Priority**: First (most common, simplest)
///
/// ## LoopBreak
/// - **Condition**: Non-empty break_targets, exactly 1 break
/// - **Handler**: `loop_patterns::lower_loop_with_break_to_joinir()`
/// - **Priority**: Second (common, medium complexity)
///
/// ## Pattern 3: Loop with If-Else PHI
/// - **Condition**: Empty break/continue, if-else in body
/// - **Handler**: `loop_patterns::lower_loop_with_conditional_phi_to_joinir()`
/// - **Priority**: Third (reuses If lowering infrastructure)
///
/// ## Pattern 4: Loop with Continue
/// - **Condition**: Non-empty continue_targets
/// - **Handler**: `loop_patterns::lower_loop_with_continue_to_joinir()`
/// - **Priority**: Fourth (most complex)
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
/// use crate::mir::join_ir::lowering::try_lower_loop_pattern_to_joinir;
///
/// // In loop lowering entry point:
/// if let Some(joinir_inst) = try_lower_loop_pattern_to_joinir(&loop_form, &mut lowerer) {
///     // Pattern matched, use JoinIR
///     return Some(joinir_inst);
/// }
/// // No pattern matched, use existing lowering
/// existing_loop_lowering(&loop_form, &mut lowerer)
/// ```
///
/// # Reference
///
/// See design.md for complete pattern specifications and transformation rules:
/// `docs/private/roadmap2/phases/phase-188-joinir-loop-pattern-expansion/design.md`
pub fn try_lower_loop_pattern_to_joinir(
    loop_form: &LoopForm,
    lowerer: &mut crate::mir::join_ir::lowering::LoopToJoinLowerer,
) -> Option<JoinInst> {
    // Phase 194: Structure-based pattern classification
    // Tries patterns based on CFG structure, not function names

    use crate::mir::loop_pattern_detection::{classify, extract_features, LoopPatternKind};

    // Step 1: Extract features from LoopForm (no LoopScope needed for now)
    let features = extract_features(loop_form, None);

    // Step 2: Classify pattern based on structure
    let pattern = classify(&features);

    // Step 3: Route to appropriate lowerer based on pattern
    match pattern {
        LoopPatternKind::NestedLoopMinimal => {
            // Phase 188.2: NestedLoopMinimal lowering stub (infrastructure only)
            // Currently unreachable: LoopForm has no nesting info, so classify() never returns NestedLoopMinimal
            #[cfg(debug_assertions)]
            if crate::config::env::joinir_dev::debug_enabled() {
                get_global_ring0().log.debug(
                    "[try_lower_loop_pattern] ℹ️ NestedLoopMinimal reached (should be unreachable until Phase 188.3)",
                );
            }

            if let Some(inst) =
                super::loop_patterns::lower_nested_loop_minimal_to_joinir(loop_form, lowerer)
            {
                return Some(inst);
            }
            // Stub returns None - fallback to existing lowering
        }
        LoopPatternKind::LoopContinueOnly => {
            if let Some(inst) =
                super::loop_patterns::lower_loop_with_continue_to_joinir(loop_form, lowerer)
            {
                if crate::config::env::joinir_dev::debug_enabled() {
                    get_global_ring0()
                        .log
                        .debug("[try_lower_loop_pattern] ✅ LoopContinueOnly matched");
                }
                return Some(inst);
            }
        }
        LoopPatternKind::IfPhiJoin => {
            if let Some(inst) =
                super::loop_patterns::lower_loop_with_conditional_phi_to_joinir(loop_form, lowerer)
            {
                if crate::config::env::joinir_dev::debug_enabled() {
                    get_global_ring0()
                        .log
                        .debug("[try_lower_loop_pattern] ✅ IfPhiJoin matched");
                }
                return Some(inst);
            }
        }
        LoopPatternKind::LoopBreak => {
            if let Some(inst) =
                super::loop_patterns::lower_loop_with_break_to_joinir(loop_form, lowerer)
            {
                if crate::config::env::joinir_dev::debug_enabled() {
                    get_global_ring0()
                        .log
                        .debug("[try_lower_loop_pattern] ✅ LoopBreak matched");
                }
                return Some(inst);
            }
        }
        LoopPatternKind::LoopSimpleWhile => {
            if let Some(inst) =
                super::loop_patterns::lower_simple_while_to_joinir(loop_form, lowerer)
            {
                if crate::config::env::joinir_dev::debug_enabled() {
                    get_global_ring0()
                        .log
                        .debug("[try_lower_loop_pattern] ✅ LoopSimpleWhile matched");
                }
                return Some(inst);
            }
        }
        LoopPatternKind::InfiniteEarlyExit => {
            // Phase 131-11: Not implemented yet in LoopForm-based router
            if crate::config::env::joinir_dev::debug_enabled() {
                get_global_ring0().log.debug(
                    "[try_lower_loop_pattern] ⚠️ Pattern 5 (InfiniteEarlyExit) not implemented in LoopForm router",
                );
            }
        }
        LoopPatternKind::Unknown => {
            // Phase 188.1: Check for explicit rejection reasons (depth > 2)
            if features.max_loop_depth > 2 {
                if crate::config::env::joinir_dev::debug_enabled() {
                    let ring0 = get_global_ring0();
                    ring0.log.debug(&format!(
                        "[try_lower_loop_pattern] ❌ EXPLICIT ERROR: max_loop_depth={} exceeds limit (max=2)",
                        features.max_loop_depth
                    ));
                    ring0.log.debug(
                        "[try_lower_loop_pattern]   Hint: Nested loops with depth > 2 not supported in Phase 188.1",
                    );
                }
                // Fallback will trigger error (no silent Ok(None))
            } else {
                if crate::config::env::joinir_dev::debug_enabled() {
                    get_global_ring0().log.debug(
                        "[try_lower_loop_pattern] ❌ Unknown pattern, fallback to existing lowering",
                    );
                }
            }
        }
    }

    // No Pattern Matched (fallback to existing lowering)
    // ===================================================
    if crate::config::env::joinir_dev::debug_enabled() {
        get_global_ring0().log.debug(
            "[try_lower_loop_pattern] ❌ Pattern lowering failed, fallback to existing lowering",
        );
    }
    None
}
