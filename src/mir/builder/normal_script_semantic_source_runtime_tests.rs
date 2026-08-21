use super::*;
use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::builder::raw_invocation_source_transport::{
    RawInvocationRootLineageV1, RawInvocationSourceContextV1, RawInvocationSourceTransportV1,
};
use crate::mir::builder::RawSourceLocatorV1;
use crate::mir::resolved_semantics::{
    FunctionSemanticResolverSessionV1, ResolveScriptOutcomeV1, ScriptRootRuntimeDispositionV1,
    ScriptRootSemanticDispositionV1, ScriptSyntaxViewV1, SourcePathSegmentV1, SourcePathV1,
    VerifiedScriptRootDemandEntryV1, VerifiedScriptRootDemandWindowV1,
};
use crate::mir::{MirCompiler, NormalCompileRequestV1};
use crate::parser::NyashParser;

fn assert_selected_parity(source: &str, hint: &str) {
    let mut legacy = MirCompiler::with_options(false);
    let legacy = legacy
        .compile_with_source(NyashParser::parse_from_string(source).unwrap(), Some(hint))
        .unwrap();
    let normal = MirCompiler::with_options(false)
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                NyashParser::parse_from_string(source).unwrap(),
                Some(hint),
                std::collections::HashMap::new(),
            )
            .unwrap(),
        )
        .unwrap();
    assert_eq!(
        crate::mir::MirPrinter::new().print_module(&normal.module),
        crate::mir::MirPrinter::new().print_module(&legacy.module)
    );
    assert_eq!(normal.verification_result, legacy.verification_result);
}

fn resolved_entry(index: u32) -> VerifiedScriptRootDemandEntryV1 {
    VerifiedScriptRootDemandEntryV1::new(
        SourcePathV1::program_body()
            .child(SourcePathSegmentV1::ProgramBody(index))
            .stmt(),
        ScriptRootSemanticDispositionV1::Resolved(
            crate::mir::resolved_semantics::ScriptRootResolvedDemandV1::LexicalCore,
        ),
        ScriptRootRuntimeDispositionV1::RetainedExistingTerminal,
    )
}

