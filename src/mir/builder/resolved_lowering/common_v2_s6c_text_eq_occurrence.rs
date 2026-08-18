//! Callback-scoped source/ExactText occurrence co-seal for S6C TextEq.
//!
//! This BoxShape validates the source Needle relation against the canonical
//! entry sidecar and one session-owned segment.  It emits no instruction and
//! never exposes the sidecar's physical `ValueId` pair or a runtime wire.

use std::marker::PhantomData;

use crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1;
use crate::mir::loop_recipe_contract::S6CTextEqOccurrenceSourceViewV1;
use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIdV1};

use super::super::common_v2_segment_block_allocation::PreparedSegmentBlockReceiptV1;
use super::CommonV2CanonicalSessionRefV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum S6CTextEqOccurrenceViewRejectV1 {
    AlreadyIssued,
    OwnerMismatch,
    MissingPhysicalEntryStamp,
    SegmentScopeMismatch,
    LayoutMismatch,
    BodySegmentMissing,
    BodySegmentDuplicate,
    Sidecar(String),
    CarrierMismatch,
    Callback(String),
}

/// Opaque physical occurrence proof.  The source view and all physical row
/// borrows are tied to this callback lifetime; no slot/generation `ValueId`
/// or runtime handle is projected from it.
#[derive(Debug)]
pub(in crate::mir::builder) struct S6CTextEqOccurrencePhysicalViewV1<'view> {
    source: &'view S6CTextEqOccurrenceSourceViewV1,
    owner: FunctionOwnerIdV1,
    entry: crate::mir::BasicBlockId,
    physical_block: crate::mir::BasicBlockId,
    binding: BindingRefV1,
    logical_ordinal: u32,
    carrier: PhysicalCallableLaneCarrierV1,
    _lifetime: PhantomData<&'view ()>,
    _segment: PhantomData<&'view PreparedSegmentBlockReceiptV1>,
}

impl S6CTextEqOccurrencePhysicalViewV1<'_> {
    pub(in crate::mir::builder) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir::builder) const fn entry(&self) -> crate::mir::BasicBlockId {
        self.entry
    }

    pub(in crate::mir::builder) const fn physical_block(&self) -> crate::mir::BasicBlockId {
        self.physical_block
    }

    pub(in crate::mir::builder) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }

    pub(in crate::mir::builder) const fn logical_ordinal(&self) -> u32 {
        self.logical_ordinal
    }

    pub(in crate::mir::builder) const fn carrier(&self) -> PhysicalCallableLaneCarrierV1 {
        self.carrier
    }

    pub(in crate::mir::builder) const fn text_eq_item(
        &self,
    ) -> crate::mir::loop_recipe_contract::LoopItemKeyV1 {
        self.source.text_eq_item()
    }

    pub(in crate::mir::builder) const fn text_eq_block(
        &self,
    ) -> crate::mir::loop_recipe_contract::LoopBlockKeyV1 {
        self.source.text_eq_block()
    }

    pub(in crate::mir::builder) const fn text_eq_right(
        &self,
    ) -> crate::mir::loop_recipe_contract::LoopValueKeyV1 {
        self.source.text_eq_right()
    }
}

impl<'source, 'envelope> CommonV2CanonicalSessionRefV1<'source, 'envelope> {
    pub(in crate::mir::builder) fn with_s6c_text_eq_occurrence<R>(
        &mut self,
        segment: &PreparedSegmentBlockReceiptV1,
        callback: impl for<'view> FnOnce(S6CTextEqOccurrencePhysicalViewV1<'view>) -> Result<R, String>,
    ) -> Result<R, S6CTextEqOccurrenceViewRejectV1> {
        if self.s6c_text_eq_occurrence_issued {
            return Err(S6CTextEqOccurrenceViewRejectV1::AlreadyIssued);
        }

        let source = self.envelope.text_eq_occurrence();
        let owner = self.session.owner();
        if source.owner() != owner || segment.owner() != owner {
            return Err(S6CTextEqOccurrenceViewRejectV1::OwnerMismatch);
        }
        if !self.session.owns_segment_receipt(segment) {
            return Err(S6CTextEqOccurrenceViewRejectV1::SegmentScopeMismatch);
        }
        let stamp = self
            .session
            .physical_entry_stamp()
            .map_err(|_| S6CTextEqOccurrenceViewRejectV1::MissingPhysicalEntryStamp)?;
        if stamp.owner() != owner {
            return Err(S6CTextEqOccurrenceViewRejectV1::OwnerMismatch);
        }

        let layout = self
            .envelope
            .layout()
            .segment_for_block(source.text_eq_block())
            .ok_or(S6CTextEqOccurrenceViewRejectV1::LayoutMismatch)?;
        let mut rows = segment
            .rows()
            .iter()
            .filter(|row| row.logical_block() == source.text_eq_block());
        let row = rows
            .next()
            .ok_or(S6CTextEqOccurrenceViewRejectV1::BodySegmentMissing)?;
        if rows.next().is_some() {
            return Err(S6CTextEqOccurrenceViewRejectV1::BodySegmentDuplicate);
        }
        if row.loop_key() != layout.loop_key() || row.split_ordinal() != layout.split_ordinal() {
            return Err(S6CTextEqOccurrenceViewRejectV1::LayoutMismatch);
        }
        let entry = self
            .session
            .physical_entry_sidecar_entry()
            .map_err(S6CTextEqOccurrenceViewRejectV1::Sidecar)?;

        self.s6c_text_eq_occurrence_issued = true;
        self.session
            .with_exact_text_sidecar_row(
                source.needle_binding(),
                source.needle_ordinal(),
                |sidecar_row| {
                    if sidecar_row.carrier() != PhysicalCallableLaneCarrierV1::U64BitsOnI64 {
                        return Err(S6CTextEqOccurrenceViewRejectV1::CarrierMismatch);
                    }
                    let view = S6CTextEqOccurrencePhysicalViewV1 {
                        source,
                        owner,
                        entry,
                        physical_block: row.physical_block(),
                        binding: sidecar_row.binding(),
                        logical_ordinal: sidecar_row.logical_ordinal(),
                        carrier: sidecar_row.carrier(),
                        _lifetime: PhantomData,
                        _segment: PhantomData,
                    };
                    callback(view).map_err(S6CTextEqOccurrenceViewRejectV1::Callback)
                },
            )
            .map_err(S6CTextEqOccurrenceViewRejectV1::Sidecar)?
    }
}
