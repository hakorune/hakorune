//! Route-local JoinIR builder for `StageBFuncScannerBox.scan_all_boxes/1`.

use crate::mir::join_ir::lowering::value_id_ranges::stageb_funcscanner as vid;
use crate::mir::join_ir::{
    BinOpKind, CompareOp, ConstValue, JoinContId, JoinFuncId, JoinFunction, JoinInst, JoinModule,
    LoopExitShape, LoopHeaderShape, MirLikeInst,
};
use crate::runtime::get_global_ring0;

/// Common JoinIR construction shared by MIR-probed and handwritten routes.
pub(super) fn build_stageb_funcscanner_joinir(
    module: &crate::mir::MirModule,
) -> Option<JoinModule> {
    // ターゲット関数が無ければ None でフォールバック
    let _target = module
        .functions
        .get("StageBFuncScannerBox.scan_all_boxes/1")?;

    if crate::config::env::joinir_dev::debug_enabled() {
        let ring0 = get_global_ring0();
        ring0
            .log
            .debug("[joinir/stageb_funcscanner/build] Generating JoinIR for scan_all_boxes");
        ring0
            .log
            .debug("[joinir/stageb_funcscanner/build] Using ValueId range: 13000-14999");
    }

    let mut join_module = JoinModule::new();

    // Entry: scan_all_boxes(src)
    let entry_id = JoinFuncId::new(0);
    let src_param = vid::entry(0); // 13000
    let n_val = vid::entry(1); // 13001
    let defs_init = vid::entry(2); // 13002
    let i_init = vid::entry(3); // 13003
    let loop_step_id = JoinFuncId::new(1);

    let mut entry_func = JoinFunction::new(entry_id, "scan_all_boxes".to_string(), vec![src_param]);

    // n = src.length()
    entry_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BoxCall {
            dst: Some(n_val),
            box_name: "StringBox".to_string(),
            method: "length".to_string(),
            args: vec![src_param],
        }));

    // defs_init = 0 (プレースホルダ: defs 配列長)
    entry_func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: defs_init,
        value: ConstValue::Integer(0),
    }));

    // i_init = 0
    entry_func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: i_init,
        value: ConstValue::Integer(0),
    }));

    // loop_step(src, n, defs_init, i_init)
    entry_func.body.push(JoinInst::Call {
        func: loop_step_id,
        args: vec![src_param, n_val, defs_init, i_init],
        k_next: None,
        dst: None,
    });

    join_module.entry = Some(entry_id);
    join_module.add_function(entry_func);

    // Loop step: loop_step(src, n, defs, i)
    let src_loop = vid::loop_step(0); // 14000 (Pinned)
    let n_loop = vid::loop_step(1); // 14001 (Pinned)
    let defs_loop = vid::loop_step(2); // 14002 (Carrier)
    let i_loop = vid::loop_step(3); // 14003 (Carrier)

    let _header_shape = LoopHeaderShape::new_manual(
        vec![src_loop, n_loop],  // Pinned
        vec![defs_loop, i_loop], // Carrier
    );

    let mut loop_func = JoinFunction::new(
        loop_step_id,
        "loop_step".to_string(),
        vec![src_loop, n_loop, defs_loop, i_loop],
    );

    let cmp_exit = vid::loop_step(10); // 14010
    let const_one = vid::loop_step(11); // 14011
    let next_i = vid::loop_step(12); // 14012

    // cmp_exit = (i >= n)
    loop_func.body.push(JoinInst::Compute(MirLikeInst::Compare {
        dst: cmp_exit,
        op: CompareOp::Ge,
        lhs: i_loop,
        rhs: n_loop,
    }));

    // Exit shape: defs を返す
    let _exit_shape = LoopExitShape::new_manual(vec![defs_loop]);

    // if i >= n { return defs }
    loop_func.body.push(JoinInst::Jump {
        cont: JoinContId::new(0),
        args: vec![defs_loop],
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

    // loop_step(..., defs_loop, next_i)
    loop_func.body.push(JoinInst::Call {
        func: loop_step_id,
        args: vec![src_loop, n_loop, defs_loop, next_i],
        k_next: None,
        dst: None,
    });

    join_module.add_function(loop_func);

    if crate::config::env::joinir_dev::debug_enabled() {
        get_global_ring0()
            .log
            .debug("[joinir/stageb_funcscanner/build] ✅ JoinIR construction completed");
    }
    Some(join_module)
}
