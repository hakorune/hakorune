//! CheckedCallOut corridor emission over the already-admitted session facts.

use super::super::formal_header::DynamicV2OpenedFormalHeaderV1;
use super::super::i8_i9_control;
use super::super::operation_cursor::DynamicV2PhysicalOperationCensusV1;
use super::super::targets::{
    DynamicV2OpaquePhysicalTargetV1, DynamicV2PhysicalTargetRoleV1, DynamicV2PhysicalTargetSetV1,
};
use super::super::value_ledger::DynamicV2PhysicalValueLedgerV1;
use super::super::{DynamicV2I8EmitterRejectV1, DynamicV2PhysicalSessionBrandV1};
use super::{reject, DynamicV2CallOutCorridorV1, DynamicV2InstalledCallOutSitesV1};
use crate::mir::builder::calls::CanonicalFunctionLoweringSessionV1;
use crate::mir::builder::emission::{constant, loop_operation};
use crate::mir::builder::resolved_lowering::canonical_ssa::CanonicalSsaFunctionSessionV2;
use crate::mir::builder::resolved_lowering::selected_dynamic_physical_abi::DynamicV2I8EvidenceV1;
use crate::mir::builder::resolved_lowering::selected_dynamic_physical_capability::{
    DynamicV2CompareI64CapabilityDemandV1, DynamicV2PhysicalRepresentationV1,
};
use crate::mir::checked_callout::{CheckedCallOutNormalShapeV1, CheckedCallOutSiteIdV1};
use crate::mir::compiler::a_prime_i64_physical_capability::VerifiedAPrimeI64PhysicalDemandV1;
use crate::mir::compiler::dynamic_full_body_recipe::PreparedDynamicLoopOperationProgramV2;
use crate::mir::compiler::dynamic_full_body_source::DynamicFullBodySourceRoleV1;
use crate::mir::loop_recipe_contract::{
    LoopBinaryI64OpV2, LoopCompareI64OpV2, LoopOperationV2, LoopValueKeyV1,
};
use crate::mir::CompareOp;

const V0: LoopValueKeyV1 = LoopValueKeyV1::new(0);
const V2: LoopValueKeyV1 = LoopValueKeyV1::new(2);
const V3: LoopValueKeyV1 = LoopValueKeyV1::new(3);
const V4: LoopValueKeyV1 = LoopValueKeyV1::new(4);
const V5: LoopValueKeyV1 = LoopValueKeyV1::new(5);
const V6: LoopValueKeyV1 = LoopValueKeyV1::new(6);
const V7: LoopValueKeyV1 = LoopValueKeyV1::new(7);
const V8: LoopValueKeyV1 = LoopValueKeyV1::new(8);
const V9: LoopValueKeyV1 = LoopValueKeyV1::new(9);
const V10: LoopValueKeyV1 = LoopValueKeyV1::new(10);
const V11: LoopValueKeyV1 = LoopValueKeyV1::new(11);

pub(in crate::mir::builder::resolved_lowering::selected_dynamic_physical_emitter) fn require_read(
    row: &crate::mir::compiler::dynamic_full_body_recipe::DynamicFullLoopOperationPhysicalRefV2<'_>,
    expected: LoopValueKeyV1,
    induction: crate::mir::loop_recipe_contract::LoopBindingKeyV1,
) -> Result<(), DynamicV2I8EmitterRejectV1> {
    match row.operation() {
        LoopOperationV2::ReadBinding { binding, result }
            if *binding == induction && *result == expected =>
        {
            Ok(())
        }
        _ => Err(reject(format!(
            "Recipe read row drift item={:?} expected={expected:?}",
            row.item()
        ))),
    }
}

pub(in crate::mir::builder::resolved_lowering::selected_dynamic_physical_emitter) fn require_const(
    row: &crate::mir::compiler::dynamic_full_body_recipe::DynamicFullLoopOperationPhysicalRefV2<'_>,
    expected: LoopValueKeyV1,
    literal: i64,
) -> Result<(), DynamicV2I8EmitterRejectV1> {
    match row.operation() {
        LoopOperationV2::ConstI64 { result, value } if *result == expected && *value == literal => {
            Ok(())
        }
        _ => Err(reject(format!(
            "Recipe const row drift item={:?} expected={expected:?}",
            row.item()
        ))),
    }
}

