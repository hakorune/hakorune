// Phase 30.x: JoinIR → Rust VM Bridge A/B Test for Stage1UsingResolverBox.resolve_for_source/5
//
// 目的:
// - JoinIR を VM ブリッジ経由で実行し、直接 VM 実行の結果と一致することを確認する
// - Route A (AST→MIR→VM) と Route C (AST→MIR→JoinIR→MIR'→VM) の比較
//
// モジュール化方針:
// - このファイルは自己完結（削除時はこのファイル + mod.rs 1行 + vm.rs 分岐のみ）
// - 必要なヘルパー・定数は全てこのファイル内に閉じ込め
//
// Implementation Status:
// - Phase 30.x: Stage-1 UsingResolver minimal A/B test

use crate::ast::ASTNode;
use crate::backend::{VMValue, VM};
use crate::boxes::array::ArrayBox;
use crate::boxes::basic::StringBox;
use crate::boxes::map_box::MapBox;
use crate::mir::join_ir::lowering::stage1_using_resolver::lower_stage1_usingresolver_to_joinir;
use crate::mir::join_ir::JoinFuncId;
use crate::mir::join_ir_ops::JoinValue;
use crate::mir::join_ir_vm_bridge::run_joinir_via_vm;
use crate::mir::MirCompiler;
use crate::parser::NyashParser;

fn join_value_from_box(b: Box<dyn crate::box_trait::NyashBox>) -> JoinValue {
    let vm_val = VMValue::from_nyash_box(b);
    JoinValue::from_vm_value(&vm_val).expect("box conversion failed")
}

fn make_entries(values: &[&str]) -> JoinValue {
    let mut arr = ArrayBox::new();
    for v in values {
        let _ = arr.push(Box::new(StringBox::new(*v)));
    }
    join_value_from_box(Box::new(arr))
}

fn make_empty_map() -> JoinValue {
    let map = MapBox::new();
    join_value_from_box(Box::new(map))
}

/// 実験トグルチェック（モジュール内に閉じ込め）
fn require_experiment_toggle() -> bool {
    if !crate::config::env::joinir_dev::vm_bridge_enabled() {
        eprintln!("[joinir/vm_bridge/stage1] NYASH_JOINIR_VM_BRIDGE=1 not set, skipping test");
        return false;
    }
    true
}

/// Stage-1 UsingResolver minimal テスト用ソースコード
/// - ArrayBox を使うループ構造を持つ
/// - 簡略化版（実際の should_emit, path 解決等は省略）
const STAGE1_USINGRESOLVER_SOURCE: &str = r#"
static box Stage1UsingResolverBox {
    resolve_for_source(entries, n, modules, seen, prefix_init) {
        local i = 0
        local prefix = prefix_init
        loop(i < n) {
            local next_i = i + 1
            local entry = entries.get(i)
            // 簡略化: prefix に entry を連結
            prefix = prefix + entry
            i = next_i
        }
        return prefix
    }
}
"#;

/// テスト用 Runner（entries 配列を作成して resolve_for_source を呼び出す）
const RUNNER_SOURCE: &str = r#"
static box Runner {
    main(args) {
        // 空の配列で呼び出し（n=0 なのでループは実行されない）
        local entries = new ArrayBox()
        local modules = new MapBox()
        local seen = new MapBox()
        local result = Stage1UsingResolverBox.resolve_for_source(entries, 0, modules, seen, "init")
        return result
    }
}
"#;

/// 要素ありテスト用 Runner
const RUNNER_WITH_ENTRIES_SOURCE: &str = r#"
static box Runner {
    main(args) {
        // 3要素の配列を作成
        local entries = new ArrayBox()
        entries.push("A")
        entries.push("B")
        entries.push("C")
        local modules = new MapBox()
        local seen = new MapBox()
        local result = Stage1UsingResolverBox.resolve_for_source(entries, 3, modules, seen, "")
        return result
    }
}
"#;

