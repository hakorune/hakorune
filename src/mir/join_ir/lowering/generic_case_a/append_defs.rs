//! Array Append Loop Lowering (Case A)
//!
//! Phase 192: Extracted from generic_case_a.rs monolith.
//!
//! ## Responsibility
//!
//! Lowers FuncScannerBox.append_defs/2 array concatenation loop to JoinIR.
//!
//! ## Shape Structure
//!
//! ```text
//! entry: append_defs_entry(dst, defs_box, n)
//!   i = 0
//!   call loop_step(dst, defs_box, n, i)
//!
//! loop_step(dst, defs_box, n, i):
//!   if i >= n { return }         // Exit condition
//!   item = defs_box.get(i)
//!   dst.push(item)               // Append item to dst
//!   continue loop_step(dst, defs_box, n, i+1)
//! ```
//!
//! ## ValueId Allocation
//!
//! - Entry range: 8000-8999 (`value_id_ranges::funcscanner_append_defs::entry`)
//! - Loop range: 9000-9999 (`value_id_ranges::funcscanner_append_defs::loop_step`)
//!
//! ## See Also
//!
//! - `value_id_ranges::funcscanner_append_defs` - ValueId allocation strategy
//! - `loop_scope_shape::CaseAContext` - Context extraction

use crate::mir::join_ir::lowering::loop_scope_shape::CaseAContext;
use crate::mir::join_ir::lowering::value_id_ranges;
use crate::mir::join_ir::{
    BinOpKind, CompareOp, ConstValue, JoinContId, JoinFuncId, JoinFunction, JoinInst, JoinModule,
    LoopExitShape, LoopHeaderShape, MirLikeInst,
};
use crate::mir::ValueId;
use crate::runtime::get_global_ring0;

use super::entry_builder::EntryFunctionBuilder;

/// Phase 30 F-3.0.4: LoopScopeShape を直接受け取る append_defs lowerer
///
/// 呼び出し元で LoopScopeShape を明示的に構築し、この関数に渡す。
/// CaseAContext::from_scope() 経由で ctx を作成。
pub(crate) fn lower_case_a_append_defs_with_scope(
    scope: crate::mir::join_ir::lowering::loop_scope_shape::LoopScopeShape,
) -> Option<JoinModule> {
    let ctx = CaseAContext::from_scope(scope, "append_defs", |offset| {
        value_id_ranges::funcscanner_append_defs::loop_step(offset)
    })?;
    lower_case_a_append_defs_core(&ctx)
}

