// Phase 49: JoinIR Frontend Mainline Integration Test
//
// このテストは cf_loop の JoinIR Frontend mainline route が
// 正常に動作することを確認する。
//
// Phase 49-3.2 実装済み:
// - merge_joinir_mir_blocks() によるブロックマージ
// - A/B 比較テスト（Route A vs Route B）
//
// Phase 49-4 実装済み:
// - ArrayExtBox.filter/2 対応
// - HAKO_JOINIR_ARRAY_FILTER_MAIN=1 dev フラグ追加
//
// テスト方法:
// HAKO_JOINIR_PRINT_TOKENS_MAIN=1 cargo test --release joinir_mainline_phase49
// HAKO_JOINIR_ARRAY_FILTER_MAIN=1 cargo test --release phase49_joinir_array_filter

use crate::ast::ASTNode;
use crate::mir::MirCompiler;
use crate::parser::NyashParser;
use crate::tests::helpers::joinir_env::clear_joinir_flags;

/// Phase 49-3: JoinIR Frontend mainline パイプラインが
/// print_tokens 関数のコンパイル時にクラッシュしないことを確認
#[test]
fn phase49_joinir_mainline_pipeline_smoke() {
    clear_joinir_flags();
    // Phase 49 mainline route は dev フラグで制御
    std::env::set_var("HAKO_JOINIR_PRINT_TOKENS_MAIN", "1");
    std::env::set_var("NYASH_JOINIR_MAINLINE_DEBUG", "1");
    std::env::set_var("NYASH_FEATURES", "stage3");
    std::env::set_var("NYASH_DISABLE_PLUGINS", "1");

    // print_tokens を含む最小限の JsonTokenizer 定義
    let src = r#"
box JsonTokenizer {
    tokens: ArrayBox
    errors: ArrayBox

    birth() {
        me.tokens = new ArrayBox()
        me.errors = new ArrayBox()
    }

    // Phase 49 ターゲット関数
    print_tokens() {
        local i = 0
        loop(i < me.tokens.length()) {
            local token = me.tokens.get(i)
            print(token)
            i = i + 1
        }
    }
}

static box Main {
    main() {
        local t = new JsonTokenizer()
        t.print_tokens()
        return 0
    }
}
"#;

    let ast: ASTNode = NyashParser::parse_from_string(src).expect("phase49: parse failed");

    let mut mc = MirCompiler::with_options(false);
    let result = mc.compile(ast);

    // MVP: パイプラインがクラッシュしないことを確認
    // Phase 49-3.2 で実際の A/B 比較を追加
    assert!(
        result.is_ok(),
        "phase49 mainline compile should not crash: {:?}",
        result.err()
    );

    // クリーンアップ
    clear_joinir_flags();
    std::env::remove_var("NYASH_JOINIR_MAINLINE_DEBUG");
    std::env::remove_var("NYASH_FEATURES");
    std::env::remove_var("NYASH_DISABLE_PLUGINS");
}

/// Phase 49-3: dev フラグ OFF 時は従来経路を使用することを確認
#[test]
fn phase49_joinir_mainline_fallback_without_flag() {
    clear_joinir_flags();
    // dev フラグ OFF
    std::env::remove_var("HAKO_JOINIR_PRINT_TOKENS_MAIN");
    std::env::set_var("NYASH_FEATURES", "stage3");
    std::env::set_var("NYASH_DISABLE_PLUGINS", "1");

    let src = r#"
box JsonTokenizer {
    tokens: ArrayBox

    birth() {
        me.tokens = new ArrayBox()
    }

    print_tokens() {
        local i = 0
        loop(i < me.tokens.length()) {
            i = i + 1
        }
    }
}

static box Main {
    main() {
        local t = new JsonTokenizer()
        t.print_tokens()
        return 0
    }
}
"#;

    let ast: ASTNode = NyashParser::parse_from_string(src).expect("phase49 fallback: parse failed");

    let mut mc = MirCompiler::with_options(false);
    let result = mc.compile(ast);

    assert!(
        result.is_ok(),
        "phase49 fallback compile should succeed: {:?}",
        result.err()
    );

    // クリーンアップ
    clear_joinir_flags();
    std::env::remove_var("NYASH_FEATURES");
    std::env::remove_var("NYASH_DISABLE_PLUGINS");
}