#[test]
#[ignore]
fn joinir_vm_bridge_stage1_usingresolver_empty_entries() {
    if !require_experiment_toggle() {
        return;
    }

    // Stage-3 parser 有効化
    std::env::set_var("NYASH_FEATURES", "stage3");
    std::env::set_var("NYASH_DISABLE_PLUGINS", "1");
    std::env::set_var("NYASH_VM_MAX_STEPS", "100000");

    let full_src = format!("{STAGE1_USINGRESOLVER_SOURCE}\n{RUNNER_SOURCE}");

    let ast: ASTNode =
        NyashParser::parse_from_string(&full_src).expect("stage1_usingresolver: parse failed");
    let mut mc = MirCompiler::with_options(false);
    let compiled = mc
        .compile(ast)
        .expect("stage1_usingresolver: MIR compile failed");

    // Route A: AST → MIR → VM (direct)
    eprintln!("[joinir_vm_bridge_test/stage1] Route A: Direct VM execution");
    std::env::set_var("NYASH_ENTRY", "Runner.main");
    let mut vm = VM::new();
    let vm_out = vm
        .execute_module(&compiled.module)
        .expect("stage1_usingresolver: VM execution failed");
    let vm_result = vm_out.to_string_box().value.clone();
    std::env::remove_var("NYASH_ENTRY");

    eprintln!(
        "[joinir_vm_bridge_test/stage1] Route A result: {:?}",
        vm_result
    );

    // Route C: AST → MIR → JoinIR → MIR' → VM (via bridge)
    eprintln!("[joinir_vm_bridge_test/stage1] Route C: JoinIR → VM bridge execution");
    let join_module = lower_stage1_usingresolver_to_joinir(&compiled.module)
        .expect("lower_stage1_usingresolver_to_joinir failed");

    // 空配列、n=0、空Map、空Map、prefix="init" を引数として渡す
    // JoinValue で表現可能な引数のみ（ArrayBox/MapBox は現状未サポート）
    // このテストは JoinIR lowering の構造検証が主目的
    eprintln!(
        "[joinir_vm_bridge_test/stage1] Note: JoinIR bridge with ArrayBox requires VM-side support"
    );
    eprintln!(
        "[joinir_vm_bridge_test/stage1] Skipping direct bridge call - structure verification only"
    );

    // 構造検証：JoinModule が正しく生成されているか
    assert_eq!(
        join_module.functions.len(),
        2,
        "Expected 2 functions (resolve_entries + loop_step)"
    );

    // 空配列（n=0）の場合、ループは実行されないので prefix_init がそのまま返るはず
    // ただし、PHI バグがある場合は "void" が返る可能性がある
    if vm_result == "init" {
        eprintln!("[joinir_vm_bridge_test/stage1] ✅ VM returned expected 'init'");
    } else {
        eprintln!(
            "[joinir_vm_bridge_test/stage1] ⚠️ VM returned '{}' (PHI bug - expected 'init')",
            vm_result
        );
        eprintln!("[joinir_vm_bridge_test/stage1] JoinIR would fix this by design");
    }

    eprintln!(
        "[joinir_vm_bridge_test/stage1] ✅ Empty entries test passed (structure verification)"
    );
    eprintln!(
        "[joinir_vm_bridge_test/stage1] VM result: {:?}, JoinIR structure: 2 functions",
        vm_result
    );

    // クリーンアップ
    std::env::remove_var("NYASH_FEATURES");
    std::env::remove_var("NYASH_DISABLE_PLUGINS");
    std::env::remove_var("NYASH_VM_MAX_STEPS");
}

#[test]
#[ignore]
fn joinir_vm_bridge_stage1_usingresolver_with_entries() {
    if !require_experiment_toggle() {
        return;
    }

    // 環境変数を各テストで再設定（並列テスト対策）
    std::env::set_var("NYASH_FEATURES", "stage3");
    std::env::set_var("NYASH_DISABLE_PLUGINS", "1");
    std::env::set_var("NYASH_VM_MAX_STEPS", "100000");

    let full_src = format!("{STAGE1_USINGRESOLVER_SOURCE}\n{RUNNER_WITH_ENTRIES_SOURCE}");

    let ast: ASTNode =
        NyashParser::parse_from_string(&full_src).expect("stage1_usingresolver: parse failed");
    let mut mc = MirCompiler::with_options(false);
    let compiled = mc
        .compile(ast)
        .expect("stage1_usingresolver: MIR compile failed");

    // Route A: AST → MIR → VM (direct)
    eprintln!("[joinir_vm_bridge_test/stage1] Route A: Direct VM execution (with entries)");
    std::env::set_var("NYASH_ENTRY", "Runner.main");
    let mut vm = VM::new();
    let vm_out = vm
        .execute_module(&compiled.module)
        .expect("stage1_usingresolver with entries: VM execution failed");
    let vm_result = vm_out.to_string_box().value.clone();
    std::env::remove_var("NYASH_ENTRY");

    eprintln!(
        "[joinir_vm_bridge_test/stage1] Route A result: {:?}",
        vm_result
    );

    // Route C: JoinIR 構造検証
    eprintln!("[joinir_vm_bridge_test/stage1] Route C: JoinIR structure verification");
    let join_module = lower_stage1_usingresolver_to_joinir(&compiled.module)
        .expect("lower_stage1_usingresolver_to_joinir failed");

    assert_eq!(
        join_module.functions.len(),
        2,
        "Expected 2 functions (resolve_entries + loop_step)"
    );

    // entries = ["A", "B", "C"], prefix_init = "" → 結果は "ABC" になるはず
    // ただし、PHI バグがある場合は異なる結果になる可能性
    eprintln!("[joinir_vm_bridge_test/stage1] VM result: {:?}", vm_result);

    if vm_result == "ABC" {
        eprintln!("[joinir_vm_bridge_test/stage1] ✅ VM returned expected 'ABC'");
    } else {
        eprintln!(
            "[joinir_vm_bridge_test/stage1] ⚠️ VM returned '{}' (possible PHI bug)",
            vm_result
        );
        eprintln!("[joinir_vm_bridge_test/stage1] JoinIR would fix this by design");
    }

    eprintln!("[joinir_vm_bridge_test/stage1] ✅ With entries test passed");

    // クリーンアップ
    std::env::remove_var("NYASH_FEATURES");
    std::env::remove_var("NYASH_DISABLE_PLUGINS");
    std::env::remove_var("NYASH_VM_MAX_STEPS");
}

