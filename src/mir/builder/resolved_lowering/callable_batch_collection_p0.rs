//! CUT0-I0-COLLECT0-BATCH0 fixtures.
//!
//! These tests use the existing verified unpublished callable owner, the real
//! catalog source, and the disconnected collector co-seal. No production
//! callable ingress is changed.

use crate::ast::{ASTNode, DeclarationAttrs, ParamDecl, Span};
use crate::mir::builder::module_draft_collector::{
    CompletedDraftSignatureViewV1, DraftPublicationPolicyV1, FunctionDraftKeyV1,
    ModuleDraftCollectorV1,
};
use crate::mir::builder::module_invocation_callable_batch::{
    physical_receipt_from_test, seal_callable_batch, shell_fact_from_test, source_from_test,
    CallableBatchSealErrorV1, CallableBatchSourceErrorV1,
};
use crate::mir::builder::module_invocation_identity::{
    ModuleInvocationFamilyV1, TestInvocationPreflightFactoryV1,
};
use crate::mir::builder::module_invocation_owner_chain::InvocationBranded;
use crate::mir::builder::module_lowering_shell::ModuleLoweringShellV1;
use crate::mir::canonical_recursive_callable_module_capability::
    CanonicalRecursiveCallableModuleCapabilityV1;
use crate::mir::compiler::acyclic_callable_module_plan::VerifiedAcyclicCallableModulePlanV1;
use crate::mir::compiler::recursive_callable_module_plan::VerifiedRecursiveCallableModulePlanV1;
use crate::mir::compiler::resolved_callable_module::VerifiedResolvedCallableModuleV1;
use crate::mir::function::{FunctionSignature, MirFunction};
use crate::mir::resolved_semantics::{
    CallableCatalogSealOutcomeV1, VerifiedCallableHeaderSourceUnitV1,
    VerifiedOwnerFreeCallableCatalogSourceUnitV1,
};
use crate::mir::{BasicBlockId, EffectMask, MirType};

use super::{CanonicalResolvedBuildErrorV1, MirBuilder, VerifiedUnpublishedCallableDraftSetV1};

fn variable() -> ASTNode {
    ASTNode::Variable {
        name: "n".into(),
        span: Span::unknown(),
    }
}

fn function(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.into(),
        params: vec!["n".into()],
        param_decls: vec![ParamDecl {
            name: "n".into(),
            declared_type_name: Some("i64".into()),
        }],
        return_type_name: Some("i64".into()),
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

fn call(name: &str) -> ASTNode {
    ASTNode::FunctionCall {
        name: name.into(),
        arguments: vec![variable()],
        span: Span::unknown(),
    }
}

fn resolve(functions: Vec<ASTNode>) -> VerifiedResolvedCallableModuleV1 {
    let source = VerifiedCallableHeaderSourceUnitV1::seal_header_surface(ASTNode::Program {
        statements: functions,
        span: Span::unknown(),
    })
    .unwrap();
    let owner_free = VerifiedOwnerFreeCallableCatalogSourceUnitV1::seal(source).unwrap();
    let catalog = CallableCatalogSealOutcomeV1::seal(owner_free, 91).unwrap();
    VerifiedResolvedCallableModuleV1::resolve(catalog).unwrap()
}

fn acyclic_source() -> VerifiedResolvedCallableModuleV1 {
    resolve(vec![function("first", call("second")), function("second", variable())])
}

fn recursive_source() -> VerifiedResolvedCallableModuleV1 {
    resolve(vec![function("even", call("odd")), function("odd", call("even"))])
}

fn collect_acyclic(
    source: &VerifiedResolvedCallableModuleV1,
) -> super::PreparedCallableCollectorInvocationV1<'_> {
    let plan = VerifiedAcyclicCallableModulePlanV1::verify(source).unwrap();
    let mut builder = MirBuilder::new();
    builder.prepare_module().unwrap();
    let drafts = VerifiedUnpublishedCallableDraftSetV1::collect_acyclic_with(plan, |_, plan| {
        builder
            .lower_resolved_trivial_function_draft(plan)
            .map_err(CanonicalResolvedBuildErrorV1::from)
    })
    .unwrap();
    drafts
        .prepare_collector_batch(ModuleDraftCollectorV1::default())
        .unwrap()
}