pub(in crate::mir::builder::resolved_lowering::selected_dynamic_physical_emitter) fn require_add(
    row: &crate::mir::compiler::dynamic_full_body_recipe::DynamicFullLoopOperationPhysicalRefV2<'_>,
    expected_left: LoopValueKeyV1,
    expected_right: LoopValueKeyV1,
    expected_result: LoopValueKeyV1,
) -> Result<(), DynamicV2I8EmitterRejectV1> {
    match row.operation() {
        LoopOperationV2::BinaryI64 {
            op: LoopBinaryI64OpV2::Add,
            left,
            right,
            result,
        } if *left == expected_left && *right == expected_right && *result == expected_result => {
            Ok(())
        }
        _ => Err(reject(format!(
            "Recipe add row drift item={:?}",
            row.item()
        ))),
    }
}

pub(in crate::mir::builder::resolved_lowering::selected_dynamic_physical_emitter) fn require_compare(
    row: &crate::mir::compiler::dynamic_full_body_recipe::DynamicFullLoopOperationPhysicalRefV2<'_>,
    expected_left: LoopValueKeyV1,
    expected_right: LoopValueKeyV1,
    expected_result: LoopValueKeyV1,
) -> Result<(), DynamicV2I8EmitterRejectV1> {
    match row.operation() {
        LoopOperationV2::CompareI64 {
            op: LoopCompareI64OpV2::Less,
            left,
            right,
            result,
        } if *left == expected_left && *right == expected_right && *result == expected_result => {
            Ok(())
        }
        _ => Err(reject(format!(
            "Recipe compare row drift item={:?}",
            row.item()
        ))),
    }
}

fn require_call(
    row: &crate::mir::compiler::dynamic_full_body_recipe::DynamicFullLoopOperationPhysicalRefV2<'_>,
    role: DynamicFullBodySourceRoleV1,
    receiver: LoopValueKeyV1,
    args: &[LoopValueKeyV1],
    result: LoopValueKeyV1,
) -> Result<(), DynamicV2I8EmitterRejectV1> {
    if row.call_role() != Some(role) {
        return Err(reject(format!(
            "Recipe call role drift item={:?}",
            row.item()
        )));
    }
    match row.operation() {
        LoopOperationV2::CallSlot {
            receiver: Some(actual_receiver),
            args: actual_args,
            result: Some(actual_result),
        } if *actual_receiver == receiver
            && actual_args.as_slice() == args
            && *actual_result == result =>
        {
            Ok(())
        }
        _ => Err(reject(format!(
            "Recipe call shape drift item={:?}",
            row.item()
        ))),
    }
}

fn value(
    values: &DynamicV2PhysicalValueLedgerV1,
    result: LoopValueKeyV1,
    representation: DynamicV2PhysicalRepresentationV1,
) -> Result<crate::mir::ValueId, DynamicV2I8EmitterRejectV1> {
    values
        .with_value(result, representation, |view| view.value())
        .map_err(|error| reject(format!("physical value ledger: {error:?}")))
}

fn formal_value(
    formals: &DynamicV2OpenedFormalHeaderV1,
    result: LoopValueKeyV1,
) -> Result<crate::mir::ValueId, DynamicV2I8EmitterRejectV1> {
    formals
        .value_for_recipe(result)
        .ok_or_else(|| reject(format!("missing canonical formal value {result:?}")))
}

fn representation(shape: CheckedCallOutNormalShapeV1) -> DynamicV2PhysicalRepresentationV1 {
    match shape {
        CheckedCallOutNormalShapeV1::EndAuthorizedHandle { lease_slot } => {
            DynamicV2PhysicalRepresentationV1::EndAuthorizedHandle { lease_slot }
        }
        CheckedCallOutNormalShapeV1::ImmediateI64 => {
            DynamicV2PhysicalRepresentationV1::ImmediateI64
        }
    }
}