/// Phase 49-3.2: A/B 比較テスト - Route A (legacy) vs Route B (JoinIR)
///
/// このテストは同じソースコードを2つの経路でコンパイルし、
/// 両方が正常に完了することを確認する。
#[test]
fn phase49_joinir_mainline_ab_comparison() {
    clear_joinir_flags();
    let src = r#"
box JsonTokenizer {
    tokens: ArrayBox

    birth() {
        me.tokens = new ArrayBox()
    }

    print_tokens() {
        local i = 0
        loop(i < me.tokens.length()) {
            i = i + 1
        }
    }
}

static box Main {
    main() {
        local t = new JsonTokenizer()
        t.print_tokens()
        return 0
    }
}
"#;

    // Route A: Legacy path (flag OFF)
    std::env::remove_var("HAKO_JOINIR_PRINT_TOKENS_MAIN");
    std::env::set_var("NYASH_FEATURES", "stage3");
    std::env::set_var("NYASH_DISABLE_PLUGINS", "1");

    let ast_a: ASTNode =
        NyashParser::parse_from_string(src).expect("phase49 A/B: parse failed (Route A)");
    let mut mc_a = MirCompiler::with_options(false);
    let result_a = mc_a.compile(ast_a);
    assert!(
        result_a.is_ok(),
        "Route A compile should succeed: {:?}",
        result_a.err()
    );
    let module_a = result_a.unwrap().module;
    let blocks_a: usize = module_a.functions.values().map(|f| f.blocks.len()).sum();

    // Route B: JoinIR Frontend path (flag ON)
    // Re-set flags to ensure they're active
    std::env::set_var("NYASH_FEATURES", "stage3");
    std::env::set_var("NYASH_DISABLE_PLUGINS", "1");
    std::env::set_var("HAKO_JOINIR_PRINT_TOKENS_MAIN", "1");

    let ast_b: ASTNode =
        NyashParser::parse_from_string(src).expect("phase49 A/B: parse failed (Route B)");
    let mut mc_b = MirCompiler::with_options(false);
    let result_b = mc_b.compile(ast_b);
    assert!(
        result_b.is_ok(),
        "Route B compile should succeed: {:?}",
        result_b.err()
    );
    let module_b = result_b.unwrap().module;
    let blocks_b: usize = module_b.functions.values().map(|f| f.blocks.len()).sum();

    // Log block counts for debugging
    eprintln!(
        "[phase49 A/B] Route A: {} total blocks, Route B: {} total blocks",
        blocks_a, blocks_b
    );

    // Both should complete successfully (main assertion is the compile succeeds)
    // Block counts may differ due to JoinIR's different structure
    // Future: Add execution comparison

    // クリーンアップ
    clear_joinir_flags();
    std::env::remove_var("NYASH_FEATURES");
    std::env::remove_var("NYASH_DISABLE_PLUGINS");
}

// =============================================================================
// Phase 49-4: ArrayExtBox.filter/2 Tests
// =============================================================================

/// Phase 49-4: JoinIR Frontend mainline パイプラインが
/// ArrayExtBox.filter 関数のコンパイル時にクラッシュしないことを確認
#[test]
fn phase49_joinir_array_filter_smoke() {
    clear_joinir_flags();
    // Phase 49-4 mainline route は dev フラグで制御
    std::env::set_var("HAKO_JOINIR_ARRAY_FILTER_MAIN", "1");
    std::env::set_var("NYASH_JOINIR_MAINLINE_DEBUG", "1");
    std::env::set_var("NYASH_FEATURES", "stage3");
    std::env::set_var("NYASH_DISABLE_PLUGINS", "1");

    // ArrayExtBox.filter 簡易実装（if-in-loop パターン）
    // fn パラメータは pred に変更（予約語競合回避）
    let src = r#"
static box ArrayExtBox {
    filter(arr, pred) {
        local out = new ArrayBox()
        local i = 0
        local n = arr.size()
        loop(i < n) {
            local v = arr.get(i)
            if pred(v) {
                out.push(v)
            }
            i = i + 1
        }
        return out
    }
}

static box Main {
    main() {
        return 0
    }
}
"#;

    let ast: ASTNode = NyashParser::parse_from_string(src).expect("phase49-4: parse failed");

    let mut mc = MirCompiler::with_options(false);
    let result = mc.compile(ast);

    // パイプラインがクラッシュしないことを確認
    assert!(
        result.is_ok(),
        "phase49-4 array_filter mainline compile should not crash: {:?}",
        result.err()
    );

    // クリーンアップ
    clear_joinir_flags();
    std::env::remove_var("NYASH_JOINIR_MAINLINE_DEBUG");
    std::env::remove_var("NYASH_FEATURES");
    std::env::remove_var("NYASH_DISABLE_PLUGINS");
}

