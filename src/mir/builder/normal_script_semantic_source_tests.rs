use super::VerifiedScriptSemanticSourceV1;
use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::builder::raw_invocation_source_transport::{
    RawInvocationRootLineageV1, RawInvocationSourceContextV1, RawInvocationSourceTransportV1,
};
use crate::mir::builder::PreparedNormalDefaultProgramRootV1;
use crate::mir::builder::RawSourceLocatorV1;
use crate::mir::resolved_semantics::{
    FunctionSemanticResolverSessionV1, ResolveScriptOutcomeV1, ScriptDiagnosticBoundaryV1,
    ScriptRootRuntimeDispositionV1, ScriptRootSemanticDispositionV1, ScriptSyntaxViewV1,
    ScriptTransparentBoundaryV1, SourcePathSegmentV1, SourcePathV1,
    VerifiedScriptRootDemandEntryV1, VerifiedScriptRootDemandWindowV1,
};
use crate::mir::{MirCompiler, MirPrinter, NormalCompileRequestV1};
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
        MirPrinter::new().print_module(&normal.module),
        MirPrinter::new().print_module(&legacy.module)
    );
    assert_eq!(normal.verification_result, legacy.verification_result);
}
fn assert_selected_program_parity(program: ASTNode, hint: &str) {
    let mut legacy = MirCompiler::with_options(false);
    let legacy = legacy
        .compile_with_source(program.clone(), Some(hint))
        .unwrap();
    let normal = MirCompiler::with_options(false)
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                program,
                Some(hint),
                std::collections::HashMap::new(),
            )
            .unwrap(),
        )
        .unwrap();
    assert_eq!(
        MirPrinter::new().print_module(&normal.module),
        MirPrinter::new().print_module(&legacy.module)
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

fn root_if_entry(index: u32) -> VerifiedScriptRootDemandEntryV1 {
    VerifiedScriptRootDemandEntryV1::new(
        SourcePathV1::program_body()
            .child(SourcePathSegmentV1::ProgramBody(index))
            .stmt(),
        ScriptRootSemanticDispositionV1::Resolved(
            crate::mir::resolved_semantics::ScriptRootResolvedDemandV1::IfControl(
                crate::mir::resolved_semantics::ScriptRootIfControlAdmissionV1::new(),
            ),
        ),
        ScriptRootRuntimeDispositionV1::RetainedExistingTerminal,
    )
}

#[test]
fn root_if_control_receipt_seals_one_script_owner_and_matches_legacy() {
    let program = ASTNode::Program {
        statements: vec![ASTNode::If {
            condition: Box::new(ASTNode::Literal {
                value: LiteralValue::Bool(true),
                span: Span::unknown(),
            }),
            then_body: vec![ASTNode::Print {
                expression: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }],
            else_body: Some(vec![ASTNode::Print {
                expression: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(2),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }]),
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    };
    let source = PreparedNormalDefaultProgramRootV1::seal(program.clone()).expect("Program source");
    let window =
        VerifiedScriptRootDemandWindowV1::seal(vec![root_if_entry(0)], 1).expect("If window");
    let mut resolver = FunctionSemanticResolverSessionV1::new(0).expect("resolver");
    let view = ScriptSyntaxViewV1::from_program(source.source_ast()).expect("Script view");
    assert!(matches!(
        resolver
            .resolve_script(view, &window)
            .expect("root If resolve"),
        ResolveScriptOutcomeV1::Complete(_)
    ));
    assert_selected_program_parity(program, "script-root-if-control.hako");
}

fn receiver_absent_entry(index: u32) -> VerifiedScriptRootDemandEntryV1 {
    VerifiedScriptRootDemandEntryV1::new(
        SourcePathV1::program_body()
            .child(SourcePathSegmentV1::ProgramBody(index))
            .stmt(),
        ScriptRootSemanticDispositionV1::Diagnostic(
            ScriptDiagnosticBoundaryV1::ExistingReceiverAbsent,
        ),
        ScriptRootRuntimeDispositionV1::RetainedExistingTerminal,
    )
}

fn using_directive_entry(index: u32) -> VerifiedScriptRootDemandEntryV1 {
    VerifiedScriptRootDemandEntryV1::new(
        SourcePathV1::program_body()
            .child(SourcePathSegmentV1::ProgramBody(index))
            .stmt(),
        ScriptRootSemanticDispositionV1::Transparent(ScriptTransparentBoundaryV1::UsingDirective),
        ScriptRootRuntimeDispositionV1::RetainedExistingTerminal,
    )
}

fn bare_this_unsupported_entry(index: u32) -> VerifiedScriptRootDemandEntryV1 {
    VerifiedScriptRootDemandEntryV1::new(
        SourcePathV1::program_body()
            .child(SourcePathSegmentV1::ProgramBody(index))
            .stmt(),
        ScriptRootSemanticDispositionV1::Diagnostic(
            ScriptDiagnosticBoundaryV1::ExistingBareThisUnsupported,
        ),
        ScriptRootRuntimeDispositionV1::RetainedExistingTerminal,
    )
}

fn context_scope_unsupported_entry(index: u32) -> VerifiedScriptRootDemandEntryV1 {
    VerifiedScriptRootDemandEntryV1::new(
        SourcePathV1::program_body()
            .child(SourcePathSegmentV1::ProgramBody(index))
            .stmt(),
        ScriptRootSemanticDispositionV1::Diagnostic(
            ScriptDiagnosticBoundaryV1::ExistingContextScopeUnsupported,
        ),
        ScriptRootRuntimeDispositionV1::RetainedExistingTerminal,
    )
}

#[test]
fn literal_program_seals_one_shared_script_owner_and_projection() {
    let ast = NyashParser::parse_from_string("0").expect("literal source");
    let source = PreparedNormalDefaultProgramRootV1::seal(ast).expect("Program source");
    let window = VerifiedScriptRootDemandWindowV1::seal(vec![resolved_entry(0)], 1)
        .expect("total source window");
    let mut resolver = FunctionSemanticResolverSessionV1::new(0).expect("resolver");
    let view = ScriptSyntaxViewV1::from_program(source.source_ast()).expect("Script view");
    let ResolveScriptOutcomeV1::Complete(owner) = resolver
        .resolve_script(view, &window)
        .expect("Script resolve")
    else {
        panic!("literal Script must Complete");
    };
    let product = VerifiedScriptSemanticSourceV1::seal(&source, owner, &window)
        .expect("Script source product");

    assert_eq!(product.forest().owner_count(), 1);
    assert_eq!(product.runtime_source_indices(), &[0]);
    assert!(product
        .projection()
        .owner_root(source.source_ast(), product.forest().roots()[0])
        .is_ok());
}

#[test]
fn bare_me_seals_script_source_then_uses_existing_rootlower_diagnostic() {
    let program = ASTNode::Program {
        statements: vec![ASTNode::Me {
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    };
    let source = PreparedNormalDefaultProgramRootV1::seal(program.clone()).expect("Program source");
    let window = VerifiedScriptRootDemandWindowV1::seal(vec![receiver_absent_entry(0)], 1)
        .expect("bare Me window");
    let mut resolver = FunctionSemanticResolverSessionV1::new(0).expect("resolver");
    let view = ScriptSyntaxViewV1::from_program(source.source_ast()).expect("Script view");
    let ResolveScriptOutcomeV1::Complete(owner) = resolver
        .resolve_script(view, &window)
        .expect("bare Me resolve")
    else {
        panic!("bare Me is a zero-child diagnostic boundary");
    };
    let product = VerifiedScriptSemanticSourceV1::seal(&source, owner, &window)
        .expect("bare Me Script source");
    assert_eq!(product.forest().owner_count(), 1);
    assert_eq!(product.receiver_absent_sites().count(), 1);

    let mut legacy = MirCompiler::with_options(false);
    let legacy_error = legacy
        .compile_with_source(program.clone(), Some("script-bare-me.hako"))
        .expect_err("legacy bare Me rejects through the existing owner");
    let mut normal = MirCompiler::with_options(false);
    let normal_error = normal
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                program,
                Some("script-bare-me.hako"),
                std::collections::HashMap::new(),
            )
            .expect("normal request"),
        )
        .expect_err("normal bare Me rejects through the existing owner");
    assert_eq!(normal_error, legacy_error);
    normal
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                NyashParser::parse_from_string("0").expect("reuse source"),
                Some("script-bare-me-reuse.hako"),
                std::collections::HashMap::new(),
            )
            .expect("reuse request"),
        )
        .expect("fresh request succeeds");
}

#[test]
fn using_directive_is_complete_and_retains_void_runtime_completion() {
    let using = ASTNode::UsingStatement {
        namespace_name: "std.math".to_owned(),
        span: Span::unknown(),
    };
    let source = PreparedNormalDefaultProgramRootV1::seal(ASTNode::Program {
        statements: vec![using.clone()],
        span: Span::unknown(),
    })
    .expect("Program source");
    let window = VerifiedScriptRootDemandWindowV1::seal(vec![using_directive_entry(0)], 1)
        .expect("Using window");
    let mut resolver = FunctionSemanticResolverSessionV1::new(0).expect("resolver");
    let view = ScriptSyntaxViewV1::from_program(source.source_ast()).expect("Script view");
    let ResolveScriptOutcomeV1::Complete(owner) = resolver
        .resolve_script(view, &window)
        .expect("Using resolve")
    else {
        panic!("Using is a transparent Script boundary");
    };
    let product =
        VerifiedScriptSemanticSourceV1::seal(&source, owner, &window).expect("Using Script source");
    assert_eq!(product.forest().owner_count(), 1);
    assert_eq!(product.using_directive_sites().count(), 1);
    assert_selected_program_parity(
        ASTNode::Program {
            statements: vec![
                ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                },
                using,
            ],
            span: Span::unknown(),
        },
        "script-using.hako",
    );
}

