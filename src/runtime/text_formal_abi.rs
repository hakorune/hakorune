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
pub(super) struct TextFormalWirePairV1 {
    pub(super) slot: u64,
    pub(super) generation: u64,
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
