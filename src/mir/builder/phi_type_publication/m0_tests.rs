use super::{commit_prepared_phi_type, PhiTransientTypeDecisionV1, PreparedPhiTypePublicationV1};
use crate::ast::Span;
use crate::mir::builder::emission::phi_lifecycle::{self, PhiBatchItem, PhiTxn};
use crate::mir::builder::ssa::local;
use crate::mir::builder::MirBuilder;
use crate::mir::{
    BasicBlock, BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirInstruction, MirType,
    ValueId,
};

const OWNER: &str = "CurrentReceiverTypePublishOwnerV1";

#[derive(Clone, Copy)]
struct PhiTimingIds {
    receiver: ValueId,
    phi: ValueId,
    then_block: BasicBlockId,
    else_block: BasicBlockId,
    merge_block: BasicBlockId,
    consumer_block: BasicBlockId,
}

fn timing_builder() -> (MirBuilder, PhiTimingIds) {
    let entry = BasicBlockId::new(0);
    let then_block = BasicBlockId::new(1);
    let else_block = BasicBlockId::new(2);
    let merge_block = BasicBlockId::new(3);
    let consumer_block = BasicBlockId::new(4);
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "phi_type_publication_m0/1".to_string(),
            params: vec![MirType::Box(OWNER.to_string())],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        entry,
    );
    let receiver = function.next_value_id();
    let phi = function.next_value_id();
    function.params.push(receiver);
    function.add_block(BasicBlock::new(then_block));
    function.add_block(BasicBlock::new(else_block));
    function.add_block(BasicBlock::new(merge_block));
    function.add_block(BasicBlock::new(consumer_block));
    function
        .get_block_mut(entry)
        .unwrap()
        .set_terminator(MirInstruction::Branch {
            condition: receiver,
            then_bb: then_block,
            else_bb: else_block,
            then_edge_args: None,
            else_edge_args: None,
        });
    function
        .get_block_mut(then_block)
        .unwrap()
        .set_terminator(MirInstruction::Jump {
            target: merge_block,
            edge_args: None,
        });
    function
        .get_block_mut(else_block)
        .unwrap()
        .set_terminator(MirInstruction::Jump {
            target: merge_block,
            edge_args: None,
        });
    function
        .get_block_mut(merge_block)
        .unwrap()
        .set_terminator(MirInstruction::Jump {
            target: consumer_block,
            edge_args: None,
        });
    function.update_cfg();

    let mut builder = MirBuilder::new();
    builder.scope_ctx.current_function = Some(function);
    builder.current_block = Some(merge_block);
    builder
        .type_ctx
        .value_types
        .insert(receiver, MirType::Box(OWNER.to_string()));
    builder
        .type_ctx
        .value_origin_newbox
        .insert(receiver, OWNER.to_string());

    (
        builder,
        PhiTimingIds {
            receiver,
            phi,
            then_block,
            else_block,
            merge_block,
            consumer_block,
        },
    )
}

fn logical_inputs(ids: PhiTimingIds) -> Vec<(BasicBlockId, ValueId)> {
    vec![
        (ids.then_block, ids.receiver),
        (ids.else_block, ids.receiver),
    ]
}

fn phi_inputs(builder: &MirBuilder, ids: PhiTimingIds) -> Vec<(BasicBlockId, ValueId)> {
    builder
        .scope_ctx
        .current_function
        .as_ref()
        .unwrap()
        .get_block(ids.merge_block)
        .unwrap()
        .instructions
        .iter()
        .find_map(|instruction| match instruction {
            MirInstruction::Phi { dst, inputs, .. } if *dst == ids.phi => Some(inputs.clone()),
            _ => None,
        })
        .expect("M0 fixture PHI")
}

fn raw_phi(ids: PhiTimingIds) -> MirInstruction {
    MirInstruction::Phi {
        dst: ids.phi,
        inputs: logical_inputs(ids),
        type_hint: None,
    }
}

