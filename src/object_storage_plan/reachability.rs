use super::fastpath::LocalFastPathKind;
use super::ids::{LocalFastPathSiteId, RoutePlanId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FastPathReachability {
    pub site_id: LocalFastPathSiteId,
    pub fact_kind: LocalFastPathKind,
    pub reachable_in_active_route: bool,
    pub selected_route: Option<RoutePlanId>,
    pub preempted_by: Option<RoutePlanId>,
}

impl FastPathReachability {
    pub const fn selected(
        site_id: LocalFastPathSiteId,
        fact_kind: LocalFastPathKind,
        selected_route: RoutePlanId,
    ) -> Self {
        Self {
            site_id,
            fact_kind,
            reachable_in_active_route: true,
            selected_route: Some(selected_route),
            preempted_by: None,
        }
    }

    pub const fn preempted(
        site_id: LocalFastPathSiteId,
        fact_kind: LocalFastPathKind,
        selected_route: RoutePlanId,
        preempted_by: RoutePlanId,
    ) -> Self {
        Self {
            site_id,
            fact_kind,
            reachable_in_active_route: false,
            selected_route: Some(selected_route),
            preempted_by: Some(preempted_by),
        }
    }

    pub const fn unreachable(site_id: LocalFastPathSiteId, fact_kind: LocalFastPathKind) -> Self {
        Self {
            site_id,
            fact_kind,
            reachable_in_active_route: false,
            selected_route: None,
            preempted_by: None,
        }
    }

    #[inline]
    pub const fn winner_claim_allowed(self) -> bool {
        self.reachable_in_active_route
    }
}
