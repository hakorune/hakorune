//! CUT0-I0-ROOT0-RAW0: one raw root batch with retained root evidence.
//!
//! This module is disconnected from compiler ingress.  It is the only raw
//! root terminal that may consume the prepared Main/condition batch: the
//! physical collector receipts, the raw ledger, and `CompletedRootBodyV1`
//! become one unpublished completion product.

use super::module_draft_collector::CollectedDraftAdmissionReceiptV1;
use super::module_draft_collector::{
    FunctionDraftKeyV1, ModuleDraftCollectorV1, RootCollectorBatchPrepareErrorV1,
};
use super::module_invocation_drain::ConditionFnPolicyV1;
use super::module_invocation_identity::{ModuleInvocationBrandV1, ModuleInvocationTokenV1};
use super::module_invocation_owner_chain::{BrandedCollectorV1, InvocationBranded};
use super::raw_expansion_receipt_ledger::{
    RawCallableMainCompatibilityDispositionV1, RawExpansionReceiptLedgerErrorV1,
    RawExpansionReceiptLedgerV1, RawExpansionReservationV1, SealedRawExpansionReceiptLedgerV1,
};
use super::root_body_completion::CompletedRootBodyV1;
use super::root_draft_batch::{PreparedRootDraftBatchV1, RootDraftBatchErrorV1};

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum RawRootCompletionErrorV1 {
    NonRawFamily,
    ForeignBrand { expected: u64, actual: u64 },
    RootBatchPolicy,
    RootBatch(RootDraftBatchErrorV1),
    Collector(RootCollectorBatchPrepareErrorV1),
    Ledger(RawExpansionReceiptLedgerErrorV1),
    MissingMainReceipt,
    MissingConditionReceipt,
    UnexpectedReceipt { key: FunctionDraftKeyV1 },
    ReservationMismatch { key: FunctionDraftKeyV1 },
    SelectedCallableMainMissing,
}

impl std::fmt::Display for RawRootCompletionErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "[freeze:contract][raw_root_completion] {self:?}")
    }
}

impl std::error::Error for RawRootCompletionErrorV1 {}

#[derive(Debug)]
pub(in crate::mir::builder) struct RawInvocationRootWitnessV1 {
    brand: ModuleInvocationBrandV1,
    root_body: CompletedRootBodyV1,
    main: InvocationBranded<CollectedDraftAdmissionReceiptV1>,
    condition: InvocationBranded<CollectedDraftAdmissionReceiptV1>,
    callable_main: RawCallableMainCompatibilityDispositionV1,
    _seal: RawInvocationRootWitnessSealV1,
}

#[derive(Debug)]
struct RawInvocationRootWitnessSealV1;

impl RawInvocationRootWitnessV1 {
    pub(in crate::mir::builder) const fn brand(&self) -> ModuleInvocationBrandV1 {
        self.brand
    }

    pub(in crate::mir::builder) fn root_body(&self) -> &CompletedRootBodyV1 {
        &self.root_body
    }

    pub(in crate::mir::builder) fn callable_main(
        &self,
    ) -> RawCallableMainCompatibilityDispositionV1 {
        self.callable_main
    }
}

#[derive(Debug)]
pub(in crate::mir::builder) struct RawCompleteInvocationV1 {
    brand: ModuleInvocationBrandV1,
    collector: BrandedCollectorV1<ModuleDraftCollectorV1>,
    ledger: SealedRawExpansionReceiptLedgerV1,
    root: RawInvocationRootWitnessV1,
    _seal: RawCompleteInvocationSealV1,
}

#[derive(Debug)]
struct RawCompleteInvocationSealV1;

impl RawCompleteInvocationV1 {
    pub(in crate::mir::builder) const fn brand(&self) -> ModuleInvocationBrandV1 {
        self.brand
    }

    pub(in crate::mir::builder) fn collector(&self) -> &BrandedCollectorV1<ModuleDraftCollectorV1> {
        &self.collector
    }

    pub(in crate::mir::builder) fn ledger(&self) -> &SealedRawExpansionReceiptLedgerV1 {
        &self.ledger
    }

    pub(in crate::mir::builder) fn root(&self) -> &RawInvocationRootWitnessV1 {
        &self.root
    }

    /// Consume the raw completion proof for the later physical-owner bridge.
    /// The raw ledger, collector, and retained root witness stay together.
    pub(in crate::mir::builder) fn into_parts(
        self,
    ) -> (
        ModuleInvocationBrandV1,
        BrandedCollectorV1<ModuleDraftCollectorV1>,
        SealedRawExpansionReceiptLedgerV1,
        RawInvocationRootWitnessV1,
    ) {
        (self.brand, self.collector, self.ledger, self.root)
    }
}

