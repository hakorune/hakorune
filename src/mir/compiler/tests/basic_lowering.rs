//! Test-only basic and legacy lowering.

use super::*;

#[test]
fn test_mir_dump() {
    let mut compiler = MirCompiler::new();

    let ast = ASTNode::Program {
        statements: vec![ASTNode::Print {
            expression: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(42),
                span: crate::ast::Span::unknown(),
            }),
            span: crate::ast::Span::unknown(),
        }],
        span: crate::ast::Span::unknown(),
    };

    let result = crate::test_support::with_env_vars(&[("NYASH_MIR_UNIFIED_CALL", None)], || {
        compiler.compile(ast)
    }).unwrap();
    let mir_dump = compiler.dump_mir(&result.module);

    assert!(!mir_dump.is_empty(), "MIR dump should not be empty");
    assert!(
        mir_dump.contains("define"),
        "MIR dump should contain function definition"
    );
}

fn assert_print_lowers_typeop(expression: ASTNode, expectation: &str) {
    let ast = ASTNode::Program {
        statements: vec![ASTNode::Print {
            expression: Box::new(expression),
            span: crate::ast::Span::unknown(),
        }],
        span: crate::ast::Span::unknown(),
    };

    let mut compiler = MirCompiler::new();
    let result = crate::test_support::with_env_vars(&[("NYASH_MIR_UNIFIED_CALL", None)], || {
        compiler.compile(ast)
    })
    .expect("compile should succeed");

    // Ensure TypeOp exists in the resulting MIR
    let has_typeop = result.module.functions.values().any(|f| {
        f.blocks.values().any(|b| {
            b.all_spanned_instructions()
                .any(|sp| matches!(sp.inst, MirInstruction::TypeOp { .. }))
        })
    });
    assert!(has_typeop, "{expectation}");
}

#[test]
fn test_lowering_is_type_function_call_in_print() {
    let expression = ASTNode::FunctionCall {
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
    };
    assert_print_lowers_typeop(
        expression,
        "Expected TypeOp lowering for print(isType(...))",
    );
}

#[test]
fn test_lowering_is_method_call_in_print() {
    let expression = ASTNode::MethodCall {
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
    };
    assert_print_lowers_typeop(
        expression,
        "Expected TypeOp lowering for print(obj.is(...))",
    );
}

#[test]
fn test_lowering_boxcall_array_push() {
    // Build AST: (new ArrayBox()).push(1)
    let ast = ASTNode::Program {
        statements: vec![ASTNode::MethodCall {
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
        }],
        span: crate::ast::Span::unknown(),
    };

    let mut compiler = MirCompiler::new();
    let result = crate::test_support::with_env_vars(&[("NYASH_MIR_UNIFIED_CALL", None)], || {
        compiler.compile(ast)
    }).expect("compile should succeed");
    let dump = MirPrinter::new().print_module(&result.module);
    // Known Array writes converge before downstream planners observe MIR.
    assert!(
        dump.contains("array.write #0 push"),
        "Expected canonical ArrayElementWrite push. Got:\n{}",
        dump
    );
}

#[test]
fn compile_static_call_cohort_with_literal_and_computed_arguments() {
    for (first, second) in [("7", "7 + 1"), ("7 + 1", "7")] {
        let source = format!(
            "static box Layout {{ pick(p) {{ return 0 }} }} \
             static box Main {{ main() {{ local a = Layout.pick({first}) \
             local b = Layout.pick({second}) return 0 }} }}"
        );
        verify_callable_source(&source, "mixed-static-input-cohort.hako");
    }
}
