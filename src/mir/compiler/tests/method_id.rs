//! Test-only legacy method id observation.

use super::*;

#[test]
#[ignore = "MIR13 migration: method id naming in printer pending"]
fn test_boxcall_method_id_on_universal_slot() {
    // Build AST: (new ArrayBox()).toString()
    let ast = ASTNode::MethodCall {
        object: Box::new(ASTNode::New {
            class: "ArrayBox".to_string(),
            arguments: vec![],
            field_initializers: vec![],
            type_arguments: vec![],
            span: crate::ast::Span::unknown(),
        }),
        method: "toString".to_string(),
        arguments: vec![],
        span: crate::ast::Span::unknown(),
    };

    let mut compiler = MirCompiler::new();
    let result = compiler.compile(ast).expect("compile should succeed");
    let dump = MirPrinter::new().print_module(&result.module);
    // Expect a BoxCall with numeric method id [#0] for toString universal slot
    assert!(
        dump.contains("toString[#0]"),
        "Expected method_id #0 for toString. Dump:\n{}",
        dump
    );
}
