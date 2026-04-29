//! JoinIR JSON シリアライズテスト (Phase 30.x)
//!
//! 手動構築した JoinIR を JSON に変換し、構造の妥当性を検証する。

use crate::mir::join_ir::json::join_module_to_json_string;
use crate::mir::join_ir::{
    BinOpKind, CompareOp, ConstValue, JoinContId, JoinFuncId, JoinFunction, JoinInst, JoinModule,
    MirLikeInst,
};
use crate::mir::ValueId;

/// 手動構築した JoinModule の JSON 出力テスト（NYASH_JOINIR_EXPERIMENT 不要）
#[test]
fn test_manual_joinir_json() {
    // skip_ws 相当の JoinIR を手動で構築
    let mut module = JoinModule::new();

    // skip 関数: skip(s) -> i
    let mut skip_func = JoinFunction::new(
        JoinFuncId::new(0),
        "skip".to_string(),
        vec![ValueId(3000)], // s
    );

    // i_init = 0
    skip_func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: ValueId(3001),
        value: ConstValue::Integer(0),
    }));

    // n = s.length()
    skip_func.body.push(JoinInst::Compute(MirLikeInst::BoxCall {
        dst: Some(ValueId(3002)),
        box_name: "StringBox".to_string(),
        method: "length".to_string(),
        args: vec![ValueId(3000)],
    }));

    // loop_step(s, i_init, n) - 末尾呼び出し
    skip_func.body.push(JoinInst::Call {
        func: JoinFuncId::new(1),
        args: vec![ValueId(3000), ValueId(3001), ValueId(3002)],
        k_next: None,
        dst: None,
    });

    module.add_function(skip_func);

    // loop_step 関数: loop_step(s, i, n) -> i
    let mut loop_step = JoinFunction::new(
        JoinFuncId::new(1),
        "loop_step".to_string(),
        vec![ValueId(3100), ValueId(3101), ValueId(3102)], // s, i, n
    );

    // cond = i < n
    loop_step.body.push(JoinInst::Compute(MirLikeInst::Compare {
        dst: ValueId(3103),
        op: CompareOp::Lt,
        lhs: ValueId(3101),
        rhs: ValueId(3102),
    }));

    // 条件分岐（簡略化: 条件付き ret）
    loop_step.body.push(JoinInst::Ret {
        value: Some(ValueId(3101)),
    });

    module.add_function(loop_step);
    module.entry = Some(JoinFuncId::new(0));

    // JSON に変換
    let json = join_module_to_json_string(&module);

    // 構造チェック
    assert!(json.contains("\"version\":0"));
    assert!(json.contains("\"entry\":0"));
    assert!(json.contains("\"name\":\"skip\""));
    assert!(json.contains("\"name\":\"loop_step\""));

    // 命令チェック
    assert!(json.contains("\"kind\":\"const\""));
    assert!(json.contains("\"kind\":\"boxcall\""));
    assert!(json.contains("\"kind\":\"compare\""));
    assert!(json.contains("\"type\":\"call\""));
    assert!(json.contains("\"type\":\"ret\""));

    // 値チェック
    assert!(json.contains("\"value_type\":\"integer\""));
    assert!(json.contains("\"value\":0"));
    assert!(json.contains("\"box\":\"StringBox\""));
    assert!(json.contains("\"method\":\"length\""));
    assert!(json.contains("\"op\":\"lt\""));

    eprintln!("[joinir/json] manual JoinIR JSON output:");
    eprintln!("{}", json);
}

