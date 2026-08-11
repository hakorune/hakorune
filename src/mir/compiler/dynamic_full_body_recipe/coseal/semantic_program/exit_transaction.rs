//! Bounded exit-transaction co-seal for the Dynamic callable.
//!
//! The existing `VerifiedFunctionCompletionV1` remains the source-side owner
//! of return coverage and result classification. This child consumes the
//! carrier cleanup projection and seals the relation from the exact inner
//! Recipe Return and outer Callable Tail to one logical function-exit target.
//! The cleanup, flow, and semantic program remain transitively owned; this is
//! the final semantic co-seal for the current lane, not a second wrapper.
//! It does not write a Return, create merge/ABI facts, or invoke the final
//! function-seal stage.

use crate::mir::resolved_semantics::{FunctionOwnerIdV1, RegionId, SourceStmtSiteV1};

use super::super::a_prime_source::{
    DynamicAPrimeI64SourceRelationRejectV1, DynamicAPrimeI64SourceRelationViewV1,
};
use super::VerifiedDynamicInvocationCleanupProjectionV1;
use super::{DynamicFullLoopPhysicalInputRejectV2, DynamicFullLoopPhysicalInputViewV2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum DynamicExitTransactionCoSealRejectV1 {
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
enum DynamicExitTransactionRouteV1 {
    InnerRecipeReturn {
        site: SourceStmtSiteV1,
        target: DynamicCallableFunctionExitTargetV1,
    },
    OuterCallableTail {
        site: SourceStmtSiteV1,
        target: DynamicCallableFunctionExitTargetV1,
    },
}

const EXIT_TRANSACTION_ROUTE_COUNT_V1: usize = 2;

#[derive(Debug)]
pub(in crate::mir) struct VerifiedDynamicExitTransactionCoSealV1 {
    cleanup: VerifiedDynamicInvocationCleanupProjectionV1,
    routes: [DynamicExitTransactionRouteV1; EXIT_TRANSACTION_ROUTE_COUNT_V1],
    target: DynamicCallableFunctionExitTargetV1,
}

impl VerifiedDynamicExitTransactionCoSealV1 {
    pub(in crate::mir) fn with_a_prime_source_relation<R>(
        &self,
        callback: impl for<'program> FnOnce(DynamicAPrimeI64SourceRelationViewV1<'program>) -> R,
    ) -> Result<R, DynamicAPrimeI64SourceRelationRejectV1> {
        self.cleanup
            .with_semantic_program(|semantic| semantic.with_a_prime_source_relation(callback))
    }

    pub(in crate::mir) fn with_physical_input<R>(
        &self,
        callback: impl for<'program> FnOnce(DynamicFullLoopPhysicalInputViewV2<'program>) -> R,
    ) -> Result<R, DynamicFullLoopPhysicalInputRejectV2> {
        self.cleanup
            .with_semantic_program(|semantic| super::physical_input::issue(semantic, callback))
    }

    pub(in crate::mir) fn a_prime_source_relation_view(
        &self,
    ) -> Result<DynamicAPrimeI64SourceRelationViewV1<'_>, DynamicAPrimeI64SourceRelationRejectV1>
    {
        self.cleanup.a_prime_source_relation_view()
    }

    pub(in crate::mir) fn physical_input_view(
        &self,
    ) -> Result<DynamicFullLoopPhysicalInputViewV2<'_>, DynamicFullLoopPhysicalInputRejectV2> {
        self.cleanup.physical_input_view()
    }

    #[cfg(test)]
    pub(in crate::mir) fn current(&self) -> super::DynamicInvocationCleanupCurrentDispositionV1 {
        self.cleanup.current()
    }

    #[cfg(test)]
    pub(in crate::mir) fn routes(&self) -> &[DynamicExitTransactionRouteV1; 2] {
        &self.routes
    }

    #[cfg(test)]
    pub(in crate::mir) fn target(&self) -> DynamicCallableFunctionExitTargetV1 {
        self.target
    }

    #[cfg(test)]
    pub(in crate::mir) fn cleanup(&self) -> &VerifiedDynamicInvocationCleanupProjectionV1 {
        &self.cleanup
    }
}