fn check_reservation(
    reservation: &RawExpansionReservationV1,
    brand: ModuleInvocationBrandV1,
    key: FunctionDraftKeyV1,
    symbol: &str,
    arity: usize,
    policy: super::module_draft_collector::DraftPublicationPolicyV1,
) -> Result<(), RawRootCompletionErrorV1> {
    if reservation.brand() != brand {
        return Err(RawRootCompletionErrorV1::ForeignBrand {
            expected: brand.ordinal(),
            actual: reservation.brand().ordinal(),
        });
    }
    if reservation.key() != &key
        || reservation.symbol() != symbol
        || reservation.arity() != arity
        || reservation.policy() != policy
    {
        return Err(RawRootCompletionErrorV1::ReservationMismatch { key });
    }
    Ok(())
}

/// Consume one prepared raw root batch.  All identity and admission checks
/// happen before the collector or ledger can publish a root witness.
pub(in crate::mir::builder) fn complete_raw_root(
    token: &ModuleInvocationTokenV1,
    collector: BrandedCollectorV1<ModuleDraftCollectorV1>,
    ledger: RawExpansionReceiptLedgerV1,
    batch: PreparedRootDraftBatchV1,
    main_reservation: RawExpansionReservationV1,
    condition_reservation: RawExpansionReservationV1,
    callable_main: RawCallableMainCompatibilityDispositionV1,
) -> Result<RawCompleteInvocationV1, RawRootCompletionErrorV1> {
    if token.family() != super::module_invocation_identity::ModuleInvocationFamilyV1::Raw {
        return Err(RawRootCompletionErrorV1::NonRawFamily);
    }
    let brand = token.brand();
    if collector.brand() != brand {
        return Err(RawRootCompletionErrorV1::ForeignBrand {
            expected: brand.ordinal(),
            actual: collector.brand().ordinal(),
        });
    }
    if ledger.brand() != brand {
        return Err(RawRootCompletionErrorV1::ForeignBrand {
            expected: brand.ordinal(),
            actual: ledger.brand().ordinal(),
        });
    }
    if batch.policy() != ConditionFnPolicyV1::Required {
        return Err(RawRootCompletionErrorV1::RootBatchPolicy);
    }
    check_reservation(
        &main_reservation,
        brand,
        FunctionDraftKeyV1::Main,
        "main",
        0,
        super::module_draft_collector::DraftPublicationPolicyV1::LegacyReplaceWholePair,
    )?;
    check_reservation(
        &condition_reservation,
        brand,
        FunctionDraftKeyV1::SyntheticConditionFn,
        "condition_fn",
        1,
        super::module_draft_collector::DraftPublicationPolicyV1::CanonicalRejectDuplicate,
    )?;
    if callable_main == RawCallableMainCompatibilityDispositionV1::Selected {
        return Err(RawRootCompletionErrorV1::SelectedCallableMainMissing);
    }

    let prepared = collector
        .into_payload()
        .prepare_root_batch(batch)
        .map_err(|rejected| RawRootCompletionErrorV1::Collector(rejected.error().clone()))?;
    let (collector, branded_receipt) =
        prepared
            .commit_branded()
            .map_err(|_| RawRootCompletionErrorV1::ForeignBrand {
                expected: brand.ordinal(),
                actual: 0,
            })?;
    let (admissions, root_body, receipt_brand) = branded_receipt.into_parts();
    if receipt_brand != brand || root_body.brand() != brand {
        return Err(RawRootCompletionErrorV1::ForeignBrand {
            expected: brand.ordinal(),
            actual: root_body.brand().ordinal(),
        });
    }
    let mut main = None;
    let mut condition = None;
    for receipt in admissions.into_vec() {
        match receipt.payload().key() {
            FunctionDraftKeyV1::Main => main = Some(receipt),
            FunctionDraftKeyV1::SyntheticConditionFn => condition = Some(receipt),
            key => return Err(RawRootCompletionErrorV1::UnexpectedReceipt { key: key.clone() }),
        }
    }
    let main = main.ok_or(RawRootCompletionErrorV1::MissingMainReceipt)?;
    let condition = condition.ok_or(RawRootCompletionErrorV1::MissingConditionReceipt)?;
    let mut ledger = ledger;
    ledger
        .complete_required_root_batch(main_reservation, &main, condition_reservation, &condition)
        .map_err(RawRootCompletionErrorV1::Ledger)?;
    let ledger = ledger.seal().map_err(RawRootCompletionErrorV1::Ledger)?;
    let collector = InvocationBranded::from_source(brand, collector);
    Ok(RawCompleteInvocationV1 {
        brand,
        collector,
        ledger,
        root: RawInvocationRootWitnessV1 {
            brand,
            root_body,
            main,
            condition,
            callable_main,
            _seal: RawInvocationRootWitnessSealV1,
        },
        _seal: RawCompleteInvocationSealV1,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::main_pending_draft::{
        MainCompletionRequestV1, MainDraftIdentityV1, MainHeaderLoanV1, MainHeaderSourceV1,
    };
    use crate::mir::builder::module_draft_collector::{
        CompletedDraftSignatureViewV1, DraftPublicationPolicyV1,
    };
    use crate::mir::builder::module_invocation_identity::{
        ModuleInvocationFamilyV1, TestInvocationPreflightFactoryV1,
    };
    use crate::mir::builder::root_body_completion::{
        RootBodyCompletionTrackerV1, RootBodyResultV1,
    };
    use crate::mir::{
        BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirModule, MirType,
    };

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

    fn input() -> (
        ModuleInvocationTokenV1,
        BrandedCollectorV1<ModuleDraftCollectorV1>,
        RawExpansionReceiptLedgerV1,
        PreparedRootDraftBatchV1,
        RawExpansionReservationV1,
        RawExpansionReservationV1,
    ) {
        let mut factory = TestInvocationPreflightFactoryV1::new();
        let token = factory.mint(ModuleInvocationFamilyV1::Raw).unwrap();
        let brand = token.brand();
        let root_body = RootBodyCompletionTrackerV1::new_for_brand(brand)
            .complete(RootBodyResultV1::NoValue)
            .unwrap();
        let headers = MirModule::new("headers".into());
        let main = MainCompletionRequestV1::new(MainDraftIdentityV1::root(), root_body, false)
            .finish(
                draft("main", 0),
                MainHeaderLoanV1::new(&headers, MainHeaderSourceV1::InvocationCollector),
            )
            .unwrap();
        let batch = PreparedRootDraftBatchV1::prepare(
            main,
            Some(draft("condition_fn", 1)),
            ConditionFnPolicyV1::Required,
        )
        .unwrap();
        let mut ledger = RawExpansionReceiptLedgerV1::new_for_token(
            &token,
            RawCallableMainCompatibilityDispositionV1::NotSelected,
        );
        let main_reservation = ledger
            .reserve(
                super::super::raw_expansion_receipt_ledger::RawExpansionDraftRequestV1::root_main(),
            )
            .unwrap();
        let condition_reservation = ledger
            .reserve(super::super::raw_expansion_receipt_ledger::RawExpansionDraftRequestV1::required_condition_fn())
            .unwrap();
        (
            token,
            InvocationBranded::from_test(brand, ModuleDraftCollectorV1::with_brand(brand)),
            ledger,
            batch,
            main_reservation,
            condition_reservation,
        )
    }

    #[test]
    fn raw_root_success_retains_body_and_exact_root_receipts() {
        let (token, collector, ledger, batch, main, condition) = input();
        let complete = complete_raw_root(
            &token,
            collector,
            ledger,
            batch,
            main,
            condition,
            RawCallableMainCompatibilityDispositionV1::NotSelected,
        )
        .unwrap();
        assert_eq!(
            complete.root().root_body().result(),
            RootBodyResultV1::NoValue
        );
        assert_eq!(complete.ledger().final_count(), 2);
        assert_eq!(complete.collector().payload().symbol_count(), 2);
    }

    #[test]
    fn late_collector_admission_failure_happens_before_root_commit() {
        let (token, collector, ledger, batch, main, condition) = input();
        let brand = token.brand();
        let mut collector_payload = collector.into_payload();
        collector_payload
            .prepare_admission(
                FunctionDraftKeyV1::SyntheticConditionFn,
                "condition_fn".into(),
                1,
                DraftPublicationPolicyV1::CanonicalRejectDuplicate,
            )
            .unwrap()
            .seal(draft("condition_fn", 1))
            .unwrap()
            .collect();
        let error = complete_raw_root(
            &token,
            InvocationBranded::from_test(brand, collector_payload),
            ledger,
            batch,
            main,
            condition,
            RawCallableMainCompatibilityDispositionV1::NotSelected,
        )
        .unwrap_err();
        assert!(matches!(error, RawRootCompletionErrorV1::Collector(_)));
    }
}
