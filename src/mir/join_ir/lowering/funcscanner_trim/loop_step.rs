//! Route-local `loop_step` JoinIR function builder for trim lowering.

use crate::mir::join_ir::lowering::common::string_whitespace::{
    append_string_whitespace_predicate, WhitespacePredicateIds,
};
use crate::mir::join_ir::{
    BinOpKind, CompareOp, ConstValue, JoinFuncId, JoinFunction, JoinInst, LoopExitShape,
    LoopHeaderShape, MirLikeInst,
};
use crate::mir::ValueId;

pub(super) fn build_loop_step_function(
    loop_step_id: JoinFuncId,
    k_exit_id: JoinFuncId,
) -> JoinFunction {
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
                                                                    // 実装上は既に trimmed_base =
                                                                    // substring(b, e) を計算済み。

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

    let or1 = ValueId(6016);
    let or2 = ValueId(6017);
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
            is_space: ValueId(6018),
        },
    );

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

    loop_step_func
}
