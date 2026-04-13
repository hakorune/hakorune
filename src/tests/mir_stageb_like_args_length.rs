use crate::mir::MirPrinter;
use crate::parser::NyashParser;

fn ensure_stage3_env() {
    std::env::set_var("NYASH_FEATURES", "stage3");
    std::env::set_var("NYASH_PARSER_ALLOW_SEMICOLON", "1");
}
use crate::ast::ASTNode;
use crate::mir::{MirCompiler, MirVerifier};

/// Stage-B に似たパターン:
/// static box StageBArgsBox {
///   method resolve_src(args) {
///     if args != null {
///       local n = args.length();
///     }
///     return 0;
///   }
/// }
///
/// を Rust MirBuilder で MIR 化し、SSA/PHI が破綻していないことを検証する。
#[test]
fn mir_stageb_like_args_length_verifies() {
    ensure_stage3_env();
    let src = r#"
static box StageBArgsBox {
  method resolve_src(args) {
    if args != null {
      local n = args.length();
    }
    return 0;
  }
}
"#;

    // Parse Hako source into AST
    let ast: ASTNode = NyashParser::parse_from_string(src).expect("parse ok");

    // Compile to MIR (Rust MirBuilder)
    let mut mc = MirCompiler::with_options(false);
    let cr = mc.compile(ast).expect("compile");

    // Verify MIR SSA/PHI invariants
    let mut verifier = MirVerifier::new();
    if let Err(errors) = verifier.verify_module(&cr.module) {
        if std::env::var("NYASH_MIR_TEST_DUMP").ok().as_deref() == Some("1") {
            let dump = MirPrinter::new().print_module(&cr.module);
            eprintln!("----- MIR DUMP (StageBArgsBox.resolve_src) -----\n{}", dump);
        }
        for e in &errors {
            eprintln!("[rust-mir-verify] {}", e);
        }
        panic!("MIR verification failed for StageB-like args.length pattern");
    }
}

/// Stage-B 最小ハーネスに近いパターン:
/// static box StageBArgsBox {
///   method process(args) {
///     if args != null {
///       local n = args.length();
///       local i = 0;
///       loop (i < n) {
///         local item = args.get(i);
///         i = i + 1;
///       }
///     }
///     return 0;
///   }
/// }
///
/// を Rust MirBuilder で MIR 化し、SSA/PHI が破綻していないことを検証する。
#[test]
fn mir_stageb_like_if_args_length_loop_verifies() {
    ensure_stage3_env();
    let src = r#"
static box StageBArgsBox {
  method process(args) {
    if args != null {
      local n = args.length();
      local i = 0;
      loop (i < n) {
        local item = args.get(i);
        i = i + 1;
      }
    }
    return 0;
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
                "----- MIR DUMP (StageBArgsBox.process if+loop) -----\n{}",
                dump
            );
        }
        for e in &errors {
            eprintln!("[rust-mir-verify] {}", e);
        }
        panic!("MIR verification failed for StageB-like if+loop args.length pattern");
    }
}

/// Stage-B の TestNested.complex に近いネスト構造:
/// if data != null {
///   local count = data.length();
///   if count > 0 {
///     local j = 0;
///     loop (j < count) {
///       local val = data.get(j);
///       if val != null {
///         local s = "" + val;
///       }
///       j = j + 1;
///     }
///   }
/// }
#[test]
fn mir_stageb_like_nested_if_loop_verifies() {
    ensure_stage3_env();
    let src = r#"
static box TestNested {
  method complex(data) {
    if data != null {
      local count = data.length();
      if count > 0 {
        local j = 0;
        loop (j < count) {
          local val = data.get(j);
          if val != null {
            local s = "" + val;
          }
          j = j + 1;
        }
      }
    }
    return 0;
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
                "----- MIR DUMP (TestNested.complex nested if+loop) -----\n{}",
                dump
            );
        }
        for e in &errors {
            eprintln!("[rust-mir-verify] {}", e);
        }
        panic!("MIR verification failed for StageB-like nested if+loop pattern");
    }
}