#[test]
fn scopebox_with_disabled_control_stays_deferred() {
    let program = ASTNode::Program {
        statements: vec![ASTNode::ScopeBox {
            body: vec![ASTNode::If {
                condition: Box::new(ASTNode::Literal {
                    value: LiteralValue::Bool(true),
                    span: Span::unknown(),
                }),
                then_body: Vec::new(),
                else_body: None,
                span: Span::unknown(),
            }],
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    };
    let source = PreparedNormalDefaultProgramRootV1::seal(program).expect("Program source");
    let window = VerifiedScriptRootDemandWindowV1::seal(vec![resolved_entry(0)], 1)
        .expect("ScopeBox window");
    let mut resolver = FunctionSemanticResolverSessionV1::new(0).expect("resolver");
    let view = ScriptSyntaxViewV1::from_program(source.source_ast()).expect("Script view");
    assert!(matches!(
        resolver
            .resolve_script(view, &window)
            .expect("ScopeBox resolve"),
        ResolveScriptOutcomeV1::Deferred(_),
    ));
}

#[test]
fn scopebox_local_does_not_leak_outside_its_lexical_body() {
    let program = ASTNode::Program {
        statements: vec![
            ASTNode::ScopeBox {
                body: vec![ASTNode::Local {
                    variables: vec!["x".to_owned()],
                    initial_values: vec![Some(Box::new(ASTNode::Literal {
                        value: LiteralValue::Integer(1),
                        span: Span::unknown(),
                    }))],
                    declared_type_names: vec![None],
                    span: Span::unknown(),
                }],
                span: Span::unknown(),
            },
            ASTNode::Print {
                expression: Box::new(ASTNode::Variable {
                    name: "x".to_owned(),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            },
        ],
        span: Span::unknown(),
    };
    let mut legacy = MirCompiler::with_options(false);
    let legacy_error = legacy
        .compile_with_source(program.clone(), Some("script-scopebox-leak.hako"))
        .expect_err("ScopeBox local cannot leak");
    let normal_error = MirCompiler::with_options(false)
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                program,
                Some("script-scopebox-leak.hako"),
                std::collections::HashMap::new(),
            )
            .expect("normal request"),
        )
        .expect_err("ScopeBox local must defer to the existing diagnostic");
    assert_eq!(normal_error, legacy_error);
}

#[test]
fn lexical_nowait_matches_legacy_and_records_future_binding() {
    assert_selected_parity(
        "nowait pending = 41 + 1\nawait pending",
        "script-nowait-lexical.hako",
    );
}

#[test]
fn nowait_with_disabled_operand_stays_deferred() {
    let program = ASTNode::Program {
        statements: vec![ASTNode::Nowait {
            variable: "pending".to_owned(),
            expression: Box::new(ASTNode::This {
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    };
    let source = PreparedNormalDefaultProgramRootV1::seal(program).expect("Program source");
    let window =
        VerifiedScriptRootDemandWindowV1::seal(vec![resolved_entry(0)], 1).expect("Nowait window");
    let mut resolver = FunctionSemanticResolverSessionV1::new(0).expect("resolver");
    let view = ScriptSyntaxViewV1::from_program(source.source_ast()).expect("Script view");
    assert!(matches!(
        resolver
            .resolve_script(view, &window)
            .expect("Nowait resolve"),
        ResolveScriptOutcomeV1::Deferred(_),
    ));
}

#[test]
fn script_transport_keeps_explicit_root_and_program_item_receipts() {
    let (_, script) = RawInvocationSourceContextV1::from_transport(
        RawInvocationSourceTransportV1::script_root(Vec::<ASTNode>::new()),
    );
    let RawInvocationSourceContextV1::Located {
        site, body_kind, ..
    } = script
    else {
        panic!("script root must be located");
    };
    assert_eq!(site.segments(), &[SourcePathSegmentV1::ProgramBodyRoot]);
    assert_eq!(
        body_kind,
        Some(crate::mir::resolved_semantics::SourceBodyKindV1::Program)
    );

    let (_, function) =
        RawInvocationSourceContextV1::from_transport(RawInvocationSourceTransportV1::root(
            Vec::<ASTNode>::new(),
            RawInvocationRootLineageV1::Main(RawSourceLocatorV1::for_test(
                0,
                "Main",
                "main",
                "Main.main/0",
                0,
            )),
        ));
    let RawInvocationSourceContextV1::Located { body_kind, .. } = function else {
        panic!("function root must be located");
    };
    assert_eq!(
        body_kind,
        Some(crate::mir::resolved_semantics::SourceBodyKindV1::Function)
    );

    let (_, root) = RawInvocationSourceContextV1::from_transport(
        RawInvocationSourceTransportV1::script_semantic_root(Vec::<ASTNode>::new()),
    );
    let (_, child) = RawInvocationSourceContextV1::from_transport(root.body_statement(
        ASTNode::StaticConstTable {
            name: "TABLE".to_owned(),
            element_type: "u16".to_owned(),
            values: vec![1, 2, 3],
            span: Span::unknown(),
        },
        2,
    ));
    assert_eq!(
        child.site().expect("located Program item").segments(),
        &[
            SourcePathSegmentV1::ProgramBodyRoot,
            SourcePathSegmentV1::ProgramBody(2),
        ]
    );
}

#[test]
fn fastmem_weak_child_remains_deferred_before_name_resolution() {
    let mut compiler = MirCompiler::with_options(false);
    let weak_missing = ASTNode::UnaryOp {
        operator: crate::ast::UnaryOperator::Weak,
        operand: Box::new(ASTNode::Variable {
            name: "missing".to_owned(),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };
    let program = ASTNode::Program {
        statements: vec![ASTNode::FastMemRegion {
            contract: "PageMapV0".to_owned(),
            body: vec![weak_missing],
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    };
    let error = compiler
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                program,
                Some("script-fastmem-weak.hako"),
                std::collections::HashMap::new(),
            )
            .unwrap(),
        )
        .expect_err("Weak FastMem child must use the existing lower route");
    assert!(error.contains("Undefined variable: missing"), "{error}");
}

#[test]
fn selected_and_or_failure_discards_candidate_and_reuses_compiler() {
    let mut compiler = MirCompiler::with_options(false);
    let error = compiler
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                NyashParser::parse_from_string("local x = true\nprint(x and missing)").unwrap(),
                Some("script-failure.hako"),
                std::collections::HashMap::new(),
            )
            .unwrap(),
        )
        .expect_err("undefined name must reject");
    assert!(error.contains("Undefined variable: missing"), "{error}");
    compiler
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                NyashParser::parse_from_string("print(1)").unwrap(),
                Some("script-reuse.hako"),
                std::collections::HashMap::new(),
            )
            .unwrap(),
        )
        .expect("fresh request succeeds");
}

#[test]
fn script_static_const_u16_completion_matches_legacy_metadata() {
    assert_selected_parity(
        "static const TABLE: u16[] = [1, 2, 3]\nprint(1)",
        "script-static.hako",
    );
}

#[test]
fn weak_unary_remains_deferred() {
    let mut compiler = MirCompiler::with_options(false);
    let error = compiler
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                NyashParser::parse_from_string("weak missing").unwrap(),
                Some("script-weak.hako"),
                std::collections::HashMap::new(),
            )
            .unwrap(),
        )
        .expect_err("weak stays on existing lower route");
    assert!(error.contains("Undefined variable: missing"), "{error}");
}
