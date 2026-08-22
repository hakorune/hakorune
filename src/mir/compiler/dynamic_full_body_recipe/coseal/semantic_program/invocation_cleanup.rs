//! Invocation-only cleanup projection for the mixed-I64 Dynamic Loop.
//!
//! The induction carrier is an exact trivial I64 and has no Home/End
//! lifecycle.  This product retains only the Dynamic substring result (V10)
//! and the already sealed invocation lifecycle.  The indexOf result (V11) is
//! an exact I64 with no lifecycle obligation.  It does not issue
//! source semantics, a second JoinSig, a physical block, or a runtime Fault.

use crate::mir::loop_recipe_contract::{
    LoopItemKeyV1, LoopNodeKeyV1, LoopValueClassV2, LoopValueKeyV1,
};
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, RegionId, SourceStmtSiteV1};

use super::{
    DynamicFullLoopFaultFamilyV2, DynamicInvocationCarrierLifecycleRowRefV1,
    VerifiedDynamicFullLoopSemanticProgramV2, VerifiedDynamicInvocationCarrierLifecycleProgramV1,
};

const INVOCATION_CLEANUP_ROW_COUNT_V1: usize = 4;

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

/// Borrow-free identity view of one already verified cleanup row.  This is a
/// semantic evidence view only; it contains no ValueId, block, End
/// instruction, or physical capability.  Boundary identity is retained so a
/// later physical consumer cannot treat the fixed row order as provenance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum DynamicInvocationCleanupRowKindV1 {
    Fault,
    InnerReturn,
    Backedge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) struct DynamicInvocationCleanupActionViewV1 {
    producer: LoopItemKeyV1,
    result: LoopValueKeyV1,
}

impl DynamicInvocationCleanupActionViewV1 {
    pub(in crate::mir) const fn producer(self) -> LoopItemKeyV1 {
        self.producer
    }

    pub(in crate::mir) const fn result(self) -> LoopValueKeyV1 {
        self.result
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) struct DynamicInvocationCleanupRowViewV1 {
    kind: DynamicInvocationCleanupRowKindV1,
    item: Option<LoopItemKeyV1>,
    inner_return_site: Option<SourceStmtSiteV1>,
    backedge_loop: Option<LoopNodeKeyV1>,
    first: Option<DynamicInvocationCleanupActionViewV1>,
    second: Option<DynamicInvocationCleanupActionViewV1>,
}

impl DynamicInvocationCleanupRowViewV1 {
    pub(in crate::mir) const fn kind(&self) -> DynamicInvocationCleanupRowKindV1 {
        self.kind
    }

    pub(in crate::mir) const fn item(&self) -> Option<LoopItemKeyV1> {
        self.item
    }

    pub(in crate::mir) fn inner_return_site(&self) -> Option<&SourceStmtSiteV1> {
        self.inner_return_site.as_ref()
    }

    pub(in crate::mir) const fn backedge_loop(&self) -> Option<LoopNodeKeyV1> {
        self.backedge_loop
    }

    pub(in crate::mir) const fn first(&self) -> Option<DynamicInvocationCleanupActionViewV1> {
        self.first
    }

    pub(in crate::mir) const fn second(&self) -> Option<DynamicInvocationCleanupActionViewV1> {
        self.second
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum InvocationCleanupCutPointV1 {
    Fault {
        item: LoopItemKeyV1,
        family: DynamicFullLoopFaultFamilyV2,
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

    pub(in crate::mir) fn physical_rows(&self) -> [DynamicInvocationCleanupRowViewV1; 4] {
        std::array::from_fn(|index| cleanup_row_view(&self.rows[index]))
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
    pub(in crate::mir) fn rows(&self) -> &[InvocationCleanupCutPointV1; 4] {
        &self.rows
    }
}

fn cleanup_row_view(row: &InvocationCleanupCutPointV1) -> DynamicInvocationCleanupRowViewV1 {
    let (kind, item, inner_return_site, backedge_loop, actions) = match row {
        InvocationCleanupCutPointV1::Fault { item, actions, .. } => (
            DynamicInvocationCleanupRowKindV1::Fault,
            Some(*item),
            None,
            None,
            actions.as_ref(),
        ),
        InvocationCleanupCutPointV1::InnerReturn { site, action } => (
            DynamicInvocationCleanupRowKindV1::InnerReturn,
            None,
            Some(site.clone()),
            None,
            std::slice::from_ref(action),
        ),
        InvocationCleanupCutPointV1::Backedge { loop_key, action } => (
            DynamicInvocationCleanupRowKindV1::Backedge,
            None,
            None,
            Some(*loop_key),
            std::slice::from_ref(action),
        ),
    };
    let action = |action: &InvocationCleanupActionV1| match action {
        InvocationCleanupActionV1::EndTemporary { producer, result } => {
            DynamicInvocationCleanupActionViewV1 {
                producer: *producer,
                result: *result,
            }
        }
    };
    DynamicInvocationCleanupRowViewV1 {
        kind,
        item,
        inner_return_site,
        backedge_loop,
        first: actions.first().map(action),
        second: actions.get(1).map(action),
    }
}

pub(in crate::mir) fn issue_dynamic_invocation_cleanup_projection_i0(
    invocation: VerifiedDynamicInvocationCarrierLifecycleProgramV1,
) -> Result<VerifiedDynamicInvocationCleanupProjectionV1, DynamicInvocationCleanupProjectionRejectV1>
{
    let rows = invocation.invocation_lifecycle().rows().collect::<Vec<_>>();
    if rows.len() != 1 || !has_invocation_row(&rows, 6, 10) {
        return Err(DynamicInvocationCleanupProjectionRejectV1::InvocationCoverage);
    }

    let typed = invocation.with_semantic_program(|program| {
        program.after().class() == LoopValueClassV2::I64
            && program.recipe_value_class(LoopValueKeyV1::new(10))
                == Some(LoopValueClassV2::Dynamic)
            && program.recipe_value_class(LoopValueKeyV1::new(11)) == Some(LoopValueClassV2::I64)
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
    if faults.len() != 2 || [i6.item(), i7.item()] != [LoopItemKeyV1::new(6), LoopItemKeyV1::new(7)]
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
