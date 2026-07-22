//! CUT0-I0-CANON-BRIDGE0-COMPLETION.
//!
//! Compiler-side completion for the real physical owner.  This deliberately
//! does not reuse the older Builder-only canonical completion scaffold: the
//! source continuation and the collector/receipt product already came from
//! the compiler-owned bridge and are moved here unchanged.

use super::source_bound_package::{
    CanonicalSourceContinuationV1, CollectedCanonicalPhysicalInvocationV1,
};
use crate::mir::builder::{
    CanonicalCallableCapabilityWitnessV1, CollectedCanonicalCallablePhysicalV1,
    CollectedCanonicalSinglePhysicalV1, ModuleBuilderInvocationSessionV1,
};
use crate::mir::module_invocation_identity::{ModuleInvocationBrandV1, ModuleInvocationTokenV1};

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir) enum CanonicalPhysicalCompletionErrorV1 {
    ForeignBrand,
    CapabilityMismatch,
}

#[derive(Debug)]
pub(in crate::mir) struct CanonicalSinglePhysicalCompleteInvocationV1<'a> {
    pub(in crate::mir) token: ModuleInvocationTokenV1,
    pub(in crate::mir) continuation: CanonicalSourceContinuationV1<'a>,
    pub(in crate::mir) session: ModuleBuilderInvocationSessionV1,
    pub(in crate::mir) physical: CollectedCanonicalSinglePhysicalV1,
}

#[derive(Debug)]
pub(in crate::mir) struct CanonicalCallablePhysicalCompleteInvocationV1<'a> {
    pub(in crate::mir) token: ModuleInvocationTokenV1,
    pub(in crate::mir) continuation: CanonicalSourceContinuationV1<'a>,
    pub(in crate::mir) session: ModuleBuilderInvocationSessionV1,
    pub(in crate::mir) capability: CanonicalCallableCapabilityWitnessV1,
    pub(in crate::mir) physical: CollectedCanonicalCallablePhysicalV1,
}

#[derive(Debug)]
pub(in crate::mir) enum CanonicalPhysicalCompleteInvocationV1<'a> {
    Single(CanonicalSinglePhysicalCompleteInvocationV1<'a>),
    Callable(CanonicalCallablePhysicalCompleteInvocationV1<'a>),
}

#[derive(Debug)]
pub(in crate::mir) struct RejectedCanonicalPhysicalCompletionV1<'a> {
    pub(in crate::mir) owner: CollectedCanonicalPhysicalInvocationV1<'a>,
    pub(in crate::mir) error: CanonicalPhysicalCompletionErrorV1,
}

fn same_brand(
    token: &ModuleInvocationTokenV1,
    session: &ModuleBuilderInvocationSessionV1,
    physical_brand: ModuleInvocationBrandV1,
    receipt_brand: ModuleInvocationBrandV1,
) -> bool {
    let brand = token.brand();
    brand == session.brand() && brand == physical_brand && brand == receipt_brand
}

impl<'a> CollectedCanonicalPhysicalInvocationV1<'a> {
    /// Consume the collected physical owner exactly once.  No receipt,
    /// collector, source header, or capability can be supplied separately.
    pub(in crate::mir) fn complete(
        self,
    ) -> Result<CanonicalPhysicalCompleteInvocationV1<'a>, RejectedCanonicalPhysicalCompletionV1<'a>> {
        match &self {
            Self::Single {
                token,
                session,
                physical,
                ..
            } if same_brand(token, session, physical.brand(), physical.receipt_brand()) => {}
            Self::Callable {
                token,
                session,
                capability,
                physical,
                ..
            } if same_brand(token, session, physical.brand(), physical.receipt_brand())
                && capability.brand() == token.brand()
                && capability.family() == token.family() => {}
            _ => {
                return Err(RejectedCanonicalPhysicalCompletionV1 {
                    owner: self,
                    error: CanonicalPhysicalCompletionErrorV1::ForeignBrand,
                })
            }
        }

        match self {
            Self::Single {
                token,
                continuation,
                session,
                physical,
            } => Ok(CanonicalPhysicalCompleteInvocationV1::Single(
                CanonicalSinglePhysicalCompleteInvocationV1 {
                    token,
                    continuation,
                    session,
                    physical,
                },
            )),
            Self::Callable {
                token,
                continuation,
                session,
                capability,
                physical,
            } => Ok(CanonicalPhysicalCompleteInvocationV1::Callable(
                CanonicalCallablePhysicalCompleteInvocationV1 {
                    token,
                    continuation,
                    session,
                    capability,
                    physical,
                },
            )),
        }
    }
}
