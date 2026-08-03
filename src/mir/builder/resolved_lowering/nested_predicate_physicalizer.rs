//! Candidate-only physicalizer for the bounded Nested Predicate shape.
//!
//! The topology owns placement; this box only allocates the verified blocks,
//! emits the fixed predicate/backedge graph, and borrows the canonical CFG,
//! identity adapter, and PHI transaction. It has no route or retry authority.

use std::collections::BTreeSet;

use crate::mir::builder::emission::loop_operation;
use crate::mir::builder::emission::phi_lifecycle::PhiTxn;
use crate::mir::builder::resolved_lowering::canonical_cfg::CanonicalCfgSessionV1;
use crate::mir::builder::resolved_lowering::nested_predicate_adapter::CanonicalNestedBindingPort;
use crate::mir::builder::MirBuilder;
use crate::mir::compiler::nested_predicate_effect_plan::NestedBindingEffectRoleV1;
use crate::mir::compiler::nested_predicate_physical_input::{
    VerifiedNestedPhysicalBlockProjectionV1, VerifiedNestedPhysicalCandidateInputV1,
};
use crate::mir::compiler::nested_predicate_topology::{
    NestedPhysicalNodeRefV1, NestedPhysicalSourceRoleV1, NestedPhysicalStageV1,
    VerifiedNestedPhysicalEmissionInputV1,
};
use crate::mir::{BasicBlockId, ValueId};

pub(in crate::mir::builder::resolved_lowering) struct NestedPhysicalContinuationV1 {
    pub(in crate::mir::builder::resolved_lowering) continuation_block: BasicBlockId,
}

