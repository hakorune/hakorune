use crate::ast::ASTNode;
use crate::mir::compiler::capability::CanonicalLoweringPreflightV1;
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::resolved_semantics::{BindingKindV1, SourceBindingSiteV1};
use crate::mir::{MirInstruction, MirType};

use super::identity::ResolvedIdentityStateV1;
use super::*;

const FIXTURE_SOURCE: &str = "static function canonical_fixture(arg) { local x = arg + 1\n x = x + 1\n outbox result\n return x }";

fn fixture() -> ASTNode {
    let ASTNode::Program { mut statements, .. } =
        crate::parser::NyashParser::parse_from_string(FIXTURE_SOURCE).expect("fixture parses")
    else {
        panic!("fixture must be a program")
    };
    assert_eq!(statements.len(), 1);
    statements.remove(0)
}

#[test]
fn closed_family_uses_resolver_bindings_without_legacy_allocation() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(fixture()).unwrap();
    let plan = CanonicalLoweringPreflightV1::verify(&unit).unwrap();
    let mut builder = MirBuilder::new();
    let crate::mir::compiler::capability::CanonicalFirstFamilyPlanV1::CurrentCanonicalAPlus(plan) =
        plan
    else {
        panic!("parameter/outbox fixture must remain on temporary A+")
    };
    let module = builder.build_resolved_function_module(plan).unwrap();

    assert!(module.functions.contains_key("canonical_fixture/1"));
    assert_eq!(builder.core_ctx.next_binding_id, 0);
    assert!(builder.function_state.binding_ctx.is_empty());
    assert!(builder.function_state.variable_ctx.variable_map.is_empty());
}

fn compile_source(text: &str) -> crate::mir::compiler::MirCompileResult {
    let parsed = crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
        text,
        crate::parser::ParserBuildConfig::default(),
    )
    .expect("fixture parses");
    let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) =
        crate::r#macro::transform_normal_callable_program_v1(parsed).expect("fixture transform")
    else {
        panic!("fixture must retain source identity")
    };
    let mut compiler = crate::mir::MirCompiler::with_options(false);
    compiler
        .compile_normal(
            crate::mir::compiler::NormalCompileRequestV1::for_mir_mode_callable_source(
                source,
                Some("canonical_fixture.hako"),
                std::collections::HashMap::new(),
            ),
        )
        .unwrap()
}

#[test]
fn source_backed_route_produces_verifier_clean_mir() {
    let result = compile_source(FIXTURE_SOURCE);

    assert!(
        result.verification_result.is_ok(),
        "{:?}",
        result.verification_result
    );
    assert!(result.module.functions.contains_key("canonical_fixture/1"));
    let function = &result.module.functions["canonical_fixture/1"];
    assert_eq!(function.metadata.outbox_bindings, vec!["result"]);
}

#[test]
fn trivial_float_route_keeps_exact_type_and_emits_no_legacy_release() {
    let result = compile_source("static function trivial_float() { return 1.5 + 2.5 }");
    let function = &result.module.functions["trivial_float/0"];
    let binop_dst = function
        .blocks
        .values()
        .flat_map(|block| &block.instructions)
        .find_map(|instruction| match instruction {
            MirInstruction::BinOp { dst, .. } => Some(*dst),
            _ => None,
        })
        .expect("direct trivial Float BinOp must be materialized");

    assert_eq!(
        function.metadata.value_types.get(&binop_dst),
        Some(&MirType::Float)
    );
    assert!(function.blocks.values().all(|block| block
        .instructions
        .iter()
        .all(|instruction| !matches!(instruction, MirInstruction::ReleaseStrong { .. }))));
    assert!(result.verification_result.is_ok());
}