fn normal_result_type(
    shape: CheckedCallOutNormalShapeV1,
) -> Result<crate::mir::MirType, DynamicV2I8EmitterRejectV1> {
    let return_shape = match shape {
        CheckedCallOutNormalShapeV1::EndAuthorizedHandle { .. } => "string_handle",
        CheckedCallOutNormalShapeV1::ImmediateI64 => "ScalarI64",
    };
    crate::mir::route_value_type_publication::route_return_shape_value_type(Some(return_shape))
        .ok_or_else(|| reject(format!("missing canonical MIR type for {return_shape}")))
}

fn emit_jump(
    canonical: &mut CanonicalSsaFunctionSessionV2<'_>,
    outer: &mut CanonicalFunctionLoweringSessionV1<'_>,
    source: crate::mir::BasicBlockId,
    target: crate::mir::BasicBlockId,
) -> Result<(), DynamicV2I8EmitterRejectV1> {
    let function = outer
        .builder_view_mut_for_lowering()
        .function_state
        .current_function
        .as_mut()
        .ok_or_else(|| reject("missing function while emitting corridor jump"))?;
    canonical
        .cfg
        .emit_jump(function, source, target)
        .map_err(|error| reject(error.to_string()))
}

fn emit_header_branch(
    canonical: &mut CanonicalSsaFunctionSessionV2<'_>,
    outer: &mut CanonicalFunctionLoweringSessionV1<'_>,
    source: crate::mir::BasicBlockId,
    condition: crate::mir::ValueId,
    then_block: crate::mir::BasicBlockId,
    else_block: crate::mir::BasicBlockId,
) -> Result<(), DynamicV2I8EmitterRejectV1> {
    let function = outer
        .builder_view_mut_for_lowering()
        .function_state
        .current_function
        .as_mut()
        .ok_or_else(|| reject("missing function while emitting Header branch"))?;
    canonical
        .cfg
        .emit_branch(function, source, condition, then_block, else_block)
        .map_err(|error| reject(error.to_string()))
}

fn select_block(
    canonical: &mut CanonicalSsaFunctionSessionV2<'_>,
    outer: &mut CanonicalFunctionLoweringSessionV1<'_>,
    block: crate::mir::BasicBlockId,
) -> Result<(), DynamicV2I8EmitterRejectV1> {
    canonical
        .cfg
        .select_block(outer.builder_view_mut_for_lowering(), block)
        .map_err(|error| reject(error.to_string()))
}

fn emit_checked_callout(
    canonical: &mut CanonicalSsaFunctionSessionV2<'_>,
    outer: &mut CanonicalFunctionLoweringSessionV1<'_>,
    source: crate::mir::BasicBlockId,
    site: CheckedCallOutSiteIdV1,
    receiver: crate::mir::ValueId,
    arguments: Vec<crate::mir::ValueId>,
    normal: crate::mir::BasicBlockId,
    fault: crate::mir::BasicBlockId,
) -> Result<(), DynamicV2I8EmitterRejectV1> {
    let function = outer
        .builder_view_mut_for_lowering()
        .function_state
        .current_function
        .as_mut()
        .ok_or_else(|| reject("missing function while emitting CheckedCallOut"))?;
    canonical
        .cfg
        .emit_checked_callout(function, source, site, receiver, arguments, normal, fault)
        .map_err(|error| reject(error.to_string()))
}

fn publish_i64_alias(
    values: &mut DynamicV2PhysicalValueLedgerV1,
    row: &crate::mir::compiler::dynamic_full_body_recipe::DynamicFullLoopOperationPhysicalRefV2<'_>,
    result: LoopValueKeyV1,
    target: &DynamicV2OpaquePhysicalTargetV1,
    value: crate::mir::ValueId,
) -> Result<(), DynamicV2I8EmitterRejectV1> {
    values
        .publish(
            row.item(),
            result,
            target,
            value,
            DynamicV2PhysicalRepresentationV1::ImmediateI64,
        )
        .map_err(|error| reject(format!("physical value ledger: {error:?}")))
}

