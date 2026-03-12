//! Phase 259 P0: bool_predicate_scan JoinIR Lowerer
//!
//! Target: apps/tests/phase259_p0_is_integer_min.hako
//!
//! Expected JoinIR:
//! ```text
//! fn main(i, me, s):
//!   result = loop_step(i, me, s)
//!   ret result
//!
//! fn loop_step(i, me, s):
//!   // 1. Check exit condition: i >= s.length()
//!   len = StringBox.length(s)
//!   exit_cond = (i >= len)
//!   const_true = true
//!   Jump(k_exit, [const_true], cond=exit_cond)  // All passed
//!
//!   // 2. Extract character
//!   const_1 = 1
//!   i_plus_1 = i + 1
//!   ch = StringBox.substring(s, i, i_plus_1)
//!
//!   // 3. Call predicate (Me method → MethodCall with me receiver)
//!   ok = MethodCall(me, predicate_method, [ch])
//!
//!   // 4. Check predicate result (inverted - if !ok, return false)
//!   const_false = false
//!   not_ok = !ok
//!   Jump(k_exit, [const_false], cond=not_ok)  // Predicate failed
//!
//!   // 5. Tail recursion
//!   tail_result = Call(loop_step, [i_plus_1, me, s])
//!   Ret(tail_result)
//!
//! fn k_exit(ret_bool):
//!   ret ret_bool
//! ```

use crate::mir::join_ir::lowering::canonical_names as cn;
use crate::mir::join_ir::lowering::join_value_space::JoinValueSpace;
use crate::mir::join_ir::{
    CompareOp, ConstValue, JoinFuncId, JoinFunction, JoinInst, JoinModule, MirLikeInst, MirType,
    UnaryOp,
};

