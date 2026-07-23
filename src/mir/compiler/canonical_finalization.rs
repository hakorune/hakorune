//! CUT0-I0-FINAL0: route-specific finalization handoff.
//!
//! This row consumes the real DRAIN0 products without adapting them to the
//! legacy Main-only candidate.  It remains disconnected from public ingress;
//! POST0 and COMMIT0 will consume the finalized product later.

use super::canonical_physical_completion::{
    CanonicalDrainedCallableInvocationV1, CanonicalDrainedInvocationV1,
    CanonicalDrainedSingleInvocationV1,
};
use super::source_bound_package::CanonicalSourceContinuationV1;
use crate::mir::builder::{
    BuilderCommitReadinessErrorV1, CanonicalCallableCapabilityWitnessV1,
    CanonicalDrainedCallablePhysicalV1, CanonicalDrainedSinglePhysicalV1,
    ModuleBuilderInvocationSessionV1, PreparedBuilderModuleSessionV1,
};
use crate::mir::module_invocation_identity::{ModuleInvocationFamilyV1, ModuleInvocationTokenV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum CanonicalFinalizationErrorV1 {
    ForeignBrand,
    WrongFamily { family: ModuleInvocationFamilyV1 },
    BuilderReadiness(BuilderCommitReadinessErrorV1),
}

#[derive(Debug)]
pub(in crate::mir) struct CanonicalSingleFinalizationInputV1<'a> {
    pub(in crate::mir) token: ModuleInvocationTokenV1,
    pub(in crate::mir) continuation: CanonicalSourceContinuationV1<'a>,
    pub(in crate::mir) builder: PreparedBuilderModuleSessionV1,
    pub(in crate::mir) physical: CanonicalDrainedSinglePhysicalV1,
}

#[derive(Debug)]
pub(in crate::mir) struct CanonicalCallableFinalizationInputV1<'a> {
    pub(in crate::mir) token: ModuleInvocationTokenV1,
    pub(in crate::mir) continuation: CanonicalSourceContinuationV1<'a>,
    pub(in crate::mir) builder: PreparedBuilderModuleSessionV1,
    pub(in crate::mir) capability: CanonicalCallableCapabilityWitnessV1,
    pub(in crate::mir) physical: CanonicalDrainedCallablePhysicalV1,
}

#[derive(Debug)]
pub(in crate::mir) enum CanonicalFinalizationInputV1<'a> {
    Single(CanonicalSingleFinalizationInputV1<'a>),
    Callable(CanonicalCallableFinalizationInputV1<'a>),
}

#[derive(Debug)]
pub(in crate::mir) struct FinalizedModuleInvocationV1<'a> {
    pub(in crate::mir) input: CanonicalFinalizationInputV1<'a>,
    _seal: FinalizedModuleInvocationSealV1,
}

#[derive(Debug)]
struct FinalizedModuleInvocationSealV1;

#[derive(Debug)]
pub(in crate::mir) enum RejectedCanonicalFinalizationOwnerV1<'a> {
    Single {
        token: ModuleInvocationTokenV1,
        continuation: CanonicalSourceContinuationV1<'a>,
        session: ModuleBuilderInvocationSessionV1,
        physical: CanonicalDrainedSinglePhysicalV1,
    },
    Callable {
        token: ModuleInvocationTokenV1,
        continuation: CanonicalSourceContinuationV1<'a>,
        session: ModuleBuilderInvocationSessionV1,
        capability: CanonicalCallableCapabilityWitnessV1,
        physical: CanonicalDrainedCallablePhysicalV1,
    },
}

#[derive(Debug)]
pub(in crate::mir) struct RejectedCanonicalFinalizationV1<'a> {
    pub(in crate::mir) owner: RejectedCanonicalFinalizationOwnerV1<'a>,
    pub(in crate::mir) error: CanonicalFinalizationErrorV1,
}

impl<'a> CanonicalDrainedInvocationV1<'a> {
    /// Consume the drained owner and prepare a route-specific finalization
    /// input.  Builder readiness is checked before any finalizer work.
    pub(in crate::mir) fn prepare_finalization(
        self,
    ) -> Result<CanonicalFinalizationInputV1<'a>, RejectedCanonicalFinalizationV1<'a>> {
        match self {
            Self::Single(drained) => prepare_single(drained),
            Self::Callable(drained) => prepare_callable(drained),
        }
    }
}

