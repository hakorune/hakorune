//! Phase 254 P1: scan_with_init Minimal Lowerer
//!
//! Target: apps/tests/phase254_p0_index_of_min.hako
//!
//! Code:
//! ```nyash
//! static box StringUtils {
//!   index_of(s, ch) {
//!     local i = 0
//!     loop(i < s.length()) {
//!       if s.substring(i, i + 1) == ch {
//!         return i
//!       }
//!       i = i + 1
//!     }
//!     return -1
//!   }
//! }
//! ```
//!
//! Expected JoinIR:
//! ```text
//! fn main(s, ch, i):
//!   result = loop_step(s, ch, i)
//!   // Post-loop early return will be inserted by MirBuilder
//!
//! fn loop_step(s, ch, i):
//!   // 1. Check exit condition: i >= s.length()
//!   len = StringBox.length(s)
//!   exit_cond = (i >= len)
//!   Jump(k_exit, [-1], cond=exit_cond)  // Not found case
//!
//!   // 2. Calculate i_plus_1 for substring
//!   i_plus_1 = i + 1
//!
//!   // 3. Hoist MethodCall(substring) to init-time BoxCall
//!   cur = StringBox.substring(s, i, i_plus_1)
//!
//!   // 4. Check match condition
//!   match = (cur == ch)
//!   Jump(k_exit, [i], cond=match)  // Found case
//!
//!   // 5. Tail recursion
//!   Call(loop_step, [s, ch, i_plus_1])
//!
//! fn k_exit(i_exit):
//!   return i_exit
//! ```
//!
//! ## Design Notes
//!
//! This is a MINIMAL P0 implementation targeting index_of pattern specifically.
//! Key features:
//! - substring is emitted as BoxCall (init-time, not condition whitelist)
//! - Two Jump instructions to k_exit (not found: -1, found: i)
//! - Step must be 1 (P0 restriction)
//! - not_found_return_lit must be -1 (P0 restriction)

use crate::mir::join_ir::lowering::canonical_names as cn;
use crate::mir::join_ir::lowering::join_value_space::JoinValueSpace;
use crate::mir::join_ir::{
    BinOpKind, CompareOp, ConstValue, JoinFuncId, JoinFunction, JoinInst, JoinModule, MirLikeInst,
};
use crate::runtime::get_global_ring0;

