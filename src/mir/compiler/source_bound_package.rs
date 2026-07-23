//! CUT0-I0-ROOT0-CANON0-SOURCE-BIND0.
//!
//! This is the compiler-owned source provenance boundary.  It accepts one
//! exact canonical preflight plan, validates the continuation before issuing
//! identity, and retains the plan and continuation in one non-Clone package.
//! LOWER0 is the only future consumer allowed to destructure that package.
//!
//! The logical brand is a process-scoped compiler domain plus a
//! compiler-local monotonic ordinal; process-crossing identity is not claimed.

use std::num::NonZeroU64;
use std::sync::atomic::{AtomicU64, Ordering};

use super::acyclic_callable_module_plan::VerifiedAcyclicCallableModulePlanV1;
use super::capability::{
    CanonicalCurrentAPlusPlanV1, CanonicalFirstFamilyPlanV1,
    CanonicalTrivialBindingSsaPlanV1, ResolvedOwnerHeaderFamilyV1,
    ResolvedOwnerHeaderSealErrorV1, VerifiedResolvedOwnerHeaderV1,
};
use super::canonical_drain_manifest::{
    CanonicalDrainIdentityV1, CanonicalDrainManifestErrorV1, CanonicalDrainManifestV1,
    CanonicalDrainRowV1,
};
use super::recursive_callable_module_plan::VerifiedRecursiveCallableModulePlanV1;
use super::resolved_callable_module::VerifiedResolvedCallableModuleV1;
use crate::mir::builder::resolved_lowering::{
    CallableModuleTransactionErrorV1, CanonicalResolvedBuildErrorV1,
    VerifiedUnpublishedCallableDraftSetV1,
};
use crate::mir::builder::{
    BuilderInvocationConfigV1, CanonicalCallableCapabilityWitnessV1,
    CanonicalPhysicalCollectionErrorV1,
    CollectedCanonicalCallablePhysicalV1, CollectedCanonicalSinglePhysicalV1,
    InvocationPhysicalStateV1, MirBuilder, ModuleBuilderInvocationSessionV1,
    ModuleLoweringShellErrorV1, RejectedCanonicalPhysicalCollectionV1,
};
use crate::mir::function::MirFunction;
use crate::mir::module_invocation_identity::{
    ModuleInvocationBrandV1, ModuleInvocationFamilyV1, ModuleInvocationTokenV1,
};
use crate::mir::module_invocation_policy::ModuleInvocationPolicyV1;

static NEXT_COMPILER_DOMAIN: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum CanonicalSourceRouteV1 {
    APlus,
    BindingSsaTrivial,
    BindingSsaAcyclic,
    BindingSsaRecursive,
}

const fn family_for_route(route: CanonicalSourceRouteV1) -> ModuleInvocationFamilyV1 {
    match route {
        CanonicalSourceRouteV1::APlus => ModuleInvocationFamilyV1::CanonicalAPlus,
        CanonicalSourceRouteV1::BindingSsaTrivial => ModuleInvocationFamilyV1::BindingSsaTrivial,
        CanonicalSourceRouteV1::BindingSsaAcyclic => ModuleInvocationFamilyV1::BindingSsaAcyclic,
        CanonicalSourceRouteV1::BindingSsaRecursive => ModuleInvocationFamilyV1::BindingSsaRecursive,
    }
}

fn route_for_family(family: ModuleInvocationFamilyV1) -> CanonicalSourceRouteV1 {
    match family {
        ModuleInvocationFamilyV1::CanonicalAPlus => CanonicalSourceRouteV1::APlus,
        ModuleInvocationFamilyV1::BindingSsaTrivial => CanonicalSourceRouteV1::BindingSsaTrivial,
        ModuleInvocationFamilyV1::BindingSsaAcyclic => CanonicalSourceRouteV1::BindingSsaAcyclic,
        ModuleInvocationFamilyV1::BindingSsaRecursive => CanonicalSourceRouteV1::BindingSsaRecursive,
        ModuleInvocationFamilyV1::Raw => unreachable!("canonical package cannot carry Raw"),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum SourceBindingErrorV1 {
    DomainExhausted,
    OrdinalExhausted,
    Header(ResolvedOwnerHeaderSealErrorV1),
}

impl std::fmt::Display for SourceBindingErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "[freeze:contract][source_binding] {self:?}")
    }
}