pub(in crate::mir) struct CanonicalModuleFinalizerV1;

impl CanonicalModuleFinalizerV1 {
    pub(in crate::mir) fn finalize<'a>(
        input: CanonicalFinalizationInputV1<'a>,
    ) -> Result<FinalizedModuleInvocationV1<'a>, CanonicalFinalizationErrorV1> {
        validate_input(&input)?;
        Ok(FinalizedModuleInvocationV1 {
            input,
            _seal: FinalizedModuleInvocationSealV1,
        })
    }
}

fn prepare_single<'a>(
    drained: CanonicalDrainedSingleInvocationV1<'a>,
) -> Result<CanonicalFinalizationInputV1<'a>, RejectedCanonicalFinalizationV1<'a>> {
    let CanonicalDrainedSingleInvocationV1 {
        token,
        continuation,
        session,
        physical,
    } = drained;
    let builder = match session.prepare_module_session() {
        Ok(builder) => builder,
        Err(rejected) => {
            let error = rejected.error().clone();
            let session = rejected.into_parts().0;
            return Err(RejectedCanonicalFinalizationV1 {
                owner: RejectedCanonicalFinalizationOwnerV1::Single {
                    token,
                    continuation,
                    session,
                    physical,
                },
                error: CanonicalFinalizationErrorV1::BuilderReadiness(error),
            });
        }
    };
    Ok(CanonicalFinalizationInputV1::Single(
        CanonicalSingleFinalizationInputV1 {
            token,
            continuation,
            builder,
            physical,
        },
    ))
}

fn prepare_callable<'a>(
    drained: CanonicalDrainedCallableInvocationV1<'a>,
) -> Result<CanonicalFinalizationInputV1<'a>, RejectedCanonicalFinalizationV1<'a>> {
    let CanonicalDrainedCallableInvocationV1 {
        token,
        continuation,
        session,
        capability,
        physical,
    } = drained;
    let builder = match session.prepare_module_session() {
        Ok(builder) => builder,
        Err(rejected) => {
            let error = rejected.error().clone();
            let session = rejected.into_parts().0;
            return Err(RejectedCanonicalFinalizationV1 {
                owner: RejectedCanonicalFinalizationOwnerV1::Callable {
                    token,
                    continuation,
                    session,
                    capability,
                    physical,
                },
                error: CanonicalFinalizationErrorV1::BuilderReadiness(error),
            });
        }
    };
    Ok(CanonicalFinalizationInputV1::Callable(
        CanonicalCallableFinalizationInputV1 {
            token,
            continuation,
            builder,
            capability,
            physical,
        },
    ))
}

fn validate_input(
    input: &CanonicalFinalizationInputV1<'_>,
) -> Result<(), CanonicalFinalizationErrorV1> {
    match input {
        CanonicalFinalizationInputV1::Single(input) => {
            if input.token.brand() != input.builder.brand()
                || input.token.family() != input.builder.family()
                || input.token.brand() != input.physical.brand
                || input.token.family() != input.physical.family
            {
                return Err(CanonicalFinalizationErrorV1::ForeignBrand);
            }
            if !matches!(
                input.token.family(),
                ModuleInvocationFamilyV1::CanonicalAPlus
                    | ModuleInvocationFamilyV1::BindingSsaTrivial
            ) {
                return Err(CanonicalFinalizationErrorV1::WrongFamily {
                    family: input.token.family(),
                });
            }
        }
        CanonicalFinalizationInputV1::Callable(input) => {
            if input.token.brand() != input.builder.brand()
                || input.token.family() != input.builder.family()
                || input.token.brand() != input.physical.brand
                || input.token.family() != input.physical.family
                || input.capability.brand() != input.token.brand()
                || input.capability.family() != input.token.family()
            {
                return Err(CanonicalFinalizationErrorV1::ForeignBrand);
            }
            if !matches!(
                input.token.family(),
                ModuleInvocationFamilyV1::BindingSsaAcyclic
                    | ModuleInvocationFamilyV1::BindingSsaRecursive
            ) {
                return Err(CanonicalFinalizationErrorV1::WrongFamily {
                    family: input.token.family(),
                });
            }
        }
    }
    Ok(())
}