/// 全命令タイプの JSON 出力をカバーするテスト
#[test]
fn test_all_instruction_types_json() {
    let mut module = JoinModule::new();
    let mut func = JoinFunction::new(JoinFuncId::new(0), "all_types".to_string(), vec![]);

    // Const - Integer
    func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: ValueId(1),
        value: ConstValue::Integer(42),
    }));

    // Const - Bool
    func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: ValueId(2),
        value: ConstValue::Bool(true),
    }));

    // Const - String
    func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: ValueId(3),
        value: ConstValue::String("hello".to_string()),
    }));

    // Const - Null
    func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: ValueId(4),
        value: ConstValue::Null,
    }));

    // BinOp - all types
    for (i, op) in [
        BinOpKind::Add,
        BinOpKind::Sub,
        BinOpKind::Mul,
        BinOpKind::Div,
        BinOpKind::Or,
        BinOpKind::And,
    ]
    .iter()
    .enumerate()
    {
        func.body.push(JoinInst::Compute(MirLikeInst::BinOp {
            dst: ValueId(10 + i as u32),
            op: *op,
            lhs: ValueId(1),
            rhs: ValueId(1),
        }));
    }

    // Compare - all types
    for (i, op) in [
        CompareOp::Lt,
        CompareOp::Le,
        CompareOp::Gt,
        CompareOp::Ge,
        CompareOp::Eq,
        CompareOp::Ne,
    ]
    .iter()
    .enumerate()
    {
        func.body.push(JoinInst::Compute(MirLikeInst::Compare {
            dst: ValueId(20 + i as u32),
            op: *op,
            lhs: ValueId(1),
            rhs: ValueId(1),
        }));
    }

    // BoxCall
    func.body.push(JoinInst::Compute(MirLikeInst::BoxCall {
        dst: Some(ValueId(30)),
        box_name: "TestBox".to_string(),
        method: "test_method".to_string(),
        args: vec![ValueId(1), ValueId(2)],
    }));

    // Call
    func.body.push(JoinInst::Call {
        func: JoinFuncId::new(1),
        args: vec![ValueId(1)],
        k_next: Some(JoinContId::new(2)),
        dst: Some(ValueId(40)),
    });

    // Jump (conditional)
    func.body.push(JoinInst::Jump {
        cont: JoinContId::new(3),
        args: vec![ValueId(1)],
        cond: Some(ValueId(2)),
    });

    // Jump (unconditional)
    func.body.push(JoinInst::Jump {
        cont: JoinContId::new(4),
        args: vec![],
        cond: None,
    });

    // Ret with value
    func.body.push(JoinInst::Ret {
        value: Some(ValueId(1)),
    });

    module.add_function(func);

    // JSON に変換
    let json = join_module_to_json_string(&module);

    // 全 BinOp タイプをチェック
    assert!(json.contains("\"op\":\"add\""));
    assert!(json.contains("\"op\":\"sub\""));
    assert!(json.contains("\"op\":\"mul\""));
    assert!(json.contains("\"op\":\"div\""));
    assert!(json.contains("\"op\":\"or\""));
    assert!(json.contains("\"op\":\"and\""));

    // 全 Compare タイプをチェック
    assert!(json.contains("\"op\":\"lt\""));
    assert!(json.contains("\"op\":\"le\""));
    assert!(json.contains("\"op\":\"gt\""));
    assert!(json.contains("\"op\":\"ge\""));
    assert!(json.contains("\"op\":\"eq\""));
    assert!(json.contains("\"op\":\"ne\""));

    // 全 ConstValue タイプをチェック
    assert!(json.contains("\"value_type\":\"integer\""));
    assert!(json.contains("\"value_type\":\"bool\""));
    assert!(json.contains("\"value_type\":\"string\""));
    assert!(json.contains("\"value_type\":\"null\""));

    eprintln!("[joinir/json] all_instruction_types test passed");
}

// ============================================================================
// Phase 30.x: jsonir v0 スナップショットテスト
// ============================================================================
//
// 目的:
// - 現状の JoinIR 形（pre-generic な generic_case_a / case-specific lowering）を
//   v0_ フィクスチャとして固定する
// - JoinIR/LoopScopeShape の汎用化中に意図しない退行を検知する
//
// 実行方法:
//   NYASH_JOINIR_SNAPSHOT_TEST=1 cargo test --release joinir_json_v0_ -- --nocapture
//
// フィクスチャ初期生成:
//   NYASH_JOINIR_SNAPSHOT_GENERATE=1 cargo test --release joinir_json_v0_ -- --nocapture

use crate::ast::ASTNode;
use crate::mir::join_ir::lowering::{
    lower_funcscanner_trim_to_joinir, lower_skip_ws_to_joinir,
    lower_stage1_usingresolver_to_joinir, lower_stageb_body_to_joinir,
    lower_stageb_funcscanner_to_joinir,
};
use crate::mir::MirCompiler;
use crate::parser::NyashParser;

