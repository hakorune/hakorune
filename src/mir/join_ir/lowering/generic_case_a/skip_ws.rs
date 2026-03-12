//! Skip Whitespace Loop Lowering (Case A)
//!
//! Phase 192: Extracted from generic_case_a.rs monolith.
//!
//! ## Responsibility
//!
//! Lowers Main.skip/1 whitespace skipping loop to JoinIR.
//!
//! ## Shape Structure
//!
//! ```text
//! entry: skip(s)
//!   i = 0
//!   n = s.length()
//!   call loop_step(s, i, n)
//!
//! loop_step(s, i, n):
//!   if i >= n { return i }         // Exit condition
//!   ch = s.substring(i, i+1)
//!   if ch != " " { return i }      // Non-space found
//!   continue loop_step(s, i+1, n)  // Skip space
//! ```
//!
//! ## ValueId Allocation
//!
//! - Entry range: 3000-3999 (`value_id_ranges::skip_ws::entry`)
//! - Loop range: 4000-5999 (`value_id_ranges::skip_ws::loop_step`)
//!
//! ## See Also
//!
//! - `value_id_ranges::skip_ws` - ValueId allocation strategy
//! - `loop_scope_shape::CaseAContext` - Context extraction

use crate::mir::join_ir::lowering::loop_scope_shape::CaseAContext;
use crate::mir::join_ir::lowering::value_id_ranges;
use crate::mir::join_ir::lowering::value_id_ranges::skip_ws as vid;
use crate::mir::join_ir::{
    BinOpKind, CompareOp, ConstValue, JoinContId, JoinFuncId, JoinFunction, JoinInst, JoinModule,
    LoopExitShape, LoopHeaderShape, MirLikeInst,
};
use crate::mir::ValueId;
use crate::runtime::get_global_ring0;

use super::entry_builder::EntryFunctionBuilder;

/// Phase 30: LoopScopeShape を直接受け取る skip_ws lowerer
///
/// 呼び出し元で LoopScopeShape を明示的に構築し、この関数に渡す。
/// CaseAContext::from_scope() 経由で ctx を作成。
///
/// # Arguments
///
/// - `scope`: 事前に構築済みの LoopScopeShape
///
/// # Returns
///
/// Some(JoinModule) if successful, None if context building fails.
pub(crate) fn lower_case_a_skip_ws_with_scope(
    scope: crate::mir::join_ir::lowering::loop_scope_shape::LoopScopeShape,
) -> Option<JoinModule> {
    // CaseAContext::from_scope() で ctx を構築
    let ctx = CaseAContext::from_scope(scope, "skip_ws", |offset| vid::loop_step(offset))?;

    // JoinModule 構築（既存 _for_minimal_skip_ws と同一ロジック）
    lower_case_a_skip_ws_core(&ctx)
}

