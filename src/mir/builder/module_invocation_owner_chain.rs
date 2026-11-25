//! Branded collector receipts for direct admission tests.

use super::module_invocation_identity::ModuleInvocationBrandV1;

#[derive(Debug)]
pub(in crate::mir) struct InvocationBranded<T> {
    brand: ModuleInvocationBrandV1,
    payload: T,
    _seal: InvocationBrandedOwnerSealV1,
}

#[derive(Debug)]
struct InvocationBrandedOwnerSealV1;

impl<T> InvocationBranded<T> {
    pub(in crate::mir) fn from_source(brand: ModuleInvocationBrandV1, payload: T) -> Self {
        Self {
            brand,
            payload,
            _seal: InvocationBrandedOwnerSealV1,
        }
    }

    pub(in crate::mir) fn brand(&self) -> ModuleInvocationBrandV1 {
        self.brand
    }

    pub(in crate::mir) fn payload(&self) -> &T {
        &self.payload
    }


    pub(in crate::mir) fn into_payload(self) -> T {
        self.payload
    }
}
