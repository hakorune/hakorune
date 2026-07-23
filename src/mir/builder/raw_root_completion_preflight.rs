//! ROOT-RETENTION0-PREFLIGHT: mutation-free Raw root owner validation.
//!
//! This row deliberately stops before collector receipt production and ledger
//! publication.  One input owns every unpublished root component; every
//! failed borrowed check returns that same owner in a rejected product.

use super::module_draft_collector::{ModuleDraftCollectorV1, RootCollectorBatchPrepareErrorV1};
use super::module_invocation_identity::ModuleInvocationTokenV1;
use super::module_invocation_identity::{ModuleInvocationBrandV1, ModuleInvocationFamilyV1};
use super::module_invocation_owner_chain::{BrandedCollectorV1, InvocationBranded};
use super::raw_expansion_receipt_ledger::{
    RawCallableMainCompatibilityDispositionV1, RawExpansionReceiptLedgerErrorV1,
    RawExpansionReceiptLedgerV1, RawExpansionReservationV1,
};
use super::raw_root_completion::RawCompleteInvocationV1;
use super::root_draft_batch::PreparedRootDraftBatchV1;

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum RawRootCompletionPreflightErrorV1 {
    NonRawFamily,
    ForeignBrand { expected: u64, actual: u64 },
    RootBatchPolicy,
    Collector(RootCollectorBatchPrepareErrorV1),
    Ledger(RawExpansionReceiptLedgerErrorV1),
    SelectedCallableMainMissing,
}

impl std::fmt::Display for RawRootCompletionPreflightErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "[freeze:contract][raw_root/preflight] {self:?}")
    }
}

impl std::error::Error for RawRootCompletionPreflightErrorV1 {}

/// The sole unpublished Raw root input owner for the retention preflight.
/// None of these fields is exposed independently, so a failed check cannot
/// silently discard one part of the invocation state.
#[derive(Debug)]
pub(in crate::mir::builder) struct RawRootCompletionInputV1 {
    token: ModuleInvocationTokenV1,
    collector: BrandedCollectorV1<ModuleDraftCollectorV1>,
    ledger: RawExpansionReceiptLedgerV1,
    batch: PreparedRootDraftBatchV1,
    main_reservation: RawExpansionReservationV1,
    condition_reservation: RawExpansionReservationV1,
    callable_main: RawCallableMainCompatibilityDispositionV1,
}

#[derive(Debug)]
pub(in crate::mir::builder) struct PreparedRawRootCompletionV1 {
    input: RawRootCompletionInputV1,
    _seal: PreparedRawRootCompletionSealV1,
}

#[derive(Debug)]
struct PreparedRawRootCompletionSealV1;

#[derive(Debug)]
pub(in crate::mir::builder) struct RejectedRawRootCompletionV1 {
    owner: RawRootCompletionInputV1,
    error: RawRootCompletionPreflightErrorV1,
}

impl RawRootCompletionInputV1 {
    pub(in crate::mir::builder) fn new(
        token: ModuleInvocationTokenV1,
        collector: BrandedCollectorV1<ModuleDraftCollectorV1>,
        ledger: RawExpansionReceiptLedgerV1,
        batch: PreparedRootDraftBatchV1,
        main_reservation: RawExpansionReservationV1,
        condition_reservation: RawExpansionReservationV1,
        callable_main: RawCallableMainCompatibilityDispositionV1,
    ) -> Self {
        Self {
            token,
            collector,
            ledger,
            batch,
            main_reservation,
            condition_reservation,
            callable_main,
        }
    }

    pub(in crate::mir::builder) fn prepare(
        self,
    ) -> Result<PreparedRawRootCompletionV1, RejectedRawRootCompletionV1> {
        if let Err(error) = self.validate() {
            return Err(RejectedRawRootCompletionV1 { owner: self, error });
        }
        Ok(PreparedRawRootCompletionV1 {
            input: self,
            _seal: PreparedRawRootCompletionSealV1,
        })
    }

