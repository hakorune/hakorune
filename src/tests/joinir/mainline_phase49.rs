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

/// Compiler entry recurses deeply enough in debug builds to sit near the
/// default 8 MiB test-thread stack boundary; run these pins on a widened
/// thread the same way the callable pipeline loop pins do.
fn run_on_test_thread(name: &str, body: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .name(name.to_owned())
        .stack_size(32 * 1024 * 1024)
        .spawn(body)
        .expect("spawn test thread")
        .join()
        .expect("test thread panicked");
}

/// Phase 49-3: JoinIR Frontend mainline パイプラインが
/// print_tokens 関数のコンパイル時にクラッシュしないことを確認
#[test]
fn phase49_joinir_mainline_pipeline_smoke() {
    run_on_test_thread("phase49_joinir_mainline_pipeline_smoke", phase49_joinir_mainline_pipeline_smoke_inner);
}

fn phase49_joinir_mainline_pipeline_smoke_inner() {
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


/// Phase 49-3.2: A/B 比較テスト - Route A (legacy) vs Route B (JoinIR)
///
/// このテストは同じソースコードを2つの経路でコンパイルし、
/// 両方が正常に完了することを確認する。


// =============================================================================
// Phase 49-4: ArrayExtBox.filter/2 Tests
// =============================================================================

/// Phase 49-4: JoinIR Frontend mainline パイプラインが
/// ArrayExtBox.filter 関数のコンパイル時にクラッシュしないことを確認
#[test]
fn phase49_joinir_array_filter_smoke() {
    run_on_test_thread("phase49_joinir_array_filter_smoke", phase49_joinir_array_filter_smoke_inner);
}

fn phase49_joinir_array_filter_smoke_inner() {
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
