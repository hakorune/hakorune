//! CUT0-I0-FINAL0 focused fixtures.
//!
//! These tests exercise the disconnected route-specific finalization seam
//! using the real compiler-owned bridge.  No public ingress is wired here.

use super::canonical_finalization::{CanonicalFinalizationInputV1, CanonicalModuleFinalizerV1};
use super::capability::{CanonicalFirstFamilyPlanV1, CanonicalLoweringPreflightV1};
use super::source_bound_package::ExactCanonicalPreflightPlanV1;
use super::{MirCompiler, VerifiedResolvedCallableProgramV1, VerifiedResolvedSourceUnitV1};
use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, ParamDecl, Span};
use crate::mir::module_invocation_identity::ModuleInvocationBrandV1;

fn literal(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn single_source(name: &str) -> VerifiedResolvedSourceUnitV1 {
    VerifiedResolvedSourceUnitV1::resolve_function(ASTNode::FunctionDeclaration {
        name: name.into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: None,
        body: vec![ASTNode::Return {
            value: Some(Box::new(literal(1))),
            span: Span::unknown(),
        }],
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    })
    .expect("finalization fixture source must resolve")
}

fn callable_program() -> VerifiedResolvedCallableProgramV1 {
    let function = |name: &str, body: ASTNode| ASTNode::FunctionDeclaration {
        name: name.into(),
        params: vec!["x".into()],
        param_decls: vec![ParamDecl {
            name: "x".into(),
            declared_type_name: Some("i64".into()),
        }],
        return_type_name: Some("i64".into()),
        body: vec![ASTNode::Return {
            value: Some(Box::new(body)),
            span: Span::unknown(),
        }],
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    };
    VerifiedResolvedCallableProgramV1::resolve(ASTNode::Program {
        statements: vec![
            function(
                "caller",
                ASTNode::FunctionCall {
                    name: "callee".into(),
                    arguments: vec![ASTNode::Variable {
                        name: "x".into(),
                        span: Span::unknown(),
                    }],
                    span: Span::unknown(),
                },
            ),
            function(
                "callee",
                ASTNode::Variable {
                    name: "x".into(),
                    span: Span::unknown(),
                },
            ),
        ],
        span: Span::unknown(),
    })
    .expect("callable finalization fixture must resolve")
}

#[test]
fn final0_prepares_and_finalizes_single_route() {
    let source = single_source("final_single");
    let plan = match CanonicalLoweringPreflightV1::verify(&source).unwrap() {
        CanonicalFirstFamilyPlanV1::TrivialBindingSsa(plan) => {
            ExactCanonicalPreflightPlanV1::BindingSsaTrivial(plan)
        }
        _ => panic!("single fixture must remain trivial SSA"),
    };
    let mut compiler = MirCompiler::new();
    let package = compiler.bind_canonical_source(plan).unwrap();
    let finalized = compiler
        .begin_canonical_invocation(package, Some("final_single.hako"), "final_single".into())
        .unwrap()
        .lower()
        .unwrap()
        .collect()
        .unwrap()
        .complete()
        .unwrap()
        .prepare_drain()
        .unwrap()
        .drain()
        .prepare_finalization()
        .unwrap();
    let finalized = CanonicalModuleFinalizerV1::finalize(finalized).unwrap();
    let CanonicalFinalizationInputV1::Single(input) = finalized.input else {
        panic!("A+ finalization must remain single route")
    };
    assert_eq!(
        input.token.family(),
        crate::mir::module_invocation_identity::ModuleInvocationFamilyV1::BindingSsaTrivial
    );
    assert_eq!(input.builder.brand(), input.token.brand());
    assert_eq!(input.physical.brand, input.token.brand());
    assert!(compiler.builder.current_module.is_none());
}

#[test]
fn owner_retention0_finalizer_failure_keeps_complete_input() {
    let source = single_source("owner_retention_finalizer");
    let plan = match CanonicalLoweringPreflightV1::verify(&source).unwrap() {
        CanonicalFirstFamilyPlanV1::TrivialBindingSsa(plan) => {
            ExactCanonicalPreflightPlanV1::BindingSsaTrivial(plan)
        }
        _ => panic!("owner-retention fixture must remain trivial SSA"),
    };
    let mut compiler = MirCompiler::new();
    let package = compiler.bind_canonical_source(plan).unwrap();
    let input = compiler
        .begin_canonical_invocation(
            package,
            Some("owner_retention_finalizer.hako"),
            "owner_retention_finalizer".into(),
        )
        .unwrap()
        .lower()
        .unwrap()
        .collect()
        .unwrap()
        .complete()
        .unwrap()
        .prepare_drain()
        .unwrap()
        .drain()
        .prepare_finalization()
        .unwrap();
    let crate::mir::compiler::canonical_finalization::CanonicalFinalizationInputV1::Single(
        mut single,
    ) = input
    else {
        panic!("owner-retention fixture changed route shape")
    };
    single.physical.brand = ModuleInvocationBrandV1::legacy_test();
    let rejected =
        CanonicalModuleFinalizerV1::finalize(CanonicalFinalizationInputV1::Single(single))
            .expect_err("foreign physical brand must retain finalizer input");
    assert!(matches!(
        rejected.error,
        super::canonical_finalization::CanonicalFinalizationErrorV1::ForeignBrand
    ));
    let CanonicalFinalizationInputV1::Single(retained) = rejected.input else {
        panic!("rejected finalizer owner changed route shape")
    };
    assert_eq!(
        retained.physical.brand,
        ModuleInvocationBrandV1::legacy_test()
    );
    assert!(compiler.builder.current_module.is_none());
}

#[test]
fn final0_prepares_and_finalizes_callable_route() {
    let source = callable_program();
    let plan = super::acyclic_callable_module_plan::VerifiedAcyclicCallableModulePlanV1::verify(
        source.module(),
    )
    .expect("callable fixture must remain acyclic");
    let plan = ExactCanonicalPreflightPlanV1::BindingSsaAcyclic(plan);
    let mut compiler = MirCompiler::new();
    let package = compiler.bind_canonical_source(plan).unwrap();
    let finalized = compiler
        .begin_canonical_invocation(
            package,
            Some("final_callable.hako"),
            "final_callable".into(),
        )
        .unwrap()
        .lower()
        .unwrap()
        .collect()
        .unwrap()
        .complete()
        .unwrap()
        .prepare_drain()
        .unwrap()
        .drain()
        .prepare_finalization()
        .unwrap();
    let finalized = CanonicalModuleFinalizerV1::finalize(finalized).unwrap();
    let CanonicalFinalizationInputV1::Callable(input) = finalized.input else {
        panic!("acyclic finalization must remain callable route")
    };
    assert_eq!(
        input.token.family(),
        crate::mir::module_invocation_identity::ModuleInvocationFamilyV1::BindingSsaAcyclic
    );
    assert_eq!(input.builder.brand(), input.token.brand());
    assert_eq!(input.capability.brand(), input.token.brand());
    assert_eq!(input.physical.brand, input.token.brand());
}
