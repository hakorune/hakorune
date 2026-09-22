//! Physical key preservation through the existing allocation/PHI/exit owners.
//! These tests do not establish source admission or ledger publication.
use super::carriers::{
    allocate_generic_loop_v1_carriers, close_generic_loop_v1_carrier_phis,
    GenericLoopCarrierEntryV1,
};
use crate::mir::builder::control_flow::plan::generic_loop::carrier_representation::prepare_generic_loop_carrier_representation_v1;
use crate::mir::builder::control_flow::plan::generic_loop::facts_types::GenericLoopCarrierRoleV1;
use crate::mir::builder::control_flow::plan::parts::exit::build_continue_with_phi_args;
use crate::mir::builder::control_flow::plan::skeletons::generic_loop::{
    alloc_generic_loop_skeleton_from_representation, GenericLoopSkeleton,
};
use crate::mir::builder::control_flow::plan::CoreExitPlan;
use crate::mir::builder::MirBuilder;
use crate::mir::{MirType, ValueId};
use std::collections::BTreeMap;

fn skeleton() -> (MirBuilder, GenericLoopSkeleton) {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("keyed_carrier".to_owned());
    let entry = builder.alloc_typed(MirType::Integer);
    let poisoned = builder.alloc_typed(MirType::String);
    builder
        .function_state
        .variable_ctx
        .variable_map
        .insert("i".to_owned(), poisoned);
    let representation = prepare_generic_loop_carrier_representation_v1(
        GenericLoopCarrierRoleV1::NumericProgression,
        Some(entry),
        Some(&MirType::Integer),
    )
    .unwrap();
    let skeleton =
        alloc_generic_loop_skeleton_from_representation(&mut builder, "i", representation).unwrap();
    assert_eq!(skeleton.loop_var_init, entry);
    assert_eq!(
        builder.function_state.variable_ctx.variable_map["i"],
        poisoned
    );
    (builder, skeleton)
}

#[test]
fn keyed_carrier_allocation_keeps_equal_labels_as_distinct_keys() {
    let (mut builder, skeleton) = skeleton();
    let first = builder.alloc_typed(MirType::Integer);
    let second = builder.alloc_typed(MirType::String);
    let state = allocate_generic_loop_v1_carriers(
        &mut builder,
        0u32,
        skeleton.loop_var_current,
        &skeleton.carrier_representation,
        vec![
            GenericLoopCarrierEntryV1 {
                key: 1,
                label: "same".to_owned(),
                init: first,
                ty: MirType::Integer,
            },
            GenericLoopCarrierEntryV1 {
                key: 2,
                label: "same".to_owned(),
                init: second,
                ty: MirType::String,
            },
        ],
    );
    assert_eq!(state.phi_bindings.len(), 3);
    assert_ne!(state.phi_bindings[&1], state.phi_bindings[&2]);
    assert_eq!(
        builder
            .function_state
            .type_ctx
            .get_type(state.phi_bindings[&1]),
        Some(&MirType::Integer)
    );
    assert_eq!(
        builder
            .function_state
            .type_ctx
            .get_type(state.phi_bindings[&2]),
        Some(&MirType::String)
    );
    let incoming = BTreeMap::from([
        (0, ValueId::new(500)),
        (1, ValueId::new(501)),
        (2, ValueId::new(502)),
    ]);
    let CoreExitPlan::ContinueWithPhiArgs { depth, phi_args } =
        build_continue_with_phi_args(&builder, &state.carrier_step_phis, &incoming, "keyed-test")
            .unwrap()
    else {
        panic!("existing continue owner");
    };
    assert_eq!(depth, 1);
    for (key, dst) in &state.carrier_step_phis {
        assert!(phi_args.contains(&(*dst, incoming[key])));
    }
    let phis = close_generic_loop_v1_carrier_phis(
        &skeleton.plan,
        &state,
        "i",
        skeleton.loop_var_init,
        skeleton.loop_var_current,
        Some(&incoming),
    )
    .unwrap();
    for row in &state.carrier_infos {
        let step = phis.iter().find(|phi| phi.dst == row.step).unwrap();
        assert_eq!(
            step.inputs,
            vec![(skeleton.plan.body_bb, incoming[&row.key])]
        );
        let header = phis.iter().find(|phi| phi.dst == row.header).unwrap();
        assert_eq!(
            header.inputs,
            vec![
                (skeleton.plan.preheader_bb, row.init),
                (skeleton.plan.step_bb, row.step)
            ]
        );
    }
}

#[test]
fn keyed_carrier_missing_backedge_rejects_without_header_default() {
    let (mut builder, skeleton) = skeleton();
    let init = builder.alloc_typed(MirType::Integer);
    let state = allocate_generic_loop_v1_carriers(
        &mut builder,
        0u32,
        skeleton.loop_var_current,
        &skeleton.carrier_representation,
        vec![GenericLoopCarrierEntryV1 {
            key: 1,
            label: "sum".to_owned(),
            init,
            ty: MirType::Integer,
        }],
    );
    let empty = BTreeMap::new();
    let error = close_generic_loop_v1_carrier_phis(
        &skeleton.plan,
        &state,
        "i",
        skeleton.loop_var_init,
        skeleton.loop_var_current,
        Some(&empty),
    )
    .unwrap_err();
    assert!(error.contains("carrier-backedge-missing"));
    assert!(skeleton.plan.phis.is_empty());
    assert!(
        build_continue_with_phi_args(&builder, &state.carrier_step_phis, &empty, "keyed-test")
            .unwrap_err()
            .contains("not found")
    );
}

#[test]
fn keyed_carrier_without_backedge_uses_only_preheader_inputs() {
    let (mut builder, skeleton) = skeleton();
    let init = builder.alloc_typed(MirType::Integer);
    let state = allocate_generic_loop_v1_carriers(
        &mut builder,
        0u32,
        skeleton.loop_var_current,
        &skeleton.carrier_representation,
        vec![GenericLoopCarrierEntryV1 {
            key: 1,
            label: "sum".to_owned(),
            init,
            ty: MirType::Integer,
        }],
    );
    let phis = close_generic_loop_v1_carrier_phis(
        &skeleton.plan,
        &state,
        "i",
        skeleton.loop_var_init,
        skeleton.loop_var_current,
        None,
    )
    .unwrap();
    assert_eq!(phis.len(), 2);
    assert!(phis.iter().all(|phi| phi.block == skeleton.plan.header_bb
        && phi.inputs.len() == 1
        && phi.inputs[0].0 == skeleton.plan.preheader_bb));
    assert_eq!(
        phis.iter()
            .find(|phi| phi.dst == state.phi_bindings[&1])
            .unwrap()
            .inputs[0]
            .1,
        init
    );
}