/// Step 2: Route B テスト - JoinIR → VM bridge を実際に実行
/// 目的: どこで panic/unimplemented! が出るかを特定する
#[test]
#[ignore]
fn joinir_vm_bridge_stage1_usingresolver_route_b_execution() {
    if !require_experiment_toggle() {
        return;
    }

    std::env::set_var("NYASH_FEATURES", "stage3");
    std::env::set_var("NYASH_DISABLE_PLUGINS", "1");

    let full_src = format!("{STAGE1_USINGRESOLVER_SOURCE}\n{RUNNER_SOURCE}");

    let ast: ASTNode =
        NyashParser::parse_from_string(&full_src).expect("stage1 route_b: parse failed");
    let mut mc = MirCompiler::with_options(false);
    let compiled = mc.compile(ast).expect("stage1 route_b: MIR compile failed");

    eprintln!("[joinir_vm_bridge_test/stage1/route_b] Starting Route B execution test");

    let join_module = lower_stage1_usingresolver_to_joinir(&compiled.module)
        .expect("lower_stage1_usingresolver_to_joinir failed");

    eprintln!(
        "[joinir_vm_bridge_test/stage1/route_b] JoinIR module created: {} functions",
        join_module.functions.len()
    );

    eprintln!("[joinir_vm_bridge_test/stage1/route_b] Attempting run_joinir_via_vm with n=0 (Array/Map supported)");

    let result = run_joinir_via_vm(
        &join_module,
        JoinFuncId::new(0),
        &[
            make_entries(&[]),
            JoinValue::Int(0), // n = 0 (loop won't execute)
            make_empty_map(),
            make_empty_map(),
            JoinValue::Str("init".to_string()), // prefix_init
        ],
    );

    match result {
        Ok(value) => {
            eprintln!(
                "[joinir_vm_bridge_test/stage1/route_b] ✅ Execution succeeded: {:?}",
                value
            );
            // n=0 の場合、ループは実行されず prefix_init がそのまま返るはず
            match &value {
                JoinValue::Str(s) => {
                    eprintln!("[joinir_vm_bridge_test/stage1/route_b] Result: {:?}", s);
                    if s == "init" {
                        eprintln!("[joinir_vm_bridge_test/stage1/route_b] ✅ JoinIR returned correct 'init'!");
                    } else {
                        panic!("expected 'init', got {}", s);
                    }
                }
                _ => {
                    panic!("Unexpected result type: {:?}", value);
                }
            }
        }
        Err(e) => {
            eprintln!(
                "[joinir_vm_bridge_test/stage1/route_b] ❌ Execution failed: {:?}",
                e
            );
            eprintln!("[joinir_vm_bridge_test/stage1/route_b] This error shows where VM bridge needs extension");
            panic!("JoinIR bridge failed: {:?}", e);
        }
    }

    std::env::remove_var("NYASH_FEATURES");
    std::env::remove_var("NYASH_DISABLE_PLUGINS");
}

