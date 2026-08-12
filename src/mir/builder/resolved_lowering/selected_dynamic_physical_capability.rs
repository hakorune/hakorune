//! Builder-free capability admission for the selected Dynamic V2 cohort.
//!
//! The module owns no semantic meaning.  It only checks that the already
//! co-sealed I9 I64 operation and four cleanup rows can be handed to physical
//! producers.  The current backend leaves intentionally stop at
//! `RejectBeforeEffect`; no session or ValueId is created here.

use crate::mir::compiler::dynamic_full_body_recipe::{
    DynamicInvocationCleanupActionViewV1, DynamicInvocationCleanupRowKindV1,
    DynamicInvocationCleanupRowViewV1,
};
use crate::mir::compiler::dynamic_full_body_source::DynamicFullBodySourceRoleV1;
use crate::mir::core_method_op::CoreMethodOp;
use crate::mir::core_method_result_kind::{CoreMethodEffectV1, CoreMethodResultKindV1};
use crate::mir::loop_recipe_contract::{
    LoopItemKeyV1, LoopOperationExecutionClassV2, LoopOperationV2, LoopValueKeyV1,
};
use crate::mir::resolved_semantics::SourceStmtSiteV1;

use super::selected_dynamic_physical_abi::PreparedSelectedDynamicV2EmissionPlanV1;

