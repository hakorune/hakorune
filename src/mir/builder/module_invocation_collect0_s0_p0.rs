//! CUT0-I0-COLLECT0-S0 fixtures for real ledger/header co-seal.

use super::module_draft_collector::{
    CollectedDraftAdmissionReceiptV1, DraftPublicationPolicyV1, FunctionDraftKeyV1,
    ModuleDraftCollectorV1,
};
use super::module_invocation_collection::{
    canonical_source_from_parts, physical_receipt_from_test, raw_source_from_parts, seal_canonical_single,
    seal_raw, CanonicalSingleCollectedInvocationDraftSetV1, InvocationCollectionSealErrorV1,
    InvocationPhysicalReceiptV1, RawCollectedInvocationDraftSetV1,
};
use super::module_invocation_identity::TestInvocationPreflightFactoryV1;
use super::module_invocation_owner_chain::InvocationBranded;
use super::module_invocation_route_matrix::InvocationRootFamilyV1;
use super::raw_expansion_receipt_ledger::{
    RawExpansionDraftRequestV1, RawExpansionDraftRoleV1, RawCallableMainCompatibilityDispositionV1,
    RawExpansionReceiptLedgerV1,
};
use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, ParamDecl, Span};
use crate::mir::compiler::capability::{
    CanonicalLoweringPreflightV1, VerifiedResolvedOwnerHeaderV1,
};
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirType};

fn draft(symbol: &str, arity: usize) -> MirFunction {
    MirFunction::new(
        FunctionSignature {
            name: symbol.to_owned(),
            params: vec![MirType::Integer; arity],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    )
}

fn collect(
    collector: &mut ModuleDraftCollectorV1,
    key: FunctionDraftKeyV1,
    symbol: &str,
    arity: usize,
    policy: DraftPublicationPolicyV1,
) -> CollectedDraftAdmissionReceiptV1 {
    collector
        .prepare_admission(key, symbol.to_owned(), arity, policy)
        .unwrap()
        .seal(draft(symbol, arity))
        .unwrap()
        .collect()
}

fn raw_source_and_rows() -> (
    super::raw_expansion_receipt_ledger::SealedRawExpansionReceiptLedgerV1,
    Vec<InvocationPhysicalReceiptV1>,
    ModuleDraftCollectorV1,
) {
    let mut ledger = RawExpansionReceiptLedgerV1::new(
        RawCallableMainCompatibilityDispositionV1::NotSelected,
    );
    let mut ledger_collector = ModuleDraftCollectorV1::default();
    for (request, key, symbol, arity, policy) in [
        (
            RawExpansionDraftRequestV1::root_main(),
            FunctionDraftKeyV1::Main,
            "main",
            0,
            DraftPublicationPolicyV1::LegacyReplaceWholePair,
        ),
        (
            RawExpansionDraftRequestV1::required_condition_fn(),
            FunctionDraftKeyV1::SyntheticConditionFn,
            "condition_fn",
            1,
            DraftPublicationPolicyV1::CanonicalRejectDuplicate,
        ),
        (
            RawExpansionDraftRequestV1::legacy_discovered(
                RawExpansionDraftRoleV1::TopLevelFunction,
                "Top.f/0",
                0,
            )
            .unwrap(),
            FunctionDraftKeyV1::LegacySymbol("Top.f/0".into()),
            "Top.f/0",
            0,
            DraftPublicationPolicyV1::LegacyReplaceWholePair,
        ),
        (
            RawExpansionDraftRequestV1::legacy_discovered(
                RawExpansionDraftRoleV1::TopLevelFunction,
                "Top.f/0",
                0,
            )
            .unwrap(),
            FunctionDraftKeyV1::LegacySymbol("Top.f/0".into()),
            "Top.f/0",
            0,
            DraftPublicationPolicyV1::LegacyReplaceWholePair,
        ),
    ] {
        let reservation = ledger.reserve(request).unwrap();
        let receipt = collect(&mut ledger_collector, key, symbol, arity, policy);
        ledger.complete(reservation, receipt).unwrap();
    }
    let sealed = ledger.seal().unwrap();

    let mut collector = ModuleDraftCollectorV1::default();
    let main = collect(
        &mut collector,
        FunctionDraftKeyV1::Main,
        "main",
        0,
        DraftPublicationPolicyV1::LegacyReplaceWholePair,
    );
    let condition = collect(
        &mut collector,
        FunctionDraftKeyV1::SyntheticConditionFn,
        "condition_fn",
        1,
        DraftPublicationPolicyV1::CanonicalRejectDuplicate,
    );
    let _first = collect(
        &mut collector,
        FunctionDraftKeyV1::LegacySymbol("Top.f/0".into()),
        "Top.f/0",
        0,
        DraftPublicationPolicyV1::LegacyReplaceWholePair,
    );
    let final_f = collect(
        &mut collector,
        FunctionDraftKeyV1::LegacySymbol("Top.f/0".into()),
        "Top.f/0",
        0,
        DraftPublicationPolicyV1::LegacyReplaceWholePair,
    );
    let receipts = vec![main, condition, final_f];
    let mut factory = TestInvocationPreflightFactoryV1::new();
    let brand = factory.mint(InvocationRootFamilyV1::Raw).unwrap().brand();
    let receipts = receipts
        .into_iter()
        .map(|receipt| physical_receipt_from_test(brand, receipt))
        .collect();
    (sealed, receipts, collector)
}

fn header(name: &str, typed: bool, a_plus: bool) -> VerifiedResolvedOwnerHeaderV1 {
    let root = ASTNode::FunctionDeclaration {
        name: name.to_owned(),
        params: if typed { vec!["n".into()] } else { Vec::new() },
        param_decls: if typed {
            vec![ParamDecl {
                name: "n".into(),
                declared_type_name: Some("i64".into()),
            }]
        } else {
            Vec::new()
        },
        return_type_name: None,
        body: if a_plus {
            vec![ASTNode::If {
                condition: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                }),
                then_body: Vec::new(),
                else_body: None,
                span: Span::unknown(),
            }]
        } else {
            vec![ASTNode::Return {
                value: Some(Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(0),
                    span: Span::unknown(),
                })),
                span: Span::unknown(),
            }]
        },
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    };
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(root).unwrap();
    CanonicalLoweringPreflightV1::verify(&unit)
        .unwrap()
        .seal_resolved_owner_header_v1()
        .unwrap()
}

