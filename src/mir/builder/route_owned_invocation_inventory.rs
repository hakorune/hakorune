//! HEADERPORT0-WIRING-I0-ROUTEINV-S0: route-owned drain policy.
//!
//! The existing invocation route matrix remains the sole route-identity
//! authority. This disconnected product projects each root family to exactly
//! one drain-policy lane without carrying a Builder, collector, module,
//! function draft, caller-authored symbol list, retry, or fallback authority.
//!
//! Exact function identities are still supplied later by the selected source
//! authority: raw expansion receipts, one canonical resolved owner, or the
//! canonical callable catalog. This S0 product only seals which authority is
//! allowed to supply that inventory and which root/condition law applies.

use super::module_invocation_route_matrix::{
    InvocationRootFamilyV1, InvocationRouteMatrixRowV1, InvocationRouteMatrixV1,
};
pub(in crate::mir::builder) use crate::mir::module_invocation_policy::{
    InvocationConditionPolicyV1 as RouteConditionPolicyV2,
    InvocationFallbackPolicyV1 as RouteFallbackPolicyV2,
    InvocationInventoryAuthorityV1 as InvocationInventoryAuthorityV2,
    InvocationRootPolicyV1 as InvocationRootPolicyV2,
    ModuleInvocationPolicyV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum StaticRouteReachabilityV2 {
    Reachable,
    Unreachable,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) struct ExactInvocationSourceSymbolsV2 {
    ingress: &'static str,
    lowering_root: &'static str,
}

impl ExactInvocationSourceSymbolsV2 {
    pub(in crate::mir::builder) const fn ingress(self) -> &'static str {
        self.ingress
    }

    pub(in crate::mir::builder) const fn lowering_root(self) -> &'static str {
        self.lowering_root
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum RouteOwnedInventorySealErrorV2 {
    RouteMatrixFamilyMissing {
        family: InvocationRootFamilyV1,
    },
    IngressNotReachable {
        family: InvocationRootFamilyV1,
        state: StaticRouteReachabilityV2,
    },
    LoweringRootNotReachable {
        family: InvocationRootFamilyV1,
        state: StaticRouteReachabilityV2,
    },
}

impl std::fmt::Display for RouteOwnedInventorySealErrorV2 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "[freeze:contract][route_inventory] {self:?}")
    }
}

impl std::error::Error for RouteOwnedInventorySealErrorV2 {}

#[derive(Debug)]
pub(in crate::mir::builder) enum RouteOwnedInvocationInventoryV2 {
    Raw(RouteOwnedInventoryPolicyV2),
    CanonicalSingle(RouteOwnedInventoryPolicyV2),
    BindingSsaAcyclic(RouteOwnedInventoryPolicyV2),
    BindingSsaRecursive(RouteOwnedInventoryPolicyV2),
}

#[derive(Debug)]
pub(in crate::mir::builder) struct RouteOwnedInventoryPolicyV2 {
    matrix_rows: Box<[InvocationRouteMatrixRowV1]>,
    source_symbols: ExactInvocationSourceSymbolsV2,
    policy: ModuleInvocationPolicyV1,
    _seal: RouteOwnedInventoryPolicySealV2,
}

#[derive(Debug)]
struct RouteOwnedInventoryPolicySealV2;

impl RouteOwnedInvocationInventoryV2 {
    pub(in crate::mir::builder) fn derive(
        family: InvocationRootFamilyV1,
    ) -> Result<Self, RouteOwnedInventorySealErrorV2> {
        let (source_symbols, policy) = route_policy(family);
        let policy = seal_policy(
            family,
            source_symbols,
            StaticRouteReachabilityV2::Reachable,
            StaticRouteReachabilityV2::Reachable,
            policy,
        )?;
        Ok(match family {
            InvocationRootFamilyV1::Raw => Self::Raw(policy),
            InvocationRootFamilyV1::CanonicalAPlus | InvocationRootFamilyV1::BindingSsaTrivial => {
                Self::CanonicalSingle(policy)
            }
            InvocationRootFamilyV1::BindingSsaAcyclic => Self::BindingSsaAcyclic(policy),
            InvocationRootFamilyV1::BindingSsaRecursive => Self::BindingSsaRecursive(policy),
        })
    }

    pub(in crate::mir::builder) const fn policy(&self) -> &RouteOwnedInventoryPolicyV2 {
        match self {
            Self::Raw(policy)
            | Self::CanonicalSingle(policy)
            | Self::BindingSsaAcyclic(policy)
            | Self::BindingSsaRecursive(policy) => policy,
        }
    }
}

