//! ROOTBATCH0-S0c: mutation-free required Main/condition ledger plan.

use super::*;

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

/// Mutation-free plan for the required Raw root pair.  The two ordinals are
/// allocated only when the private commit terminal consumes this product.
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

impl RawExpansionReceiptLedgerV1 {
    /// Borrow-only preparation for the required Raw root pair.  No ordinal,
    /// open-set entry, event, or index is changed here.
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
        let callable_present =
            self.contains_final_role(RawExpansionDraftRoleV1::CallableMainCompatibility);
        match (self.callable_main, callable_present) {
            (RawCallableMainCompatibilityDispositionV1::Selected, false) => {
                return Err(RawExpansionReceiptLedgerErrorV1::MissingCallableMainCompatibility)
            }
            (RawCallableMainCompatibilityDispositionV1::NotSelected, true) => {
                return Err(RawExpansionReceiptLedgerErrorV1::UnexpectedCallableMainCompatibility)
            }
            _ => {}
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn required_pair_prepare_does_not_open_or_advance_ledger() {
        let ledger = RawExpansionReceiptLedgerV1::new(
            RawCallableMainCompatibilityDispositionV1::NotSelected,
        );
        let plan = ledger.prepare_required_root_pair().unwrap();
        assert!(ledger.is_clean_open());
        assert_eq!(ledger.next_ordinal, 0);
        assert!(ledger.events.is_empty());
        assert_eq!(
            plan.main_disposition(),
            &RawRootMainCommitDispositionV1::Insert
        );
    }

    #[test]
    fn required_pair_commit_materializes_exactly_two_open_slots() {
        let ledger = RawExpansionReceiptLedgerV1::new(
            RawCallableMainCompatibilityDispositionV1::NotSelected,
        );
        let plan = ledger.prepare_required_root_pair().unwrap();
        let (ledger, main, condition) = plan.commit_reservations(ledger);
        assert_eq!(main.ordinal, 0);
        assert_eq!(condition.ordinal, 1);
        assert_eq!(ledger.next_ordinal, 2);
        assert_eq!(ledger.open.len(), 2);
    }
}
