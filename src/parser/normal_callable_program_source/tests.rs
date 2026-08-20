use crate::ast::{ASTNode, LiteralValue, Span};
use crate::parser::{BuildMode, NyashParser, ParserBuildConfig};

use super::*;

fn parse(source: &str) -> ParsedNormalCallableProgramV1 {
    NyashParser::parse_normal_callable_program_with_build_config(
        source,
        ParserBuildConfig::default(),
    )
    .expect("normal callable source")
}

fn transform(
    parsed: ParsedNormalCallableProgramV1,
    mutate: impl FnOnce(&mut ASTNode),
) -> Result<VerifiedFinalCallableProgramSourceV1, FinalCallableProgramSourceRejectV1> {
    let ParsedNormalCallableProgramV1::SourceBacked(initial) = parsed else {
        panic!("fixture must be source-backed")
    };
    let mut output = initial.ast().clone();
    mutate(&mut output);
    issue_final_callable_program_source_v1(initial, output)
}

#[test]
fn exact_static_callable_set_survives_one_transform() {
    let final_source = transform(
        parse("static box Scan { run(x, pos: i64, end: i64, y) { return x } }"),
        |_| {},
    )
    .expect("exact transform");
    assert_eq!(final_source.callable_count(), 1);
    let parameter_types = final_source
        .with_callable_semantic_syntax(|loan| {
            loan.rows()[0]
                .parameters()
                .expect("direct method parameter source")
                .iter()
                .map(|parameter| parameter.declared_type_name().map(str::to_owned))
                .collect::<Vec<_>>()
        })
        .expect("semantic syntax loan");
    assert_eq!(
        parameter_types,
        [None, Some("i64".to_owned()), Some("i64".to_owned()), None]
    );
    let parameter_count = final_source
        .with_callable_parameter_syntax(|catalog, loan| {
            assert_eq!(catalog.declarations().len(), 1);
            assert_eq!(loan.declarations().len(), 1);
            catalog.declarations()[0].parameters().len()
        })
        .expect("exact parameter syntax")
        .expect("direct method parameter source");
    assert_eq!(parameter_count, 4);
    let constructor_count = final_source
        .with_constructor_semantic_syntax(|loan| loan.rows().len())
        .expect("static-only source must carry an empty constructor catalog");
    assert_eq!(constructor_count, 0);
}

#[test]
fn mixed_compatibility_source_carries_constructor_catalog_without_widening_cohort() {
    let final_source = transform(
        parse("box Plain { run() { return 1 } }\nstatic box Utility { ping() { return 2 } }"),
        |_| {},
    )
    .expect("mixed source-backed callable source");
    assert_eq!(final_source.callable_count(), 2);
    let constructor_count = final_source
        .with_constructor_semantic_syntax(|loan| loan.rows().len())
        .expect("mixed source must carry the parser-owned catalog");
    assert_eq!(constructor_count, 0);
}

#[test]
fn ordinary_constructor_source_catalog_survives_normal_source_transform() {
    let final_source = transform(
        parse("box Page { init(a) {} pack(b) {} birth() {} }"),
        |_| {},
    )
    .expect("ordinary constructor source");
    let keys = final_source
        .with_constructor_semantic_syntax(|loan| {
            loan.rows()
                .iter()
                .map(|row| row.key().to_owned())
                .collect::<Vec<_>>()
        })
        .expect("constructor semantic syntax loan");
    assert_eq!(keys, ["init/1", "pack/1", "birth/0"]);
}

#[test]
fn unsupported_compatibility_cohorts_do_not_enter_initial_source_lane() {
    for source in ["interface box Api { run() }", "record Data { value: i64 }"] {
        assert!(matches!(
            parse(source),
            ParsedNormalCallableProgramV1::Compatibility { .. }
        ));
    }
}

#[test]
fn top_level_callable_does_not_fabricate_parameter_source() {
    let final_source = transform(parse("function helper(pos: i64) { return pos }"), |_| {})
        .expect("exact transform");
    final_source
        .with_callable_semantic_syntax(|loan| {
            assert!(loan.rows()[0].parameters().is_none());
        })
        .expect("semantic syntax loan");
}

