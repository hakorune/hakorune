// mir_joinir_min.rs
// Phase 26-H: JoinIR型定義妥当性確認テスト（最小ループ）
//
// 目的:
// - JoinFunction/JoinInst の型が破綻していないか確認
// - 手書きで JoinIR を組み立ててみて、設計の妥当性をチェック
// - まだ LoopForm → JoinIR 自動変換は書かない（Phase 27以降）
//
// 実行条件:
// - デフォルトでは #[ignore] にしておいて手動実行用にする
// - 環境変数 NYASH_JOINIR_EXPERIMENT=1 で実験モード有効化

use crate::ast::ASTNode;
use crate::mir::join_ir::*;
use crate::mir::{MirCompiler, ValueId};
use crate::parser::NyashParser;
use crate::tests::helpers::joinir_env;

#[test]
#[ignore] // 手動実行用（Phase 26-H 実験段階）
fn mir_joinir_min_manual_construction() {
    // Phase 26-H スコープ: 型定義の妥当性確認のみ
    // LoopForm からの自動変換は Phase 27 以降で実装

    // 環境変数トグルチェック
    if !joinir_env::is_experiment_enabled() {
        eprintln!(
            "[joinir/min] NYASH_JOINIR_EXPERIMENT=1 not set, skipping manual construction test"
        );
        return;
    }

    // Step 1: MIR までコンパイル（既存パイプラインで）
    // Stage-3 環境変数を設定（local キーワード対応）
    std::env::set_var("NYASH_FEATURES", "stage3");

    let test_file = "apps/tests/joinir_min_loop.hako";
    let src = std::fs::read_to_string(test_file)
        .unwrap_or_else(|_| panic!("Failed to read {}", test_file));

    let ast: ASTNode = NyashParser::parse_from_string(&src).expect("joinir_min: parse failed");

    let mut mc = MirCompiler::with_options(false);
    let compiled = mc.compile(ast).expect("joinir_min: MIR compile failed");

    eprintln!(
        "[joinir/min] MIR module compiled, {} functions",
        compiled.module.functions.len()
    );

    // Step 2: 手書きで JoinIR を構築（設計の妥当性チェック）
    let mut join_module = JoinModule::new();

    // fn main(k_exit) { loop_step(0, k_exit) }
    let main_id = JoinFuncId::new(0);
    let mut main_func = JoinFunction::new(main_id, "main".to_string(), vec![]);

    // 引数: i_init = 0 (ValueId(100) とする)
    let i_init = ValueId(100);
    main_func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: i_init,
        value: ConstValue::Integer(0),
    }));

    // loop_step(i_init, k_exit)
    let loop_step_id = JoinFuncId::new(1);
    let k_exit_id = JoinContId::new(0);
    main_func.body.push(JoinInst::Call {
        func: loop_step_id,
        args: vec![i_init],
        k_next: Some(k_exit_id),
        dst: None,
    });

    join_module.add_function(main_func);

    // fn loop_step(i, k_exit) { if i >= 2 { k_exit(i) } else { loop_step(i+1, k_exit) } }
    let mut loop_step_func = JoinFunction::new(
        loop_step_id,
        "loop_step".to_string(),
        vec![ValueId(200)], // i の引数
    );

    let i_param = ValueId(200);
    let cmp_result = ValueId(201);
    let i_plus_1 = ValueId(202);

    // cmp_result = (i >= 2)
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Compare {
            dst: cmp_result,
            op: CompareOp::Ge,
            lhs: i_param,
            rhs: ValueId(203), // const 2
        }));

    // const 2
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Const {
            dst: ValueId(203),
            value: ConstValue::Integer(2),
        }));

    // if cmp_result { k_exit(i) } else { loop_step(i+1, k_exit) }
    // ここでは簡略化して Jump 命令だけ書く（実際は分岐制御が必要だが Phase 26-H では型チェックのみ）
    loop_step_func.body.push(JoinInst::Jump {
        cont: k_exit_id,
        args: vec![i_param],
        cond: Some(cmp_result),
    });

    // i_plus_1 = i + 1
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BinOp {
            dst: i_plus_1,
            op: BinOpKind::Add,
            lhs: i_param,
            rhs: ValueId(204), // const 1
        }));

    join_module.add_function(loop_step_func);

    // Step 3: Debug 出力で妥当性確認
    eprintln!("[joinir/min] JoinIR module constructed:");
    eprintln!("{:#?}", join_module);

    // アサーション（型定義が使えることを確認）
    assert_eq!(join_module.functions.len(), 2);
    assert!(join_module.functions.contains_key(&main_id));
    assert!(join_module.functions.contains_key(&loop_step_id));

    eprintln!("[joinir/min] ✅ JoinIR型定義は妥当（Phase 26-H）");
}