/// Lower the scan_with_init route to JoinIR
///
/// # Phase 254 P1: Pure JoinIR Fragment Generation
///
/// This version generates JoinIR using **JoinValueSpace** for unified ValueId allocation.
/// It uses the Param region (100+) for function parameters and Local region (1000+) for
/// temporary values.
///
/// ## Design Philosophy
///
/// Following the loop_simple_while lowerer's architecture:
/// - **Pure transformer**: No side effects, only JoinIR generation
/// - **Reusable**: Works in any context with proper boundary
/// - **Testable**: Can test JoinIR independently
///
/// ## Boundary Contract
///
/// This function returns a JoinModule with:
/// - **Input slots**: main() params for (s, ch, i)
/// - **Caller responsibility**: Create JoinInlineBoundary to map params to host variables
/// - **Exit binding**: k_exit param receives found index or -1
///
/// # Arguments
///
/// * `join_value_space` - Unified ValueId allocator (Phase 202-A)
///
/// # Returns
///
/// * `JoinModule` - Successfully lowered to JoinIR
#[allow(dead_code)]
pub(crate) fn lower_scan_with_init_minimal(
    join_value_space: &mut JoinValueSpace,
    dynamic_needle: bool, // Phase 258 P0: true if substr.length(), false if fixed (ch)
) -> JoinModule {
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
    // Phase 255 P0: Loop variable MUST be first, then alphabetical order [ch, s]
    // (CarrierInfo sorts carriers alphabetically for determinism)
    let i_main_param = join_value_space.alloc_param(); // loop index
    let ch_main_param = join_value_space.alloc_param(); // needle character (alphabetically first)
    let s_main_param = join_value_space.alloc_param(); // haystack string (alphabetically second)
    let loop_result = join_value_space.alloc_local(); // result from loop_step

    // loop_step params/locals
    // Phase 255 P0: Loop variable MUST be first, then alphabetical order [ch, s]
    let i_step_param = join_value_space.alloc_param(); // loop index
    let ch_step_param = join_value_space.alloc_param(); // needle (alphabetically first)
    let s_step_param = join_value_space.alloc_param(); // haystack (alphabetically second)
    let len = join_value_space.alloc_local(); // s.length()
    let exit_cond = join_value_space.alloc_local(); // i >= len
    let const_minus_1 = join_value_space.alloc_local(); // -1 for not found
    let const_1 = join_value_space.alloc_local(); // 1 for increment
    let i_plus_1 = join_value_space.alloc_local(); // i + 1
    let cur = join_value_space.alloc_local(); // substring result
    let match_cond = join_value_space.alloc_local(); // cur == ch

    // Phase 258 P0: Conditional allocation for dynamic needle
    let (needle_len, bound) = if dynamic_needle {
        (
            Some(join_value_space.alloc_local()), // substr.length()
            Some(join_value_space.alloc_local()), // len - needle_len
        )
    } else {
        (None, None)
    };

    // k_exit params
    let i_exit_param = join_value_space.alloc_param(); // exit parameter (index or -1)

    // ==================================================================
    // main() function
    // ==================================================================
    let mut main_func = JoinFunction::new(
        main_id,
        "main".to_string(),
        vec![i_main_param, ch_main_param, s_main_param],  // Phase 255 P0: [i, ch, s] alphabetical
    );

    // result = loop_step(i, ch, s)  // Phase 255 P0: alphabetical order
    main_func.body.push(JoinInst::Call {
        func: loop_step_id,
        args: vec![i_main_param, ch_main_param, s_main_param],  // Phase 255 P0: [i, ch, s] alphabetical
        k_next: None,
        dst: Some(loop_result),
    });

    // Return loop_result (found index or -1)
    main_func.body.push(JoinInst::Ret { value: Some(loop_result) });

    join_module.add_function(main_func);

    // ==================================================================
    // loop_step(i, ch, s) function
    // ==================================================================
    // Phase 255 P0: Loop variable first, then alphabetical [ch, s]
    let mut loop_step_func = JoinFunction::new(
        loop_step_id,
        cn::LOOP_STEP.to_string(),
        vec![i_step_param, ch_step_param, s_step_param],  // Phase 255 P0: [i, ch, s] alphabetical
    );

    // 1. len = s.length()
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BoxCall {
            dst: Some(len),
            box_name: "StringBox".to_string(),
            method: "length".to_string(),
            args: vec![s_step_param],
        }));

    // Phase 258 P0: Dynamic needle support - compute exit condition and window size
    let (exit_cond_rhs, i_plus_n) = if dynamic_needle {
        // 2a. needle_len = substr.length()
        loop_step_func
            .body
            .push(JoinInst::Compute(MirLikeInst::BoxCall {
                dst: Some(needle_len.unwrap()),
                box_name: "StringBox".to_string(),
                method: "length".to_string(),
                args: vec![ch_step_param], // ch_step_param is actually substr in dynamic mode
            }));

        // 2b. bound = len - needle_len
        loop_step_func
            .body
            .push(JoinInst::Compute(MirLikeInst::BinOp {
                dst: bound.unwrap(),
                op: BinOpKind::Sub,
                lhs: len,
                rhs: needle_len.unwrap(),
            }));

        // 2c. const 1 (for loop increment)
        loop_step_func
            .body
            .push(JoinInst::Compute(MirLikeInst::Const {
                dst: const_1,
                value: ConstValue::Integer(1),
            }));

        // 2d. i_plus_1 = i + 1 (loop increment)
        loop_step_func
            .body
            .push(JoinInst::Compute(MirLikeInst::BinOp {
                dst: i_plus_1,
                op: BinOpKind::Add,
                lhs: i_step_param,
                rhs: const_1,
            }));

        // 2e. i_plus_needle_len = i + needle_len (substring window end)
        let i_plus_n_id = join_value_space.alloc_local();
        loop_step_func
            .body
            .push(JoinInst::Compute(MirLikeInst::BinOp {
                dst: i_plus_n_id,
                op: BinOpKind::Add,
                lhs: i_step_param,
                rhs: needle_len.unwrap(),
            }));

        (bound.unwrap(), i_plus_n_id)
    } else {
        // 2a. Fixed: const 1
        loop_step_func
            .body
            .push(JoinInst::Compute(MirLikeInst::Const {
                dst: const_1,
                value: ConstValue::Integer(1),
            }));

        // 2b. i_plus_1 = i + 1 (both loop increment and substring window end)
        loop_step_func
            .body
            .push(JoinInst::Compute(MirLikeInst::BinOp {
                dst: i_plus_1,
                op: BinOpKind::Add,
                lhs: i_step_param,
                rhs: const_1,
            }));

        (len, i_plus_1) // exit_cond = (i >= len)
    };

    // 3. exit_cond comparison (dynamic: i > bound, fixed: i >= len)
    if dynamic_needle {
        loop_step_func
            .body
            .push(JoinInst::Compute(MirLikeInst::Compare {
                dst: exit_cond,
                op: CompareOp::Gt,
                lhs: i_step_param,
                rhs: exit_cond_rhs,
            }));
    } else {
        loop_step_func
            .body
            .push(JoinInst::Compute(MirLikeInst::Compare {
                dst: exit_cond,
                op: CompareOp::Ge,
                lhs: i_step_param,
                rhs: exit_cond_rhs,
            }));
    }

    // 4. const -1
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Const {
            dst: const_minus_1,
            value: ConstValue::Integer(-1),
        }));

    // 5. Jump(k_exit, [-1], cond=exit_cond) - not found case
    loop_step_func.body.push(JoinInst::Jump {
        cont: k_exit_id.as_cont(),
        args: vec![const_minus_1],
        cond: Some(exit_cond),
    });

    // 6. cur = s.substring(i, i_plus_N) - dynamic window
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BoxCall {
            dst: Some(cur),
            box_name: "StringBox".to_string(),
            method: "substring".to_string(),
            args: vec![s_step_param, i_step_param, i_plus_n],
        }));

    // 7. match_cond = (cur == ch)
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Compare {
            dst: match_cond,
            op: CompareOp::Eq,
            lhs: cur,
            rhs: ch_step_param,
        }));

    // 8. Jump(k_exit, [i], cond=match_cond) - found case
    loop_step_func.body.push(JoinInst::Jump {
        cont: k_exit_id.as_cont(),
        args: vec![i_step_param],
        cond: Some(match_cond),
    });

    // 9. Call(loop_step, [i_plus_1, ch, s]) - tail recursion
    // Phase 255 P0: Loop variable first, then alphabetical [ch, s]
    loop_step_func.body.push(JoinInst::Call {
        func: loop_step_id,
        args: vec![i_plus_1, ch_step_param, s_step_param],  // Phase 255 P0: [i_plus_1, ch, s] alphabetical
        k_next: None, // CRITICAL: None for tail call
        dst: None,
    });

    join_module.add_function(loop_step_func);

    // ==================================================================
    // k_exit(i_exit) function
    // ==================================================================
    let mut k_exit_func = JoinFunction::new(k_exit_id, cn::K_EXIT.to_string(), vec![i_exit_param]);

    // Return i_exit (found index or -1)
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
            .debug("[joinir/scan_with_init] Generated JoinIR for scan_with_init route");
        ring0
            .log
            .debug("[joinir/scan_with_init] Functions: main, loop_step, k_exit");
        ring0
            .log
            .debug("[joinir/scan_with_init] BoxCall: substring (init-time, not condition whitelist)");
    }

    join_module
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lower_scan_with_init_minimal() {
        let mut join_value_space = JoinValueSpace::new();

        let join_module = lower_scan_with_init_minimal(&mut join_value_space, false);

        // main + loop_step + k_exit の3関数
        assert_eq!(join_module.functions.len(), 3);

        // Entry が main(0) に設定されている
        assert_eq!(join_module.entry, Some(JoinFuncId::new(0)));

        // k_exit 関数が取れる
        let k_exit_func = join_module
            .functions
            .get(&JoinFuncId::new(2))
            .expect("k_exit function should exist");
        assert_eq!(k_exit_func.name, cn::K_EXIT);
    }

    #[test]
    fn test_loop_step_has_substring_box_call() {
        let mut join_value_space = JoinValueSpace::new();

        let join_module = lower_scan_with_init_minimal(&mut join_value_space, false);

        // loop_step 関数を取得
        let loop_step = join_module
            .functions
            .get(&JoinFuncId::new(1))
            .expect("loop_step function should exist");

        // BoxCall(substring) が含まれることを確認
        let has_substring = loop_step.body.iter().any(|inst| {
            matches!(
                inst,
                JoinInst::Compute(MirLikeInst::BoxCall { method, .. })
                if method == "substring"
            )
        });

        assert!(
            has_substring,
            "loop_step should contain substring BoxCall"
        );
    }

    #[test]
    fn test_loop_step_has_exit_jumps() {
        let mut join_value_space = JoinValueSpace::new();

        let join_module = lower_scan_with_init_minimal(&mut join_value_space, false);

        // loop_step 関数を取得
        let loop_step = join_module
            .functions
            .get(&JoinFuncId::new(1))
            .expect("loop_step function should exist");

        // Jump(k_exit, ...) が2つ含まれることを確認
        let exit_jump_count = loop_step
            .body
            .iter()
            .filter(|inst| {
                matches!(
                    inst,
                    JoinInst::Jump { cont, .. }
                    if *cont == JoinFuncId::new(2).as_cont()
                )
            })
            .count();

        assert_eq!(
            exit_jump_count, 2,
            "loop_step should have 2 exit jumps"
        );
    }
}
