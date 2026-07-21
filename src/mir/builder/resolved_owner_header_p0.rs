//! ROUTEINV-P0c-SINGLEHDR-P0 disconnected family and collector projection proof.

use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, ParamDecl, Span};
use crate::mir::compiler::capability::{
    CanonicalFirstFamilyPlanV1, CanonicalLoweringPreflightV1, ResolvedOwnerHeaderFamilyV1,
    ResolvedOwnerHeaderSealErrorV1, VerifiedResolvedOwnerHeaderV1,
};
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirType};

use super::module_draft_collector::{
    DraftPublicationPolicyV1, FunctionDraftKeyV1, ModuleDraftAdmissionErrorV1,
    ModuleDraftCollectorV1,
};

fn integer(value: i64) -> ASTNode {
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

fn local(name: &str, value: i64) -> ASTNode {
    ASTNode::Local {
        variables: vec![name.into()],
        initial_values: vec![Some(Box::new(integer(value)))],
        declared_type_names: vec![None],
        span: Span::unknown(),
    }
}

fn assignment(name: &str, value: i64) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(variable(name)),
        value: Box::new(integer(value)),
        span: Span::unknown(),
    }
}

fn non_bool_if() -> ASTNode {
    ASTNode::If {
        condition: Box::new(integer(1)),
        then_body: vec![assignment("x", 1)],
        else_body: None,
        span: Span::unknown(),
    }
}

fn return_value(value: ASTNode) -> ASTNode {
    ASTNode::Return {
        value: Some(Box::new(value)),
        span: Span::unknown(),
    }
}

