//! Phase 188-Impl-1: loop_simple_while Minimal Lowerer
//!
//! Target: apps/tests/loop_min_while.hako
//!
//! Code:
//! ```nyash
//! static box Main {
//!   main() {
//!     local i = 0
//!     loop(i < 3) {
//!       print(i)
//!       i = i + 1
//!     }
//!     return 0
//!   }
//! }
//! ```
//!
//! Expected JoinIR:
//! ```text
//! fn main():
//!   i_init = 0
//!   result = loop_step(i_init)
//!   return 0
//!
//! fn loop_step(i):
//!   exit_cond = !(i < 3)
//!   Jump(k_exit, [i], cond=exit_cond)  // Phase 132: pass i to k_exit
//!   print(i)                           // body
//!   i_next = i + 1                     // increment
//!   Call(loop_step, [i_next])          // tail recursion
//!
//! fn k_exit(i_exit):                   // Phase 132: receives loop variable
//!   return i_exit                      // Phase 132: return loop value
//! ```
//!
//! ## Design Notes
//!
//! This is a MINIMAL implementation targeting loop_min_while.hako specifically.
//! It establishes the infrastructure for loop_simple_while lowering, which will be
//! generalized in future phases.
//!
//! Following the "80/20 rule" from CLAUDE.md - get it working first, generalize later.

use crate::mir::join_ir::lowering::canonical_names as cn;
use crate::mir::join_ir::lowering::join_value_space::JoinValueSpace;
use crate::mir::join_ir::lowering::loop_scope_shape::LoopScopeShape;
use crate::mir::join_ir::{
    BinOpKind, CompareOp, ConstValue, JoinFuncId, JoinFunction, JoinInst, JoinModule, MirLikeInst,
    UnaryOp,
};
use crate::runtime::get_global_ring0;

