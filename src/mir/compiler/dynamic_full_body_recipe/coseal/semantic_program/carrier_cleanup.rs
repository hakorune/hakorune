//! Carrier-only cleanup projection for the bounded Dynamic Loop.
//!
//! This child consumes the complete semantic carrier-flow product and records
//! which already-issued dispositions are relevant at the exact fault/return/
//! backedge cut points. It does not extend lexical cleanup, issue Home, or
//! execute a physical End.

use crate::mir::loop_recipe_contract::{LoopItemKeyV1, LoopValueKeyV1};
use crate::mir::resolved_semantics::SourceStmtSiteV1;

use super::carrier_flow::{
    DynamicCarrierFlowBoundaryV1, DynamicCarrierFlowProgramRejectV1, DynamicCarrierFlowStateV1,
    VerifiedDynamicCarrierFlowProgramV1,
};
use super::{DynamicFullLoopFaultCutPointV2, DynamicFullLoopFaultFamilyV2};

const CLEANUP_CUT_POINT_COUNT_V1: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum DynamicCarrierCleanupProjectionRejectV1 {
    Flow(DynamicCarrierFlowProgramRejectV1),
    FaultCoverage,
    FaultDisposition,
    ReturnPartition,
    BackedgeDischarge,
    DuplicateCutPoint,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DynamicCarrierCleanupActionV1 {
    NoLiveLocalCarrier,
    DelegatedPublication {
        producer: LoopItemKeyV1,
        result: LoopValueKeyV1,
    },
    EndAuthorized {
        producer: LoopItemKeyV1,
        result: LoopValueKeyV1,
    },
    DischargeBeforeBackedge,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum DynamicCarrierCleanupCutPointV1 {
    Fault {
        item: LoopItemKeyV1,
        family: DynamicFullLoopFaultFamilyV2,
        actions: Box<[DynamicCarrierCleanupActionV1]>,
    },
    InnerReturn {
        site: SourceStmtSiteV1,
        action: DynamicCarrierCleanupActionV1,
    },
    Backedge {
        loop_key: crate::mir::loop_recipe_contract::LoopNodeKeyV1,
        write: LoopItemKeyV1,
        action: DynamicCarrierCleanupActionV1,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DynamicCarrierReturnPartitionV1 {
    inner: SourceStmtSiteV1,
    inner_action: DynamicCarrierCleanupActionV1,
    outer: SourceStmtSiteV1,
}

#[derive(Debug)]
pub(in crate::mir) struct VerifiedDynamicCarrierCleanupProjectionV1 {
    flow: VerifiedDynamicCarrierFlowProgramV1,
    rows: [DynamicCarrierCleanupCutPointV1; CLEANUP_CUT_POINT_COUNT_V1],
    return_partition: DynamicCarrierReturnPartitionV1,
}

impl VerifiedDynamicCarrierCleanupProjectionV1 {
    #[cfg(test)]
    pub(in crate::mir) fn current(&self) -> super::DynamicCarrierCurrentDispositionV1 {
        self.flow.current()
    }

    #[cfg(test)]
    pub(in crate::mir) fn rows(&self) -> &[DynamicCarrierCleanupCutPointV1; 8] {
        &self.rows
    }

    #[cfg(test)]
    pub(in crate::mir) fn return_partition(&self) -> &DynamicCarrierReturnPartitionV1 {
        &self.return_partition
    }

    pub(in crate::mir) fn completion_sites(&self) -> [SourceStmtSiteV1; 2] {
        [
            self.return_partition.inner.clone(),
            self.return_partition.outer.clone(),
        ]
    }

    pub(in crate::mir) fn completion_summary(
        &self,
    ) -> Option<(
        crate::mir::resolved_semantics::FunctionOwnerIdV1,
        crate::mir::resolved_semantics::RegionId,
        bool,
    )> {
        self.flow.completion_summary()
    }
}

pub(in crate::mir) fn issue_dynamic_carrier_cleanup_projection_i0(
    flow: VerifiedDynamicCarrierFlowProgramV1,
) -> Result<VerifiedDynamicCarrierCleanupProjectionV1, DynamicCarrierCleanupProjectionRejectV1> {
    let publications = flow.publications();
    let find_publication = |producer: u32, result: u32| {
        publications.iter().find(|row| {
            row.producer() == LoopItemKeyV1::new(producer)
                && row.result() == LoopValueKeyV1::new(result)
        })
    };
    let Some(v9) = find_publication(5, 9) else {
        return Err(DynamicCarrierCleanupProjectionRejectV1::FaultCoverage);
    };
    let Some(v10) = find_publication(6, 10) else {
        return Err(DynamicCarrierCleanupProjectionRejectV1::FaultCoverage);
    };
    let Some(v11) = find_publication(7, 11) else {
        return Err(DynamicCarrierCleanupProjectionRejectV1::FaultCoverage);
    };
    if v9.terminal() != DynamicCarrierFlowStateV1::EndAuthorized
        || v11.terminal() != DynamicCarrierFlowStateV1::EndAuthorized
        || v10.terminal() != DynamicCarrierFlowStateV1::EndAuthorized
        || !matches!(
            v10.boundary(),
            DynamicCarrierFlowBoundaryV1::LoopBodyExit { .. }
        )
    {
        return Err(DynamicCarrierCleanupProjectionRejectV1::Flow(
            DynamicCarrierFlowProgramRejectV1::RebindRelation,
        ));
    }

    let fault_rows = flow.fault_cut_points().rows();
    let exact_fault = |item: u32, family| {
        fault_rows
            .iter()
            .find(|row| row.item() == LoopItemKeyV1::new(item) && row.family() == family)
    };
    let Some(i1) = exact_fault(1, DynamicFullLoopFaultFamilyV2::DynamicLess) else {
        return Err(DynamicCarrierCleanupProjectionRejectV1::FaultCoverage);
    };
    let Some(i5) = exact_fault(5, DynamicFullLoopFaultFamilyV2::DynamicAdd) else {
        return Err(DynamicCarrierCleanupProjectionRejectV1::FaultCoverage);
    };
    let Some(i6) = exact_fault(6, DynamicFullLoopFaultFamilyV2::DynamicInvocation) else {
        return Err(DynamicCarrierCleanupProjectionRejectV1::FaultCoverage);
    };
    let Some(i7) = exact_fault(7, DynamicFullLoopFaultFamilyV2::DynamicInvocation) else {
        return Err(DynamicCarrierCleanupProjectionRejectV1::FaultCoverage);
    };
    let Some(i9) = exact_fault(9, DynamicFullLoopFaultFamilyV2::DynamicLess) else {
        return Err(DynamicCarrierCleanupProjectionRejectV1::FaultCoverage);
    };
    let Some(i15) = exact_fault(15, DynamicFullLoopFaultFamilyV2::DynamicAdd) else {
        return Err(DynamicCarrierCleanupProjectionRejectV1::FaultCoverage);
    };
    if [i1, i5, i6, i7, i9, i15]
        .iter()
        .map(|row| row.item())
        .collect::<std::collections::BTreeSet<_>>()
        .len()
        != 6
    {
        return Err(DynamicCarrierCleanupProjectionRejectV1::DuplicateCutPoint);
    }
    let end_v10 = DynamicCarrierCleanupActionV1::EndAuthorized {
        producer: v10.producer(),
        result: v10.result(),
    };
    let return_sites = flow
        .return_partition()
        .ok_or(DynamicCarrierCleanupProjectionRejectV1::ReturnPartition)?;
    let (backedge_loop, backedge_write) = flow.backedge();
    let rows = [
        fault_row(i1, [DynamicCarrierCleanupActionV1::NoLiveLocalCarrier]),
        fault_row(i5, [DynamicCarrierCleanupActionV1::NoLiveLocalCarrier]),
        fault_row(
            i6,
            [
                DynamicCarrierCleanupActionV1::NoLiveLocalCarrier,
                DynamicCarrierCleanupActionV1::DelegatedPublication {
                    producer: v9.producer(),
                    result: v9.result(),
                },
            ],
        ),
        fault_row(i7, [end_v10]),
        fault_row(
            i9,
            [
                DynamicCarrierCleanupActionV1::DelegatedPublication {
                    producer: v11.producer(),
                    result: v11.result(),
                },
                end_v10,
            ],
        ),
        fault_row(i15, [end_v10]),
        DynamicCarrierCleanupCutPointV1::InnerReturn {
            site: return_sites[0].clone(),
            action: end_v10,
        },
        DynamicCarrierCleanupCutPointV1::Backedge {
            loop_key: backedge_loop,
            write: backedge_write,
            action: DynamicCarrierCleanupActionV1::DischargeBeforeBackedge,
        },
    ];
    let return_partition = DynamicCarrierReturnPartitionV1 {
        inner: return_sites[0].clone(),
        inner_action: end_v10,
        outer: return_sites[1].clone(),
    };
    if !matches!(rows[7], DynamicCarrierCleanupCutPointV1::Backedge { .. }) {
        return Err(DynamicCarrierCleanupProjectionRejectV1::BackedgeDischarge);
    }
    Ok(VerifiedDynamicCarrierCleanupProjectionV1 {
        flow,
        rows,
        return_partition,
    })
}

fn fault_row(
    row: &DynamicFullLoopFaultCutPointV2,
    actions: impl Into<Box<[DynamicCarrierCleanupActionV1]>>,
) -> DynamicCarrierCleanupCutPointV1 {
    DynamicCarrierCleanupCutPointV1::Fault {
        item: row.item(),
        family: row.family(),
        actions: actions.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::compiler::dynamic_full_body_recipe::coseal::tests::fixture;
    use crate::mir::compiler::dynamic_full_body_recipe::coseal::{
        issue_dynamic_carrier_flow_program_v1, issue_dynamic_carrier_ingress_lifecycle_program_v1,
        issue_dynamic_carrier_rebind_transaction_program_v1,
        issue_dynamic_full_loop_semantic_program_v2,
        issue_dynamic_full_loop_source_recipe_envelope_v2,
        issue_dynamic_invocation_carrier_lifecycle_program_v1,
        issue_dynamic_operator_carrier_lifecycle_program_v1,
    };
    use crate::mir::compiler::dynamic_full_body_source::DynamicFullBodyBindingRoleV1;
    use crate::mir::resolved_semantics::HomeDemandV1;

    fn exact_projection() -> VerifiedDynamicCarrierCleanupProjectionV1 {
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
            issue_dynamic_full_loop_source_recipe_envelope_v2(fixture.candidate, fixture.calls)
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
        issue_dynamic_carrier_cleanup_projection_i0(flow).expect("cleanup projection")
    }

    #[test]
    fn exact_projection_seals_fault_rows_and_return_partition() {
        let projection = exact_projection();
        assert_eq!(projection.rows.len(), CLEANUP_CUT_POINT_COUNT_V1);
        assert!(matches!(
            projection.rows[0],
            DynamicCarrierCleanupCutPointV1::Fault {
                item,
                family: DynamicFullLoopFaultFamilyV2::DynamicLess,
                ..
            } if item == LoopItemKeyV1::new(1)
        ));
        assert!(matches!(
            projection.rows[5],
            DynamicCarrierCleanupCutPointV1::Fault {
                item,
                family: DynamicFullLoopFaultFamilyV2::DynamicAdd,
                ..
            } if item == LoopItemKeyV1::new(15)
        ));
        assert_eq!(
            projection.return_partition.inner_action,
            DynamicCarrierCleanupActionV1::EndAuthorized {
                producer: LoopItemKeyV1::new(6),
                result: LoopValueKeyV1::new(10),
            }
        );
        assert_ne!(
            projection.return_partition.inner,
            projection.return_partition.outer
        );
    }

    #[test]
    fn projection_has_no_physical_or_empty_cleanup_authority() {
        let source = include_str!("carrier_cleanup.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("cleanup production source");
        for forbidden in [
            "ValueId",
            "BasicBlockId",
            "MirBuilder",
            "ReleaseStrong",
            "ReadyFunctionCompletion",
            "explicit_empty",
            "into_parts",
            "fallback",
        ] {
            assert!(
                !source.contains(forbidden),
                "forbidden term in carrier cleanup projection: {forbidden}"
            );
        }
    }
}