/// Stage-B line-map debug に近いパターン:
/// outer loop が nested loop を含む no-exit body を持ち、
/// fallthrough continue edge を carrier PHI に反映する必要がある。
#[test]
fn mir_stageb_like_nested_loop_fallthrough_continue_verifies() {
    ensure_stage3_env();
    let src = r#"
static box StageBLineMapBox {
  method build_map_lines(bundles) {
    if bundles == null { return 0 }
    local total = 0
    local n = bundles.length()
    local i = 0
    loop (i < n) {
      local seg = "" + bundles.get(i)
      local ln = 0
      {
        local s2 = seg
        if s2 == null {
          ln = 0
        } else {
          local ii = 0
          local nn = ("" + s2).length()
          local cc = 1
          loop (ii < nn) {
            if ("" + s2).substring(ii, ii + 1) == "\n" { cc = cc + 1 }
            ii = ii + 1
          }
          ln = cc
        }
      }
      total = total + ln
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
                "----- MIR DUMP (StageBLineMapBox.build_map_lines nested fallthrough continue) -----\n{}",
                dump
            );
        }
        for e in &errors {
            eprintln!("[rust-mir-verify] {}", e);
        }
        panic!("MIR verification failed for StageB-like nested loop fallthrough continue pattern");
    }
}

/// Stage-B duplicate bundle check に近いパターン:
/// inner `loop(cond)` が `if return` + step だけを持つ。
/// これは loop_cond_return_in_body が優先される必要がある。
#[test]
fn mir_stageb_like_nested_loop_return_only_inner_verifies() {
    ensure_stage3_env();
    let src = r#"
static box StageBDupeBox {
  method find_duplicate(bundle_names) {
    if bundle_names == null { return 0 }
    local n = bundle_names.length()
    local i = 0
    loop (i < n) {
      local name_i = "" + bundle_names.get(i)
      local j = i + 1
      loop (j < n) {
        if ("" + bundle_names.get(j)) == name_i { return 1 }
        j = j + 1
      }
      i = i + 1
    }
    return 0
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
                "----- MIR DUMP (StageBDupeBox.find_duplicate nested return-only inner loop) -----\n{}",
                dump
            );
        }
        for e in &errors {
            eprintln!("[rust-mir-verify] {}", e);
        }
        panic!("MIR verification failed for StageB-like nested loop return-only inner pattern");
    }
}

/// Stage-B で出がちな「length を条件に直接使う」パターン:
/// if args != null {
///   local i = 0;
///   loop (i < args.length()) {
///     i = i + 1;
///   }
/// }
#[test]
fn mir_stageb_like_loop_cond_uses_length_verifies() {
    ensure_stage3_env();
    let src = r#"
static box StageBArgsBox {
  method process(args) {
    if args != null {
      local i = 0;
      loop (i < args.length()) {
        i = i + 1;
      }
    }
    return 0;
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
                "----- MIR DUMP (StageBArgsBox.process loop cond uses length) -----\n{}",
                dump
            );
        }
        for e in &errors {
            eprintln!("[rust-mir-verify] {}", e);
        }
        panic!("MIR verification failed for StageB-like loop cond args.length pattern");
    }
}

/// length を if 条件と body の両方で使うパターン:
/// if data != null && data.length() > 0 {
///   local i = 0;
///   loop (i < data.length()) {
///     i = i + 1;
///   }
/// }
#[test]
fn mir_stageb_like_conditional_and_loop_length_verifies() {
    ensure_stage3_env();
    let src = r#"
static box TestNested2 {
  method walk(data) {
    if data != null && data.length() > 0 {
      local i = 0;
      loop (i < data.length()) {
        i = i + 1;
      }
    }
    return 0;
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
                "----- MIR DUMP (TestNested2.walk conditional+loop length) -----\n{}",
                dump
            );
        }
        for e in &errors {
            eprintln!("[rust-mir-verify] {}", e);
        }
        panic!("MIR verification failed for StageB-like conditional+loop length pattern");
    }
}

/// JsonScanBox.seek_array_end に近いパターン:
/// - text.length() をループ条件・境界チェック・内部でも使う
/// - 文字列内/エスケープなどの分岐を含むが、ここでは最小限の骨格のみを再現。
#[test]
fn mir_jsonscanbox_like_seek_array_end_verifies() {
    ensure_stage3_env();
    let src = r#"
using selfhost.shared.json.core.string_scan as StringScanBox

static box JsonScanBoxMini {
  method seek_array_end(text, start) {
    if text == null { return -1 }
    local n = text.length()
    if start < 0 || start >= n { return -1 }
    local depth = 0
    local i = start
    loop (i < n) {
      local ch = StringScanBox.read_char(text, i)
      if ch == "[" {
        depth = depth + 1
      } else if ch == "]" {
        depth = depth - 1
        if depth == 0 { return i }
      }
      i = i + 1
    }
    return -1
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
                "----- MIR DUMP (JsonScanBoxMini.seek_array_end) -----\n{}",
                dump
            );
        }
        for e in &errors {
            eprintln!("[rust-mir-verify] {}", e);
        }
        panic!("MIR verification failed for JsonScanBox-like seek_array_end pattern");
    }
}