const I6: u32 = 6;
const I7: u32 = 7;
const I8: u32 = 8;
const I9: u32 = 9;
const V10: u32 = 10;
const V11: u32 = 11;
const V12: u32 = 12;
const V13: u32 = 13;
const CLEANUP_ROW_COUNT: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum SelectedDynamicV2PhysicalCapabilityRejectV1 {
    CompareI64Operation,
    ProducerReceiptUnavailable,
    CleanupCoverage,
    CleanupOrder,
    EndCapabilityUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum DynamicV2ProducerFamilyV1 {
    DynamicCallSlot,
    ConstI64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum DynamicV2PhysicalRepresentationV1 {
    ImmediateI64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) struct DynamicV2ProducerReceiptRequirementV1 {
    producer: LoopItemKeyV1,
    result: LoopValueKeyV1,
    family: DynamicV2ProducerFamilyV1,
    representation: DynamicV2PhysicalRepresentationV1,
}

impl DynamicV2ProducerReceiptRequirementV1 {
    pub(in crate::mir) const fn producer(self) -> LoopItemKeyV1 {
        self.producer
    }

    pub(in crate::mir) const fn result(self) -> LoopValueKeyV1 {
        self.result
    }

    pub(in crate::mir) const fn family(self) -> DynamicV2ProducerFamilyV1 {
        self.family
    }

    pub(in crate::mir) const fn representation(self) -> DynamicV2PhysicalRepresentationV1 {
        self.representation
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) struct DynamicV2CompareI64CapabilityDemandV1 {
    item: LoopItemKeyV1,
    left: LoopValueKeyV1,
    right: LoopValueKeyV1,
    result: LoopValueKeyV1,
    v11: DynamicV2ProducerReceiptRequirementV1,
    v12: DynamicV2ProducerReceiptRequirementV1,
}

impl DynamicV2CompareI64CapabilityDemandV1 {
    pub(in crate::mir) const fn item(self) -> LoopItemKeyV1 {
        self.item
    }

    pub(in crate::mir) const fn left(self) -> LoopValueKeyV1 {
        self.left
    }

    pub(in crate::mir) const fn right(self) -> LoopValueKeyV1 {
        self.right
    }

    pub(in crate::mir) const fn result(self) -> LoopValueKeyV1 {
        self.result
    }

    pub(in crate::mir) const fn v11(self) -> DynamicV2ProducerReceiptRequirementV1 {
        self.v11
    }

    pub(in crate::mir) const fn v12(self) -> DynamicV2ProducerReceiptRequirementV1 {
        self.v12
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) struct DynamicV2DischargeActionRequirementV1 {
    producer: LoopItemKeyV1,
    result: LoopValueKeyV1,
}

impl DynamicV2DischargeActionRequirementV1 {
    pub(in crate::mir) const fn producer(self) -> LoopItemKeyV1 {
        self.producer
    }

    pub(in crate::mir) const fn result(self) -> LoopValueKeyV1 {
        self.result
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) struct DynamicV2TemporaryDischargeRowV1 {
    kind: DynamicInvocationCleanupRowKindV1,
    item: Option<LoopItemKeyV1>,
    inner_return_site: Option<SourceStmtSiteV1>,
    backedge_loop: Option<crate::mir::loop_recipe_contract::LoopNodeKeyV1>,
    first: Option<DynamicV2DischargeActionRequirementV1>,
    second: Option<DynamicV2DischargeActionRequirementV1>,
}

impl DynamicV2TemporaryDischargeRowV1 {
    pub(in crate::mir) const fn kind(&self) -> DynamicInvocationCleanupRowKindV1 {
        self.kind
    }

    pub(in crate::mir) const fn item(&self) -> Option<LoopItemKeyV1> {
        self.item
    }

    pub(in crate::mir) fn inner_return_site(&self) -> Option<&SourceStmtSiteV1> {
        self.inner_return_site.as_ref()
    }

    pub(in crate::mir) const fn backedge_loop(
        &self,
    ) -> Option<crate::mir::loop_recipe_contract::LoopNodeKeyV1> {
        self.backedge_loop
    }

    pub(in crate::mir) const fn first(&self) -> Option<DynamicV2DischargeActionRequirementV1> {
        self.first
    }

    pub(in crate::mir) const fn second(&self) -> Option<DynamicV2DischargeActionRequirementV1> {
        self.second
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum DynamicV2PhysicalCapabilityDispositionV1 {
    RejectBeforeEffect,
}

/// Move-only pair of physical capability demands.  The current pair carries
/// the exact requirements but has no backend leaf yet, so its disposition is
/// an explicit pre-effect rejection rather than an implicit no-op.
#[derive(Debug)]
pub(in crate::mir) struct SelectedDynamicV2PhysicalCapabilityAdmissionV1<'program> {
    plan: PreparedSelectedDynamicV2EmissionPlanV1<'program>,
    compare_i64: DynamicV2CompareI64CapabilityDemandV1,
    cleanup: [DynamicV2TemporaryDischargeRowV1; CLEANUP_ROW_COUNT],
    disposition: DynamicV2PhysicalCapabilityDispositionV1,
}

impl<'program> SelectedDynamicV2PhysicalCapabilityAdmissionV1<'program> {
    pub(in crate::mir) const fn disposition(&self) -> DynamicV2PhysicalCapabilityDispositionV1 {
        self.disposition
    }

    #[cfg(test)]
    pub(in crate::mir) const fn compare_i64(&self) -> DynamicV2CompareI64CapabilityDemandV1 {
        self.compare_i64
    }

    #[cfg(test)]
    pub(in crate::mir) const fn cleanup(
        &self,
    ) -> &[DynamicV2TemporaryDischargeRowV1; CLEANUP_ROW_COUNT] {
        &self.cleanup
    }

    pub(in crate::mir) fn into_rejected_plan(
        self,
    ) -> Result<
        PreparedSelectedDynamicV2EmissionPlanV1<'program>,
        SelectedDynamicV2PhysicalCapabilityRejectV1,
    > {
        match self.disposition {
            DynamicV2PhysicalCapabilityDispositionV1::RejectBeforeEffect => {
                Err(SelectedDynamicV2PhysicalCapabilityRejectV1::ProducerReceiptUnavailable)
            }
        }
    }
}

pub(in crate::mir) fn issue_selected_dynamic_v2_physical_capability_admission<'program>(
    plan: PreparedSelectedDynamicV2EmissionPlanV1<'program>,
) -> Result<
    SelectedDynamicV2PhysicalCapabilityAdmissionV1<'program>,
    SelectedDynamicV2PhysicalCapabilityRejectV1,
> {
    let compare_i64 = issue_compare_i64_demand(&plan)?;
    let cleanup = issue_cleanup_demand(&plan)?;
    Ok(SelectedDynamicV2PhysicalCapabilityAdmissionV1 {
        plan,
        compare_i64,
        cleanup,
        disposition: DynamicV2PhysicalCapabilityDispositionV1::RejectBeforeEffect,
    })
}

fn issue_compare_i64_demand(
    plan: &PreparedSelectedDynamicV2EmissionPlanV1<'_>,
) -> Result<DynamicV2CompareI64CapabilityDemandV1, SelectedDynamicV2PhysicalCapabilityRejectV1> {
    plan.with_operation_program(|program| {
        let i7 = program
            .operation_rows()
            .iter()
            .find(|row| row.item() == LoopItemKeyV1::new(I7))
            .ok_or(SelectedDynamicV2PhysicalCapabilityRejectV1::CompareI64Operation)?;
        let i8 = program
            .operation_rows()
            .iter()
            .find(|row| row.item() == LoopItemKeyV1::new(I8))
            .ok_or(SelectedDynamicV2PhysicalCapabilityRejectV1::CompareI64Operation)?;
        let i9 = program
            .operation_rows()
            .iter()
            .find(|row| row.item() == LoopItemKeyV1::new(I9))
            .ok_or(SelectedDynamicV2PhysicalCapabilityRejectV1::CompareI64Operation)?;
        let (left, right, result) = match i9.operation() {
            LoopOperationV2::CompareI64 {
                op: crate::mir::loop_recipe_contract::LoopCompareI64OpV2::Less,
                left,
                right,
                result,
            } => (*left, *right, *result),
            _ => return Err(SelectedDynamicV2PhysicalCapabilityRejectV1::CompareI64Operation),
        };
        if left != LoopValueKeyV1::new(V11)
            || right != LoopValueKeyV1::new(V12)
            || result != LoopValueKeyV1::new(V13)
            || i9.execution() != LoopOperationExecutionClassV2::NonFaulting
        {
            return Err(SelectedDynamicV2PhysicalCapabilityRejectV1::CompareI64Operation);
        }
        if !matches!(i7.operation(), LoopOperationV2::CallSlot { result: Some(value), .. } if *value == LoopValueKeyV1::new(V11))
            || i7.call_role() != Some(DynamicFullBodySourceRoleV1::IndexOfCall)
            || !matches!(i8.operation(), LoopOperationV2::ConstI64 { result, value: 0 } if *result == LoopValueKeyV1::new(V12))
        {
            return Err(SelectedDynamicV2PhysicalCapabilityRejectV1::ProducerReceiptUnavailable);
        }
        let i7_core = i7
            .core_method()
            .ok_or(SelectedDynamicV2PhysicalCapabilityRejectV1::ProducerReceiptUnavailable)?;
        if i7_core.op != CoreMethodOp::StringIndexOf
            || i7_core.result_kind != CoreMethodResultKindV1::I64Value
            || i7_core.effect != CoreMethodEffectV1::PureRead
        {
            return Err(SelectedDynamicV2PhysicalCapabilityRejectV1::ProducerReceiptUnavailable);
        }
        if program
            .faults()
            .rows()
            .iter()
            .any(|row| row.item() == LoopItemKeyV1::new(I9))
        {
            return Err(SelectedDynamicV2PhysicalCapabilityRejectV1::CompareI64Operation);
        }
        Ok(DynamicV2CompareI64CapabilityDemandV1 {
            item: LoopItemKeyV1::new(I9),
            left,
            right,
            result,
            v11: DynamicV2ProducerReceiptRequirementV1 {
                producer: LoopItemKeyV1::new(I7),
                result: LoopValueKeyV1::new(V11),
                family: DynamicV2ProducerFamilyV1::DynamicCallSlot,
                representation: DynamicV2PhysicalRepresentationV1::ImmediateI64,
            },
            v12: DynamicV2ProducerReceiptRequirementV1 {
                producer: LoopItemKeyV1::new(I8),
                result: LoopValueKeyV1::new(V12),
                family: DynamicV2ProducerFamilyV1::ConstI64,
                representation: DynamicV2PhysicalRepresentationV1::ImmediateI64,
            },
        })
    })
}

fn issue_cleanup_demand(
    plan: &PreparedSelectedDynamicV2EmissionPlanV1<'_>,
) -> Result<
    [DynamicV2TemporaryDischargeRowV1; CLEANUP_ROW_COUNT],
    SelectedDynamicV2PhysicalCapabilityRejectV1,
> {
    let expected_sites = plan
        .completion_sites()
        .ok_or(SelectedDynamicV2PhysicalCapabilityRejectV1::CleanupCoverage)?;
    let expected_loop = plan.with_operation_program(|program| {
        program.control().rows().first().map(|row| row.loop_key())
    });
    let expected_loop =
        expected_loop.ok_or(SelectedDynamicV2PhysicalCapabilityRejectV1::CleanupCoverage)?;
    plan.with_cleanup_physical_rows(|rows| {
        let converted = rows.map(convert_cleanup_row);
        validate_cleanup_rows(&converted, &expected_sites, expected_loop)?;
        Ok(converted)
    })
}

fn convert_cleanup_row(row: DynamicInvocationCleanupRowViewV1) -> DynamicV2TemporaryDischargeRowV1 {
    DynamicV2TemporaryDischargeRowV1 {
        kind: row.kind(),
        item: row.item(),
        inner_return_site: row.inner_return_site().cloned(),
        backedge_loop: row.backedge_loop(),
        first: row.first().map(convert_action),
        second: row.second().map(convert_action),
    }
}

fn convert_action(
    action: DynamicInvocationCleanupActionViewV1,
) -> DynamicV2DischargeActionRequirementV1 {
    DynamicV2DischargeActionRequirementV1 {
        producer: action.producer(),
        result: action.result(),
    }
}

fn validate_cleanup_rows(
    rows: &[DynamicV2TemporaryDischargeRowV1; CLEANUP_ROW_COUNT],
    completion_sites: &[SourceStmtSiteV1; 2],
    loop_key: crate::mir::loop_recipe_contract::LoopNodeKeyV1,
) -> Result<(), SelectedDynamicV2PhysicalCapabilityRejectV1> {
    let expected = [
        (
            DynamicInvocationCleanupRowKindV1::Fault,
            Some(I6),
            None,
            None,
        ),
        (
            DynamicInvocationCleanupRowKindV1::Fault,
            Some(I7),
            Some((I6, V10)),
            None,
        ),
        (
            DynamicInvocationCleanupRowKindV1::InnerReturn,
            None,
            Some((I6, V10)),
            None,
        ),
        (
            DynamicInvocationCleanupRowKindV1::Backedge,
            None,
            Some((I6, V10)),
            None,
        ),
    ];
    for (index, (row, (kind, item, first, second))) in rows.iter().zip(expected).enumerate() {
        if row.kind() != kind
            || row.item().map(|key| key.raw()) != item
            || action_pair(row.first()) != first
            || action_pair(row.second()) != second
        {
            return Err(SelectedDynamicV2PhysicalCapabilityRejectV1::CleanupOrder);
        }
        match index {
            2 if row.inner_return_site() != Some(&completion_sites[0]) => {
                return Err(SelectedDynamicV2PhysicalCapabilityRejectV1::CleanupOrder)
            }
            3 if row.backedge_loop() != Some(loop_key) => {
                return Err(SelectedDynamicV2PhysicalCapabilityRejectV1::CleanupOrder)
            }
            0..=1 if row.inner_return_site().is_some() || row.backedge_loop().is_some() => {
                return Err(SelectedDynamicV2PhysicalCapabilityRejectV1::CleanupOrder)
            }
            _ => {}
        }
    }
    Ok(())
}

fn action_pair(action: Option<DynamicV2DischargeActionRequirementV1>) -> Option<(u32, u32)> {
    action.map(|action| (action.producer().raw(), action.result().raw()))
}
