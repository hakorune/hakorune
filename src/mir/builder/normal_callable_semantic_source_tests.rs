use super::{NormalCallableSemanticAdmissionV1, VerifiedNormalCallableSemanticSourceV1};
use crate::mir::builder::callable_declaration_catalog::{
    SameModuleCallableNamespaceV1, SelectedNormalCallableKeyV1,
    VerifiedSameModuleCallableDeclarationCatalogV1,
};
use crate::mir::resolved_semantics::{FunctionSemanticResolverSessionV1, SourcePathV1};
use crate::mir::{MirCompiler, MirPrinter, NormalCompileRequestV1};
use crate::parser::NyashParser;

fn loop_program() -> crate::ast::ASTNode {
    NyashParser::parse_from_string(
        r#"
                static box StringHelpers {
                    int_to_str(n) {
                        local value = me.to_i64(n)
                        local i = 0
                        loop(i < 1) { i = i + 1 }
                        return value
                    }
                    to_i64(x) { return x + 1 }
                }
            "#,
    )
    .expect("callable loop source")
}

fn assert_callable_materialization_parity(source: &str) {
    let legacy = MirCompiler::with_options(false)
        .compile_with_source(
            NyashParser::parse_from_string(source).unwrap(),
            Some("callable-materialization.hako"),
        )
        .unwrap();
    let normal = MirCompiler::with_options(false)
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                NyashParser::parse_from_string(source).unwrap(),
                Some("callable-materialization.hako"),
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

#[test]
fn callable_entry_local_variable_and_rebind_materialization_keeps_parity() {
    assert_callable_materialization_parity(
        "function helper(x) { local y = x y += 1 return y }\n\
             static box Tools { add(x) { local y = x y += 1 return y } }\n\
             box Page { show(x) { local y = x y += 1 return y } }\n\
             function capture(first, second) {\n\
                 local f = fn(){ first + second }\n\
                 return first\n\
             }",
    );

    let mut compiler = MirCompiler::with_options(false);
    assert!(compiler
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                NyashParser::parse_from_string("function bad(x) { local y = missing return y }",)
                    .unwrap(),
                Some("callable-materialization-failure.hako"),
                std::collections::HashMap::new(),
            )
            .unwrap(),
        )
        .is_err());
    compiler
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                NyashParser::parse_from_string("function good(x) { local y = x y += 1 return y }")
                    .unwrap(),
                Some("callable-materialization-reuse.hako"),
                std::collections::HashMap::new(),
            )
            .unwrap(),
        )
        .unwrap();
}

#[test]
fn mixed_callable_batch_seals_and_reacquires_exact_program_sites() {
    let program = NyashParser::parse_from_string(
        "function helper(x) { return x }\n\
             static box Tools { add(x) { return x } }\n\
             box Page { show(x) { return x } }",
    )
    .unwrap();
    let catalog = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&program).unwrap();
    let mut resolver = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let NormalCallableSemanticAdmissionV1::Complete(source) =
        VerifiedNormalCallableSemanticSourceV1::seal(
            &program,
            catalog.selected_source_inventory(),
            false,
            &mut resolver,
        )
        .unwrap()
    else {
        panic!("mixed callable batch deferred")
    };
    for (key, _) in catalog.selected_source_inventory().entries() {
        source.loan(key).unwrap();
    }
    assert_eq!(source.keys().count(), 3);
}

#[test]
fn callable_loop_handoff_issues_exact_resolver_sites_before_lowering() {
    let program = loop_program();
    let catalog = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&program)
        .expect("callable catalog");
    let mut resolver = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let NormalCallableSemanticAdmissionV1::Complete(source) =
        VerifiedNormalCallableSemanticSourceV1::seal(
            &program,
            catalog.selected_source_inventory(),
            false,
            &mut resolver,
        )
        .unwrap()
    else {
        panic!("callable semantic source deferred")
    };
    let key = source
        .keys()
        .find(|key| {
            matches!(
                key,
                SelectedNormalCallableKeyV1::Cataloged(key)
                    if key.owner() == "StringHelpers" && key.name() == "int_to_str"
            )
        })
        .expect("loop callable key")
        .clone();
    let ingress = source.loan(&key).unwrap().into_source_ingress();
    assert_eq!(ingress.owner(), ingress.input().owner());
    assert_eq!(ingress.owner(), ingress.input().source().owner());
    assert_eq!(ingress.owner(), ingress.ledger().owner());
    assert!(ingress.input().callable_index().is_none());
    assert!(ingress.input().callable_header().is_none());

    // The source rows remain reusable: issuing a fresh loan creates a
    // fresh receipt without rewalking or mutating the resolver forest.
    let (_, ingress) = source.loan(&key).unwrap().into_parts();
    let state = super::super::normal_callable_semantic_lowering_state::CallableSemanticLoweringState::from_exact_source(
            ingress.input(),
        )
        .unwrap();
    let schedule = state
        .loop_binding_source_projection()
        .project(SourcePathV1::root_body(2).node())
        .expect("loop schedule");
    assert_eq!(schedule.receipt_count(), 3);
    assert_eq!(
            schedule
                .receipts()
                .filter(|receipt| {
                    matches!(
                        receipt.role(),
                        super::super::normal_callable_loop_handoff::CallableLoopBindingRoleV1::ConditionRead
                            | super::super::normal_callable_loop_handoff::CallableLoopBindingRoleV1::BodyRead
                    )
                })
                .count(),
            2
        );
    assert_eq!(
            schedule
                .receipts()
                .filter(|receipt| {
                    matches!(
                        receipt.role(),
                        super::super::normal_callable_loop_handoff::CallableLoopBindingRoleV1::BodyRebind
                    )
                })
                .count(),
            1
        );
}

