//! CUT0-I0-CANON-BRIDGE0-COMPLETION.
//!
//! Compiler-side completion for the real physical owner.  This deliberately
//! does not reuse the older Builder-only canonical completion scaffold: the
//! source continuation and the collector/receipt product already came from
//! the compiler-owned bridge and are moved here unchanged.

use super::canonical_drain_manifest::CanonicalDrainManifestErrorV1;
use super::source_bound_package::{
    CanonicalSourceContinuationV1, CollectedCanonicalPhysicalInvocationV1,
};
use crate::mir::builder::{
    CanonicalCallableCapabilityWitnessV1, CanonicalDrainedCallablePhysicalV1,
    CanonicalDrainedSinglePhysicalV1, CanonicalPhysicalDrainPrepareErrorV1,
    CollectedCanonicalCallablePhysicalV1, CollectedCanonicalSinglePhysicalV1,
    ModuleBuilderInvocationSessionV1, PreparedCanonicalCallablePhysicalDrainV1,
    PreparedCanonicalSinglePhysicalDrainV1, RejectedCanonicalCallablePhysicalDrainV1,
    RejectedCanonicalSinglePhysicalDrainV1,
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
pub(in crate::mir) enum CanonicalDrainedInvocationV1<'a> {
    Single(CanonicalDrainedSingleInvocationV1<'a>),
    Callable(CanonicalDrainedCallableInvocationV1<'a>),
}

#[derive(Debug)]
pub(in crate::mir) struct CanonicalDrainedSingleInvocationV1<'a> {
    pub(in crate::mir) token: ModuleInvocationTokenV1,
    pub(in crate::mir) continuation: CanonicalSourceContinuationV1<'a>,
    pub(in crate::mir) session: ModuleBuilderInvocationSessionV1,
    pub(in crate::mir) physical: CanonicalDrainedSinglePhysicalV1,
}

#[derive(Debug)]
pub(in crate::mir) struct CanonicalDrainedCallableInvocationV1<'a> {
    pub(in crate::mir) token: ModuleInvocationTokenV1,
    pub(in crate::mir) continuation: CanonicalSourceContinuationV1<'a>,
    pub(in crate::mir) session: ModuleBuilderInvocationSessionV1,
    pub(in crate::mir) capability: CanonicalCallableCapabilityWitnessV1,
    pub(in crate::mir) physical: CanonicalDrainedCallablePhysicalV1,
}

#[derive(Debug)]
pub(in crate::mir) enum PreparedCanonicalDrainV1<'a> {
    Single {
        token: ModuleInvocationTokenV1,
        continuation: CanonicalSourceContinuationV1<'a>,
        session: ModuleBuilderInvocationSessionV1,
        physical: PreparedCanonicalSinglePhysicalDrainV1,
    },
    Callable {
        token: ModuleInvocationTokenV1,
        continuation: CanonicalSourceContinuationV1<'a>,
        session: ModuleBuilderInvocationSessionV1,
        capability: CanonicalCallableCapabilityWitnessV1,
        physical: PreparedCanonicalCallablePhysicalDrainV1,
    },
}

#[derive(Debug)]
pub(in crate::mir) enum RejectedCanonicalDrainOwnerV1<'a> {
    Complete(CanonicalPhysicalCompleteInvocationV1<'a>),
    Single {
        token: ModuleInvocationTokenV1,
        continuation: CanonicalSourceContinuationV1<'a>,
        session: ModuleBuilderInvocationSessionV1,
        physical: RejectedCanonicalSinglePhysicalDrainV1,
    },
    Callable {
        token: ModuleInvocationTokenV1,
        continuation: CanonicalSourceContinuationV1<'a>,
        session: ModuleBuilderInvocationSessionV1,
        capability: CanonicalCallableCapabilityWitnessV1,
        physical: RejectedCanonicalCallablePhysicalDrainV1,
    },
}

#[derive(Debug)]
pub(in crate::mir) enum CanonicalDrainPrepareErrorV1 {
    Manifest(CanonicalDrainManifestErrorV1),
    Physical(CanonicalPhysicalDrainPrepareErrorV1),
}

#[derive(Debug)]
pub(in crate::mir) struct RejectedCanonicalPhysicalCompletionV1<'a> {
    pub(in crate::mir) owner: CollectedCanonicalPhysicalInvocationV1<'a>,
    pub(in crate::mir) error: CanonicalPhysicalCompletionErrorV1,
}