#[test]
fn bare_this_seals_script_source_then_uses_existing_unsupported_diagnostic() {
    let program = ASTNode::Program {
        statements: vec![ASTNode::This {
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    };
    let source = PreparedNormalDefaultProgramRootV1::seal(program.clone()).expect("Program source");
    let window = VerifiedScriptRootDemandWindowV1::seal(vec![bare_this_unsupported_entry(0)], 1)
        .expect("bare This window");
    let mut resolver = FunctionSemanticResolverSessionV1::new(0).expect("resolver");
    let view = ScriptSyntaxViewV1::from_program(source.source_ast()).expect("Script view");
    let ResolveScriptOutcomeV1::Complete(owner) = resolver
        .resolve_script(view, &window)
        .expect("bare This resolve")
    else {
        panic!("bare This is an existing diagnostic boundary");
    };
    let product = VerifiedScriptSemanticSourceV1::seal(&source, owner, &window)
        .expect("bare This Script source");
    assert_eq!(product.forest().owner_count(), 1);
    assert_eq!(product.bare_this_unsupported_sites().count(), 1);

    let mut legacy = MirCompiler::with_options(false);
    let legacy_error = legacy
        .compile_with_source(program.clone(), Some("script-bare-this.hako"))
        .expect_err("legacy bare This rejects through the existing terminal");
    let mut normal = MirCompiler::with_options(false);
    let normal_error = normal
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                program,
                Some("script-bare-this.hako"),
                std::collections::HashMap::new(),
            )
            .expect("normal request"),
        )
        .expect_err("normal bare This rejects through the existing terminal");
    assert_eq!(normal_error, legacy_error);
    normal
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                NyashParser::parse_from_string("0").expect("reuse source"),
                Some("script-bare-this-reuse.hako"),
                std::collections::HashMap::new(),
            )
            .expect("reuse request"),
        )
        .expect("fresh request succeeds");
}