/// フィクスチャファイルのベースディレクトリ
const FIXTURE_DIR: &str = "tests/fixtures/joinir";

/// v0 スナップショットのケース定義
#[derive(Debug, Clone, Copy)]
enum SnapshotCase {
    SkipWsMin,
    FuncscannerTrimMin,
    Stage1UsingresolverMin,
    StagebBodyMin,
    StagebFuncscannerMin,
}

impl SnapshotCase {
    fn fixture_filename(&self) -> &'static str {
        match self {
            Self::SkipWsMin => "v0_skip_ws_min.jsonir",
            Self::FuncscannerTrimMin => "v0_funcscanner_trim_min.jsonir",
            Self::Stage1UsingresolverMin => "v0_stage1_usingresolver_min.jsonir",
            Self::StagebBodyMin => "v0_stageb_body_min.jsonir",
            Self::StagebFuncscannerMin => "v0_stageb_funcscanner_min.jsonir",
        }
    }

    fn source_file(&self) -> &'static str {
        match self {
            Self::SkipWsMin => "apps/tests/minimal_ssa_skip_ws.hako",
            Self::FuncscannerTrimMin => "lang/src/compiler/tests/funcscanner_trim_min.hako",
            Self::Stage1UsingresolverMin => "apps/tests/stage1_usingresolver_minimal.hako",
            Self::StagebBodyMin => "apps/tests/stageb_body_extract_minimal.hako",
            Self::StagebFuncscannerMin => "apps/tests/stageb_funcscanner_scan_boxes_minimal.hako",
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Self::SkipWsMin => "skip_ws_min",
            Self::FuncscannerTrimMin => "funcscanner_trim_min",
            Self::Stage1UsingresolverMin => "stage1_usingresolver_min",
            Self::StagebBodyMin => "stageb_body_min",
            Self::StagebFuncscannerMin => "stageb_funcscanner_min",
        }
    }
}

/// ケースに対応する JoinIR JSON を生成
fn generate_joinir_json(case: SnapshotCase) -> Option<String> {
    // Stage-3 parser を有効化
    std::env::set_var("NYASH_FEATURES", "stage3");

    let src = std::fs::read_to_string(case.source_file()).ok()?;

    // FuncScanner.trim は FuncScannerBox の定義が必要
    let full_src = if matches!(case, SnapshotCase::FuncscannerTrimMin) {
        let func_scanner_src =
            std::fs::read_to_string("lang/src/compiler/entry/func_scanner.hako").ok()?;
        format!("{func_scanner_src}\n\n{src}")
    } else {
        src
    };

    let ast: ASTNode = NyashParser::parse_from_string(&full_src).ok()?;
    let mut mc = MirCompiler::with_options(false);
    let compiled = mc.compile(ast).ok()?;

    let join_module = match case {
        SnapshotCase::SkipWsMin => lower_skip_ws_to_joinir(&compiled.module),
        SnapshotCase::FuncscannerTrimMin => lower_funcscanner_trim_to_joinir(&compiled.module),
        SnapshotCase::Stage1UsingresolverMin => {
            lower_stage1_usingresolver_to_joinir(&compiled.module)
        }
        SnapshotCase::StagebBodyMin => lower_stageb_body_to_joinir(&compiled.module),
        SnapshotCase::StagebFuncscannerMin => lower_stageb_funcscanner_to_joinir(&compiled.module),
    }?;

    Some(join_module_to_json_string(&join_module))
}

