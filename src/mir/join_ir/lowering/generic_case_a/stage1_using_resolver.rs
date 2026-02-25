//! Stage-1 Using Resolver Loop Lowering (Case A)
//!
//! Phase 192: Extracted from generic_case_a.rs monolith.
//!
//! ## Responsibility
//!
//! Lowers Stage1UsingResolverBox.resolve_for_source/5 using namespace resolution loop to JoinIR.
//!
//! ## Pattern Structure
//!
//! ```text
//! entry: resolve_entries(entries, n, modules, seen, prefix_init)
//!   i = 0
//!   call loop_step(entries, n, modules, seen, prefix_init, i)
//!
//! loop_step(entries, n, modules, seen, prefix, i):
//!   if i >= n { return prefix }  // Exit condition
//!   entry = entries.get(i)
//!   new_prefix = prefix | entry  // OR operation (accumulate)
//!   continue loop_step(entries, n, modules, seen, new_prefix, i+1)
//! ```
//!
//! ## ValueId Allocation
//!
//! - Entry range: 10000-10999 (`value_id_ranges::stage1_using_resolver::entry`)
//! - Loop range: 11000-11999 (`value_id_ranges::stage1_using_resolver::loop_step`)
//!
//! ## See Also
//!
//! - `value_id_ranges::stage1_using_resolver` - ValueId allocation strategy
//! - `loop_scope_shape::CaseAContext` - Context extraction

use crate::mir::join_ir::lowering::loop_scope_shape::CaseAContext;
use crate::mir::join_ir::lowering::value_id_ranges;
use crate::mir::join_ir::lowering::value_id_ranges::stage1_using_resolver as stage1_vid;
use crate::mir::join_ir::{
    BinOpKind, CompareOp, ConstValue, JoinContId, JoinFuncId, JoinFunction, JoinInst, JoinModule,
    LoopExitShape, LoopHeaderShape, MirLikeInst,
};
use crate::mir::ValueId;
use crate::runtime::get_global_ring0;

use super::entry_builder::EntryFunctionBuilder;

/// Phase 30 F-3.0.5: LoopScopeShape を直接受け取る Stage-1 UsingResolver lowerer
///
/// 呼び出し元で LoopScopeShape を明示的に構築し、この関数に渡す。
/// CaseAContext::from_scope() 経由で ctx を作成。
pub(crate) fn lower_case_a_stage1_usingresolver_with_scope(
    scope: crate::mir::join_ir::lowering::loop_scope_shape::LoopScopeShape,
) -> Option<JoinModule> {
    let ctx = CaseAContext::from_scope(scope, "stage1", |offset| stage1_vid::loop_step(offset))?;
    lower_case_a_stage1_usingresolver_core(&ctx)
}

