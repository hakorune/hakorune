use crate::mir::{
    BasicBlock, BasicBlockId, BinaryOp, CompareOp, ConstValue, EffectMask, FunctionSignature,
    MirFunction, MirInstruction, MirType, ValueId,
};

mod array_routes;
mod array_write;
mod checked_callout_transport;
mod constructor_call_routes;
mod control_edge_args;
mod debug;
mod decl_values;
mod direct_array_access_plans;
mod exact_numeric_routes;
mod exact_seed_backend_route;
mod extern_call_routes;
mod fastmem_metadata;
mod function_attrs;
mod generic_method_routes;
mod global_call_routes;
mod hmi_t0_fixtures;
mod local_contracts;
mod map_lookup_fusion_routes;
mod map_repr_plans;
mod ordering;
mod ownership_transport;
mod parameter_contracts;
mod placement;
mod proof_envelopes;
mod record_contracts;
mod return_contracts;
mod static_table_contracts;
mod string_corridor;
mod string_direct_set_routes;
mod thin_entry;
mod typed_object_exact_slot_routes;
mod weak_field_contracts;

fn make_function(name: &str, is_entry_point: bool) -> MirFunction {
    let signature = FunctionSignature {
        name: name.to_string(),
        params: vec![],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId::new(0));
    function.metadata.is_entry_point = is_entry_point;
    function
}

fn make_string_loop_function() -> MirFunction {
    let mut function = make_function("main", true);
    let entry = BasicBlockId::new(0);
    let header = BasicBlockId::new(18);
    let body = BasicBlockId::new(19);
    let exit = BasicBlockId::new(21);

    function
        .blocks
        .get_mut(&entry)
        .unwrap()
        .instructions
        .extend([
            MirInstruction::Const {
                dst: ValueId::new(3),
                value: ConstValue::String("line-seed-abcdef".to_string()),
            },
            MirInstruction::Copy {
                dst: ValueId::new(4),
                src: ValueId::new(3),
            },
            MirInstruction::Const {
                dst: ValueId::new(5),
                value: ConstValue::Integer(16),
            },
        ]);

    let mut header_block = BasicBlock::new(header);
    header_block.instructions.extend([
        MirInstruction::Phi {
            dst: ValueId::new(15),
            inputs: vec![(entry, ValueId::new(12)), (body, ValueId::new(16))],
            type_hint: Some(MirType::Integer),
        },
        MirInstruction::Phi {
            dst: ValueId::new(21),
            inputs: vec![(entry, ValueId::new(4)), (body, ValueId::new(36))],
            type_hint: Some(MirType::String),
        },
        MirInstruction::Const {
            dst: ValueId::new(41),
            value: ConstValue::Integer(300000),
        },
        MirInstruction::Compare {
            dst: ValueId::new(37),
            op: CompareOp::Lt,
            lhs: ValueId::new(15),
            rhs: ValueId::new(41),
        },
        MirInstruction::Branch {
            condition: ValueId::new(37),
            then_bb: body,
            else_bb: exit,
            then_edge_args: None,
            else_edge_args: None,
        },
    ]);
    function.blocks.insert(header, header_block);

    let mut body_block = BasicBlock::new(body);
    body_block.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(50),
            value: ConstValue::Integer(2),
        },
        MirInstruction::BinOp {
            dst: ValueId::new(47),
            op: BinaryOp::Div,
            lhs: ValueId::new(5),
            rhs: ValueId::new(50),
        },
        MirInstruction::Const {
            dst: ValueId::new(66),
            value: ConstValue::String("xx".to_string()),
        },
        MirInstruction::Copy {
            dst: ValueId::new(36),
            src: ValueId::new(21),
        },
    ]);
    function.blocks.insert(body, body_block);
    function.blocks.insert(exit, BasicBlock::new(exit));
    function
}

#[test]
fn build_mir_json_root_requires_and_emits_pinned_text_plan_census() {
    use super::root::build_mir_json_root;
    use crate::mir::pinned_text_access_plan::{
        PinnedTextAccessKindV1, PinnedTextAccessPlanTableV1, PinnedTextRootIdV1,
    };
    use crate::mir::MirModule;

    let mut function = make_function("pinned_text_transport", false);
    let root = PinnedTextRootIdV1::from_frame_row(0);
    let kind = PinnedTextAccessKindV1::ByteLen { root };
    let mut plans = PinnedTextAccessPlanTableV1::new(19);
    let plan = plans.issue(kind);
    function.metadata.pinned_text_access_plans = plans;
    function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry block")
        .instructions
        .extend([
            MirInstruction::PinnedTextOp {
                dst: ValueId::new(1),
                plan,
                kind,
            },
            MirInstruction::Return {
                value: Some(ValueId::new(1)),
            },
        ]);
    let mut module = MirModule::new("pinned_text_transport".to_owned());
    module.add_function(function);
    let root = build_mir_json_root(&module).expect("plan census should pass");
    let instruction = &root["functions"][0]["blocks"][0]["instructions"][0];
    assert_eq!(instruction["op"], "pinned_text_op");
    assert_eq!(instruction["plan_stamp"], 19);
    assert_eq!(instruction["access"]["kind"], "byte_len");
}