/// スナップショット比較を実行（共通ロジック）
fn run_snapshot_test(case: SnapshotCase) {
    // トグルチェック
    if !crate::config::env::joinir_dev::snapshot_test_enabled() {
        eprintln!(
            "[joinir/snapshot] NYASH_JOINIR_SNAPSHOT_TEST=1 not set, skipping {}",
            case.name()
        );
        return;
    }

    let fixture_path = format!("{}/{}", FIXTURE_DIR, case.fixture_filename());

    // JoinIR JSON 生成
    let json = match generate_joinir_json(case) {
        Some(j) => j,
        None => {
            eprintln!(
                "[joinir/snapshot] Failed to generate JoinIR for {}, skipping",
                case.name()
            );
            return;
        }
    };

    // フィクスチャ生成モード
    if crate::config::env::joinir_dev::snapshot_generate_enabled() {
        std::fs::write(&fixture_path, &json).expect("Failed to write fixture");
        eprintln!(
            "[joinir/snapshot] Generated fixture: {} ({} bytes)",
            fixture_path,
            json.len()
        );
        return;
    }

    // フィクスチャ読み込み
    let fixture = match std::fs::read_to_string(&fixture_path) {
        Ok(f) => f,
        Err(_) => {
            eprintln!(
                "[joinir/snapshot] Fixture not found: {}\n\
                 Run with NYASH_JOINIR_SNAPSHOT_GENERATE=1 to create it.",
                fixture_path
            );
            panic!("Fixture not found: {}", fixture_path);
        }
    };

    // 比較
    if json != fixture {
        eprintln!(
            "[joinir/snapshot] MISMATCH for {}\n\
             --- actual ({} bytes) ---\n{}\n\
             --- fixture ({} bytes) ---\n{}",
            case.name(),
            json.len(),
            json,
            fixture.len(),
            fixture
        );
        panic!("jsonir v0 snapshot mismatch for {}", case.name());
    }

    eprintln!("[joinir/snapshot] {} matches fixture ✓", case.name());
}

// ============================================================================
// 個別スナップショットテスト
// ============================================================================

#[test]
fn joinir_json_v0_skip_ws_min_matches_fixture() {
    run_snapshot_test(SnapshotCase::SkipWsMin);
}

#[test]
fn joinir_json_v0_funcscanner_trim_min_matches_fixture() {
    run_snapshot_test(SnapshotCase::FuncscannerTrimMin);
}

#[test]
fn joinir_json_v0_stage1_usingresolver_min_matches_fixture() {
    run_snapshot_test(SnapshotCase::Stage1UsingresolverMin);
}

#[test]
fn joinir_json_v0_stageb_body_min_matches_fixture() {
    run_snapshot_test(SnapshotCase::StagebBodyMin);
}

#[test]
fn joinir_json_v0_stageb_funcscanner_min_matches_fixture() {
    run_snapshot_test(SnapshotCase::StagebFuncscannerMin);
}

// ============================================================================
// Phase 32 L-2.2 Step-3: JoinIR → MIR 構造テスト
// ============================================================================
//
// 目的:
// - JoinIR lowering と JoinIR→MIR 変換が Stage-B ループ構造を壊さないことを検証
// - Route B (JoinIR→MIR) の健全性チェック
//
// 実行方法:
//   cargo test --release joinir_stageb_structure -- --nocapture

use crate::mir::join_ir_vm_bridge::bridge_joinir_to_mir;

/// Stage-B Body: JoinIR lowering + JoinIR→MIR 変換の構造テスト
#[test]
fn joinir_stageb_body_structure_test() {
    // Stage-3 parser を有効化
    std::env::set_var("NYASH_FEATURES", "stage3");

    let src = match std::fs::read_to_string("apps/tests/stageb_body_extract_minimal.hako") {
        Ok(s) => s,
        Err(_) => {
            eprintln!("[joinir/stageb_body] Source file not found, skipping");
            return;
        }
    };

    let ast: ASTNode = match NyashParser::parse_from_string(&src) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("[joinir/stageb_body] Parse failed: {:?}", e);
            return;
        }
    };

    let mut mc = MirCompiler::with_options(false);
    let compiled = match mc.compile(ast) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("[joinir/stageb_body] MIR compile failed: {:?}", e);
            return;
        }
    };

    // Step 1: JoinIR lowering
    let join_module = match lower_stageb_body_to_joinir(&compiled.module) {
        Some(jm) => jm,
        None => {
            eprintln!("[joinir/stageb_body] lowering returned None, skipping");
            return;
        }
    };

    eprintln!(
        "[joinir/stageb_body] ✅ JoinIR lowering succeeded: {} functions",
        join_module.functions.len()
    );

    // 構造チェック: 2 関数（entry + loop_step）
    assert!(
        join_module.functions.len() >= 2,
        "Stage-B Body should have at least 2 JoinIR functions"
    );

    // Step 2: JoinIR → MIR 変換
    let mir_module = match bridge_joinir_to_mir(&join_module) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("[joinir/stageb_body] JoinIR→MIR conversion failed: {:?}", e);
            panic!("JoinIR→MIR conversion should succeed");
        }
    };

    eprintln!(
        "[joinir/stageb_body] ✅ JoinIR→MIR conversion succeeded: {} functions",
        mir_module.functions.len()
    );

    // 構造チェック: MIR 関数数が JoinIR 関数数と一致
    assert_eq!(
        mir_module.functions.len(),
        join_module.functions.len(),
        "MIR function count should match JoinIR function count"
    );

    // 各関数の Block 数をチェック
    for (name, func) in &mir_module.functions {
        let block_count = func.blocks.len();
        eprintln!(
            "[joinir/stageb_body] Function '{}': {} blocks",
            name, block_count
        );
        assert!(
            block_count >= 1,
            "Each function should have at least 1 block"
        );
    }

    eprintln!("[joinir/stageb_body] ✅ Structure test passed");
}

