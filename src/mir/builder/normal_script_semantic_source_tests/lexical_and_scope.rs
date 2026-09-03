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
fn selected_normal_lexical_closure_matrix_matches_legacy() {
    for (source, hint) in [
        ("local x = 1\nprint(-x)", "script-unary.hako"),
        ("print(1)", "script-print.hako"),
        ("local x = 1\nprint((x * 2) + 3)", "script-binary.hako"),
        ("local x = 1\nprint(await -(x + 2))", "script-await.hako"),
        ("local x = true\nprint(check { x })", "script-check.hako"),
        (
            "local x = true\nprint(x and x)\nprint(x or x)",
            "script-andor.hako",
        ),
    ] {
        assert_selected_parity(source, hint);
    }
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
        ResolveScriptOutcomeV1::Deferred(_)
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
