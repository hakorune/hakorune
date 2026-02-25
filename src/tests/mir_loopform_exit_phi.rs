/*!
 * Unit tests for LoopForm v2 exit PHI generation
 *
 * Tests the build_exit_phis() implementation in loopform_builder.rs
 * Focus: predecessor tracking and PHI input generation for break statements
 */

use crate::mir::{MirCompiler, MirVerifier};
use crate::parser::NyashParser;

#[test]
fn test_loopform_exit_phi_single_break() {
    // LoopForm PHI v2 はデフォルト実装（フラグ不要）
    // Enable MIR verification and debug traces
    std::env::set_var("NYASH_VM_VERIFY_MIR", "1");
    std::env::set_var("NYASH_LOOPFORM_DEBUG", "1");
    std::env::set_var("NYASH_FEATURES", "stage3");
    std::env::set_var("NYASH_PARSER_ALLOW_SEMICOLON", "1");

    let src = r#"
static box TestExitPhi {
  test() {
    local i = 0
    loop(i < 10) {
      if i == 5 { break }
      i = i + 1
    }
    return i
  }
}
"#;

    println!("=== Test: Single break statement ===");

    // Parse
    let ast = NyashParser::parse_from_string(src).expect("parse failed");

    // Compile
    let mut mc = MirCompiler::with_options(false);
    let cr = mc.compile(ast).expect("compile failed");

    // DEBUG: Dump MIR structure
    for (fname, func) in &cr.module.functions {
        eprintln!("=== Function: {} ===", fname);
        eprintln!("Entry block: {:?}", func.entry_block);
        eprintln!("Total blocks: {}", func.blocks.len());
        for (bid, block) in &func.blocks {
            eprintln!(
                "  Block {:?}: {} instructions, successors={:?}",
                bid,
                block.instructions.len(),
                block.successors
            );
            if *bid == crate::mir::BasicBlockId(10) {
                eprintln!("    BB10 instructions:");
                for inst in &block.instructions {
                    eprintln!("      {:?}", inst);
                }
            }
        }
    }

    // MIR verification
    let mut verifier = MirVerifier::new();
    if let Err(errors) = verifier.verify_module(&cr.module) {
        for err in &errors {
            eprintln!("❌ MIR verification error: {}", err);
        }
        panic!("❌ MIR verification failed with {} errors", errors.len());
    }
    println!("✅ MIR verification passed");
}

#[test]
fn test_loopform_exit_phi_multiple_breaks() {
    // LoopForm PHI v2 はデフォルト実装（フラグ不要）
    std::env::set_var("NYASH_VM_VERIFY_MIR", "1");
    std::env::set_var("NYASH_LOOPFORM_DEBUG", "1");
    std::env::set_var("NYASH_FEATURES", "stage3");
    std::env::set_var("NYASH_PARSER_ALLOW_SEMICOLON", "1");

    let src = r#"
static box TestMultiBreak {
  test() {
    local i = 0
    loop(i < 10) {
      if i == 3 { break }
      if i == 5 { break }
      i = i + 1
    }
    return i
  }
}
"#;

    println!("=== Test: Multiple break statements ===");

    let ast = NyashParser::parse_from_string(src).expect("parse failed");
    let mut mc = MirCompiler::with_options(false);
    let cr = mc.compile(ast).expect("compile failed");

    let mut verifier = MirVerifier::new();
    if let Err(errors) = verifier.verify_module(&cr.module) {
        for err in &errors {
            eprintln!("❌ MIR verification error: {}", err);
        }
        panic!("❌ MIR verification failed with {} errors", errors.len());
    }
    println!("✅ MIR verification passed");
}

