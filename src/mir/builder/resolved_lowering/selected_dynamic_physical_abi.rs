//! Builder-free preflight for the selected Dynamic V2 physical emitter.
//!
//! This module consumes the package-backed A-prime demand and records only a
//! family-native V2 schedule.  It does not open a session, allocate physical
//! IDs, or reinterpret a V2 row as a V1 operation.

use std::collections::BTreeSet;

use crate::mir::compiler::a_prime_i64_physical_capability::VerifiedAPrimeI64PhysicalDemandV1;
use crate::mir::compiler::dynamic_full_body_recipe::{
    DynamicFullLoopFaultFamilyV2, DynamicFullLoopOperationEffectV2,
    DynamicFullLoopOperationPhysicalRefV2, DynamicFullLoopPhysicalItemKindV2,
    DynamicFullLoopPhysicalItemPlacementV2, DynamicLoopPhysicalArmV2,
    DynamicLoopPhysicalControlViewV2, PreparedDynamicLoopOperationProgramV2,
};
use crate::mir::compiler::dynamic_full_body_source::DynamicFullBodySourceRoleV1;
use crate::mir::loop_recipe_contract::{
    LoopBlockKeyV1, LoopConditionV2, LoopExitKindV2, LoopItemKeyV1, LoopNodeKeyV1,
    LoopOperationExecutionClassV2, LoopOperationV2, LoopValueKeyV1,
};
use crate::mir::resolved_semantics::{SourceExprSiteV1, SourceStmtSiteV1};

