//! ROOT-RETENTION0-PREFLIGHT ledger-side borrowed validation.
//!
//! This child module keeps the mutable ledger owner and its later commit
//! implementation separate from the read-only reservation/history proof.

use super::super::module_draft_collector::{DraftPublicationPolicyV1, FunctionDraftKeyV1};
use super::super::raw_expansion_receipt_ledger::RawExpansionReservationV1;
use super::{RawExpansionReceiptLedgerErrorV1, RawExpansionReceiptLedgerV1};

impl RawExpansionReceiptLedgerV1 {
    /// Borrow-only reservation/history validation for the required Raw root
    /// pair. No open reservation, event, or index is changed here; the
    /// retention preflight can return the complete ledger owner on failure.
    pub(in crate::mir::builder) fn validate_required_root_batch(
        &self,
        main_reservation: &RawExpansionReservationV1,
        condition_reservation: &RawExpansionReservationV1,
    ) -> Result<(), RawExpansionReceiptLedgerErrorV1> {
        if self.poisoned {
            return Err(RawExpansionReceiptLedgerErrorV1::LedgerPoisoned);
        }
        self.validate_root_reservation(
            main_reservation,
            &FunctionDraftKeyV1::Main,
            "main",
            0,
            DraftPublicationPolicyV1::LegacyReplaceWholePair,
        )?;
        self.validate_root_reservation(
            condition_reservation,
            &FunctionDraftKeyV1::SyntheticConditionFn,
            "condition_fn",
            1,
            DraftPublicationPolicyV1::CanonicalRejectDuplicate,
        )?;

        // Main may replace an existing legacy pair, but a half-pair is
        // already ledger corruption. Condition is insert-only.
        let main_key = FunctionDraftKeyV1::Main;
        let main_event = self.final_event_by_key.get(&main_key);
        let main_index = self.key_by_symbol.get("main");
        if main_event.is_some() != main_index.is_some()
            || main_index.is_some_and(|key| key != &main_key)
        {
            return Err(RawExpansionReceiptLedgerErrorV1::ReplacementHistoryMismatch);
        }
        let condition_key = FunctionDraftKeyV1::SyntheticConditionFn;
        if self.final_event_by_key.contains_key(&condition_key)
            || self.key_by_symbol.contains_key("condition_fn")
        {
            return Err(RawExpansionReceiptLedgerErrorV1::InsertedReceiptCollision);
        }
        Ok(())
    }

    fn validate_root_reservation(
        &self,
        reservation: &RawExpansionReservationV1,
        expected_key: &FunctionDraftKeyV1,
        expected_symbol: &str,
        expected_arity: usize,
        expected_policy: DraftPublicationPolicyV1,
    ) -> Result<(), RawExpansionReceiptLedgerErrorV1> {
        if reservation.brand != self.brand || !self.open.contains(&reservation.ordinal) {
            return Err(RawExpansionReceiptLedgerErrorV1::ForeignReservation);
        }
        if reservation.request.key != *expected_key
            || reservation.request.symbol.as_ref() != expected_symbol
            || reservation.request.arity != expected_arity
            || reservation.request.policy != expected_policy
        {
            return Err(RawExpansionReceiptLedgerErrorV1::ReceiptKeyMismatch);
        }
        Ok(())
    }
}
