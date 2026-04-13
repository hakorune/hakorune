//! LoopForm / Region+next_i 複雑パターン向けスモークテスト
//!
//! 目的:
//! - continue + break 混在ループ
//! - break + early-return 混在ループ
//! - 外側ループ + 内側 Region 型ループのネスト
//! を MIR Verifier ベースで「構造テスト」として押さえる。

use crate::mir::{MirCompiler, MirPrinter, MirVerifier};
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
    std::env::remove_var("NYASH_JOINIR_DEV");
    std::env::remove_var("HAKO_JOINIR_STRICT");
    std::env::remove_var("HAKO_JOINIR_PLANNER_REQUIRED");
}

fn compile_module(src: &str) -> crate::mir::MirCompileResult {
    setup_stage3_env();
    let ast = NyashParser::parse_from_string(src).expect("parse ok");
    let mut mc = MirCompiler::with_options(false);
    mc.compile(ast).expect("compile ok")
}

fn compile_module_strict_planner(src: &str) -> crate::mir::MirCompileResult {
    setup_stage3_env();
    std::env::set_var("NYASH_JOINIR_DEV", "1");
    std::env::set_var("HAKO_JOINIR_STRICT", "1");
    std::env::set_var("HAKO_JOINIR_PLANNER_REQUIRED", "1");
    let ast = NyashParser::parse_from_string(src).expect("parse ok");
    let mut mc = MirCompiler::with_options(false);
    mc.compile(ast).expect("compile ok")
}

fn with_env_var<T>(key: &str, value: &str, f: impl FnOnce() -> T) -> T {
    let prev = std::env::var(key).ok();
    std::env::set_var(key, value);
    let out = f();
    match prev {
        Some(v) => std::env::set_var(key, v),
        None => std::env::remove_var(key),
    }
    out
}

#[test]
fn mir_loopform_continue_break_scan_verify() {
    // Case C-ish: constant-true + continue/backedge + break（Region+next_i 型）
    let src = include_str!("../../apps/tests/loopform_continue_break_scan.hako");
    let compiled = compile_module(src);

    let mut verifier = MirVerifier::new();
    if let Err(errors) = verifier.verify_module(&compiled.module) {
        for e in &errors {
            eprintln!("[mir-verify] {}", e);
        }
        teardown_stage3_env();
        panic!("mir_loopform_continue_break_scan_verify: MIR verification failed");
    }
    teardown_stage3_env();
}

#[test]
fn mir_loopform_break_and_return_verify() {
    // Case: header-cond + break + early-return 混在
    let src = include_str!("../../apps/tests/loopform_break_and_return.hako");
    let compiled = compile_module(src);

    let mut verifier = MirVerifier::new();
    if let Err(errors) = verifier.verify_module(&compiled.module) {
        for e in &errors {
            eprintln!("[mir-verify] {}", e);
        }
        teardown_stage3_env();
        panic!("mir_loopform_break_and_return_verify: MIR verification failed");
    }
    teardown_stage3_env();
}

#[test]
fn mir_loopform_nested_region_verify() {
    // Case: 外側 Loop + 内側 Region 型 Loop（inner は constant-true + continue + break）
    let src = include_str!("../../apps/tests/loopform_nested_region.hako");
    let compiled = compile_module(src);

    let mut verifier = MirVerifier::new();
    if let Err(errors) = verifier.verify_module(&compiled.module) {
        for e in &errors {
            eprintln!("[mir-verify] {}", e);
        }
        teardown_stage3_env();
        panic!("mir_loopform_nested_region_verify: MIR verification failed");
    }
    teardown_stage3_env();
}

#[test]
fn mir_loopform_nested_loop_if_else_return_verify() {
    // phase29bq blocker: nested loop body exits through if/else return only.
    let src = include_str!(
        "../../apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_return_min.hako"
    );
    let compiled = compile_module(src);

    let mut verifier = MirVerifier::new();
    if let Err(errors) = verifier.verify_module(&compiled.module) {
        let dump = MirPrinter::verbose().print_module(&compiled.module);
        eprintln!(
            "----- MIR DUMP (nested_loop_if_else_return) -----\n{}",
            dump
        );
        for e in &errors {
            eprintln!("[mir-verify] {}", e);
        }
        teardown_stage3_env();
        panic!("mir_loopform_nested_loop_if_else_return_verify: MIR verification failed");
    }
    teardown_stage3_env();
}

