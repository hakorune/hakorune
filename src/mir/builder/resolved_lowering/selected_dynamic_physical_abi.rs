//! Builder-free preflight for the selected Dynamic V2 physical emitter.
//!
//! This module consumes the package-backed A-prime demand and records only a
//! family-native V2 schedule.  It does not open a session, allocate physical
//! IDs, or reinterpret a V2 row as a V1 operation.

use std::collections::BTreeSet;

use crate::mir::compiler::a_prime_i64_physical_capability::VerifiedAPrimeI64PhysicalDemandV1;
use crate::mir::compiler::dynamic_full_body_recipe::PreparedDynamicLoopOperationProgramV2;
use crate::mir::compiler::dynamic_full_body_source::DynamicFullBodySourceRoleV1;
use crate::mir::loop_recipe_contract::{LoopExitKindV2, LoopItemKeyV1, LoopOperationV2};

const EXPECTED_OPERATION_COUNT: usize = 15;
const EXPECTED_PLACEMENT_COUNT: usize = 17;
const EXPECTED_CONTROL_COUNT: usize = 1;
const EXPECTED_FAULT_COUNT: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum SelectedDynamicV2PhysicalPlanRejectV1 {
    Coverage,
    OperationOrder,
    UnsupportedSourceRole,
    OperationCallRelation,
    ControlShape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum DynamicV2PhysicalScheduleSegmentV1 {
    Prelude,
    ThenTerminal,
    Continuation,
    StepBackedge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) struct DynamicV2PhysicalScheduleRowV1 {
    item: LoopItemKeyV1,
    segment: DynamicV2PhysicalScheduleSegmentV1,
}

impl DynamicV2PhysicalScheduleRowV1 {
    pub(in crate::mir) const fn item(self) -> LoopItemKeyV1 {
        self.item
    }

    pub(in crate::mir) const fn segment(self) -> DynamicV2PhysicalScheduleSegmentV1 {
        self.segment
    }
}

/// Move-only, Builder-free plan for one selected Dynamic V2 cohort.
#[derive(Debug)]
pub(in crate::mir) struct PreparedSelectedDynamicV2EmissionPlanV1<'program> {
    demand: VerifiedAPrimeI64PhysicalDemandV1<'program>,
    schedule: Box<[DynamicV2PhysicalScheduleRowV1]>,
}

impl PreparedSelectedDynamicV2EmissionPlanV1<'_> {
    pub(in crate::mir) fn schedule_rows(&self) -> &[DynamicV2PhysicalScheduleRowV1] {
        &self.schedule
    }

    pub(in crate::mir) fn with_operation_program<R>(
        &self,
        callback: impl FnOnce(&PreparedDynamicLoopOperationProgramV2<'_>) -> R,
    ) -> R {
        self.demand.with_operation_program(callback)
    }
}

pub(in crate::mir) fn issue_selected_dynamic_v2_emission_plan<'program>(
    demand: VerifiedAPrimeI64PhysicalDemandV1<'program>,
) -> Result<PreparedSelectedDynamicV2EmissionPlanV1<'program>, SelectedDynamicV2PhysicalPlanRejectV1>
{
    let schedule = demand.with_operation_program(build_schedule)?;
    Ok(PreparedSelectedDynamicV2EmissionPlanV1 { demand, schedule })
}

fn build_schedule(
    program: &PreparedDynamicLoopOperationProgramV2<'_>,
) -> Result<Box<[DynamicV2PhysicalScheduleRowV1]>, SelectedDynamicV2PhysicalPlanRejectV1> {
    let coverage = program.coverage();
    if coverage.operation_count() != EXPECTED_OPERATION_COUNT
        || coverage.placement_count() != EXPECTED_PLACEMENT_COUNT
        || coverage.control_count() != EXPECTED_CONTROL_COUNT
        || coverage.fault_count() != EXPECTED_FAULT_COUNT
    {
        return Err(SelectedDynamicV2PhysicalPlanRejectV1::Coverage);
    }

    validate_control(program)?;

    let mut seen = BTreeSet::new();
    let mut rows = Vec::with_capacity(EXPECTED_OPERATION_COUNT);
    for operation in program.operation_rows() {
        if !seen.insert(operation.item()) {
            return Err(SelectedDynamicV2PhysicalPlanRejectV1::OperationOrder);
        }
        let segment = segment_for_role(operation.source_role())
            .ok_or(SelectedDynamicV2PhysicalPlanRejectV1::UnsupportedSourceRole)?;
        let is_call = matches!(operation.operation(), LoopOperationV2::CallSlot { .. });
        if is_call != operation.call_role().is_some() {
            return Err(SelectedDynamicV2PhysicalPlanRejectV1::OperationCallRelation);
        }
        rows.push(DynamicV2PhysicalScheduleRowV1 {
            item: operation.item(),
            segment,
        });
    }
    if rows.len() != EXPECTED_OPERATION_COUNT {
        return Err(SelectedDynamicV2PhysicalPlanRejectV1::OperationOrder);
    }
    Ok(rows.into_boxed_slice())
}

fn segment_for_role(
    role: DynamicFullBodySourceRoleV1,
) -> Option<DynamicV2PhysicalScheduleSegmentV1> {
    match role {
        DynamicFullBodySourceRoleV1::LoopConditionI
        | DynamicFullBodySourceRoleV1::LoopCondition
        | DynamicFullBodySourceRoleV1::SubstringStartI
        | DynamicFullBodySourceRoleV1::SubstringEndI
        | DynamicFullBodySourceRoleV1::SubstringEndDelta
        | DynamicFullBodySourceRoleV1::SubstringEndAdd
        | DynamicFullBodySourceRoleV1::SubstringCall
        | DynamicFullBodySourceRoleV1::IndexOfCall
        | DynamicFullBodySourceRoleV1::InnerIfZero
        | DynamicFullBodySourceRoleV1::InnerIfCondition => {
            Some(DynamicV2PhysicalScheduleSegmentV1::Prelude)
        }
        DynamicFullBodySourceRoleV1::InnerReturnI => {
            Some(DynamicV2PhysicalScheduleSegmentV1::ThenTerminal)
        }
        DynamicFullBodySourceRoleV1::StepReadI
        | DynamicFullBodySourceRoleV1::StepDelta
        | DynamicFullBodySourceRoleV1::StepAdd
        | DynamicFullBodySourceRoleV1::StepTargetI => {
            Some(DynamicV2PhysicalScheduleSegmentV1::Continuation)
        }
        _ => None,
    }
}

fn validate_control(
    program: &PreparedDynamicLoopOperationProgramV2<'_>,
) -> Result<(), SelectedDynamicV2PhysicalPlanRejectV1> {
    let control = program.control();
    if control.rows().len() != EXPECTED_CONTROL_COUNT
        || control.logical().branches().len() != EXPECTED_CONTROL_COUNT
    {
        return Err(SelectedDynamicV2PhysicalPlanRejectV1::ControlShape);
    }
    let row = &control.rows()[0];
    let branch = row
        .branches()
        .first()
        .ok_or(SelectedDynamicV2PhysicalPlanRejectV1::ControlShape)?;
    if branch.else_block().is_some()
        || !matches!(
            branch.then_arm().exit_kind(),
            Some(LoopExitKindV2::Return { .. })
        )
        || branch.else_arm().exit_kind().is_some()
    {
        return Err(SelectedDynamicV2PhysicalPlanRejectV1::ControlShape);
    }
    Ok(())
}
