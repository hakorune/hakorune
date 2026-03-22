//! LoopTrueSkeleton: allocates loop(true) blocks/frag only.

use crate::mir::EdgeArgs;
use crate::mir::builder::control_flow::plan::edgecfg_facade::Frag;
use crate::mir::builder::control_flow::plan::features::edgecfg_stubs;
use crate::mir::builder::control_flow::plan::normalizer::helpers::LoopBlocksStandard5;
use crate::mir::builder::control_flow::plan::CoreEffectPlan;
use crate::mir::builder::MirBuilder;
use crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
use crate::mir::{ConstValue, MirType};
use std::collections::BTreeMap;

pub(in crate::mir::builder) struct LoopTrueSkeleton {
    pub preheader_bb: crate::mir::BasicBlockId,
    pub header_bb: crate::mir::BasicBlockId,
    pub body_bb: crate::mir::BasicBlockId,
    pub step_bb: crate::mir::BasicBlockId,
    pub after_bb: crate::mir::BasicBlockId,
    pub cond_loop: crate::mir::ValueId,
    pub block_effects: Vec<(crate::mir::BasicBlockId, Vec<CoreEffectPlan>)>,
    pub frag: Frag,
}

pub(in crate::mir::builder) fn alloc_loop_true_skeleton(
    builder: &mut MirBuilder,
) -> Result<LoopTrueSkeleton, String> {
    let blocks = LoopBlocksStandard5::allocate(builder)?;
    let LoopBlocksStandard5 {
        preheader_bb,
        header_bb,
        body_bb,
        step_bb,
        after_bb,
    } = blocks;

    let cond_loop = builder.alloc_typed(MirType::Bool);
    let header_effects = vec![CoreEffectPlan::Const {
        dst: cond_loop,
        value: ConstValue::Bool(true),
    }];

    let block_effects = vec![
        (preheader_bb, vec![]),
        (header_bb, header_effects),
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

    Ok(LoopTrueSkeleton {
        preheader_bb,
        header_bb,
        body_bb,
        step_bb,
        after_bb,
        cond_loop,
        block_effects,
        frag,
    })
}
