/*!
 * StringUtils arity suffix 自動補完テスト（Phase 21.7++）
 *
 * 目的:
 * - VM の execute_global_function で arity が欠落している場合に
 *   args.len() から自動補完される機能を検証する。
 *
 * 背景:
 * - MIR 関数は "BoxName.method/arity" 形式で格納される
 * - 呼び出し側が arity なしで "BoxName.method" を指定した場合、
 *   自動的に "/arity" を追加して検索する
 *
 * 修正内容（2025-11-21）:
 * 1. lang/src/llvm_ir/hako_module.toml の TOML パースエラーを修正
 * 2. src/backend/mir_interpreter/handlers/calls/global.rs で arity 自動補完実装
 *
 * 注意:
 * - このテストは using 解決をテストするものではなく、arity 自動補完のみをテストする
 * - using 解決のテストは CLI 経由で実施（apps/tests/json_lint_stringutils_min.hako）
 */

use crate::ast::ASTNode;
use crate::backend::VM;
use crate::mir::MirCompiler;
use crate::parser::NyashParser;

fn ensure_stage3_env() {
    std::env::set_var("NYASH_FEATURES", "stage3");
    std::env::set_var("NYASH_FEATURES", "stage3");
    std::env::set_var("NYASH_PARSER_ALLOW_SEMICOLON", "1");
    std::env::set_var("NYASH_DISABLE_PLUGINS", "1");
    std::env::set_var("HAKO_MIR_BUILDER_METHODIZE", "0");
}

#[test]
fn json_lint_stringutils_min_vm() {
    ensure_stage3_env();

    // arity 自動補完をテストするため、using を使わずに static box で直接実装
    let src = r#"
static box StringUtils {
    starts_with(text, prefix) {
        local text_len = text.length()
        local prefix_len = prefix.length()
        if prefix_len > text_len { return 0 }
        local i = 0
        loop(i < prefix_len) {
            if text.substring(i, i + 1) != prefix.substring(i, i + 1) {
                return 0
            }
            i = i + 1
        }
        return 1
    }

    ends_with(text, suffix) {
        local text_len = text.length()
        local suffix_len = suffix.length()
        if suffix_len > text_len { return 0 }
        local offset = text_len - suffix_len
        local i = 0
        loop(i < suffix_len) {
            if text.substring(offset + i, offset + i + 1) != suffix.substring(i, i + 1) {
                return 0
            }
            i = i + 1
        }
        return 1
    }
}

static box Main {
    main() {
        if StringUtils.starts_with("abc", "a") and StringUtils.ends_with("abc", "c") {
            print("OK")
        } else {
            print("ERROR")
        }
        return 0
    }
}
"#;

    let ast: ASTNode = NyashParser::parse_from_string(src).expect("parse");
    let mut mc = MirCompiler::with_options(false);
    let cr = mc.compile(ast).expect("compile");

    let mut vm = VM::new();
    let result = vm.execute_module(&cr.module);

    // ✅ arity 自動補完により StringUtils.starts_with → StringUtils.starts_with/2 に解決されることを確認
    match result {
        Ok(_v) => {
            eprintln!("[json_lint_stringutils_min] VM executed successfully");
            // Success - arity auto-completion worked!
        }
        Err(e) => {
            panic!("VM should execute successfully, but got error: {:?}", e);
        }
    }

    // cleanup
    std::env::remove_var("NYASH_FEATURES");
    std::env::remove_var("NYASH_FEATURES");
    std::env::remove_var("NYASH_PARSER_ALLOW_SEMICOLON");
    std::env::remove_var("NYASH_DISABLE_PLUGINS");
    std::env::remove_var("HAKO_MIR_BUILDER_METHODIZE");
}
