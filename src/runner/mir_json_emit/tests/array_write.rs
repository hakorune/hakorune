use super::super::emit_mir_json_string_for_harness_bin;
use crate::mir::{
    ArrayElementWriteKind, ArrayWriteSiteId, BasicBlockId, EffectMask, FunctionSignature,
    MirFunction, MirModule, MirType, ValueId,
};

#[test]
fn explicit_array_write_is_observable_in_mir_json() {
    let mut module = MirModule::new("array-write-json".to_string());
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "Main.main/0".to_string(),
            params: Vec::new(),
            return_type: MirType::Void,
            effects: EffectMask::MUT,
        },
        BasicBlockId::new(0),
    );
    function
        .get_block_mut(function.entry_block)
        .unwrap()
        .add_instruction(
            crate::mir::array_element_write::instruction(
                ArrayWriteSiteId::new(7),
                Some(ValueId::new(3)),
                ArrayElementWriteKind::Set,
                crate::mir::ArrayWriteProducerKind::IndexAssignment,
                ValueId::new(0),
                Some(ValueId::new(1)),
                ValueId::new(2),
            )
            .unwrap(),
        );
    module.add_function(function);

    let output = emit_mir_json_string_for_harness_bin(&module).unwrap();
    let root: serde_json::Value = serde_json::from_str(&output).unwrap();
    let instruction = &root["functions"][0]["blocks"][0]["instructions"][0];
    assert_eq!(instruction["op"], "array_element_write");
    assert_eq!(instruction["site_id"], 7);
    assert_eq!(instruction["kind"], "set");
    assert_eq!(instruction["receiver"], 0);
    assert_eq!(instruction["index"], 1);
    assert_eq!(instruction["value"], 2);
    assert_eq!(instruction["dst"], 3);
    let metadata = &root["functions"][0]["metadata"];
    assert_eq!(metadata["array_element_write_witnesses"][0]["site_id"], 7);
    assert_eq!(
        metadata["array_state_terms"][0]["relation"]["kind"],
        "dynamic_boundary"
    );
}
