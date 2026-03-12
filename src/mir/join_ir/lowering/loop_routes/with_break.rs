//! LoopBreak route lowering
//!
//! Target: Loops with conditional break statements
//! Example: `while(i < 10) { if i >= 5 { break } i = i + 1 }`
//!
//! # Transformation
//!
//! ```text
//! fn loop_step(i):
//!   exit_cond = !(i < 3)
//!   Jump(k_exit, [i], cond=exit_cond)     // Natural exit
//!   break_cond = (i >= 2)
//!   Jump(k_exit, [i], cond=break_cond)    // Break exit
//!   i_next = i + 1
//!   Call(loop_step, [i_next])
//! ```

use crate::mir::join_ir::lowering::loop_to_join::LoopToJoinLowerer;
use crate::mir::join_ir::JoinInst;
use crate::mir::loop_form::LoopForm;
use crate::runtime::get_global_ring0;

/// Lowering for LoopBreak route (conditional break)
///
/// # Transformation (Pseudocode from design.md)
///
/// ```text
/// fn loop_step(i):
///   exit_cond = !(i < 3)
///   Jump(k_exit, [i], cond=exit_cond)     // Natural exit
///   break_cond = (i >= 2)
///   Jump(k_exit, [i], cond=break_cond)    // Break exit
///   i_next = i + 1
///   Call(loop_step, [i_next])
/// ```
///
/// # Steps (from design.md § LoopBreak)
///
/// 1. **Extract Loop Variables** (same as LoopSimpleWhile route)
/// 2. **Create loop_step Function** (same as LoopSimpleWhile route)
/// 3. **Create k_exit with Exit PHI**
///    - k_exit(i_exit) - receives exit value from both exit paths
/// 4. **Generate Natural Exit Check** (same as LoopSimpleWhile route)
/// 5. **Generate Break Check**
///    - Extract break condition: `break_cond = (i >= 2)`
///    - Add conditional Jump to k_exit: `Jump(k_exit, [i], cond=break_cond)`
/// 6. **Translate Loop Body** (same as LoopSimpleWhile route)
/// 7. **Generate Tail Recursion** (same as LoopSimpleWhile route)
///
/// # Key Difference from LoopSimpleWhile route
///
/// - **Multiple Exit Paths**: Natural exit + break exit
/// - **Exit PHI**: k_exit receives exit value from both paths
/// - **Sequential Jumps**: Natural exit check → break check → body
///
/// # Arguments
///
/// * `loop_form` - The loop structure to lower (must have break_targets)
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
/// - Loop has no breaks (use LoopSimpleWhile route instead)
/// - Loop has multiple break targets (not yet supported)
/// - Break is not in an if statement
///
/// # Reference
///
/// See design.md § LoopBreak for full pseudocode.
///
/// # Example Usage
///
/// ```rust,ignore
/// use crate::mir::loop_route_detection::is_loop_break_route;
///
/// if is_loop_break_route(&loop_form) {
///     lower_loop_with_break_to_joinir(&loop_form, &mut lowerer)?;
/// }
/// ```
pub fn lower_loop_with_break_to_joinir(
    _loop_form: &LoopForm,
    _lowerer: &mut LoopToJoinLowerer,
) -> Option<JoinInst> {
    // Phase 203-A: STUB FUNCTION - Called by router but always returns None
    //
    // Status: This function is called by loop_route_router.rs but is a NO-OP stub.
    // The actual LoopBreak route lowering happens via control_flow.rs.
    //
    // Why this stub exists:
    // - Router expects unified interface: lower_*_to_joinir(loop_form, lowerer)
    // - LoopBreak route is tightly integrated with control_flow.rs
    // - Removing it would require updating router dispatch logic
    //
    // Current behavior:
    // 1. Router calls this function from loop_route_router.rs
    // 2. Function logs a message and returns None
    // 3. Router falls back to control_flow.rs hardcoded LoopBreak route
    //
    // Migration options (future):
    // - Option 1: Remove stub and update router to call control_flow.rs directly
    // - Option 2: Implement JoinModule → JoinInst conversion here
    //
    // Related code:
    // - Router callsite: loop_route_router.rs
    // - Actual implementation: Plan/Composer route (PlanLowerer)
    // - Minimal lowerer: loop_with_break_minimal::lower_loop_with_break_minimal()

    if crate::config::env::joinir_dev::debug_enabled() {
        get_global_ring0()
            .log
            .debug("[loop_routes] LoopBreak route: lowering delegated to control_flow.rs (stub)");
    }
    None
}