pub(in crate::mir) fn issue_dynamic_exit_transaction_coseal_i0(
    cleanup: VerifiedDynamicInvocationCleanupProjectionV1,
) -> Result<VerifiedDynamicExitTransactionCoSealV1, DynamicExitTransactionCoSealRejectV1> {
    let Some(sites) = cleanup.completion_sites() else {
        return Err(DynamicExitTransactionCoSealRejectV1::CompletionCoverage);
    };
    if sites[0] == sites[1] {
        return Err(DynamicExitTransactionCoSealRejectV1::CompletionCoverage);
    }
    let Some((owner, target, returns_value)) = cleanup.completion_summary() else {
        return Err(DynamicExitTransactionCoSealRejectV1::CleanupPartition);
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
        DynamicExitTransactionRouteV1::InnerRecipeReturn {
            site: sites[0].clone(),
            target,
        },
        DynamicExitTransactionRouteV1::OuterCallableTail {
            site: sites[1].clone(),
            target,
        },
    ];
    Ok(VerifiedDynamicExitTransactionCoSealV1 {
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
        issue_dynamic_full_loop_semantic_program_v2,
        issue_dynamic_full_loop_source_recipe_envelope_v2,
        issue_dynamic_invocation_carrier_lifecycle_program_v1,
        issue_dynamic_invocation_cleanup_projection_i0,
    };
    use crate::mir::compiler::dynamic_full_body_recipe::issue_dynamic_full_loop_operation_physical_demand_v2;
    use crate::mir::loop_recipe_contract::{
        LoopBlockKeyV1, LoopConditionV2, LoopExitKindV2, LoopItemKeyV1,
        LoopJoinBranchArmTransferRefV2, LoopNodeKeyV1, LoopValueKeyV1,
    };

    fn exact_coseal() -> VerifiedDynamicExitTransactionCoSealV1 {
        let fixture = fixture(true);
        let envelope =
            issue_dynamic_full_loop_source_recipe_envelope_v2(fixture.candidate, fixture.calls)
                .expect("source/Recipe envelope");
        let semantic =
            issue_dynamic_full_loop_semantic_program_v2(envelope).expect("semantic program");
        let invocation = issue_dynamic_invocation_carrier_lifecycle_program_v1(semantic)
            .expect("invocation lifecycle");
        let cleanup = issue_dynamic_invocation_cleanup_projection_i0(invocation)
            .expect("invocation cleanup projection");
        issue_dynamic_exit_transaction_coseal_i0(cleanup).expect("exit transaction co-seal")
    }

    #[test]
    fn exact_coseal_seals_two_routes_to_one_function_exit() {
        let projection = exact_coseal();
        assert_eq!(projection.routes.len(), 2);
        assert_eq!(projection.routes[0].target(), projection.routes[1].target());
        assert_ne!(projection.routes[0].site(), projection.routes[1].site());
        assert_eq!(projection.target.result, DynamicCallableReturnKindV1::Value);
    }

    #[test]
    fn coseal_has_no_physical_or_runtime_authority() {
        let source = include_str!("exit_transaction.rs")
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
                "forbidden term in exit transaction co-seal: {forbidden}"
            );
        }
    }

    #[test]
    fn final_exit_coseal_lends_one_complete_physical_input_view() {
        let transaction = exact_coseal();
        transaction
            .with_physical_input(|input| {
                assert_eq!(input.placements().len(), 17);
                assert_eq!(input.operations().len(), 15);
                assert_eq!(
                    input
                        .operations()
                        .iter()
                        .filter(|row| row.call().is_some())
                        .count(),
                    2
                );
                assert_eq!(input.control().rows().len(), 1);
                assert_eq!(input.control().logical().branches().len(), 1);
                let control = &input.control().rows()[0];
                assert_eq!(control.loop_key(), LoopNodeKeyV1::new(0));
                assert_eq!(control.body_block(), LoopBlockKeyV1::new(1));
                assert_eq!(
                    control.condition(),
                    LoopConditionV2::Predicate {
                        block: LoopBlockKeyV1::new(0),
                        value: LoopValueKeyV1::new(5),
                    }
                );
                assert_eq!(control.branches().len(), 1);
                let branch = &control.branches()[0];
                assert_eq!(branch.owner_block(), LoopBlockKeyV1::new(1));
                assert_eq!(branch.if_item(), LoopItemKeyV1::new(10));
                assert_eq!(branch.condition(), LoopValueKeyV1::new(13));
                assert_eq!(branch.then_block(), LoopBlockKeyV1::new(2));
                assert_eq!(branch.else_block(), None);
                assert_eq!(
                    branch.then_arm(),
                    crate::mir::compiler::dynamic_full_body_recipe::coseal::DynamicLoopPhysicalArmV2::Exit {
                        item: LoopItemKeyV1::new(12),
                        kind: LoopExitKindV2::Return {
                            value: Some(LoopValueKeyV1::new(14)),
                        },
                    }
                );
                assert!(matches!(
                    branch.else_arm(),
                    crate::mir::compiler::dynamic_full_body_recipe::coseal::DynamicLoopPhysicalArmV2::Fallthrough
                ));
                let logical = &input.control().logical().branches()[0];
                assert!(matches!(
                    logical.then_arm,
                    LoopJoinBranchArmTransferRefV2::Exit(_)
                ));
                assert_eq!(input.faults().rows().len(), 3);
            })
            .expect("final exit co-seal physical input");
    }

    #[test]
    fn physical_demand_consumes_the_complete_view_inside_the_htrb_loan() {
        let transaction = exact_coseal();
        transaction
            .with_physical_input(|input| {
                let prepared = issue_dynamic_full_loop_operation_physical_demand_v2(input)
                    .expect("complete physical demand")
                    .prepare_all()
                    .expect("complete physical demand coverage");
                let coverage = prepared.coverage();
                assert_eq!(coverage.operation_count(), 15);
                assert_eq!(coverage.placement_count(), 17);
                assert_eq!(coverage.control_count(), 1);
                assert_eq!(coverage.fault_count(), 3);
                assert_eq!(prepared.operation_rows().len(), 15);
                assert_eq!(prepared.placement_rows().len(), 17);
                assert_eq!(prepared.control().rows().len(), 1);
                assert_eq!(prepared.faults().rows().len(), 3);
            })
            .expect("physical demand HRTB loan");
    }

    impl DynamicExitTransactionRouteV1 {
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