/// Stage-B FuncScanner: JoinIR lowering + JoinIR→MIR 変換の構造テスト
#[test]
fn joinir_stageb_funcscanner_structure_test() {
    // Stage-3 parser を有効化
    std::env::set_var("NYASH_FEATURES", "stage3");

    let src = match std::fs::read_to_string("apps/tests/stageb_funcscanner_scan_boxes_minimal.hako")
    {
        Ok(s) => s,
        Err(_) => {
            eprintln!("[joinir/stageb_funcscanner] Source file not found, skipping");
            return;
        }
    };

    let ast: ASTNode = match NyashParser::parse_from_string(&src) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("[joinir/stageb_funcscanner] Parse failed: {:?}", e);
            return;
        }
    };

    let mut mc = MirCompiler::with_options(false);
    let compiled = match mc.compile(ast) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("[joinir/stageb_funcscanner] MIR compile failed: {:?}", e);
            return;
        }
    };

    // Step 1: JoinIR lowering
    let join_module = match lower_stageb_funcscanner_to_joinir(&compiled.module) {
        Some(jm) => jm,
        None => {
            eprintln!("[joinir/stageb_funcscanner] lowering returned None, skipping");
            return;
        }
    };

    eprintln!(
        "[joinir/stageb_funcscanner] ✅ JoinIR lowering succeeded: {} functions",
        join_module.functions.len()
    );

    // 構造チェック: 2 関数（entry + loop_step）
    assert!(
        join_module.functions.len() >= 2,
        "Stage-B FuncScanner should have at least 2 JoinIR functions"
    );

    // Step 2: JoinIR → MIR 変換
    let mir_module = match bridge_joinir_to_mir(&join_module) {
        Ok(m) => m,
        Err(e) => {
            eprintln!(
                "[joinir/stageb_funcscanner] JoinIR→MIR conversion failed: {:?}",
                e
            );
            panic!("JoinIR→MIR conversion should succeed");
        }
    };

    eprintln!(
        "[joinir/stageb_funcscanner] ✅ JoinIR→MIR conversion succeeded: {} functions",
        mir_module.functions.len()
    );

    // 構造チェック: JoinIR の関数名は MIR に存在すること
    let mut missing = Vec::new();
    for join_func in join_module.functions.values() {
        if !mir_module.functions.contains_key(&join_func.name) {
            missing.push(join_func.name.clone());
        }
    }
    assert!(
        missing.is_empty(),
        "MIR should contain all JoinIR function names, missing={:?}",
        missing
    );

    // 各関数の Block 数をチェック（JoinIR の関数に対応するものだけ）
    for join_func in join_module.functions.values() {
        let name = &join_func.name;
        let func = mir_module
            .functions
            .get(name)
            .expect("JoinIR function should exist in MIR");
        let block_count = func.blocks.len();
        eprintln!(
            "[joinir/stageb_funcscanner] Function '{}': {} blocks",
            name, block_count
        );
        assert!(
            block_count >= 1,
            "Each function should have at least 1 block"
        );
    }

    eprintln!("[joinir/stageb_funcscanner] ✅ Structure test passed");
}
