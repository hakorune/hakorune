//! Module-local object/field coordinates, not source membership proofs.
//!
//! The semantic package assigns object indices once. Callers must retain its
//! invocation identity until atomic publication; equal indices from different
//! modules do not identify the same declaration. Runtime type IDs are separate.

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CanonicalObjectIdV1(u32);

impl CanonicalObjectIdV1 {
    /// Encode an already-selected module declaration index without truncation.
    /// This conversion neither resolves a source declaration nor admits a use.
    pub fn from_declaration_index(index: usize) -> Option<Self> {
        u32::try_from(index).ok().map(Self)
    }

    pub const fn declaration_index(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CanonicalFieldRefV1 {
    object: CanonicalObjectIdV1,
    declaration_ordinal: u32,
}

impl CanonicalFieldRefV1 {
    /// Encode a source-selected field. Definition/range validation belongs to
    /// the source issuer and atomic publication, not this structural carrier.
    pub fn from_declaration_ordinal(
        object: CanonicalObjectIdV1,
        ordinal: usize,
    ) -> Option<Self> {
        Some(Self { object, declaration_ordinal: u32::try_from(ordinal).ok()? })
    }

    pub const fn object(self) -> CanonicalObjectIdV1 {
        self.object
    }

    pub const fn declaration_ordinal(self) -> u32 {
        self.declaration_ordinal
    }
}