fn canonical_case(
    family: InvocationRootFamilyV1,
    header: VerifiedResolvedOwnerHeaderV1,
) -> Result<CanonicalSingleCollectedInvocationDraftSetV1, InvocationCollectionSealErrorV1> {
    let mut factory = TestInvocationPreflightFactoryV1::new();
    let token = factory.mint(family).unwrap();
    let brand = token.brand();
    let symbol = header.symbol().as_mir_name();
    let key = FunctionDraftKeyV1::CanonicalResolvedOwner(header.owner());
    let arity = header.arity();
    let mut collector = ModuleDraftCollectorV1::default();
    let receipt = collect(
        &mut collector,
        key,
        symbol,
        arity,
        DraftPublicationPolicyV1::CanonicalRejectDuplicate,
    );
    let source = canonical_source_from_parts(token, header)?;
    seal_canonical_single(
        source,
        InvocationBranded::from_test(brand, collector),
        physical_receipt_from_test(brand, receipt),
    )
}

#[test]
fn raw_seal_checks_final_ledger_and_replacement_history() {
    let (ledger, receipts, collector) = raw_source_and_rows();
    let mut factory = TestInvocationPreflightFactoryV1::new();
    let token = factory.mint(InvocationRootFamilyV1::Raw).unwrap();
    let brand = token.brand();
    let source = raw_source_from_parts(token, ledger).unwrap();
    let receipts = receipts
        .into_iter()
        .map(|receipt| InvocationBranded::from_test(brand, receipt.into_payload()))
        .collect();
    let sealed = seal_raw(source, InvocationBranded::from_test(brand, collector), receipts).unwrap();
    assert_eq!(sealed.receipt_count(), 3);
}