#[test]
fn trivial_comparison_route_keeps_the_profile_sealed_bool_result() {
    let result = compile_source("static function trivial_bool_compare() { return true == false }");
    let function = &result.module.functions["trivial_bool_compare/0"];
    let comparison = function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .find_map(|instruction| match instruction {
            MirInstruction::Compare { dst, .. } => Some(*dst),
            _ => None,
        })
        .expect("sealed trivial Bool equality must materialize Compare");

    assert_eq!(
        function.metadata.value_types.get(&comparison),
        Some(&MirType::Bool)
    );
    assert!(result.verification_result.is_ok());
}

#[test]
fn incomplete_identity_coverage_cannot_finish() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(fixture()).unwrap();
    let input = unit.root_function_input().unwrap();
    let mut state = ResolvedIdentityStateV1::new(input.function());
    state
        .publish_declaration(
            &SourceBindingSiteV1::Parameter { index: 0 },
            BindingKindV1::Parameter { index: 0 },
            "arg",
            crate::mir::ValueId::new(0),
        )
        .unwrap();
    assert!(state.finish().is_err());
}

#[test]
fn installed_product_structurally_vetoes_legacy_binding_allocator() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(fixture()).unwrap();
    let input = unit.root_function_input().unwrap();
    let mut builder = MirBuilder::new();
    builder
        .function_state
        .resolved_binding_state
        .install(input.function())
        .unwrap();

    let error = builder.allocate_binding_id().unwrap_err();
    assert!(error.contains("legacy_allocation_forbidden"));
    assert_eq!(builder.core_ctx.next_binding_id, 0);
}

#[test]
fn duplicate_exact_declaration_materialization_is_rejected() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(fixture()).unwrap();
    let input = unit.root_function_input().unwrap();
    let mut state = ResolvedIdentityStateV1::new(input.function());
    let site = SourceBindingSiteV1::Parameter { index: 0 };
    state
        .publish_declaration(
            &site,
            BindingKindV1::Parameter { index: 0 },
            "arg",
            crate::mir::ValueId::new(0),
        )
        .unwrap();

    assert!(state
        .publish_declaration(
            &site,
            BindingKindV1::Parameter { index: 0 },
            "arg",
            crate::mir::ValueId::new(1),
        )
        .is_err());
}

#[test]
fn function_error_discards_unpublished_canonical_draft() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(fixture()).unwrap();
    let plan = CanonicalLoweringPreflightV1::verify(&unit).unwrap();
    let crate::mir::compiler::capability::CanonicalFirstFamilyPlanV1::CurrentCanonicalAPlus(plan) =
        plan
    else {
        panic!("parameter/outbox fixture must remain on temporary A+")
    };
    let (input, _flow, _completion, _block_expr_count) = plan.into_parts();

    let mut missing_authority = MirBuilder::new();
    crate::test_support::with_env_vars(&crate::test_support::METHODIZE_SELECTOR_UNSET, || missing_authority.prepare_module()).unwrap();
    let error = missing_authority
        .with_resolved_function_lowering_session("missing_authority/0", |builder| {
            builder.create_function_skeleton("missing_authority/0".into(), &[], &[])?;
            builder.finalize_function_draft(false)
        })
        .unwrap_err();
    assert!(error.to_string().contains("resolved_binding_authority"));
    assert!(!missing_authority
        .current_module
        .as_ref()
        .unwrap()
        .functions
        .contains_key("missing_authority/0"));

    let mut builder = MirBuilder::new();
    crate::test_support::with_env_vars(&crate::test_support::METHODIZE_SELECTOR_UNSET, || builder.prepare_module()).unwrap();
    let error = builder
        .with_resolved_function_lowering_session("canonical_fixture/1", |builder| {
            builder
                .function_state
                .resolved_binding_state
                .install(input.function())?;
            builder.create_function_skeleton("canonical_fixture/1".into(), &["arg".into()], &[])?;
            Err("[injected/canonical_body]".to_string())
        })
        .unwrap_err();

    assert_eq!(error.to_string(), "[injected/canonical_body]");
    assert!(!builder
        .current_module
        .as_ref()
        .unwrap()
        .functions
        .contains_key("canonical_fixture/1"));
    assert!(!builder.function_state.resolved_binding_state.is_installed());
}