impl std::error::Error for SourceBindingErrorV1 {}

/// The four canonical preflight routes.  Raw remains on the closed RAW0
/// chain and is intentionally not rewrapped by SOURCE-BIND0.
#[derive(Debug)]
pub(in crate::mir) enum ExactCanonicalPreflightPlanV1<'a> {
    APlus(CanonicalCurrentAPlusPlanV1<'a>),
    BindingSsaTrivial(CanonicalTrivialBindingSsaPlanV1<'a>),
    BindingSsaAcyclic(VerifiedAcyclicCallableModulePlanV1<'a>),
    BindingSsaRecursive(VerifiedRecursiveCallableModulePlanV1<'a>),
}

impl<'a> ExactCanonicalPreflightPlanV1<'a> {
    pub(crate) fn from_first_family(plan: CanonicalFirstFamilyPlanV1<'a>) -> Self {
        match plan {
            CanonicalFirstFamilyPlanV1::CurrentCanonicalAPlus(plan) => Self::APlus(plan),
            CanonicalFirstFamilyPlanV1::TrivialBindingSsa(plan) => Self::BindingSsaTrivial(plan),
        }
    }

    fn route(&self) -> CanonicalSourceRouteV1 {
        match self {
            Self::APlus(_) => CanonicalSourceRouteV1::APlus,
            Self::BindingSsaTrivial(_) => CanonicalSourceRouteV1::BindingSsaTrivial,
            Self::BindingSsaAcyclic(_) => CanonicalSourceRouteV1::BindingSsaAcyclic,
            Self::BindingSsaRecursive(_) => CanonicalSourceRouteV1::BindingSsaRecursive,
        }
    }
}

#[derive(Debug)]
pub(in crate::mir) enum CanonicalSourceContinuationV1<'a> {
    Single {
        header: VerifiedResolvedOwnerHeaderV1,
        policy: ModuleInvocationPolicyV1,
    },
    Callable {
        source: &'a VerifiedResolvedCallableModuleV1,
        policy: ModuleInvocationPolicyV1,
    },
}

#[derive(Debug)]
pub(in crate::mir) enum CanonicalPlanLoweringErrorV1 {
    Single(CanonicalResolvedBuildErrorV1),
    Callable(CallableModuleTransactionErrorV1),
}

#[derive(Debug)]
pub(in crate::mir) struct RejectedCanonicalLoweringV1<'a> {
    token: ModuleInvocationTokenV1,
    continuation: CanonicalSourceContinuationV1<'a>,
    error: CanonicalPlanLoweringErrorV1,
}

#[derive(Debug)]
pub(in crate::mir) enum LoweredCanonicalPlanV1<'a> {
    Single {
        token: ModuleInvocationTokenV1,
        continuation: CanonicalSourceContinuationV1<'a>,
        draft: MirFunction,
    },
    Callable {
        token: ModuleInvocationTokenV1,
        continuation: CanonicalSourceContinuationV1<'a>,
        drafts: VerifiedUnpublishedCallableDraftSetV1<'a>,
    },
}

impl LoweredCanonicalPlanV1<'_> {
    pub(in crate::mir) fn brand(&self) -> ModuleInvocationBrandV1 {
        match self {
            Self::Single { token, .. } | Self::Callable { token, .. } => token.brand(),
        }
    }
}

#[derive(Debug)]
pub(in crate::mir) struct SourceBoundCanonicalPackageV1<'a> {
    token: ModuleInvocationTokenV1,
    plan: ExactCanonicalPreflightPlanV1<'a>,
    continuation: CanonicalSourceContinuationV1<'a>,
}

/// OWNER0 physical owner opened from one source-bound package.
///
/// The package is consumed only after the real candidate session, shell, and
/// collector have been created.  The plan remains inside this owner until the
/// same session performs the draft-only lowering.
#[derive(Debug)]
pub(in crate::mir) struct CanonicalPhysicalInvocationV1<'a> {
    token: ModuleInvocationTokenV1,
    session: ModuleBuilderInvocationSessionV1,
    physical: InvocationPhysicalStateV1,
    callable_capability: Option<CanonicalCallableCapabilityWitnessV1>,
    plan: ExactCanonicalPreflightPlanV1<'a>,
    continuation: CanonicalSourceContinuationV1<'a>,
}

