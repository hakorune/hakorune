use super::{BindingSsaBuilderV1, BindingSsaErrorV1, BindingSsaIrV1, MirBindingSsaAdapterV1};
use crate::mir::builder::emission::phi_lifecycle::{PhiToken, PhiTxn};
use crate::mir::builder::resolved_lowering::canonical_cfg::CanonicalCfgSessionV1;
use crate::mir::builder::MirBuilder;
use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIdV1, FunctionOwnerIssuerV1};
use crate::mir::{
    BasicBlock, BasicBlockId, BindingId, ConstValue, EffectMask, FunctionSignature, MirFunction,
    MirInstruction, MirType, ValueId,
};

fn bb(id: u32) -> BasicBlockId {
    BasicBlockId::new(id)
}

fn owner() -> FunctionOwnerIdV1 {
    FunctionOwnerIssuerV1::new_for_compilation()
        .unwrap()
        .issue()
        .unwrap()
}

fn binding(owner: FunctionOwnerIdV1, slot: u32) -> BindingRefV1 {
    BindingRefV1::new(owner, BindingId::new(slot))
}

fn builder(block_count: u32, params: Vec<MirType>) -> MirBuilder {
    let mut builder = MirBuilder::new();
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "binding_ssa_mir_adapter_test/0".to_string(),
            params: params.clone(),
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        bb(0),
    );
    for id in 1..block_count {
        function.add_block(BasicBlock::new(bb(id)));
    }
    for (index, ty) in params.into_iter().enumerate() {
        builder
            .function_state
            .type_ctx
            .value_types
            .insert(ValueId::new(index as u32), ty);
    }
    builder.function_state.current_function = Some(function);
    builder
}

fn emit_const(
    builder: &mut MirBuilder,
    block: BasicBlockId,
    value: ConstValue,
    ty: MirType,
) -> ValueId {
    let dst = builder.next_value_id();
    builder
        .function_state
        .current_function
        .as_mut()
        .unwrap()
        .get_block_mut(block)
        .unwrap()
        .add_instruction(MirInstruction::Const { dst, value });
    builder.function_state.type_ctx.value_types.insert(dst, ty);
    dst
}

fn set_return(builder: &mut MirBuilder, block: BasicBlockId, value: Option<ValueId>) {
    builder
        .function_state
        .current_function
        .as_mut()
        .unwrap()
        .get_block_mut(block)
        .unwrap()
        .set_terminator(MirInstruction::Return { value });
}

fn phi(builder: &MirBuilder, block: BasicBlockId, dst: ValueId) -> Option<&MirInstruction> {
    builder.function_state.current_function
        .as_ref()
        .unwrap()
        .get_block(block)
        .unwrap()
        .instructions
        .iter()
        .find(|instruction| {
            matches!(instruction, MirInstruction::Phi { dst: candidate, .. } if *candidate == dst)
        })
}

fn seal_ssa(
    builder: &mut MirBuilder,
    cfg: &mut CanonicalCfgSessionV1,
    ssa: &mut BindingSsaBuilderV1<PhiToken>,
    phis: &mut PhiTxn,
    block: BasicBlockId,
) -> Result<(), BindingSsaErrorV1> {
    let witness = cfg
        .seal_block(
            builder.function_state.current_function.as_mut().unwrap(),
            block,
        )
        .unwrap();
    let mut adapter = MirBindingSsaAdapterV1::new(builder, phis);
    ssa.seal(&mut adapter, block, &witness)
}

