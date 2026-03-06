//! IfPhiJoin route lowering (if-else carrier merge)
//!
//! Target: Loops with if expressions inside that merge values
//! Example: `while(i < 10) { if x { y = 1 } else { y = 2 } }`
//!
//! # Transformation
//!
//! ```text
//! fn loop_step(i, sum):
//!   exit_cond = !(i <= 5)
//!   Jump(k_exit, [sum], cond=exit_cond)
//!   sum_new = Select(cond=(i%2==1), then=sum+i, else=sum+0)
//!   i_next = i + 1
//!   Call(loop_step, [i_next, sum_new])
//! ```

use crate::mir::join_ir::lowering::loop_to_join::LoopToJoinLowerer;
use crate::mir::join_ir::JoinInst;
use crate::mir::loop_form::LoopForm;
use crate::runtime::get_global_ring0;

/// Lowering for IfPhiJoin route
///
/// # Transformation (Pseudocode from design.md)
///
/// ```text
/// fn loop_step(i, sum):
///   exit_cond = !(i <= 5)
///   Jump(k_exit, [sum], cond=exit_cond)
///   sum_new = Select(cond=(i%2==1), then=sum+i, else=sum+0)
///   i_next = i + 1
///   Call(loop_step, [i_next, sum_new])
/// ```
///
/// # Steps (from design.md § IfPhiJoin)
///
/// 1. **Extract Loop Variables** (multiple carriers: i + sum)
/// 2. **Create loop_step Function** (params: i, sum, k_exit)
/// 3. **Create k_exit with Exit PHI** (receives sum exit value)
/// 4. **Generate Exit Condition Check** (same as LoopSimpleWhile route)
/// 5. **Translate If-Else to Select**
///    - Use existing If lowering (Phase 33: Select/IfMerge)
///    - Generate: `sum_new = Select(cond, then_val, else_val)`
/// 6. **Translate Loop Body** (remaining instructions)
/// 7. **Generate Tail Recursion** (with multiple carriers: i_next, sum_new)
///
/// # Key Difference from LoopSimpleWhile route
///
/// - **Multiple Carrier Variables**: Loop updates i + sum
/// - **In-Loop If Lowering**: Reuses existing Select/IfMerge lowering
/// - **PHI in Loop Body**: If-else assigns to same variable (becomes Select)
///
/// # Arguments
///
/// * `loop_form` - The loop structure to lower (must have if-else in body)
/// * `lowerer` - The LoopToJoinLowerer builder
///
/// # Returns
///
/// * `Some(JoinInst)` - Lowering succeeded, returns generated JoinIR instruction
/// * `None` - Lowering failed (route shape not matched or unsupported)
///
/// # Errors
///
/// Returns `None` if:
/// - Loop has breaks or continues
/// - If-else does not assign to same variable
/// - If context cannot be resolved
///
/// # Reference
///
/// See design.md § IfPhiJoin for full pseudocode.
///
/// # Example Usage
///
/// ```rust,ignore
/// use crate::mir::loop_pattern_detection::is_loop_with_conditional_phi_pattern;
///
/// if is_loop_with_conditional_phi_pattern(&loop_form) {
///     lower_loop_with_conditional_phi_to_joinir(&loop_form, &mut lowerer)?;
/// }
/// ```
pub fn lower_loop_with_conditional_phi_to_joinir(
    _loop_form: &LoopForm,
    _lowerer: &mut LoopToJoinLowerer,
) -> Option<JoinInst> {
    // Phase 242-EX-A: Legacy stub removed
    // IfPhiJoin route is now fully handled via router → legacy if-phi lowerer
    // → `loop_with_if_phi_if_sum.rs`.
    // This stub function is unused and kept only for API compatibility
    if crate::config::env::joinir_dev::debug_enabled() {
        get_global_ring0()
            .log
            .debug(
                "[loop_patterns] IfPhiJoin route: stub (routing via legacy if-phi lowerer)",
            );
    }
    None
}