fn publish_i64(
    values: &mut DynamicV2PhysicalValueLedgerV1,
    row: &crate::mir::compiler::dynamic_full_body_recipe::DynamicFullLoopOperationPhysicalRefV2<'_>,
    result: LoopValueKeyV1,
    target: &DynamicV2OpaquePhysicalTargetV1,
    value: crate::mir::ValueId,
) -> Result<(), DynamicV2I8EmitterRejectV1> {
    publish_i64_alias(values, row, result, target, value)
}

pub(in crate::mir::builder::resolved_lowering::selected_dynamic_physical_emitter) fn emit(
    canonical: &mut CanonicalSsaFunctionSessionV2<'_>,
    outer: &mut CanonicalFunctionLoweringSessionV1<'_>,
    demand: &VerifiedAPrimeI64PhysicalDemandV1<'_>,
    targets: &DynamicV2PhysicalTargetSetV1,
    formals: &DynamicV2OpenedFormalHeaderV1,
    values: &mut DynamicV2PhysicalValueLedgerV1,
    brand: &DynamicV2PhysicalSessionBrandV1,
    sites: DynamicV2InstalledCallOutSitesV1,
    i8_evidence: DynamicV2I8EvidenceV1,
    compare_i64: DynamicV2CompareI64CapabilityDemandV1,
    operation_census: &mut DynamicV2PhysicalOperationCensusV1,
) -> Result<DynamicV2CallOutCorridorV1, DynamicV2I8EmitterRejectV1> {
    demand.with_operation_program(|program| {
        emit_program(
            canonical,
            outer,
            program,
            demand.source_relation(),
            targets,
            formals,
            values,
            brand,
            sites,
            i8_evidence,
            compare_i64,
            operation_census,
        )
    })
}