pub(in crate::mir::builder::resolved_lowering) fn physicalize_nested_predicate_v1(
    builder: &mut MirBuilder,
    input: VerifiedNestedPhysicalCandidateInputV1,
    cfg: &mut CanonicalCfgSessionV1,
    port: &mut CanonicalNestedBindingPort<'_, '_>,
    phis: &mut PhiTxn,
) -> Result<NestedPhysicalContinuationV1, String> {
    let (emission, blocks) = input.into_parts();
    verify_placement(&emission)?;
    let topology = emission.topology();
    let root_preheader = block_at(topology, &blocks, 0, NestedPhysicalStageV1::Preheader)?;
    if builder.function_state.current_block != Some(root_preheader) {
        return Err("[freeze:contract][nested_physicalizer/preheader_mismatch]".into());
    }
    create_blocks(builder, cfg, &blocks, topology, root_preheader)?;

    let root_header = block_at(topology, &blocks, 0, NestedPhysicalStageV1::Header)?;
    let root_body = block_at(topology, &blocks, 0, NestedPhysicalStageV1::Body)?;
    let root_step = block_at(topology, &blocks, 0, NestedPhysicalStageV1::Step)?;
    let root_after = block_at(topology, &blocks, 0, NestedPhysicalStageV1::After)?;
    let child_header = block_at(topology, &blocks, 1, NestedPhysicalStageV1::Header)?;
    let child_body = block_at(topology, &blocks, 1, NestedPhysicalStageV1::Body)?;
    let child_step = block_at(topology, &blocks, 1, NestedPhysicalStageV1::Step)?;
    let child_after = block_at(topology, &blocks, 1, NestedPhysicalStageV1::After)?;
    let parent_resume = blocks.block(NestedPhysicalNodeRefV1::ParentResume(
        topology.parent_resume(),
    ));

    emit_jump(builder, cfg, root_preheader, root_header)?;
    seal(builder, cfg, port, phis, root_preheader)?;

    select(builder, cfg, root_header)?;
    let root_condition = emit_predicate(
        builder,
        port,
        phis,
        NestedBindingEffectRoleV1::RootPredicateReadI,
        root_header,
    )?;
    emit_branch_and_select(
        builder,
        cfg,
        root_header,
        root_condition,
        root_body,
        root_after,
    )?;

    select(builder, cfg, root_body)?;
    seal(builder, cfg, port, phis, root_body)?;
    port.write_first(
        builder,
        NestedBindingEffectRoleV1::ChildInitializeWriteJ,
        root_body,
        0,
    )?;
    emit_jump(builder, cfg, root_body, child_header)?;

    select(builder, cfg, child_header)?;
    let child_condition = emit_predicate(
        builder,
        port,
        phis,
        NestedBindingEffectRoleV1::ChildPredicateReadJ,
        child_header,
    )?;
    emit_branch_and_select(
        builder,
        cfg,
        child_header,
        child_condition,
        child_body,
        child_after,
    )?;

    select(builder, cfg, child_body)?;
    seal(builder, cfg, port, phis, child_body)?;
    let sum = port.read(
        NestedBindingEffectRoleV1::ChildAncestorReadSum,
        builder,
        phis,
        child_body,
    )?;
    port.write_delta(
        builder,
        NestedBindingEffectRoleV1::ChildAncestorWriteSum,
        child_body,
        sum,
    )?;
    emit_jump(builder, cfg, child_body, child_step)?;

    // The topology intentionally places the recurrence update in Child.Step,
    // after the ancestor update in Child.Body and before Step -> Header.
    select(builder, cfg, child_step)?;
    seal(builder, cfg, port, phis, child_step)?;
    let child_j = port.read(
        NestedBindingEffectRoleV1::ChildReadJ,
        builder,
        phis,
        child_step,
    )?;
    port.write_delta(
        builder,
        NestedBindingEffectRoleV1::ChildWriteJ,
        child_step,
        child_j,
    )?;
    emit_jump(builder, cfg, child_step, child_header)?;

    select(builder, cfg, child_header)?;
    seal(builder, cfg, port, phis, child_header)?;
    select(builder, cfg, child_after)?;
    seal(builder, cfg, port, phis, child_after)?;
    emit_jump(builder, cfg, child_after, parent_resume)?;

    select(builder, cfg, parent_resume)?;
    seal(builder, cfg, port, phis, parent_resume)?;
    emit_jump(builder, cfg, parent_resume, root_step)?;

    select(builder, cfg, root_step)?;
    seal(builder, cfg, port, phis, root_step)?;
    let root_i = port.read(
        NestedBindingEffectRoleV1::RootStepReadI,
        builder,
        phis,
        root_step,
    )?;
    port.write_delta(
        builder,
        NestedBindingEffectRoleV1::RootStepWriteI,
        root_step,
        root_i,
    )?;
    emit_jump(builder, cfg, root_step, root_header)?;

    select(builder, cfg, root_header)?;
    seal(builder, cfg, port, phis, root_header)?;
    select(builder, cfg, root_after)?;
    port.finish_effect_claims()?;
    Ok(NestedPhysicalContinuationV1 {
        continuation_block: root_after,
    })
}

fn emit_predicate(
    builder: &mut MirBuilder,
    port: &mut CanonicalNestedBindingPort<'_, '_>,
    phis: &mut PhiTxn,
    role: NestedBindingEffectRoleV1,
    block: BasicBlockId,
) -> Result<ValueId, String> {
    let value = port.read(role, builder, phis, block)?;
    let limit = loop_operation::emit_const_i64(builder, 3)?;
    loop_operation::emit_less_i64(builder, value, limit)
}

fn create_blocks(
    builder: &mut MirBuilder,
    cfg: &CanonicalCfgSessionV1,
    blocks: &VerifiedNestedPhysicalBlockProjectionV1,
    topology: &crate::mir::compiler::nested_predicate_topology::VerifiedNestedPhysicalTopologyV1,
    root_preheader: BasicBlockId,
) -> Result<(), String> {
    let mut seen = BTreeSet::new();
    let function = builder
        .function_state
        .current_function
        .as_mut()
        .ok_or_else(|| "[freeze:contract][nested_physicalizer/function_missing]".to_string())?;
    for port in topology.ports().iter().copied() {
        let block = blocks.block(NestedPhysicalNodeRefV1::Port(port));
        if block != root_preheader && seen.insert(block) {
            cfg.create_block(function, block).map_err(|error| {
                format!("[freeze:contract][nested_physicalizer/create] {error:?}")
            })?;
        }
    }
    let resume = blocks.block(NestedPhysicalNodeRefV1::ParentResume(
        topology.parent_resume(),
    ));
    if resume != root_preheader && seen.insert(resume) {
        cfg.create_block(function, resume)
            .map_err(|error| format!("[freeze:contract][nested_physicalizer/create] {error:?}"))?;
    }
    Ok(())
}

