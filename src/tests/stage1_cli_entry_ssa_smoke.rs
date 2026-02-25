use crate::ast::ASTNode;
use crate::mir::{MirCompiler, MirVerifier};
use crate::parser::NyashParser;

/// Minimal reproduction of Stage1Cli-style entry that calls `.size()` on an
/// argument-derived value. The goal is to mirror the Stage1 CLI pattern
/// (args/argc handling and a small loop) and ensure MIR/SSA stays consistent.
#[test]
fn mir_stage1_cli_entry_like_pattern_verifies() {
    // Enable Stage‑3 and using so the parser accepts modern syntax.
    std::env::set_var("NYASH_FEATURES", "stage3");
    std::env::set_var("NYASH_PARSER_ALLOW_SEMICOLON", "1");
    std::env::set_var("NYASH_ENABLE_USING", "1");
    std::env::set_var("HAKO_ENABLE_USING", "1");

    let src = r#"
static box Stage1CliEntryLike {
  main(args) {
    // Guard args and argc before calling size() to keep SSA simple.
    if args == null { return 97 }

    local argc = 0
    argc = args.size()

    // Region+next_i style loop over argv for future extension.
    local i = 0
    loop(i < argc) {
      local next_i = i + 1
      local arg = "" + args.get(i)
      if arg == "" { /* skip */ }
      i = next_i
    }
    return argc
  }
}
"#;

    let ast: ASTNode = NyashParser::parse_from_string(src).expect("parse ok");
    let mut mc = MirCompiler::with_options(false);
    let cr = mc.compile(ast).expect("compile");

    let mut verifier = MirVerifier::new();
    if let Err(errors) = verifier.verify_module(&cr.module) {
        for e in &errors {
            eprintln!("[rust-mir-verify] {}", e);
        }
        panic!("MIR verification failed for Stage1CliEntryLike");
    }
}

/// Shape test: env-only の最小ディスパッチャで SSA/PHI 崩れない形を固定する。
#[test]
fn mir_stage1_cli_stage1_main_shape_verifies() {
    std::env::set_var("NYASH_FEATURES", "stage3");
    std::env::set_var("NYASH_PARSER_ALLOW_SEMICOLON", "1");
    std::env::set_var("NYASH_ENABLE_USING", "1");
    std::env::set_var("HAKO_ENABLE_USING", "1");

    let src = r#"
static box Stage1CliShape {
  // env-only 仕様の最小スタブ
  emit_program_json(source) { if source == null || source == "" { return null } return "{prog:" + source + "}" }
  emit_mir_json(program_json) { if program_json == null || program_json == "" { return null } return "{mir:" + program_json + "}" }
  run_program_json(program_json, backend) {
    if backend == null || backend == "" { backend = "vm" }
    if program_json == null || program_json == "" { return 96 }
    if backend == "vm" || backend == "llvm" || backend == "pyvm" { return 0 }
    return 99
  }

  // args 依存を排し、少数の分岐だけで SSA を固定
  stage1_main(args) {
    // 最小の“形”だけを固定する（env トグル確認 → 即 return）
    if env.get("NYASH_USE_STAGE1_CLI") != "1" { return 97 }
    if env.get("STAGE1_EMIT_PROGRAM_JSON") == "1" { return 0 }
    if env.get("STAGE1_EMIT_MIR_JSON") == "1" { return 0 }
    if env.get("STAGE1_SOURCE") == null || env.get("STAGE1_SOURCE") == "" { return 96 }
    return 0
  }
}
"#;

    let ast: ASTNode = NyashParser::parse_from_string(src).expect("parse ok");
    let mut mc = MirCompiler::with_options(false);
    let cr = mc.compile(ast).expect("compile");

    let mut verifier = MirVerifier::new();
    if let Err(errors) = verifier.verify_module(&cr.module) {
        for e in &errors {
            eprintln!("[rust-mir-verify] {}", e);
        }
        panic!("MIR verification failed for Stage1CliShape.stage1_main");
    }
}