#[test]
fn direct_instance_method_carries_one_co_sealed_source_observation() {
    let final_source =
        transform(parse("box Scan { run(x) { return x } }"), |_| {}).expect("exact transform");
    final_source
        .with_callable_semantic_syntax(|loan| {
            let row = loan.rows().first().expect("method row");
            let observation = row
                .method_source_observation()
                .expect("direct method observation");
            assert_eq!(observation.source_site().box_statement_ordinal(), 0);
            assert_eq!(observation.source_site().member_ordinal(), 0);
            assert!(observation.identity().same_as(row.identity()));
        })
        .expect("semantic syntax loan");
}

#[test]
fn selected_member_gate_retains_callable_anchors_without_forging_parameter_source() {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        "box Choice { gate Build.test { run(x) { return x } } else { run(x) { return x } } }",
        ParserBuildConfig {
            mode: BuildMode::Test,
            ..ParserBuildConfig::default()
        },
    )
    .expect("selected member gate source");
    let final_source = transform(parsed, |_| {}).expect("exact gate transform");
    assert_eq!(final_source.callable_count(), 1);
    assert!(final_source
        .with_callable_parameter_syntax(|_, _| ())
        .expect("typed unavailable disposition")
        .is_none());
    final_source
        .with_callable_semantic_syntax(|loan| {
            assert_eq!(loan.rows().len(), 1);
            assert!(loan.rows()[0].method_source_observation().is_none());
        })
        .expect("semantic syntax loan");
}

#[test]
fn non_callable_tail_may_change_without_reissuing_callable_identity() {
    let final_source = transform(parse("static box Scan { run(x) { return x } }"), |ast| {
        let ASTNode::Program { statements, .. } = ast else {
            unreachable!()
        };
        statements.push(ASTNode::Literal {
            value: LiteralValue::Integer(1),
            span: Span::unknown(),
        });
    })
    .expect("tail does not change callable set");
    assert_eq!(final_source.callable_count(), 1);
}

#[test]
fn added_or_changed_callable_rejects_without_compatibility_fallback() {
    let added = transform(parse("static box Scan { run(x) { return x } }"), |ast| {
        let ASTNode::Program { statements, .. } = ast else {
            unreachable!()
        };
        statements.push(ASTNode::FunctionDeclaration {
            name: "extra".to_owned(),
            params: Vec::new(),
            param_decls: Vec::new(),
            return_type_name: None,
            body: Vec::new(),
            is_static: false,
            is_override: false,
            attrs: crate::ast::DeclarationAttrs::default(),
            uses: Vec::new(),
            contracts: Vec::new(),
            span: Span::unknown(),
        });
    });
    assert!(added.is_err(), "added callable must not retain old anchors");

    let changed = transform(parse("static box Scan { run(x) { return x } }"), |ast| {
        let ASTNode::Program { statements, .. } = ast else {
            unreachable!()
        };
        let ASTNode::BoxDeclaration { methods, .. } = &mut statements[0] else {
            unreachable!()
        };
        *methods = std::mem::take(methods)
            .map_declarations(|mut declaration| {
                let ASTNode::FunctionDeclaration { body, .. } = &mut declaration else {
                    unreachable!()
                };
                body.clear();
                declaration
            })
            .expect("valid transformed inventory");
    });
    assert!(matches!(
        changed,
        Err(FinalCallableProgramSourceRejectV1::CallableDeclarationChanged { row: 0 })
    ));

    let changed_parameter_type = transform(
        parse("static box Scan { run(value: i64) { return value } }"),
        |ast| {
            let ASTNode::Program { statements, .. } = ast else {
                unreachable!()
            };
            let ASTNode::BoxDeclaration { methods, .. } = &mut statements[0] else {
                unreachable!()
            };
            *methods = std::mem::take(methods)
                .map_declarations(|mut declaration| {
                    let ASTNode::FunctionDeclaration { param_decls, .. } = &mut declaration else {
                        unreachable!()
                    };
                    param_decls[0].declared_type_name = None;
                    declaration
                })
                .expect("valid transformed inventory");
        },
    );
    assert!(matches!(
        changed_parameter_type,
        Err(FinalCallableProgramSourceRejectV1::CallableDeclarationChanged { row: 0 })
    ));
}