#[test]
fn test_loopform_exit_phi_nested_if_break() {
    // LoopForm PHI v2 はデフォルト実装（フラグ不要）
    std::env::set_var("NYASH_VM_VERIFY_MIR", "1");
    std::env::set_var("NYASH_LOOPFORM_DEBUG", "1");
    std::env::set_var("NYASH_FEATURES", "stage3");
    std::env::set_var("NYASH_PARSER_ALLOW_SEMICOLON", "1");

    let src = r#"
static box TestNestedBreak {
  test() {
    local i = 0
    local found = 0
    loop(i < 10) {
      if i > 5 {
        if i == 7 {
          found = 1
          break
        }
      }
      i = i + 1
    }
    return found
  }
}
"#;

    println!("=== Test: Nested if with break ===");

    let ast = NyashParser::parse_from_string(src).expect("parse failed");
    let mut mc = MirCompiler::with_options(false);
    let cr = mc.compile(ast).expect("compile failed");

    let mut verifier = MirVerifier::new();
    if let Err(errors) = verifier.verify_module(&cr.module) {
        for err in &errors {
            eprintln!("❌ MIR verification error: {}", err);
        }
        panic!("❌ MIR verification failed with {} errors", errors.len());
    }
    println!("✅ MIR verification passed");
}

#[test]
fn test_loopform_exit_phi_multiple_vars() {
    // LoopForm PHI v2 はデフォルト実装（フラグ不要）
    std::env::set_var("NYASH_VM_VERIFY_MIR", "1");
    std::env::set_var("NYASH_LOOPFORM_DEBUG", "1");
    std::env::set_var("NYASH_PARSER_ALLOW_SEMICOLON", "1");

    let src = r#"
static box TestMultiVars {
  test() {
    local i = 0
    local sum = 0
    local product = 1
    loop(i < 10) {
      if sum > 20 { break }
      sum = sum + i
      product = product * 2
      i = i + 1
    }
    return sum
  }
}
"#;

    println!("=== Test: Multiple variables with break ===");

    let ast = NyashParser::parse_from_string(src).expect("parse failed");
    let mut mc = MirCompiler::with_options(false);
    let cr = mc.compile(ast).expect("compile failed");

    let mut verifier = MirVerifier::new();
    if let Err(errors) = verifier.verify_module(&cr.module) {
        for err in &errors {
            eprintln!("❌ MIR verification error: {}", err);
        }
        panic!("❌ MIR verification failed with {} errors", errors.len());
    }
    println!("✅ MIR verification passed");
}

/// LoopScope/Env_in/out の基本挙動テスト
///
/// - Carrier: i
/// - Invariant: len
/// - 期待: i は PHI を通じてループキャリーされるが、len は PHI には乗らない。
///   （MirVerifier が SSA を検証しつつ、Phi の個数が過剰になっていないことを確認）
#[test]
fn test_loop_scope_env_carrier_and_invariant() {
    std::env::set_var("NYASH_VM_VERIFY_MIR", "1");
    std::env::set_var("NYASH_LOOPFORM_DEBUG", "1");
    std::env::set_var("NYASH_PARSER_ALLOW_SEMICOLON", "1");

    let src = r#"
static box TestLoopScopeEnv {
  test() {
    local i = 0       // carrier
    local len = 5     // invariant
    loop(i < len) {
      i = i + 1
    }
    return i + len
  }
}
"#;

    let ast = NyashParser::parse_from_string(src).expect("parse failed");
    let mut mc = MirCompiler::with_options(false);
    let cr = mc.compile(ast).expect("compile failed");

    // MIR 構造検証（SSA / PHI まわり）
    let mut verifier = MirVerifier::new();
    if let Err(errors) = verifier.verify_module(&cr.module) {
        for err in &errors {
            eprintln!("❌ MIR verification error: {}", err);
        }
        panic!("❌ MIR verification failed with {} errors", errors.len());
    }

    // PHI 命令数が「キャリア i の header/exit 用」に相当する範囲に収まっていることを軽く確認
    // （Invariant len に対して余計な PHI が増えていないことの簡易チェック）
    let mut phi_count = 0usize;
    for func in cr.module.functions.values() {
        for block in func.blocks.values() {
            for inst in &block.instructions {
                if let crate::mir::MirInstruction::Phi { .. } = inst {
                    phi_count += 1;
                }
            }
        }
    }
    // 25.2 以降は pinned / carrier / body-local exit PHI が追加されるため、
    // PHI 数は実装詳細に依存する。ここでは「極端に増えていないこと」だけを確認する。
    assert!(
        phi_count >= 1 && phi_count <= 10,
        "unexpected PHI count for simple carrier+invariant loop: {}",
        phi_count
    );
}
