use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, Span};
use crate::mir::compiler::capability::CanonicalLoweringPreflightV1;
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::resolved_semantics::{BindingKindV1, SourceBindingSiteV1};
use crate::mir::{MirInstruction, MirType};

use super::identity::ResolvedIdentityStateV1;
use super::*;

fn literal(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
        span: Span::unknown(),
    }
}

fn add(left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left: Box::new(left),
        right: Box::new(right),
        span: Span::unknown(),
    }
}

fn fixture() -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "canonical_fixture".into(),
        params: vec!["arg".into()],
        param_decls: Vec::new(),
        return_type_name: None,
        body: vec![
            ASTNode::Local {
                variables: vec!["x".into()],
                initial_values: vec![Some(Box::new(add(variable("arg"), literal(1))))],
                declared_type_names: vec![None],
                span: Span::unknown(),
            },
            ASTNode::Assignment {
                target: Box::new(variable("x")),
                value: Box::new(add(variable("x"), literal(1))),
                span: Span::unknown(),
            },
            ASTNode::Outbox {
                variables: vec!["result".into()],
                initial_values: vec![None],
                span: Span::unknown(),
            },
            ASTNode::Return {
                value: Some(Box::new(variable("x"))),
                span: Span::unknown(),
            },
        ],
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn trivial_returning(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: None,
        body: vec![ASTNode::Return {
            value: Some(Box::new(value)),
            span: Span::unknown(),
        }],
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
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
    let function = &module.functions["canonical_fixture/1"];
    assert_eq!(function.metadata.outbox_bindings, vec!["result"]);
}

#[test]
fn public_resolved_route_produces_verifier_clean_mir() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(fixture()).unwrap();
    let mut compiler = crate::mir::MirCompiler::with_options(false);
    let result = compiler
        .compile_resolved(unit.lowering_input(), Some("canonical_fixture.hako"))
        .unwrap();

    assert!(
        result.verification_result.is_ok(),
        "{:?}",
        result.verification_result
    );
    assert!(result.module.functions.contains_key("canonical_fixture/1"));
}

#[test]
fn trivial_float_route_keeps_exact_type_and_emits_no_legacy_release() {
    let expression = ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left: Box::new(ASTNode::Literal {
            value: LiteralValue::Float(1.5),
            span: Span::unknown(),
        }),
        right: Box::new(ASTNode::Literal {
            value: LiteralValue::Float(2.5),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(trivial_returning(
        "trivial_float",
        expression,
    ))
    .unwrap();
    let result = crate::mir::MirCompiler::with_options(false)
        .compile_resolved(unit.lowering_input(), Some("trivial_float.hako"))
        .unwrap();
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
    let expression = ASTNode::BinaryOp {
        operator: BinaryOperator::Equal,
        left: Box::new(ASTNode::Literal {
            value: LiteralValue::Bool(true),
            span: Span::unknown(),
        }),
        right: Box::new(ASTNode::Literal {
            value: LiteralValue::Bool(false),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(trivial_returning(
        "trivial_bool_compare",
        expression,
    ))
    .unwrap();
    let result = crate::mir::MirCompiler::with_options(false)
        .compile_resolved(unit.lowering_input(), Some("trivial_bool_compare.hako"))
        .unwrap();
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
    missing_authority.prepare_module().unwrap();
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
    builder.prepare_module().unwrap();
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