#[test]
fn phi_type_publication_m0_records_the_pre_i0_producer_split() {
    let (mut raw, ids) = timing_builder();
    raw.emit_instruction(raw_phi(ids)).unwrap();
    assert_eq!(
        raw.type_ctx.value_types.get(&ids.phi),
        Some(&MirType::Box(OWNER.to_string()))
    );
    assert_eq!(
        raw.type_ctx.value_origin_newbox.get(&ids.phi),
        Some(&OWNER.to_string())
    );

    let (mut final_builder, ids) = timing_builder();
    phi_lifecycle::define_phi_final_with_type_hint(
        &mut final_builder,
        ids.merge_block,
        ids.phi,
        logical_inputs(ids),
        None,
        "type-publish0-m0-final",
    )
    .unwrap();
    assert_eq!(final_builder.type_ctx.value_types.get(&ids.phi), None);

    let (mut patch_builder, ids) = timing_builder();
    let mut transaction = PhiTxn::begin("type-publish0-m0-patch");
    let token = transaction
        .define_provisional_phi(
            &mut patch_builder,
            ids.merge_block,
            ids.phi,
            "type-publish0-m0-provisional",
        )
        .unwrap();
    assert_eq!(patch_builder.type_ctx.value_types.get(&ids.phi), None);
    transaction
        .patch_phi_inputs(
            &mut patch_builder,
            token,
            logical_inputs(ids),
            "type-publish0-m0-patch",
        )
        .unwrap();
    assert_eq!(patch_builder.type_ctx.value_types.get(&ids.phi), None);

    let (mut batch_builder, ids) = timing_builder();
    phi_lifecycle::define_phi_batch_prepend(
        &mut batch_builder,
        ids.merge_block,
        vec![PhiBatchItem {
            dst: ids.phi,
            inputs: logical_inputs(ids),
            type_hint: None,
            span: Span::unknown(),
            item_tag: "type-publish0-m0-batch-item".to_string(),
        }],
        "type-publish0-m0-batch",
    )
    .unwrap();
    assert_eq!(batch_builder.type_ctx.value_types.get(&ids.phi), None);
}

#[test]
fn phi_type_publication_m0_proves_logical_publication_before_local_copy() {
    let (mut builder, ids) = timing_builder();
    let logical = logical_inputs(ids);
    let prepared = PhiTransientTypeDecisionV1::prepare(
        ids.phi,
        &logical,
        &builder.type_ctx.value_types,
        builder.type_ctx.value_types.get(&ids.phi),
        None,
    )
    .unwrap();
    assert_eq!(
        prepared,
        PreparedPhiTypePublicationV1::Publish(MirType::Box(OWNER.to_string()))
    );

    phi_lifecycle::define_phi_final_with_type_hint(
        &mut builder,
        ids.merge_block,
        ids.phi,
        logical.clone(),
        None,
        "type-publish0-m0-disconnected-adapter",
    )
    .unwrap();
    assert_eq!(phi_inputs(&builder, ids), logical);
    assert_eq!(builder.type_ctx.value_types.get(&ids.phi), None);
    commit_prepared_phi_type(&mut builder.type_ctx.value_types, ids.phi, prepared);
    assert_eq!(
        builder.type_ctx.value_types.get(&ids.phi),
        Some(&MirType::Box(OWNER.to_string()))
    );

    builder.current_block = Some(ids.consumer_block);
    let copy = local::field_base(&mut builder, ids.phi);
    assert_ne!(copy, ids.phi);
    assert_eq!(
        builder.type_ctx.value_types.get(&copy),
        Some(&MirType::Box(OWNER.to_string()))
    );
    let function = builder.scope_ctx.current_function.as_ref().unwrap();
    assert!(function
        .get_block(ids.consumer_block)
        .unwrap()
        .instructions
        .iter()
        .any(|instruction| matches!(instruction, MirInstruction::Copy { dst, src } if *dst == copy && *src == ids.phi)));
    assert_eq!(function.metadata.value_types.get(&ids.phi), None);
    assert_eq!(function.metadata.value_types.get(&copy), None);
    assert_eq!(builder.type_ctx.value_origin_newbox.get(&ids.phi), None);
    assert_eq!(builder.type_ctx.value_origin_newbox.get(&copy), None);
}
