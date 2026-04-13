use super::super::emit_mir_json_string_for_harness_bin;
use super::make_function;
use crate::mir::{BasicBlockId, MirInstruction, MirModule, ValueId};

#[test]
fn emit_debug_instruction_for_harness_bin() {
    let mut module = MirModule::new("test".to_string());
    let mut function = make_function("main", true);
    function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry block")
        .instructions
        .extend([
            MirInstruction::Debug {
                value: ValueId::new(1),
                message: "probe".to_string(),
            },
            MirInstruction::Return {
                value: Some(ValueId::new(1)),
            },
        ]);
    module.functions.insert("main".to_string(), function);

    let json = emit_mir_json_string_for_harness_bin(&module).expect("emit");
    assert!(json.contains("\"op\":\"debug\""), "json={}", json);
    assert!(json.contains("\"message\":\"probe\""), "json={}", json);
    assert!(json.contains("\"value\":1"), "json={}", json);
}