const EXPECTED_OPERATION_COUNT: usize = 15;
const EXPECTED_PLACEMENT_COUNT: usize = 17;
const EXPECTED_CONTROL_COUNT: usize = 1;
const EXPECTED_FAULT_COUNT: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum SelectedDynamicV2PhysicalPlanRejectV1 {
    Coverage,
    OperationOrder,
    PlacementShape,
    OperationCallRelation,
    ControlShape,
    I8Evidence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum DynamicV2PhysicalScheduleSegmentV1 {
    Prelude,
    ThenTerminal,
    Continuation,
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

/// Exact Builder-free handoff evidence for the first physical leaf.
#[derive(Debug, PartialEq, Eq)]
pub(super) struct DynamicV2I8EvidenceV1 {
    item: LoopItemKeyV1,
    result: LoopValueKeyV1,
    literal: i64,
    owner_loop: LoopNodeKeyV1,
    block: LoopBlockKeyV1,
    segment: DynamicV2PhysicalScheduleSegmentV1,
}

impl DynamicV2I8EvidenceV1 {
    pub(super) const fn item(&self) -> LoopItemKeyV1 {
        self.item
    }

    pub(super) const fn result(&self) -> LoopValueKeyV1 {
        self.result
    }

    pub(super) const fn literal(&self) -> i64 {
        self.literal
    }

    pub(super) const fn owner_loop(&self) -> LoopNodeKeyV1 {
        self.owner_loop
    }

    pub(super) const fn block(&self) -> LoopBlockKeyV1 {
        self.block
    }

    pub(super) const fn segment(&self) -> DynamicV2PhysicalScheduleSegmentV1 {
        self.segment
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DynamicV2PlacementEvidenceV1 {
    item: LoopItemKeyV1,
    owner_loop: LoopNodeKeyV1,
    block: LoopBlockKeyV1,
    kind: DynamicFullLoopPhysicalItemKindV2,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DynamicV2OperationEvidenceV1 {
    item: LoopItemKeyV1,
    owner_loop: LoopNodeKeyV1,
    block: LoopBlockKeyV1,
    source_role: DynamicFullBodySourceRoleV1,
    source_site: SourceExprSiteV1,
    effect: DynamicFullLoopOperationEffectV2,
    execution: LoopOperationExecutionClassV2,
    call_role: Option<DynamicFullBodySourceRoleV1>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DynamicV2ControlEvidenceV1 {
    loop_key: LoopNodeKeyV1,
    owner_block: LoopBlockKeyV1,
    if_item: LoopItemKeyV1,
    condition: LoopValueKeyV1,
    then_block: LoopBlockKeyV1,
    else_block: Option<LoopBlockKeyV1>,
    then_exit: Option<LoopExitKindV2>,
    then_exit_item: Option<LoopItemKeyV1>,
    else_exit: Option<LoopExitKindV2>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DynamicV2FaultEvidenceV1 {
    item: LoopItemKeyV1,
    family: DynamicFullLoopFaultFamilyV2,
    normal_result: LoopValueKeyV1,
}

/// Private evidence ledger for the preflight plan.  It copies only the
/// already co-sealed identities; it never becomes a second source/Recipe or
/// JoinSig authority.  Session emission later consumes this ledger exactly
/// once and adds session-local receipts in a child product.
#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir) struct DynamicV2NativePreflightLedgerV1 {
    placements: Box<[DynamicV2PlacementEvidenceV1]>,
    operations: Box<[DynamicV2OperationEvidenceV1]>,
    control: DynamicV2ControlEvidenceV1,
    faults: Box<[DynamicV2FaultEvidenceV1]>,
    i8: Option<DynamicV2I8EvidenceV1>,
    completion_sites: [SourceStmtSiteV1; 2],
    inner_return_value: LoopValueKeyV1,
    outer_tail_binding: crate::mir::loop_recipe_contract::LoopBindingKeyV1,
}

impl DynamicV2NativePreflightLedgerV1 {
    fn issue(
        program: &PreparedDynamicLoopOperationProgramV2<'_>,
        source_relation: &crate::mir::compiler::dynamic_full_body_recipe::
            DynamicAPrimeI64SourceRelationViewV1<'_>,
        schedule: &[DynamicV2PhysicalScheduleRowV1],
    ) -> Result<Self, SelectedDynamicV2PhysicalPlanRejectV1> {
        let placements = program
            .placement_rows()
            .iter()
            .map(|row| DynamicV2PlacementEvidenceV1 {
                item: row.item(),
                owner_loop: row.owner_loop(),
                block: row.block(),
                kind: row.kind(),
            })
            .collect::<Vec<_>>();
        if placements.len() != EXPECTED_PLACEMENT_COUNT {
            return Err(SelectedDynamicV2PhysicalPlanRejectV1::Coverage);
        }

        let operations = program
            .operation_rows()
            .iter()
            .map(|row| DynamicV2OperationEvidenceV1 {
                item: row.item(),
                owner_loop: row.owner_loop(),
                block: row.block(),
                source_role: row.source_role(),
                source_site: row.source_site().clone(),
                effect: row.effect(),
                execution: row.execution(),
                call_role: row.call_role(),
            })
            .collect::<Vec<_>>();
        if operations.len() != EXPECTED_OPERATION_COUNT {
            return Err(SelectedDynamicV2PhysicalPlanRejectV1::OperationOrder);
        }

        let control = program.control();
        let control_row = control
            .rows()
            .first()
            .ok_or(SelectedDynamicV2PhysicalPlanRejectV1::ControlShape)?;
        let branch = control_row
            .branches()
            .first()
            .ok_or(SelectedDynamicV2PhysicalPlanRejectV1::ControlShape)?;
        let control_evidence = DynamicV2ControlEvidenceV1 {
            loop_key: control_row.loop_key(),
            owner_block: branch.owner_block(),
            if_item: branch.if_item(),
            condition: branch.condition(),
            then_block: branch.then_block(),
            else_block: branch.else_block(),
            then_exit: branch.then_arm().exit_kind(),
            then_exit_item: match branch.then_arm() {
                crate::mir::compiler::dynamic_full_body_recipe::DynamicLoopPhysicalArmV2::Exit {
                    item, ..
                } => Some(item),
                crate::mir::compiler::dynamic_full_body_recipe::DynamicLoopPhysicalArmV2::Fallthrough => None,
            },
            else_exit: branch.else_arm().exit_kind(),
        };

        let faults = program
            .faults()
            .rows()
            .iter()
            .map(|row| DynamicV2FaultEvidenceV1 {
                item: row.item(),
                family: row.family(),
                normal_result: row.normal_result(),
            })
            .collect::<Vec<_>>();
        if faults.len() != EXPECTED_FAULT_COUNT {
            return Err(SelectedDynamicV2PhysicalPlanRejectV1::Coverage);
        }

        let i8 = issue_i8_evidence(program, &placements, schedule)?;

        let completion = source_relation.completion_sites();
        let completion_sites = [completion[0].clone(), completion[1].clone()];
        Ok(Self {
            placements: placements.into_boxed_slice(),
            operations: operations.into_boxed_slice(),
            control: control_evidence,
            faults: faults.into_boxed_slice(),
            i8: Some(i8),
            completion_sites,
            inner_return_value: source_relation.inner_return_value(),
            outer_tail_binding: source_relation.outer_tail_binding(),
        })
    }

    pub(in crate::mir) fn coverage_counts(&self) -> (usize, usize, usize, usize) {
        (
            self.placements.len(),
            self.operations.len(),
            self.faults.len(),
            self.completion_sites.len(),
        )
    }

    pub(super) fn take_i8_evidence(&mut self) -> Option<DynamicV2I8EvidenceV1> {
        self.i8.take()
    }
}

/// Move-only, Builder-free plan for one selected Dynamic V2 cohort.
#[derive(Debug)]
pub(in crate::mir) struct PreparedSelectedDynamicV2EmissionPlanV1<'program> {
    demand: VerifiedAPrimeI64PhysicalDemandV1<'program>,
    schedule: Box<[DynamicV2PhysicalScheduleRowV1]>,
    ledger: DynamicV2NativePreflightLedgerV1,
}

impl<'program> PreparedSelectedDynamicV2EmissionPlanV1<'program> {
    pub(in crate::mir) fn schedule_rows(&self) -> &[DynamicV2PhysicalScheduleRowV1] {
        &self.schedule
    }

    pub(in crate::mir) fn with_operation_program<R>(
        &self,
        callback: impl FnOnce(&PreparedDynamicLoopOperationProgramV2<'_>) -> R,
    ) -> R {
        self.demand.with_operation_program(callback)
    }

    #[cfg(test)]
    pub(in crate::mir) fn with_ledger<R>(
        &self,
        callback: impl FnOnce(&DynamicV2NativePreflightLedgerV1) -> R,
    ) -> R {
        callback(&self.ledger)
    }

    pub(in crate::mir) fn with_cleanup_physical_rows<R>(
        &self,
        callback: impl FnOnce(
            [crate::mir::compiler::dynamic_full_body_recipe::DynamicInvocationCleanupRowViewV1; 4],
        ) -> R,
    ) -> R {
        self.demand.with_cleanup_physical_rows(callback)
    }

    pub(in crate::mir) fn completion_sites(
        &self,
    ) -> Option<[crate::mir::resolved_semantics::SourceStmtSiteV1; 2]> {
        self.demand.completion_sites()
    }

    /// Consume the preflight plan into the family-native physical session.
    /// The session is the only non-test owner allowed to move the ledger.
    pub(super) fn into_emitter_parts(
        self,
    ) -> (
        VerifiedAPrimeI64PhysicalDemandV1<'program>,
        Box<[DynamicV2PhysicalScheduleRowV1]>,
        DynamicV2NativePreflightLedgerV1,
    ) {
        (self.demand, self.schedule, self.ledger)
    }
}

pub(in crate::mir) fn issue_selected_dynamic_v2_emission_plan<'program>(
    demand: VerifiedAPrimeI64PhysicalDemandV1<'program>,
) -> Result<PreparedSelectedDynamicV2EmissionPlanV1<'program>, SelectedDynamicV2PhysicalPlanRejectV1>
{
    let source_relation = demand.source_relation();
    let (schedule, ledger) = demand.with_operation_program(|program| {
        let schedule = build_schedule(program)?;
        let ledger = DynamicV2NativePreflightLedgerV1::issue(program, source_relation, &schedule)?;
        Ok::<_, SelectedDynamicV2PhysicalPlanRejectV1>((schedule, ledger))
    })?;
    Ok(PreparedSelectedDynamicV2EmissionPlanV1 {
        demand,
        schedule,
        ledger,
    })
}

fn issue_i8_evidence(
    program: &PreparedDynamicLoopOperationProgramV2<'_>,
    placements: &[DynamicV2PlacementEvidenceV1],
    schedule: &[DynamicV2PhysicalScheduleRowV1],
) -> Result<DynamicV2I8EvidenceV1, SelectedDynamicV2PhysicalPlanRejectV1> {
    const I8: u32 = 8;
    const V12: u32 = 12;
    let rows = program
        .operation_rows()
        .iter()
        .filter(|row| row.item() == LoopItemKeyV1::new(I8))
        .collect::<Vec<_>>();
    let [row] = rows.as_slice() else {
        return Err(SelectedDynamicV2PhysicalPlanRejectV1::I8Evidence);
    };
    let (result, literal) = match row.operation() {
        LoopOperationV2::ConstI64 { result, value } => (*result, *value),
        _ => return Err(SelectedDynamicV2PhysicalPlanRejectV1::I8Evidence),
    };
    if result != LoopValueKeyV1::new(V12) || literal != 0 {
        return Err(SelectedDynamicV2PhysicalPlanRejectV1::I8Evidence);
    }
    let placements = placements
        .iter()
        .filter(|placement| placement.item == LoopItemKeyV1::new(I8))
        .collect::<Vec<_>>();
    let [placement] = placements.as_slice() else {
        return Err(SelectedDynamicV2PhysicalPlanRejectV1::I8Evidence);
    };
    if placement.kind != DynamicFullLoopPhysicalItemKindV2::Operation {
        return Err(SelectedDynamicV2PhysicalPlanRejectV1::I8Evidence);
    }
    let segments = schedule
        .iter()
        .filter(|row| row.item == LoopItemKeyV1::new(I8))
        .map(|row| (*row).segment())
        .collect::<Vec<_>>();
    let [segment] = segments.as_slice() else {
        return Err(SelectedDynamicV2PhysicalPlanRejectV1::I8Evidence);
    };
    let segment = *segment;
    if segment != DynamicV2PhysicalScheduleSegmentV1::Prelude
        || placement.owner_loop != row.owner_loop()
        || placement.block != row.block()
    {
        return Err(SelectedDynamicV2PhysicalPlanRejectV1::I8Evidence);
    }
    Ok(DynamicV2I8EvidenceV1 {
        item: row.item(),
        result,
        literal,
        owner_loop: row.owner_loop(),
        block: row.block(),
        segment,
    })
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
    let control = program.control();
    for operation in program.operation_rows() {
        if !seen.insert(operation.item()) {
            return Err(SelectedDynamicV2PhysicalPlanRejectV1::OperationOrder);
        }
        let segment = segment_for_operation(operation, control, program.placement_rows())?;
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

fn segment_for_operation(
    operation: &DynamicFullLoopOperationPhysicalRefV2<'_>,
    control: &DynamicLoopPhysicalControlViewV2<'_>,
    placements: &[DynamicFullLoopPhysicalItemPlacementV2],
) -> Result<DynamicV2PhysicalScheduleSegmentV1, SelectedDynamicV2PhysicalPlanRejectV1> {
    let row = control
        .rows()
        .first()
        .ok_or(SelectedDynamicV2PhysicalPlanRejectV1::ControlShape)?;
    let branch = row
        .branches()
        .first()
        .ok_or(SelectedDynamicV2PhysicalPlanRejectV1::ControlShape)?;
    let if_placement = placements
        .iter()
        .find(|placement| placement.item() == branch.if_item())
        .ok_or(SelectedDynamicV2PhysicalPlanRejectV1::PlacementShape)?;
    if if_placement.owner_loop() != row.loop_key()
        || if_placement.block() != branch.owner_block()
        || if_placement.kind() != DynamicFullLoopPhysicalItemKindV2::If
    {
        return Err(SelectedDynamicV2PhysicalPlanRejectV1::PlacementShape);
    }
    let then_exit_item = match branch.then_arm() {
        DynamicLoopPhysicalArmV2::Exit {
            item,
            kind: LoopExitKindV2::Return { .. },
        } => item,
        _ => return Err(SelectedDynamicV2PhysicalPlanRejectV1::ControlShape),
    };
    let then_exit_placement = placements
        .iter()
        .find(|placement| placement.item() == then_exit_item)
        .ok_or(SelectedDynamicV2PhysicalPlanRejectV1::PlacementShape)?;
    if then_exit_placement.owner_loop() != row.loop_key()
        || then_exit_placement.block() != branch.then_block()
        || then_exit_placement.kind() != DynamicFullLoopPhysicalItemKindV2::Exit
    {
        return Err(SelectedDynamicV2PhysicalPlanRejectV1::PlacementShape);
    }
    let if_order = placements
        .iter()
        .position(|placement| placement.item() == branch.if_item())
        .ok_or(SelectedDynamicV2PhysicalPlanRejectV1::PlacementShape)?;
    let then_exit_order = placements
        .iter()
        .position(|placement| placement.item() == then_exit_item)
        .ok_or(SelectedDynamicV2PhysicalPlanRejectV1::PlacementShape)?;
    if then_exit_order <= if_order {
        return Err(SelectedDynamicV2PhysicalPlanRejectV1::PlacementShape);
    }
    if operation.owner_loop() != row.loop_key() {
        return Err(SelectedDynamicV2PhysicalPlanRejectV1::PlacementShape);
    }
    let operation_placement = placements
        .iter()
        .find(|placement| placement.item() == operation.item())
        .ok_or(SelectedDynamicV2PhysicalPlanRejectV1::PlacementShape)?;
    if operation_placement.owner_loop() != row.loop_key()
        || operation_placement.block() != operation.block()
        || operation_placement.kind() != DynamicFullLoopPhysicalItemKindV2::Operation
    {
        return Err(SelectedDynamicV2PhysicalPlanRejectV1::PlacementShape);
    }
    let operation_order = placements
        .iter()
        .position(|placement| placement.item() == operation.item())
        .ok_or(SelectedDynamicV2PhysicalPlanRejectV1::PlacementShape)?;
    if let LoopConditionV2::Predicate { block, .. } = row.condition() {
        if operation.block() == block {
            return if operation_order < if_order {
                Ok(DynamicV2PhysicalScheduleSegmentV1::Prelude)
            } else {
                Err(SelectedDynamicV2PhysicalPlanRejectV1::PlacementShape)
            };
        }
    }
    if operation.block() == branch.then_block() {
        return if operation_order < then_exit_order {
            Ok(DynamicV2PhysicalScheduleSegmentV1::ThenTerminal)
        } else {
            Err(SelectedDynamicV2PhysicalPlanRejectV1::PlacementShape)
        };
    }
    if operation.block() != branch.owner_block() {
        return Err(SelectedDynamicV2PhysicalPlanRejectV1::PlacementShape);
    }
    if operation_order < if_order {
        Ok(DynamicV2PhysicalScheduleSegmentV1::Prelude)
    } else if operation_order > if_order {
        Ok(DynamicV2PhysicalScheduleSegmentV1::Continuation)
    } else {
        Err(SelectedDynamicV2PhysicalPlanRejectV1::PlacementShape)
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
