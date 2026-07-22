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
use super::recursive_callable_module_plan::VerifiedRecursiveCallableModulePlanV1;
use super::resolved_callable_module::VerifiedResolvedCallableModuleV1;
use crate::mir::builder::resolved_lowering::{
    CallableModuleTransactionErrorV1, CanonicalResolvedBuildErrorV1,
    VerifiedUnpublishedCallableDraftSetV1,
};
use crate::mir::builder::MirBuilder;
use crate::mir::function::MirFunction;
use crate::mir::module_invocation_identity::{
    ModuleInvocationBrandV1, ModuleInvocationFamilyV1, ModuleInvocationTokenV1,
};

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

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CanonicalRoutePolicyV1 {
    ExactOwner,
    ExactCallableCatalog,
}

#[derive(Debug)]
pub(in crate::mir) enum CanonicalSourceContinuationV1<'a> {
    Single {
        header: VerifiedResolvedOwnerHeaderV1,
        policy: CanonicalRoutePolicyV1,
    },
    Callable {
        source: &'a VerifiedResolvedCallableModuleV1,
        policy: CanonicalRoutePolicyV1,
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

#[derive(Debug)]
pub(in crate::mir) struct SourceBoundCanonicalPackageV1<'a> {
    token: ModuleInvocationTokenV1,
    plan: ExactCanonicalPreflightPlanV1<'a>,
    continuation: CanonicalSourceContinuationV1<'a>,
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
                    policy: CanonicalRoutePolicyV1::ExactOwner,
                })
            }
            ExactCanonicalPreflightPlanV1::BindingSsaTrivial(plan) => {
                let header = plan
                    .seal_resolved_owner_header_v1()
                    .map_err(SourceBindingErrorV1::Header)?;
                debug_assert_eq!(header.family(), ResolvedOwnerHeaderFamilyV1::TrivialBindingSsa);
                Ok(CanonicalSourceContinuationV1::Single {
                    header,
                    policy: CanonicalRoutePolicyV1::ExactOwner,
                })
            }
            ExactCanonicalPreflightPlanV1::BindingSsaAcyclic(plan) => {
                Ok(CanonicalSourceContinuationV1::Callable {
                    source: plan.module(),
                    policy: CanonicalRoutePolicyV1::ExactCallableCatalog,
                })
            }
            ExactCanonicalPreflightPlanV1::BindingSsaRecursive(plan) => {
                Ok(CanonicalSourceContinuationV1::Callable {
                    source: plan.module(),
                    policy: CanonicalRoutePolicyV1::ExactCallableCatalog,
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
            family_for_route(route),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};

    fn literal(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    fn function(name: &str) -> ASTNode {
        ASTNode::FunctionDeclaration {
            name: name.into(),
            params: Vec::new(),
            param_decls: Vec::new(),
            return_type_name: None,
            body: vec![ASTNode::Return {
                value: Some(Box::new(literal(1))),
                span: Span::unknown(),
            }],
            uses: Vec::new(),
            contracts: Vec::new(),
            is_static: true,
            is_override: false,
            attrs: DeclarationAttrs::default(),
            span: Span::unknown(),
        }
    }

    #[test]
    fn issuer_is_monotonic_and_does_not_reuse_dropped_ordinals() {
        let mut issuer = InvocationIdentityIssuerV1::new();
        let first = issuer.issue(CanonicalSourceRouteV1::APlus).unwrap();
        let second = issuer.issue(CanonicalSourceRouteV1::APlus).unwrap();
        assert_eq!(first.brand().ordinal(), 1);
        assert_eq!(second.brand().ordinal(), 2);
        assert!(!first.brand().same(second.brand()));
    }

    #[test]
    fn separate_compiler_domains_do_not_equate_local_ordinals() {
        let mut first = InvocationIdentityIssuerV1::new();
        let mut second = InvocationIdentityIssuerV1::new();
        let first_token = first.issue(CanonicalSourceRouteV1::APlus).unwrap();
        let second_token = second.issue(CanonicalSourceRouteV1::APlus).unwrap();
        assert_eq!(first_token.brand().ordinal(), second_token.brand().ordinal());
        assert!(!first_token.brand().same(second_token.brand()));
    }

    #[test]
    fn package_binds_exact_plan_before_issuing_identity() {
        let unit = super::super::VerifiedResolvedSourceUnitV1::resolve_function(function("bound"))
            .unwrap();
        let plan = super::super::CanonicalLoweringPreflightV1::verify(&unit).unwrap();
        let exact = ExactCanonicalPreflightPlanV1::from_first_family(plan);
        let mut compiler = super::super::MirCompiler::new();
        let package = compiler.bind_canonical_source(exact).unwrap();
        assert_eq!(package.route(), CanonicalSourceRouteV1::BindingSsaTrivial);
        assert!(package.has_plan_and_continuation());
    }

    #[test]
    fn issuer_failure_returns_the_exact_plan_owner() {
        let unit = super::super::VerifiedResolvedSourceUnitV1::resolve_function(function("bound"))
            .unwrap();
        let plan = super::super::CanonicalLoweringPreflightV1::verify(&unit).unwrap();
        let exact = ExactCanonicalPreflightPlanV1::from_first_family(plan);
        let mut issuer = InvocationIdentityIssuerV1::new();
        issuer.next_ordinal = u64::MAX;
        let rejected = SourceBoundCanonicalPackageV1::bind(&mut issuer, exact).unwrap_err();
        assert_eq!(rejected.error(), &SourceBindingErrorV1::OrdinalExhausted);
        assert!(matches!(
            rejected.plan(),
            ExactCanonicalPreflightPlanV1::BindingSsaTrivial(_)
        ));
    }

    #[test]
    fn package_consume_moves_trivial_plan_into_a_draft_lowerer() {
        let unit = super::super::VerifiedResolvedSourceUnitV1::resolve_function(function("lowered"))
            .unwrap();
        let plan = super::super::CanonicalLoweringPreflightV1::verify(&unit).unwrap();
        let exact = ExactCanonicalPreflightPlanV1::from_first_family(plan);
        let mut issuer = InvocationIdentityIssuerV1::new();
        let package = SourceBoundCanonicalPackageV1::bind(&mut issuer, exact).unwrap();
        let mut builder = MirBuilder::new();
        let lowered = package.consume(&mut builder).unwrap();
        assert!(matches!(lowered, LoweredCanonicalPlanV1::Single { .. }));
    }

    #[test]
    fn compiler_lower_terminal_keeps_live_builder_outside_candidate_session() {
        let unit = super::super::VerifiedResolvedSourceUnitV1::resolve_function(function("session"))
            .unwrap();
        let plan = super::super::CanonicalLoweringPreflightV1::verify(&unit).unwrap();
        let exact = ExactCanonicalPreflightPlanV1::from_first_family(plan);
        let mut compiler = super::super::MirCompiler::new();
        let package = compiler.bind_canonical_source(exact).unwrap();
        let candidate = compiler.lower_canonical_source(package, Some("lower0.hako")).unwrap();
        assert!(matches!(candidate.lowered, LoweredCanonicalPlanV1::Single { .. }));
        assert!(compiler.builder.current_module.is_none());
    }
}
