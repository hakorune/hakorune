//! CUT0-I0-ROOT0-RAW0: one raw root batch with retained root evidence.
//!
//! This module is disconnected from compiler ingress.  It is the only raw
//! root terminal that may consume the prepared Main/condition batch: the
//! physical collector receipts, the raw ledger, and `CompletedRootBodyV1`
//! become one unpublished completion product.

use super::module_draft_collector::CollectedDraftAdmissionReceiptV1;
use super::module_draft_collector::{FunctionDraftKeyV1, ModuleDraftCollectorV1};
use super::module_invocation_drain::ConditionFnPolicyV1;
use super::module_invocation_identity::{ModuleInvocationBrandV1, ModuleInvocationTokenV1};
use super::module_invocation_owner_chain::{BrandedCollectorV1, InvocationBranded};
use super::raw_expansion_receipt_ledger::{
    RawCallableMainCompatibilityDispositionV1, RawExpansionReceiptLedgerV1,
    RawExpansionReservationV1, SealedRawExpansionReceiptLedgerV1,
};
use super::root_body_completion::CompletedRootBodyV1;
use super::root_draft_batch::PreparedRootDraftBatchV1;

#[derive(Debug)]
pub(in crate::mir) struct RawInvocationRootWitnessV1 {
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
    pub(in crate::mir) const fn brand(&self) -> ModuleInvocationBrandV1 {
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
    token: ModuleInvocationTokenV1,
    collector: BrandedCollectorV1<ModuleDraftCollectorV1>,
    ledger: SealedRawExpansionReceiptLedgerV1,
    root: RawInvocationRootWitnessV1,
    _seal: RawCompleteInvocationSealV1,
}

#[derive(Debug)]
struct RawCompleteInvocationSealV1;

impl RawCompleteInvocationV1 {
    pub(in crate::mir::builder) const fn brand(&self) -> ModuleInvocationBrandV1 {
        self.token.brand()
    }

    pub(in crate::mir::builder) const fn token(&self) -> &ModuleInvocationTokenV1 {
        &self.token
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
        ModuleInvocationTokenV1,
        BrandedCollectorV1<ModuleDraftCollectorV1>,
        SealedRawExpansionReceiptLedgerV1,
        RawInvocationRootWitnessV1,
    ) {
        (self.token, self.collector, self.ledger, self.root)
    }

    pub(in crate::mir::builder) fn from_committed_parts(
        token: ModuleInvocationTokenV1,
        collector: BrandedCollectorV1<ModuleDraftCollectorV1>,
        ledger: SealedRawExpansionReceiptLedgerV1,
        root_body: CompletedRootBodyV1,
        main: InvocationBranded<CollectedDraftAdmissionReceiptV1>,
        condition: InvocationBranded<CollectedDraftAdmissionReceiptV1>,
        callable_main: RawCallableMainCompatibilityDispositionV1,
    ) -> Self {
        let brand = token.brand();
        Self {
            token,
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
        }
    }
}

/// Compatibility wrapper for disconnected fixtures. The input owner carries
/// the token by value; preflight is the only fallible phase and commit is the
/// only consuming success terminal.
pub(in crate::mir::builder) fn complete_raw_root(
    input: super::raw_root_completion_preflight::RawRootCompletionInputV1,
) -> Result<
    RawCompleteInvocationV1,
    super::raw_root_completion_preflight::RawRootCompletionPreflightErrorV1,
> {
    input
        .prepare()
        .map(|prepared| prepared.commit())
        .map_err(|rejected| rejected.into_error())
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
    use crate::mir::builder::raw_root_completion_preflight::RawRootCompletionInputV1;
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
        let complete = complete_raw_root(RawRootCompletionInputV1::new(
            token,
            collector,
            ledger,
            batch,
            main,
            condition,
            RawCallableMainCompatibilityDispositionV1::NotSelected,
        ))
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
        let error = complete_raw_root(RawRootCompletionInputV1::new(
            token,
            InvocationBranded::from_test(brand, collector_payload),
            ledger,
            batch,
            main,
            condition,
            RawCallableMainCompatibilityDispositionV1::NotSelected,
        ))
        .unwrap_err();
        assert!(matches!(
            error,
            super::super::raw_root_completion_preflight::RawRootCompletionPreflightErrorV1::Collector(_)
        ));
    }
}