#[derive(Debug)]
pub(in crate::mir) struct RejectedCanonicalPhysicalOpenV1<'a> {
    package: SourceBoundCanonicalPackageV1<'a>,
    error: CanonicalPhysicalOpenErrorV1,
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir) enum CanonicalPhysicalOpenErrorV1 {
    Shell(ModuleLoweringShellErrorV1),
    Capability(&'static str),
}

impl<'a> RejectedCanonicalPhysicalOpenV1<'a> {
    pub(in crate::mir) fn error(&self) -> &CanonicalPhysicalOpenErrorV1 {
        &self.error
    }
}

#[derive(Debug)]
pub(in crate::mir) struct LoweredCanonicalPhysicalInvocationV1<'a> {
    session: ModuleBuilderInvocationSessionV1,
    physical: InvocationPhysicalStateV1,
    callable_capability: Option<CanonicalCallableCapabilityWitnessV1>,
    lowered: LoweredCanonicalPlanV1<'a>,
}

#[derive(Debug)]
pub(in crate::mir) struct RejectedCanonicalPhysicalLoweringV1<'a> {
    session: ModuleBuilderInvocationSessionV1,
    physical: InvocationPhysicalStateV1,
    callable_capability: Option<CanonicalCallableCapabilityWitnessV1>,
    rejected: RejectedCanonicalLoweringV1<'a>,
}

#[derive(Debug)]
pub(in crate::mir) enum CollectedCanonicalPhysicalInvocationV1<'a> {
    Single {
        token: ModuleInvocationTokenV1,
        continuation: CanonicalSourceContinuationV1<'a>,
        session: ModuleBuilderInvocationSessionV1,
        physical: CollectedCanonicalSinglePhysicalV1,
    },
    Callable {
        token: ModuleInvocationTokenV1,
        continuation: CanonicalSourceContinuationV1<'a>,
        session: ModuleBuilderInvocationSessionV1,
        capability: CanonicalCallableCapabilityWitnessV1,
        physical: CollectedCanonicalCallablePhysicalV1,
    },
}

#[derive(Debug)]
pub(in crate::mir) struct RejectedCanonicalPhysicalCollectionInvocationV1<'a> {
    token: ModuleInvocationTokenV1,
    continuation: CanonicalSourceContinuationV1<'a>,
    session: ModuleBuilderInvocationSessionV1,
    callable_capability: Option<CanonicalCallableCapabilityWitnessV1>,
    physical: RejectedCanonicalPhysicalCollectionV1,
}

impl<'a> CanonicalPhysicalInvocationV1<'a> {
    pub(super) fn lower(
        self,
    ) -> Result<LoweredCanonicalPhysicalInvocationV1<'a>, RejectedCanonicalPhysicalLoweringV1<'a>> {
        let Self {
            token,
            mut session,
            physical,
            callable_capability,
            plan,
            continuation,
        } = self;
        match SourceBoundCanonicalPackageV1::consume_parts(
            token,
            plan,
            continuation,
            session.builder_mut(),
        ) {
            Ok(lowered) => Ok(LoweredCanonicalPhysicalInvocationV1 {
                session,
                physical,
                callable_capability,
                lowered,
            }),
            Err(rejected) => Err(RejectedCanonicalPhysicalLoweringV1 {
                session,
                physical,
                callable_capability,
                rejected,
            }),
        }
    }

    pub(in crate::mir) fn brand(&self) -> ModuleInvocationBrandV1 {
        self.token.brand()
    }
}

impl<'a> LoweredCanonicalPhysicalInvocationV1<'a> {
    pub(in crate::mir) fn brand(&self) -> ModuleInvocationBrandV1 {
        self.lowered.brand()
    }

    pub(in crate::mir) fn session_brand(&self) -> ModuleInvocationBrandV1 {
        self.session.brand()
    }

    pub(in crate::mir) fn physical_brand(&self) -> ModuleInvocationBrandV1 {
        self.physical.brand()
    }

