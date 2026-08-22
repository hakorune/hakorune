//! Strict callable-entry Text formal lane.
//!
//! This is a caller-zero runtime capability, not a TextEq or loop
//! physicalizer.  A source/header issuer supplies the published slot and the
//! host-handle registry generation is carried alongside it.  The payload is
//! lent only through a closure, so the borrow cannot escape the entry call.

use super::host_handles::{self, TextFormalLookupRejectV1};

/// Fixed-width status values shared by the Rust validator and the C wire.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextFormalBorrowStatusV1 {
    Valid = 0,
    ZeroOrOutOfRangeSlot = 1,
    MissingSlot = 2,
    GenerationMismatch = 3,
    NonTextPayload = 4,
}

impl TextFormalBorrowStatusV1 {
    #[inline(always)]
    pub const fn as_u32(self) -> u32 {
        self as u32
    }

    #[inline(always)]
    fn from_lookup_reject(reject: TextFormalLookupRejectV1) -> Self {
        match reject {
            TextFormalLookupRejectV1::ZeroOrOutOfRangeSlot => Self::ZeroOrOutOfRangeSlot,
            TextFormalLookupRejectV1::MissingSlot => Self::MissingSlot,
            TextFormalLookupRejectV1::GenerationMismatch => Self::GenerationMismatch,
            TextFormalLookupRejectV1::NonTextPayload => Self::NonTextPayload,
        }
    }
}

/// Generation-branded callable Text formal capability.
///
/// The fields are private and the type is intentionally neither `Clone` nor
/// `Copy`.  A caller must obtain it from the sole issuer below; it cannot
/// manufacture a raw-handle-only formal lane.
#[repr(C)]
#[derive(Debug)]
pub struct TextFormalBorrowV1 {
    slot: u64,
    generation: u64,
}

/// Runtime-private pair projection used only by the call-lifetime owner.
///
/// This is not a semantic formal, source binding, or public ABI surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TextFormalWirePairV1 {
    pub(super) slot: u64,
    pub(super) generation: u64,
}

impl TextFormalWirePairV1 {
    /// Construct the already-published physical pair used by a caller-zero
    /// residence entry.  This never captures a generation from a raw handle.
    #[inline(always)]
    pub(crate) const fn from_published_wire(slot: u64, generation: u64) -> Self {
        Self { slot, generation }
    }

    #[inline(always)]
    pub(crate) const fn slot(self) -> u64 {
        self.slot
    }

    #[inline(always)]
    pub(crate) const fn generation(self) -> u64 {
        self.generation
    }
}

impl TextFormalBorrowV1 {
    #[inline(always)]
    pub(super) const fn wire_pair(&self) -> TextFormalWirePairV1 {
        TextFormalWirePairV1 {
            slot: self.slot,
            generation: self.generation,
        }
    }

    /// Validate the captured slot/generation once and consume the capability.
    #[inline(always)]
    pub fn validate(self) -> Result<(), TextFormalBorrowStatusV1> {
        host_handles::with_text_formal_wire(self.slot, self.generation, |_| ())
            .map(|_| ())
            .map_err(TextFormalBorrowStatusV1::from_lookup_reject)
    }

    /// Lend the live Text payload for the duration of one closure.
    #[inline(always)]
    pub fn with_text<R>(self, f: impl FnOnce(&str) -> R) -> Result<R, TextFormalBorrowStatusV1> {
        host_handles::with_text_formal_wire(self.slot, self.generation, f)
            .map_err(TextFormalBorrowStatusV1::from_lookup_reject)
    }
}

/// Issue a validated wire from already-published StableText lanes.  The
/// caller supplies both halves of the generation-branded pair; this function
/// never recaptures a generation from a raw handle and never exposes a raw
/// tuple outside the runtime owner.
#[inline(always)]
pub(crate) fn issue_stable_text_formal_wire_v1(
    slot: u64,
    generation: u64,
) -> Result<TextFormalWirePairV1, TextFormalBorrowStatusV1> {
    if slot == 0 || generation == 0 {
        return Err(TextFormalBorrowStatusV1::ZeroOrOutOfRangeSlot);
    }
    host_handles::with_stable_text_formal_wire(slot, generation, |_| ())
        .map(|_| TextFormalWirePairV1::from_published_wire(slot, generation))
        .map_err(TextFormalBorrowStatusV1::from_lookup_reject)
}

/// Issue one generation-branded formal from an already-published entry lane.
///
/// Unlike `issue_text_formal_borrow_v1`, this seam never accepts a raw handle
/// and never recaptures a generation.  The lane pair is validated against the
/// exact live payload first; the returned capability is then consumed by the
/// invocation Residence owner.
#[inline(always)]
pub(crate) fn issue_text_formal_borrow_from_published_wire_v1(
    slot: u64,
    generation: u64,
) -> Result<TextFormalBorrowV1, TextFormalBorrowStatusV1> {
    if slot == 0 || generation == 0 {
        return Err(TextFormalBorrowStatusV1::ZeroOrOutOfRangeSlot);
    }
    host_handles::with_text_formal_wire(slot, generation, |_| ())
        .map(|_| TextFormalBorrowV1 { slot, generation })
        .map_err(TextFormalBorrowStatusV1::from_lookup_reject)
}

/// Issue a move-only batch from adjacent published ExactText entry lanes.
///
/// The slice is an invocation-entry transport view only.  No pair is stored
/// in the returned product, and the Residence owner performs the final
/// all-pairs write-lock validation/pin transaction.
#[inline(always)]
pub(crate) fn issue_text_formal_borrows_from_published_wires_v1(
    wires: &[(u64, u64)],
) -> Result<Box<[TextFormalBorrowV1]>, TextFormalBorrowStatusV1> {
    wires
        .iter()
        .map(|&(slot, generation)| {
            issue_text_formal_borrow_from_published_wire_v1(slot, generation)
        })
        .collect::<Result<Vec<_>, _>>()
        .map(Vec::into_boxed_slice)
}

/// Sole Rust issuer for a callable-entry Text formal capability.
#[inline(always)]
pub fn issue_text_formal_borrow_v1(
    handle: u64,
) -> Result<TextFormalBorrowV1, TextFormalBorrowStatusV1> {
    host_handles::capture_text_formal_pair(handle)
        .map(|(slot, generation)| TextFormalBorrowV1 { slot, generation })
        .map_err(TextFormalBorrowStatusV1::from_lookup_reject)
}

/// Validate an already published `{slot,generation}` pair for the fixed C
/// status projection.  This function does not recapture a generation from a
/// raw handle and therefore rejects stale identities fail-closed.
#[inline(always)]
pub fn validate_text_formal_wire_v1(slot: u64, generation: u64) -> TextFormalBorrowStatusV1 {
    if slot == 0 || generation == 0 {
        return TextFormalBorrowStatusV1::ZeroOrOutOfRangeSlot;
    }
    host_handles::with_text_formal_wire(slot, generation, |_| ())
        .map(|_| TextFormalBorrowStatusV1::Valid)
        .unwrap_or_else(TextFormalBorrowStatusV1::from_lookup_reject)
}

#[cfg(test)]
#[path = "text_formal_abi_tests.rs"]
mod tests;
