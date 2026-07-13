//! Function-owner-local semantic handles.
//!
//! The owner token prevents accidental cross-function lookup. It is an
//! invocation-local membership brand, not source identity or parity data.

use hakorune_mir_core::BindingId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FunctionOwnerIdV1(u32);

impl FunctionOwnerIdV1 {
    pub(crate) const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> u32 {
        self.0
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
