//! Phase 27.1: FuncScannerBox.trim/1 の MIR → JoinIR 変換
//!
//! 目的: lang/src/compiler/entry/func_scanner.hako の trim メソッドを JoinIR に変換
//!
//! 期待される変換:
//! ```text
//! // MIR (元):
//! method trim(s) {
//!   local e = n
//!   loop(e > b) {
//!     local ch = str.substring(e - 1, e)
//!     if ch == " " || ch == "\t" || ch == "\n" || ch == "\r" {
//!       e = e - 1
//!     } else {
//!       break
//!     }
//!   }
//!   return substring(b, e)
//! }
//!
//! // JoinIR (変換後):
//! fn trim_main(s_param, k_exit) {
//!     str = "" + s_param
//!     n = str.length()
//!     b = skip_whitespace(str, 0)
//!     e_init = n
//!     loop_step(str, b, e_init, k_exit)
//! }
//!
//! fn loop_step(str, b, e, k_exit) {
//!     cond = (e > b)
//!     if cond {
//!         ch = str.substring(e - 1, e)
//!         is_space = (ch == " " || ch == "\t" || ch == "\n" || ch == "\r")
//!         if is_space {
//!             e_next = e - 1
//!             loop_step(str, b, e_next, k_exit)
//!         } else {
//!             k_exit(e)
//!         }
//!     } else {
//!         k_exit(e)
//!     }
//! }
//! ```

use crate::mir::join_ir::{
    BinOpKind, ConstValue, JoinFuncId, JoinFunction, JoinInst, JoinModule, MirLikeInst,
};
use crate::mir::ValueId;
use crate::runtime::get_global_ring0;

use super::loop_step::build_loop_step_function;
use super::skip_leading::build_skip_leading_function;

/// Phase 27.11: Common JoinIR builder for FuncScannerBox.trim/1
///
/// This function generates the JoinIR for trim/1, shared by both:
/// - lower_trim_handwritten (always uses this)
/// - lower_trim_from_mir (uses this after CFG sanity checks pass)
pub(super) fn build_funcscanner_trim_joinir(module: &crate::mir::MirModule) -> Option<JoinModule> {
    // Step 1: "FuncScannerBox.trim/1" を探す
    let target_func = module.functions.get("FuncScannerBox.trim/1")?;

    if crate::config::env::joinir_dev::debug_enabled() {
        let ring0 = get_global_ring0();
        ring0
            .log
            .debug("[joinir/trim/build] Found FuncScannerBox.trim/1");
        ring0.log.debug(&format!(
            "[joinir/trim/build] MIR blocks: {}",
            target_func.blocks.len()
        ));
    }

    let mut join_module = JoinModule::new();

    // Phase 29bq: k_exit continuation (SSOT for Jump → tail-call return)
    //
    // JoinInst::Jump is lowered as a tail call to a continuation function.
    // For trim/1 we use a single 1-arg continuation that simply returns its argument.
    let k_exit_id = JoinFuncId::new(3);
    let k_exit_param = ValueId(8000);
    let mut k_exit_func = JoinFunction::new(k_exit_id, "k_exit".to_string(), vec![k_exit_param]);
    k_exit_func.body.push(JoinInst::Ret {
        value: Some(k_exit_param),
    });
    join_module.add_function(k_exit_func);

    // trim_main 関数: 前処理 + 先頭/末尾の空白を除去
    let trim_main_id = JoinFuncId::new(0);
    let s_param = ValueId(5000);
    let mut trim_main_func =
        JoinFunction::new(trim_main_id, "trim_main".to_string(), vec![s_param]);

    let str_val = ValueId(5001);
    let n_val = ValueId(5002);
    let b_val = ValueId(5003);
    let e_init = ValueId(5004);
    let const_empty = ValueId(5005);
    let const_zero = ValueId(5006);

    // str = "" + s_param (文字列化)
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

    // n = str.length()
    trim_main_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BoxCall {
            dst: Some(n_val),
            box_name: "StringBox".to_string(),
            method: "length".to_string(),
            args: vec![str_val],
        }));

    // const 0
    trim_main_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Const {
            dst: const_zero,
            value: ConstValue::Integer(0),
        }));

    // b = skip_leading_whitespace(str, 0, n)
    let skip_leading_id = JoinFuncId::new(2);
    trim_main_func.body.push(JoinInst::Call {
        func: skip_leading_id,
        args: vec![str_val, const_zero, n_val],
        k_next: None,
        dst: Some(b_val),
    });

    // e_init = n (コピー)
    trim_main_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BinOp {
            dst: e_init,
            op: BinOpKind::Add,
            lhs: n_val,
            rhs: const_zero,
        }));

    // loop_step(str, b, e_init) -> 戻り値をそのまま返す
    let loop_step_id = JoinFuncId::new(1);
    trim_main_func.body.push(JoinInst::Call {
        func: loop_step_id,
        args: vec![str_val, b_val, e_init],
        k_next: None,
        dst: None,
    });

    join_module.entry = Some(trim_main_id);
    join_module.add_function(trim_main_func);

    join_module.add_function(build_loop_step_function(loop_step_id, k_exit_id));
    join_module.add_function(build_skip_leading_function(skip_leading_id, k_exit_id));
    if crate::config::env::joinir_dev::debug_enabled() {
        get_global_ring0().log.debug(&format!(
            "[joinir/trim] Generated {} JoinIR functions",
            join_module.functions.len()
        ));
    }

    Some(join_module)
}
