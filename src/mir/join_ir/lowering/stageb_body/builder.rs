//! Route-local JoinIR builder for `StageBBodyExtractorBox.build_body_src/2`.

use crate::mir::join_ir::lowering::value_id_ranges::stageb_body_extract as vid;
use crate::mir::join_ir::{
    BinOpKind, CompareOp, ConstValue, JoinContId, JoinFuncId, JoinFunction, JoinInst, JoinModule,
    LoopExitShape, LoopHeaderShape, MirLikeInst,
};
use crate::runtime::get_global_ring0;

/// Common JoinIR construction shared by MIR-probed and handwritten routes.
pub(super) fn build_stageb_body_joinir(module: &crate::mir::MirModule) -> Option<JoinModule> {
    // ターゲット関数が無ければ None でフォールバック
    let _target = module
        .functions
        .get("StageBBodyExtractorBox.build_body_src/2")?;

    if crate::config::env::joinir_dev::debug_enabled() {
        let ring0 = get_global_ring0();
        ring0
            .log
            .debug("[joinir/stageb_body/build] Generating JoinIR for build_body_src");
        ring0
            .log
            .debug("[joinir/stageb_body/build] Using ValueId range: 11000-12999");
    }

    let mut join_module = JoinModule::new();

    // Entry: build_body_src(src, args)
    let entry_id = JoinFuncId::new(0);
    let src_param = vid::entry(0); // 11000
    let args_param = vid::entry(1); // 11001
    let n_val = vid::entry(2); // 11002
    let i_init = vid::entry(3); // 11003
    let acc_init = vid::entry(4); // 11004
    let loop_step_id = JoinFuncId::new(1);

    let mut entry_func = JoinFunction::new(
        entry_id,
        "build_body_src".to_string(),
        vec![src_param, args_param],
    );

    // n = src.length()
    entry_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BoxCall {
            dst: Some(n_val),
            box_name: "StringBox".to_string(),
            method: "length".to_string(),
            args: vec![src_param],
        }));

    // i_init = 0
    entry_func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: i_init,
        value: ConstValue::Integer(0),
    }));

    // acc_init = 0
    entry_func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: acc_init,
        value: ConstValue::Integer(0),
    }));

    // loop_step(src, args, n, acc_init, i_init)
    entry_func.body.push(JoinInst::Call {
        func: loop_step_id,
        args: vec![src_param, args_param, n_val, acc_init, i_init],
        k_next: None,
        dst: None,
    });

    join_module.entry = Some(entry_id);
    join_module.add_function(entry_func);

    // Loop step: loop_step(src, args, n, acc, i)
    let src_loop = vid::loop_step(0); // 12000 (Pinned)
    let args_loop = vid::loop_step(1); // 12001 (Pinned)
    let n_loop = vid::loop_step(2); // 12002 (Pinned)
    let acc_loop = vid::loop_step(3); // 12003 (Carrier)
    let i_loop = vid::loop_step(4); // 12004 (Carrier)

    let _header_shape = LoopHeaderShape::new_manual(
        vec![src_loop, args_loop, n_loop], // Pinned
        vec![acc_loop, i_loop],            // Carrier
    );

    let mut loop_func = JoinFunction::new(
        loop_step_id,
        "loop_step".to_string(),
        vec![src_loop, args_loop, n_loop, acc_loop, i_loop],
    );

    let cmp_exit = vid::loop_step(10); // 12010
    let const_one = vid::loop_step(11); // 12011
    let next_i = vid::loop_step(12); // 12012
    let next_acc = vid::loop_step(13); // 12013

    // cmp_exit = (i >= n)
    loop_func.body.push(JoinInst::Compute(MirLikeInst::Compare {
        dst: cmp_exit,
        op: CompareOp::Ge,
        lhs: i_loop,
        rhs: n_loop,
    }));

    // Exit shape: acc をそのまま返す
    let _exit_shape = LoopExitShape::new_manual(vec![acc_loop]);

    // if i >= n { return acc }
    loop_func.body.push(JoinInst::Jump {
        cont: JoinContId::new(0),
        args: vec![acc_loop],
        cond: Some(cmp_exit),
    });

    // const_one = 1
    loop_func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: const_one,
        value: ConstValue::Integer(1),
    }));

    // next_i = i + 1
    loop_func.body.push(JoinInst::Compute(MirLikeInst::BinOp {
        dst: next_i,
        op: BinOpKind::Add,
        lhs: i_loop,
        rhs: const_one,
    }));

    // next_acc = acc + 1 （body 長カウントの簡略表現）
    loop_func.body.push(JoinInst::Compute(MirLikeInst::BinOp {
        dst: next_acc,
        op: BinOpKind::Add,
        lhs: acc_loop,
        rhs: const_one,
    }));

    // loop_step(..., next_acc, next_i)
    loop_func.body.push(JoinInst::Call {
        func: loop_step_id,
        args: vec![src_loop, args_loop, n_loop, next_acc, next_i],
        k_next: None,
        dst: None,
    });

    join_module.add_function(loop_func);

    if crate::config::env::joinir_dev::debug_enabled() {
        get_global_ring0()
            .log
            .debug("[joinir/stageb_body/build] ✅ JoinIR construction completed");
    }
    Some(join_module)
}
