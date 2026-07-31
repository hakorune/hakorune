use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::{MirCompiler, MirPrinter, NormalCompileRequestV1};

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_owned(),
        span: Span::unknown(),
    }
}

fn local(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::Local {
        variables: vec![name.to_owned()],
        initial_values: vec![Some(Box::new(value))],
        declared_type_names: vec![None],
        span: Span::unknown(),
    }
}

fn array(elements: Vec<ASTNode>) -> ASTNode {
    ASTNode::ArrayLiteral {
        elements,
        span: Span::unknown(),
    }
}

#[test]
fn prior_local_array_elements_use_complete_script_sources_and_match_legacy() {
    let program = ASTNode::Program {
        statements: vec![
            local("x", integer(1)),
            array(vec![array(vec![variable("x")])]),
        ],
        span: Span::unknown(),
    };

    let mut legacy = MirCompiler::with_options(false);
    let legacy = legacy
        .compile_with_source(program.clone(), Some("script-array-lexical.hako"))
        .expect("legacy compile");
    let normal = MirCompiler::with_options(false)
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                program,
                Some("script-array-lexical.hako"),
                std::collections::HashMap::new(),
            )
            .expect("normal request"),
        )
        .expect("normal compile");

    assert_eq!(
        MirPrinter::new().print_module(&normal.module),
        MirPrinter::new().print_module(&legacy.module)
    );
    assert_eq!(normal.verification_result, legacy.verification_result);
}

#[test]
fn missing_array_element_defers_to_existing_root_lower_and_allows_fresh_reuse() {
    let failing_program = ASTNode::Program {
        statements: vec![array(vec![variable("missing")])],
        span: Span::unknown(),
    };
    let mut compiler = MirCompiler::with_options(false);
    let error = compiler
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                failing_program,
                Some("script-array-missing.hako"),
                std::collections::HashMap::new(),
            )
            .expect("normal request"),
        )
        .expect_err("missing Array element must keep the existing RootLower diagnostic");
    assert!(error.contains("Undefined variable: missing"), "{error}");

    compiler
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                ASTNode::Program {
                    statements: vec![array(vec![integer(1)])],
                    span: Span::unknown(),
                },
                Some("script-array-reuse.hako"),
                std::collections::HashMap::new(),
            )
            .expect("fresh request"),
        )
        .expect("fresh request must not reuse a rejected Script semantic product");
}
