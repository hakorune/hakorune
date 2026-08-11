//! Invocation-only cleanup projection for the mixed-I64 Dynamic Loop.
//!
//! The induction carrier is an exact trivial I64 and has no Home/End
//! lifecycle.  This product retains only the two Dynamic invocation results
//! (V10/V11) and the already sealed invocation lifecycle.  It does not issue
//! source semantics, a second JoinSig, a physical block, or a runtime Fault.

use crate::mir::loop_recipe_contract::{
    LoopItemKeyV1, LoopNodeKeyV1, LoopValueClassV2, LoopValueKeyV1,
};
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, RegionId, SourceStmtSiteV1};

use super::{
    DynamicFullLoopFaultFamilyV2, DynamicInvocationCarrierLifecycleRowRefV1,
    VerifiedDynamicFullLoopSemanticProgramV2, VerifiedDynamicInvocationCarrierLifecycleProgramV1,
};

const INVOCATION_CLEANUP_ROW_COUNT_V1: usize = 6;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum DynamicInvocationCleanupCurrentDispositionV1 {
    ExactI64TrivialNoEnd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InvocationCleanupActionV1 {
    EndTemporary {
        producer: LoopItemKeyV1,
        result: LoopValueKeyV1,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum InvocationCleanupCutPointV1 {
    Fault {
        item: LoopItemKeyV1,
        family: DynamicFullLoopFaultFamilyV2,
        actions: Box<[InvocationCleanupActionV1]>,
    },
    NormalBoundary {
        item: LoopItemKeyV1,
        actions: Box<[InvocationCleanupActionV1]>,
    },
    InnerReturn {
        site: SourceStmtSiteV1,
        action: InvocationCleanupActionV1,
    },
    Backedge {
        loop_key: LoopNodeKeyV1,
        action: InvocationCleanupActionV1,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum DynamicInvocationCleanupProjectionRejectV1 {
    InvocationCoverage,
    TypedCarrier,
    FaultCoverage,
    CompletionCoverage,
    BackedgeCoverage,
}

#[derive(Debug)]
pub(in crate::mir) struct VerifiedDynamicInvocationCleanupProjectionV1 {
    invocation: VerifiedDynamicInvocationCarrierLifecycleProgramV1,
    rows: [InvocationCleanupCutPointV1; INVOCATION_CLEANUP_ROW_COUNT_V1],
}

impl VerifiedDynamicInvocationCleanupProjectionV1 {
    pub(in crate::mir) fn with_semantic_program<R>(
        &self,
        callback: impl for<'program> FnOnce(&'program VerifiedDynamicFullLoopSemanticProgramV2) -> R,
    ) -> R {
        self.invocation.with_semantic_program(callback)
    }

    pub(in crate::mir) fn a_prime_source_relation_view(
        &self,
    ) -> Result<
        super::DynamicAPrimeI64SourceRelationViewV1<'_>,
        super::DynamicAPrimeI64SourceRelationRejectV1,
    > {
        self.invocation.a_prime_source_relation_view()
    }

    pub(in crate::mir) fn physical_input_view(
        &self,
    ) -> Result<
        super::DynamicFullLoopPhysicalInputViewV2<'_>,
        super::DynamicFullLoopPhysicalInputRejectV2,
    > {
        self.invocation.physical_input_view()
    }

    pub(in crate::mir) const fn current(&self) -> DynamicInvocationCleanupCurrentDispositionV1 {
        DynamicInvocationCleanupCurrentDispositionV1::ExactI64TrivialNoEnd
    }

    pub(in crate::mir) fn completion_sites(&self) -> Option<[SourceStmtSiteV1; 2]> {
        self.invocation
            .with_semantic_program(|program| program.completion_sites())
    }

    pub(in crate::mir) fn completion_summary(&self) -> Option<(FunctionOwnerIdV1, RegionId, bool)> {
        self.invocation
            .with_semantic_program(|program| program.completion_summary())
    }

    #[cfg(test)]
    pub(in crate::mir) fn rows(&self) -> &[InvocationCleanupCutPointV1; 6] {
        &self.rows
    }
}

pub(in crate::mir) fn issue_dynamic_invocation_cleanup_projection_i0(
    invocation: VerifiedDynamicInvocationCarrierLifecycleProgramV1,
) -> Result<VerifiedDynamicInvocationCleanupProjectionV1, DynamicInvocationCleanupProjectionRejectV1>
{
    let rows = invocation.invocation_lifecycle().rows().collect::<Vec<_>>();
    if rows.len() != 2 || !has_invocation_row(&rows, 6, 10) || !has_invocation_row(&rows, 7, 11) {
        return Err(DynamicInvocationCleanupProjectionRejectV1::InvocationCoverage);
    }

    let typed = invocation.with_semantic_program(|program| {
        program.after().class() == LoopValueClassV2::I64
            && program.recipe_value_class(LoopValueKeyV1::new(10))
                == Some(LoopValueClassV2::Dynamic)
            && program.recipe_value_class(LoopValueKeyV1::new(11))
                == Some(LoopValueClassV2::Dynamic)
            && program.recipe_value_class(LoopValueKeyV1::new(14)) == Some(LoopValueClassV2::I64)
            && program.recipe_value_class(LoopValueKeyV1::new(15)) == Some(LoopValueClassV2::I64)
            && program.recipe_value_class(LoopValueKeyV1::new(17)) == Some(LoopValueClassV2::I64)
    });
    if !typed {
        return Err(DynamicInvocationCleanupProjectionRejectV1::TypedCarrier);
    }

    let faults = invocation.fault_cut_points().rows();
    let i6 = exact_fault(faults, 6, DynamicFullLoopFaultFamilyV2::DynamicInvocation)?;
    let i7 = exact_fault(faults, 7, DynamicFullLoopFaultFamilyV2::DynamicInvocation)?;
    let i9 = exact_fault(faults, 9, DynamicFullLoopFaultFamilyV2::DynamicLess)?;
    if faults.len() != 3
        || [i6.item(), i7.item(), i9.item()]
            != [
                LoopItemKeyV1::new(6),
                LoopItemKeyV1::new(7),
                LoopItemKeyV1::new(9),
            ]
    {
        return Err(DynamicInvocationCleanupProjectionRejectV1::FaultCoverage);
    }
    let Some(sites) = invocation.with_semantic_program(|program| program.completion_sites()) else {
        return Err(DynamicInvocationCleanupProjectionRejectV1::CompletionCoverage);
    };
    let loop_key = invocation.after().loop_key();
    let end_v10 = InvocationCleanupActionV1::EndTemporary {
        producer: LoopItemKeyV1::new(6),
        result: LoopValueKeyV1::new(10),
    };
    let end_v11 = InvocationCleanupActionV1::EndTemporary {
        producer: LoopItemKeyV1::new(7),
        result: LoopValueKeyV1::new(11),
    };
    let rows = [
        InvocationCleanupCutPointV1::Fault {
            item: i6.item(),
            family: i6.family(),
            actions: Box::new([]),
        },
        InvocationCleanupCutPointV1::Fault {
            item: i7.item(),
            family: i7.family(),
            actions: Box::new([end_v10]),
        },
        InvocationCleanupCutPointV1::Fault {
            item: i9.item(),
            family: i9.family(),
            actions: Box::new([end_v11, end_v10]),
        },
        InvocationCleanupCutPointV1::NormalBoundary {
            item: i9.item(),
            actions: Box::new([end_v11]),
        },
        InvocationCleanupCutPointV1::InnerReturn {
            site: sites[0].clone(),
            action: end_v10,
        },
        InvocationCleanupCutPointV1::Backedge {
            loop_key,
            action: end_v10,
        },
    ];
    Ok(VerifiedDynamicInvocationCleanupProjectionV1 { invocation, rows })
}

fn has_invocation_row(
    rows: &[DynamicInvocationCarrierLifecycleRowRefV1<'_>],
    producer: u32,
    result: u32,
) -> bool {
    rows.iter().any(|row| {
        row.producer() == LoopItemKeyV1::new(producer)
            && row.result() == LoopValueKeyV1::new(result)
    })
}

fn exact_fault<'a>(
    rows: &'a [super::DynamicFullLoopFaultCutPointV2],
    item: u32,
    family: DynamicFullLoopFaultFamilyV2,
) -> Result<&'a super::DynamicFullLoopFaultCutPointV2, DynamicInvocationCleanupProjectionRejectV1> {
    rows.iter()
        .find(|row| row.item() == LoopItemKeyV1::new(item) && row.family() == family)
        .ok_or(DynamicInvocationCleanupProjectionRejectV1::FaultCoverage)
}
