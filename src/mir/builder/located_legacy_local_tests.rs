use std::collections::BTreeSet;

use crate::ast::FieldDecl;
use crate::mir::builder::vars::lexical_scope::LexicalScopeGuard;
use crate::mir::builder::{
    SameModuleCallableNamespaceV1, VerifiedSameModuleCallableDeclarationCatalogV1,
};
use crate::mir::callable_result_representation::{
    VerifiedCallableResultActivationPlanV1, VerifiedCallableResultActivationRowsV1,
    VerifiedCallableResultLegacySourceViewV1, VerifiedSameModuleCallableResultCatalogV1,
};
use crate::mir::resolved_semantics::{SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1};
use crate::mir::source_call_target::{
    VerifiedQualifiedCallRouteFactsV1, VerifiedQualifiedReceiverLexicalDispositionsV1,
    VerifiedSourceMethodCallSiteV1, VerifiedSourceStaticCallTargetCatalogV1,
    VerifiedStaticImportAliasViewV1,
};
use crate::mir::{MirBuilder, MirInstruction};
use crate::parser::NyashParser;

use super::{
    CanonicalSameModuleCallableKeyV1, LocatedLegacyLoweringErrorV1, LocatedLegacyLoweringSessionV1,
};

#[derive(Clone)]
pub(super) struct CallSiteSpecV1 {
    pub(super) site: SourceExprSiteV1,
}

pub(super) fn site(segments: Vec<SourcePathSegmentV1>) -> SourceExprSiteV1 {
    SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(segments))
}

fn declarations(source: &str) -> VerifiedSameModuleCallableDeclarationCatalogV1 {
    let root = NyashParser::parse_from_string(source).expect("located Local fixture must parse");
    VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&root)
        .expect("located Local declarations must seal")
}

pub(super) fn caller(
    declarations: &VerifiedSameModuleCallableDeclarationCatalogV1,
) -> CanonicalSameModuleCallableKeyV1 {
    declarations
        .declaration_for(
            SameModuleCallableNamespaceV1::InstanceBoxMethod,
            "ParserBox",
            "parse",
            2,
        )
        .expect("ParserBox.parse/2")
        .key()
        .clone()
}

pub(super) fn seal_plan(
    source: &str,
    selected: Vec<CallSiteSpecV1>,
) -> VerifiedCallableResultActivationPlanV1 {
    let declarations = Box::new(declarations(source));
    let caller = caller(&declarations);
    let imports =
        VerifiedStaticImportAliasViewV1::seal(&declarations, Vec::new()).expect("empty aliases");
    let calls = selected
        .iter()
        .map(|spec| {
            VerifiedSourceMethodCallSiteV1::verify(&declarations, &caller, spec.site.clone())
                .expect("exact located Local call site")
        })
        .collect::<Vec<_>>();
    let caller_keys = calls
        .iter()
        .map(|call| call.caller().clone())
        .collect::<BTreeSet<_>>();
    let lexical = caller_keys
        .iter()
        .map(|key| {
            let rows = calls
                .iter()
                .filter(|call| call.caller() == key)
                .collect::<Vec<_>>();
            VerifiedQualifiedReceiverLexicalDispositionsV1::verify(&rows)
                .expect("located Local lexical facts")
        })
        .collect::<Vec<_>>();
    let facts = calls
        .iter()
        .map(|call| {
            let lexical = lexical
                .iter()
                .find(|rows| rows.caller() == call.caller())
                .expect("caller-local lexical facts");
            VerifiedQualifiedCallRouteFactsV1::verify(call, lexical, &imports)
                .expect("located Local qualified route")
        })
        .collect::<Vec<_>>();
    let targets = VerifiedSourceStaticCallTargetCatalogV1::seal_qualified(&imports, facts)
        .expect("located Local targets");
    let results = VerifiedSameModuleCallableResultCatalogV1::verify(&declarations, &targets)
        .expect("located Local result catalog");
    let rows = VerifiedCallableResultActivationRowsV1::verify(&declarations, &targets, &results)
        .expect("located Local activation rows");
    drop(results);
    drop(targets);
    drop(imports);
    VerifiedCallableResultActivationPlanV1::seal(declarations, rows).expect("located Local plan")
}

