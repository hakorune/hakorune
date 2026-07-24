//! WIRING-I0-ROUTEINV-P0b-RAWLEDGER-S0/P0 raw completion ledger.
//!
//! Recursive raw lowering discovers function work incrementally. This
//! disconnected owner reserves each discovered unit and consumes exactly one
//! successful collector receipt. It never scans AST or collector inventory and
//! has no Builder, module, draft, header, retry, or publication capability.

use std::collections::{BTreeMap, BTreeSet};

#[cfg(test)]
use std::sync::{Mutex, OnceLock};

use super::module_draft_collector::{
    CollectedDraftAdmissionReceiptV1, CollectedDraftReplacementDispositionV1,
    DraftPublicationPolicyV1, FunctionDraftKeyV1,
};
use super::module_invocation_identity::{ModuleInvocationBrandV1, ModuleInvocationTokenV1};
use super::module_invocation_owner_chain::InvocationBranded;
use super::root_batch_slot::RawRootBatchSlotV1;

mod preflight;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum RawConditionDispositionV1 {
    RequiredCompatibility,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum RawCallableMainCompatibilityDispositionV1 {
    NotSelected,
    Selected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum RawExpansionCutoverStopV1 {
    DuplicateMainSourcePolicySelectionRequired,
    CallableMainFailurePropagationPolicySelectionRequired,
}

const RAW_EXPANSION_CUTOVER_STOPS: [RawExpansionCutoverStopV1; 2] = [
    RawExpansionCutoverStopV1::DuplicateMainSourcePolicySelectionRequired,
    RawExpansionCutoverStopV1::CallableMainFailurePropagationPolicySelectionRequired,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum RawExpansionDraftRoleV1 {
    RootMain,
    SyntheticConditionFn,
    TopLevelFunction,
    StaticMethod,
    InstanceMethod,
    Constructor,
    CallableMainCompatibility,
    NestedStaticMethod,
    NestedInstanceMethod,
    NestedConstructor,
}

impl RawExpansionDraftRoleV1 {
    const fn is_legacy_discovered(self) -> bool {
        !matches!(self, Self::RootMain | Self::SyntheticConditionFn)
    }
}

#[derive(Debug)]
pub(in crate::mir::builder) struct RawExpansionDraftRequestV1 {
    role: RawExpansionDraftRoleV1,
    key: FunctionDraftKeyV1,
    symbol: Box<str>,
    arity: usize,
    policy: DraftPublicationPolicyV1,
    _seal: RawExpansionDraftRequestSealV1,
}

#[derive(Debug)]
struct RawExpansionDraftRequestSealV1;

impl RawExpansionDraftRequestV1 {
    pub(in crate::mir::builder) fn root_main() -> Self {
        let contract = RawRootBatchSlotV1::Main.contract();
        Self {
            role: RawExpansionDraftRoleV1::RootMain,
            key: contract.key().clone(),
            symbol: contract.symbol().into(),
            arity: contract.arity(),
            policy: contract.policy(),
            _seal: RawExpansionDraftRequestSealV1,
        }
    }

    pub(in crate::mir::builder) fn required_condition_fn() -> Self {
        let contract = RawRootBatchSlotV1::RequiredCondition.contract();
        Self {
            role: RawExpansionDraftRoleV1::SyntheticConditionFn,
            key: contract.key().clone(),
            symbol: contract.symbol().into(),
            arity: contract.arity(),
            policy: contract.policy(),
            _seal: RawExpansionDraftRequestSealV1,
        }
    }

    pub(in crate::mir::builder) fn legacy_discovered(
        role: RawExpansionDraftRoleV1,
        symbol: impl Into<Box<str>>,
        arity: usize,
    ) -> Result<Self, RawExpansionReceiptLedgerErrorV1> {
        if !role.is_legacy_discovered() {
            return Err(RawExpansionReceiptLedgerErrorV1::InvalidLegacyRole(role));
        }
        let symbol = symbol.into();
        if symbol.is_empty() {
            return Err(RawExpansionReceiptLedgerErrorV1::EmptySymbol);
        }
        Ok(Self {
            role,
            key: FunctionDraftKeyV1::LegacySymbol(symbol.to_string()),
            symbol,
            arity,
            policy: DraftPublicationPolicyV1::LegacyReplaceWholePair,
            _seal: RawExpansionDraftRequestSealV1,
        })
    }

    pub(in crate::mir::builder) fn callable_main_compatibility(
        symbol: impl Into<Box<str>>,
        arity: usize,
    ) -> Result<Self, RawExpansionReceiptLedgerErrorV1> {
        let symbol = symbol.into();
        if symbol.is_empty() {
            return Err(RawExpansionReceiptLedgerErrorV1::EmptySymbol);
        }
        Ok(Self {
            role: RawExpansionDraftRoleV1::CallableMainCompatibility,
            key: FunctionDraftKeyV1::LegacySymbol(symbol.to_string()),
            symbol,
            arity,
            policy: DraftPublicationPolicyV1::LegacyReplaceWholePair,
            _seal: RawExpansionDraftRequestSealV1,
        })
    }
}

#[derive(Debug)]
pub(in crate::mir::builder) struct RawExpansionReservationV1 {
    brand: ModuleInvocationBrandV1,
    ordinal: u32,
    request: RawExpansionDraftRequestV1,
    _seal: RawExpansionReservationSealV1,
}

impl RawExpansionReservationV1 {
    pub(in crate::mir::builder) const fn brand(&self) -> ModuleInvocationBrandV1 {
        self.brand
    }

    pub(in crate::mir::builder) fn key(&self) -> &FunctionDraftKeyV1 {
        &self.request.key
    }

    pub(in crate::mir::builder) fn symbol(&self) -> &str {
        &self.request.symbol
    }

    pub(in crate::mir::builder) const fn arity(&self) -> usize {
        self.request.arity
    }

    pub(in crate::mir::builder) const fn policy(&self) -> DraftPublicationPolicyV1 {
        self.request.policy
    }
}

#[derive(Debug)]
struct RawExpansionReservationSealV1;

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum RawExpansionReplacementEventV1 {
    Inserted,
    ReplacedWholePair {
        previous_key: FunctionDraftKeyV1,
        previous_symbol: Box<str>,
    },
}

/// The only Main replacement disposition admitted by the Raw root batch.
/// This is a borrowed-preflight fact; it carries no mutation capability.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum RawRootMainCommitDispositionV1 {
    Insert,
    ReplaceExact {
        previous_key: FunctionDraftKeyV1,
        previous_symbol: Box<str>,
    },
}

/// Mutation-free plan for the required Main/condition pair.  The two
/// ordinals are allocated only when the private commit terminal consumes this
/// product, so a rejected prepare leaves the ledger byte-for-byte unchanged.
#[derive(Debug)]
pub(in crate::mir::builder) struct PreparedRawRootLedgerPairV1 {
    brand: ModuleInvocationBrandV1,
    main_ordinal: u32,
    condition_ordinal: u32,
    main_request: RawExpansionDraftRequestV1,
    condition_request: RawExpansionDraftRequestV1,
    main_disposition: RawRootMainCommitDispositionV1,
    _seal: PreparedRawRootLedgerPairSealV1,
}

#[derive(Debug)]
struct PreparedRawRootLedgerPairSealV1;

impl PreparedRawRootLedgerPairV1 {
    pub(in crate::mir::builder) fn main_disposition(&self) -> &RawRootMainCommitDispositionV1 {
        &self.main_disposition
    }

    /// Materialize both open reservations in one private consuming step.
    /// `prepare_required_root_pair` proves the ordinal capacity and all
    /// history invariants before this method is reachable.
    pub(in crate::mir::builder) fn commit_reservations(
        self,
        mut ledger: RawExpansionReceiptLedgerV1,
    ) -> (
        RawExpansionReceiptLedgerV1,
        RawExpansionReservationV1,
        RawExpansionReservationV1,
    ) {
        debug_assert_eq!(ledger.brand, self.brand);
        debug_assert_eq!(ledger.next_ordinal, self.main_ordinal);
        debug_assert_eq!(self.condition_ordinal, self.main_ordinal + 1);
        debug_assert!(ledger.open.is_empty());

        ledger.next_ordinal = self.condition_ordinal + 1;
        ledger.open.insert(self.main_ordinal);
        ledger.open.insert(self.condition_ordinal);
        let main = RawExpansionReservationV1 {
            brand: self.brand,
            ordinal: self.main_ordinal,
            request: self.main_request,
            _seal: RawExpansionReservationSealV1,
        };
        let condition = RawExpansionReservationV1 {
            brand: self.brand,
            ordinal: self.condition_ordinal,
            request: self.condition_request,
            _seal: RawExpansionReservationSealV1,
        };
        (ledger, main, condition)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) struct RawExpansionCompletedEventV1 {
    ordinal: u32,
    role: RawExpansionDraftRoleV1,
    key: FunctionDraftKeyV1,
    symbol: Box<str>,
    arity: usize,
    policy: DraftPublicationPolicyV1,
    replacement: RawExpansionReplacementEventV1,
}

impl RawExpansionCompletedEventV1 {
    pub(in crate::mir::builder) const fn role(&self) -> RawExpansionDraftRoleV1 {
        self.role
    }

    pub(in crate::mir::builder) const fn ordinal(&self) -> u32 {
        self.ordinal
    }

    pub(in crate::mir::builder) fn symbol(&self) -> &str {
        &self.symbol
    }

    pub(in crate::mir::builder) fn key(&self) -> &FunctionDraftKeyV1 {
        &self.key
    }

    pub(in crate::mir::builder) const fn arity(&self) -> usize {
        self.arity
    }

    pub(in crate::mir::builder) const fn policy(&self) -> DraftPublicationPolicyV1 {
        self.policy
    }

    pub(in crate::mir::builder) const fn replacement(&self) -> &RawExpansionReplacementEventV1 {
        &self.replacement
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum RawExpansionAbortReasonV1 {
    Primary,
    Cleanup,
    Admission,
    Panic,
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum RawExpansionReceiptLedgerErrorV1 {
    InvalidLegacyRole(RawExpansionDraftRoleV1),
    EmptySymbol,
    ReservationOrdinalOverflow,
    ForeignReservation,
    UnknownReservation,
    LedgerPoisoned,
    ReceiptKeyMismatch,
    ReceiptSymbolMismatch,
    ReceiptArityMismatch,
    ReceiptPolicyMismatch,
    InsertedReceiptCollision,
    ReplacementPolicyMismatch,
    ReplacementHistoryMismatch,
    OpenReservations { count: usize },
    MissingRootMain,
    MissingConditionFn,
    MissingCallableMainCompatibility,
    UnexpectedCallableMainCompatibility,
}

impl std::fmt::Display for RawExpansionReceiptLedgerErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "[freeze:contract][raw_expansion_ledger] {self:?}"
        )
    }
}

impl std::error::Error for RawExpansionReceiptLedgerErrorV1 {}

#[derive(Debug)]
pub(in crate::mir::builder) struct RawExpansionReceiptLedgerV1 {
    brand: ModuleInvocationBrandV1,
    next_ordinal: u32,
    open: BTreeSet<u32>,
    events: Vec<RawExpansionCompletedEventV1>,
    final_event_by_key: BTreeMap<FunctionDraftKeyV1, usize>,
    key_by_symbol: BTreeMap<Box<str>, FunctionDraftKeyV1>,
    condition: RawConditionDispositionV1,
    callable_main: RawCallableMainCompatibilityDispositionV1,
    poisoned: bool,
    _seal: RawExpansionReceiptLedgerSealV1,
}

#[derive(Debug)]
struct RawExpansionReceiptLedgerSealV1;

#[derive(Debug)]
pub(in crate::mir) struct SealedRawExpansionReceiptLedgerV1 {
    brand: ModuleInvocationBrandV1,
    events: Box<[RawExpansionCompletedEventV1]>,
    final_event_by_key: BTreeMap<FunctionDraftKeyV1, usize>,
    key_by_symbol: BTreeMap<Box<str>, FunctionDraftKeyV1>,
    condition: RawConditionDispositionV1,
    callable_main: RawCallableMainCompatibilityDispositionV1,
    _seal: SealedRawExpansionReceiptLedgerSealV1,
}

#[derive(Debug)]
struct SealedRawExpansionReceiptLedgerSealV1;

#[derive(Debug)]
pub(in crate::mir::builder) struct AbortedRawExpansionReceiptLedgerV1 {
    brand: ModuleInvocationBrandV1,
    events: Box<[RawExpansionCompletedEventV1]>,
    final_event_by_key: BTreeMap<FunctionDraftKeyV1, usize>,
    failed_ordinal: u32,
    failed_role: RawExpansionDraftRoleV1,
    reason: RawExpansionAbortReasonV1,
    outstanding_reservations: usize,
    _seal: AbortedRawExpansionReceiptLedgerSealV1,
}

#[derive(Debug)]
struct AbortedRawExpansionReceiptLedgerSealV1;

impl RawExpansionReceiptLedgerV1 {
    pub(in crate::mir::builder) fn is_clean_open(&self) -> bool {
        !self.poisoned && self.open.is_empty()
    }

    /// Borrow-only preparation for the required Raw root pair.  In
    /// particular, this never calls `reserve` and never changes an ordinal,
    /// open-set entry, event, or index.
    pub(in crate::mir::builder) fn prepare_required_root_pair(
        &self,
    ) -> Result<PreparedRawRootLedgerPairV1, RawExpansionReceiptLedgerErrorV1> {
        if self.poisoned {
            return Err(RawExpansionReceiptLedgerErrorV1::LedgerPoisoned);
        }
        if !self.open.is_empty() {
            return Err(RawExpansionReceiptLedgerErrorV1::OpenReservations {
                count: self.open.len(),
            });
        }
        let condition_key = FunctionDraftKeyV1::SyntheticConditionFn;
        if self.final_event_by_key.contains_key(&condition_key)
            || self.key_by_symbol.contains_key("condition_fn")
        {
            return Err(RawExpansionReceiptLedgerErrorV1::InsertedReceiptCollision);
        }
        let main_key = FunctionDraftKeyV1::Main;
        let main_by_key = self.final_event_by_key.get(&main_key);
        let main_by_symbol = self.key_by_symbol.get("main");
        if main_by_key.is_some() != main_by_symbol.is_some()
            || main_by_symbol.is_some_and(|key| key != &main_key)
        {
            return Err(RawExpansionReceiptLedgerErrorV1::ReplacementHistoryMismatch);
        }
        let main_disposition = match main_by_key {
            None => RawRootMainCommitDispositionV1::Insert,
            Some(index) => {
                let event = &self.events[*index];
                RawRootMainCommitDispositionV1::ReplaceExact {
                    previous_key: event.key.clone(),
                    previous_symbol: event.symbol.clone(),
                }
            }
        };
        let condition_ordinal = self
            .next_ordinal
            .checked_add(1)
            .ok_or(RawExpansionReceiptLedgerErrorV1::ReservationOrdinalOverflow)?;
        let _after_pair = condition_ordinal
            .checked_add(1)
            .ok_or(RawExpansionReceiptLedgerErrorV1::ReservationOrdinalOverflow)?;
        Ok(PreparedRawRootLedgerPairV1 {
            brand: self.brand,
            main_ordinal: self.next_ordinal,
            condition_ordinal,
            main_request: RawExpansionDraftRequestV1::root_main(),
            condition_request: RawExpansionDraftRequestV1::required_condition_fn(),
            main_disposition,
            _seal: PreparedRawRootLedgerPairSealV1,
        })
    }
    pub(in crate::mir::builder) fn new_for_token(
        token: &ModuleInvocationTokenV1,
        callable_main: RawCallableMainCompatibilityDispositionV1,
    ) -> Self {
        Self::new_with_brand(token.brand(), callable_main)
    }

    fn new_with_brand(
        brand: ModuleInvocationBrandV1,
        callable_main: RawCallableMainCompatibilityDispositionV1,
    ) -> Self {
        Self {
            brand,
            next_ordinal: 0,
            open: BTreeSet::new(),
            events: Vec::new(),
            final_event_by_key: BTreeMap::new(),
            key_by_symbol: BTreeMap::new(),
            condition: RawConditionDispositionV1::RequiredCompatibility,
            callable_main,
            poisoned: false,
            _seal: RawExpansionReceiptLedgerSealV1,
        }
    }

    #[cfg(test)]
    pub(in crate::mir::builder) fn new(
        callable_main: RawCallableMainCompatibilityDispositionV1,
    ) -> Self {
        static NEXT_TEST_BRAND: OnceLock<Mutex<u64>> = OnceLock::new();
        let counter = NEXT_TEST_BRAND.get_or_init(|| Mutex::new(0));
        let mut ordinal = counter.lock().expect("test brand counter poisoned");
        *ordinal += 1;
        Self::new_with_brand(
            ModuleInvocationBrandV1::test_with_ordinal(*ordinal),
            callable_main,
        )
    }

    pub(in crate::mir::builder) fn reserve(
        &mut self,
        request: RawExpansionDraftRequestV1,
    ) -> Result<RawExpansionReservationV1, RawExpansionReceiptLedgerErrorV1> {
        if self.poisoned {
            return Err(RawExpansionReceiptLedgerErrorV1::LedgerPoisoned);
        }
        let ordinal = self.next_ordinal;
        self.next_ordinal = self
            .next_ordinal
            .checked_add(1)
            .ok_or(RawExpansionReceiptLedgerErrorV1::ReservationOrdinalOverflow)?;
        self.open.insert(ordinal);
        Ok(RawExpansionReservationV1 {
            brand: self.brand,
            ordinal,
            request,
            _seal: RawExpansionReservationSealV1,
        })
    }

    pub(in crate::mir) const fn brand(&self) -> ModuleInvocationBrandV1 {
        self.brand
    }

    pub(in crate::mir) const fn callable_main(&self) -> RawCallableMainCompatibilityDispositionV1 {
        self.callable_main
    }

    pub(in crate::mir::builder) fn complete(
        &mut self,
        reservation: RawExpansionReservationV1,
        receipt: CollectedDraftAdmissionReceiptV1,
    ) -> Result<(), RawExpansionReceiptLedgerErrorV1> {
        self.complete_inner(reservation, &receipt)
    }

    fn complete_inner(
        &mut self,
        reservation: RawExpansionReservationV1,
        receipt: &CollectedDraftAdmissionReceiptV1,
    ) -> Result<(), RawExpansionReceiptLedgerErrorV1> {
        if self.poisoned {
            return Err(RawExpansionReceiptLedgerErrorV1::LedgerPoisoned);
        }
        if reservation.brand != self.brand {
            return Err(RawExpansionReceiptLedgerErrorV1::ForeignReservation);
        }
        if !self.open.contains(&reservation.ordinal) {
            return Err(RawExpansionReceiptLedgerErrorV1::UnknownReservation);
        }
        if receipt.key() != &reservation.request.key {
            return self.poison(RawExpansionReceiptLedgerErrorV1::ReceiptKeyMismatch);
        }
        if receipt.symbol() != reservation.request.symbol.as_ref() {
            return self.poison(RawExpansionReceiptLedgerErrorV1::ReceiptSymbolMismatch);
        }
        if receipt.arity() != reservation.request.arity {
            return self.poison(RawExpansionReceiptLedgerErrorV1::ReceiptArityMismatch);
        }
        if receipt.policy() != reservation.request.policy {
            return self.poison(RawExpansionReceiptLedgerErrorV1::ReceiptPolicyMismatch);
        }

        let replacement = match receipt.replacement() {
            CollectedDraftReplacementDispositionV1::Inserted => {
                if self.final_event_by_key.contains_key(receipt.key())
                    || self.key_by_symbol.contains_key(receipt.symbol())
                {
                    return self.poison(RawExpansionReceiptLedgerErrorV1::InsertedReceiptCollision);
                }
                RawExpansionReplacementEventV1::Inserted
            }
            CollectedDraftReplacementDispositionV1::ReplacedWholePair {
                previous_key,
                previous_symbol,
            } => {
                if receipt.policy() != DraftPublicationPolicyV1::LegacyReplaceWholePair {
                    return self
                        .poison(RawExpansionReceiptLedgerErrorV1::ReplacementPolicyMismatch);
                }
                if previous_key != receipt.key()
                    || previous_symbol.as_ref() != receipt.symbol()
                    || !self.final_event_by_key.contains_key(previous_key)
                    || self.key_by_symbol.get(previous_symbol.as_ref()) != Some(previous_key)
                {
                    return self
                        .poison(RawExpansionReceiptLedgerErrorV1::ReplacementHistoryMismatch);
                }
                self.final_event_by_key.remove(previous_key);
                self.key_by_symbol.remove(previous_symbol.as_ref());
                RawExpansionReplacementEventV1::ReplacedWholePair {
                    previous_key: previous_key.clone(),
                    previous_symbol: previous_symbol.clone(),
                }
            }
        };

        self.open.remove(&reservation.ordinal);
        let event_index = self.events.len();
        self.final_event_by_key
            .insert(reservation.request.key.clone(), event_index);
        self.key_by_symbol.insert(
            reservation.request.symbol.clone(),
            reservation.request.key.clone(),
        );
        self.events.push(RawExpansionCompletedEventV1 {
            ordinal: reservation.ordinal,
            role: reservation.request.role,
            key: reservation.request.key,
            symbol: reservation.request.symbol,
            arity: reservation.request.arity,
            policy: reservation.request.policy,
            replacement,
        });
        Ok(())
    }

    pub(in crate::mir::builder) fn complete_branded(
        &mut self,
        reservation: RawExpansionReservationV1,
        receipt: &InvocationBranded<CollectedDraftAdmissionReceiptV1>,
    ) -> Result<(), RawExpansionReceiptLedgerErrorV1> {
        if receipt.brand() != self.brand {
            return Err(RawExpansionReceiptLedgerErrorV1::ForeignReservation);
        }
        if receipt.payload().collector_brand() != Some(self.brand) {
            return Err(RawExpansionReceiptLedgerErrorV1::ForeignReservation);
        }
        self.complete_inner(reservation, receipt.payload())
    }

    /// Atomically record the required raw root pair.  All identity, open
    /// reservation, and replacement checks happen before either event is
    /// appended, so a late condition failure cannot leave a Main-only history.
    pub(in crate::mir::builder) fn complete_required_root_batch(
        &mut self,
        main_reservation: RawExpansionReservationV1,
        main: &InvocationBranded<CollectedDraftAdmissionReceiptV1>,
        condition_reservation: RawExpansionReservationV1,
        condition: &InvocationBranded<CollectedDraftAdmissionReceiptV1>,
    ) -> Result<(), RawExpansionReceiptLedgerErrorV1> {
        self.validate_branded_root_slot(&main_reservation, main, true)?;
        self.validate_branded_root_slot(&condition_reservation, condition, false)?;
        self.complete_inner(main_reservation, main.payload())?;
        self.complete_inner(condition_reservation, condition.payload())
    }

    fn validate_branded_root_slot(
        &self,
        reservation: &RawExpansionReservationV1,
        receipt: &InvocationBranded<CollectedDraftAdmissionReceiptV1>,
        main: bool,
    ) -> Result<(), RawExpansionReceiptLedgerErrorV1> {
        if receipt.brand() != self.brand || receipt.payload().collector_brand() != Some(self.brand)
        {
            return Err(RawExpansionReceiptLedgerErrorV1::ForeignReservation);
        }
        if reservation.brand != self.brand || !self.open.contains(&reservation.ordinal) {
            return Err(RawExpansionReceiptLedgerErrorV1::ForeignReservation);
        }
        let expected_key = if main {
            FunctionDraftKeyV1::Main
        } else {
            FunctionDraftKeyV1::SyntheticConditionFn
        };
        let expected_symbol = if main { "main" } else { "condition_fn" };
        let expected_arity = if main { 0 } else { 1 };
        let expected_policy = if main {
            DraftPublicationPolicyV1::LegacyReplaceWholePair
        } else {
            DraftPublicationPolicyV1::CanonicalRejectDuplicate
        };
        if reservation.request.key != expected_key
            || reservation.request.symbol.as_ref() != expected_symbol
            || reservation.request.arity != expected_arity
            || reservation.request.policy != expected_policy
            || receipt.payload().key() != &expected_key
            || receipt.payload().symbol() != expected_symbol
            || receipt.payload().arity() != expected_arity
            || receipt.payload().policy() != expected_policy
        {
            return Err(RawExpansionReceiptLedgerErrorV1::ReceiptKeyMismatch);
        }
        if !main
            && (self.final_event_by_key.contains_key(&expected_key)
                || self.key_by_symbol.contains_key(expected_symbol))
        {
            return Err(RawExpansionReceiptLedgerErrorV1::InsertedReceiptCollision);
        }
        if main {
            match receipt.payload().replacement() {
                CollectedDraftReplacementDispositionV1::Inserted => {
                    if self.final_event_by_key.contains_key(&expected_key)
                        || self.key_by_symbol.contains_key(expected_symbol)
                    {
                        return Err(RawExpansionReceiptLedgerErrorV1::InsertedReceiptCollision);
                    }
                }
                CollectedDraftReplacementDispositionV1::ReplacedWholePair {
                    previous_key,
                    previous_symbol,
                } => {
                    if previous_key != &expected_key
                        || previous_symbol.as_ref() != expected_symbol
                        || !self.final_event_by_key.contains_key(previous_key)
                        || self.key_by_symbol.get(previous_symbol.as_ref()) != Some(previous_key)
                    {
                        return Err(RawExpansionReceiptLedgerErrorV1::ReplacementHistoryMismatch);
                    }
                }
            }
        }
        Ok(())
    }

    pub(in crate::mir::builder) fn abort(
        mut self,
        reservation: RawExpansionReservationV1,
        reason: RawExpansionAbortReasonV1,
    ) -> Result<AbortedRawExpansionReceiptLedgerV1, RawExpansionReceiptLedgerErrorV1> {
        if self.poisoned {
            return Err(RawExpansionReceiptLedgerErrorV1::LedgerPoisoned);
        }
        if reservation.brand != self.brand {
            return Err(RawExpansionReceiptLedgerErrorV1::ForeignReservation);
        }
        if !self.open.remove(&reservation.ordinal) {
            return Err(RawExpansionReceiptLedgerErrorV1::UnknownReservation);
        }
        Ok(AbortedRawExpansionReceiptLedgerV1 {
            brand: self.brand,
            events: self.events.into_boxed_slice(),
            final_event_by_key: self.final_event_by_key,
            failed_ordinal: reservation.ordinal,
            failed_role: reservation.request.role,
            reason,
            outstanding_reservations: self.open.len(),
            _seal: AbortedRawExpansionReceiptLedgerSealV1,
        })
    }

    pub(in crate::mir::builder) fn seal(
        self,
    ) -> Result<SealedRawExpansionReceiptLedgerV1, RawExpansionReceiptLedgerErrorV1> {
        if self.poisoned {
            return Err(RawExpansionReceiptLedgerErrorV1::LedgerPoisoned);
        }
        if !self.open.is_empty() {
            return Err(RawExpansionReceiptLedgerErrorV1::OpenReservations {
                count: self.open.len(),
            });
        }
        if !self
            .final_event_by_key
            .contains_key(&FunctionDraftKeyV1::Main)
        {
            return Err(RawExpansionReceiptLedgerErrorV1::MissingRootMain);
        }
        if !self
            .final_event_by_key
            .contains_key(&FunctionDraftKeyV1::SyntheticConditionFn)
        {
            return Err(RawExpansionReceiptLedgerErrorV1::MissingConditionFn);
        }
        let callable_main_present =
            self.contains_final_role(RawExpansionDraftRoleV1::CallableMainCompatibility);
        match (self.callable_main, callable_main_present) {
            (RawCallableMainCompatibilityDispositionV1::Selected, false) => {
                return Err(RawExpansionReceiptLedgerErrorV1::MissingCallableMainCompatibility);
            }
            (RawCallableMainCompatibilityDispositionV1::NotSelected, true) => {
                return Err(RawExpansionReceiptLedgerErrorV1::UnexpectedCallableMainCompatibility);
            }
            _ => {}
        }
        Ok(SealedRawExpansionReceiptLedgerV1 {
            brand: self.brand,
            events: self.events.into_boxed_slice(),
            final_event_by_key: self.final_event_by_key,
            key_by_symbol: self.key_by_symbol,
            condition: self.condition,
            callable_main: self.callable_main,
            _seal: SealedRawExpansionReceiptLedgerSealV1,
        })
    }

    fn contains_final_role(&self, role: RawExpansionDraftRoleV1) -> bool {
        self.final_event_by_key
            .values()
            .any(|index| self.events[*index].role == role)
    }

    fn poison(
        &mut self,
        error: RawExpansionReceiptLedgerErrorV1,
    ) -> Result<(), RawExpansionReceiptLedgerErrorV1> {
        self.poisoned = true;
        Err(error)
    }

    #[cfg(test)]
    pub(in crate::mir::builder) fn completed_event_count(&self) -> usize {
        self.events.len()
    }

    #[cfg(test)]
    pub(in crate::mir::builder) fn last_completed_event(
        &self,
    ) -> Option<&RawExpansionCompletedEventV1> {
        self.events.last()
    }
}

impl SealedRawExpansionReceiptLedgerV1 {
    pub(in crate::mir) const fn brand(&self) -> ModuleInvocationBrandV1 {
        self.brand
    }

    pub(in crate::mir::builder) fn events(&self) -> &[RawExpansionCompletedEventV1] {
        &self.events
    }

    pub(in crate::mir::builder) fn final_count(&self) -> usize {
        self.final_event_by_key.len()
    }

    pub(in crate::mir::builder) fn contains_symbol(&self, symbol: &str) -> bool {
        self.key_by_symbol.contains_key(symbol)
    }

    pub(in crate::mir::builder) fn final_event_for_symbol(
        &self,
        symbol: &str,
    ) -> Option<&RawExpansionCompletedEventV1> {
        let key = self.key_by_symbol.get(symbol)?;
        let event_index = self.final_event_by_key.get(key)?;
        self.events.get(*event_index)
    }

    pub(in crate::mir::builder) const fn condition(&self) -> RawConditionDispositionV1 {
        self.condition
    }

    pub(in crate::mir::builder) const fn callable_main(
        &self,
    ) -> RawCallableMainCompatibilityDispositionV1 {
        self.callable_main
    }

    pub(in crate::mir::builder) const fn cutover_stops(&self) -> &[RawExpansionCutoverStopV1; 2] {
        &RAW_EXPANSION_CUTOVER_STOPS
    }
}

impl AbortedRawExpansionReceiptLedgerV1 {
    pub(in crate::mir::builder) const fn brand(&self) -> ModuleInvocationBrandV1 {
        self.brand
    }

    pub(in crate::mir::builder) fn events(&self) -> &[RawExpansionCompletedEventV1] {
        &self.events
    }

    pub(in crate::mir::builder) fn final_count(&self) -> usize {
        self.final_event_by_key.len()
    }

    pub(in crate::mir::builder) const fn failed_ordinal(&self) -> u32 {
        self.failed_ordinal
    }

    pub(in crate::mir::builder) const fn failed_role(&self) -> RawExpansionDraftRoleV1 {
        self.failed_role
    }

    pub(in crate::mir::builder) const fn reason(&self) -> RawExpansionAbortReasonV1 {
        self.reason
    }

    pub(in crate::mir::builder) const fn outstanding_reservations(&self) -> usize {
        self.outstanding_reservations
    }
}
