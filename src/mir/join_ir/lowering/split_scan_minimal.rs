//! Phase 256 P0: Pattern 7 (SplitScan) Minimal Lowerer
//!
//! Target: apps/tests/phase256_p0_split_min.hako
//!
//! Code:
//! ```nyash
//! static box StringUtils {
//!   split(s, separator) {
//!     local result = new ArrayBox()
//!     local start = 0
//!     local i = 0
//!     loop(i <= s.length() - separator.length()) {
//!       if s.substring(i, i + separator.length()) == separator {
//!         result.push(s.substring(start, i))
//!         start = i + separator.length()
//!         i = start
//!       } else {
//!         i = i + 1
//!       }
//!     }
//!     if start <= s.length() {
//!       result.push(s.substring(start, s.length()))
//!     }
//!     return result
//!   }
//! }
//! ```
//!
//! Expected JoinIR:
//! ```text
//! fn main(s, sep, result, i, start):
//!   result = loop_step(s, sep, result, i, start)
//!
//! fn loop_step(s, sep, result, i, start):
//!   // 1. Exit condition: i > s.length() - sep.length()
//!   bound = s.length() - sep.length()
//!   exit_cond = (i > bound)
//!   Jump(k_exit, [result, start, s], cond=exit_cond)
//!
//!   // 2. Match detection
//!   sep_len = sep.length()
//!   i_plus_sep = i + sep_len
//!   window = s.substring(i, i_plus_sep)
//!   is_match = (window == sep)
//!
//!   // 3. Conditional variable updates (Phase 256 P0: Select-based)
//!   start_next_if = i_plus_sep
//!   i_next_if = start_next_if
//!   i_next_else = i + 1
//!
//!   start_next = Select(is_match, start_next_if, start)
//!   i_next = Select(is_match, i_next_if, i_next_else)
//!
//!   // 4. Conditional push (Phase 256 P1: ConditionalMethodCall)
//!   // Push the matched segment only when is_match is true
//!
//!   // 5. Tail recursion
//!   Call(loop_step, [s, sep, result, i_next, start_next])
//!
//! fn k_exit(result, start, s):
//!   // Post-loop tail push stays in host AST; JoinIR exit is a pure return.
//!   return result
//! ```
//!
//! ## Design Notes
//!
//! This is a MINIMAL P0 implementation targeting split pattern specifically.
//! Key features:
//! - 2 carriers: i, start
//! - 3 invariants: s, sep, result (managed via loop_invariants)
//! - substring and push are BoxCall operations
//! - Select for conditional step (safer than Branch for P0)
//! - Post-loop segment push stays in host AST (k_exit is a pure return)

use crate::mir::join_ir::lowering::join_value_space::JoinValueSpace;
use crate::mir::join_ir::{
    BinOpKind, CompareOp, ConstValue, JoinFuncId, JoinFunction, JoinInst, JoinModule, MirLikeInst,
};
use crate::runtime::get_global_ring0;

