//! ROOT0-DRAIN0-MANIFEST0 focused source-projection proof.

use super::acyclic_callable_module_plan::VerifiedAcyclicCallableModulePlanV1;
use super::source_bound_package::ExactCanonicalPreflightPlanV1;
use super::{MirCompiler, VerifiedResolvedCallableProgramV1, VerifiedResolvedSourceUnitV1};
use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, ParamDecl, Span};
use crate::mir::canonical_physical_drain::CanonicalPhysicalDrainManifestV1;
use crate::mir::compiler::canonical_drain_manifest::CanonicalDrainIdentityV1;
use crate::mir::module_invocation_identity::ModuleInvocationFamilyV1;
use crate::mir::module_invocation_policy::{
    InvocationConditionPolicyV1, InvocationInventoryAuthorityV1, InvocationRootPolicyV1,
};

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

fn call(name: &str, argument: ASTNode) -> ASTNode {
    ASTNode::FunctionCall {
        name: name.into(),
        arguments: vec![argument],
        span: Span::unknown(),
    }
}

fn first_family_function(name: &str) -> ASTNode {
    ASTNode::FunctionDeclaration {
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
    }
}

fn callable_function(name: &str, body: ASTNode) -> ASTNode {
    ASTNode::FunctionDeclaration {
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
    }
}

#[test]
fn single_manifest_projects_retained_header_without_physical_evidence() {
    let unit =
        VerifiedResolvedSourceUnitV1::resolve_function(first_family_function("owner")).unwrap();
    let preflight = super::capability::CanonicalLoweringPreflightV1::verify(&unit).unwrap();
    let exact = ExactCanonicalPreflightPlanV1::from_first_family(preflight);
    let mut compiler = MirCompiler::new();
    let package = compiler.bind_canonical_source(exact).unwrap();
    let brand = package.brand();
    let manifest = package.project_drain_manifest().unwrap();

    assert_eq!(manifest.brand(), brand);
    assert_eq!(
        manifest.family(),
        ModuleInvocationFamilyV1::BindingSsaTrivial
    );
    assert_eq!(
        manifest.policy().inventory_authority(),
        InvocationInventoryAuthorityV1::CanonicalResolvedOwner
    );
    assert_eq!(
        manifest.policy().root_policy(),
        InvocationRootPolicyV1::ExactCanonicalOwner
    );
    assert_eq!(
        manifest.policy().condition_policy(),
        InvocationConditionPolicyV1::Forbidden
    );
    assert_eq!(manifest.rows().len(), 1);
    let row = &manifest.rows()[0];
    assert_eq!(row.symbol(), "owner/0");
    assert_eq!(row.arity(), 0);
    assert!(matches!(
        row.identity(),
        CanonicalDrainIdentityV1::ResolvedOwner(_)
    ));
}

#[test]
fn callable_manifest_projects_catalog_in_canonical_key_order() {
    let program = VerifiedResolvedCallableProgramV1::resolve(ASTNode::Program {
        statements: vec![
            callable_function("zeta", call("alpha", variable("x"))),
            callable_function("alpha", variable("x")),
        ],
        span: Span::unknown(),
    })
    .unwrap();
    let plan = VerifiedAcyclicCallableModulePlanV1::verify(program.module()).unwrap();
    let mut compiler = MirCompiler::new();
    let package = compiler
        .bind_canonical_source(ExactCanonicalPreflightPlanV1::BindingSsaAcyclic(plan))
        .unwrap();
    let manifest = package.project_drain_manifest().unwrap();
    assert_eq!(manifest.rows().len(), 2);
    assert_eq!(
        manifest.policy().inventory_authority(),
        InvocationInventoryAuthorityV1::CanonicalCallableCatalog
    );
    assert_eq!(
        manifest.policy().root_policy(),
        InvocationRootPolicyV1::ExactCallableCatalog
    );
    assert_eq!(manifest.rows()[0].symbol(), "alpha/1");
    assert_eq!(manifest.rows()[1].symbol(), "zeta/1");
    assert!(matches!(
        manifest.rows()[0].identity(),
        CanonicalDrainIdentityV1::Callable(_)
    ));
    assert!(matches!(
        manifest.rows()[1].identity(),
        CanonicalDrainIdentityV1::Callable(_)
    ));
}

#[test]
fn source_manifest_consumes_into_neutral_physical_rows() {
    let unit =
        VerifiedResolvedSourceUnitV1::resolve_function(first_family_function("owner")).unwrap();
    let preflight = super::capability::CanonicalLoweringPreflightV1::verify(&unit).unwrap();
    let exact = ExactCanonicalPreflightPlanV1::from_first_family(preflight);
    let mut compiler = MirCompiler::new();
    let package = compiler.bind_canonical_source(exact).unwrap();
    let physical = package.project_drain_manifest().unwrap().into_physical();

    assert_eq!(physical.rows_len(), 1);
    assert_eq!(
        physical.family(),
        ModuleInvocationFamilyV1::BindingSsaTrivial
    );
    let row = physical.single_row().expect("single physical row");
    assert_eq!(row.symbol(), "owner/0");
    assert_eq!(row.arity(), 0);
}

#[test]
fn callable_source_manifest_keeps_canonical_key_order_across_handoff() {
    let program = VerifiedResolvedCallableProgramV1::resolve(ASTNode::Program {
        statements: vec![
            callable_function("zeta", call("alpha", variable("x"))),
            callable_function("alpha", variable("x")),
        ],
        span: Span::unknown(),
    })
    .unwrap();
    let plan = VerifiedAcyclicCallableModulePlanV1::verify(program.module()).unwrap();
    let mut compiler = MirCompiler::new();
    let package = compiler
        .bind_canonical_source(ExactCanonicalPreflightPlanV1::BindingSsaAcyclic(plan))
        .unwrap();
    let physical = package.project_drain_manifest().unwrap().into_physical();
    let rows = physical.callable_rows().expect("callable physical rows");

    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].symbol(), "alpha/1");
    assert_eq!(rows[1].symbol(), "zeta/1");
}