    pub(in crate::mir) fn lowered(&self) -> &LoweredCanonicalPlanV1<'a> {
        &self.lowered
    }

    /// COLLECT0: consume the draft payload in the same physical collector
    /// that was opened before lowering.  The source continuation and session
    /// remain attached to the resulting completion owner.
    pub(in crate::mir) fn collect(
        self,
    ) -> Result<
        CollectedCanonicalPhysicalInvocationV1<'a>,
        RejectedCanonicalPhysicalCollectionInvocationV1<'a>,
    > {
        let Self {
            session,
            physical,
            callable_capability,
            lowered,
        } = self;
        match lowered {
            LoweredCanonicalPlanV1::Single {
                token,
                continuation: CanonicalSourceContinuationV1::Single { header, policy },
                draft,
            } => match physical.collect_single(&header, draft) {
                Ok(physical) => Ok(CollectedCanonicalPhysicalInvocationV1::Single {
                    token,
                    continuation: CanonicalSourceContinuationV1::Single { header, policy },
                    session,
                    physical,
                }),
                Err(rejected) => Err(RejectedCanonicalPhysicalCollectionInvocationV1 {
                    token,
                    continuation: CanonicalSourceContinuationV1::Single { header, policy },
                    session,
                    callable_capability: None,
                    physical: rejected,
                }),
            },
            LoweredCanonicalPlanV1::Callable {
                token,
                continuation: CanonicalSourceContinuationV1::Callable { source, policy },
                drafts,
            } => {
                let Some(capability) = callable_capability else {
                    return Err(RejectedCanonicalPhysicalCollectionInvocationV1 {
                        token,
                        continuation: CanonicalSourceContinuationV1::Callable { source, policy },
                        session,
                        callable_capability: None,
                        physical: physical.reject_capability_missing(),
                    });
                };
                match physical.collect_callable_batch(drafts) {
                Ok(physical) => Ok(CollectedCanonicalPhysicalInvocationV1::Callable {
                    token,
                    continuation: CanonicalSourceContinuationV1::Callable { source, policy },
                    session,
                    capability,
                    physical,
                }),
                Err(rejected) => Err(RejectedCanonicalPhysicalCollectionInvocationV1 {
                    token,
                    continuation: CanonicalSourceContinuationV1::Callable { source, policy },
                    session,
                    callable_capability: Some(capability),
                    physical: rejected,
                }),
                }
            }
            LoweredCanonicalPlanV1::Single { continuation, .. }
            | LoweredCanonicalPlanV1::Callable { continuation, .. } => {
                unreachable!("source-bound plan and continuation family diverged")
            }
        }
    }
}

impl CollectedCanonicalPhysicalInvocationV1<'_> {
    pub(in crate::mir) fn brand(&self) -> ModuleInvocationBrandV1 {
        match self {
            Self::Single { token, .. } | Self::Callable { token, .. } => token.brand(),
        }
    }

    pub(in crate::mir) fn session_brand(&self) -> ModuleInvocationBrandV1 {
        match self {
            Self::Single { session, .. } | Self::Callable { session, .. } => session.brand(),
        }
    }

    pub(in crate::mir) fn physical_brand(&self) -> ModuleInvocationBrandV1 {
        match self {
            Self::Single { physical, .. } => physical.brand(),
            Self::Callable { physical, .. } => physical.brand(),
        }
    }

    pub(in crate::mir) fn receipt_brand(&self) -> ModuleInvocationBrandV1 {
        match self {
            Self::Single { physical, .. } => physical.receipt_brand(),
            Self::Callable { physical, .. } => physical.receipt_brand(),
        }
    }
}

impl RejectedCanonicalPhysicalCollectionInvocationV1<'_> {
    pub(in crate::mir) fn error(&self) -> &CanonicalPhysicalCollectionErrorV1 {
        self.physical.error()
    }
}

impl<'a> SourceBoundCanonicalPackageV1<'a> {
    /// MANIFEST0 projection used by the later completion-owned drain.  The
    /// package supplies its own brand; callers cannot pair a foreign brand or
    /// author concrete rows.
    pub(super) fn project_drain_manifest(
        &self,
    ) -> Result<CanonicalDrainManifestV1, CanonicalDrainManifestErrorV1> {
        let manifest = self
            .continuation
            .project_drain_manifest(self.token.brand())?;
        debug_assert_eq!(manifest.family(), self.token.family());
        Ok(manifest)
    }