    fn validate(&self) -> Result<(), RawRootCompletionPreflightErrorV1> {
        if self.token.family() != ModuleInvocationFamilyV1::Raw {
            return Err(RawRootCompletionPreflightErrorV1::NonRawFamily);
        }
        let brand = self.token.brand();
        if self.collector.brand() != brand {
            return Err(RawRootCompletionPreflightErrorV1::ForeignBrand {
                expected: brand.ordinal(),
                actual: self.collector.brand().ordinal(),
            });
        }
        if self.ledger.brand() != brand {
            return Err(RawRootCompletionPreflightErrorV1::ForeignBrand {
                expected: brand.ordinal(),
                actual: self.ledger.brand().ordinal(),
            });
        }
        if self.batch.policy() != super::module_invocation_drain::ConditionFnPolicyV1::Required {
            return Err(RawRootCompletionPreflightErrorV1::RootBatchPolicy);
        }
        if self.callable_main == RawCallableMainCompatibilityDispositionV1::Selected {
            return Err(RawRootCompletionPreflightErrorV1::SelectedCallableMainMissing);
        }
        self.collector
            .payload()
            .validate_root_batch(&self.batch, brand)
            .map_err(RawRootCompletionPreflightErrorV1::Collector)?;
        self.ledger
            .validate_required_root_batch(&self.main_reservation, &self.condition_reservation)
            .map_err(RawRootCompletionPreflightErrorV1::Ledger)
    }

    pub(in crate::mir::builder) const fn brand(&self) -> ModuleInvocationBrandV1 {
        self.token.brand()
    }
}

impl PreparedRawRootCompletionV1 {
    pub(in crate::mir::builder) const fn brand(&self) -> ModuleInvocationBrandV1 {
        self.input.brand()
    }

    /// The only mutation terminal after borrowed preflight. All fallible
    /// branches below are invariant checks; semantic rejection happened in
    /// `RawRootCompletionInputV1::prepare`.
    pub(in crate::mir::builder) fn commit(self) -> RawCompleteInvocationV1 {
        let RawRootCompletionInputV1 {
            token,
            collector,
            ledger,
            batch,
            main_reservation,
            condition_reservation,
            callable_main,
        } = self.input;
        let brand = token.brand();
        let prepared = collector
            .into_payload()
            .prepare_root_batch_preflighted(batch);
        let (collector, branded_receipt) = prepared
            .commit_branded()
            .unwrap_or_else(|_| unreachable!("branded root collector proof drifted"));
        let (admissions, root_body, receipt_brand) = branded_receipt.into_parts();
        assert_eq!(receipt_brand, brand, "root receipt brand proof drifted");
        let mut main = None;
        let mut condition = None;
        for receipt in admissions.into_vec() {
            match receipt.payload().key() {
                super::module_draft_collector::FunctionDraftKeyV1::Main => main = Some(receipt),
                super::module_draft_collector::FunctionDraftKeyV1::SyntheticConditionFn => {
                    condition = Some(receipt)
                }
                _ => unreachable!("raw root collector emitted a non-root receipt"),
            }
        }
        let main = main.unwrap_or_else(|| unreachable!("raw root Main receipt disappeared"));
        let condition =
            condition.unwrap_or_else(|| unreachable!("raw root condition receipt disappeared"));
        let mut ledger = ledger;
        ledger.commit_required_root_batch_preflighted(
            main_reservation,
            &main,
            condition_reservation,
            &condition,
        );
        let ledger = ledger
            .seal()
            .unwrap_or_else(|_| unreachable!("raw root ledger proof drifted before seal"));
        RawCompleteInvocationV1::from_committed_parts(
            brand,
            InvocationBranded::from_source(brand, collector),
            ledger,
            root_body,
            main,
            condition,
            callable_main,
        )
    }
}

impl RejectedRawRootCompletionV1 {
    pub(in crate::mir::builder) fn error(&self) -> &RawRootCompletionPreflightErrorV1 {
        &self.error
    }

    pub(in crate::mir::builder) const fn brand(&self) -> ModuleInvocationBrandV1 {
        self.owner.brand()
    }

    /// Deliberately the only terminal exposed for a rejected preflight owner.
    pub(in crate::mir::builder) fn discard(self) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::main_pending_draft::{
        MainCompletionRequestV1, MainDraftIdentityV1, MainHeaderLoanV1, MainHeaderSourceV1,
    };
    use crate::mir::builder::module_draft_collector::CompletedDraftSignatureViewV1;
    use crate::mir::builder::module_invocation_drain::ConditionFnPolicyV1;
    use crate::mir::builder::module_invocation_identity::TestInvocationPreflightFactoryV1;
    use crate::mir::builder::module_invocation_owner_chain::InvocationBranded;
    use crate::mir::builder::raw_expansion_receipt_ledger::RawExpansionDraftRequestV1;
    use crate::mir::builder::root_body_completion::{
        RootBodyCompletionTrackerV1, RootBodyResultV1,
    };
    use crate::mir::{
        BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirModule, MirType,
    };

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

