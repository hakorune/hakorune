//! Phase 26-H: JoinIrMin.main/0 専用の MIR → JoinIR 変換
//!
//! 目的: apps/tests/joinir_min_loop.hako の MIR を JoinIR に変換する最小実装
//!
//! 期待される変換:
//! ```text
//! // MIR (元):
//! static box JoinIrMin {
//!   main() {
//!     local i = 0
//!     loop(i < 3) {
//!       if i >= 2 { break }
//!       i = i + 1
//!     }
//!     return i
//!   }
//! }
//!
//! // JoinIR (変換後):
//! fn main(k_exit) {
//!     let i_init = 0
//!     loop_step(i_init, k_exit)
//! }
//!
//! fn loop_step(i, k_exit) {
//!     if i >= 2 {
//!         k_exit(i)  // break
//!     } else {
//!         loop_step(i + 1, k_exit)  // continue
//!     }
//! }
//! ```

use crate::mir::join_ir::{
    BinOpKind, CompareOp, ConstValue, JoinFuncId, JoinFunction, JoinInst, JoinModule, MirLikeInst,
};
use crate::mir::ValueId;
use crate::runtime::get_global_ring0;

pub fn lower_min_loop_to_joinir(module: &crate::mir::MirModule) -> Option<JoinModule> {
    // Step 1: "JoinIrMin.main/0" を探す
    let target_func = module.functions.get("JoinIrMin.main/0")?;

    if crate::config::env::joinir_dev::debug_enabled() {
        let ring0 = get_global_ring0();
        ring0.log.debug("[joinir/lower] Found JoinIrMin.main/0");
        ring0
            .log
            .debug(&format!("[joinir/lower] MIR blocks: {}", target_func.blocks.len()));
    }

    // Step 2: JoinModule を構築
    let mut join_module = JoinModule::new();

    // Phase 26-H: 最小実装として、固定的な JoinIR を生成
    // （実際の MIR 解析は Phase 27 以降）

    // main 関数: i_init = 0, loop_step(0, k_exit)
    let main_id = JoinFuncId::new(0);
    let mut main_func = JoinFunction::new(main_id, "main".to_string(), vec![]);

    let i_init = ValueId(1000); // 固定 ValueId
    let const_1 = ValueId(1002);
    let const_2 = ValueId(1003);

    // const 0
    main_func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: i_init,
        value: ConstValue::Integer(0),
    }));

    // loop_step(i_init, k_exit)
    let loop_step_id = JoinFuncId::new(1);
    main_func.body.push(JoinInst::Call {
        func: loop_step_id,
        args: vec![i_init],
        k_next: None, // main は直接 loop_step を呼ぶ
        dst: None,
    });

    join_module.add_function(main_func);

    // loop_step 関数: if i >= 2 { ret i } else { loop_step(i+1) }
    let mut loop_step_func = JoinFunction::new(
        loop_step_id,
        "loop_step".to_string(),
        vec![ValueId(2000)], // i パラメータ
    );

    let i_param = ValueId(2000);
    let cmp_result = ValueId(2001);
    let i_plus_1 = ValueId(2002);

    // const 2 (for comparison)
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Const {
            dst: const_2,
            value: ConstValue::Integer(2),
        }));

    // cmp_result = (i >= 2)
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Compare {
            dst: cmp_result,
            op: CompareOp::Ge,
            lhs: i_param,
            rhs: const_2,
        }));

    // if cmp_result { ret i } else { loop_step(i+1) }
    // Phase 26-H 簡略化: 分岐はせず両方の経路を示す

    // ret i (break path)
    loop_step_func.body.push(JoinInst::Ret {
        value: Some(i_param),
    });

    // const 1 (for increment)
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
            lhs: i_param,
            rhs: const_1,
        }));

    // loop_step(i + 1) (continue path)
    loop_step_func.body.push(JoinInst::Call {
        func: loop_step_id,
        args: vec![i_plus_1],
        k_next: None,
        dst: None,
    });

    join_module.add_function(loop_step_func);

    if crate::config::env::joinir_dev::debug_enabled() {
        get_global_ring0().log.debug(&format!(
            "[joinir/lower] Generated {} JoinIR functions",
            join_module.functions.len()
        ));
    }

    Some(join_module)
}