#[test]
fn mir_loopform_loop_true_multi_break_parserish_verify() {
    // phase29bq blocker: loop(true) keeps the structural header->after predecessor
    // while also merging multiple concrete break exits.
    let src = include_str!("../../apps/tests/phase29bq_loop_true_multi_break_parserish_min.hako");
    let compiled = compile_module(src);

    let mut verifier = MirVerifier::new();
    if let Err(errors) = verifier.verify_module(&compiled.module) {
        let dump = MirPrinter::verbose().print_module(&compiled.module);
        eprintln!(
            "----- MIR DUMP (loop_true_multi_break_parserish) -----\n{}",
            dump
        );
        for e in &errors {
            eprintln!("[mir-verify] {}", e);
        }
        teardown_stage3_env();
        panic!("mir_loopform_loop_true_multi_break_parserish_verify: MIR verification failed");
    }
    teardown_stage3_env();
}

#[test]
fn mir_loopform_parse_block_min_verify() {
    // phase29bq blocker: loop-local temporaries from trim loops must not leak
    // into the outer if/return join payload.
    let src = include_str!("../../apps/tests/phase29bq_selfhost_blocker_parse_block_min.hako");
    let compiled = compile_module(src);

    let mut verifier = MirVerifier::new();
    if let Err(errors) = verifier.verify_module(&compiled.module) {
        let dump = MirPrinter::verbose().print_module(&compiled.module);
        eprintln!("----- MIR DUMP (parse_block_min) -----\n{}", dump);
        for e in &errors {
            eprintln!("[mir-verify] {}", e);
        }
        teardown_stage3_env();
        panic!("mir_loopform_parse_block_min_verify: MIR verification failed");
    }
    teardown_stage3_env();
}

#[test]
fn mir_loopform_continue_only_group_if_prelude_verify() {
    // phase29bq blocker: release selfhost entry must keep recipe-authority
    // general-if lowering inside continue-only prelude.
    let src = r#"
static box ProgramJsonV0RuneAttrsBox {
  method count_params_or_error(params_json, tag) {
    local s = "" + params_json
    local n = s.length()
    local i = 0
    local count = 0
    local in_str = 0
    local esc = 0
    loop(i < n) {
      local ch = s.substring(i, i + 1)
      if in_str == 1 {
        if esc == 1 {
          esc = 0
        } else if ch == "\\" {
          esc = 1
        } else if ch == "\"" {
          in_str = 0
        }
        i = i + 1
        continue
      }
      if ch == "\"" {
        in_str = 1
        count = count + 1
      }
      i = i + 1
    }
    return count
  }
}
"#;
    let compiled = compile_module(src);

    let mut verifier = MirVerifier::new();
    if let Err(errors) = verifier.verify_module(&compiled.module) {
        let dump = MirPrinter::verbose().print_module(&compiled.module);
        eprintln!(
            "----- MIR DUMP (continue_only_group_if_prelude) -----\n{}",
            dump
        );
        for e in &errors {
            eprintln!("[mir-verify] {}", e);
        }
        teardown_stage3_env();
        panic!("mir_loopform_continue_only_group_if_prelude_verify: MIR verification failed");
    }
    teardown_stage3_env();
}

#[test]
fn mir_loopform_continue_only_group_if_prelude_strict_planner_verify() {
    // phase29bq blocker: planner-required mode must classify nested if/else-if
    // prelude as group-prelude instead of plain ContinueIf.
    let src = include_str!(
        "../../lang/src/compiler/mirbuilder/program_json_v0_rune_attrs_box.hako"
    );
    let compiled = compile_module_strict_planner(src);

    let mut verifier = MirVerifier::new();
    if let Err(errors) = verifier.verify_module(&compiled.module) {
        let dump = MirPrinter::verbose().print_module(&compiled.module);
        eprintln!(
            "----- MIR DUMP (continue_only_group_if_prelude_strict_planner) -----\n{}",
            dump
        );
        for e in &errors {
            eprintln!("[mir-verify] {}", e);
        }
        teardown_stage3_env();
        panic!(
            "mir_loopform_continue_only_group_if_prelude_strict_planner_verify: MIR verification failed"
        );
    }
    teardown_stage3_env();
}

#[test]
fn vm_strict_planner_env_get_keeps_nonnull_string_value() {
    let src = r#"
static box Main {
  main() {
    local p = env.get("HAKO_PROGRAM_JSON_FILE")
    if p == null || ("" + p) == "" {
      return 0
    }
    return 1
  }
}
"#;
    let compiled = compile_module_strict_planner(src);
    let result = with_env_var("HAKO_PROGRAM_JSON_FILE", "/tmp/test_program_json.json", || {
        let mut vm = crate::backend::VM::new();
        vm.execute_module(&compiled.module).expect("vm exec failed")
    });
    assert_eq!(result.to_string_box().value, "1");
    teardown_stage3_env();
}
