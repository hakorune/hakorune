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

fn map(entries: Vec<(&str, ASTNode)>) -> ASTNode {
    ASTNode::MapLiteral {
        entries: entries
            .into_iter()
            .map(|(key, value)| (key.to_owned(), value))
            .collect(),
        span: Span::unknown(),
    }
}

#[test]
fn prior_local_map_values_use_complete_script_sources_and_match_legacy() {
    let program = ASTNode::Program {
        statements: vec![
            local("x", integer(1)),
            map(vec![
                ("outer", variable("x")),
                ("nested", map(vec![("inner", variable("x"))])),
            ]),
        ],
        span: Span::unknown(),
    };
    let mut legacy = MirCompiler::with_options(false);
    let legacy = legacy
        .compile_with_source(program.clone(), Some("script-map-lexical.hako"))
        .expect("legacy Map");
    let normal = MirCompiler::with_options(false)
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                program,
                Some("script-map-lexical.hako"),
                std::collections::HashMap::new(),
            )
            .expect("normal request"),
        )
        .expect("selected Map");
    assert_eq!(
        MirPrinter::new().print_module(&normal.module),
        MirPrinter::new().print_module(&legacy.module)
    );
    assert_eq!(normal.verification_result, legacy.verification_result);
}

#[test]
fn missing_map_value_stays_deferred_and_allows_fresh_reuse() {
    let mut compiler = MirCompiler::with_options(false);
    let error = compiler
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                ASTNode::Program {
                    statements: vec![map(vec![("missing", variable("missing"))])],
                    span: Span::unknown(),
                },
                Some("script-map-missing.hako"),
                std::collections::HashMap::new(),
            )
            .expect("normal request"),
        )
        .expect_err("missing Map value must retain RootLower diagnostic");
    assert!(error.contains("Undefined variable: missing"), "{error}");

    compiler
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                ASTNode::Program {
                    statements: vec![map(vec![("ok", integer(1))])],
                    span: Span::unknown(),
                },
                Some("script-map-reuse.hako"),
                std::collections::HashMap::new(),
            )
            .expect("fresh request"),
        )
        .expect("fresh request must not reuse a Deferred Map product");
}