#[test]
fn function_call_defers_before_unresolved_argument_child() {
    let program =
        NyashParser::parse_from_string("function helper() { return unknown(missing) }").unwrap();
    let catalog = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&program).unwrap();
    let mut resolver = FunctionSemanticResolverSessionV1::new(0).unwrap();
    assert!(matches!(
        VerifiedNormalCallableSemanticSourceV1::seal(
            &program,
            catalog.selected_source_inventory(),
            false,
            &mut resolver,
        )
        .unwrap(),
        NormalCallableSemanticAdmissionV1::Deferred
    ));
}

#[test]
fn selected_direct_call_observation_is_a_typed_package_terminal() {
    let program = NyashParser::parse_from_string("function caller() { return helper() }").unwrap();
    let catalog = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&program).unwrap();
    let mut resolver = FunctionSemanticResolverSessionV1::new(813).unwrap();
    assert!(matches!(
        VerifiedNormalCallableSemanticSourceV1::seal(
            &program,
            catalog.selected_source_inventory(),
            true,
            &mut resolver,
        )
        .unwrap(),
        NormalCallableSemanticAdmissionV1::Rejected(
            super::NormalCallableSemanticAdmissionRejectV1::UnissuedDirectCallObservation
        )
    ));
}

#[test]
fn main_methods_are_absent_from_callable_semantic_batch() {
    let program = NyashParser::parse_from_string(
        "static box Main { main() { return 0 } helper() { return 1 } }\n\
             static box Tools { helper() { return 2 } }",
    )
    .unwrap();
    let catalog = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&program).unwrap();
    let inventory = catalog.selected_source_inventory();
    assert_eq!(inventory.len(), 1);
    let tools = catalog
        .declaration_for(
            SameModuleCallableNamespaceV1::StaticBoxMethod,
            "Tools",
            "helper",
            0,
        )
        .unwrap()
        .key()
        .clone();
    assert!(inventory
        .site(&SelectedNormalCallableKeyV1::Cataloged(tools))
        .is_some());
}

#[test]
fn nonplain_instance_blocker_defers_the_whole_mixed_batch_before_resolution() {
    let program = NyashParser::parse_from_string(
        "function helper(x) { return x }\n\
             record Pair { value: i64 }",
    )
    .unwrap();
    let catalog = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&program).unwrap();
    assert_eq!(catalog.selected_source_inventory().len(), 1);
    assert_eq!(catalog.selected_source_inventory().blockers().len(), 1);
    let mut resolver = FunctionSemanticResolverSessionV1::new(0).unwrap();
    assert!(matches!(
        VerifiedNormalCallableSemanticSourceV1::seal(
            &program,
            catalog.selected_source_inventory(),
            false,
            &mut resolver,
        )
        .unwrap(),
        NormalCallableSemanticAdmissionV1::Deferred
    ));
}

#[test]
fn nonplain_blocker_is_script_only_and_app_remains_complete_eligible() {
    let program = NyashParser::parse_from_string(
        "function helper(x) { return x } record Pair { value: i64 }",
    )
    .unwrap();
    let catalog = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&program).unwrap();
    let mut resolver = FunctionSemanticResolverSessionV1::new(0).unwrap();
    assert!(matches!(
        VerifiedNormalCallableSemanticSourceV1::seal(
            &program,
            catalog.selected_source_inventory(),
            true,
            &mut resolver,
        )
        .unwrap(),
        NormalCallableSemanticAdmissionV1::Complete(_)
    ));
}

#[test]
fn mixed_nonplain_batch_keeps_selected_and_legacy_lowering_in_parity() {
    let text = "function helper() { return 1 }\n\
                    record Pair { value: i64 }\n\
                    Pair { value: 1 }";
    let mut legacy = MirCompiler::with_options(false);
    let legacy = legacy
        .compile_with_source(
            NyashParser::parse_from_string(text).unwrap(),
            Some("callable-nonplain"),
        )
        .unwrap();
    let normal = MirCompiler::with_options(false)
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                NyashParser::parse_from_string(text).unwrap(),
                Some("callable-nonplain"),
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

#[test]
fn callable_parameter_and_local_bodies_do_not_borrow_the_script_ledger() {
    let text = "function helper(x) { local y = x return y }\n\
                    static box Tools { add(x) { local y = x return y } }\n\
                    box Page { show(x) { local y = x return y } }\n\
                    0";
    let mut legacy = MirCompiler::with_options(false);
    let legacy = legacy
        .compile_with_source(
            NyashParser::parse_from_string(text).unwrap(),
            Some("callable-ledger-scope"),
        )
        .unwrap();
    let normal = MirCompiler::with_options(false)
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                NyashParser::parse_from_string(text).unwrap(),
                Some("callable-ledger-scope"),
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
