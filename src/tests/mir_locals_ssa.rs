/*!
 * Test for MIR locals SSA form with Copy instructions
 *
 * This test verifies that local variable declarations emit Copy instructions
 * to establish proper SSA form.
 */

use crate::ast::ASTNode;
use crate::mir::printer::MirPrinter;
use crate::mir::MirCompiler;
use crate::parser::NyashParser;

#[test]
fn mir_locals_copy_instructions_emitted() {
    std::env::set_var("NYASH_FEATURES", "stage3");
    std::env::set_var("NYASH_PARSER_ALLOW_SEMICOLON", "1");

    let src = r#"
static box TestLocals {
  main() {
    local a = 1
    local b = 2
    local c = new ArrayBox()
    return 0
  }
}
"#;

    eprintln!("=== Parsing source ===");
    let ast: ASTNode = NyashParser::parse_from_string(src).expect("parse ok");

    eprintln!("=== AST Debug ===");
    eprintln!("{:#?}", ast);

    eprintln!("=== Compiling to MIR ===");
    let mut mc = MirCompiler::with_options(false);
    let cr = mc.compile(ast).expect("compile");

    let printer = MirPrinter::verbose();
    let mir_output = printer.print_module(&cr.module);

    eprintln!("=== MIR Dump ===");
    eprintln!("{}", mir_output);

    // Check if Copy instructions are present
    let has_copy = mir_output.contains("copy");
    eprintln!("=== Copy instruction check: {} ===", has_copy);

    // Count the number of "copy" occurrences
    let copy_count = mir_output.matches("copy").count();
    eprintln!("=== Number of copy instructions: {} ===", copy_count);

    // Check for SSA violations (multiple definitions of same register)
    let lines: Vec<&str> = mir_output.lines().collect();
    let mut defined_regs = std::collections::HashSet::new();
    let mut violations = Vec::new();

    for line in &lines {
        // Look for register definitions: %N =
        if let Some(pos) = line.find('%') {
            if let Some(eq_pos) = line.find('=') {
                if eq_pos > pos {
                    let reg_part = &line[pos..eq_pos].trim();
                    if let Some(space_pos) = reg_part.find(' ') {
                        let reg = &reg_part[..space_pos].trim();
                        if !defined_regs.insert(reg.to_string()) {
                            violations.push(format!("Register {} defined multiple times", reg));
                        }
                    }
                }
            }
        }
    }

    if !violations.is_empty() {
        eprintln!("=== SSA Violations Found ===");
        for v in &violations {
            eprintln!("  {}", v);
        }
    }

    // Assert Copy instructions are present
    assert!(
        has_copy,
        "MIR should contain Copy instructions for local variable initializations"
    );

    // Assert no SSA violations
    assert!(
        violations.is_empty(),
        "MIR should not have SSA violations: {:?}",
        violations
    );

    // We expect at least 3 copy instructions (for a, b, c)
    assert!(
        copy_count >= 3,
        "Expected at least 3 copy instructions, found {}",
        copy_count
    );
}

#[test]
fn mir_locals_uninitialized() {
    std::env::set_var("NYASH_FEATURES", "stage3");
    std::env::set_var("NYASH_PARSER_ALLOW_SEMICOLON", "1");

    let src = r#"
static box TestUninit {
  main() {
    local x
    local y
    return 0
  }
}
"#;

    let ast: ASTNode = NyashParser::parse_from_string(src).expect("parse ok");
    let mut mc = MirCompiler::with_options(false);
    let cr = mc.compile(ast).expect("compile");

    let printer = MirPrinter::verbose();
    let mir_output = printer.print_module(&cr.module);

    eprintln!("=== MIR for uninitialized locals ===");
    eprintln!("{}", mir_output);

    // Uninitialized locals should have void constants, not copy
    assert!(
        mir_output.contains("const void"),
        "Uninitialized locals should have void constants"
    );
}
