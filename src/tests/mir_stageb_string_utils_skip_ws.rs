/*!
 * Phase 25.1 - ParserStringUtilsBox.skip_ws Void < 0 TypeError 再現テスト
 *
 * 目的: stageb_program_min_using_only.hako で発生する
 *       「Void < Integer(0)」TypeError を Rust テストで再現する
 *
 * エラー詳細:
 * - 関数: ParserStringUtilsBox.skip_ws/2
 * - 条件: loop(j < n) の Compare で Void < 0 が発生
 * - 原因: j または n が Void になるパスが存在
 */

#[cfg(test)]
mod tests {
    use crate::ast::ASTNode;
    use crate::backend::VM;
    use crate::mir::{MirCompiler, MirVerifier};
    use crate::parser::NyashParser;

    /// 最小再現ケース: ParserStringUtilsBox + Main
    fn make_minimal_src() -> String {
        // ParserStringUtilsBox 本体を include
        let utils =
            include_str!("../../lang/src/compiler/parser/scan/parser_string_utils_box.hako");

        // 最小 Main: stageb_program_min_using_only と同等の呼び出し
        let main = r#"
using lang.compiler.parser.scan.parser_string_utils_box as ParserStringUtilsBox

static box Main {
  main(args) {
    // stageb_program_min_using_only と同じ入力
    local src = "using \"foo/bar.hako\" as Foo\n"

    // skip_ws を直接呼び出し（Void < 0 を誘発）
    local pos = ParserStringUtilsBox.skip_ws(src, 0)

    print("skip_ws result: " + ("" + pos))
    return 0
  }
}
"#;

        format!("{}\n{}", utils, main)
    }

    #[test]
    fn mir_stageb_string_utils_skip_ws_compile() {
        // Enable Stage-3 parser for 'local' keyword
        std::env::set_var("NYASH_FEATURES", "stage3");
        std::env::set_var("NYASH_FEATURES", "stage3");

        let src = make_minimal_src();
        let ast: ASTNode = NyashParser::parse_from_string(&src).expect("parse should succeed");

        let mut mc = MirCompiler::with_options(true); // Stage-3 enabled
        let cr = mc
            .compile(ast)
            .expect("ParserStringUtilsBox compile should succeed");

        // MIR 検証
        let mut verifier = MirVerifier::new();
        if let Err(errors) = verifier.verify_module(&cr.module) {
            eprintln!("[mir-verify] Verification failed:");
            for e in &errors {
                eprintln!("  - {}", e);
            }
            panic!("MIR verification failed for ParserStringUtilsBox.skip_ws");
        }

        println!("✅ MIR compile and verify succeeded");
    }

    #[test]
    #[ignore] // 修正前用テスト（commit b00cc8d5 で再現）
    fn mir_stageb_string_utils_skip_ws_exec_reproduce_void_lt_zero() {
        // Enable Stage-3 parser
        std::env::set_var("NYASH_FEATURES", "stage3");
        std::env::set_var("NYASH_FEATURES", "stage3");

        let src = make_minimal_src();
        let ast: ASTNode = NyashParser::parse_from_string(&src).expect("parse ok");

        let mut mc = MirCompiler::with_options(true); // Stage-3 enabled
        let cr = mc.compile(ast).expect("compile ok");

        // MIR 検証
        let mut verifier = MirVerifier::new();
        verifier
            .verify_module(&cr.module)
            .expect("MIR verify should pass");

        // VM 実行（ここで Void < 0 エラーが出るはず）
        let mut vm = VM::new();
        let exec_result = vm.execute_module(&cr.module);

        // 現状は TypeError が出ることを確認（再現用）
        if let Err(e) = exec_result {
            let err_msg = format!("{:?}", e);
            println!("❌ Expected error reproduced: {}", err_msg);

            // Void < Integer(0) エラーを検出
            assert!(
                err_msg.contains("Void") || err_msg.contains("TypeError"),
                "Expected Void < Integer(0) TypeError, got: {}",
                err_msg
            );
        } else {
            panic!("Expected Void < 0 TypeError, but execution succeeded!");
        }
    }

    #[test]
    fn mir_stageb_string_utils_skip_ws_exec_success() {
        // Enable Stage-3 parser
        std::env::set_var("NYASH_FEATURES", "stage3");
        std::env::set_var("NYASH_FEATURES", "stage3");

        let src = make_minimal_src();
        let ast: ASTNode = NyashParser::parse_from_string(&src).expect("parse ok");

        let mut mc = MirCompiler::with_options(true); // Stage-3 enabled
        let cr = mc.compile(ast).expect("compile ok");

        // MIR 検証
        let mut verifier = MirVerifier::new();
        verifier
            .verify_module(&cr.module)
            .expect("MIR verify should pass");

        // VM 実行（修正後は成功するはず）
        let mut vm = VM::new();
        let exec_result = vm.execute_module(&cr.module);

        assert!(
            exec_result.is_ok(),
            "skip_ws should succeed after Void < 0 fix"
        );

        println!("✅ skip_ws execution succeeded (bug fixed!)");
    }
}