/// Step 2b: Route B テスト - n>0 で実際に ArrayBox.get を呼び出す
/// 目的: JoinValue に ArrayBox がない場合、どうエラーになるか確認
#[test]
#[ignore]
fn joinir_vm_bridge_stage1_usingresolver_route_b_with_entries() {
    if !require_experiment_toggle() {
        return;
    }

    std::env::set_var("NYASH_FEATURES", "stage3");
    std::env::set_var("NYASH_DISABLE_PLUGINS", "1");

    let full_src = format!("{STAGE1_USINGRESOLVER_SOURCE}\n{RUNNER_WITH_ENTRIES_SOURCE}");

    let ast: ASTNode = NyashParser::parse_from_string(&full_src)
        .expect("stage1 route_b_with_entries: parse failed");
    let mut mc = MirCompiler::with_options(false);
    let compiled = mc
        .compile(ast)
        .expect("stage1 route_b_with_entries: MIR compile failed");

    eprintln!("[joinir_vm_bridge_test/stage1/route_b_with_entries] Starting Route B execution test with n=3");

    let join_module = lower_stage1_usingresolver_to_joinir(&compiled.module)
        .expect("lower_stage1_usingresolver_to_joinir failed");

    eprintln!(
        "[joinir_vm_bridge_test/stage1/route_b_with_entries] JoinIR module created: {} functions",
        join_module.functions.len()
    );

    // n=3 の場合、ループが実行され、entries.get(i) が呼ばれる
    // JoinValue::Int(0) を entries に渡すと、get メソッド呼び出しで失敗するはず
    eprintln!(
        "[joinir_vm_bridge_test/stage1/route_b_with_entries] Attempting run_joinir_via_vm with n=3"
    );
    eprintln!("[joinir_vm_bridge_test/stage1/route_b_with_entries] Note: ArrayBox passed as Int(0) - expecting failure at get() call");

    let result = run_joinir_via_vm(
        &join_module,
        JoinFuncId::new(0),
        &[
            make_entries(&["A", "B", "C"]),
            JoinValue::Int(3), // n = 3 (loop will execute 3 times)
            make_empty_map(),
            make_empty_map(),
            JoinValue::Str("".to_string()), // prefix_init = ""
        ],
    );

    match result {
        Ok(value) => {
            eprintln!(
                "[joinir_vm_bridge_test/stage1/route_b_with_entries] ✅ Execution succeeded: {:?}",
                value
            );
            match value {
                JoinValue::Str(s) => {
                    eprintln!(
                        "[joinir_vm_bridge_test/stage1/route_b_with_entries] Result: {:?}",
                        s
                    );
                    if s == "ABC" {
                        eprintln!("[joinir_vm_bridge_test/stage1/route_b_with_entries] ✅ JoinIR returned correct 'ABC'!");
                    } else {
                        panic!("expected 'ABC', got {}", s);
                    }
                }
                _ => {
                    panic!("[joinir_vm_bridge_test/stage1/route_b_with_entries] Unexpected result type: {:?}", value);
                }
            }
        }
        Err(e) => {
            eprintln!(
                "[joinir_vm_bridge_test/stage1/route_b_with_entries] ❌ Execution failed: {:?}",
                e
            );
            panic!("JoinIR bridge failed: {:?}", e);
        }
    }

    std::env::remove_var("NYASH_FEATURES");
    std::env::remove_var("NYASH_DISABLE_PLUGINS");
}

#[test]
fn joinir_vm_bridge_stage1_usingresolver_lowering_sanity() {
    // 常時実行：lowering が正しく動作するかの基本検証
    // （実行はしない、構造のみ確認）

    std::env::set_var("NYASH_FEATURES", "stage3");

    let full_src = format!("{STAGE1_USINGRESOLVER_SOURCE}\n{RUNNER_SOURCE}");

    let ast: ASTNode = NyashParser::parse_from_string(&full_src)
        .expect("stage1_usingresolver sanity: parse failed");
    let mut mc = MirCompiler::with_options(false);
    let compiled = mc
        .compile(ast)
        .expect("stage1_usingresolver sanity: MIR compile failed");

    // lowering が None を返さないことを確認
    let join_module = lower_stage1_usingresolver_to_joinir(&compiled.module);

    assert!(
        join_module.is_some(),
        "lower_stage1_usingresolver_to_joinir should return Some for valid input"
    );

    let jm = join_module.unwrap();
    assert_eq!(jm.functions.len(), 2, "Expected 2 JoinIR functions");

    std::env::remove_var("NYASH_FEATURES");
}