#[test]
fn real_loop_phi_is_defined_before_exposure_and_stays_fact_unknown() {
    let owner = owner();
    let binding = binding(owner, 0);
    let mut builder = builder(4, vec![MirType::Bool]);
    let mut cfg = CanonicalCfgSessionV1::new();
    let mut ssa = BindingSsaBuilderV1::new(owner);
    let mut phis = PhiTxn::begin("binding_ssa_m0_loop");

    let body_value = emit_const(&mut builder, bb(2), ConstValue::Bool(true), MirType::Bool);
    {
        let function = builder.function_state.current_function.as_mut().unwrap();
        cfg.emit_jump(function, bb(0), bb(1)).unwrap();
        cfg.emit_branch(function, bb(1), ValueId::new(0), bb(2), bb(3))
            .unwrap();
        cfg.emit_jump(function, bb(2), bb(1)).unwrap();
    }
    set_return(&mut builder, bb(3), None);

    ssa.define(binding, bb(0), ValueId::new(0)).unwrap();
    seal_ssa(&mut builder, &mut cfg, &mut ssa, &mut phis, bb(0)).unwrap();
    let header_value = {
        let mut adapter = MirBindingSsaAdapterV1::new(&mut builder, &mut phis);
        ssa.read(&mut adapter, binding, bb(1)).unwrap()
    };
    assert!(matches!(
        phi(&builder, bb(1), header_value),
        Some(MirInstruction::Phi { inputs, .. }) if inputs.is_empty()
    ));
    assert_eq!(
        builder
            .function_state
            .type_ctx
            .value_types
            .get(&header_value),
        Some(&MirType::Unknown)
    );

    ssa.define(binding, bb(2), body_value).unwrap();
    seal_ssa(&mut builder, &mut cfg, &mut ssa, &mut phis, bb(2)).unwrap();
    seal_ssa(&mut builder, &mut cfg, &mut ssa, &mut phis, bb(1)).unwrap();
    seal_ssa(&mut builder, &mut cfg, &mut ssa, &mut phis, bb(3)).unwrap();

    assert!(matches!(
        phi(&builder, bb(1), header_value),
        Some(MirInstruction::Phi { inputs, type_hint: None, .. })
            if inputs == &vec![(bb(0), ValueId::new(0)), (bb(2), body_value)]
    ));
    assert_eq!(
        builder
            .function_state
            .type_ctx
            .value_types
            .get(&header_value),
        Some(&MirType::Unknown)
    );
    ssa.finish().unwrap();
    phis.commit(&mut builder).unwrap();
    cfg.finish(builder.function_state.current_function.as_ref().unwrap())
        .unwrap();
}

#[test]
fn non_dominating_sibling_input_rolls_back_pending_phi_and_poison_ssa() {
    let owner = owner();
    let binding = binding(owner, 0);
    let mut builder = builder(4, vec![MirType::Bool]);
    let mut cfg = CanonicalCfgSessionV1::new();
    let mut ssa = BindingSsaBuilderV1::new(owner);
    let mut phis = PhiTxn::begin("binding_ssa_m0_non_dominating");
    let sibling_value = emit_const(
        &mut builder,
        bb(1),
        ConstValue::Integer(7),
        MirType::Integer,
    );
    {
        let function = builder.function_state.current_function.as_mut().unwrap();
        cfg.emit_branch(function, bb(0), ValueId::new(0), bb(1), bb(2))
            .unwrap();
        cfg.emit_jump(function, bb(1), bb(3)).unwrap();
        cfg.emit_jump(function, bb(2), bb(3)).unwrap();
    }
    set_return(&mut builder, bb(3), None);

    ssa.define(binding, bb(1), sibling_value).unwrap();
    ssa.define(binding, bb(2), sibling_value).unwrap();
    for block in [bb(0), bb(1), bb(2)] {
        let witness = cfg
            .seal_block(
                builder.function_state.current_function.as_mut().unwrap(),
                block,
            )
            .unwrap();
        let mut adapter = MirBindingSsaAdapterV1::new(&mut builder, &mut phis);
        ssa.seal(&mut adapter, block, &witness).unwrap();
    }

    let merge_witness = cfg
        .seal_block(
            builder.function_state.current_function.as_mut().unwrap(),
            bb(3),
        )
        .unwrap();
    {
        let mut adapter = MirBindingSsaAdapterV1::new(&mut builder, &mut phis);
        ssa.seal(&mut adapter, bb(3), &merge_witness).unwrap();
    }
    let error = {
        let mut adapter = MirBindingSsaAdapterV1::new(&mut builder, &mut phis);
        ssa.read(&mut adapter, binding, bb(3))
    };
    assert!(matches!(
        error,
        Err(BindingSsaErrorV1::PhiOperation {
            operation: "verify_input",
            ref detail,
        }) if detail.contains("does not dominate")
    ));
    assert!(builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .get_block(bb(3))
        .unwrap()
        .instructions
        .iter()
        .all(|instruction| !matches!(instruction, MirInstruction::Phi { .. })));
    assert!(builder
        .function_state
        .type_ctx
        .value_types
        .values()
        .all(|ty| *ty != MirType::Unknown));
    assert!(matches!(ssa.finish(), Err(BindingSsaErrorV1::Poisoned)));
    phis.commit(&mut builder).unwrap();
}

#[test]
fn unreachable_predecessor_is_rejected_explicitly() {
    let mut builder = builder(2, vec![MirType::Integer]);
    let mut phis = PhiTxn::begin("binding_ssa_m0_unreachable");
    set_return(&mut builder, bb(0), Some(ValueId::new(0)));
    set_return(&mut builder, bb(1), None);

    let adapter = MirBindingSsaAdapterV1::new(&mut builder, &mut phis);
    let error = adapter
        .verify_phi_input(bb(1), ValueId::new(0))
        .unwrap_err();
    assert!(error.contains("is unreachable"));
}

