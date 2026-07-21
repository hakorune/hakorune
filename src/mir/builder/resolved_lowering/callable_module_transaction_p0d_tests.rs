//! ROUTEINV-P0d callable-batch transaction proof.
//!
//! This child module can observe the existing private draft transaction
//! directly. It adds no production hook, collector, catalog, or key map.

use crate::ast::{ASTNode, DeclarationAttrs, ParamDecl, Span};
use crate::mir::compiler::acyclic_callable_module_plan::VerifiedAcyclicCallableModulePlanV1;
use crate::mir::compiler::recursive_callable_module_plan::VerifiedRecursiveCallableModulePlanV1;
use crate::mir::compiler::resolved_callable_module::VerifiedResolvedCallableModuleV1;
use crate::mir::function::{FunctionSignature, MirFunction, MirModule};
use crate::mir::resolved_semantics::{
    CallableCatalogSealOutcomeV1, CanonicalCallableSymbolV1, VerifiedCallableHeaderSourceUnitV1,
    VerifiedOwnerFreeCallableCatalogSourceUnitV1,
};
use crate::mir::{BasicBlockId, EffectMask, MirType};

use super::{
    CallableModuleTransactionErrorV1, CanonicalResolvedBuildErrorV1, MirBuilder,
    VerifiedUnpublishedCallableDraftSetV1,
};

fn variable() -> ASTNode {
    ASTNode::Variable {
        name: "n".into(),
        span: Span::unknown(),
    }
}

fn call(target: &str) -> ASTNode {
    ASTNode::FunctionCall {
        name: target.into(),
        arguments: vec![variable()],
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

fn resolve(functions: Vec<ASTNode>) -> VerifiedResolvedCallableModuleV1 {
    let source = VerifiedCallableHeaderSourceUnitV1::seal_header_surface(ASTNode::Program {
        statements: functions,
        span: Span::unknown(),
    })
    .unwrap();
    let owner_free = VerifiedOwnerFreeCallableCatalogSourceUnitV1::seal(source).unwrap();
    let catalog = CallableCatalogSealOutcomeV1::seal(owner_free, 73).unwrap();
    VerifiedResolvedCallableModuleV1::resolve(catalog).unwrap()
}

fn fake_draft(name: &str, arity: usize, entry: u32) -> MirFunction {
    MirFunction::new(
        FunctionSignature {
            name: name.into(),
            params: vec![MirType::Integer; arity],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(entry),
    )
}

#[test]
fn acyclic_late_draft_failure_keeps_candidate_publication_at_zero() {
    let resolved = resolve(vec![
        function("first", call("second")),
        function("second", variable()),
    ]);
    let plan = VerifiedAcyclicCallableModulePlanV1::verify(&resolved).unwrap();
    let mut builder = MirBuilder::new();
    builder.prepare_module().unwrap();
    let mut completed = 0usize;

    let error = VerifiedUnpublishedCallableDraftSetV1::collect_acyclic_with(plan, |key, plan| {
        if key.name() == "second" {
            return Err(CanonicalResolvedBuildErrorV1::BuilderContract(
                "injected acyclic late failure".into(),
            ));
        }
        let draft = builder.lower_resolved_trivial_function_draft(plan)?;
        completed += 1;
        Ok(draft)
    })
    .err()
    .unwrap();

    assert_eq!(completed, 1);
    assert!(matches!(
        error,
        CallableModuleTransactionErrorV1::FunctionDraft { key, .. }
            if key.name() == "second"
    ));
    assert!(builder
        .current_module
        .as_ref()
        .unwrap()
        .functions
        .is_empty());
}

#[test]
fn recursive_late_draft_failure_keeps_candidate_publication_at_zero() {
    let resolved = resolve(vec![
        function("first", call("second")),
        function("second", call("second")),
    ]);
    let plan = VerifiedRecursiveCallableModulePlanV1::verify(&resolved).unwrap();
    let mut builder = MirBuilder::new();
    builder.prepare_module().unwrap();
    let mut completed = 0usize;

    let error = VerifiedUnpublishedCallableDraftSetV1::collect_recursive_with(plan, |key, plan| {
        if key.name() == "second" {
            return Err(CanonicalResolvedBuildErrorV1::BuilderContract(
                "injected recursive late failure".into(),
            ));
        }
        let draft = builder.lower_resolved_trivial_function_draft(plan)?;
        completed += 1;
        Ok(draft)
    })
    .err()
    .unwrap();

    assert_eq!(completed, 1);
    assert!(matches!(
        error,
        CallableModuleTransactionErrorV1::FunctionDraft { key, .. }
            if key.name() == "second"
    ));
    assert!(builder
        .current_module
        .as_ref()
        .unwrap()
        .functions
        .is_empty());
}

#[test]
fn atomic_publication_failure_preserves_the_preexisting_module_prefix() {
    let resolved = resolve(vec![
        function("first", call("second")),
        function("second", variable()),
    ]);
    let plan = VerifiedAcyclicCallableModulePlanV1::verify(&resolved).unwrap();
    let drafts = VerifiedUnpublishedCallableDraftSetV1::collect_acyclic_with(plan, |key, _plan| {
        let symbol = CanonicalCallableSymbolV1::from_name_arity(key.name(), key.arity() as usize);
        Ok(fake_draft(symbol.as_mir_name(), key.arity() as usize, 10))
    })
    .unwrap();
    let mut module = MirModule::new("callable-p0d".into());
    module.add_function(fake_draft("first/1", 1, 99));

    let error = drafts.publish_into(&mut module).unwrap_err();

    assert!(matches!(
        error,
        CallableModuleTransactionErrorV1::Publication(_)
    ));
    assert_eq!(module.functions.len(), 1);
    assert_eq!(
        module.get_function("first/1").unwrap().entry_block,
        BasicBlockId::new(99)
    );
    assert!(module.get_function("second/1").is_none());
}
