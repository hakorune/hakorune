//! LoopForm exit PHI regression tests (conditional reassign / break)
//!
//! 目的:
//! - loop(1 == 1) + break 経路で exit PHI が壊れて ValueId 未定義になる既知バグを捕まえる。
//! - 将来: 条件付き再代入や body-local 変数が混在するケースを拡張（現状は #[ignore]）。

use crate::mir::{MirCompiler, MirVerifier};
use crate::parser::NyashParser;

fn setup_stage3_env() {
    std::env::set_var("NYASH_FEATURES", "stage3");
    std::env::set_var("NYASH_ENABLE_USING", "1");
    std::env::set_var("HAKO_ENABLE_USING", "1");
    std::env::set_var("NYASH_DISABLE_PLUGINS", "1");
}

fn teardown_stage3_env() {
    std::env::remove_var("NYASH_FEATURES");
    std::env::remove_var("NYASH_ENABLE_USING");
    std::env::remove_var("HAKO_ENABLE_USING");
    std::env::remove_var("NYASH_DISABLE_PLUGINS");
}

fn compile_module(src: &str) -> crate::mir::MirCompileResult {
    setup_stage3_env();
    let ast = NyashParser::parse_from_string(src).expect("parse ok");
    let mut mc = MirCompiler::with_options(false);
    mc.compile(ast).expect("compile ok")
}

// [LoopForm-Test] Case B: constant-true+break-only
#[test]
fn loop_constant_true_exit_phi_dominates() {
    // Repro: loop(1 == 1) + break only (header is NOT an exit predecessor)
    // This used to create exit PHIs with header inputs from non-predecessor blocks → dominator violation.
    let src = include_str!("../../apps/tests/minimal_ssa_skip_ws.hako");
    let compiled = compile_module(src);

    let mut verifier = MirVerifier::new();
    if let Err(errors) = verifier.verify_module(&compiled.module) {
        for e in &errors {
            eprintln!("[mir-verify] {}", e);
        }
        teardown_stage3_env();
        panic!("loop_constant_true_exit_phi_dominates: MIR verification failed");
    }
    teardown_stage3_env();
}

// [LoopForm-Test] Case A: header+break
#[test]
fn loop_conditional_reassign_exit_phi_header_and_break() {
    // Case A: 通常の loop(i < n) で header→exit と body→exit の両方が存在するケース。
    // ここで PHI が壊れていないことを確認し、今回の修正が既存パターンを regress していないことを見る。
    let src = r#"
static box Main {
  loop_case(s) {
    local i = 0
    local n = s.length()
    loop(i < n) {
      if i >= n { break }
      i = i + 1
    }
    return i
  }

  main(args) {
    local s = "abc"
    local r = Main.loop_case(s)
    print(r)
    return 0
  }
}
"#;
    let compiled = compile_module(src);

    let mut verifier = MirVerifier::new();
    if let Err(errors) = verifier.verify_module(&compiled.module) {
        for e in &errors {
            eprintln!("[mir-verify] {}", e);
        }
        teardown_stage3_env();
        panic!("loop_conditional_reassign_exit_phi_header_and_break: MIR verification failed");
    }
    teardown_stage3_env();
}

// [LoopForm-Test] Case C: body-local (BodyLocalInternal)
#[test]
fn loop_body_local_exit_phi_body_only() {
    // Case C: body-local 変数が break 経路にだけ存在し、header には存在しないケース。
    // Option C の分類により、こうした BodyLocalInternal には exit PHI が張られず、
    // MIR 検証が通ることを確認する。
    let src = r#"
static box Main {
  loop_case(s) {
    local i = 0
    local n = s.length()
    loop(1 == 1) {
      if i >= n { break }
      local temp = s.substring(i, i + 1)
      if temp == "x" { break } else { i = i + 1 }
    }
    return i
  }

  main(args) {
    local s = "abc"
    local r = Main.loop_case(s)
    print(r)
    return 0
  }
}
"#;
    let compiled = compile_module(src);

    let mut verifier = MirVerifier::new();
    if let Err(errors) = verifier.verify_module(&compiled.module) {
        for e in &errors {
            eprintln!("[mir-verify] {}", e);
        }
        teardown_stage3_env();
        panic!("loop_body_local_exit_phi_body_only: MIR verification failed");
    }
    teardown_stage3_env();
}

// [LoopForm-Test] Case D: continue+break (continue_merge → header → exit)
#[test]
fn loop_continue_merge_header_exit() {
    // Case D: continue 文を含むループで、continue_merge → header → exit の組み合わせを検証。
    // continue_merge ブロックで PHI を生成し、header PHI に正しく伝播することを確認。
    let src = r#"
static box Main {
  loop_case(s) {
    local i = 0
    local n = s.length()
    loop(i < n) {
      local ch = s.substring(i, i + 1)
      if ch == " " {
        i = i + 1
        continue
      }
      if ch == "x" { break }
      i = i + 1
    }
    return i
  }

  main(args) {
    local s = "  x  "
    local r = Main.loop_case(s)
    print(r)
    return 0
  }
}
"#;
    let compiled = compile_module(src);

    let mut verifier = MirVerifier::new();
    if let Err(errors) = verifier.verify_module(&compiled.module) {
        for e in &errors {
            eprintln!("[mir-verify] {}", e);
        }
        teardown_stage3_env();
        panic!("loop_continue_merge_header_exit: MIR verification failed");
    }
    teardown_stage3_env();
}
