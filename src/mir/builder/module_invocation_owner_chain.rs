//! CUT0-I0-ROOT0-BRAND0: one generic physical-owner brand carrier.
//!
//! The carrier stores only the opaque invocation brand and a real payload.
//! Placeholder session/shell/collector products are intentionally absent;
//! the active invocation constructor creates those physical payloads once.

use super::module_invocation_identity::ModuleInvocationBrandV1;

#[derive(Debug)]
pub(in crate::mir) struct InvocationBranded<T> {
    brand: ModuleInvocationBrandV1,
    payload: T,
    _seal: InvocationBrandedOwnerSealV1,
}

#[derive(Debug)]
struct InvocationBrandedOwnerSealV1;

pub(in crate::mir) type BrandedShellV1<T> = InvocationBranded<T>;
pub(in crate::mir) type BrandedCollectorV1<T> = InvocationBranded<T>;
pub(in crate::mir::builder) type BrandedLedgerV1<T> = InvocationBranded<T>;
pub(in crate::mir::builder) type BrandedCompleteV1<T> = InvocationBranded<T>;
pub(in crate::mir::builder) type BrandedDrainedV1<T> = InvocationBranded<T>;
pub(in crate::mir::builder) type BrandedFinalizedV1<T> = InvocationBranded<T>;

impl<T> InvocationBranded<T> {
    /// The only production-side constructor: the caller must already own a
    /// source-sealed brand and supplies one real physical payload.
    pub(in crate::mir) fn from_source(brand: ModuleInvocationBrandV1, payload: T) -> Self {
        Self {
            brand,
            payload,
            _seal: InvocationBrandedOwnerSealV1,
        }
    }

    #[cfg(test)]
    pub(in crate::mir::builder) fn from_test(brand: ModuleInvocationBrandV1, payload: T) -> Self {
        Self::from_source(brand, payload)
    }

    pub(in crate::mir) fn brand(&self) -> ModuleInvocationBrandV1 {
        self.brand
    }

    pub(in crate::mir) fn payload(&self) -> &T {
        &self.payload
    }

    pub(in crate::mir) fn payload_mut(&mut self) -> &mut T {
        &mut self.payload
    }

    pub(in crate::mir) fn into_payload(self) -> T {
        self.payload
    }
}
