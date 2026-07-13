//! Function-owner-local semantic handles.
//!
//! The owner token prevents accidental cross-function lookup. It is an
//! invocation-local membership brand, not source identity or parity data.

use hakorune_mir_core::BindingId;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_COMPILATION_BRAND: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FunctionOwnerIdV1 {
    compilation: u64,
    slot: u32,
}

/// One owner-brand issuer per active function-compilation session.
///
/// The emitted slot is an invocation-local membership brand. It is never a
/// source identity or Rust/Hako parity value.
#[derive(Debug)]
pub(crate) struct FunctionOwnerIssuerV1 {
    compilation: u64,
    next_slot: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FunctionOwnerIssueExhaustedV1 {
    CompilationBrand,
    FunctionSlot,
}

impl FunctionOwnerIssuerV1 {
    pub(crate) fn new_for_compilation() -> Result<Self, FunctionOwnerIssueExhaustedV1> {
        let compilation = NEXT_COMPILATION_BRAND
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
                current.checked_add(1)
            })
            .map_err(|_| FunctionOwnerIssueExhaustedV1::CompilationBrand)?;
        Ok(Self {
            compilation,
            next_slot: 0,
        })
    }

    pub(crate) fn issue(&mut self) -> Result<FunctionOwnerIdV1, FunctionOwnerIssueExhaustedV1> {
        let slot = self.next_slot;
        self.next_slot = self
            .next_slot
            .checked_add(1)
            .ok_or(FunctionOwnerIssueExhaustedV1::FunctionSlot)?;
        Ok(FunctionOwnerIdV1 {
            compilation: self.compilation,
            slot,
        })
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