pub(super) fn builder_for(source: &str, name: &str) -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder
        .comp_ctx
        .install_callable_declaration_catalog(declarations(source))
        .expect("located Local callable catalog");
    builder.enter_function_for_test(name.to_string());
    builder
}

pub(super) fn instructions(builder: &MirBuilder) -> Vec<MirInstruction> {
    builder
        .scope_ctx
        .current_function
        .as_ref()
        .expect("located Local function")
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter().cloned())
        .collect()
}

pub(super) fn lower_root_statements<'plan>(
    session: &mut LocatedLegacyLoweringSessionV1<'plan>,
    plan: &'plan VerifiedCallableResultActivationPlanV1,
    caller: &CanonicalSameModuleCallableKeyV1,
    builder: &mut MirBuilder,
    indices: &[usize],
) -> Result<(), LocatedLegacyLoweringErrorV1> {
    let view = VerifiedCallableResultLegacySourceViewV1::verify(plan, caller).unwrap();
    let body = view.root_body();
    for index in indices {
        let statement = view.body_stmt(&body, *index).unwrap();
        session.lower_statement(builder, statement)?;
    }
    Ok(())
}

#[test]
fn located_local_claims_exact_initializers_in_statement_and_expression_order() {
    const SOURCE: &str = r#"
        box ParserBox {
            parse(text, pos) {
                local first = Helpers.left(1)
                local second = 2 + Helpers.right(3)
                return second
            }
        }
        static box Helpers {
            left(value) { return value }
            right(value) { return value }
        }
    "#;
    let plan = seal_plan(
        SOURCE,
        vec![
            CallSiteSpecV1 {
                site: site(vec![
                    SourcePathSegmentV1::Body(0),
                    SourcePathSegmentV1::Initializer(0),
                ]),
            },
            CallSiteSpecV1 {
                site: site(vec![
                    SourcePathSegmentV1::Body(1),
                    SourcePathSegmentV1::Initializer(0),
                    SourcePathSegmentV1::Rhs,
                ]),
            },
        ],
    );
    let caller = caller(plan.declaration_catalog());
    let mut session = LocatedLegacyLoweringSessionV1::verify(&plan, &caller).unwrap();
    let mut builder = builder_for(SOURCE, "located_local_order/0");
    let _scope = LexicalScopeGuard::new(&mut builder);

    lower_root_statements(&mut session, &plan, &caller, &mut builder, &[0, 1]).unwrap();
    session.finish().unwrap();

    let targets = instructions(&builder)
        .into_iter()
        .filter_map(|instruction| match instruction {
            MirInstruction::Call { callee, .. } => Some(format!("{callee:?}")),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(targets.len(), 2);
    assert!(targets[0].contains("Helpers.left"), "{targets:?}");
    assert!(targets[1].contains("Helpers.right"), "{targets:?}");
    assert!(builder.binding_ctx.contains("first"));
    assert!(builder.binding_ctx.contains("second"));
    assert_eq!(builder.recursion_depth, 0);
}

#[test]
fn located_local_short_circuit_keeps_deferred_rhs_site_and_completion() {
    const SOURCE: &str = r#"
        box ParserBox {
            parse(text, pos) {
                local flag = Helpers.left(1) && Helpers.right(2)
                return flag
            }
        }
        static box Helpers {
            left(value) { return value }
            right(value) { return value }
        }
    "#;
    let plan = seal_plan(
        SOURCE,
        vec![
            CallSiteSpecV1 {
                site: site(vec![
                    SourcePathSegmentV1::Body(0),
                    SourcePathSegmentV1::Initializer(0),
                    SourcePathSegmentV1::Lhs,
                ]),
            },
            CallSiteSpecV1 {
                site: site(vec![
                    SourcePathSegmentV1::Body(0),
                    SourcePathSegmentV1::Initializer(0),
                    SourcePathSegmentV1::Rhs,
                ]),
            },
        ],
    );
    let caller = caller(plan.declaration_catalog());
    let mut session = LocatedLegacyLoweringSessionV1::verify(&plan, &caller).unwrap();
    let mut builder = builder_for(SOURCE, "located_local_short_circuit/0");
    let _scope = LexicalScopeGuard::new(&mut builder);

    lower_root_statements(&mut session, &plan, &caller, &mut builder, &[0]).unwrap();
    session.finish().unwrap();

    assert!(builder.binding_ctx.contains("flag"));
    assert!(instructions(&builder)
        .iter()
        .any(|instruction| matches!(instruction, MirInstruction::Phi { .. })));
    assert_eq!(builder.recursion_depth, 0);
}

#[test]
fn located_local_special_hooks_require_exact_inactive_initializer_subtrees() {
    const SOURCE: &str = r#"
        record Pair { value: i64 }
        box ParserBox {
            parse(text, pos) {
                local bytes: Array<u8> = [1, 2]
                local pair = new Pair(7)
                local active = Helpers.step(3)
                return active
            }
        }
        static box Helpers { step(value) { return value } }
    "#;
    let plan = seal_plan(
        SOURCE,
        vec![CallSiteSpecV1 {
            site: site(vec![
                SourcePathSegmentV1::Body(2),
                SourcePathSegmentV1::Initializer(0),
            ]),
        }],
    );
    let caller = caller(plan.declaration_catalog());
    let mut session = LocatedLegacyLoweringSessionV1::verify(&plan, &caller).unwrap();
    let mut builder = builder_for(SOURCE, "located_local_special_hooks/0");
    builder.comp_ctx.register_record_decl(
        "Pair".to_string(),
        Vec::new(),
        &[FieldDecl {
            name: "value".to_string(),
            declared_type_name: None,
            is_weak: false,
            default_value: None,
        }],
    );
    let _scope = LexicalScopeGuard::new(&mut builder);

    lower_root_statements(&mut session, &plan, &caller, &mut builder, &[0, 1, 2]).unwrap();
    session.finish().unwrap();

    let rows = instructions(&builder);
    assert!(rows
        .iter()
        .any(|instruction| matches!(instruction, MirInstruction::ArrayElementWrite { .. })));
    assert!(rows
        .iter()
        .any(|instruction| matches!(instruction, MirInstruction::RecordValuePublish { .. })));
    assert!(builder.binding_ctx.contains("bytes"));
    assert!(builder.binding_ctx.contains("pair"));
    assert!(builder.binding_ctx.contains("active"));
}

#[test]
fn active_row_below_typed_array_hook_rejects_before_builder_effects_and_poisons_session() {
    const SOURCE: &str = r#"
        box ParserBox {
            parse(text, pos) {
                local bytes: Array<u8> = [Helpers.step(1)]
                return 0
            }
        }
        static box Helpers { step(value) { return value } }
    "#;
    let plan = seal_plan(
        SOURCE,
        vec![CallSiteSpecV1 {
            site: site(vec![
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::Initializer(0),
                SourcePathSegmentV1::Element(0),
            ]),
        }],
    );
    let caller = caller(plan.declaration_catalog());
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &caller).unwrap();
    let body = view.root_body();
    let statement = view.body_stmt(&body, 0).unwrap();
    let mut session = LocatedLegacyLoweringSessionV1::verify(&plan, &caller).unwrap();
    let mut builder = builder_for(SOURCE, "located_local_array_reject/0");
    let _scope = LexicalScopeGuard::new(&mut builder);

    let error = session
        .lower_statement(&mut builder, statement)
        .unwrap_err();
    assert!(matches!(
        error,
        LocatedLegacyLoweringErrorV1::Lowering(ref text)
            if text.contains("RowsUnderPrefix")
    ));
    assert!(instructions(&builder).is_empty());
    assert!(!builder.binding_ctx.contains("bytes"));
    assert_eq!(builder.recursion_depth, 0);
    assert!(matches!(
        session.finish(),
        Err(LocatedLegacyLoweringErrorV1::Poisoned)
    ));
}

#[test]
fn active_row_below_record_hook_rejects_before_constructor_effects() {
    const SOURCE: &str = r#"
        record Pair { value: i64 }
        box ParserBox {
            parse(text, pos) {
                local pair = new Pair(Helpers.step(1))
                return 0
            }
        }
        static box Helpers { step(value) { return value } }
    "#;
    let plan = seal_plan(
        SOURCE,
        vec![CallSiteSpecV1 {
            site: site(vec![
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::Initializer(0),
                SourcePathSegmentV1::Argument(0),
            ]),
        }],
    );
    let caller = caller(plan.declaration_catalog());
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &caller).unwrap();
    let body = view.root_body();
    let statement = view.body_stmt(&body, 0).unwrap();
    let mut session = LocatedLegacyLoweringSessionV1::verify(&plan, &caller).unwrap();
    let mut builder = builder_for(SOURCE, "located_local_record_reject/0");
    builder.comp_ctx.register_record_decl(
        "Pair".to_string(),
        Vec::new(),
        &[FieldDecl {
            name: "value".to_string(),
            declared_type_name: None,
            is_weak: false,
            default_value: None,
        }],
    );
    let _scope = LexicalScopeGuard::new(&mut builder);

    let error = session
        .lower_statement(&mut builder, statement)
        .unwrap_err();
    assert!(matches!(
        error,
        LocatedLegacyLoweringErrorV1::Lowering(ref text)
            if text.contains("RowsUnderPrefix")
    ));
    assert!(instructions(&builder).is_empty());
    assert!(!builder.binding_ctx.contains("pair"));
    assert_eq!(builder.recursion_depth, 0);
}

