//! GenericLoop skeleton allocation (blocks/slots only, no AST analysis).

use crate::mir::basic_block::EdgeArgs;
use crate::mir::builder::control_flow::plan::edgecfg_facade::Frag;
use crate::mir::builder::control_flow::plan::features::edgecfg_stubs;
use crate::mir::builder::control_flow::plan::normalizer::helpers::{
    create_phi_bindings, LoopBlocksStandard5,
};
use crate::mir::builder::control_flow::plan::step_mode::extract_to_step_bb_explicit_step;
use crate::mir::builder::control_flow::plan::CoreLoopPlan;
use crate::mir::builder::MirBuilder;
use crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
use crate::mir::{MirType, ValueId};
use std::collections::BTreeMap;

pub(in crate::mir::builder) struct GenericLoopSkeleton {
    pub plan: CoreLoopPlan,
    pub loop_var_init: ValueId,
    pub loop_var_current: ValueId,
    pub loop_var_next: ValueId,
    pub phi_bindings: BTreeMap<String, ValueId>,
}

pub(in crate::mir::builder) fn alloc_generic_loop_v0_skeleton(
    builder: &mut MirBuilder,
    loop_var: &str,
) -> Result<GenericLoopSkeleton, String> {
    let loop_var_init = builder
        .variable_ctx
        .variable_map
        .get(loop_var)
        .copied()
        .ok_or_else(|| format!("[normalizer] Loop variable {} not found", loop_var))?;

    let blocks = LoopBlocksStandard5::allocate(builder)?;
    let LoopBlocksStandard5 {
        preheader_bb,
        header_bb,
        body_bb,
        step_bb,
        after_bb,
    } = blocks;

    let loop_var_current = builder.alloc_typed(MirType::Integer);
    let loop_var_next = builder.alloc_typed(MirType::Integer);
    let cond_loop = builder.alloc_typed(MirType::Bool);

    let phi_bindings = create_phi_bindings(&[(loop_var, loop_var_current)]);

    let block_effects = vec![
        (preheader_bb, vec![]),
        (header_bb, vec![]),
        (body_bb, vec![]),
        (step_bb, vec![]),
    ];

    let empty_args = EdgeArgs {
        layout: JumpArgsLayout::CarriersOnly,
        values: vec![],
    };

    let branches = vec![edgecfg_stubs::build_loop_header_branch_with_args(
        header_bb,
        cond_loop,
        body_bb,
        empty_args.clone(),
        after_bb,
        empty_args.clone(),
    )];

    let wires = vec![
        edgecfg_stubs::build_loop_back_edge_with_args(body_bb, step_bb, empty_args.clone()),
        edgecfg_stubs::build_loop_back_edge_with_args(step_bb, header_bb, empty_args.clone()),
    ];

    let frag = Frag {
        entry: header_bb,
        block_params: BTreeMap::new(),
        exits: BTreeMap::new(),
        wires,
        branches,
    };

    let final_values = vec![(loop_var.to_string(), loop_var_current)];
    let (step_mode, has_explicit_step) = extract_to_step_bb_explicit_step();

    let plan = CoreLoopPlan {
        preheader_bb,
        preheader_is_fresh: false,
        header_bb,
        body_bb,
        step_bb,
        continue_target: step_bb,
        after_bb,
        found_bb: after_bb,
        body: vec![],
        cond_loop,
        cond_match: cond_loop,
        block_effects,
        phis: Vec::new(),
        frag,
        final_values,
        step_mode,
        has_explicit_step,
    };

    Ok(GenericLoopSkeleton {
        plan,
        loop_var_init,
        loop_var_current,
        loop_var_next,
        phi_bindings,
    })
}
