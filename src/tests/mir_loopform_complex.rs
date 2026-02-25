//! LoopForm / Region+next_i 複雑パターン向けスモークテスト
//!
//! 目的:
//! - continue + break 混在ループ
//! - break + early-return 混在ループ
//! - 外側ループ + 内側 Region 型ループのネスト
//! を MIR Verifier ベースで「構造テスト」として押さえる。

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