fn collect_recursive(
    source: &VerifiedResolvedCallableModuleV1,
) -> super::PreparedCallableCollectorInvocationV1<'_> {
    let plan = VerifiedRecursiveCallableModulePlanV1::verify(source).unwrap();
    let mut builder = MirBuilder::new();
    builder.prepare_module().unwrap();
    let drafts = VerifiedUnpublishedCallableDraftSetV1::collect_recursive_with(plan, |_, plan| {
        builder
            .lower_resolved_trivial_function_draft(plan)
            .map_err(CanonicalResolvedBuildErrorV1::from)
    })
    .unwrap();
    drafts
        .prepare_collector_batch(ModuleDraftCollectorV1::default())
        .unwrap()
}

#[test]
fn exact_catalog_batch_co_seals_one_physical_receipt() {
    let source = acyclic_source();
    let prepared = collect_acyclic(&source);
    let (source_ref, collector, receipt) = prepared.collect_all();
    assert_eq!(source_ref.functions_by_key().len(), 2);
    assert_eq!(collector.symbol_count(), 2);
    assert_eq!(receipt.len(), 2);
    assert!(receipt.admissions().iter().all(|admission| {
        admission.policy() == DraftPublicationPolicyV1::CanonicalRejectDuplicate
            && matches!(
                admission.replacement(),
                crate::mir::builder::module_draft_collector::CollectedDraftReplacementDispositionV1::Inserted
            )
    }));

    let mut factory = TestInvocationPreflightFactoryV1::new();
    let token = factory.mint(ModuleInvocationFamilyV1::BindingSsaAcyclic).unwrap();
    let source_proof = source_from_test(token, source_ref, None).unwrap();
    let brand = source_proof.brand();
    let collected = seal_callable_batch(
        source_proof,
        InvocationBranded::from_test(brand, collector),
        physical_receipt_from_test(brand, receipt),
    )
    .unwrap();
    assert_eq!(collected.receipt_count(), 2);
    assert!(!collected.is_recursive());
}

