//! Route-local JoinIR builder for `Main.skip/1`.

use crate::mir::join_ir::{
    BinOpKind, CompareOp, ConstValue, JoinContId, JoinFuncId, JoinFunction, JoinInst, JoinModule,
    LoopExitShape, LoopHeaderShape, MirLikeInst,
};
use crate::mir::ValueId;
use crate::runtime::get_global_ring0;

/// Phase 27.11.1: Common JoinIR builder for Main.skip/1
///
/// This function generates the JoinIR for skip/1, shared by both:
/// - lower_skip_ws_handwritten (always uses this)
/// - lower_skip_ws_from_mir (uses this after CFG sanity checks pass)
pub(super) fn build_skip_ws_joinir(module: &crate::mir::MirModule) -> Option<JoinModule> {
    // Step 1: "Main.skip/1" を探す
    let target_func = module.functions.get("Main.skip/1")?;

    if crate::config::env::joinir_dev::debug_enabled() {
        let ring0 = get_global_ring0();
        ring0.log.debug("[joinir/skip_ws/build] Found Main.skip/1");
        ring0.log.debug(&format!(
            "[joinir/skip_ws/build] MIR blocks: {}",
            target_func.blocks.len()
        ));
    }

    // Step 2: JoinModule を構築
    let mut join_module = JoinModule::new();

    // Phase 27.1: 固定的な JoinIR を生成（実際の MIR 解析は Phase 28 以降）

    // skip 関数: i_init = 0, n = s.length(), loop_step(s, 0, n, k_exit)
    let skip_id = JoinFuncId::new(0);
    let s_param = ValueId(3000);
    let mut skip_func = JoinFunction::new(skip_id, "skip".to_string(), vec![s_param]);

    let i_init = ValueId(3001);
    let n = ValueId(3002);

    // i_init = 0
    skip_func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: i_init,
        value: ConstValue::Integer(0),
    }));

    // n = s.length() (BoxCall でメソッド呼び出し)
    skip_func.body.push(JoinInst::Compute(MirLikeInst::BoxCall {
        dst: Some(n),
        box_name: "StringBox".to_string(),
        method: "length".to_string(),
        args: vec![s_param],
    }));

    // loop_step(s, i_init, n, k_exit)
    let loop_step_id = JoinFuncId::new(1);
    skip_func.body.push(JoinInst::Call {
        func: loop_step_id,
        args: vec![s_param, i_init, n],
        k_next: None,
        dst: None,
    });

    join_module.entry = Some(skip_id);
    join_module.add_function(skip_func);

    // Phase 27.4-A: loop_step の Pinned/Carrier 構造を明示
    // skip_ws ループの場合:
    //   - Pinned: s (文字列), n (長さ) - ループ中で不変
    //   - Carrier: i (現在位置) - ループで更新される
    let s_loop = ValueId(4000); // Pinned
    let i_loop = ValueId(4001); // Carrier
    let n_loop = ValueId(4002); // Pinned

    let _header_shape = LoopHeaderShape::new_manual(
        vec![s_loop, n_loop], // Pinned: s, n
        vec![i_loop],         // Carrier: i
    );
    // 将来: LoopHeaderShape.to_loop_step_params() は [pinned..., carriers...] の順を返す。
    // 現在は既存 JoinIR テストとの互換性のため、手動で [s, i, n] の順を維持している。

    // loop_step 関数: if i >= n { return i } else if ch == " " { loop_step(i + 1) } else { return i }
    let mut loop_step_func = JoinFunction::new(
        loop_step_id,
        "loop_step".to_string(),
        vec![s_loop, i_loop, n_loop], // [pinned, carrier, pinned] の順（現行実装）
    );

    let cmp1_result = ValueId(4003);
    let ch = ValueId(4004);
    let cmp2_result = ValueId(4005);
    let i_plus_1 = ValueId(4006);
    let const_1 = ValueId(4007);
    let const_space = ValueId(4010);
    let bool_false = ValueId(4011);
    let cmp2_is_false = ValueId(4012);

    // cmp1_result = (i >= n)
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Compare {
            dst: cmp1_result,
            op: CompareOp::Ge,
            lhs: i_loop,
            rhs: n_loop,
        }));

    // Phase 27.5: Exit φ の意味を LoopExitShape で明示
    // skip_ws のループ脱出時は i の値だけを返す（先頭空白の文字数）
    let _exit_shape = LoopExitShape::new_manual(vec![i_loop]); // exit_args = [i]

    // if i >= n { return i }
    loop_step_func.body.push(JoinInst::Jump {
        cont: JoinContId::new(0),
        args: vec![i_loop], // ← LoopExitShape.exit_args に対応
        cond: Some(cmp1_result),
    });

    // const 1
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Const {
            dst: const_1,
            value: ConstValue::Integer(1),
        }));

    // i_plus_1 = i + 1 (再利用: substring end / continue path)
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

    // const " " (space)
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

    // bool false (for negation)
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

    // Phase 27.5: 2箇所目の exit パス（同じく exit_args = [i]）
    // if ch != " " { return i }
    loop_step_func.body.push(JoinInst::Jump {
        cont: JoinContId::new(1),
        args: vec![i_loop], // ← LoopExitShape.exit_args に対応（1箇所目と同じ）
        cond: Some(cmp2_is_false),
    });

    // continue path: loop_step(s, i + 1, n)
    loop_step_func.body.push(JoinInst::Call {
        func: loop_step_id,
        args: vec![s_loop, i_plus_1, n_loop],
        k_next: None,
        dst: None,
    });

    join_module.add_function(loop_step_func);

    if crate::config::env::joinir_dev::debug_enabled() {
        get_global_ring0().log.debug(&format!(
            "[joinir/skip_ws] Generated {} JoinIR functions",
            join_module.functions.len()
        ));
    }

    Some(join_module)
}