fn block_at(
    topology: &crate::mir::compiler::nested_predicate_topology::VerifiedNestedPhysicalTopologyV1,
    blocks: &VerifiedNestedPhysicalBlockProjectionV1,
    loop_key: u32,
    stage: NestedPhysicalStageV1,
) -> Result<BasicBlockId, String> {
    let port = topology
        .ports()
        .iter()
        .find(|port| port.loop_key.raw() == loop_key && port.stage == stage)
        .copied()
        .ok_or_else(|| "[freeze:contract][nested_physicalizer/port_missing]".to_string())?;
    Ok(blocks.block(NestedPhysicalNodeRefV1::Port(port)))
}

fn select(
    builder: &mut MirBuilder,
    cfg: &CanonicalCfgSessionV1,
    block: BasicBlockId,
) -> Result<(), String> {
    cfg.select_block(builder, block)
        .map_err(|error| format!("[freeze:contract][nested_physicalizer/select] {error:?}"))
}

fn emit_jump(
    builder: &mut MirBuilder,
    cfg: &CanonicalCfgSessionV1,
    source: BasicBlockId,
    target: BasicBlockId,
) -> Result<(), String> {
    let function = builder
        .function_state
        .current_function
        .as_mut()
        .ok_or_else(|| "[freeze:contract][nested_physicalizer/function_missing]".to_string())?;
    cfg.emit_jump(function, source, target)
        .map_err(|error| format!("[freeze:contract][nested_physicalizer/jump] {error:?}"))
}

fn emit_branch_and_select(
    builder: &mut MirBuilder,
    cfg: &CanonicalCfgSessionV1,
    source: BasicBlockId,
    condition: ValueId,
    then_block: BasicBlockId,
    else_block: BasicBlockId,
) -> Result<(), String> {
    let function = builder
        .function_state
        .current_function
        .as_mut()
        .ok_or_else(|| "[freeze:contract][nested_physicalizer/function_missing]".to_string())?;
    cfg.emit_branch(function, source, condition, then_block, else_block)
        .map_err(|error| format!("[freeze:contract][nested_physicalizer/branch] {error:?}"))
}

fn seal(
    builder: &mut MirBuilder,
    cfg: &mut CanonicalCfgSessionV1,
    port: &mut CanonicalNestedBindingPort<'_, '_>,
    phis: &mut PhiTxn,
    block: BasicBlockId,
) -> Result<(), String> {
    let function = builder
        .function_state
        .current_function
        .as_mut()
        .ok_or_else(|| "[freeze:contract][nested_physicalizer/function_missing]".to_string())?;
    let witness = cfg
        .seal_block(function, block)
        .map_err(|error| format!("[freeze:contract][nested_physicalizer/seal] {error:?}"))?;
    port.seal(builder, phis, block, &witness)
}

fn verify_placement(emission: &VerifiedNestedPhysicalEmissionInputV1) -> Result<(), String> {
    let topology = emission.topology();
    let child_update = topology
        .source_roles()
        .iter()
        .find(|row| matches!(row.role, NestedPhysicalSourceRoleV1::ChildUpdate(_)))
        .ok_or_else(|| {
            "[freeze:contract][nested_physicalizer/child_update_role_missing]".to_string()
        })?;
    let NestedPhysicalNodeRefV1::Port(port) = child_update.destination else {
        return Err("[freeze:contract][nested_physicalizer/child_update_destination]".into());
    };
    if port.loop_key.raw() != 1 || port.stage != NestedPhysicalStageV1::Step {
        return Err("[freeze:contract][nested_physicalizer/child_update_stage]".into());
    }
    Ok(())
}