    pub(super) fn open_physical(
        self,
        current: &MirBuilder,
        config: BuilderInvocationConfigV1,
        module_name: String,
    ) -> Result<CanonicalPhysicalInvocationV1<'a>, RejectedCanonicalPhysicalOpenV1<'a>> {
        let Self {
            token,
            plan,
            continuation,
        } = self;
        let mut physical = match InvocationPhysicalStateV1::from_token(&token, module_name) {
            Ok(physical) => physical,
            Err(error) => {
                return Err(RejectedCanonicalPhysicalOpenV1 {
                    package: Self {
                        token,
                        plan,
                        continuation,
                    },
                    error: CanonicalPhysicalOpenErrorV1::Shell(error),
                })
            }
        };
        let callable_capability = match token.family() {
            ModuleInvocationFamilyV1::BindingSsaAcyclic
            | ModuleInvocationFamilyV1::BindingSsaRecursive => {
                match physical.install_callable_capability(token.family()) {
                    Ok(witness) => Some(witness),
                    Err(error) => {
                        return Err(RejectedCanonicalPhysicalOpenV1 {
                            package: Self {
                                token,
                                plan,
                                continuation,
                            },
                            error: CanonicalPhysicalOpenErrorV1::Capability(error),
                        })
                    }
                }
            }
            ModuleInvocationFamilyV1::CanonicalAPlus
            | ModuleInvocationFamilyV1::BindingSsaTrivial
            | ModuleInvocationFamilyV1::Raw => None,
        };
        let session = ModuleBuilderInvocationSessionV1::open_for_token(&token, current, config);
        debug_assert_eq!(session.brand(), token.brand());
        debug_assert_eq!(physical.brand(), token.brand());
        Ok(CanonicalPhysicalInvocationV1 {
            token,
            session,
            physical,
            callable_capability,
            plan,
            continuation,
        })
    }
}

impl<'a> CanonicalSourceContinuationV1<'a> {
    /// Project the exact expected physical rows from retained source
    /// authority.  This is the only manifest producer; physical evidence is
    /// intentionally not consulted here.  DRAIN0 consumes this projection
    /// once while consuming the complete invocation.
    pub(in crate::mir) fn project_drain_manifest(
        &self,
        brand: ModuleInvocationBrandV1,
    ) -> Result<CanonicalDrainManifestV1, CanonicalDrainManifestErrorV1> {
        match self {
            Self::Single { header, policy } => Ok(CanonicalDrainManifestV1::single(
                brand,
                *policy,
                CanonicalDrainRowV1::new(
                    CanonicalDrainIdentityV1::ResolvedOwner(header.owner()),
                    header.symbol().as_mir_name().into(),
                    header.arity(),
                ),
            )),
            Self::Callable { source, policy } => {
                let mut rows = Vec::with_capacity(source.functions_by_key().len());
                for key in source.functions_by_key().keys() {
                    let header = source
                        .source()
                        .catalog()
                        .index()
                        .lookup(key)
                        .ok_or_else(|| CanonicalDrainManifestErrorV1::MissingCallableHeader(key.clone()))?;
                    rows.push(CanonicalDrainRowV1::new(
                        CanonicalDrainIdentityV1::Callable(key.clone()),
                        header.symbol().as_mir_name().into(),
                        header.signature().arity(),
                    ));
                }
                Ok(CanonicalDrainManifestV1::callable(
                    brand,
                    *policy,
                    rows,
                ))
            }
        }
    }
}