#[test]
fn context_scope_is_complete_without_observing_value_or_body() {
    let program = ASTNode::Program {
        statements: vec![ASTNode::ContextScope {
            name: "ctx".to_owned(),
            declared_type_name: None,
            value: Box::new(ASTNode::Variable {
                name: "missing_value".to_owned(),
                span: Span::unknown(),
            }),
            body: vec![ASTNode::Variable {
                name: "missing_body".to_owned(),
                span: Span::unknown(),
            }],
            source_keyword: "context".to_owned(),
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    };
    let source = PreparedNormalDefaultProgramRootV1::seal(program.clone()).expect("Program source");
    let window =
        VerifiedScriptRootDemandWindowV1::seal(vec![context_scope_unsupported_entry(0)], 1)
            .expect("ContextScope window");
    let mut resolver = FunctionSemanticResolverSessionV1::new(0).expect("resolver");
    let view = ScriptSyntaxViewV1::from_program(source.source_ast()).expect("Script view");
    let ResolveScriptOutcomeV1::Complete(owner) = resolver
        .resolve_script(view, &window)
        .expect("ContextScope resolve")
    else {
        panic!("ContextScope is a zero-child diagnostic boundary")
    };
    let product = VerifiedScriptSemanticSourceV1::seal(&source, owner, &window)
        .expect("ContextScope Script source");
    assert_eq!(product.forest().owner_count(), 1);
    assert_eq!(product.existing_diagnostic_sites().count(), 1);

    let mut legacy = MirCompiler::with_options(false);
    let legacy_error = legacy
        .compile_with_source(program.clone(), Some("script-context-scope.hako"))
        .expect_err("legacy ContextScope rejects through its existing terminal");
    let mut normal = MirCompiler::with_options(false);
    let normal_error = normal
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                program,
                Some("script-context-scope.hako"),
                std::collections::HashMap::new(),
            )
            .expect("normal request"),
        )
        .expect_err("normal ContextScope keeps the existing terminal");
    assert_eq!(normal_error, legacy_error);
    assert!(normal_error.contains("context_scope_lowering_missing"));
    normal
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                NyashParser::parse_from_string("0").expect("reuse source"),
                Some("context-scope-reuse.hako"),
                std::collections::HashMap::new(),
            )
            .expect("reuse request"),
        )
        .expect("fresh request succeeds");
}