/// Stage-1 UsingResolver JoinModule 構築のコア実装
///
/// CaseAContext から JoinModule を構築する共通ロジック。
/// `_for_stage1_usingresolver_minimal` と `_with_scope` の両方から呼ばれる。
fn lower_case_a_stage1_usingresolver_core(ctx: &CaseAContext) -> Option<JoinModule> {
    let entries_key = ctx.pinned_name_or_first(0)?;
    let n_key = ctx
        .pinned_name_or_first(1)
        .unwrap_or_else(|| entries_key.clone());
    let modules_key = ctx
        .pinned_name_or_first(2)
        .unwrap_or_else(|| entries_key.clone());
    let seen_key = ctx
        .pinned_name_or_first(3)
        .unwrap_or_else(|| entries_key.clone());
    let prefix_key = ctx.carrier_name_or_first(0)?;
    let i_key = ctx
        .carrier_name_or_first(1)
        .unwrap_or_else(|| prefix_key.clone());

    let entries_loop = ctx.get_loop_id(&entries_key)?;
    let n_loop = ctx.get_loop_id(&n_key)?;
    let modules_loop = ctx.get_loop_id(&modules_key)?;
    let seen_loop = ctx.get_loop_id(&seen_key)?;
    let prefix_loop = ctx.get_loop_id(&prefix_key)?;
    let i_loop = ctx.get_loop_id(&i_key)?;

    let mut join_module = JoinModule::new();

    // entry: resolve_entries(entries, n, modules, seen, prefix_init)
    let resolve_id = JoinFuncId::new(0);
    let entries_param = stage1_vid::entry(0);
    let n_param = stage1_vid::entry(1);
    let modules_param = stage1_vid::entry(2);
    let seen_param = stage1_vid::entry(3);
    let prefix_param = stage1_vid::entry(4);
    let mut resolve_func = JoinFunction::new(
        resolve_id,
        "resolve_entries".to_string(),
        vec![
            entries_param,
            n_param,
            modules_param,
            seen_param,
            prefix_param,
        ],
    );

    let i_init = stage1_vid::entry(10);
    resolve_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Const {
            dst: i_init,
            value: ConstValue::Integer(0),
        }));

    // Phase 192: Use EntryFunctionBuilder for boilerplate initialization
    let mut entry_builder = EntryFunctionBuilder::new();
    entry_builder.add_var(entries_key.clone(), entries_param);
    entry_builder.add_var(n_key.clone(), n_param);
    entry_builder.add_var(modules_key.clone(), modules_param);
    entry_builder.add_var(seen_key.clone(), seen_param);
    entry_builder.add_var(prefix_key.clone(), prefix_param);
    entry_builder.add_var(i_key.clone(), i_init);
    let entry_name_to_id = entry_builder.get_map().clone();

    let loop_call_args: Vec<ValueId> = ctx
        .ordered_pinned
        .iter()
        .chain(ctx.ordered_carriers.iter())
        .map(|name| entry_name_to_id.get(name).copied())
        .collect::<Option<_>>()?;

    let loop_step_id = JoinFuncId::new(1);
    resolve_func.body.push(JoinInst::Call {
        func: loop_step_id,
        args: loop_call_args,
        k_next: None,
        dst: None,
    });

    join_module.entry = Some(resolve_id);
    join_module.add_function(resolve_func);

    // loop_step(entries, n, modules, seen, prefix, i)
    let header_shape = LoopHeaderShape::new_manual(ctx.pinned_ids.clone(), ctx.carrier_ids.clone());
    let loop_params = header_shape.to_loop_step_params();
    let mut loop_step_func =
        JoinFunction::new(loop_step_id, "loop_step".to_string(), loop_params.clone());

    let cmp_result = stage1_vid::loop_step(10);
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Compare {
            dst: cmp_result,
            op: CompareOp::Ge,
            lhs: i_loop,
            rhs: n_loop,
        }));

    let exit_shape = if ctx.exit_args.is_empty() {
        LoopExitShape::new_manual(vec![prefix_loop])
    } else {
        LoopExitShape::new_manual(ctx.exit_args.clone())
    };

    loop_step_func.body.push(JoinInst::Jump {
        cont: JoinContId::new(0),
        args: exit_shape.exit_args.clone(),
        cond: Some(cmp_result),
    });

    let entry_value = stage1_vid::loop_step(11);
    let next_i = stage1_vid::loop_step(12);
    let const_1 = stage1_vid::loop_step(13);
    let new_prefix = stage1_vid::loop_step(14);

    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BoxCall {
            dst: Some(entry_value),
            box_name: "ArrayBox".to_string(),
            method: "get".to_string(),
            args: vec![entries_loop, i_loop],
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
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BinOp {
            dst: new_prefix,
            op: BinOpKind::Or,
            lhs: prefix_loop,
            rhs: entry_value,
        }));
    loop_step_func.body.push(JoinInst::Call {
        func: loop_step_id,
        args: vec![
            entries_loop,
            n_loop,
            modules_loop,
            seen_loop,
            new_prefix,
            next_i,
        ],
        k_next: None,
        dst: None,
    });

    join_module.add_function(loop_step_func);

    if crate::config::env::joinir_dev::debug_enabled() {
        get_global_ring0().log.debug(&format!(
            "[joinir/generic_case_a/stage1] ✅ constructed JoinIR (functions={}, value_range={}..{})",
            join_module.functions.len(),
            value_id_ranges::base::STAGE1_USING_RESOLVER,
            value_id_ranges::base::STAGE1_USING_RESOLVER + 1999
        ));
    }

    Some(join_module)
}
