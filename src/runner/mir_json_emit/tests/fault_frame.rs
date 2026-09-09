use super::super::build_mir_json_root;
use super::make_function;
use crate::mir::instruction::FaultFrameMode;
use crate::mir::{BasicBlockId, ConstValue, MirInstruction, MirModule, ValueId};

#[test]
fn fault_frame_enter_preserves_existing_mode_in_json() {
    for (mode, expected) in [
        (FaultFrameMode::RootOwned, "root_owned"),
        (FaultFrameMode::Borrowed, "borrowed"),
    ] {
        let mut function = make_function("main", true);
        let entry = function.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
        entry.instructions.extend([
            MirInstruction::FaultFrameEnter {
                dst: ValueId::new(0),
                mode,
            },
            MirInstruction::Const {
                dst: ValueId::new(1),
                value: ConstValue::Integer(30),
            },
            MirInstruction::Return {
                value: Some(ValueId::new(1)),
            },
        ]);
        let mut module = MirModule::new("fault-frame-json".to_string());
        module.add_function(function);

        let root = build_mir_json_root(&module).expect("fault frame JSON");
        let rows = root["functions"][0]["blocks"][0]["instructions"]
            .as_array()
            .expect("instruction rows");
        let frame = rows
            .iter()
            .find(|row| row["op"] == "fault_frame_enter")
            .expect("fault frame row");
        assert_eq!(frame["dst"], 0);
        assert_eq!(frame["mode"], expected);
    }
}