impl<'a> SourceBoundCanonicalPackageV1<'a> {
    pub(super) fn bind(
        issuer: &mut InvocationIdentityIssuerV1,
        plan: ExactCanonicalPreflightPlanV1<'a>,
    ) -> Result<Self, RejectedCanonicalSourceBindingV1<'a>> {
        let continuation = match Self::seal_continuation(&plan) {
            Ok(continuation) => continuation,
            Err(error) => return Err(RejectedCanonicalSourceBindingV1 { plan, error }),
        };
        let route = plan.route();
        let token = match issuer.issue(route) {
            Ok(token) => token,
            Err(error) => return Err(RejectedCanonicalSourceBindingV1 { plan, error }),
        };
        Ok(Self {
            token,
            plan,
            continuation,
        })
    }

    fn seal_continuation(
        plan: &ExactCanonicalPreflightPlanV1<'a>,
    ) -> Result<CanonicalSourceContinuationV1<'a>, SourceBindingErrorV1> {
        match plan {
            ExactCanonicalPreflightPlanV1::APlus(plan) => {
                let header = plan
                    .seal_resolved_owner_header_v1()
                    .map_err(SourceBindingErrorV1::Header)?;
                debug_assert_eq!(
                    header.family(),
                    ResolvedOwnerHeaderFamilyV1::CurrentCanonicalAPlus
                );
                Ok(CanonicalSourceContinuationV1::Single {
                    header,
                    policy: ModuleInvocationPolicyV1::policy_for_family(
                        ModuleInvocationFamilyV1::CanonicalAPlus,
                    ),
                })
            }
            ExactCanonicalPreflightPlanV1::BindingSsaTrivial(plan) => {
                let header = plan
                    .seal_resolved_owner_header_v1()
                    .map_err(SourceBindingErrorV1::Header)?;
                debug_assert_eq!(header.family(), ResolvedOwnerHeaderFamilyV1::TrivialBindingSsa);
                Ok(CanonicalSourceContinuationV1::Single {
                    header,
                    policy: ModuleInvocationPolicyV1::policy_for_family(
                        ModuleInvocationFamilyV1::BindingSsaTrivial,
                    ),
                })
            }
            ExactCanonicalPreflightPlanV1::BindingSsaAcyclic(plan) => {
                Ok(CanonicalSourceContinuationV1::Callable {
                    source: plan.module(),
                    policy: ModuleInvocationPolicyV1::policy_for_family(
                        ModuleInvocationFamilyV1::BindingSsaAcyclic,
                    ),
                })
            }
            ExactCanonicalPreflightPlanV1::BindingSsaRecursive(plan) => {
                Ok(CanonicalSourceContinuationV1::Callable {
                    source: plan.module(),
                    policy: ModuleInvocationPolicyV1::policy_for_family(
                        ModuleInvocationFamilyV1::BindingSsaRecursive,
                    ),
                })
            }
        }
    }

    pub(crate) fn route(&self) -> CanonicalSourceRouteV1 {
        route_for_family(self.token.family())
    }

    pub(crate) const fn brand(&self) -> ModuleInvocationBrandV1 {
        self.token.brand()
    }

    /// LOWER0's only package consumer.  The source-bound plan is moved into
    /// the existing draft lowerers; no module finalization or publication is
    /// reachable from this terminal.
    pub(super) fn consume(
        self,
        builder: &mut MirBuilder,
    ) -> Result<LoweredCanonicalPlanV1<'a>, RejectedCanonicalLoweringV1<'a>> {
        let Self {
            token,
            plan,
            continuation,
        } = self;
        Self::consume_parts(token, plan, continuation, builder)
    }

    fn consume_parts(
        token: ModuleInvocationTokenV1,
        plan: ExactCanonicalPreflightPlanV1<'a>,
        continuation: CanonicalSourceContinuationV1<'a>,
        builder: &mut MirBuilder,
    ) -> Result<LoweredCanonicalPlanV1<'a>, RejectedCanonicalLoweringV1<'a>> {
        match plan {
            ExactCanonicalPreflightPlanV1::APlus(plan) => {
                match builder.lower_resolved_function_draft(plan) {
                    Ok(draft) => Ok(LoweredCanonicalPlanV1::Single {
                        token,
                        continuation,
                        draft,
                    }),
                    Err(error) => Err(RejectedCanonicalLoweringV1 {
                        token,
                        continuation,
                        error: CanonicalPlanLoweringErrorV1::Single(error),
                    }),
                }
            }
            ExactCanonicalPreflightPlanV1::BindingSsaTrivial(plan) => {
                match builder.lower_resolved_trivial_function_draft(plan) {
                    Ok(draft) => Ok(LoweredCanonicalPlanV1::Single {
                        token,
                        continuation,
                        draft,
                    }),
                    Err(error) => Err(RejectedCanonicalLoweringV1 {
                        token,
                        continuation,
                        error: CanonicalPlanLoweringErrorV1::Single(error),
                    }),
                }
            }
            ExactCanonicalPreflightPlanV1::BindingSsaAcyclic(plan) => {
                match builder.lower_acyclic_callable_drafts(plan) {
                    Ok(drafts) => Ok(LoweredCanonicalPlanV1::Callable {
                        token,
                        continuation,
                        drafts,
                    }),
                    Err(error) => Err(RejectedCanonicalLoweringV1 {
                        token,
                        continuation,
                        error: CanonicalPlanLoweringErrorV1::Callable(error),
                    }),
                }
            }
            ExactCanonicalPreflightPlanV1::BindingSsaRecursive(plan) => {
                match builder.lower_recursive_callable_drafts(plan) {
                    Ok(drafts) => Ok(LoweredCanonicalPlanV1::Callable {
                        token,
                        continuation,
                        drafts,
                    }),
                    Err(error) => Err(RejectedCanonicalLoweringV1 {
                        token,
                        continuation,
                        error: CanonicalPlanLoweringErrorV1::Callable(error),
                    }),
                }
            }
        }
    }

    #[cfg(test)]
    fn has_plan_and_continuation(&self) -> bool {
        match (&self.plan, &self.continuation) {
            (
                ExactCanonicalPreflightPlanV1::APlus(_)
                | ExactCanonicalPreflightPlanV1::BindingSsaTrivial(_),
                CanonicalSourceContinuationV1::Single { .. },
            )
            | (
                ExactCanonicalPreflightPlanV1::BindingSsaAcyclic(_)
                | ExactCanonicalPreflightPlanV1::BindingSsaRecursive(_),
                CanonicalSourceContinuationV1::Callable { .. },
            ) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub(in crate::mir) struct RejectedCanonicalSourceBindingV1<'a> {
    plan: ExactCanonicalPreflightPlanV1<'a>,
    error: SourceBindingErrorV1,
}

impl<'a> RejectedCanonicalSourceBindingV1<'a> {
    pub(crate) const fn error(&self) -> &SourceBindingErrorV1 {
        &self.error
    }

    #[cfg(test)]
    fn plan(self) -> ExactCanonicalPreflightPlanV1<'a> {
        self.plan
    }
}

