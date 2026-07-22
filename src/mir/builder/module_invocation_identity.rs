//! CUT0-I0-ID0-S0: one invocation identity and five-family token vocabulary.
//!
//! This row is intentionally disconnected. It brands no production shell,
//! collector, receipt, or compiler ingress yet; ID0-P0 will replace the
//! opaque source witnesses with real preflight plans and thread the identity
//! through those owners. The existing route matrix remains the family SSOT.

use std::num::NonZeroU64;

pub(in crate::mir::builder) use crate::mir::module_invocation_identity::{
    ModuleInvocationBrandV1, ModuleInvocationFamilyV1, ModuleInvocationTokenV1,
};

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum ModuleInvocationIdentityErrorV1 {
    OrdinalExhausted,
    FamilySourceMismatch {
        family: ModuleInvocationFamilyV1,
        source_family: ModuleInvocationFamilyV1,
    },
}

impl std::fmt::Display for ModuleInvocationIdentityErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "[freeze:contract][invocation_identity] {self:?}")
    }
}

impl std::error::Error for ModuleInvocationIdentityErrorV1 {}

#[cfg(test)]
pub(in crate::mir::builder) struct TestInvocationPreflightFactoryV1 {
    next: u64,
}

#[cfg(test)]
impl TestInvocationPreflightFactoryV1 {
    pub(in crate::mir::builder) fn new() -> Self {
        Self { next: 1 }
    }

    pub(in crate::mir::builder) fn mint(
        &mut self,
        family: ModuleInvocationFamilyV1,
    ) -> Result<ModuleInvocationTokenV1, ModuleInvocationIdentityErrorV1> {
        self.mint_with_source(family, family)
    }

    pub(in crate::mir::builder) fn mint_with_source(
        &mut self,
        family: ModuleInvocationFamilyV1,
        source_family: ModuleInvocationFamilyV1,
    ) -> Result<ModuleInvocationTokenV1, ModuleInvocationIdentityErrorV1> {
        if family != source_family {
            return Err(ModuleInvocationIdentityErrorV1::FamilySourceMismatch {
                family,
                source_family,
            });
        }
        let ordinal = NonZeroU64::new(self.next)
            .ok_or(ModuleInvocationIdentityErrorV1::OrdinalExhausted)?;
        self.next = self
            .next
            .checked_add(1)
            .ok_or(ModuleInvocationIdentityErrorV1::OrdinalExhausted)?;
        Ok(ModuleInvocationTokenV1::from_test(ordinal, family))
    }
}
