//! Test-only basic and legacy lowering.

use super::*;

#[test]
fn test_mir_dump() {
    let mut compiler = MirCompiler::new();

    let ast = ASTNode::Literal {
        value: LiteralValue::Integer(42),
        span: crate::ast::Span::unknown(),
    };

    let result = compiler.compile(ast).unwrap();
    let mir_dump = compiler.dump_mir(&result.module);

    assert!(!mir_dump.is_empty(), "MIR dump should not be empty");
    assert!(
        mir_dump.contains("define"),
        "MIR dump should contain function definition"
    );
}

#[test]
fn test_lowering_is_type_function_call_in_print() {
    // Build AST: print(isType(42, "Integer"))
    let ast = ASTNode::Print {
        expression: Box::new(ASTNode::FunctionCall {
            name: "isType".to_string(),
            arguments: vec![
                ASTNode::Literal {
                    value: LiteralValue::Integer(42),
                    span: crate::ast::Span::unknown(),
                },
                ASTNode::Literal {
                    value: LiteralValue::String("Integer".to_string()),
                    span: crate::ast::Span::unknown(),
                },
            ],
            span: crate::ast::Span::unknown(),
        }),
        span: crate::ast::Span::unknown(),
    };

    let mut compiler = MirCompiler::new();
    let result = compiler.compile(ast).expect("compile should succeed");

    // Ensure TypeOp exists in the resulting MIR
    let has_typeop = result.module.functions.values().any(|f| {
        f.blocks.values().any(|b| {
            b.all_spanned_instructions()
                .any(|sp| matches!(sp.inst, MirInstruction::TypeOp { .. }))
        })
    });
    assert!(
        has_typeop,
        "Expected TypeOp lowering for print(isType(...))"
    );
}

#[test]
fn test_lowering_is_method_call_in_print() {
    // Build AST: print( (42).is("Integer") )
    let ast = ASTNode::Print {
        expression: Box::new(ASTNode::MethodCall {
            object: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(42),
                span: crate::ast::Span::unknown(),
            }),
            method: "is".to_string(),
            arguments: vec![ASTNode::Literal {
                value: LiteralValue::String("Integer".to_string()),
                span: crate::ast::Span::unknown(),
            }],
            span: crate::ast::Span::unknown(),
        }),
        span: crate::ast::Span::unknown(),
    };

    let mut compiler = MirCompiler::new();
    let result = compiler.compile(ast).expect("compile should succeed");

    // Ensure TypeOp exists in the resulting MIR
    let has_typeop = result.module.functions.values().any(|f| {
        f.blocks.values().any(|b| {
            b.all_spanned_instructions()
                .any(|sp| matches!(sp.inst, MirInstruction::TypeOp { .. }))
        })
    });
    assert!(
        has_typeop,
        "Expected TypeOp lowering for print(obj.is(...))"
    );
}

#[test]
#[ignore = "MIR13 migration: extern console.log expectation pending"]
fn test_lowering_extern_console_log() {
    // Build AST: console.log("hi") → ExternCall env.console.log
    let ast = ASTNode::MethodCall {
        object: Box::new(ASTNode::Variable {
            name: "console".to_string(),
            span: crate::ast::Span::unknown(),
        }),
        method: "log".to_string(),
        arguments: vec![ASTNode::Literal {
            value: LiteralValue::String("hi".to_string()),
            span: crate::ast::Span::unknown(),
        }],
        span: crate::ast::Span::unknown(),
    };

    let mut compiler = MirCompiler::new();
    let result = compiler.compile(ast).expect("compile should succeed");
    let dump = MirPrinter::verbose().print_module(&result.module);

    assert!(
        dump.contains("extern_call env.console.log"),
        "Expected extern_call env.console.log in MIR dump. Got:\n{}",
        dump
    );
}

#[test]
fn test_lowering_boxcall_array_push() {
    // Build AST: (new ArrayBox()).push(1)
    let ast = ASTNode::MethodCall {
        object: Box::new(ASTNode::New {
            class: "ArrayBox".to_string(),
            arguments: vec![],
            field_initializers: vec![],
            type_arguments: vec![],
            span: crate::ast::Span::unknown(),
        }),
        method: "push".to_string(),
        arguments: vec![ASTNode::Literal {
            value: LiteralValue::Integer(1),
            span: crate::ast::Span::unknown(),
        }],
        span: crate::ast::Span::unknown(),
    };

    let mut compiler = MirCompiler::new();
    let result = compiler.compile(ast).expect("compile should succeed");
    let dump = MirPrinter::new().print_module(&result.module);
    // Known Array writes converge before downstream planners observe MIR.
    assert!(
        dump.contains("array.write #0 push"),
        "Expected canonical ArrayElementWrite push. Got:\n{}",
        dump
    );
}