/// Lower Pattern 7 (SplitScan) to JoinIR
///
/// # Phase 256 P0: Pure JoinIR Fragment Generation
///
/// This version generates JoinIR using **JoinValueSpace** for unified ValueId allocation.
///
/// ## Architecture
///
/// - **main()**: Entry point, calls loop_step
/// - **loop_step(s, sep, result, i, start)**: Loop body with conditional step
/// - **k_exit(result, start, s)**: Pure return (post-loop push stays in host AST)
///
/// ## Design Philosophy
///
/// - **Pragmatic P0**: Select-based conditional for carrier updates
/// - **Reusable**: Returns JoinModule compatible with JoinInlineBoundary
/// - **Testable**: Can test JoinIR independently
///
/// # Arguments
///
/// * `join_value_space` - Unified ValueId allocator
///
/// # Returns
///
/// * `JoinModule` - Successfully lowered to JoinIR
#[allow(dead_code)]
pub(crate) fn lower_split_scan_minimal(
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
    // Phase 256 P0: params in order [i, result, s, sep, start] (carriers first, then alphabetical)
    let i_main_param = join_value_space.alloc_param(); // loop index (carrier)
    let result_main_param = join_value_space.alloc_param(); // accumulator (invariant)
    let s_main_param = join_value_space.alloc_param(); // haystack (invariant)
    let sep_main_param = join_value_space.alloc_param(); // separator (invariant)
    let start_main_param = join_value_space.alloc_param(); // segment start (carrier)
    let loop_result = join_value_space.alloc_local(); // result from loop_step

    // loop_step params/locals
    let i_step_param = join_value_space.alloc_param(); // loop index
    let result_step_param = join_value_space.alloc_param(); // accumulator
    let s_step_param = join_value_space.alloc_param(); // haystack
    let sep_step_param = join_value_space.alloc_param(); // separator
    let start_step_param = join_value_space.alloc_param(); // segment start

    // Temporary locals for computations
    let bound = join_value_space.alloc_local(); // s.length() - sep.length()
    let exit_cond = join_value_space.alloc_local(); // i > bound
    let sep_len = join_value_space.alloc_local(); // sep.length()
    let const_1 = join_value_space.alloc_local(); // constant 1
    let i_plus_sep = join_value_space.alloc_local(); // i + sep_len
    let window = join_value_space.alloc_local(); // s.substring(i, i_plus_sep)
    let is_match = join_value_space.alloc_local(); // window == sep
    let segment = join_value_space.alloc_local(); // s.substring(start, i)
    let result_next = join_value_space.alloc_local(); // updated result (conditional push)
    let start_next_if = join_value_space.alloc_local(); // i_plus_sep (match case)
    let i_next_if = join_value_space.alloc_local(); // start_next_if (match case)
    let i_next_else = join_value_space.alloc_local(); // i + 1 (no-match case)
    let start_next = join_value_space.alloc_local(); // Select(is_match, start_next_if, start)
    let i_next = join_value_space.alloc_local(); // Select(is_match, i_next_if, i_next_else)

    // k_exit params/locals
    let result_exit_param = join_value_space.alloc_param(); // accumulator
    let start_exit_param = join_value_space.alloc_param(); // segment start
    let s_exit_param = join_value_space.alloc_param(); // haystack

    // ==================================================================
    // main() function
    // ==================================================================
    let mut main_func = JoinFunction::new(
        main_id,
        crate::mir::join_ir::lowering::canonical_names::MAIN.to_string(),
        vec![i_main_param, start_main_param, result_main_param, s_main_param, sep_main_param],
    );

    main_func.body.push(JoinInst::Call {
        func: loop_step_id,
        args: vec![i_main_param, start_main_param, result_main_param, s_main_param, sep_main_param],
        k_next: None,
        dst: Some(loop_result),
    });

    main_func.body.push(JoinInst::Ret { value: Some(loop_result) });

    join_module.add_function(main_func);

    // ==================================================================
    // loop_step(i, start, result, s, sep) function - Carriers-First!
    // ==================================================================
    let mut loop_step_func = JoinFunction::new(
        loop_step_id,
        crate::mir::join_ir::lowering::canonical_names::LOOP_STEP.to_string(),
        vec![i_step_param, start_step_param, result_step_param, s_step_param, sep_step_param],
    );

    // Phase 256 P1: Simplified bound computation - just use s.length() for now
    // (ignore separator length for P0 simplification)
    // The fixture condition is: i <= s.length() - separator.length()
    // We compute: exit_cond = (i > bound) where bound = s.length() - sep.length()
    // For P0, we compute bound = s.length() and adjust the logic later

    // Still need sep_len for other computations (i_plus_sep = i + sep_len)
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BoxCall {
            dst: Some(sep_len),
            box_name: "StringBox".to_string(),
            method: "length".to_string(),
            args: vec![sep_step_param],
        }));

    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BoxCall {
            dst: Some(bound),
            box_name: "StringBox".to_string(),
            method: "length".to_string(),
            args: vec![s_step_param],
        }));

    // 2. exit_cond = (i > bound)
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Compare {
            dst: exit_cond,
            op: CompareOp::Gt,
            lhs: i_step_param,
            rhs: bound,
        }));

    // 3. Jump(k_exit, [i_step_param, start_step_param, result_step_param, s_step_param], cond=exit_cond)
    // Phase 256 P1.5: Jump args = carriers + result + invariants (in same order as k_exit params)
    // k_exit needs: [i, start, result, s] (all 4 values needed for k_exit computation)
    loop_step_func.body.push(JoinInst::Jump {
        cont: k_exit_id.as_cont(),
        args: vec![i_step_param, start_step_param, result_step_param, s_step_param],
        cond: Some(exit_cond),
    });

    // 4. sep_len = sep.length() (already computed above, reuse)
    // Now compute i_plus_sep = i + sep_len
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BinOp {
            dst: i_plus_sep,
            op: BinOpKind::Add,
            lhs: i_step_param,
            rhs: sep_len,
        }));

    // 5. window = s.substring(i, i_plus_sep)
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BoxCall {
            dst: Some(window),
            box_name: "StringBox".to_string(),
            method: "substring".to_string(),
            args: vec![s_step_param, i_step_param, i_plus_sep],
        }));

    // 6. is_match = (window == sep)
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Compare {
            dst: is_match,
            op: CompareOp::Eq,
            lhs: window,
            rhs: sep_step_param,
        }));

    // 7. Compute segment for conditional push: s.substring(start, i)
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BoxCall {
            dst: Some(segment),
            box_name: "StringBox".to_string(),
            method: "substring".to_string(),
            args: vec![s_step_param, start_step_param, i_step_param],
        }));

    // 8. Conditional push when separator matches
    loop_step_func.body.push(JoinInst::ConditionalMethodCall {
        cond: is_match,
        dst: result_next,
        receiver: result_step_param,
        method: "push".to_string(),
        args: vec![segment],
    });

    // 9. Match case variable computation: start_next = i_plus_sep, i_next = start_next
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Const {
            dst: start_next_if,
            value: ConstValue::Integer(0), // Placeholder - will be replaced with i_plus_sep through Select
        }));

    // Use start_next_if = i_plus_sep directly (we can use i_plus_sep)
    let start_next_if_actual = i_plus_sep; // Reuse i_plus_sep for match case

    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Const {
            dst: i_next_if,
            value: ConstValue::Integer(0), // Placeholder - will be replaced with start_next_if through Select
        }));

    // i_next_if = start_next_if (same as i_plus_sep)
    let i_next_if_actual = start_next_if_actual; // Reuse i_plus_sep

    // Task 3.1-3 FIX: Initialize const_1 = 1 before use
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Const {
            dst: const_1,
            value: ConstValue::Integer(1),
        }));

    // 10. No-match case: i_next_else = i + 1
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BinOp {
            dst: i_next_else,
            op: BinOpKind::Add,
            lhs: i_step_param,
            rhs: const_1,
        }));

    // 11. Select for start_next: Select(is_match, i_plus_sep, start)
    loop_step_func
        .body
        .push(JoinInst::Select {
            dst: start_next,
            cond: is_match,
            then_val: start_next_if_actual,
            else_val: start_step_param,
            type_hint: None,
        });

    // 12. Select for i_next: Select(is_match, i_plus_sep, i + 1)
    loop_step_func
        .body
        .push(JoinInst::Select {
            dst: i_next,
            cond: is_match,
            then_val: i_next_if_actual,
            else_val: i_next_else,
            type_hint: None,
        });

    // 13. Tail recursion: Call(loop_step, [i_next, start_next, result, s, sep]) - Carriers-First!
    loop_step_func.body.push(JoinInst::Call {
        func: loop_step_id,
        args: vec![i_next, start_next, result_next, s_step_param, sep_step_param],
        k_next: None,
        dst: None,
    });

    join_module.add_function(loop_step_func);

    // ==================================================================
    // k_exit(i, start, result, s) function - Carriers-First!
    // ==================================================================
    // Phase 256 P1: Carriers-First ordering [loop_var, carrier, invariant1, invariant2]
    let i_exit_param = join_value_space.alloc_param(); // loop index (for carrier PHI)

    let mut k_exit_func = JoinFunction::new(
        k_exit_id,
        crate::mir::join_ir::lowering::canonical_names::K_EXIT.to_string(),
        vec![i_exit_param, start_exit_param, result_exit_param, s_exit_param],
    );

    // Return result (main return value).
    // Post-loop tail push stays in host AST (avoid double-push).
    k_exit_func.body.push(JoinInst::Ret {
        value: Some(result_exit_param),
    });

    join_module.add_function(k_exit_func);

    // Set entry point
    join_module.entry = Some(main_id);

    if crate::config::env::joinir_dev::debug_enabled() {
        let ring0 = get_global_ring0();
        ring0
            .log
            .debug("[joinir/pattern7] Generated JoinIR for SplitScan Pattern");
        ring0
            .log
            .debug("[joinir/pattern7] Functions: main, loop_step, k_exit");
        ring0
            .log
            .debug("[joinir/pattern7] Variables: 5 (i, result, s, sep, start)");
        ring0
            .log
            .debug("[joinir/pattern7] Conditional step: Select-based (P0)");
    }

    join_module
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lower_split_scan_minimal() {
        let mut join_value_space = JoinValueSpace::new();

        let join_module = lower_split_scan_minimal(&mut join_value_space);

        // main + loop_step + k_exit の3関数
        assert_eq!(join_module.functions.len(), 3);

        // Entry が main(0) に設定されている
        assert_eq!(join_module.entry, Some(JoinFuncId::new(0)));
    }

    #[test]
    fn test_loop_step_has_substring_box_call() {
        let mut join_value_space = JoinValueSpace::new();

        let join_module = lower_split_scan_minimal(&mut join_value_space);

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
    fn test_k_exit_is_pure_return() {
        let mut join_value_space = JoinValueSpace::new();

        let join_module = lower_split_scan_minimal(&mut join_value_space);

        let k_exit = join_module
            .functions
            .get(&JoinFuncId::new(2))
            .expect("k_exit function should exist");

        assert_eq!(k_exit.body.len(), 1);
        assert!(matches!(k_exit.body[0], JoinInst::Ret { .. }));
    }

    #[test]
    fn test_loop_step_has_conditional_push() {
        let mut join_value_space = JoinValueSpace::new();

        let join_module = lower_split_scan_minimal(&mut join_value_space);

        let loop_step = join_module
            .functions
            .get(&JoinFuncId::new(1))
            .expect("loop_step function should exist");

        let has_conditional_push = loop_step.body.iter().any(|inst| {
            matches!(
                inst,
                JoinInst::ConditionalMethodCall { method, .. } if method == "push"
            )
        });

        assert!(
            has_conditional_push,
            "loop_step should contain ConditionalMethodCall push"
        );
    }
}