fn emit_program(
    canonical: &mut CanonicalSsaFunctionSessionV2<'_>,
    outer: &mut CanonicalFunctionLoweringSessionV1<'_>,
    program: &PreparedDynamicLoopOperationProgramV2<'_>,
    relation: &crate::mir::compiler::dynamic_full_body_recipe::DynamicAPrimeI64SourceRelationViewV1<
        '_,
    >,
    targets: &DynamicV2PhysicalTargetSetV1,
    formals: &DynamicV2OpenedFormalHeaderV1,
    values: &mut DynamicV2PhysicalValueLedgerV1,
    brand: &DynamicV2PhysicalSessionBrandV1,
    sites: DynamicV2InstalledCallOutSitesV1,
    i8_evidence: DynamicV2I8EvidenceV1,
    compare_i64: DynamicV2CompareI64CapabilityDemandV1,
    operation_census: &mut DynamicV2PhysicalOperationCensusV1,
) -> Result<DynamicV2CallOutCorridorV1, DynamicV2I8EmitterRejectV1> {
    let rows = program.operation_rows();
    if rows.len() != 15 {
        return Err(reject(
            "combined corridor requires exactly 15 operation rows",
        ));
    }

    let induction = relation.induction_key();
    let (header, body, after) =
        targets.with_role(DynamicV2PhysicalTargetRoleV1::Header, |header| {
            targets.with_role(DynamicV2PhysicalTargetRoleV1::BodyPrelude, |body| {
                targets.with_role(DynamicV2PhysicalTargetRoleV1::After, |after| {
                    (header, body, after)
                })
            })
        });
    let header_block = header.block();
    let body_block = body.block();
    let current = formals.header_current_value();
    let v2 = formal_value(formals, V2)?;
    canonical
        .identity
        .claim_variable_use_binding(relation.condition_i(), relation.induction_binding())
        .map_err(reject)?;
    canonical
        .identity
        .claim_variable_use_binding(relation.condition_end(), relation.end_binding())
        .map_err(reject)?;

    // Consume the existing operation census before the first MIR/ledger
    // physical effect in this corridor.  Identity observations above remain
    // covered by the unpublished outer-session discard contract.
    for raw in 0..8 {
        operation_census
            .claim_operation(crate::mir::loop_recipe_contract::LoopItemKeyV1::new(raw))
            .map_err(|error| reject(format!("physical operation claim {raw}: {error:?}")))?;
    }

    loop_operation::publish_i64_value(outer.builder_view_mut_for_lowering(), current)
        .map_err(reject)?;

    require_read(&rows[0], V4, induction)?;
    publish_i64(values, &rows[0], V4, &header, current)?;
    require_compare(&rows[1], V4, V2, V5)?;
    let v5 = canonical
        .issue_physical_value_id(outer.builder_view_mut_for_lowering())
        .map_err(reject)?;
    loop_operation::emit_compare_i64_at_with_dst(
        outer.builder_view_mut_for_lowering(),
        header_block,
        v5,
        CompareOp::Lt,
        current,
        v2,
    )
    .map_err(reject)?;
    canonical
        .publish_physical_value_type(
            outer.builder_view_mut_for_lowering(),
            v5,
            crate::mir::MirType::Bool,
        )
        .map_err(reject)?;
    values
        .publish(
            rows[1].item(),
            V5,
            &header,
            v5,
            DynamicV2PhysicalRepresentationV1::ImmediateBool,
        )
        .map_err(|error| reject(format!("physical value ledger: {error:?}")))?;
    if !after.matches(brand) {
        return Err(reject("After target has a foreign session brand"));
    }
    emit_header_branch(
        canonical,
        outer,
        header_block,
        v5,
        body_block,
        after.block(),
    )?;
    select_block(canonical, outer, body_block)?;

    require_read(&rows[2], V6, induction)?;
    publish_i64(values, &rows[2], V6, &body, current)?;
    require_read(&rows[3], V7, induction)?;
    publish_i64(values, &rows[3], V7, &body, current)?;
    require_const(&rows[4], V8, 1)?;
    let v8 = canonical
        .issue_physical_value_id(outer.builder_view_mut_for_lowering())
        .map_err(reject)?;
    constant::emit_integer_at_with_dst(outer.builder_view_mut_for_lowering(), body_block, v8, 1)
        .map_err(reject)?;
    values
        .publish(
            rows[4].item(),
            V8,
            &body,
            v8,
            DynamicV2PhysicalRepresentationV1::ImmediateI64,
        )
        .map_err(|error| reject(format!("physical value ledger: {error:?}")))?;
    require_add(&rows[5], V7, V8, V9)?;
    let v7 = value(values, V7, DynamicV2PhysicalRepresentationV1::ImmediateI64)?;
    let v9 = canonical
        .issue_physical_value_id(outer.builder_view_mut_for_lowering())
        .map_err(reject)?;
    loop_operation::emit_add_i64_at_with_dst(
        outer.builder_view_mut_for_lowering(),
        body_block,
        v9,
        v7,
        v8,
    )
    .map_err(reject)?;
    canonical
        .publish_physical_value_type(
            outer.builder_view_mut_for_lowering(),
            v9,
            crate::mir::MirType::Integer,
        )
        .map_err(reject)?;
    values
        .publish(
            rows[5].item(),
            V9,
            &body,
            v9,
            DynamicV2PhysicalRepresentationV1::ImmediateI64,
        )
        .map_err(|error| reject(format!("physical value ledger: {error:?}")))?;

    require_call(
        &rows[6],
        DynamicFullBodySourceRoleV1::SubstringCall,
        V0,
        &[V6, V9],
        V10,
    )?;
    require_call(
        &rows[7],
        DynamicFullBodySourceRoleV1::IndexOfCall,
        V3,
        &[V10],
        V11,
    )?;
    canonical
        .identity
        .claim_variable_use_binding(relation.substring_receiver(), relation.src_binding())
        .map_err(reject)?;
    canonical
        .identity
        .claim_variable_use_binding(relation.substring_start(), relation.induction_binding())
        .map_err(reject)?;
    canonical
        .identity
        .claim_variable_use_binding(relation.substring_end_i(), relation.induction_binding())
        .map_err(reject)?;

    let i6_normal = new_landing(canonical, outer, brand)?;
    let i6_fault = new_landing(canonical, outer, brand)?;
    let i7_normal = new_landing(canonical, outer, brand)?;
    let i7_fault = new_landing(canonical, outer, brand)?;
    let i6_result_type = if matches!(
        sites.i6_shape(),
        CheckedCallOutNormalShapeV1::EndAuthorizedHandle { .. }
    ) {
        normal_result_type(sites.i6_shape())?
    } else {
        return Err(reject("I6 site plan is not EndAuthorizedHandle"));
    };
    let i7_result_type = if matches!(sites.i7_shape(), CheckedCallOutNormalShapeV1::ImmediateI64) {
        normal_result_type(sites.i7_shape())?
    } else {
        return Err(reject("I7 site plan is not ImmediateI64"));
    };
    let substring_arg0 = value(values, V6, DynamicV2PhysicalRepresentationV1::ImmediateI64)?;
    let substring_arg1 = value(values, V9, DynamicV2PhysicalRepresentationV1::ImmediateI64)?;
    let i6_receiver = formal_value(formals, V0)?;
    emit_checked_callout(
        canonical,
        outer,
        body_block,
        sites.i6(),
        i6_receiver,
        vec![substring_arg0, substring_arg1],
        i6_normal.block(),
        i6_fault.block(),
    )?;
    select_block(canonical, outer, i6_normal.block())?;
    let i6_projection = canonical
        .define_checked_callout_normal_result(
            outer.builder_view_mut_for_lowering(),
            body_block,
            i6_normal.block(),
            sites.i6(),
            i6_result_type,
        )
        .map_err(reject)?;
    values
        .publish(
            rows[6].item(),
            V10,
            &i6_normal,
            i6_projection.dst(),
            representation(sites.i6_shape()),
        )
        .map_err(|error| reject(format!("physical value ledger: {error:?}")))?;
    let iteration_local = relation.iteration_local();
    canonical
        .identity
        .publish_declaration_exact(
            iteration_local.declaration(),
            iteration_local.binding(),
            i6_normal.block(),
            i6_projection.dst(),
        )
        .map_err(reject)?;

    let receiver = formal_value(formals, V3)?;
    let substring_value = value(values, V10, representation(sites.i6_shape()))?;
    canonical
        .identity
        .claim_variable_use_binding(relation.index_of_receiver(), relation.pred_chars_binding())
        .map_err(reject)?;
    canonical
        .identity
        .claim_variable_use_binding(iteration_local.read(), iteration_local.binding())
        .map_err(reject)?;
    emit_checked_callout(
        canonical,
        outer,
        i6_normal.block(),
        sites.i7(),
        receiver,
        vec![substring_value],
        i7_normal.block(),
        i7_fault.block(),
    )?;
    select_block(canonical, outer, i7_normal.block())?;
    let i7_projection = canonical
        .define_checked_callout_normal_result(
            outer.builder_view_mut_for_lowering(),
            i6_normal.block(),
            i7_normal.block(),
            sites.i7(),
            i7_result_type,
        )
        .map_err(reject)?;
    values
        .publish(
            rows[7].item(),
            V11,
            &i7_normal,
            i7_projection.dst(),
            representation(sites.i7_shape()),
        )
        .map_err(|error| reject(format!("physical value ledger: {error:?}")))?;

    let corridor = DynamicV2CallOutCorridorV1::new(
        sites.i6(),
        i6_receiver,
        [substring_arg0, substring_arg1],
        i6_projection.dst(),
        i6_normal,
        i6_fault,
        sites.i7(),
        formal_value(formals, V3)?,
        substring_value,
        i7_projection.dst(),
        i7_normal,
        i7_fault,
    );
    i8_i9_control::emit(
        canonical,
        outer,
        program,
        &corridor,
        targets,
        values,
        brand,
        i8_evidence,
        compare_i64,
        operation_census,
    )?;
    Ok(corridor)
}

fn new_landing(
    canonical: &mut CanonicalSsaFunctionSessionV2<'_>,
    outer: &mut CanonicalFunctionLoweringSessionV1<'_>,
    brand: &DynamicV2PhysicalSessionBrandV1,
) -> Result<DynamicV2OpaquePhysicalTargetV1, DynamicV2I8EmitterRejectV1> {
    let block = canonical
        .create_unpublished_block(outer.builder_view_mut_for_lowering())
        .map_err(reject)?;
    Ok(DynamicV2OpaquePhysicalTargetV1::for_block(brand, block))
}