#[test]
fn return_block_uses_the_same_cfg_witness_and_ssa_seal_path() {
    let owner = owner();
    let binding = binding(owner, 0);
    let mut builder = builder(1, vec![MirType::Integer]);
    let mut cfg = CanonicalCfgSessionV1::new();
    let mut ssa = BindingSsaBuilderV1::new(owner);
    let mut phis = PhiTxn::begin("binding_ssa_m0_return");
    set_return(&mut builder, bb(0), Some(ValueId::new(0)));
    ssa.define(binding, bb(0), ValueId::new(0)).unwrap();

    seal_ssa(&mut builder, &mut cfg, &mut ssa, &mut phis, bb(0)).unwrap();
    let value = {
        let mut adapter = MirBindingSsaAdapterV1::new(&mut builder, &mut phis);
        ssa.read(&mut adapter, binding, bb(0)).unwrap()
    };
    assert_eq!(value, ValueId::new(0));
    ssa.finish().unwrap();
    phis.commit(&mut builder).unwrap();
    cfg.finish(builder.function_state.current_function.as_ref().unwrap())
        .unwrap();
}

#[test]
fn later_patch_failure_keeps_completed_peer_and_discards_draft() {
    let owner = owner();
    let first_binding = binding(owner, 0);
    let second_binding = binding(owner, 1);
    let mut builder = builder(4, vec![MirType::Bool, MirType::Integer]);
    let mut cfg = CanonicalCfgSessionV1::new();
    let mut ssa = BindingSsaBuilderV1::new(owner);
    let mut phis = PhiTxn::begin("binding_ssa_m0_partial_patch");
    let first_backedge = emit_const(&mut builder, bb(2), ConstValue::Bool(false), MirType::Bool);
    let second_backedge = emit_const(
        &mut builder,
        bb(2),
        ConstValue::Integer(9),
        MirType::Integer,
    );
    {
        let function = builder.function_state.current_function.as_mut().unwrap();
        cfg.emit_jump(function, bb(0), bb(1)).unwrap();
        cfg.emit_branch(function, bb(1), ValueId::new(0), bb(2), bb(3))
            .unwrap();
        cfg.emit_jump(function, bb(2), bb(1)).unwrap();
    }
    set_return(&mut builder, bb(3), None);

    ssa.define(first_binding, bb(0), ValueId::new(0)).unwrap();
    ssa.define(second_binding, bb(0), ValueId::new(1)).unwrap();
    seal_ssa(&mut builder, &mut cfg, &mut ssa, &mut phis, bb(0)).unwrap();
    let first_phi = {
        let mut adapter = MirBindingSsaAdapterV1::new(&mut builder, &mut phis);
        ssa.read(&mut adapter, first_binding, bb(1)).unwrap()
    };
    let second_phi = {
        let mut adapter = MirBindingSsaAdapterV1::new(&mut builder, &mut phis);
        ssa.read(&mut adapter, second_binding, bb(1)).unwrap()
    };
    ssa.define(first_binding, bb(2), first_backedge).unwrap();
    ssa.define(second_binding, bb(2), second_backedge).unwrap();
    seal_ssa(&mut builder, &mut cfg, &mut ssa, &mut phis, bb(2)).unwrap();

    let header = builder
        .function_state
        .current_function
        .as_mut()
        .unwrap()
        .get_block_mut(bb(1))
        .unwrap();
    let index = header
        .instructions
        .iter()
        .position(|instruction| {
            matches!(instruction, MirInstruction::Phi { dst, .. } if *dst == second_phi)
        })
        .unwrap();
    header.instructions.remove(index);
    header.instruction_spans.remove(index);

    let error = seal_ssa(&mut builder, &mut cfg, &mut ssa, &mut phis, bb(1)).unwrap_err();
    assert!(matches!(error, BindingSsaErrorV1::DuringPhiCleanup { .. }));
    assert!(matches!(
        phi(&builder, bb(1), first_phi),
        Some(MirInstruction::Phi { inputs, .. }) if inputs.len() == 2
    ));
    assert!(phi(&builder, bb(1), second_phi).is_none());
    assert!(matches!(ssa.finish(), Err(BindingSsaErrorV1::Poisoned)));
    let abort = phis.commit(&mut builder).unwrap_err();
    assert_eq!(abort.pending_count(), 1);
    assert_eq!(abort.cleanup_failures().len(), 1);

    let discarded = builder.function_state.current_function.take().unwrap();
    assert!(builder.function_state.current_function.is_none());
    drop(discarded);
}
