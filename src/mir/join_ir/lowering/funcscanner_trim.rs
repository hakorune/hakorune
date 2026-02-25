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
    BinOpKind, CompareOp, ConstValue, JoinFuncId, JoinFunction, JoinInst, JoinModule,
    LoopExitShape, LoopHeaderShape, MirLikeInst,
};
use crate::mir::ValueId;
use crate::runtime::get_global_ring0;

/// Phase 27.9: Toggle dispatcher for trim lowering
/// - Default: handwritten lowering
/// - NYASH_JOINIR_LOWER_FROM_MIR=1: MIR-based lowering
pub fn lower_funcscanner_trim_to_joinir(module: &crate::mir::MirModule) -> Option<JoinModule> {
    super::common::dispatch_lowering("trim", module, lower_trim_from_mir, lower_trim_handwritten)
}

/// Phase 27.11: Common JoinIR builder for FuncScannerBox.trim/1
///
/// This function generates the JoinIR for trim/1, shared by both:
/// - lower_trim_handwritten (always uses this)
/// - lower_trim_from_mir (uses this after CFG sanity checks pass)
fn build_funcscanner_trim_joinir(module: &crate::mir::MirModule) -> Option<JoinModule> {
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
    k_exit_func
        .body
        .push(JoinInst::Ret { value: Some(k_exit_param) });
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

    // Phase 27.4-A: trim loop_step の Pinned/Carrier 構造を明示
    // trim ループの場合:
    //   - Pinned: str (文字列), b (開始位置) - ループ中で不変
    //   - Carrier: e (終了位置) - ループで後ろから前へ更新される
    let str_loop = ValueId(6000); // Pinned
    let b_loop = ValueId(6001); // Pinned
    let e_loop = ValueId(6002); // Carrier

    let _header_shape = LoopHeaderShape::new_manual(
        vec![str_loop, b_loop], // Pinned: str, b
        vec![e_loop],           // Carrier: e
    );
    // 将来: to_loop_step_params() で [str, b, e] (pinned..., carriers...) を生成する設計。
    // 現在は既存 JoinIR テストとの互換性のため、手動で [str, b, e] の順を維持している。

    // loop_step 関数: 末尾の空白を削り、最終的に substring(b, e) を返す
    let mut loop_step_func = JoinFunction::new(
        loop_step_id,
        "loop_step".to_string(),
        vec![str_loop, b_loop, e_loop],
    );

    // cond = (e > b)
    let cond = ValueId(6003);
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Compare {
            dst: cond,
            lhs: e_loop,
            rhs: b_loop,
            op: CompareOp::Gt,
        }));

    // bool false (共通)
    let bool_false = ValueId(6019);
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Const {
            dst: bool_false,
            value: ConstValue::Bool(false),
        }));

    // trimmed_base = str.substring(b, e)
    let trimmed_base = ValueId(6004);
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BoxCall {
            dst: Some(trimmed_base),
            box_name: "StringBox".to_string(),
            method: "substring".to_string(),
            args: vec![str_loop, b_loop, e_loop],
        }));

    // cond_is_false = (cond == false)
    let cond_is_false = ValueId(6020);
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Compare {
            dst: cond_is_false,
            lhs: cond,
            rhs: bool_false,
            op: CompareOp::Eq,
        }));

    // Phase 27.5: Exit φ の意味を LoopExitShape で明示（Option A）
    // trim のループ脱出時は e の値で substring(b, e) を計算済み
    let _exit_shape_trim = LoopExitShape::new_manual(vec![e_loop]); // exit_args = [e] (Option A)
                                                                    // 実装上は既に trimmed_base = substring(b, e) を計算済みで、その結果を返している

    // if !(e > b) { return substring(b, e) }
    loop_step_func.body.push(JoinInst::Jump {
        cont: k_exit_id.as_cont(),
        args: vec![trimmed_base], // ← substring(b, e) の結果
        cond: Some(cond_is_false),
    });

    // const 1
    let const_1 = ValueId(6005);
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Const {
            dst: const_1,
            value: ConstValue::Integer(1),
        }));

    // e_minus_1 = e - 1
    let e_minus_1 = ValueId(6006);
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BinOp {
            dst: e_minus_1,
            lhs: e_loop,
            rhs: const_1,
            op: BinOpKind::Sub,
        }));

    let ch = ValueId(6007);
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BoxCall {
            dst: Some(ch),
            box_name: "StringBox".to_string(),
            method: "substring".to_string(),
            args: vec![str_loop, e_minus_1, e_loop],
        }));

    // is_space = (ch == " " || ch == "\\t" || ch == "\\n" || ch == "\\r")
    let cmp_space = ValueId(6008);
    let cmp_tab = ValueId(6009);
    let cmp_newline = ValueId(6010);
    let cmp_cr = ValueId(6011);

    let const_space = ValueId(6012);
    let const_tab = ValueId(6013);
    let const_newline = ValueId(6014);
    let const_cr = ValueId(6015);

    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Const {
            dst: const_space,
            value: ConstValue::String(" ".to_string()),
        }));
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Compare {
            dst: cmp_space,
            lhs: ch,
            rhs: const_space,
            op: CompareOp::Eq,
        }));

    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Const {
            dst: const_tab,
            value: ConstValue::String("\\t".to_string()),
        }));
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Compare {
            dst: cmp_tab,
            lhs: ch,
            rhs: const_tab,
            op: CompareOp::Eq,
        }));

    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Const {
            dst: const_newline,
            value: ConstValue::String("\\n".to_string()),
        }));
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Compare {
            dst: cmp_newline,
            lhs: ch,
            rhs: const_newline,
            op: CompareOp::Eq,
        }));

    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Const {
            dst: const_cr,
            value: ConstValue::String("\\r".to_string()),
        }));
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Compare {
            dst: cmp_cr,
            lhs: ch,
            rhs: const_cr,
            op: CompareOp::Eq,
        }));

    // OR chain: (cmp_space || cmp_tab) || cmp_newline || cmp_cr
    let or1 = ValueId(6016);
    let or2 = ValueId(6017);
    let is_space = ValueId(6018);

    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BinOp {
            dst: or1,
            lhs: cmp_space,
            rhs: cmp_tab,
            op: BinOpKind::Or,
        }));

    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BinOp {
            dst: or2,
            lhs: or1,
            rhs: cmp_newline,
            op: BinOpKind::Or,
        }));

    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BinOp {
            dst: is_space,
            lhs: or2,
            rhs: cmp_cr,
            op: BinOpKind::Or,
        }));

    // is_space_false = (is_space == false)
    let is_space_false = ValueId(6021);
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Compare {
            dst: is_space_false,
            lhs: is_space,
            rhs: bool_false,
            op: CompareOp::Eq,
        }));

    // Phase 27.5: 2箇所目の exit パス（同じく exit_args = [e], Option A）
    // if !is_space { return substring(b, e) }
    loop_step_func.body.push(JoinInst::Jump {
        cont: k_exit_id.as_cont(),
        args: vec![trimmed_base], // ← substring(b, e) の結果（1箇所目と同じ）
        cond: Some(is_space_false),
    });

    // continue path: e_next = e - 1; loop_step(str, b, e_next)
    let e_next = ValueId(6022);
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BinOp {
            dst: e_next,
            lhs: e_loop,
            rhs: const_1,
            op: BinOpKind::Sub,
        }));

    loop_step_func.body.push(JoinInst::Call {
        func: loop_step_id, // 再帰呼び出し
        args: vec![str_loop, b_loop, e_next],
        k_next: None,
        dst: None,
    });

    join_module.add_function(loop_step_func);

    // skip_leading 関数: 先頭の空白をスキップして位置を返す
    let mut skip_func = JoinFunction::new(
        skip_leading_id,
        "skip_leading".to_string(),
        vec![ValueId(7000), ValueId(7001), ValueId(7002)], // (s, i, n)
    );
    let s_skip = ValueId(7000);
    let i_skip = ValueId(7001);
    let n_skip = ValueId(7002);
    let cmp_len = ValueId(7003);
    let const_1_skip = ValueId(7004);
    let i_plus_1_skip = ValueId(7005);
    let ch_skip = ValueId(7006);
    let cmp_space_skip = ValueId(7007);
    let cmp_tab_skip = ValueId(7008);
    let cmp_newline_skip = ValueId(7009);
    let cmp_cr_skip = ValueId(7010);
    let const_space_skip = ValueId(7011);
    let const_tab_skip = ValueId(7012);
    let const_newline_skip = ValueId(7013);
    let const_cr_skip = ValueId(7014);
    let or1_skip = ValueId(7015);
    let or2_skip = ValueId(7016);
    let is_space_skip = ValueId(7017);
    let bool_false_skip = ValueId(7018);
    let is_space_false_skip = ValueId(7019);

    // cmp_len = (i >= n)
    skip_func.body.push(JoinInst::Compute(MirLikeInst::Compare {
        dst: cmp_len,
        lhs: i_skip,
        rhs: n_skip,
        op: CompareOp::Ge,
    }));

    // if i >= n { return i }
    skip_func.body.push(JoinInst::Jump {
        cont: k_exit_id.as_cont(),
        args: vec![i_skip],
        cond: Some(cmp_len),
    });

    // const 1
    skip_func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: const_1_skip,
        value: ConstValue::Integer(1),
    }));

    // i_plus_1 = i + 1
    skip_func.body.push(JoinInst::Compute(MirLikeInst::BinOp {
        dst: i_plus_1_skip,
        lhs: i_skip,
        rhs: const_1_skip,
        op: BinOpKind::Add,
    }));

    // ch = s.substring(i, i + 1)
    skip_func.body.push(JoinInst::Compute(MirLikeInst::BoxCall {
        dst: Some(ch_skip),
        box_name: "StringBox".to_string(),
        method: "substring".to_string(),
        args: vec![s_skip, i_skip, i_plus_1_skip],
    }));

    // whitespace constants + comparisons
    skip_func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: const_space_skip,
        value: ConstValue::String(" ".to_string()),
    }));
    skip_func.body.push(JoinInst::Compute(MirLikeInst::Compare {
        dst: cmp_space_skip,
        lhs: ch_skip,
        rhs: const_space_skip,
        op: CompareOp::Eq,
    }));

    skip_func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: const_tab_skip,
        value: ConstValue::String("\\t".to_string()),
    }));
    skip_func.body.push(JoinInst::Compute(MirLikeInst::Compare {
        dst: cmp_tab_skip,
        lhs: ch_skip,
        rhs: const_tab_skip,
        op: CompareOp::Eq,
    }));

    skip_func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: const_newline_skip,
        value: ConstValue::String("\\n".to_string()),
    }));
    skip_func.body.push(JoinInst::Compute(MirLikeInst::Compare {
        dst: cmp_newline_skip,
        lhs: ch_skip,
        rhs: const_newline_skip,
        op: CompareOp::Eq,
    }));

    skip_func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: const_cr_skip,
        value: ConstValue::String("\\r".to_string()),
    }));
    skip_func.body.push(JoinInst::Compute(MirLikeInst::Compare {
        dst: cmp_cr_skip,
        lhs: ch_skip,
        rhs: const_cr_skip,
        op: CompareOp::Eq,
    }));

    // is_space_skip = OR chain
    skip_func.body.push(JoinInst::Compute(MirLikeInst::BinOp {
        dst: or1_skip,
        lhs: cmp_space_skip,
        rhs: cmp_tab_skip,
        op: BinOpKind::Or,
    }));
    skip_func.body.push(JoinInst::Compute(MirLikeInst::BinOp {
        dst: or2_skip,
        lhs: or1_skip,
        rhs: cmp_newline_skip,
        op: BinOpKind::Or,
    }));
    skip_func.body.push(JoinInst::Compute(MirLikeInst::BinOp {
        dst: is_space_skip,
        lhs: or2_skip,
        rhs: cmp_cr_skip,
        op: BinOpKind::Or,
    }));

    // bool false + negation
    skip_func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: bool_false_skip,
        value: ConstValue::Bool(false),
    }));
    skip_func.body.push(JoinInst::Compute(MirLikeInst::Compare {
        dst: is_space_false_skip,
        lhs: is_space_skip,
        rhs: bool_false_skip,
        op: CompareOp::Eq,
    }));

    // if not space -> return i
    skip_func.body.push(JoinInst::Jump {
        cont: k_exit_id.as_cont(),
        args: vec![i_skip],
        cond: Some(is_space_false_skip),
    });

    // continue path: skip_leading(s, i + 1, n)
    skip_func.body.push(JoinInst::Call {
        func: skip_leading_id,
        args: vec![s_skip, i_plus_1_skip, n_skip],
        k_next: None,
        dst: None,
    });

    join_module.add_function(skip_func);
    if crate::config::env::joinir_dev::debug_enabled() {
        get_global_ring0().log.debug(&format!(
            "[joinir/trim] Generated {} JoinIR functions",
            join_module.functions.len()
        ));
    }

    Some(join_module)
}

