use super::fastpath::{LocalFastPathFact, LocalFastPathFallbackReason};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlanEpoch(pub u32);

impl PlanEpoch {
    pub const INITIAL: Self = Self(0);

    #[inline]
    pub const fn is_initial(self) -> bool {
        self.0 == Self::INITIAL.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FastPathDecision {
    Allow(LocalFastPathFact),
    Deny(LocalFastPathFallbackReason),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FastPathDenyOwner {
    pub owner_lane: &'static str,
    pub next_action: &'static str,
}

impl FastPathDecision {
    #[inline]
    pub const fn allow(fact: LocalFastPathFact) -> Self {
        Self::Allow(fact)
    }

    #[inline]
    pub const fn deny(reason: LocalFastPathFallbackReason) -> Self {
        Self::Deny(reason)
    }

    #[inline]
    pub const fn is_allow(&self) -> bool {
        matches!(self, Self::Allow(_))
    }

    #[inline]
    pub const fn is_deny(&self) -> bool {
        matches!(self, Self::Deny(_))
    }

    #[inline]
    pub const fn fact(&self) -> Option<&LocalFastPathFact> {
        match self {
            Self::Allow(fact) => Some(fact),
            Self::Deny(_) => None,
        }
    }

    #[inline]
    pub const fn deny_reason(&self) -> Option<LocalFastPathFallbackReason> {
        match self {
            Self::Allow(_) => None,
            Self::Deny(reason) => Some(*reason),
        }
    }
}

impl LocalFastPathFallbackReason {
    pub const fn owner_mapping(self) -> FastPathDenyOwner {
        match self {
            Self::OpenWorld => FastPathDenyOwner {
                owner_lane: "route_open_world_boundary",
                next_action: "pin closed-world scope or keep generic route",
            },
            Self::UnknownValue => FastPathDenyOwner {
                owner_lane: "value_origin_inventory",
                next_action: "record value origin before fastpath eligibility",
            },
            Self::AliasUnknown => FastPathDenyOwner {
                owner_lane: "alias_classifier",
                next_action: "add alias observation before resolver",
            },
            Self::PublishedBeforeSite => FastPathDenyOwner {
                owner_lane: "publication_classifier",
                next_action: "inspect publication site before callsite",
            },
            Self::MaybePublishedBeforeSite => FastPathDenyOwner {
                owner_lane: "publication_classifier_or_phi_freshness",
                next_action: "split maybe-published PHI/path state or fallback",
            },
            Self::RoutePlanMissing => FastPathDenyOwner {
                owner_lane: "route_proof_producer",
                next_action: "produce RoutePlan proof before eligibility",
            },
            Self::DynamicRoute => FastPathDenyOwner {
                owner_lane: "routeplan_boxcallable_registry",
                next_action: "produce direct RoutePlan proof or keep dynamic route",
            },
            Self::ObjectPlanMissing => FastPathDenyOwner {
                owner_lane: "objectplan_producer",
                next_action: "produce ObjectPlan before eligibility",
            },
            Self::GenericStorage => FastPathDenyOwner {
                owner_lane: "object_storage_plan_producer",
                next_action: "produce local storage proof or keep generic storage",
            },
            Self::BackendMissing => FastPathDenyOwner {
                owner_lane: "backend_consumer_seam",
                next_action: "add fact-driven backend consumer after shadow",
            },
            Self::CycleDetected => FastPathDenyOwner {
                owner_lane: "recursive_dependency_inventory",
                next_action: "break alias/dependency cycle or keep fallback",
            },
            Self::PhiMergeNotProven => FastPathDenyOwner {
                owner_lane: "phi_lifecycle_alias_freshness",
                next_action: "prove same alias/publication state across PHI",
            },
            Self::LoopCarriedNotProven => FastPathDenyOwner {
                owner_lane: "loop_carried_proof_lane",
                next_action: "prove invariant loop-carried value or fallback",
            },
            Self::InterprocSummaryMissing => FastPathDenyOwner {
                owner_lane: "call_summary_lane",
                next_action: "add same-module call summary or keep fallback",
            },
            Self::UnknownCall => FastPathDenyOwner {
                owner_lane: "call_summary_lane",
                next_action: "add local call summary or deny open-world call",
            },
        }
    }
}