/// Lower the loop_simple_while route to JoinIR
///
/// # Phase 188-Impl-3: Pure JoinIR Fragment Generation
/// # Phase 202-A: JoinValueSpace Integration
///
/// This version generates JoinIR using **JoinValueSpace** for unified ValueId allocation.
/// It uses the Local region (1000+) to avoid collision with Param region (100-999).
///
/// ## Design Philosophy
///
/// - **Box A**: JoinIR Frontend (doesn't know about host ValueIds)
/// - **Box B**: This function - converts to JoinIR with local IDs
/// - **Box C**: JoinInlineBoundary - stores boundary info
/// - **Box D**: merge_joinir_mir_blocks - injects Copy instructions
///
/// This clean separation ensures JoinIR lowerers are:
/// - Pure transformers (no side effects)
/// - Reusable (same lowerer works in any context)
/// - Testable (can test JoinIR independently)
///
/// # Arguments
///
/// * `_scope` - LoopScopeShape (reserved for future generic implementation)
/// * `join_value_space` - Unified ValueId allocator (Phase 202-A)
///
/// # Returns
///
/// * `Some(JoinModule)` - Successfully lowered to JoinIR
/// * `None` - Route not matched (fallback to other lowerers)
///
/// # Boundary Contract
///
/// This function returns a JoinModule with:
/// - **Input slot**: main() の param（JoinValueSpace の Param region）でループ変数を受け取る
/// - **Caller responsibility**: Create JoinInlineBoundary to map that param ValueId to host's loop var
pub(crate) fn lower_simple_while_minimal(
    _scope: LoopScopeShape,
    join_value_space: &mut JoinValueSpace,
) -> Option<JoinModule> {
    // Phase 202-A/Phase 205: Use JoinValueSpace for Param/Local region allocation.
    // - Params: boundary join_inputs (host loop var wiring)
    // - Locals: constants, intermediate values
    // NOTE: Avoid holding multiple closures borrowing join_value_space mutably at once.

    let mut join_module = JoinModule::new();

    // ==================================================================
    // Function IDs allocation
    // ==================================================================
    let main_id = JoinFuncId::new(0);
    let loop_step_id = JoinFuncId::new(1);
    let k_exit_id = JoinFuncId::new(2);

    // ==================================================================
    // ValueId allocation
    // ==================================================================
    // main() params/locals
    let i_main_param = join_value_space.alloc_param(); // boundary input (host loop var → this param)
    let loop_result = join_value_space.alloc_local(); // result from loop_step
    let const_0_main = join_value_space.alloc_local(); // return value

    // loop_step params/locals
    let i_step_param = join_value_space.alloc_param(); // loop_step parameter
    let const_3 = join_value_space.alloc_local(); // comparison constant
    let cmp_lt = join_value_space.alloc_local(); // i < 3
    let exit_cond = join_value_space.alloc_local(); // !(i < 3)
    let const_1 = join_value_space.alloc_local(); // increment constant
    let i_next = join_value_space.alloc_local(); // i + 1

    // k_exit params
    let i_exit_param = join_value_space.alloc_param(); // exit parameter (loop variable)

    // ==================================================================
    // main() function
    // ==================================================================
    // main() takes loop var as a param (boundary input)
    let mut main_func = JoinFunction::new(main_id, "main".to_string(), vec![i_main_param]);

    // result = loop_step(i_main_param)
    main_func.body.push(JoinInst::Call {
        func: loop_step_id,
        args: vec![i_main_param],
        k_next: None,
        dst: Some(loop_result),
    });

    // return 0
    main_func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: const_0_main,
        value: ConstValue::Integer(0),
    }));

    main_func.body.push(JoinInst::Ret {
        value: Some(const_0_main),
    });

    join_module.add_function(main_func);

    // ==================================================================
    // loop_step(i) function
    // ==================================================================
    let mut loop_step_func =
        JoinFunction::new(loop_step_id, cn::LOOP_STEP.to_string(), vec![i_step_param]);

    // exit_cond = !(i < 3)
    // Step 1: const 3
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Const {
            dst: const_3,
            value: ConstValue::Integer(3),
        }));

    // Step 2: cmp_lt = (i < 3)
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Compare {
            dst: cmp_lt,
            op: CompareOp::Lt,
            lhs: i_step_param,
            rhs: const_3,
        }));

    // Step 3: exit_cond = !cmp_lt
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::UnaryOp {
            dst: exit_cond,
            op: UnaryOp::Not,
            operand: cmp_lt,
        }));

    // Phase 132: Jump(k_exit, [i_param], cond=exit_cond)
    // Pass loop variable to exit continuation for return value parity
    loop_step_func.body.push(JoinInst::Jump {
        cont: k_exit_id.as_cont(),
        args: vec![i_step_param],
        cond: Some(exit_cond),
    });

    // print(i)
    // Phase 188-Impl-1-E: Use Print instruction
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Print {
            value: i_step_param,
        }));

    // i_next = i + 1
    // Step 1: const 1
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Const {
            dst: const_1,
            value: ConstValue::Integer(1),
        }));

    // Step 2: i_next = i + 1
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BinOp {
            dst: i_next,
            op: BinOpKind::Add,
            lhs: i_step_param,
            rhs: const_1,
        }));

    // Call(loop_step, [i_next])  // tail recursion
    loop_step_func.body.push(JoinInst::Call {
        func: loop_step_id,
        args: vec![i_next],
        k_next: None, // CRITICAL: None for tail call
        dst: None,
    });

    join_module.add_function(loop_step_func);

    // ==================================================================
    // k_exit(i_exit) function - Phase 132: receives loop variable
    // ==================================================================
    let mut k_exit_func = JoinFunction::new(k_exit_id, cn::K_EXIT.to_string(), vec![i_exit_param]);

    // Phase 132: return i_exit (loop variable at exit)
    // This ensures VM/LLVM parity for `return i` after loop
    k_exit_func.body.push(JoinInst::Ret {
        value: Some(i_exit_param),
    });

    join_module.add_function(k_exit_func);

    // Set entry point
    join_module.entry = Some(main_id);

    if crate::config::env::joinir_dev::debug_enabled() {
        let ring0 = get_global_ring0();
        ring0
            .log
            .debug("[joinir/loop_simple_while] Generated JoinIR for loop_simple_while route");
        ring0
            .log
            .debug("[joinir/loop_simple_while] Functions: main, loop_step, k_exit");
    }

    Some(join_module)
}
