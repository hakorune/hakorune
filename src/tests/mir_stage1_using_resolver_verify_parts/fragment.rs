use crate::ast::ASTNode;
use crate::mir::printer::MirPrinter;
use crate::mir::{MirCompiler, MirVerifier};
use crate::parser::NyashParser;

use super::shared::ensure_stage3_env;

/// Minimal Stage‑1 using resolver harness resembling Stage1UsingResolverBox.resolve_for_source.
/// Focuses on loops over ArrayBox/MapBox and JSON scanning, without FileBox/@ sugar.
#[test]
fn mir_stage1_using_resolver_min_fragment_verifies() {
    ensure_stage3_env();
    let src = r#"
using lang.compiler.parser.scan.parser_common_utils_box as ParserCommonUtilsBox
using selfhost.shared.json.utils.json_frag as JsonFragBox

static box Stage1UsingResolverMini {
  resolve_for_source(src) {
    if src == null { return "" }

    // Collect entries; empty/zero-length guard
    local entries = me._collect_using_entries(src)
    if entries == null || entries.length() == 0 { return "" }

    // Build prefix by iterating entries (loop with MapBox/ArrayBox access)
    local prefix = ""
    local i = 0
    local n = entries.length()
    loop(i < n) {
      local entry = entries.get(i)
      local name = "" + entry.get("name")
      if name == "" {
        i = i + 1
        continue
      }
      prefix = prefix + name
      i = i + 1
    }
    return prefix
  }

  _collect_using_entries(src) {
    // Minimal JSON scan loop similar to real Stage1UsingResolverBox._collect_using_entries
    local json = "[{\"name\":\"A\"},{\"name\":\"B\"}]"
    local out = new ArrayBox()
    local pos = 0
    local n = json.length()
    loop(pos < n) {
      local name_idx = JsonFragBox.index_of_from(json, "\"name\":\"", pos)
      if name_idx < 0 { break }
      local name = JsonFragBox.read_string_after(json, name_idx + 7)
      local entry = new MapBox()
      entry.set("name", name)
      out.push(entry)
      pos = pos + 1
    }
    return out
  }
}
"#;

    let ast: ASTNode = NyashParser::parse_from_string(src).expect("parse ok");
    let mut mc = MirCompiler::with_options(false);
    let cr = mc.compile(ast).expect("compile");

    // DEBUG: Print MIR structure for ALL functions (temporary)
    for (fname, func) in &cr.module.functions {
        eprintln!("\n=== MIR for {} ===", fname);
        let printer = MirPrinter::new();
        let mir_text = printer.print_function(func);
        eprintln!("{}", mir_text);
    }

    let mut verifier = MirVerifier::new();
    if let Err(errors) = verifier.verify_module(&cr.module) {
        for e in &errors {
            eprintln!("[rust-mir-verify] {}", e);
        }
        panic!("MIR verification failed for Stage1UsingResolverMini");
    }
}

/// Full-featured Stage1UsingResolverBox._collect_using_entries test with obj_end/path_idx logic.
/// This more closely resembles the actual implementation in lang/src/compiler/entry/using_resolver_box.hako.
/// Tests complex loop with nested conditions and `me` receiver usage without external using dependencies.
#[test]
fn mir_stage1_using_resolver_full_collect_entries_verifies() {
    ensure_stage3_env();
    let src = r#"
static box Stage1UsingResolverFull {
  // Simplified helper to find substring index (replaces JsonFragBox.index_of_from)
  _find_from(text, pattern, start_pos) {
    local text_len = text.length()
    local pattern_len = pattern.length()
    local i = start_pos
    loop(i < text_len) {
      if i + pattern_len > text_len { return -1 }
      local matches = 1
      local j = 0
      loop(j < pattern_len) {
        local text_ch = text.substring(i + j, i + j + 1)
        local pat_ch = pattern.substring(j, j + 1)
        if text_ch != pat_ch {
          matches = 0
          break
        }
        j = j + 1
      }
      if matches == 1 { return i }
      i = i + 1
    }
    return -1
  }

  // Simplified helper to read string after quote (replaces JsonFragBox.read_string_after)
  _read_string_after(text, start_pos) {
    local text_len = text.length()
    local i = start_pos
    local result = ""
    loop(i < text_len) {
      local ch = text.substring(i, i + 1)
      if ch == "\"" { break }
      result = result + ch
      i = i + 1
    }
    return result
  }

  collect_entries(src_unused) {
    // Simulate realistic JSON with both name and optional path
    local json = "[{\"name\":\"A\",\"path\":\"x\"},{\"name\":\"B\"}]"
    local out = new ArrayBox()
    local pos = 0
    local n = json.length()
    loop(pos < n) {
      local name_idx = me._find_from(json, "\"name\":\"", pos)
      if name_idx < 0 { break }
      local name = me._read_string_after(json, name_idx + 8)
      local obj_end = me._find_from(json, "}", name_idx)
      if obj_end < 0 { obj_end = n }

      local path = null
      local path_idx = me._find_from(json, "\"path\":\"", name_idx)
      if path_idx >= 0 && path_idx < obj_end {
        path = me._read_string_after(json, path_idx + 8)
      }

      local entry = new MapBox()
      entry.set("name", name)
      if path != null { entry.set("path", path) }
      out.push(entry)
      pos = obj_end + 1
    }
    return out
  }

  main() {
    local entries = me.collect_entries("")
    if entries == null { return 0 }
    return entries.length()
  }
}
"#;

    let ast: ASTNode = NyashParser::parse_from_string(src).expect("parse ok");
    let mut mc = MirCompiler::with_options(false);
    let cr = mc.compile(ast).expect("compile");

    // Dump MIR for analysis
    let printer = MirPrinter::verbose();
    let mir_output = printer.print_module(&cr.module);
    println!("=== MIR Dump ===");
    println!("{}", mir_output);
    println!("=== End MIR Dump ===");

    let mut verifier = MirVerifier::new();
    if let Err(errors) = verifier.verify_module(&cr.module) {
        for e in &errors {
            eprintln!("[rust-mir-verify] {}", e);
        }
        panic!("MIR verification failed for Stage1UsingResolverFull");
    }
}

/// Region+next_i パターンのループが SSA 崩れなく MIR 化できることを固定する軽量テスト。
#[test]
fn mir_stage1_using_resolver_region_loop_verifies() {
    ensure_stage3_env();
    let src = r#"
static box Stage1UsingResolverRegionLoop {
  process(entries) {
    local prefix = ""
    local i = 0
    local n = entries.length()
    loop(i < n) {
      local next_i = i + 1
      local entry = entries.get(i)
      local name = "" + entry.get("name")
      local ok = 1
      if name == "" { ok = 0 }
      if ok == 1 { prefix = prefix + name }
      i = next_i
    }
    return prefix
  }

  main() {
    local arr = new ArrayBox()
    local m1 = new MapBox(); m1.set("name", "A"); arr.push(m1)
    local m2 = new MapBox(); m2.set("name", ""); arr.push(m2)
    local m3 = new MapBox(); m3.set("name", "B"); arr.push(m3)
    return Stage1UsingResolverRegionLoop.process(arr)
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
        panic!("MIR verification failed for Stage1UsingResolverRegionLoop");
    }
}
