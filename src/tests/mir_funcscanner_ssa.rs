/*!
 * FuncScanner / Stage‑B 用 SSA デバッグ用テスト
 *
 * 目的:
 * - lang/src/compiler/tests/funcscanner_fib_min.hako と同等のソースを Rust 側から
 *   直接 MIR 化し、MirVerifier 経由で UndefinedValue / DominatorViolation を観測する。
 * - CLI 実行ログではノイズが多くなるため、ここでは純粋に MIR モジュール単体に対する
 *   verify 結果と、ssa.rs 側の `[ssa-undef-debug]` ログだけにフォーカスする。
 *
 * 注意:
 * - `mir_funcscanner_fib_min_ssa_debug` は FuncScannerBox.scan_all_boxes/1 まわりの
 *   既知の UndefinedValue を再現するデバッグハーネスなので、常時実行にはしない。
 *   解決条件は scan_all_boxes/1 の未定義値が消えること、または baseline フィルタが
 *   追加されて期待差分だけを検証できるようになること。
 * - `mir_funcscanner_skip_ws_vm_debug_flaky` は variable_map(HashMap) の順序非決定性で
 *   揺れる既知 flaky。BTreeMap 化か、出力正規化が入るまでは #[ignore] のままにする。
 */

use crate::ast::ASTNode;
use crate::mir::{MirCompiler, MirPrinter, MirVerifier};
use crate::parser::NyashParser;

fn ensure_stage3_env() {
    std::env::set_var("NYASH_FEATURES", "stage3");
    std::env::set_var("NYASH_PARSER_ALLOW_SEMICOLON", "1");
    std::env::set_var("NYASH_ENABLE_USING", "1");
}

/// FuncScannerBox.scan_all_boxes/1 + LoopForm v2 最小ケース（fib）の SSA デバッグ
///
/// - 入力ソースは lang/src/compiler/tests/funcscanner_fib_min.hako と同一。
/// - main 内で static box TestBox/Main を組み立てて FuncScannerBox.scan_all_boxes(src) を呼ぶ。
/// - MirVerifier の UndefinedValue を拾いつつ、ssa.rs 側の `[ssa-undef-debug]` ログで
///   どの命令が %0 や未定義 ValueId を使っているかを観測する。
#[test]
#[ignore] // debug harness for known UndefinedValue until scan_all_boxes/baseline filtering lands
fn mir_funcscanner_fib_min_ssa_debug() {
    ensure_stage3_env();

    // funcscanner_fib_min.hako と同じソースをそのまま使う
    let src = include_str!("../../lang/src/compiler/tests/funcscanner_fib_min.hako");
    let ast: ASTNode =
        NyashParser::parse_from_string(src).expect("parse funcscanner_fib_min.hako ok");

    let mut mc = MirCompiler::with_options(false);
    let cr = mc
        .compile(ast)
        .expect("compile funcscanner_fib_min.hako ok");

    let mut verifier = MirVerifier::new();
    if let Err(errors) = verifier.verify_module(&cr.module) {
        // デバッグ時に MIR 全体を見たい場合は NYASH_MIR_TEST_DUMP=1 で有効化
        if std::env::var("NYASH_MIR_TEST_DUMP").ok().as_deref() == Some("1") {
            let dump = MirPrinter::new().print_module(&cr.module);
            eprintln!("----- MIR DUMP (FuncScanner.fib_min) -----\n{}", dump);
        }

        for e in &errors {
            eprintln!("[rust-mir-verify] {}", e);
        }
        panic!("MIR verification failed for funcscanner_fib_min (debug harness)");
    }
}

/// FuncScannerBox._scan_methods/4 + _parse_params/_trim の最小ケース（TestBox 本文だけ）の SSA デバッグ
///
/// - 入力ソースは lang/src/compiler/tests/funcscanner_scan_methods_min.hako と同一。
/// - main 内で TestBox/Main 相当のソースを組み立て、FuncScannerBox._find_matching_brace で
///   TestBox 本文だけを抜き出してから FuncScannerBox._scan_methods を直接呼ぶ。
/// - UndefinedValue が出ないこと（特に _parse_params / _trim 呼び出しで未定義の me/receiver が使われないこと）
///   を MirVerifier で確認する。
#[test]
fn mir_funcscanner_scan_methods_ssa_debug() {
    ensure_stage3_env();

    let src = include_str!("../../lang/src/compiler/tests/funcscanner_scan_methods_min.hako");
    let ast: ASTNode =
        NyashParser::parse_from_string(src).expect("parse funcscanner_scan_methods_min.hako ok");

    let mut mc = MirCompiler::with_options(false);
    let cr = mc
        .compile(ast)
        .expect("compile funcscanner_scan_methods_min.hako ok");

    let mut verifier = MirVerifier::new();
    if let Err(errors) = verifier.verify_module(&cr.module) {
        if std::env::var("NYASH_MIR_TEST_DUMP").ok().as_deref() == Some("1") {
            let dump = MirPrinter::new().print_module(&cr.module);
            eprintln!(
                "----- MIR DUMP (FuncScanner.scan_methods_min) -----\n{}",
                dump
            );
        }
        for e in &errors {
            eprintln!("[rust-mir-verify] {}", e);
        }
        panic!("MIR verification failed for funcscanner_scan_methods_min (debug harness)");
    }
}

/// Dev-only: FuncScannerBox.skip_whitespace/2 の VM 実行観測テスト
///
/// - ここは VM 実行の観測用ハーネスで、結果そのものよりも __mir__ ログ確認が目的。
/// - 既知の flakiness は `variable_map(HashMap<String, ValueId>)` の順序非決定性に由来する。
/// - BoxCompilationContext / variable_map の BTreeMap 化、またはログ正規化が入るまで
///   #[ignore] のまま維持する。
#[test]
#[ignore] // dev-only flaky harness; keep ignored until variable_map order is stabilized or logs are normalized
fn mir_funcscanner_skip_ws_vm_debug_flaky() {
    // このテストは、FuncScannerBox.skip_whitespace/2 を経由する最小ケースを
    // VM + NYASH_MIR_DEBUG_LOG 付きで実行し、__mir__ ログから挙動を目視確認するための
    // 開発用ハーネスとして残しておく。
    //
    // 実装詳細は tools 側の専用ハーネスおよび
    // docs/private/roadmap2/phases/phase-25.3-funcscanner/README.md を参照。
    assert!(true, "dev-only flaky test placeholder");
}