#[test]
fn late_collector_collision_rejects_without_delta() {
    let source = acyclic_source();
    let plan = VerifiedAcyclicCallableModulePlanV1::verify(&source).unwrap();
    let mut builder = MirBuilder::new();
    builder.prepare_module().unwrap();
    let drafts = VerifiedUnpublishedCallableDraftSetV1::collect_acyclic_with(plan, |_, plan| {
        builder
            .lower_resolved_trivial_function_draft(plan)
            .map_err(CanonicalResolvedBuildErrorV1::from)
    })
    .unwrap();
    let mut collector = ModuleDraftCollectorV1::default();
    let duplicate = MirFunction::new(
        FunctionSignature {
            name: "second/1".into(),
            params: vec![MirType::Integer],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    collector
        .prepare_admission(
            FunctionDraftKeyV1::CanonicalCallable(
                source.functions_by_key().keys().next_back().unwrap().clone(),
            ),
            "second/1".into(),
            1,
            DraftPublicationPolicyV1::CanonicalRejectDuplicate,
        )
        .unwrap()
        .seal(duplicate)
        .unwrap()
        .collect();
    let rejected = drafts.prepare_collector_batch(collector).unwrap_err();
    assert_eq!(rejected.collector().symbol_count(), 1);
    assert!(matches!(
        rejected.error(),
        crate::mir::builder::module_draft_collector::CallableCollectorBatchPrepareErrorV1::Admission { .. }
    ));
}

#[test]
fn recursive_batch_preserves_one_shell_capability_marker() {
    let source = recursive_source();
    let prepared = collect_recursive(&source);
    let (source_ref, collector, receipt) = prepared.collect_all();
    let mut shell = ModuleLoweringShellV1::from_empty_module(crate::mir::MirModule::new(
        "recursive-batch".into(),
    ))
    .unwrap();
    shell
        .with_port(|port| {
            port.install_callable_batch_shell_fact_for_test(
                ModuleInvocationFamilyV1::BindingSsaRecursive,
            )
        })
        .unwrap();
    let duplicate = shell.with_port(|port| {
        port.install_callable_batch_shell_fact_for_test(ModuleInvocationFamilyV1::BindingSsaRecursive)
    });
    assert!(duplicate.is_err());
    let capability = shell
        .with_port(|port| port.metadata().canonical_recursive_callable_module_capability)
        .unwrap();
    let mut factory = TestInvocationPreflightFactoryV1::new();
    let token = factory
        .mint(ModuleInvocationFamilyV1::BindingSsaRecursive)
        .unwrap();
    let source_proof = source_from_test(token, source_ref, capability.into()).unwrap();
    let brand = source_proof.brand();
    let collected = seal_callable_batch(
        source_proof,
        InvocationBranded::from_test(brand, collector),
        physical_receipt_from_test(brand, receipt),
    )
    .unwrap();
    assert_eq!(collected.receipt_count(), 2);
    assert!(collected.is_recursive());
}

#[test]
fn non_callable_family_is_rejected_before_source_co_seal() {
    let source = acyclic_source();
    let mut factory = TestInvocationPreflightFactoryV1::new();
    let token = factory.mint(ModuleInvocationFamilyV1::Raw).unwrap();
    assert_eq!(
        source_from_test(token, &source, None).unwrap_err(),
        CallableBatchSourceErrorV1::UnsupportedFamily(ModuleInvocationFamilyV1::Raw)
    );
    let _ = CallableBatchSealErrorV1::CardinalityMismatch { expected: 0, actual: 0 };
    let _ = shell_fact_from_test(ModuleInvocationFamilyV1::BindingSsaAcyclic, None).unwrap();
    let _ = CanonicalRecursiveCallableModuleCapabilityV1::v1();
}

#[test]
fn callable_family_cannot_pair_with_a_foreign_verified_source_plan() {
    let source = acyclic_source();
    let mut factory = TestInvocationPreflightFactoryV1::new();
    let token = factory
        .mint(ModuleInvocationFamilyV1::BindingSsaRecursive)
        .unwrap();
    assert_eq!(
        source_from_test(
            token,
            &source,
            Some(CanonicalRecursiveCallableModuleCapabilityV1::v1()),
        )
        .unwrap_err(),
        CallableBatchSourceErrorV1::SourcePlanMismatch {
            family: ModuleInvocationFamilyV1::BindingSsaRecursive,
        }
    );
}

#[test]
fn foreign_callable_brand_fails_before_co_seal() {
    let source = acyclic_source();
    let prepared = collect_acyclic(&source);
    let (source_ref, collector, receipt) = prepared.collect_all();
    let mut factory = TestInvocationPreflightFactoryV1::new();
    let source_token = factory
        .mint(ModuleInvocationFamilyV1::BindingSsaAcyclic)
        .unwrap();
    let foreign_token = factory
        .mint(ModuleInvocationFamilyV1::BindingSsaAcyclic)
        .unwrap();
    let source_proof = source_from_test(source_token, source_ref, None).unwrap();
    let source_brand = source_proof.brand();
    let foreign_brand = foreign_token.brand();
    let error = seal_callable_batch(
        source_proof,
        InvocationBranded::from_test(foreign_brand, collector),
        physical_receipt_from_test(source_brand, receipt),
    )
    .unwrap_err();
    assert!(matches!(error, CallableBatchSealErrorV1::ForeignOwner { .. }));
}