#[test]
fn nested_or_statement_wrapped_this_stays_deferred() {
    for program in [
        ASTNode::Program {
            statements: vec![ASTNode::UnaryOp {
                operator: crate::ast::UnaryOperator::Minus,
                operand: Box::new(ASTNode::This {
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }],
            span: Span::unknown(),
        },
        ASTNode::Program {
            statements: vec![ASTNode::Print {
                expression: Box::new(ASTNode::This {
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }],
            span: Span::unknown(),
        },
    ] {
        let error = MirCompiler::with_options(false)
            .compile_normal(
                NormalCompileRequestV1::for_mir_mode(
                    program,
                    Some("script-nested-this.hako"),
                    std::collections::HashMap::new(),
                )
                .expect("normal request"),
            )
            .expect_err("nested This stays Deferred");
        assert!(error.contains("Unsupported AST node type"), "{error}");
    }
}

#[test]
fn recursive_or_statement_wrapped_me_stays_deferred() {
    for program in [
        ASTNode::Program {
            statements: vec![ASTNode::UnaryOp {
                operator: crate::ast::UnaryOperator::Minus,
                operand: Box::new(ASTNode::Me {
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }],
            span: Span::unknown(),
        },
        ASTNode::Program {
            statements: vec![ASTNode::Print {
                expression: Box::new(ASTNode::Me {
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }],
            span: Span::unknown(),
        },
    ] {
        let error = MirCompiler::with_options(false)
            .compile_normal(
                NormalCompileRequestV1::for_mir_mode(
                    program,
                    Some("script-nested-me.hako"),
                    std::collections::HashMap::new(),
                )
                .expect("normal request"),
            )
            .expect_err("nested Me stays Deferred");
        assert!(error.contains("MeResolverBox"), "{error}");
    }
}

#[test]
fn sparse_window_preserves_original_program_ordinal() {
    let source = PreparedNormalDefaultProgramRootV1::seal(ASTNode::Program {
        statements: vec![
            ASTNode::Literal {
                value: LiteralValue::Integer(0),
                span: Span::unknown(),
            },
            ASTNode::FunctionDeclaration {
                name: "helper".to_owned(),
                params: Vec::new(),
                param_decls: Vec::new(),
                return_type_name: None,
                body: Vec::new(),
                uses: Vec::new(),
                contracts: Vec::new(),
                is_static: true,
                is_override: false,
                attrs: Default::default(),
                span: Span::unknown(),
            },
            ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            },
        ],
        span: Span::unknown(),
    })
    .expect("Program source");
    let window = VerifiedScriptRootDemandWindowV1::seal(
        vec![resolved_entry(0), resolved_entry(1), resolved_entry(2)],
        3,
    )
    .expect("window");
    assert_eq!(
        window.entry_at(2).unwrap().site().node().segments()[1],
        SourcePathSegmentV1::ProgramBody(2)
    );
    let _ = source;
}

#[test]
fn selected_normal_lexical_local_and_read_use_one_ledger() {
    assert_selected_parity("local x = 1\nprint(x)", "script-local.hako");
}

#[test]
fn outbox_receipt_completes_without_observing_initializers() {
    let outbox = ASTNode::Outbox {
        variables: vec!["first".to_owned(), "second".to_owned()],
        initial_values: vec![
            Some(Box::new(ASTNode::Variable {
                name: "missing".to_owned(),
                span: Span::unknown(),
            })),
            None,
        ],
        span: Span::unknown(),
    };
    let program = ASTNode::Program {
        statements: vec![
            outbox,
            ASTNode::Print {
                expression: Box::new(ASTNode::Variable {
                    name: "second".to_owned(),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            },
        ],
        span: Span::unknown(),
    };
    let source = PreparedNormalDefaultProgramRootV1::seal(program.clone()).expect("Program source");
    let window =
        VerifiedScriptRootDemandWindowV1::seal(vec![resolved_entry(0), resolved_entry(1)], 2)
            .expect("Outbox window");
    let mut resolver = FunctionSemanticResolverSessionV1::new(0).expect("resolver");
    let view = ScriptSyntaxViewV1::from_program(source.source_ast()).expect("Script view");
    let ResolveScriptOutcomeV1::Complete(owner) = resolver
        .resolve_script(view, &window)
        .expect("Outbox resolve")
    else {
        panic!("Outbox must Complete");
    };
    let product = VerifiedScriptSemanticSourceV1::seal(&source, owner, &window)
        .expect("Outbox source product");
    let receipts = product.outbox_materializations().collect::<Vec<_>>();
    assert_eq!(receipts.len(), 1);
    assert_eq!(
        receipts[0].0.node().segments(),
        &[
            SourcePathSegmentV1::ProgramBodyRoot,
            SourcePathSegmentV1::ProgramBody(0),
        ]
    );
    assert_eq!(receipts[0].1.len(), 2);
    assert_selected_parity("outbox payload", "script-outbox-semantic.hako");
}

#[test]
fn selected_normal_print_lexical_closure_matches_legacy() {
    assert_selected_parity("local x = 1\nprint(-x)", "script-unary.hako");
}

#[test]
fn real_print_fixture_uses_the_selected_normal_request() {
    assert_selected_parity("print(1)", "script-print.hako");
}

#[test]
fn selected_normal_binary_lexical_closure_matches_legacy() {
    assert_selected_parity("local x = 1\nprint((x * 2) + 3)", "script-binary.hako");
}

#[test]
fn selected_normal_await_lexical_closure_matches_legacy() {
    assert_selected_parity("local x = 1\nprint(await -(x + 2))", "script-await.hako");
}

#[test]
fn selected_normal_check_lexical_closure_matches_legacy() {
    assert_selected_parity("local x = true\nprint(check { x })", "script-check.hako");
}

#[test]
fn selected_normal_and_or_lexical_closure_matches_legacy() {
    assert_selected_parity(
        "local x = true\nprint(x and x)\nprint(x or x)",
        "script-andor.hako",
    );
}

#[test]
fn lexical_fastmem_scope_matches_legacy() {
    let local = ASTNode::Local {
        variables: vec!["x".to_owned()],
        initial_values: vec![Some(Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(1),
            span: Span::unknown(),
        }))],
        declared_type_names: vec![None],
        span: Span::unknown(),
    };
    let print = ASTNode::Print {
        expression: Box::new(ASTNode::Variable {
            name: "x".to_owned(),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };
    assert_selected_program_parity(
        ASTNode::Program {
            statements: vec![ASTNode::FastMemRegion {
                contract: "PageMapV0".to_owned(),
                body: vec![local, print],
                span: Span::unknown(),
            }],
            span: Span::unknown(),
        },
        "script-fastmem.hako",
    );
}

#[test]
fn lexical_scopebox_matches_legacy() {
    let local = ASTNode::Local {
        variables: vec!["x".to_owned()],
        initial_values: vec![Some(Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(1),
            span: Span::unknown(),
        }))],
        declared_type_names: vec![None],
        span: Span::unknown(),
    };
    let print = ASTNode::Print {
        expression: Box::new(ASTNode::Variable {
            name: "x".to_owned(),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };
    assert_selected_program_parity(
        ASTNode::Program {
            statements: vec![ASTNode::ScopeBox {
                body: vec![local, print],
                span: Span::unknown(),
            }],
            span: Span::unknown(),
        },
        "script-scopebox.hako",
    );
}

#[test]
fn nested_lexical_scopeboxes_match_legacy() {
    let local = ASTNode::Local {
        variables: vec!["x".to_owned()],
        initial_values: vec![Some(Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(1),
            span: Span::unknown(),
        }))],
        declared_type_names: vec![None],
        span: Span::unknown(),
    };
    let print = ASTNode::Print {
        expression: Box::new(ASTNode::Variable {
            name: "x".to_owned(),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };
    assert_selected_program_parity(
        ASTNode::Program {
            statements: vec![ASTNode::ScopeBox {
                body: vec![ASTNode::ScopeBox {
                    body: vec![local, print],
                    span: Span::unknown(),
                }],
                span: Span::unknown(),
            }],
            span: Span::unknown(),
        },
        "script-nested-scopebox.hako",
    );
}

#[test]
fn lexical_task_scope_matches_legacy_with_canonical_body_source() {
    let local = ASTNode::Local {
        variables: vec!["x".to_owned()],
        initial_values: vec![Some(Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(1),
            span: Span::unknown(),
        }))],
        declared_type_names: vec![None],
        span: Span::unknown(),
    };
    let print = ASTNode::Print {
        expression: Box::new(ASTNode::Variable {
            name: "x".to_owned(),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };
    assert_selected_program_parity(
        ASTNode::Program {
            statements: vec![ASTNode::TaskScope {
                body: vec![local, print],
                source_keyword: "co".to_owned(),
                span: Span::unknown(),
            }],
            span: Span::unknown(),
        },
        "script-task-scope-lexical.hako",
    );
}

#[test]
fn task_scope_early_exit_stays_deferred_to_existing_preflight() {
    let program = ASTNode::Program {
        statements: vec![ASTNode::TaskScope {
            body: vec![ASTNode::Return {
                value: Some(Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                })),
                span: Span::unknown(),
            }],
            source_keyword: "co".to_owned(),
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    };
    let source = PreparedNormalDefaultProgramRootV1::seal(program.clone()).expect("Program source");
    let window = VerifiedScriptRootDemandWindowV1::seal(vec![resolved_entry(0)], 1)
        .expect("TaskScope window");
    let mut resolver = FunctionSemanticResolverSessionV1::new(0).expect("resolver");
    let view = ScriptSyntaxViewV1::from_program(source.source_ast()).expect("Script view");
    assert!(matches!(
        resolver
            .resolve_script(view, &window)
            .expect("TaskScope resolve"),
        ResolveScriptOutcomeV1::Deferred
    ));

    let mut compiler = MirCompiler::with_options(false);
    let error = compiler
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                program,
                Some("script-task-scope-return.hako"),
                std::collections::HashMap::new(),
            )
            .expect("normal request"),
        )
        .expect_err("existing task-scope preflight rejects return");
    assert!(error.contains("co/early-exit-unsupported"), "{error}");
    compiler
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                NyashParser::parse_from_string("0").expect("reuse source"),
                Some("task-scope-reuse.hako"),
                std::collections::HashMap::new(),
            )
            .expect("reuse request"),
        )
        .expect("fresh request succeeds");
}
