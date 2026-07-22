//! WIRING-I0-ROUTEINV-P0b-RAWLEDGER-S0/P0 raw completion ledger.
//!
//! Recursive raw lowering discovers function work incrementally. This
//! disconnected owner reserves each discovered unit and consumes exactly one
//! successful collector receipt. It never scans AST or collector inventory and
//! has no Builder, module, draft, header, retry, or publication capability.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::atomic::{AtomicU64, Ordering};

use super::module_draft_collector::{
    CollectedDraftAdmissionReceiptV1, CollectedDraftReplacementDispositionV1,
    DraftPublicationPolicyV1, FunctionDraftKeyV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum RawConditionDispositionV1 {
    RequiredCompatibility,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum RawCallableMainCompatibilityDispositionV1 {
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
        Self {
            role: RawExpansionDraftRoleV1::RootMain,
            key: FunctionDraftKeyV1::Main,
            symbol: "main".into(),
            arity: 0,
            policy: DraftPublicationPolicyV1::LegacyReplaceWholePair,
            _seal: RawExpansionDraftRequestSealV1,
        }
    }

    pub(in crate::mir::builder) fn required_condition_fn() -> Self {
        Self {
            role: RawExpansionDraftRoleV1::SyntheticConditionFn,
            key: FunctionDraftKeyV1::SyntheticConditionFn,
            symbol: "condition_fn".into(),
            arity: 1,
            policy: DraftPublicationPolicyV1::CanonicalRejectDuplicate,
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
    owner: u64,
    ordinal: u32,
    request: RawExpansionDraftRequestV1,
    _seal: RawExpansionReservationSealV1,
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
    owner: u64,
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
pub(in crate::mir::builder) struct SealedRawExpansionReceiptLedgerV1 {
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

static NEXT_RAW_EXPANSION_LEDGER_OWNER: AtomicU64 = AtomicU64::new(1);

impl RawExpansionReceiptLedgerV1 {
    pub(in crate::mir::builder) fn new(
        callable_main: RawCallableMainCompatibilityDispositionV1,
    ) -> Self {
        Self {
            owner: NEXT_RAW_EXPANSION_LEDGER_OWNER.fetch_add(1, Ordering::Relaxed),
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
            owner: self.owner,
            ordinal,
            request,
            _seal: RawExpansionReservationSealV1,
        })
    }

    pub(in crate::mir::builder) fn complete(
        &mut self,
        reservation: RawExpansionReservationV1,
        receipt: CollectedDraftAdmissionReceiptV1,
    ) -> Result<(), RawExpansionReceiptLedgerErrorV1> {
        if self.poisoned {
            return Err(RawExpansionReceiptLedgerErrorV1::LedgerPoisoned);
        }
        if reservation.owner != self.owner {
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

    pub(in crate::mir::builder) fn abort(
        mut self,
        reservation: RawExpansionReservationV1,
        reason: RawExpansionAbortReasonV1,
    ) -> Result<AbortedRawExpansionReceiptLedgerV1, RawExpansionReceiptLedgerErrorV1> {
        if self.poisoned {
            return Err(RawExpansionReceiptLedgerErrorV1::LedgerPoisoned);
        }
        if reservation.owner != self.owner {
            return Err(RawExpansionReceiptLedgerErrorV1::ForeignReservation);
        }
        if !self.open.remove(&reservation.ordinal) {
            return Err(RawExpansionReceiptLedgerErrorV1::UnknownReservation);
        }
        Ok(AbortedRawExpansionReceiptLedgerV1 {
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