/// Phase 27.11: Handwritten lowering wrapper for FuncScannerBox.trim/1
///
/// This is a thin wrapper that calls the shared build_funcscanner_trim_joinir() function.
/// Maintains the handwritten lowering path as the baseline reference.
fn lower_trim_handwritten(module: &crate::mir::MirModule) -> Option<JoinModule> {
    if crate::config::env::joinir_dev::debug_enabled() {
        get_global_ring0()
            .log
            .debug("[joinir/trim/handwritten] Using handwritten lowering path");
    }
    build_funcscanner_trim_joinir(module)
}

/// Phase 27.9: MIR-based lowering for FuncScannerBox.trim/1
/// - Lightweight CFG sanity checks
/// - Fallback to handwritten if MIR structure is unexpected
fn lower_trim_from_mir(module: &crate::mir::MirModule) -> Option<JoinModule> {
    use super::common::{
        ensure_entry_has_succs, has_binop, has_const_string, has_string_method, log_fallback,
    };
    use crate::mir::query::MirQueryBox;
    use crate::mir::BinaryOp;

    // Step 1: "FuncScannerBox.trim/1" を探す
    let target_func = module.functions.get("FuncScannerBox.trim/1")?;

    if crate::config::env::joinir_dev::debug_enabled() {
        let ring0 = get_global_ring0();
        ring0
            .log
            .debug("[joinir/trim/mir] Found FuncScannerBox.trim/1 (MIR-based lowering)");
        ring0.log.debug(&format!(
            "[joinir/trim/mir] MIR blocks: {}",
            target_func.blocks.len()
        ));
    }

    // Phase 27.10: Lightweight CFG sanity checks using common utilities
    let query = MirQueryBox::new(target_func);
    let entry_id = target_func.entry_block;

    // Check 1: Entry block has at least 1 successor
    if !ensure_entry_has_succs(&query, entry_id) {
        log_fallback("trim", "entry has no successors");
        return lower_trim_handwritten(module);
    }

    // Check 2: Entry block contains expected patterns
    // - Const("") for string coercion
    // - BoxCall(String.length)
    // - BinOp(Add) for "" + s
    if !has_const_string(&query, entry_id, "")
        || !has_string_method(&query, entry_id, "length")
        || !has_binop(&query, entry_id, BinaryOp::Add)
    {
        log_fallback(
            "trim",
            "entry block missing expected patterns (Const(\"\"), String.length, or BinOp(Add))",
        );
        return lower_trim_handwritten(module);
    }

    if crate::config::env::joinir_dev::debug_enabled() {
        get_global_ring0()
            .log
            .debug("[joinir/trim/mir] CFG sanity checks passed ✅");
    }

    // Phase 31: LoopToJoinLowerer 統一箱経由に移行
    // Phase 32: construct_simple_while_loopform 共通ヘルパーを使用
    if crate::mir::join_ir::env_flag_is_1("NYASH_JOINIR_LOWER_GENERIC") {
        use crate::mir::join_ir::lowering::common::construct_simple_while_loopform;
        use crate::mir::join_ir::lowering::loop_to_join::LoopToJoinLowerer;

        // trim: entry_is_preheader=true, has_break=true
        let Some(loop_form) = construct_simple_while_loopform(entry_id, &query, true, true) else {
            if crate::config::env::joinir_dev::debug_enabled() {
                get_global_ring0()
                    .log
                    .debug("[joinir/trim/generic-hook] failed to construct LoopForm from CFG");
            }
            return build_funcscanner_trim_joinir(module);
        };
        if crate::mir::join_ir::lowering::common::case_a::is_simple_case_a_loop(&loop_form) {
            if crate::config::env::joinir_dev::debug_enabled() {
                get_global_ring0().log.debug(
                    "[joinir/trim/generic-hook] simple Case A loop detected (LoopToJoinLowerer)",
                );
            }
            let lowerer = LoopToJoinLowerer::new();
            if let Some(jm) = lowerer.lower_case_a_for_trim(target_func, &loop_form) {
                if crate::config::env::joinir_dev::debug_enabled() {
                    get_global_ring0().log.debug(
                        "[joinir/trim/generic-hook] LoopToJoinLowerer produced JoinIR, returning early",
                    );
                }
                return Some(jm);
            }
            if crate::config::env::joinir_dev::debug_enabled() {
                get_global_ring0().log.debug(
                    "[joinir/trim/generic-hook] LoopToJoinLowerer returned None, falling back to handwritten",
                );
            }
        }
    }

    // Phase 27.11: Generate JoinIR using shared builder
    // CFG checks passed, so we can use build_funcscanner_trim_joinir() directly
    if crate::config::env::joinir_dev::debug_enabled() {
        get_global_ring0().log.debug(
            "[joinir/trim/mir] Calling build_funcscanner_trim_joinir() after CFG validation",
        );
    }
    build_funcscanner_trim_joinir(module)
}