impl RouteOwnedInventoryPolicyV2 {
    pub(in crate::mir::builder) const fn family(&self) -> InvocationRootFamilyV1 {
        self.policy.family()
    }

    pub(in crate::mir::builder) fn matrix_rows(&self) -> &[InvocationRouteMatrixRowV1] {
        &self.matrix_rows
    }

    pub(in crate::mir::builder) const fn source_symbols(&self) -> ExactInvocationSourceSymbolsV2 {
        self.source_symbols
    }

    pub(in crate::mir::builder) const fn inventory_authority(
        &self,
    ) -> InvocationInventoryAuthorityV2 {
        self.policy.inventory_authority()
    }

    pub(in crate::mir::builder) const fn root_policy(&self) -> InvocationRootPolicyV2 {
        self.policy.root_policy()
    }

    pub(in crate::mir::builder) const fn condition_policy(&self) -> RouteConditionPolicyV2 {
        self.policy.condition_policy()
    }

    pub(in crate::mir::builder) const fn fallback(&self) -> RouteFallbackPolicyV2 {
        self.policy.fallback_policy()
    }
}

fn seal_policy(
    family: InvocationRootFamilyV1,
    source_symbols: ExactInvocationSourceSymbolsV2,
    ingress_reachability: StaticRouteReachabilityV2,
    root_reachability: StaticRouteReachabilityV2,
    policy: ModuleInvocationPolicyV1,
) -> Result<RouteOwnedInventoryPolicyV2, RouteOwnedInventorySealErrorV2> {
    if ingress_reachability != StaticRouteReachabilityV2::Reachable {
        return Err(RouteOwnedInventorySealErrorV2::IngressNotReachable {
            family,
            state: ingress_reachability,
        });
    }
    if root_reachability != StaticRouteReachabilityV2::Reachable {
        return Err(RouteOwnedInventorySealErrorV2::LoweringRootNotReachable {
            family,
            state: root_reachability,
        });
    }
    let matrix_rows = InvocationRouteMatrixV1::rows()
        .iter()
        .copied()
        .filter(|row| row.family() == family)
        .collect::<Vec<_>>();
    if matrix_rows.is_empty() {
        return Err(RouteOwnedInventorySealErrorV2::RouteMatrixFamilyMissing { family });
    }
    Ok(RouteOwnedInventoryPolicyV2 {
        matrix_rows: matrix_rows.into_boxed_slice(),
        source_symbols,
        policy,
        _seal: RouteOwnedInventoryPolicySealV2,
    })
}

