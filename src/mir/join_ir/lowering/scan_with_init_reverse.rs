//! Phase 257 P0: scan_with_init reverse lowerer (last_index_of)
//!
//! Target: apps/tests/phase257_p0_last_index_of_min.hako
//!
//! Code:
//! ```nyash
//! static box Main {
//!   main() {
//!     local s = "hello world"
//!     local ch = "o"
//!     local i = s.length() - 1
//!     loop(i >= 0) {
//!       if s.substring(i, i + 1) == ch {
//!         return i
//!       }
//!       i = i - 1
//!     }
//!     return -1
//!   }
//! }
//! ```
//!
//! Expected JoinIR:
//! ```text
//! fn main(i, ch, s):
//!   result = loop_step(i, ch, s)
//!
//! fn loop_step(i, ch, s):
//!   // 1. Check exit condition: i < 0
//!   exit_cond = (i < 0)
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
//!   // 5. Decrement and tail recurse
//!   i_minus_1 = i - 1
//!   Call(loop_step, [i_minus_1, ch, s])
//!
//! fn k_exit(i_exit):
//!   return i_exit
//! ```

use crate::mir::join_ir::lowering::canonical_names as cn;
use crate::mir::join_ir::lowering::join_value_space::JoinValueSpace;
use crate::mir::join_ir::{
    BinOpKind, CompareOp, ConstValue, JoinFuncId, JoinFunction, JoinInst, JoinModule, MirLikeInst,
};
use crate::runtime::get_global_ring0;

/// Lower the reverse scan_with_init route to JoinIR
///
/// # Phase 257 P0: Reverse scan (backward iteration)
///
/// This is a variant of scan_with_init_minimal that scans backward from i = s.length() - 1.
///
/// ## Key Differences from Forward Scan
///
/// - Exit condition: `i < 0` instead of `i >= len`
/// - Step: `i = i - 1` instead of `i = i + 1`
/// - Init: Caller must provide `i = s.length() - 1` before calling
///
/// ## Boundary Contract
///
/// This function returns a JoinModule with:
/// - **Input slots**: main() params for (i, ch, s)
/// - **Caller responsibility**: Ensure i is initialized to s.length() - 1
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
pub(crate) fn lower_scan_with_init_reverse(
    join_value_space: &mut JoinValueSpace,
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
    let i_main_param = join_value_space.alloc_param(); // loop index
    let ch_main_param = join_value_space.alloc_param(); // needle character (alphabetically first)
    let s_main_param = join_value_space.alloc_param(); // haystack string (alphabetically second)
    let loop_result = join_value_space.alloc_local(); // result from loop_step

    // loop_step params/locals
    let i_step_param = join_value_space.alloc_param(); // loop index
    let ch_step_param = join_value_space.alloc_param(); // needle (alphabetically first)
    let s_step_param = join_value_space.alloc_param(); // haystack (alphabetically second)
    let const_0 = join_value_space.alloc_local(); // 0 for exit condition
    let exit_cond = join_value_space.alloc_local(); // i < 0
    let const_minus_1 = join_value_space.alloc_local(); // -1 for not found
    let const_1 = join_value_space.alloc_local(); // 1 for i + 1
    let i_plus_1 = join_value_space.alloc_local(); // i + 1
    let cur = join_value_space.alloc_local(); // substring result
    let match_cond = join_value_space.alloc_local(); // cur == ch
    let i_minus_1 = join_value_space.alloc_local(); // i - 1

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
    let mut loop_step_func = JoinFunction::new(
        loop_step_id,
        cn::LOOP_STEP.to_string(),
        vec![i_step_param, ch_step_param, s_step_param],  // Phase 255 P0: [i, ch, s] alphabetical
    );

    // 1. const 0
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Const {
            dst: const_0,
            value: ConstValue::Integer(0),
        }));

    // 2. exit_cond = (i < 0)
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Compare {
            dst: exit_cond,
            op: CompareOp::Lt,
            lhs: i_step_param,
            rhs: const_0,
        }));

    // 3. const -1
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Const {
            dst: const_minus_1,
            value: ConstValue::Integer(-1),
        }));

    // 4. Jump(k_exit, [-1], cond=exit_cond) - not found case
    loop_step_func.body.push(JoinInst::Jump {
        cont: k_exit_id.as_cont(),
        args: vec![const_minus_1],
        cond: Some(exit_cond),
    });

    // 5. i_plus_1 = i + 1
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Const {
            dst: const_1,
            value: ConstValue::Integer(1),
        }));

    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BinOp {
            dst: i_plus_1,
            op: BinOpKind::Add,
            lhs: i_step_param,
            rhs: const_1,
        }));

    // 6. cur = s.substring(i, i_plus_1) - init-time BoxCall
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BoxCall {
            dst: Some(cur),
            box_name: "StringBox".to_string(),
            method: "substring".to_string(),
            args: vec![s_step_param, i_step_param, i_plus_1],
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

    // 9. i_minus_1 = i - 1
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BinOp {
            dst: i_minus_1,
            op: BinOpKind::Sub,
            lhs: i_step_param,
            rhs: const_1,
        }));

    // 10. Call(loop_step, [i_minus_1, ch, s]) - tail recursion
    loop_step_func.body.push(JoinInst::Call {
        func: loop_step_id,
        args: vec![i_minus_1, ch_step_param, s_step_param],  // Phase 255 P0: [i_minus_1, ch, s] alphabetical
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
            .debug("[joinir/scan_with_init] Generated JoinIR for reverse scan_with_init route");
        ring0
            .log
            .debug("[joinir/scan_with_init] Functions: main, loop_step, k_exit");
        ring0
            .log
            .debug("[joinir/scan_with_init] Direction: Reverse (i >= 0, i = i - 1)");
    }

    join_module
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lower_scan_with_init_reverse() {
        let mut join_value_space = JoinValueSpace::new();

        let join_module = lower_scan_with_init_reverse(&mut join_value_space);

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
    fn test_loop_step_has_reverse_step() {
        let mut join_value_space = JoinValueSpace::new();

        let join_module = lower_scan_with_init_reverse(&mut join_value_space);

        // loop_step 関数を取得
        let loop_step = join_module
            .functions
            .get(&JoinFuncId::new(1))
            .expect("loop_step function should exist");

        // BinOp(Sub) (i - 1) が含まれることを確認
        let has_decrement = loop_step.body.iter().any(|inst| {
            matches!(
                inst,
                JoinInst::Compute(MirLikeInst::BinOp { op: BinOpKind::Sub, .. })
            )
        });

        assert!(
            has_decrement,
            "loop_step should contain i - 1 decrement"
        );
    }
}
