use super::super::build_mir_json_root;
use super::make_function;
use crate::mir::{BasicBlockId, Callee, EffectMask, MirInstruction, ValueId};

#[test]
fn build_mir_json_root_emits_constructor_call_lowering_plan() {
    let mut module = crate::mir::MirModule::new("json_constructor_call_routes_test".to_string());
    let mut function = make_function("main", true);
    function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .unwrap()
        .instructions
        .push(MirInstruction::Call {
            dst: Some(ValueId::new(7)),
            func: ValueId::INVALID,
            callee: Some(Callee::Constructor {
                box_type: "MapBox".to_string(),
            }),
            args: vec![],
            effects: EffectMask::PURE,
        });
    module.add_function(function);
    crate::mir::refresh_module_semantic_metadata(&mut module);

    let root = build_mir_json_root(&module).expect("mir json root");
    let metadata = &root["functions"][0]["metadata"];
    let plan = metadata["lowering_plan"]
        .as_array()
        .expect("lowering_plan")
        .iter()
        .find(|row| row["source"] == "constructor_call_routes")
        .expect("constructor_call_routes row");

    assert_eq!(plan["site"], "b0.i0");
    assert_eq!(plan["source_route_id"], "constructor.map_birth");
    assert_eq!(plan["box_type"], "MapBox");
    assert_eq!(plan["core_op"], "MapBirth");
    assert_eq!(plan["tier"], "ColdRuntime");
    assert_eq!(plan["emit_kind"], "runtime_call");
    assert_eq!(plan["symbol"], "nyash.map.birth_h");
    assert_eq!(plan["route_kind"], "constructor.map_birth");
    assert_eq!(plan["result_value"], 7);
    assert_eq!(plan["return_shape"], "mixed_runtime_i64_or_handle");
    assert_eq!(plan["value_demand"], "runtime_i64_or_handle");
    assert_eq!(plan["result_origin"], "map_birth");
    assert_eq!(plan["need_kind"], "map_birth");
}
