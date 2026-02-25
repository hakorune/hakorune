use crate::ast::ASTNode;
use crate::mir::{MirCompiler, MirPrinter, MirVerifier};
use crate::parser::NyashParser;

fn ensure_stage3_env() {
    std::env::set_var("NYASH_FEATURES", "stage3");
    std::env::set_var("NYASH_PARSER_ALLOW_SEMICOLON", "1");
}

/// Stage‑B 風: loop + break/continue + ArrayBox.length/get
///
/// static box LoopBreakContinueBox {
///   method sum_positive_until_null(arr) {
///     if arr == null { return 0 }
///     local i = 0
///     local acc = 0
///     loop (i < arr.length()) {
///       local v = arr.get(i)
///       if v == null { break }
///       if v < 0 {
///         i = i + 1
///         continue
///       }
///       acc = acc + v
///       i = i + 1
///     }
///     return acc
///   }
/// }
#[test]
fn mir_stageb_loop_break_continue_verifies() {
    ensure_stage3_env();
    let src = r#"
static box LoopBreakContinueBox {
  method sum_positive_until_null(arr) {
    if arr == null { return 0 }
    local i = 0
    local acc = 0
    loop (i < arr.length()) {
      local v = arr.get(i)
      if v == null { break }
      if v < 0 {
        i = i + 1
        continue
      }
      acc = acc + v
      i = i + 1
    }
    return acc
  }
}
"#;

    let ast: ASTNode = NyashParser::parse_from_string(src).expect("parse ok");
    let mut mc = MirCompiler::with_options(false);
    let cr = mc.compile(ast).expect("compile");

    let mut verifier = MirVerifier::new();
    if let Err(errors) = verifier.verify_module(&cr.module) {
        if std::env::var("NYASH_MIR_TEST_DUMP").ok().as_deref() == Some("1") {
            let dump = MirPrinter::new().print_module(&cr.module);
            eprintln!(
                "----- MIR DUMP (LoopBreakContinueBox.sum_positive_until_null) -----\n{}",
                dump
            );
        }
        for e in &errors {
            eprintln!("[rust-mir-verify] {}", e);
        }
        panic!("MIR verification failed for StageB-like loop+break/continue pattern");
    }
}

/// Stage‑B 風: 入れ子ループ + break/continue + length/get
///
/// static box LoopNestedBreakBox {
///   method nested_walk(arr) {
///     if arr == null { return 0 }
///     local i = 0
///     local total = 0
///     loop (i < arr.length()) {
///       local inner = arr.get(i)
///       if inner == null {
///         i = i + 1
///         continue
///       }
///       local j = 0
///       loop (j < inner.length()) {
///         local v = inner.get(j)
///         if v == null { break }
///         total = total + v
///         j = j + 1
///       }
///       i = i + 1
///     }
///     return total
///   }
/// }
#[test]
fn mir_stageb_nested_loop_break_continue_verifies() {
    ensure_stage3_env();
    let src = r#"
static box LoopNestedBreakBox {
  method nested_walk(arr) {
    if arr == null { return 0 }
    local i = 0
    local total = 0
    loop (i < arr.length()) {
      local inner = arr.get(i)
      if inner == null {
        i = i + 1
        continue
      }
      local j = 0
      loop (j < inner.length()) {
        local v = inner.get(j)
        if v == null { break }
        total = total + v
        j = j + 1
      }
      i = i + 1
    }
    return total
  }
}
"#;

    let ast: ASTNode = NyashParser::parse_from_string(src).expect("parse ok");
    let mut mc = MirCompiler::with_options(false);
    let cr = mc.compile(ast).expect("compile");

    let mut verifier = MirVerifier::new();
    if let Err(errors) = verifier.verify_module(&cr.module) {
        if std::env::var("NYASH_MIR_TEST_DUMP").ok().as_deref() == Some("1") {
            let dump = MirPrinter::new().print_module(&cr.module);
            eprintln!(
                "----- MIR DUMP (LoopNestedBreakBox.nested_walk) -----\n{}",
                dump
            );
        }
        for e in &errors {
            eprintln!("[rust-mir-verify] {}", e);
        }
        panic!("MIR verification failed for StageB-like nested loop+break/continue pattern");
    }
}