/// Phase 49-4: dev フラグ OFF 時は従来経路を使用することを確認
#[test]
fn phase49_joinir_array_filter_fallback() {
    clear_joinir_flags();
    // dev フラグ OFF
    std::env::remove_var("HAKO_JOINIR_ARRAY_FILTER_MAIN");
    std::env::set_var("NYASH_FEATURES", "stage3");
    std::env::set_var("NYASH_DISABLE_PLUGINS", "1");

    let src = r#"
static box ArrayExtBox {
    filter(arr, pred) {
        local out = new ArrayBox()
        local i = 0
        local n = arr.size()
        loop(i < n) {
            local v = arr.get(i)
            if pred(v) {
                out.push(v)
            }
            i = i + 1
        }
        return out
    }
}

static box Main {
    main() {
        return 0
    }
}
"#;

    let ast: ASTNode =
        NyashParser::parse_from_string(src).expect("phase49-4 fallback: parse failed");

    let mut mc = MirCompiler::with_options(false);
    let result = mc.compile(ast);

    assert!(
        result.is_ok(),
        "phase49-4 fallback compile should succeed: {:?}",
        result.err()
    );

    // クリーンアップ
    clear_joinir_flags();
    std::env::remove_var("NYASH_FEATURES");
    std::env::remove_var("NYASH_DISABLE_PLUGINS");
}

/// Phase 49-4: A/B 比較テスト - Route A (legacy) vs Route B (JoinIR)
/// ArrayExtBox.filter版
#[test]
fn phase49_joinir_array_filter_ab_comparison() {
    clear_joinir_flags();
    let src = r#"
static box ArrayExtBox {
    filter(arr, pred) {
        local out = new ArrayBox()
        local i = 0
        local n = arr.size()
        loop(i < n) {
            local v = arr.get(i)
            if pred(v) {
                out.push(v)
            }
            i = i + 1
        }
        return out
    }
}

static box Main {
    main() {
        return 0
    }
}
"#;

    // Route A: Legacy path (flag OFF)
    std::env::remove_var("HAKO_JOINIR_ARRAY_FILTER_MAIN");
    std::env::set_var("NYASH_FEATURES", "stage3");
    std::env::set_var("NYASH_DISABLE_PLUGINS", "1");

    let ast_a: ASTNode =
        NyashParser::parse_from_string(src).expect("phase49-4 A/B: parse failed (Route A)");
    let mut mc_a = MirCompiler::with_options(false);
    let result_a = mc_a.compile(ast_a);
    assert!(
        result_a.is_ok(),
        "Route A compile should succeed: {:?}",
        result_a.err()
    );
    let module_a = result_a.unwrap().module;
    let blocks_a: usize = module_a.functions.values().map(|f| f.blocks.len()).sum();

    // Route B: JoinIR Frontend path (flag ON)
    std::env::set_var("NYASH_FEATURES", "stage3");
    std::env::set_var("NYASH_DISABLE_PLUGINS", "1");
    std::env::set_var("HAKO_JOINIR_ARRAY_FILTER_MAIN", "1");

    let ast_b: ASTNode =
        NyashParser::parse_from_string(src).expect("phase49-4 A/B: parse failed (Route B)");
    let mut mc_b = MirCompiler::with_options(false);
    let result_b = mc_b.compile(ast_b);
    assert!(
        result_b.is_ok(),
        "Route B compile should succeed: {:?}",
        result_b.err()
    );
    let module_b = result_b.unwrap().module;
    let blocks_b: usize = module_b.functions.values().map(|f| f.blocks.len()).sum();

    // Log block counts for debugging
    eprintln!(
        "[phase49-4 A/B filter] Route A: {} total blocks, Route B: {} total blocks",
        blocks_a, blocks_b
    );

    // クリーンアップ
    clear_joinir_flags();
    std::env::remove_var("NYASH_FEATURES");
    std::env::remove_var("NYASH_DISABLE_PLUGINS");
}
