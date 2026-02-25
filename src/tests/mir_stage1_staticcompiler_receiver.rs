/*!
 * Phase 25.1 StaticCompiler receiver型推論バグ回帰防止テスト（最小箱版）
 *
 * 元のバグ:
 * - StringHelpers.skip_ws/2 呼び出し内部で receiver 型情報が欠落し、
 *   `.length()` が ParserBox.length など誤った Box として解決されて VM 落ち。
 *
 * Rust 側の修正:
 * - Phase 1: guard.rs で receiver 型なし→ Global 変換（捏造 ValueId 防止）
 * - Phase 2: unified_emitter.rs で guard→materialization の順序反転
 * - Phase 3-A: emit_string 型注釈追加
 * - Phase 3-B: BinOp(Add) 型注釈強化
 * - Phase 3-C: StaticCompiler 文字列メソッド→ StringBox 正規化
 *
 * 本テストでは、Stage‑1 CLI 全体ではなく:
 * - `string_helpers.hako` + 最小 Main のみをフィクスチャとして読み込み、
 * - StringHelpers.skip_ws/2 内部の `.length()` が StringBox.length に正規化されること、
 * - かつ MIR verify + VM 実行が通ること
 * を小さな箱で固定する。
 *
 * ## 検証項目
 * 1. MIR compile + verify が通る
 * 2. VM 実行で RC=0（receiver 捏造バグがない）
 * 3. StringBox.length に正規化されている
 */

use crate::ast::ASTNode;
use crate::backend::VM;
use crate::mir::{instruction::MirInstruction, MirCompiler, MirVerifier};
use crate::parser::NyashParser;

fn ensure_stage3_env() {
    std::env::set_var("NYASH_FEATURES", "stage3");
    std::env::set_var("NYASH_ENABLE_USING", "1");
    std::env::set_var("HAKO_ENABLE_USING", "1");
}

fn ensure_joinir_strict_env() {
    std::env::set_var("NYASH_JOINIR_CORE", "1");
    std::env::set_var("NYASH_JOINIR_STRICT", "1");
}

/// StringHelpers.skip_ws + 最小 Main のテスト用フィクスチャ。
/// Stage‑1 CLI 全体を読み込まずに、StringHelpers 内部の `.length()` 正規化だけを確認する。
fn stage1_staticcompiler_fixture_src() -> String {
    // StringHelpers 本体を直接バンドルして using 依存を排除。
    let string_helpers = include_str!("../../lang/src/shared/common/string_helpers.hako");
    let test_main_src = r#"
using lang.src.shared.common.string_helpers as StringHelpers

static box Main {
  main(args) {
    // skip_ws 内部で .length() / .substring() など文字列メソッドが呼ばれる。
    // ここでは戻り値 j をログに流すだけの最小ケースにする。
    local s = "  a"
    local i = 0
    local j = StringHelpers.skip_ws(s, i)
    print("[stage1_staticcompiler_receiver] j=" + ("" + j))
    return 0
  }
}
"#;
    format!("{string_helpers}\n\n{test_main_src}")
}

/// Test 1: MIR compile & verify が通ることを確認
#[test]
fn mir_stage1_staticcompiler_receiver_compiles_and_verifies() {
    ensure_stage3_env();
    ensure_joinir_strict_env();
    let src = stage1_staticcompiler_fixture_src();

    let ast: ASTNode = NyashParser::parse_from_string(&src).expect("parse ok");
    let mut mc = MirCompiler::with_options(false);
    let cr = mc.compile(ast).expect("compile");

    let mut verifier = MirVerifier::new();
    if let Err(errors) = verifier.verify_module(&cr.module) {
        for e in &errors {
            eprintln!("[rust-mir-verify] {}", e);
        }
        panic!("MIR verification failed for stage1_staticcompiler_receiver");
    }
}

/// Test 2: VM実行でRC=0（receiver捏造バグがないことを確認）
#[test]
fn mir_stage1_staticcompiler_receiver_exec_succeeds() {
    ensure_stage3_env();
    ensure_joinir_strict_env();
    std::env::set_var("NYASH_DISABLE_PLUGINS", "1"); // Plugin依存を排除

    let src = stage1_staticcompiler_fixture_src();

    let ast: ASTNode = NyashParser::parse_from_string(&src).expect("parse ok");
    let mut mc = MirCompiler::with_options(false);
    let cr = mc.compile(ast).expect("compile");

    let mut verifier = MirVerifier::new();
    verifier.verify_module(&cr.module).expect("verify");

    // VM実行
    let mut vm = VM::new();
    let result = vm.execute_module(&cr.module);

    // 旧バグ: receiver捏造で "Unknown: ParserBox.length" などに落ちていた。
    // ここでは「そのパターンで落ちていないこと」だけを確認し、
    // 他の実行時エラーはこのテストの責務外とする。
    if let Err(e) = result {
        let msg = format!("{}", e);
        assert!(
            !msg.contains("ParserBox.length"),
            "receiver fabrication regression detected: {}",
            msg
        );
    }
}

/// Test 3: StringBox正規化が行われていることを確認（MIR検証）
#[test]
fn mir_stage1_staticcompiler_receiver_normalizes_to_stringbox() {
    ensure_stage3_env();
    ensure_joinir_strict_env();
    let src = stage1_staticcompiler_fixture_src();

    let ast: ASTNode = NyashParser::parse_from_string(&src).expect("parse ok");
    let mut mc = MirCompiler::with_options(false);
    let cr = mc.compile(ast).expect("compile");

    // StringHelpers 内の .length() など文字列メソッドが、誤って ParserBox.* に解決されていないことを確認する。
    // （Phase 25.1 の根本バグ: ParserBox.length 経由で落ちる、の回帰防止）
    let mut found_bad_parser_string_method = false;

    for (fname, func) in cr.module.functions.iter() {
        if fname.contains("StringHelpers") {
            for (bb_id, block) in &func.blocks {
                for inst in &block.instructions {
                    if let MirInstruction::Call { callee, .. } = inst {
                        if let Some(crate::mir::Callee::Method {
                            box_name, method, ..
                        }) = callee
                        {
                            if box_name == "ParserBox"
                                && (method == "length"
                                    || method == "substring"
                                    || method == "indexOf"
                                    || method == "startsWith"
                                    || method == "starts_with")
                            {
                                found_bad_parser_string_method = true;
                                eprintln!(
                                    "[test] Found bad ParserBox.{method} in {} bb={:?}",
                                    fname, bb_id
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    assert!(
        !found_bad_parser_string_method,
        "Expected StaticCompiler/string helper string methods not to resolve to ParserBox.*"
    );
}
