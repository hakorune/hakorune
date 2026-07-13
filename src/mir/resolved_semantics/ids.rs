//! Function-owner-local semantic handles.
//!
//! The owner token prevents accidental cross-function lookup. It is an
//! invocation-local membership brand, not source identity or parity data.

use hakorune_mir_core::BindingId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FunctionOwnerIdV1(u32);

/// One owner-brand issuer per active function-compilation session.
///
/// The emitted slot is an invocation-local membership brand. It is never a
/// source identity or Rust/Hako parity value.
#[derive(Debug, Default)]
pub(crate) struct FunctionOwnerIssuerV1 {
    next_slot: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct FunctionOwnerIssueExhaustedV1;

impl FunctionOwnerIssuerV1 {
    pub(crate) const fn new_for_compilation() -> Self {
        Self { next_slot: 0 }
    }

    pub(crate) fn issue(&mut self) -> Result<FunctionOwnerIdV1, FunctionOwnerIssueExhaustedV1> {
        let slot = self.next_slot;
        self.next_slot = self
            .next_slot
            .checked_add(1)
            .ok_or(FunctionOwnerIssueExhaustedV1)?;
        Ok(FunctionOwnerIdV1(slot))
    }
}

/// Owner-branded reference to the canonical lexical BindingId.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BindingRefV1 {
    owner: FunctionOwnerIdV1,
    binding: BindingId,
}

impl BindingRefV1 {
    pub(crate) const fn new(owner: FunctionOwnerIdV1, binding: BindingId) -> Self {
        Self { owner, binding }
    }

    pub const fn owner(self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub const fn binding(self) -> BindingId {
        self.binding
    }
}

/// Lexical-scope handle local to one resolved function.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ScopeId {
    owner: FunctionOwnerIdV1,
    slot: u32,
}

impl ScopeId {
    pub(crate) const fn new(owner: FunctionOwnerIdV1, slot: u32) -> Self {
        Self { owner, slot }
    }

    pub const fn owner(self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub const fn slot(self) -> u32 {
        self.slot
    }
}

/// Structured-control-region handle local to one resolved function.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RegionId {
    owner: FunctionOwnerIdV1,
    slot: u32,
}

impl RegionId {
    pub(crate) const fn new(owner: FunctionOwnerIdV1, slot: u32) -> Self {
        Self { owner, slot }
    }

    pub const fn owner(self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub const fn slot(self) -> u32 {
        self.slot
    }
}