    fn input(collector_brand: ModuleInvocationBrandV1) -> RawRootCompletionInputV1 {
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
        let batch = super::super::root_draft_batch::PreparedRootDraftBatchV1::prepare(
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
            .reserve(RawExpansionDraftRequestV1::root_main())
            .unwrap();
        let condition_reservation = ledger
            .reserve(RawExpansionDraftRequestV1::required_condition_fn())
            .unwrap();
        RawRootCompletionInputV1::new(
            token,
            InvocationBranded::from_test(
                collector_brand,
                ModuleDraftCollectorV1::with_brand(collector_brand),
            ),
            ledger,
            batch,
            main_reservation,
            condition_reservation,
            RawCallableMainCompatibilityDispositionV1::NotSelected,
        )
    }

    #[test]
    fn valid_input_reaches_prepared_owner_without_mutation() {
        let brand = ModuleInvocationBrandV1::test_with_ordinal(1);
        let prepared = input(brand).prepare().unwrap();
        assert_eq!(prepared.brand(), brand);
    }

    #[test]
    fn prepared_commit_publishes_one_root_pair() {
        let brand = ModuleInvocationBrandV1::test_with_ordinal(1);
        let complete = input(brand).prepare().unwrap().commit();
        assert_eq!(complete.brand(), brand);
        assert_eq!(complete.ledger().final_count(), 2);
        assert_eq!(complete.collector().payload().symbol_count(), 2);
        assert_eq!(complete.root().root_body().brand(), brand);
    }

    #[test]
    fn foreign_collector_returns_the_full_input_owner() {
        let expected = ModuleInvocationBrandV1::test_with_ordinal(1);
        let foreign = ModuleInvocationBrandV1::test_with_ordinal(2);
        let rejected = input(foreign).prepare().unwrap_err();
        assert!(matches!(
            rejected.error(),
            RawRootCompletionPreflightErrorV1::ForeignBrand { .. }
        ));
        assert_eq!(rejected.brand(), expected);
        assert_eq!(rejected.owner.collector.payload().symbol_count(), 0);
        assert_eq!(rejected.owner.batch.admissions().len(), 2);
    }

    #[test]
    fn foreign_reservation_returns_the_unmodified_ledger_owner() {
        let mut owner = input(ModuleInvocationBrandV1::test_with_ordinal(1));
        let mut factory = TestInvocationPreflightFactoryV1::new();
        let _token = factory.mint(ModuleInvocationFamilyV1::Raw).unwrap();
        let foreign_token = factory.mint(ModuleInvocationFamilyV1::Raw).unwrap();
        let mut foreign_ledger = RawExpansionReceiptLedgerV1::new_for_token(
            &foreign_token,
            RawCallableMainCompatibilityDispositionV1::NotSelected,
        );
        owner.main_reservation = foreign_ledger
            .reserve(RawExpansionDraftRequestV1::root_main())
            .unwrap();
        let rejected = owner.prepare().unwrap_err();
        assert!(matches!(
            rejected.error(),
            RawRootCompletionPreflightErrorV1::Ledger(
                RawExpansionReceiptLedgerErrorV1::ForeignReservation
            )
        ));
        assert_eq!(rejected.owner.ledger.completed_event_count(), 0);
    }

    #[test]
    fn duplicate_collector_admission_returns_the_unmodified_indexes() {
        let brand = ModuleInvocationBrandV1::test_with_ordinal(1);
        let mut owner = input(brand);
        owner
            .collector
            .payload_mut()
            .prepare_admission(
                super::super::module_draft_collector::FunctionDraftKeyV1::SyntheticConditionFn,
                "condition_fn".into(),
                1,
                super::super::module_draft_collector::DraftPublicationPolicyV1::CanonicalRejectDuplicate,
            )
            .unwrap()
            .seal(draft("condition_fn", 1))
            .unwrap()
            .collect();
        let rejected = owner.prepare().unwrap_err();
        assert!(matches!(
            rejected.error(),
            RawRootCompletionPreflightErrorV1::Collector(
                RootCollectorBatchPrepareErrorV1::Admission { .. }
            )
        ));
        assert_eq!(rejected.owner.collector.payload().symbol_count(), 1);
    }
}
