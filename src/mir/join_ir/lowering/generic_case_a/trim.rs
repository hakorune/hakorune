//! String Trim Loop Lowering (Case A)
//!
//! Phase 192: Extracted from generic_case_a.rs monolith.
//!
//! ## Responsibility
//!
//! Lowers FuncScannerBox.trim/1 string trimming loop to JoinIR.
//!
//! ## Shape Structure
//!
//! ```text
//! entry: trim_main(s)
//!   str = "" + s               // Copy string
//!   n = str.length()
//!   b = skip_leading(str, 0, n)  // Find first non-space
//!   e = n + 0
//!   call loop_step(str, b, e)
//!
//! loop_step(str, b, e):
//!   if e <= b { return str.substring(b, e) }  // Empty or done
//!   ch = str.substring(e-1, e)
//!   if ch not in [' ', '\t', '\n', '\r'] {
//!     return str.substring(b, e)  // Non-space found
//!   }
//!   continue loop_step(str, b, e-1)  // Skip trailing space
//!
//! skip_leading(s, i, n):
//!   if i >= n { return i }
//!   ch = s.substring(i, i+1)
//!   if ch not in [' ', '\t', '\n', '\r'] {
//!     return i  // Non-space found
//!   }
//!   continue skip_leading(s, i+1, n)  // Skip leading space
//! ```
//!
//! ## ValueId Allocation
//!
//! - Entry range: 5000-5999 (`value_id_ranges::funcscanner_trim::entry`)
//! - Loop range: 6000-6999 (`value_id_ranges::funcscanner_trim::loop_step`)
//! - Skip range: 7000-7999 (hardcoded ValueId for skip_leading)
//!
//! ## See Also
//!
//! - `value_id_ranges::funcscanner_trim` - ValueId allocation strategy

use crate::mir::join_ir::lowering::common::string_whitespace::{
    append_string_whitespace_predicate, WhitespacePredicateIds,
};
use crate::mir::join_ir::lowering::loop_scope_shape::CaseAContext;
use crate::mir::join_ir::lowering::value_id_ranges;
use crate::mir::join_ir::{
    BinOpKind, CompareOp, ConstValue, JoinContId, JoinFuncId, JoinFunction, JoinInst, JoinModule,
    LoopExitShape, LoopHeaderShape, MirLikeInst,
};
use crate::mir::ValueId;
use crate::runtime::get_global_ring0;

use super::entry_builder::EntryFunctionBuilder;

mod skip_leading;

/// Phase 30 F-3.0.3: LoopScopeShape を直接受け取る trim lowerer
///
/// 呼び出し元で LoopScopeShape を明示的に構築し、この関数に渡す。
/// CaseAContext::from_scope() 経由で ctx を作成。
pub(crate) fn lower_case_a_trim_with_scope(
    scope: crate::mir::join_ir::lowering::loop_scope_shape::LoopScopeShape,
) -> Option<JoinModule> {
    let ctx = CaseAContext::from_scope(scope, "trim", |offset| {
        value_id_ranges::funcscanner_trim::loop_step(offset)
    })?;
    lower_case_a_trim_core(&ctx)
}

