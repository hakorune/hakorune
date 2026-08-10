//! Logical two-site Completion projection for the bounded Dynamic callable.
//!
//! The existing `VerifiedFunctionCompletionV1` remains the source-side owner
//! of return coverage and result classification. This child consumes the
//! carrier cleanup projection and seals only the relation from the exact inner
//! Recipe Return and outer Callable Tail to one logical function-exit target.
//! It does not write a Return, create merge/ABI facts, or invoke the final
//! function-seal stage.

use crate::mir::resolved_semantics::{FunctionOwnerIdV1, RegionId, SourceStmtSiteV1};

use super::carrier_rebind::VerifiedDynamicCarrierCleanupProjectionV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum DynamicCallableCompletionProjectionRejectV1 {
    CleanupPartition,
    CompletionCoverage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DynamicCallableReturnKindV1 {
    Value,
    Unit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DynamicCallableFunctionExitTargetV1 {
    owner: FunctionOwnerIdV1,
    target: RegionId,
    result: DynamicCallableReturnKindV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum DynamicCallableCompletionRouteV1 {
    InnerRecipeReturn {
        site: SourceStmtSiteV1,
        target: DynamicCallableFunctionExitTargetV1,
    },
    OuterCallableTail {
        site: SourceStmtSiteV1,
        target: DynamicCallableFunctionExitTargetV1,
    },
}

const COMPLETION_ROUTE_COUNT_V1: usize = 2;

#[derive(Debug)]
pub(in crate::mir) struct VerifiedDynamicCallableCompletionProjectionV1 {
    cleanup: VerifiedDynamicCarrierCleanupProjectionV1,
    routes: [DynamicCallableCompletionRouteV1; COMPLETION_ROUTE_COUNT_V1],
    target: DynamicCallableFunctionExitTargetV1,
}

impl VerifiedDynamicCallableCompletionProjectionV1 {
    #[cfg(test)]
    pub(in crate::mir) fn current(&self) -> super::DynamicCarrierCurrentDispositionV1 {
        self.cleanup.current()
    }

    #[cfg(test)]
    pub(in crate::mir) fn routes(&self) -> &[DynamicCallableCompletionRouteV1; 2] {
        &self.routes
    }

    #[cfg(test)]
    pub(in crate::mir) fn target(&self) -> DynamicCallableFunctionExitTargetV1 {
        self.target
    }

    #[cfg(test)]
    pub(in crate::mir) fn cleanup(&self) -> &VerifiedDynamicCarrierCleanupProjectionV1 {
        &self.cleanup
    }
}

pub(in crate::mir) fn issue_dynamic_callable_completion_projection_i0(
    cleanup: VerifiedDynamicCarrierCleanupProjectionV1,
) -> Result<
    VerifiedDynamicCallableCompletionProjectionV1,
    DynamicCallableCompletionProjectionRejectV1,
> {
    let sites = cleanup.completion_sites();
    if sites[0] == sites[1] {
        return Err(DynamicCallableCompletionProjectionRejectV1::CompletionCoverage);
    }
    let Some((owner, target, returns_value)) = cleanup.completion_summary() else {
        return Err(DynamicCallableCompletionProjectionRejectV1::CleanupPartition);
    };
    let result = if returns_value {
        DynamicCallableReturnKindV1::Value
    } else {
        DynamicCallableReturnKindV1::Unit
    };
    let target = DynamicCallableFunctionExitTargetV1 {
        owner,
        target,
        result,
    };
    let routes = [
        DynamicCallableCompletionRouteV1::InnerRecipeReturn {
            site: sites[0].clone(),
            target,
        },
        DynamicCallableCompletionRouteV1::OuterCallableTail {
            site: sites[1].clone(),
            target,
        },
    ];
    Ok(VerifiedDynamicCallableCompletionProjectionV1 {
        cleanup,
        routes,
        target,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::compiler::dynamic_full_body_recipe::coseal::tests::fixture;
    use crate::mir::compiler::dynamic_full_body_recipe::coseal::{
        issue_dynamic_carrier_cleanup_projection_i0, issue_dynamic_carrier_flow_program_v1,
        issue_dynamic_carrier_ingress_lifecycle_program_v1,
        issue_dynamic_carrier_rebind_transaction_program_v1,
        issue_dynamic_full_loop_semantic_program_v2,
        issue_dynamic_full_loop_source_recipe_envelope_v2,
        issue_dynamic_invocation_carrier_lifecycle_program_v1,
        issue_dynamic_operator_carrier_lifecycle_program_v1,
    };
    use crate::mir::compiler::dynamic_full_body_source::DynamicFullBodyBindingRoleV1;
    use crate::mir::resolved_semantics::HomeDemandV1;

    fn exact_projection() -> VerifiedDynamicCallableCompletionProjectionV1 {
        let fixture = fixture(true);
        let parameter_binding = fixture
            .candidate
            .source
            .bindings
            .iter()
            .find(|row| row.role() == DynamicFullBodyBindingRoleV1::Pos)
            .expect("pos binding")
            .binding();
        let envelope =
            issue_dynamic_full_loop_source_recipe_envelope_v2(fixture.candidate, &fixture.calls)
                .expect("source/Recipe envelope");
        let semantic =
            issue_dynamic_full_loop_semantic_program_v2(envelope).expect("semantic program");
        let invocation = issue_dynamic_invocation_carrier_lifecycle_program_v1(semantic)
            .expect("invocation lifecycle");
        let operator = issue_dynamic_operator_carrier_lifecycle_program_v1(invocation)
            .expect("operator lifecycle");
        let ingress = issue_dynamic_carrier_ingress_lifecycle_program_v1(
            operator,
            1,
            parameter_binding,
            HomeDemandV1::Handle,
        )
        .expect("ingress lifecycle");
        let rebind =
            issue_dynamic_carrier_rebind_transaction_program_v1(ingress).expect("rebind relation");
        let flow = issue_dynamic_carrier_flow_program_v1(rebind).expect("carrier flow");
        let cleanup =
            issue_dynamic_carrier_cleanup_projection_i0(flow).expect("cleanup projection");
        issue_dynamic_callable_completion_projection_i0(cleanup).expect("completion projection")
    }

    #[test]
    fn exact_projection_seals_two_routes_to_one_function_exit() {
        let projection = exact_projection();
        assert_eq!(projection.routes.len(), 2);
        assert_eq!(projection.routes[0].target(), projection.routes[1].target());
        assert_ne!(projection.routes[0].site(), projection.routes[1].site());
        assert_eq!(projection.target.result, DynamicCallableReturnKindV1::Value);
    }

    #[test]
    fn projection_has_no_physical_completion_authority() {
        let source = include_str!("carrier_completion.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("completion production source");
        for forbidden in [
            "BasicBlockId",
            "MirBuilder",
            "DraftSeal",
            "ReadyFunctionCompletion",
            "FunctionSignature",
            "PHI",
            "into_parts",
            "fallback",
        ] {
            assert!(
                !source.contains(forbidden),
                "forbidden term in completion projection: {forbidden}"
            );
        }
    }

    impl DynamicCallableCompletionRouteV1 {
        fn target(&self) -> DynamicCallableFunctionExitTargetV1 {
            match self {
                Self::InnerRecipeReturn { target, .. } | Self::OuterCallableTail { target, .. } => {
                    *target
                }
            }
        }

        fn site(&self) -> &SourceStmtSiteV1 {
            match self {
                Self::InnerRecipeReturn { site, .. } | Self::OuterCallableTail { site, .. } => site,
            }
        }
    }
}
