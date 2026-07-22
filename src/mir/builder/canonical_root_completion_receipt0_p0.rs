//! RECEIPT0 focused fixtures for collector/receipt inseparable products.

use super::module_draft_collector::{
    CallableCollectorDraftEntryV1, CompletedDraftSignatureViewV1, FunctionDraftKeyV1,
    ModuleDraftCollectorV1,
};
use super::module_invocation_identity::ModuleInvocationBrandV1;
use super::module_invocation_owner_chain::InvocationBranded;
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

fn collector(brand: ModuleInvocationBrandV1) -> InvocationBranded<ModuleDraftCollectorV1> {
    InvocationBranded::from_test(brand, ModuleDraftCollectorV1::with_brand(brand))
}

#[test]
fn single_collection_moves_collector_and_exact_receipt_together() {
    let brand = ModuleInvocationBrandV1::legacy_test();
    let product = collector(brand)
        .collect_canonical_single(
            FunctionDraftKeyV1::LegacySymbol("Canonical.f/0".into()),
            "Canonical.f/0".into(),
            0,
            draft("Canonical.f/0", 0),
        )
        .unwrap();
    let (collector, receipt) = product.into_parts();
    assert_eq!(collector.brand(), brand);
    assert_eq!(receipt.brand(), brand);
    assert_eq!(collector.payload().symbol_count(), 1);
    assert_eq!(receipt.payload().collector_brand(), Some(brand));
}

#[test]
fn foreign_or_duplicate_single_collection_rejects_without_prefix_mutation() {
    let brand = ModuleInvocationBrandV1::legacy_test();
    let error = InvocationBranded::from_test(brand, ModuleDraftCollectorV1::default())
        .collect_canonical_single(
            FunctionDraftKeyV1::LegacySymbol("Canonical.f/0".into()),
            "Canonical.f/0".into(),
            0,
            draft("Canonical.f/0", 0),
        )
        .unwrap_err();
    assert_eq!(error.collector().payload().symbol_count(), 0);

    let product = collector(brand)
        .collect_canonical_single(
            FunctionDraftKeyV1::LegacySymbol("Canonical.f/0".into()),
            "Canonical.f/0".into(),
            0,
            draft("Canonical.f/0", 0),
        )
        .unwrap();
    let (collector, _) = product.into_parts();
    let duplicate = collector
        .collect_canonical_single(
            FunctionDraftKeyV1::LegacySymbol("Canonical.f/0".into()),
            "Canonical.f/0".into(),
            0,
            draft("Canonical.f/0", 0),
        )
        .unwrap_err();
    assert_eq!(duplicate.collector().payload().symbol_count(), 1);
}

#[test]
fn callable_batch_product_uses_collector_brand_for_the_whole_receipt() {
    let brand = ModuleInvocationBrandV1::legacy_test();
    let prepared = ModuleDraftCollectorV1::with_brand(brand)
        .prepare_callable_batch(vec![CallableCollectorDraftEntryV1::new(
            FunctionDraftKeyV1::LegacySymbol("Callable.f/0".into()),
            "Callable.f/0".into(),
            0,
            draft("Callable.f/0", 0),
        )])
        .unwrap();
    let product = prepared.collect_all_branded().unwrap();
    let (collector, receipt) = product.into_parts();
    assert_eq!(collector.brand(), brand);
    assert_eq!(receipt.brand(), brand);
    assert_eq!(receipt.payload().len(), 1);
    assert_eq!(receipt.payload().admissions()[0].collector_brand(), Some(brand));
}