/// append_defs JoinModule 構築のコア実装
///
/// CaseAContext から JoinModule を構築する共通ロジック。
/// `_for_append_defs_minimal` と `_with_scope` の両方から呼ばれる。
fn lower_case_a_append_defs_core(ctx: &CaseAContext) -> Option<JoinModule> {
    let dst_key = ctx.pinned_name_or_first(0)?;
    let defs_key = ctx
        .pinned_name_or_first(1)
        .unwrap_or_else(|| dst_key.clone());
    let n_key = ctx
        .pinned_name_or_first(2)
        .unwrap_or_else(|| defs_key.clone());
    let i_key = ctx.carrier_name_or_first(0)?;

    let dst_loop = ctx.get_loop_id(&dst_key)?;
    let defs_loop = ctx.get_loop_id(&defs_key)?;
    let n_loop = ctx.get_loop_id(&n_key)?;
    let i_loop = ctx.get_loop_id(&i_key)?;

    let mut join_module = JoinModule::new();

    // entry: append_defs_entry(dst, defs_box, n)
    let entry_id = JoinFuncId::new(0);
    let dst_param = value_id_ranges::funcscanner_append_defs::entry(0);
    let defs_box_param = value_id_ranges::funcscanner_append_defs::entry(1);
    let n_param = value_id_ranges::funcscanner_append_defs::entry(2);
    let mut entry_func = JoinFunction::new(
        entry_id,
        "append_defs_entry".to_string(),
        vec![dst_param, defs_box_param, n_param],
    );

    let i_init = value_id_ranges::funcscanner_append_defs::entry(10);
    entry_func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: i_init,
        value: ConstValue::Integer(0),
    }));
    // Phase 192: Use EntryFunctionBuilder for boilerplate initialization
    let mut entry_builder = EntryFunctionBuilder::new();
    entry_builder.add_var("s".to_string(), dst_param); // intake で param0 を "s" にするため
    entry_builder.add_var("dst".to_string(), dst_param);
    entry_builder.add_var("param1".to_string(), defs_box_param);
    entry_builder.add_var("defs_box".to_string(), defs_box_param);
    entry_builder.add_var("n".to_string(), n_param);
    entry_builder.add_var("i".to_string(), i_init);
    let entry_name_to_id = entry_builder.get_map().clone();

    let loop_call_args: Vec<ValueId> = ctx
        .ordered_pinned
        .iter()
        .chain(ctx.ordered_carriers.iter())
        .map(|name| entry_name_to_id.get(name).copied())
        .collect::<Option<_>>()?;

    let loop_step_id = JoinFuncId::new(1);
    entry_func.body.push(JoinInst::Call {
        func: loop_step_id,
        args: loop_call_args,
        k_next: None,
        dst: None,
    });

    join_module.entry = Some(entry_id);
    join_module.add_function(entry_func);

    // loop_step(dst, defs_box, n, i)
    let header_shape = LoopHeaderShape::new_manual(ctx.pinned_ids.clone(), ctx.carrier_ids.clone());
    let loop_params = header_shape.to_loop_step_params();
    let mut loop_step_func =
        JoinFunction::new(loop_step_id, "loop_step".to_string(), loop_params.clone());

    let cmp_result = value_id_ranges::funcscanner_append_defs::loop_step(10);
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Compare {
            dst: cmp_result,
            op: CompareOp::Ge,
            lhs: i_loop,
            rhs: n_loop,
        }));

    let _exit_shape = LoopExitShape::new_manual(ctx.exit_args.clone());

    loop_step_func.body.push(JoinInst::Jump {
        cont: JoinContId::new(0),
        args: ctx.exit_args.clone(),
        cond: Some(cmp_result),
    });

    let item_value = value_id_ranges::funcscanner_append_defs::loop_step(11);
    let next_i = value_id_ranges::funcscanner_append_defs::loop_step(12);
    let const_1 = value_id_ranges::funcscanner_append_defs::loop_step(13);

    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BoxCall {
            dst: Some(item_value),
            box_name: "ArrayBox".to_string(),
            method: "get".to_string(),
            args: vec![defs_loop, i_loop],
        }));

    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BoxCall {
            dst: None,
            box_name: "ArrayBox".to_string(),
            method: "push".to_string(),
            args: vec![dst_loop, item_value],
        }));

    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Const {
            dst: const_1,
            value: ConstValue::Integer(1),
        }));

    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BinOp {
            dst: next_i,
            op: BinOpKind::Add,
            lhs: i_loop,
            rhs: const_1,
        }));

    loop_step_func.body.push(JoinInst::Call {
        func: loop_step_id,
        args: vec![dst_loop, defs_loop, n_loop, next_i],
        k_next: None,
        dst: None,
    });

    join_module.add_function(loop_step_func);

    if crate::config::env::joinir_dev::debug_enabled() {
        get_global_ring0().log.debug(&format!(
            "[joinir/generic_case_a/append_defs] ✅ constructed JoinIR (functions={}, value_range={}..{})",
            join_module.functions.len(),
            value_id_ranges::base::FUNCSCANNER_APPEND_DEFS,
            value_id_ranges::base::FUNCSCANNER_APPEND_DEFS + 1999
        ));
    }

    Some(join_module)
}