#[test]
fn wrong_statement_order_fails_before_local_initializer_or_binding_effects() {
    const SOURCE: &str = r#"
        box ParserBox {
            parse(text, pos) {
                local first = Helpers.left(1)
                local second = Helpers.right(2)
                return second
            }
        }
        static box Helpers {
            left(value) { return value }
            right(value) { return value }
        }
    "#;
    let plan = seal_plan(
        SOURCE,
        vec![
            CallSiteSpecV1 {
                site: site(vec![
                    SourcePathSegmentV1::Body(0),
                    SourcePathSegmentV1::Initializer(0),
                ]),
            },
            CallSiteSpecV1 {
                site: site(vec![
                    SourcePathSegmentV1::Body(1),
                    SourcePathSegmentV1::Initializer(0),
                ]),
            },
        ],
    );
    let caller = caller(plan.declaration_catalog());
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &caller).unwrap();
    let body = view.root_body();
    let second = view.body_stmt(&body, 1).unwrap();
    let mut session = LocatedLegacyLoweringSessionV1::verify(&plan, &caller).unwrap();
    let mut builder = builder_for(SOURCE, "located_local_wrong_order/0");
    let _scope = LexicalScopeGuard::new(&mut builder);

    let error = session.lower_statement(&mut builder, second).unwrap_err();
    assert!(matches!(
        error,
        LocatedLegacyLoweringErrorV1::Lowering(ref text)
            if text.contains("WrongOrder")
    ));
    assert!(instructions(&builder).is_empty());
    assert!(!builder.binding_ctx.contains("second"));
    assert_eq!(builder.recursion_depth, 0);
    assert!(matches!(
        session.finish(),
        Err(LocatedLegacyLoweringErrorV1::Poisoned)
    ));

    let mut fresh = LocatedLegacyLoweringSessionV1::verify(&plan, &caller).unwrap();
    let mut fresh_builder = builder_for(SOURCE, "located_local_fresh/0");
    let _fresh_scope = LexicalScopeGuard::new(&mut fresh_builder);
    lower_root_statements(&mut fresh, &plan, &caller, &mut fresh_builder, &[0, 1]).unwrap();
    fresh.finish().unwrap();
}
