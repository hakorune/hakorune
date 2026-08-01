use crate::ast::{ASTNode, DeclarationAttrs, FieldDecl, LiteralValue, Span};
use crate::mir::{MirCompiler, MirPrinter, NormalCompileRequestV1};
use std::collections::HashMap;

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn record_declaration(name: &str, fields: Vec<FieldDecl>) -> ASTNode {
    ASTNode::BoxDeclaration {
        name: name.to_owned(),
        fields: Vec::new(),
        field_decls: fields,
        public_fields: Vec::new(),
        private_fields: Vec::new(),
        methods: HashMap::new(),
        constructors: HashMap::new(),
        init_fields: Vec::new(),
        weak_fields: Vec::new(),
        delegates: Vec::new(),
        invariants: Vec::new(),
        transitions: Vec::new(),
        is_interface: false,
        is_record: true,
        extends: Vec::new(),
        implements: Vec::new(),
        type_parameters: Vec::new(),
        is_sync: false,
        is_static: false,
        static_init: None,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn field(name: &str, default_value: Option<ASTNode>) -> FieldDecl {
    FieldDecl {
        name: name.to_owned(),
        declared_type_name: Some("i64".to_owned()),
        is_weak: false,
        default_value: default_value.map(Box::new),
    }
}

fn record_literal(name: &str, fields: Vec<(&str, ASTNode)>) -> ASTNode {
    ASTNode::RecordLiteral {
        record_type_name: name.to_owned(),
        fields: fields
            .into_iter()
            .map(|(field_name, value)| (field_name.to_owned(), value))
            .collect(),
        span: Span::unknown(),
    }
}

fn assert_selected_parity(program: ASTNode, hint: &str) {
    let mut legacy = MirCompiler::with_options(false);
    let legacy = legacy
        .compile_with_source(program.clone(), Some(hint))
        .expect("legacy compile");
    let normal = MirCompiler::with_options(false)
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(program, Some(hint), HashMap::new())
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
fn fully_explicit_record_literal_matches_legacy_with_lexical_values() {
    assert_selected_parity(
        ASTNode::Program {
            statements: vec![
                record_declaration("Pair", vec![field("left", None), field("right", None)]),
                ASTNode::Local {
                    variables: vec!["x".to_owned()],
                    initial_values: vec![Some(Box::new(integer(1)))],
                    declared_type_names: vec![None],
                    span: Span::unknown(),
                },
                ASTNode::Print {
                    expression: Box::new(record_literal(
                        "Pair",
                        vec![
                            ("right", integer(2)),
                            (
                                "left",
                                ASTNode::Variable {
                                    name: "x".to_owned(),
                                    span: Span::unknown(),
                                },
                            ),
                        ],
                    )),
                    span: Span::unknown(),
                },
            ],
            span: Span::unknown(),
        },
        "script-fully-explicit-record.hako",
    );
}

#[test]
fn omitted_default_record_literal_keeps_existing_lowering_parity() {
    let program = ASTNode::Program {
        statements: vec![
            record_declaration("Pair", vec![field("value", Some(integer(9)))]),
            ASTNode::Print {
                expression: Box::new(record_literal("Pair", Vec::new())),
                span: Span::unknown(),
            },
        ],
        span: Span::unknown(),
    };
    assert_selected_parity(program, "script-record-defaulted.hako");
}

#[test]
fn invalid_record_literal_defers_and_allows_a_fresh_normal_request() {
    let mut compiler = MirCompiler::with_options(false);
    let error = compiler
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                ASTNode::Program {
                    statements: vec![record_literal("Missing", vec![("value", integer(1))])],
                    span: Span::unknown(),
                },
                Some("script-record-missing.hako"),
                HashMap::new(),
            )
            .expect("normal request"),
        )
        .expect_err("unknown record must retain the existing RootLower diagnostic");
    assert!(
        error.contains("[type/record_contract_unknown_record] record=Missing"),
        "{error}"
    );

    compiler
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                ASTNode::Program {
                    statements: vec![
                        record_declaration("Pair", vec![field("value", None)]),
                        record_literal("Pair", vec![("value", integer(1))]),
                    ],
                    span: Span::unknown(),
                },
                Some("script-record-fresh.hako"),
                HashMap::new(),
            )
            .expect("fresh normal request"),
        )
        .expect("fresh request must not reuse a deferred Script semantic product");
}