/// trim JoinModule 構築のコア実装
///
/// CaseAContext から JoinModule を構築する共通ロジック。
/// `_for_trim_minimal` と `_with_scope` の両方から呼ばれる。
fn lower_case_a_trim_core(ctx: &CaseAContext) -> Option<JoinModule> {
    let string_key = ctx.pinned_name_or_first(0)?;
    let base_key = ctx
        .pinned_name_or_first(1)
        .unwrap_or_else(|| string_key.clone());
    let carrier_key = ctx.carrier_name_or_first(0)?;

    let s_loop = ctx.get_loop_id(&string_key)?;
    let b_loop = ctx.get_loop_id(&base_key)?;
    let e_loop = ctx.get_loop_id(&carrier_key)?;

    let mut join_module = JoinModule::new();

    // entry: trim_main(s_param)
    let trim_main_id = JoinFuncId::new(0);
    let s_param = value_id_ranges::funcscanner_trim::entry(0);
    let mut trim_main_func =
        JoinFunction::new(trim_main_id, "trim_main".to_string(), vec![s_param]);

    let str_val = value_id_ranges::funcscanner_trim::entry(1);
    let n_val = value_id_ranges::funcscanner_trim::entry(2);
    let b_val = value_id_ranges::funcscanner_trim::entry(3);
    let e_init = value_id_ranges::funcscanner_trim::entry(4);
    let const_empty = value_id_ranges::funcscanner_trim::entry(5);
    let const_zero = value_id_ranges::funcscanner_trim::entry(6);

    trim_main_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Const {
            dst: const_empty,
            value: ConstValue::String("".to_string()),
        }));
    trim_main_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BinOp {
            dst: str_val,
            lhs: const_empty,
            rhs: s_param,
            op: BinOpKind::Add,
        }));
    trim_main_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BoxCall {
            dst: Some(n_val),
            box_name: "StringBox".to_string(),
            method: "length".to_string(),
            args: vec![str_val],
        }));
    trim_main_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Const {
            dst: const_zero,
            value: ConstValue::Integer(0),
        }));

    let skip_leading_id = JoinFuncId::new(2);
    trim_main_func.body.push(JoinInst::Call {
        func: skip_leading_id,
        args: vec![str_val, const_zero, n_val],
        k_next: None,
        dst: Some(b_val),
    });

    trim_main_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BinOp {
            dst: e_init,
            op: BinOpKind::Add,
            lhs: n_val,
            rhs: const_zero,
        }));

    // Phase 192: Use EntryFunctionBuilder for boilerplate initialization
    let mut entry_builder = EntryFunctionBuilder::new();
    entry_builder.add_var(string_key.clone(), str_val);
    entry_builder.add_var(base_key.clone(), b_val);
    entry_builder.add_var(carrier_key.clone(), e_init);
    let entry_name_to_id = entry_builder.get_map().clone();

    let loop_call_args: Vec<ValueId> = ctx
        .ordered_pinned
        .iter()
        .chain(ctx.ordered_carriers.iter())
        .map(|name| entry_name_to_id.get(name).copied())
        .collect::<Option<_>>()?;

    let loop_step_id = JoinFuncId::new(1);
    trim_main_func.body.push(JoinInst::Call {
        func: loop_step_id,
        args: loop_call_args,
        k_next: None,
        dst: None,
    });

    join_module.entry = Some(trim_main_id);
    join_module.add_function(trim_main_func);

    // loop_step(str, b, e)
    let header_shape = LoopHeaderShape::new_manual(ctx.pinned_ids.clone(), ctx.carrier_ids.clone());
    let loop_params = header_shape.to_loop_step_params();
    let mut loop_step_func =
        JoinFunction::new(loop_step_id, "loop_step".to_string(), loop_params.clone());

    let cond = value_id_ranges::funcscanner_trim::loop_step(3);
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Compare {
            dst: cond,
            lhs: e_loop,
            rhs: b_loop,
            op: CompareOp::Gt,
        }));

    let bool_false = value_id_ranges::funcscanner_trim::loop_step(19);
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Const {
            dst: bool_false,
            value: ConstValue::Bool(false),
        }));

    let trimmed_base = value_id_ranges::funcscanner_trim::loop_step(4);
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BoxCall {
            dst: Some(trimmed_base),
            box_name: "StringBox".to_string(),
            method: "substring".to_string(),
            args: vec![s_loop, b_loop, e_loop],
        }));

    let cond_is_false = value_id_ranges::funcscanner_trim::loop_step(20);
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Compare {
            dst: cond_is_false,
            lhs: cond,
            rhs: bool_false,
            op: CompareOp::Eq,
        }));

    let _exit_shape_trim = if ctx.exit_args.is_empty() {
        LoopExitShape::new_manual(vec![e_loop])
    } else {
        LoopExitShape::new_manual(ctx.exit_args.clone())
    };

    loop_step_func.body.push(JoinInst::Jump {
        cont: JoinContId::new(0),
        args: vec![trimmed_base],
        cond: Some(cond_is_false),
    });

    let const_1 = value_id_ranges::funcscanner_trim::loop_step(5);
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Const {
            dst: const_1,
            value: ConstValue::Integer(1),
        }));

    let e_minus_1 = value_id_ranges::funcscanner_trim::loop_step(6);
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BinOp {
            dst: e_minus_1,
            lhs: e_loop,
            rhs: const_1,
            op: BinOpKind::Sub,
        }));

    let ch = value_id_ranges::funcscanner_trim::loop_step(7);
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BoxCall {
            dst: Some(ch),
            box_name: "StringBox".to_string(),
            method: "substring".to_string(),
            args: vec![s_loop, e_minus_1, e_loop],
        }));

    let cmp_space = value_id_ranges::funcscanner_trim::loop_step(8);
    let cmp_tab = value_id_ranges::funcscanner_trim::loop_step(9);
    let cmp_newline = value_id_ranges::funcscanner_trim::loop_step(10);
    let cmp_cr = value_id_ranges::funcscanner_trim::loop_step(11);

    let const_space = value_id_ranges::funcscanner_trim::loop_step(12);
    let const_tab = value_id_ranges::funcscanner_trim::loop_step(13);
    let const_newline = value_id_ranges::funcscanner_trim::loop_step(14);
    let const_cr = value_id_ranges::funcscanner_trim::loop_step(15);

    let or1 = value_id_ranges::funcscanner_trim::loop_step(16);
    let or2 = value_id_ranges::funcscanner_trim::loop_step(17);
    let is_space = append_string_whitespace_predicate(
        &mut loop_step_func,
        ch,
        WhitespacePredicateIds {
            cmp_space,
            cmp_tab,
            cmp_newline,
            cmp_cr,
            const_space,
            const_tab,
            const_newline,
            const_cr,
            or1,
            or2,
            is_space: value_id_ranges::funcscanner_trim::loop_step(18),
        },
    );

    let is_space_false = value_id_ranges::funcscanner_trim::loop_step(21);
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Compare {
            dst: is_space_false,
            lhs: is_space,
            rhs: bool_false,
            op: CompareOp::Eq,
        }));

    loop_step_func.body.push(JoinInst::Jump {
        cont: JoinContId::new(1),
        args: vec![trimmed_base],
        cond: Some(is_space_false),
    });

    let e_next = value_id_ranges::funcscanner_trim::loop_step(22);
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BinOp {
            dst: e_next,
            lhs: e_loop,
            rhs: const_1,
            op: BinOpKind::Sub,
        }));

    loop_step_func.body.push(JoinInst::Call {
        func: loop_step_id,
        args: vec![s_loop, b_loop, e_next],
        k_next: None,
        dst: None,
    });

    join_module.add_function(loop_step_func);

    join_module.add_function(skip_leading::build(skip_leading_id));

    if crate::config::env::joinir_dev::debug_enabled() {
        get_global_ring0().log.debug(&format!(
            "[joinir/generic_case_a/trim] ✅ constructed JoinIR (functions={}, value_range={}..{})",
            join_module.functions.len(),
            value_id_ranges::base::FUNCSCANNER_TRIM,
            value_id_ranges::base::FUNCSCANNER_TRIM + 1999
        ));
    }

    Some(join_module)
}
