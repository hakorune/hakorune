use super::super::build_mir_json_root;
use crate::mir::function::{TypedObjectFieldPlan, TypedObjectFieldStorage, TypedObjectPlan};
use crate::mir::route_decision::refresh_module_typed_object_exact_slot_route_decisions;
use crate::mir::{
    BasicBlock, BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirInstruction,
    MirModule, MirType, ValueId,
};

fn typed_object_route_plan() -> TypedObjectPlan {
    TypedObjectPlan {
        box_name: "Page".to_string(),
        type_id: 294019300,
        layout_kind: "typed_object_v0".to_string(),
        field_count: 3,
        fields: vec![
            TypedObjectFieldPlan {
                name: "capacity".to_string(),
                slot: 0,
                declared_type_name: Some("usize".to_string()),
                storage: TypedObjectFieldStorage::U64,
                is_weak: false,
            },
            TypedObjectFieldPlan {
                name: "used".to_string(),
                slot: 1,
                declared_type_name: Some("i64".to_string()),
                storage: TypedObjectFieldStorage::I64,
                is_weak: false,
            },
        ],
    }
}

#[test]
fn build_mir_json_root_emits_typed_object_exact_slot_route_decisions() {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let mut block = BasicBlock::new(BasicBlockId::new(0));
    block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(1),
        box_type: "Page".to_string(),
        args: vec![],
    });
    block.add_instruction(MirInstruction::FieldGet {
        dst: ValueId::new(2),
        base: ValueId::new(1),
        field: "capacity".to_string(),
        declared_type: Some(MirType::Integer),
    });
    block.add_instruction(MirInstruction::FieldSet {
        base: ValueId::new(1),
        field: "used".to_string(),
        value: ValueId::new(3),
        declared_type: Some(MirType::Integer),
    });
    block.set_terminator(MirInstruction::Return { value: None });
    function.add_block(block);

    let mut module = MirModule::new("typed_object_exact_slot_route_json_test".to_string());
    module
        .metadata
        .typed_object_plans
        .push(typed_object_route_plan());
    module.add_function(function);

    refresh_module_typed_object_exact_slot_route_decisions(&mut module);

    let root = build_mir_json_root(&module).expect("mir json root");
    let decisions = root["functions"][0]["metadata"]["route_decisions"]
        .as_array()
        .expect("route_decisions");

    assert_eq!(decisions.len(), 2);

    let get_decision = &decisions[0];
    assert_eq!(get_decision["route_id"], "route.decision");
    assert_eq!(
        get_decision["source_plan_kind"],
        "TypedObjectExactSlotRoute"
    );
    assert_eq!(get_decision["semantic_op"], "FieldGet");
    assert_eq!(
        get_decision["access_kind"],
        "typed_object_exact_slot_get_u64"
    );
    assert_eq!(
        get_decision["preferred_route"],
        "hako.typed_object.slot_load_u64"
    );
    assert_eq!(
        get_decision["selected_route"],
        "hako.typed_object.slot_load_u64"
    );
    assert_eq!(
        get_decision["selected_lowering_form"],
        "exact_helper_bridge"
    );
    assert_eq!(
        get_decision["selected_bridge_symbol"],
        "hako.object.exact_slot_get_u64_hii"
    );
    assert_eq!(get_decision["selected_slot"], 0);
    assert_eq!(get_decision["selected_storage"], "u64");
    assert_eq!(get_decision["receiver_box_name"], "Page");
    assert_eq!(get_decision["field_id"], "capacity");
    assert_eq!(get_decision["fallback_policy"], "fail_fast");

    let set_decision = &decisions[1];
    assert_eq!(
        set_decision["source_plan_kind"],
        "TypedObjectExactSlotRoute"
    );
    assert_eq!(set_decision["semantic_op"], "FieldSet");
    assert_eq!(
        set_decision["access_kind"],
        "typed_object_exact_slot_set_i64"
    );
    assert_eq!(
        set_decision["preferred_route"],
        "hako.typed_object.slot_store_i64"
    );
    assert_eq!(
        set_decision["selected_route"],
        "hako.typed_object.slot_store_i64"
    );
    assert_eq!(
        set_decision["selected_lowering_form"],
        "exact_helper_bridge"
    );
    assert_eq!(
        set_decision["selected_bridge_symbol"],
        "hako.object.exact_slot_set_i64_hii"
    );
    assert_eq!(set_decision["selected_slot"], 1);
    assert_eq!(set_decision["selected_storage"], "i64");
    assert_eq!(set_decision["receiver_box_name"], "Page");
    assert_eq!(set_decision["field_id"], "used");
    assert_eq!(set_decision["fallback_policy"], "fail_fast");
}