#[derive(Debug)]
pub(super) struct InvocationIdentityIssuerV1 {
    domain: Option<NonZeroU64>,
    next_ordinal: u64,
}

impl InvocationIdentityIssuerV1 {
    pub(super) const fn new() -> Self {
        Self {
            domain: None,
            next_ordinal: 1,
        }
    }

    fn issue(
        &mut self,
        route: CanonicalSourceRouteV1,
    ) -> Result<ModuleInvocationTokenV1, SourceBindingErrorV1> {
        self.issue_family(family_for_route(route))
    }

    /// RAW-SOURCE0-BIND0: Raw may mint only after its source continuation has
    /// been sealed by the compiler-owned binding terminal.  Callers cannot
    /// select a generic family and this method is not a public issuer API.
    pub(super) fn issue_raw(&mut self) -> Result<ModuleInvocationTokenV1, SourceBindingErrorV1> {
        self.issue_family(ModuleInvocationFamilyV1::Raw)
    }

    fn issue_family(
        &mut self,
        family: ModuleInvocationFamilyV1,
    ) -> Result<ModuleInvocationTokenV1, SourceBindingErrorV1> {
        let domain = match self.domain {
            Some(domain) => domain,
            None => {
                let raw = NEXT_COMPILER_DOMAIN
                    .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |value| {
                        value.checked_add(1)
                    })
                    .map_err(|_| SourceBindingErrorV1::DomainExhausted)?;
                let domain = NonZeroU64::new(raw).ok_or(SourceBindingErrorV1::DomainExhausted)?;
                self.domain = Some(domain);
                domain
            }
        };
        let ordinal = NonZeroU64::new(self.next_ordinal)
            .ok_or(SourceBindingErrorV1::OrdinalExhausted)?;
        self.next_ordinal = self
            .next_ordinal
            .checked_add(1)
            .ok_or(SourceBindingErrorV1::OrdinalExhausted)?;
        Ok(ModuleInvocationTokenV1::from_issued(
            domain,
            ordinal,
            family,
        ))
    }
}