#[test]
#[ignore] // 手動実行用（Phase 26-H 自動変換テスト）
fn mir_joinir_min_auto_lowering() {
    // Phase 26-H Step 2: 自動変換テスト
    // MIR → JoinIR 自動変換の動作確認

    // 環境変数トグルチェック
    if !joinir_env::is_experiment_enabled() {
        eprintln!("[joinir/auto] NYASH_JOINIR_EXPERIMENT=1 not set, skipping auto-lowering test");
        return;
    }

    // Step 1: MIR までコンパイル
    std::env::set_var("NYASH_FEATURES", "stage3");

    let test_file = "apps/tests/joinir_min_loop.hako";
    let src = std::fs::read_to_string(test_file)
        .unwrap_or_else(|_| panic!("Failed to read {}", test_file));

    let ast: ASTNode = NyashParser::parse_from_string(&src).expect("joinir_min: parse failed");

    let mut mc = MirCompiler::with_options(false);
    let compiled = mc.compile(ast).expect("joinir_min: MIR compile failed");

    eprintln!(
        "[joinir/auto] MIR module compiled, {} functions",
        compiled.module.functions.len()
    );

    // Step 2: MIR → JoinIR 自動変換
    let join_module =
        lower_min_loop_to_joinir(&compiled.module).expect("lower_min_loop_to_joinir failed");

    eprintln!("[joinir/auto] JoinIR module generated:");
    eprintln!("{:#?}", join_module);

    // Step 3: 妥当性検証
    assert_eq!(
        join_module.functions.len(),
        2,
        "Expected 2 functions (main + loop_step)"
    );

    let main_id = JoinFuncId::new(0);
    let loop_step_id = JoinFuncId::new(1);

    // main 関数の検証
    let main_func = join_module
        .functions
        .get(&main_id)
        .expect("main function not found");
    assert_eq!(main_func.name, "main");
    assert_eq!(main_func.params.len(), 0, "main has no parameters");
    assert!(
        main_func.body.len() >= 2,
        "main should have at least 2 instructions (const + call)"
    );

    // loop_step 関数の検証
    let loop_step_func = join_module
        .functions
        .get(&loop_step_id)
        .expect("loop_step function not found");
    assert_eq!(loop_step_func.name, "loop_step");
    assert_eq!(
        loop_step_func.params.len(),
        1,
        "loop_step has 1 parameter (i)"
    );
    assert!(
        loop_step_func.body.len() >= 4,
        "loop_step should have multiple instructions"
    );

    eprintln!("[joinir/auto] ✅ 自動変換成功（Phase 26-H Step 2）");
}

#[test]
fn mir_joinir_min_type_sanity() {
    // Phase 26-H: 型定義の基本的なサニティチェック（常時実行）
    let func_id = JoinFuncId::new(0);
    let func = JoinFunction::new(func_id, "test".to_string(), vec![ValueId(1)]);

    assert_eq!(func.id, func_id);
    assert_eq!(func.name, "test");
    assert_eq!(func.params.len(), 1);
    assert_eq!(func.body.len(), 0);
}