/// skip_ws JoinModule 構築のコア実装
///
/// CaseAContext から JoinModule を構築する共通ロジック。
/// `_for_minimal_skip_ws` と `_with_scope` の両方から呼ばれる。
fn lower_case_a_skip_ws_core(ctx: &CaseAContext) -> Option<JoinModule> {
    let string_key = ctx.pinned_name_or_first(0)?;
    let len_key = ctx
        .pinned_name_or_first(1)
        .unwrap_or_else(|| string_key.clone());
    let index_key = ctx.carrier_name_or_first(0)?;

    let s_loop = ctx.get_loop_id(&string_key)?;
    let i_loop = ctx.get_loop_id(&index_key)?;
    let n_loop = ctx.get_loop_id(&len_key)?;

    let mut join_module = JoinModule::new();

    // entry: skip(s)
    let skip_id = JoinFuncId::new(0);
    let s_param = vid::entry(0); // 3000
    let mut skip_func = JoinFunction::new(skip_id, "skip".to_string(), vec![s_param]);

    let i_init = vid::entry(1); // 3001
    let n_val = vid::entry(2); // 3002

    // Phase 192: Use EntryFunctionBuilder for boilerplate initialization
    let mut entry_builder = EntryFunctionBuilder::new();
    entry_builder.add_var(string_key.clone(), s_param);
    entry_builder.add_var(index_key.clone(), i_init);
    entry_builder.add_var(len_key.clone(), n_val);
    let entry_name_to_id = entry_builder.get_map().clone();

    skip_func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: i_init,
        value: ConstValue::Integer(0),
    }));

    skip_func.body.push(JoinInst::Compute(MirLikeInst::BoxCall {
        dst: Some(n_val),
        box_name: "StringBox".to_string(),
        method: "length".to_string(),
        args: vec![s_param],
    }));

    let loop_step_id = JoinFuncId::new(1);
    let loop_call_args: Vec<ValueId> = ctx
        .ordered_pinned
        .iter()
        .chain(ctx.ordered_carriers.iter())
        .map(|name| entry_name_to_id.get(name).copied())
        .collect::<Option<_>>()?;

    skip_func.body.push(JoinInst::Call {
        func: loop_step_id,
        args: loop_call_args,
        k_next: None,
        dst: None,
    });

    join_module.entry = Some(skip_id);
    join_module.add_function(skip_func);

    // loop_step(s, i, n)
    let header_shape = LoopHeaderShape::new_manual(ctx.pinned_ids.clone(), ctx.carrier_ids.clone());
    let loop_params = header_shape.to_loop_step_params();
    let mut loop_step_func =
        JoinFunction::new(loop_step_id, "loop_step".to_string(), loop_params.clone());

    let cmp1_result = vid::loop_step(3); // 4003
    let ch = vid::loop_step(4); // 4004
    let cmp2_result = vid::loop_step(5); // 4005
    let i_plus_1 = vid::loop_step(6); // 4006
    let const_1 = vid::loop_step(7); // 4007
    let const_space = vid::loop_step(10); // 4010
    let bool_false = vid::loop_step(11); // 4011
    let cmp2_is_false = vid::loop_step(12); // 4012

    // cmp1_result = (i >= n)
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Compare {
            dst: cmp1_result,
            op: CompareOp::Ge,
            lhs: i_loop,
            rhs: n_loop,
        }));

    let _exit_shape = if ctx.exit_args.is_empty() {
        LoopExitShape::new_manual(vec![i_loop])
    } else {
        LoopExitShape::new_manual(ctx.exit_args.clone())
    }; // exit_args = [i] が期待値

    // if i >= n { return i }
    loop_step_func.body.push(JoinInst::Jump {
        cont: JoinContId::new(0),
        args: vec![i_loop],
        cond: Some(cmp1_result),
    });

    // const 1
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Const {
            dst: const_1,
            value: ConstValue::Integer(1),
        }));

    // i_plus_1 = i + 1
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BinOp {
            dst: i_plus_1,
            op: BinOpKind::Add,
            lhs: i_loop,
            rhs: const_1,
        }));

    // ch = s.substring(i, i + 1)
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BoxCall {
            dst: Some(ch),
            box_name: "StringBox".to_string(),
            method: "substring".to_string(),
            args: vec![s_loop, i_loop, i_plus_1],
        }));

    // const " "
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Const {
            dst: const_space,
            value: ConstValue::String(" ".to_string()),
        }));

    // cmp2_result = (ch == " ")
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Compare {
            dst: cmp2_result,
            op: CompareOp::Eq,
            lhs: ch,
            rhs: const_space,
        }));

    // bool false
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Const {
            dst: bool_false,
            value: ConstValue::Bool(false),
        }));

    // cmp2_is_false = (cmp2_result == false)
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Compare {
            dst: cmp2_is_false,
            op: CompareOp::Eq,
            lhs: cmp2_result,
            rhs: bool_false,
        }));

    // if ch != " " { return i }
    loop_step_func.body.push(JoinInst::Jump {
        cont: JoinContId::new(1),
        args: vec![i_loop],
        cond: Some(cmp2_is_false),
    });

    // continue: loop_step(s, i+1, n)
    loop_step_func.body.push(JoinInst::Call {
        func: loop_step_id,
        args: vec![s_loop, i_plus_1, n_loop],
        k_next: None,
        dst: None,
    });

    join_module.add_function(loop_step_func);

    if crate::config::env::joinir_dev::debug_enabled() {
        get_global_ring0().log.debug(&format!(
            "[joinir/generic_case_a/skip_ws] ✅ constructed JoinIR (functions={}, value_range={}..{})",
            join_module.functions.len(),
            value_id_ranges::base::SKIP_WS,
            value_id_ranges::base::SKIP_WS + 1999
        ));
    }

    Some(join_module)
}