fn function(name: &str, params: &[(&str, Option<&str>)], body: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.into(),
        params: params.iter().map(|(name, _)| (*name).into()).collect(),
        param_decls: params
            .iter()
            .map(|(name, declared_type)| ParamDecl {
                name: (*name).into(),
                declared_type_name: declared_type.map(str::to_owned),
            })
            .collect(),
        return_type_name: None,
        body,
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn seal_header(root: ASTNode) -> VerifiedResolvedOwnerHeaderV1 {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(root).unwrap();
    let plan = CanonicalLoweringPreflightV1::verify(&unit).unwrap();
    plan.seal_resolved_owner_header_v1().unwrap()
}

fn draft(symbol: &str, arity: usize) -> MirFunction {
    MirFunction::new(
        FunctionSignature {
            name: symbol.into(),
            params: vec![MirType::Integer; arity],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    )
}

fn collect_header(collector: &mut ModuleDraftCollectorV1, header: &VerifiedResolvedOwnerHeaderV1) {
    let symbol = header.symbol().as_mir_name();
    collector
        .prepare_admission(
            FunctionDraftKeyV1::CanonicalResolvedOwner(header.owner()),
            symbol.into(),
            header.arity(),
            DraftPublicationPolicyV1::CanonicalRejectDuplicate,
        )
        .unwrap()
        .seal(draft(symbol, header.arity()))
        .unwrap()
        .collect();
}

#[test]
fn single_header_exact_families_nonzero_arity_and_source_reorder_are_stable() {
    let binding = seal_header(function(
        "binding_header",
        &[("n", Some("i64"))],
        vec![return_value(variable("n"))],
    ));
    let a_plus = seal_header(function(
        "a_plus_header",
        &[("n", None)],
        vec![local("x", 0), non_bool_if()],
    ));

    assert_eq!(
        binding.family(),
        ResolvedOwnerHeaderFamilyV1::TrivialBindingSsa
    );
    assert_eq!(binding.symbol().as_mir_name(), "binding_header/1");
    assert_eq!(binding.arity(), 1);
    assert_eq!(
        a_plus.family(),
        ResolvedOwnerHeaderFamilyV1::CurrentCanonicalAPlus
    );
    assert_eq!(a_plus.symbol().as_mir_name(), "a_plus_header/1");
    assert_eq!(a_plus.arity(), 1);

    let first = seal_header(function(
        "reorder_header",
        &[("n", Some("i64"))],
        vec![local("a", 1), local("b", 2), return_value(variable("n"))],
    ));
    let reordered = seal_header(function(
        "reorder_header",
        &[("n", Some("i64"))],
        vec![local("b", 2), local("a", 1), return_value(variable("n"))],
    ));
    assert_eq!(first.family(), reordered.family());
    assert_eq!(first.symbol(), reordered.symbol());
    assert_eq!(first.arity(), reordered.arity());
    assert_ne!(first.owner(), reordered.owner());
}

#[test]
fn single_header_rejects_separator_and_foreign_owner_family_pairing() {
    let bad_unit =
        VerifiedResolvedSourceUnitV1::resolve_function(function("bad/name", &[], vec![integer(1)]))
            .unwrap();
    let bad_plan = CanonicalLoweringPreflightV1::verify(&bad_unit).unwrap();
    assert!(matches!(
        bad_plan.seal_resolved_owner_header_v1(),
        Err(ResolvedOwnerHeaderSealErrorV1::SourceNameContainsPhysicalSeparator { .. })
    ));

    let first_unit = VerifiedResolvedSourceUnitV1::resolve_function(function(
        "foreign_header",
        &[("n", Some("i64"))],
        vec![return_value(variable("n"))],
    ))
    .unwrap();
    let foreign_unit = VerifiedResolvedSourceUnitV1::resolve_function(function(
        "foreign_header",
        &[("n", None)],
        vec![local("x", 0), non_bool_if()],
    ))
    .unwrap();
    let first_plan = CanonicalLoweringPreflightV1::verify(&first_unit).unwrap();
    let foreign_plan = CanonicalLoweringPreflightV1::verify(&foreign_unit).unwrap();
    let header = first_plan.seal_resolved_owner_header_v1().unwrap();
    assert!(matches!(
        header.require_same_plan(&foreign_plan),
        Err(ResolvedOwnerHeaderSealErrorV1::ForeignPlan { .. })
    ));
}

#[test]
fn single_header_projects_canonical_duplicate_symbol_and_arity_failures() {
    let first = seal_header(function(
        "collector_header",
        &[("n", Some("i64"))],
        vec![return_value(variable("n"))],
    ));
    let same_symbol_foreign_owner = seal_header(function(
        "collector_header",
        &[("n", Some("i64"))],
        vec![return_value(variable("n"))],
    ));
    let mut collector = ModuleDraftCollectorV1::default();
    collect_header(&mut collector, &first);
    let before = collector.receipt_proof_snapshot();

    let duplicate_key = collector.prepare_admission(
        FunctionDraftKeyV1::CanonicalResolvedOwner(first.owner()),
        "other_header/1".into(),
        1,
        DraftPublicationPolicyV1::CanonicalRejectDuplicate,
    );
    assert!(matches!(
        duplicate_key,
        Err(ModuleDraftAdmissionErrorV1::DuplicateKey(_))
    ));
    assert_eq!(collector.receipt_proof_snapshot(), before);

    let duplicate_symbol = collector.prepare_admission(
        FunctionDraftKeyV1::CanonicalResolvedOwner(same_symbol_foreign_owner.owner()),
        same_symbol_foreign_owner.symbol().as_mir_name().into(),
        same_symbol_foreign_owner.arity(),
        DraftPublicationPolicyV1::CanonicalRejectDuplicate,
    );
    assert!(matches!(
        duplicate_symbol,
        Err(ModuleDraftAdmissionErrorV1::DuplicateSymbol(_))
    ));
    assert_eq!(collector.receipt_proof_snapshot(), before);

    let symbol_header = seal_header(function("symbol_header", &[], vec![integer(1)]));
    let symbol_error = collector
        .prepare_admission(
            FunctionDraftKeyV1::CanonicalResolvedOwner(symbol_header.owner()),
            symbol_header.symbol().as_mir_name().into(),
            symbol_header.arity(),
            DraftPublicationPolicyV1::CanonicalRejectDuplicate,
        )
        .unwrap()
        .seal(draft("wrong_symbol/0", 0));
    assert!(matches!(
        symbol_error,
        Err(ModuleDraftAdmissionErrorV1::SymbolMismatch { .. })
    ));
    assert_eq!(collector.receipt_proof_snapshot(), before);

    let arity_header = seal_header(function("arity_header", &[], vec![integer(1)]));
    let arity_error = collector
        .prepare_admission(
            FunctionDraftKeyV1::CanonicalResolvedOwner(arity_header.owner()),
            arity_header.symbol().as_mir_name().into(),
            arity_header.arity(),
            DraftPublicationPolicyV1::CanonicalRejectDuplicate,
        )
        .unwrap()
        .seal(draft(arity_header.symbol().as_mir_name(), 1));
    assert!(matches!(
        arity_error,
        Err(ModuleDraftAdmissionErrorV1::ArityMismatch { .. })
    ));
    assert_eq!(collector.receipt_proof_snapshot(), before);
}

#[test]
fn single_header_family_selection_remains_exact_before_collector_projection() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function(
        "exact_family",
        &[("n", Some("i64"))],
        vec![return_value(variable("n"))],
    ))
    .unwrap();
    let plan = CanonicalLoweringPreflightV1::verify(&unit).unwrap();
    assert!(matches!(
        plan,
        CanonicalFirstFamilyPlanV1::TrivialBindingSsa(_)
    ));
}