#[derive(Debug)]
pub(in crate::mir) struct RejectedCanonicalDrainV1<'a> {
    pub(in crate::mir) owner: RejectedCanonicalDrainOwnerV1<'a>,
    pub(in crate::mir) error: CanonicalDrainPrepareErrorV1,
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
    ) -> Result<CanonicalPhysicalCompleteInvocationV1<'a>, RejectedCanonicalPhysicalCompletionV1<'a>>
    {
        match &self {
            Self::Single {
                token,
                session,
                physical,
                ..
            } => {
                if !same_brand(token, session, physical.brand(), physical.receipt_brand()) {
                    return Err(RejectedCanonicalPhysicalCompletionV1 {
                        owner: self,
                        error: CanonicalPhysicalCompletionErrorV1::ForeignBrand,
                    });
                }
            }
            Self::Callable {
                token,
                session,
                capability,
                physical,
                ..
            } => {
                if !same_brand(token, session, physical.brand(), physical.receipt_brand()) {
                    return Err(RejectedCanonicalPhysicalCompletionV1 {
                        owner: self,
                        error: CanonicalPhysicalCompletionErrorV1::ForeignBrand,
                    });
                }
                if capability.brand() != token.brand() || capability.family() != token.family() {
                    return Err(RejectedCanonicalPhysicalCompletionV1 {
                        owner: self,
                        error: CanonicalPhysicalCompletionErrorV1::CapabilityMismatch,
                    });
                }
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

impl<'a> CanonicalPhysicalCompleteInvocationV1<'a> {
    #[cfg(test)]
    pub(in crate::mir) fn publish_single_shell_for_test(&mut self) {
        match self {
            Self::Single(product) => product.physical.publish_probe_for_test(),
            Self::Callable(_) => panic!("single-shell test seam used for callable route"),
        }
    }

    /// Consume the complete invocation and perform all source/physical
    /// preparation before the one-shot Builder drain.  No shell mutation is
    /// reachable from this terminal.
    pub(in crate::mir) fn prepare_drain(
        self,
    ) -> Result<PreparedCanonicalDrainV1<'a>, RejectedCanonicalDrainV1<'a>> {
        match self {
            Self::Single(CanonicalSinglePhysicalCompleteInvocationV1 {
                token,
                continuation,
                session,
                physical,
            }) => {
                let manifest = match continuation
                    .project_drain_manifest(token.brand())
                    .map(|manifest| manifest.into_physical())
                {
                    Ok(manifest) => manifest,
                    Err(error) => {
                        return Err(RejectedCanonicalDrainV1 {
                            owner: RejectedCanonicalDrainOwnerV1::Complete(
                                CanonicalPhysicalCompleteInvocationV1::Single(
                                    CanonicalSinglePhysicalCompleteInvocationV1 {
                                        token,
                                        continuation,
                                        session,
                                        physical,
                                    },
                                ),
                            ),
                            error: CanonicalDrainPrepareErrorV1::Manifest(error),
                        })
                    }
                };
                match physical.prepare_drain(manifest) {
                    Ok(physical) => Ok(PreparedCanonicalDrainV1::Single {
                        token,
                        continuation,
                        session,
                        physical,
                    }),
                    Err(physical) => {
                        let error = physical.error().clone();
                        Err(RejectedCanonicalDrainV1 {
                            owner: RejectedCanonicalDrainOwnerV1::Single {
                                token,
                                continuation,
                                session,
                                physical,
                            },
                            error: CanonicalDrainPrepareErrorV1::Physical(error),
                        })
                    }
                }
            }
            Self::Callable(CanonicalCallablePhysicalCompleteInvocationV1 {
                token,
                continuation,
                session,
                capability,
                physical,
            }) => {
                let manifest = match continuation
                    .project_drain_manifest(token.brand())
                    .map(|manifest| manifest.into_physical())
                {
                    Ok(manifest) => manifest,
                    Err(error) => {
                        return Err(RejectedCanonicalDrainV1 {
                            owner: RejectedCanonicalDrainOwnerV1::Complete(
                                CanonicalPhysicalCompleteInvocationV1::Callable(
                                    CanonicalCallablePhysicalCompleteInvocationV1 {
                                        token,
                                        continuation,
                                        session,
                                        capability,
                                        physical,
                                    },
                                ),
                            ),
                            error: CanonicalDrainPrepareErrorV1::Manifest(error),
                        })
                    }
                };
                match physical.prepare_drain(manifest) {
                    Ok(physical) => Ok(PreparedCanonicalDrainV1::Callable {
                        token,
                        continuation,
                        session,
                        capability,
                        physical,
                    }),
                    Err(physical) => {
                        let error = physical.error().clone();
                        Err(RejectedCanonicalDrainV1 {
                            owner: RejectedCanonicalDrainOwnerV1::Callable {
                                token,
                                continuation,
                                session,
                                capability,
                                physical,
                            },
                            error: CanonicalDrainPrepareErrorV1::Physical(error),
                        })
                    }
                }
            }
        }
    }
}

impl<'a> PreparedCanonicalDrainV1<'a> {
    /// The prepared physical owner is the sole infallible drain terminal.
    pub(in crate::mir) fn drain(self) -> CanonicalDrainedInvocationV1<'a> {
        match self {
            Self::Single {
                token,
                continuation,
                session,
                physical,
            } => CanonicalDrainedInvocationV1::Single(CanonicalDrainedSingleInvocationV1 {
                token,
                continuation,
                session,
                physical: physical.drain(),
            }),
            Self::Callable {
                token,
                continuation,
                session,
                capability,
                physical,
            } => CanonicalDrainedInvocationV1::Callable(CanonicalDrainedCallableInvocationV1 {
                token,
                continuation,
                session,
                capability,
                physical: physical.drain(),
            }),
        }
    }
}