fn route_policy(
    family: InvocationRootFamilyV1,
) -> (ExactInvocationSourceSymbolsV2, ModuleInvocationPolicyV1) {
    match family {
        InvocationRootFamilyV1::Raw => (
            ExactInvocationSourceSymbolsV2 {
                ingress: "MirCompiler::compile_legacy_request",
                lowering_root: "MirBuilder::build_module",
            },
            ModuleInvocationPolicyV1::policy_for_family(family),
        ),
        InvocationRootFamilyV1::CanonicalAPlus => (
            ExactInvocationSourceSymbolsV2 {
                ingress: "MirCompiler::compile_resolved_first_family",
                lowering_root: "MirBuilder::build_resolved_function_module",
            },
            ModuleInvocationPolicyV1::policy_for_family(family),
        ),
        InvocationRootFamilyV1::BindingSsaTrivial => (
            ExactInvocationSourceSymbolsV2 {
                ingress: "MirCompiler::compile_resolved_first_family",
                lowering_root: "MirBuilder::build_resolved_trivial_function_module",
            },
            ModuleInvocationPolicyV1::policy_for_family(family),
        ),
        InvocationRootFamilyV1::BindingSsaAcyclic => (
            ExactInvocationSourceSymbolsV2 {
                ingress: "MirCompiler::compile_resolved_callable_module",
                lowering_root: "MirBuilder::build_acyclic_callable_module_candidate",
            },
            ModuleInvocationPolicyV1::policy_for_family(family),
        ),
        InvocationRootFamilyV1::BindingSsaRecursive => (
            ExactInvocationSourceSymbolsV2 {
                ingress: "MirCompiler::compile_resolved_recursive_callable_module",
                lowering_root: "MirBuilder::build_recursive_callable_module_candidate",
            },
            ModuleInvocationPolicyV1::policy_for_family(family),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn route_matrix_projects_to_four_policy_lanes_without_merging_families() {
        let raw = RouteOwnedInvocationInventoryV2::derive(InvocationRootFamilyV1::Raw).unwrap();
        assert!(matches!(raw, RouteOwnedInvocationInventoryV2::Raw(_)));
        assert_eq!(raw.policy().matrix_rows().len(), 4);

        for family in [
            InvocationRootFamilyV1::CanonicalAPlus,
            InvocationRootFamilyV1::BindingSsaTrivial,
        ] {
            let single = RouteOwnedInvocationInventoryV2::derive(family).unwrap();
            assert!(matches!(
                single,
                RouteOwnedInvocationInventoryV2::CanonicalSingle(_)
            ));
            assert_eq!(single.policy().family(), family);
        }

        assert!(matches!(
            RouteOwnedInvocationInventoryV2::derive(InvocationRootFamilyV1::BindingSsaAcyclic)
                .unwrap(),
            RouteOwnedInvocationInventoryV2::BindingSsaAcyclic(_)
        ));
        assert!(matches!(
            RouteOwnedInvocationInventoryV2::derive(InvocationRootFamilyV1::BindingSsaRecursive)
                .unwrap(),
            RouteOwnedInvocationInventoryV2::BindingSsaRecursive(_)
        ));
    }

    #[test]
    fn raw_and_canonical_policies_keep_distinct_root_and_condition_laws() {
        let raw = RouteOwnedInvocationInventoryV2::derive(InvocationRootFamilyV1::Raw).unwrap();
        assert_eq!(
            raw.policy().root_policy(),
            InvocationRootPolicyV2::RequiredMain
        );
        assert_eq!(
            raw.policy().condition_policy(),
            RouteConditionPolicyV2::RawSourceSelected
        );
        assert_eq!(
            raw.policy().inventory_authority(),
            InvocationInventoryAuthorityV2::RawExpansionReceipts
        );

        for family in [
            InvocationRootFamilyV1::CanonicalAPlus,
            InvocationRootFamilyV1::BindingSsaTrivial,
            InvocationRootFamilyV1::BindingSsaAcyclic,
            InvocationRootFamilyV1::BindingSsaRecursive,
        ] {
            let canonical = RouteOwnedInvocationInventoryV2::derive(family).unwrap();
            assert_ne!(
                canonical.policy().root_policy(),
                InvocationRootPolicyV2::RequiredMain
            );
            assert_eq!(
                canonical.policy().condition_policy(),
                RouteConditionPolicyV2::Forbidden
            );
            assert_eq!(
                canonical.policy().fallback(),
                RouteFallbackPolicyV2::Forbidden
            );
        }
    }

    #[test]
    fn exact_ingress_and_lowering_root_symbols_are_sealed_per_family() {
        let raw = RouteOwnedInvocationInventoryV2::derive(InvocationRootFamilyV1::Raw).unwrap();
        assert_eq!(
            raw.policy().source_symbols().ingress(),
            "MirCompiler::compile_legacy_request"
        );
        assert_eq!(
            raw.policy().source_symbols().lowering_root(),
            "MirBuilder::build_module"
        );

        let recursive =
            RouteOwnedInvocationInventoryV2::derive(InvocationRootFamilyV1::BindingSsaRecursive)
                .unwrap();
        assert_eq!(
            recursive.policy().source_symbols().ingress(),
            "MirCompiler::compile_resolved_recursive_callable_module"
        );
        assert_eq!(
            recursive.policy().source_symbols().lowering_root(),
            "MirBuilder::build_recursive_callable_module_candidate"
        );
    }

    #[test]
    fn unknown_or_unreachable_source_topology_cannot_issue_a_policy() {
        let symbols = route_policy(InvocationRootFamilyV1::Raw).0;
        let error = seal_policy(
            InvocationRootFamilyV1::Raw,
            symbols,
            StaticRouteReachabilityV2::Unknown,
            StaticRouteReachabilityV2::Reachable,
            ModuleInvocationPolicyV1::policy_for_family(InvocationRootFamilyV1::Raw),
        )
        .unwrap_err();
        assert!(matches!(
            error,
            RouteOwnedInventorySealErrorV2::IngressNotReachable {
                state: StaticRouteReachabilityV2::Unknown,
                ..
            }
        ));

        let error = seal_policy(
            InvocationRootFamilyV1::Raw,
            symbols,
            StaticRouteReachabilityV2::Reachable,
            StaticRouteReachabilityV2::Unreachable,
            ModuleInvocationPolicyV1::policy_for_family(InvocationRootFamilyV1::Raw),
        )
        .unwrap_err();
        assert!(matches!(
            error,
            RouteOwnedInventorySealErrorV2::LoweringRootNotReachable {
                state: StaticRouteReachabilityV2::Unreachable,
                ..
            }
        ));
    }
}
