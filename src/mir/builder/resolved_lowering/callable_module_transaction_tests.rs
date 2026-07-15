use crate::ast::{ASTNode, DeclarationAttrs, ParamDecl, Span};
use crate::mir::compiler::acyclic_callable_module_plan::VerifiedAcyclicCallableModulePlanV1;
use crate::mir::compiler::resolved_callable_module::VerifiedResolvedCallableModuleV1;
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

fn calling_function(name: &str, target: &str) -> ASTNode {
    let mut function = function(name);
    let ASTNode::FunctionDeclaration { body, .. } = &mut function else {
        unreachable!()
    };
    *body = vec![ASTNode::Return {
        value: Some(Box::new(ASTNode::FunctionCall {
            name: target.into(),
            arguments: vec![variable("n")],
            span: Span::unknown(),
        })),
        span: Span::unknown(),
    }];
    function
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
    let resolved = resolve(vec![
        calling_function("first", "second"),
        function("second"),
    ]);
    let plan = VerifiedAcyclicCallableModulePlanV1::verify(&resolved).unwrap();
    let mut builder = MirBuilder::new();

    let module = builder
        .build_acyclic_callable_module_candidate(plan)
        .unwrap();

    assert!(module.get_function("first/1").is_some());
    assert!(module.get_function("second/1").is_some());
}

#[test]
fn declaration_reorder_preserves_the_published_callable_symbol_set() {
    let mut observed = Vec::new();
    for functions in [
        vec![calling_function("first", "second"), function("second")],
        vec![function("second"), calling_function("first", "second")],
    ] {
        let resolved = resolve(functions);
        let plan = VerifiedAcyclicCallableModulePlanV1::verify(&resolved).unwrap();
        let mut builder = MirBuilder::new();
        let module = builder
            .build_acyclic_callable_module_candidate(plan)
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
fn typed_acyclic_plan_late_failure_keeps_atomic_publication_at_zero() {
    let resolved = resolve(vec![
        calling_function("first", "second"),
        function("second"),
    ]);
    let plan = VerifiedAcyclicCallableModulePlanV1::verify(&resolved).unwrap();
    let mut builder = MirBuilder::new();

    let error = builder
        .build_acyclic_callable_module_candidate_with(plan, |_builder, key, _plan| {
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
    let resolved = resolve(vec![
        calling_function("first", "second"),
        function("second"),
    ]);
    let plan = VerifiedAcyclicCallableModulePlanV1::verify(&resolved).unwrap();
    let mut builder = MirBuilder::new();

    let error = builder
        .build_acyclic_callable_module_candidate_with(plan, |_builder, key, _plan| {
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