#[test]
fn canonical_a_plus_and_trivial_each_seal_one_exact_row() {
    let a_plus = canonical_case(
        InvocationRootFamilyV1::CanonicalAPlus,
        header("a_plus_collect", false, true),
    )
    .unwrap();
    assert_eq!(a_plus.collector_symbol_count(), 1);
    let trivial = canonical_case(
        InvocationRootFamilyV1::BindingSsaTrivial,
        header("trivial_collect", true, false),
    )
    .unwrap();
    assert_eq!(trivial.collector_symbol_count(), 1);
}

#[test]
fn foreign_brand_missing_row_and_wrong_policy_fail_before_co_seal() {
    let (ledger, receipts, collector) = raw_source_and_rows();
    let mut factory = TestInvocationPreflightFactoryV1::new();
    let source_token = factory.mint(InvocationRootFamilyV1::Raw).unwrap();
    let foreign = factory.mint(InvocationRootFamilyV1::Raw).unwrap().brand();
    let source = raw_source_from_parts(source_token, ledger).unwrap();
    let receipts = receipts
        .into_iter()
        .map(|receipt| InvocationBranded::from_test(foreign, receipt.into_payload()))
        .collect();
    assert!(matches!(
        seal_raw(source, InvocationBranded::from_test(foreign, collector), receipts),
        Err(InvocationCollectionSealErrorV1::ForeignOwner { .. })
    ));

    let (ledger, mut receipts, collector) = raw_source_and_rows();
    receipts.pop();
    let mut factory = TestInvocationPreflightFactoryV1::new();
    let token = factory.mint(InvocationRootFamilyV1::Raw).unwrap();
    let brand = token.brand();
    let source = raw_source_from_parts(token, ledger).unwrap();
    let receipts = receipts
        .into_iter()
        .map(|receipt| InvocationBranded::from_test(brand, receipt.into_payload()))
        .collect();
    assert!(matches!(
        seal_raw(source, InvocationBranded::from_test(brand, collector), receipts),
        Err(InvocationCollectionSealErrorV1::CardinalityMismatch { .. })
    ));

    let (ledger, mut receipts, mut collector) = raw_source_and_rows();
    let extra = collect(
        &mut collector,
        FunctionDraftKeyV1::LegacySymbol("extra/0".into()),
        "extra/0",
        0,
        DraftPublicationPolicyV1::LegacyReplaceWholePair,
    );
    let mut factory = TestInvocationPreflightFactoryV1::new();
    let token = factory.mint(InvocationRootFamilyV1::Raw).unwrap();
    let brand = token.brand();
    let source = raw_source_from_parts(token, ledger).unwrap();
    receipts.push(physical_receipt_from_test(brand, extra));
    let receipts = receipts
        .into_iter()
        .map(|receipt| InvocationBranded::from_test(brand, receipt.into_payload()))
        .collect();
    assert!(matches!(
        seal_raw(source, InvocationBranded::from_test(brand, collector), receipts),
        Err(InvocationCollectionSealErrorV1::CardinalityMismatch { .. })
    ));

    let mut factory = TestInvocationPreflightFactoryV1::new();
    let foreign_header = header("foreign_header", false, true);
    let error = canonical_source_from_parts(
        factory.mint(InvocationRootFamilyV1::BindingSsaTrivial).unwrap(),
        foreign_header,
    )
    .unwrap_err();
    assert!(matches!(
        error,
        InvocationCollectionSealErrorV1::SourceFamilyMismatch { .. }
    ));

    let mut factory = TestInvocationPreflightFactoryV1::new();
    let token = factory.mint(InvocationRootFamilyV1::CanonicalAPlus).unwrap();
    let brand = token.brand();
    let selected_header = header("wrong_policy", false, true);
    let symbol = selected_header.symbol().as_mir_name();
    let mut collector = ModuleDraftCollectorV1::default();
    let receipt = collect(
        &mut collector,
        FunctionDraftKeyV1::CanonicalResolvedOwner(selected_header.owner()),
        symbol,
        selected_header.arity(),
        DraftPublicationPolicyV1::LegacyReplaceWholePair,
    );
    let source = canonical_source_from_parts(token, selected_header).unwrap();
    assert!(matches!(
        seal_canonical_single(
            source,
            InvocationBranded::from_test(brand, collector),
            physical_receipt_from_test(brand, receipt),
        ),
        Err(InvocationCollectionSealErrorV1::PolicyMismatch { .. })
    ));
}