/// Lower the bool_predicate_scan route to JoinIR
///
/// # Arguments
///
/// * `join_value_space` - Unified ValueId allocator
/// * `predicate_receiver` - Receiver name (e.g., "this")
/// * `predicate_method` - Method name (e.g., "is_digit")
///
/// # Returns
///
/// * `JoinModule` - Successfully lowered to JoinIR
#[allow(dead_code)]
pub(crate) fn lower_scan_bool_predicate_minimal(
    join_value_space: &mut JoinValueSpace,
    predicate_receiver: &str,
    predicate_method: &str,
) -> JoinModule {
    let mut join_module = JoinModule::new();

    // Function IDs
    let main_id = JoinFuncId::new(0);
    let loop_step_id = JoinFuncId::new(1);
    let k_exit_id = JoinFuncId::new(2);

    // main() params/locals
    let i_main_param = join_value_space.alloc_param();
    let me_main_param = join_value_space.alloc_param(); // Phase 259 P0: Me receiver param
    let s_main_param = join_value_space.alloc_param();
    let loop_result = join_value_space.alloc_local();

    // loop_step params/locals
    let i_step_param = join_value_space.alloc_param();
    let me_step_param = join_value_space.alloc_param(); // Phase 259 P0: Me receiver param
    let s_step_param = join_value_space.alloc_param();
    let len = join_value_space.alloc_local();
    let exit_cond = join_value_space.alloc_local();
    let const_true = join_value_space.alloc_local();
    let const_1 = join_value_space.alloc_local();
    let i_plus_1 = join_value_space.alloc_local();
    let ch = join_value_space.alloc_local();
    let ok = join_value_space.alloc_local();
    let const_false = join_value_space.alloc_local();
    let not_ok = join_value_space.alloc_local();

    // k_exit params
    let ret_bool_param = join_value_space.alloc_param();

    // main() function
    let mut main_func = JoinFunction::new(
        main_id,
        "main".to_string(),
        vec![i_main_param, me_main_param, s_main_param], // Phase 259 P0: +me_param
    );

    main_func.body.push(JoinInst::Call {
        func: loop_step_id,
        args: vec![i_main_param, me_main_param, s_main_param], // Phase 259 P0: +me_param
        k_next: None,
        dst: Some(loop_result),
    });

    main_func.body.push(JoinInst::Ret {
        value: Some(loop_result),
    });

    join_module.add_function(main_func);

    // loop_step(i, me, s) function
    let mut loop_step_func = JoinFunction::new(
        loop_step_id,
        cn::LOOP_STEP.to_string(),
        vec![i_step_param, me_step_param, s_step_param], // Phase 259 P0: +me_param
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

    // 2. exit_cond = (i >= len)
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Compare {
            dst: exit_cond,
            op: CompareOp::Ge,
            lhs: i_step_param,
            rhs: len,
        }));

    // 3. const_true = true
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Const {
            dst: const_true,
            value: ConstValue::Bool(true),
        }));

    // 4. Jump(k_exit, [true], cond=exit_cond) - all passed
    loop_step_func.body.push(JoinInst::Jump {
        cont: k_exit_id.as_cont(),
        args: vec![const_true],
        cond: Some(exit_cond),
    });

    // 5. const_1 = 1
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Const {
            dst: const_1,
            value: ConstValue::Integer(1),
        }));

    // 6. i_plus_1 = i + 1
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BinOp {
            dst: i_plus_1,
            op: crate::mir::join_ir::BinOpKind::Add,
            lhs: i_step_param,
            rhs: const_1,
        }));

    // 7. ch = s.substring(i, i_plus_1)
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BoxCall {
            dst: Some(ch),
            box_name: "StringBox".to_string(),
            method: "substring".to_string(),
            args: vec![s_step_param, i_step_param, i_plus_1],
        }));

    // 8. ok = me.predicate_method(ch)
    // P0: predicate_receiver is "this" → use MethodCall with me_step_param
    // Phase 259 P0: Me receiver passed as param (by-name 禁止)
    loop_step_func.body.push(JoinInst::MethodCall {
        dst: ok,
        receiver: me_step_param, // Me receiver as param
        method: predicate_method.to_string(),
        args: vec![ch],
        type_hint: Some(MirType::Bool),
    });

    // 9. const_false = false
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Const {
            dst: const_false,
            value: ConstValue::Bool(false),
        }));

    // 10. not_ok = !ok (invert predicate for jump condition)
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::UnaryOp {
            dst: not_ok,
            op: UnaryOp::Not,
            operand: ok,
        }));

    // 11. Jump(k_exit, [false], cond=not_ok) - predicate failed
    loop_step_func.body.push(JoinInst::Jump {
        cont: k_exit_id.as_cont(),
        args: vec![const_false],
        cond: Some(not_ok),
    });

    // 12. Call(loop_step, [i_plus_1, me, s]) - tail recursion
    // Phase 259 P0: dst: None for tail-call (bridge handles optimization)
    loop_step_func.body.push(JoinInst::Call {
        func: loop_step_id,
        args: vec![i_plus_1, me_step_param, s_step_param], // Phase 259 P0: +me_param
        k_next: None,
        dst: None, // Tail-call: bridge が適切に処理
    });

    join_module.add_function(loop_step_func);

    // k_exit(ret_bool) function
    let mut k_exit_func =
        JoinFunction::new(k_exit_id, cn::K_EXIT.to_string(), vec![ret_bool_param]);

    k_exit_func.body.push(JoinInst::Ret {
        value: Some(ret_bool_param),
    });

    join_module.add_function(k_exit_func);

    // Set entry point
    join_module.entry = Some(main_id);

    if crate::config::env::joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0
            .log
            .debug("[joinir/bool_predicate_scan] Generated JoinIR for bool_predicate_scan route");
        ring0
            .log
            .debug("[joinir/bool_predicate_scan] Functions: main, loop_step, k_exit");
        ring0.log.debug(&format!(
            "[joinir/bool_predicate_scan] Predicate: {}.{}",
            predicate_receiver, predicate_method
        ));
    }

    join_module
}
