use crate::ast::{ASTNode, DeclarationAttrs, ParamDecl, Span};
use crate::mir::compiler::resolved_callable_module::VerifiedResolvedCallableModuleV1;
use crate::mir::compiler::resolved_callable_module_preflight::VerifiedCallableModulePreflightV1;
use crate::mir::function::{FunctionSignature, MirFunction};
use crate::mir::resolved_semantics::{
    CallableCatalogSealOutcomeV1, VerifiedCallableHeaderSourceUnitV1,
    VerifiedOwnerFreeCallableCatalogSourceUnitV1,
};
use crate::mir::{BasicBlockId, EffectMask, MirType};

use super::callable_module_transaction::CallableModuleTransactionErrorV1;
use super::{CanonicalResolvedBuildErrorV1, MirBuilder};

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
        span: Span::unknown(),
    }
}

fn function(name: &str) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.into(),
        params: vec!["n".into()],
        param_decls: vec![ParamDecl {
            name: "n".into(),
            declared_type_name: Some("i64".into()),
        }],
        return_type_name: Some("i64".into()),
        body: vec![ASTNode::Return {
            value: Some(Box::new(variable("n"))),
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
    let catalog = CallableCatalogSealOutcomeV1::seal(owner_free, 41).unwrap();
    VerifiedResolvedCallableModuleV1::resolve(catalog).unwrap()
}

fn fake_draft(symbol: String, arity: usize) -> MirFunction {
    MirFunction::new(
        FunctionSignature {
            name: symbol,
            params: vec![MirType::Integer; arity],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    )
}

#[test]
fn lowers_all_drafts_before_atomic_candidate_publication() {
    let resolved = resolve(vec![function("first"), function("second")]);
    let preflight = VerifiedCallableModulePreflightV1::verify(&resolved).unwrap();
    let mut builder = MirBuilder::new();

    let module = builder
        .build_resolved_callable_module_candidate(preflight)
        .unwrap();

    assert!(module.get_function("first/1").is_some());
    assert!(module.get_function("second/1").is_some());
}

#[test]
fn declaration_reorder_preserves_the_published_callable_symbol_set() {
    let mut observed = Vec::new();
    for functions in [
        vec![function("first"), function("second")],
        vec![function("second"), function("first")],
    ] {
        let resolved = resolve(functions);
        let preflight = VerifiedCallableModulePreflightV1::verify(&resolved).unwrap();
        let mut builder = MirBuilder::new();
        let module = builder
            .build_resolved_callable_module_candidate(preflight)
            .unwrap();
        observed.push(
            module
                .function_names()
                .into_iter()
                .filter(|name| name.ends_with("/1"))
                .cloned()
                .collect::<Vec<_>>(),
        );
    }
    assert_eq!(observed[0], observed[1]);
}

#[test]
fn late_draft_failure_leaves_candidate_function_publication_at_zero() {
    let resolved = resolve(vec![function("first"), function("second")]);
    let preflight = VerifiedCallableModulePreflightV1::verify(&resolved).unwrap();
    let mut builder = MirBuilder::new();

    let error = builder
        .build_resolved_callable_module_candidate_with(preflight, |_builder, key, _plan| {
            if key.name() == "second" {
                return Err(CanonicalResolvedBuildErrorV1::BuilderContract(
                    "injected late draft failure".to_string(),
                ));
            }
            Ok(fake_draft(
                format!("{}/{}", key.name(), key.arity()),
                key.arity() as usize,
            ))
        })
        .err()
        .expect("late failure must reject the unpublished set");

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
fn symbol_drift_rejects_before_candidate_publication() {
    let resolved = resolve(vec![function("first"), function("second")]);
    let preflight = VerifiedCallableModulePreflightV1::verify(&resolved).unwrap();
    let mut builder = MirBuilder::new();

    let error = builder
        .build_resolved_callable_module_candidate_with(preflight, |_builder, key, _plan| {
            Ok(fake_draft("wrong/1".to_string(), key.arity() as usize))
        })
        .err()
        .expect("symbol drift must reject the unpublished set");

    assert!(matches!(
        error,
        CallableModuleTransactionErrorV1::SymbolMismatch { .. }
    ));
    assert!(builder
        .current_module
        .as_ref()
        .unwrap()
        .functions
        .is_empty());
}
