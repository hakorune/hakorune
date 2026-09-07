use super::*;
use crate::mir::{BasicBlock, BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirType};

#[test]
fn frame_is_issued_once_and_root_selection_precedes_materialization() {
    // Physical-state test; App Main source identity is checked by its
    // production root entry, not manufactured by this fixture.
    for mode in [FaultFrameMode::Borrowed, FaultFrameMode::RootOwned] {
        let mut state = FunctionFaultFrameV1::borrowed();
        if mode == FaultFrameMode::RootOwned {
            state.select_root().unwrap();
            assert!(state.select_root().is_err());
        }
        let mut builder = MirBuilder::new();
        let entry = BasicBlockId::new(0);
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "frame_test".into(),
                params: vec![],
                return_type: MirType::Void,
                effects: EffectMask::CONTROL,
            },
            entry,
        );
        function.add_block(BasicBlock::new(entry));
        builder.function_state.current_function = Some(function);
        let value = state.materialize(&mut builder).unwrap();
        assert_eq!(state.materialize(&mut builder).unwrap(), value);
        assert!(state.select_root().is_err());
        let function = builder.function_state.current_function.as_ref().unwrap();
        assert!(function.params.is_empty());
        let block = &function.blocks[&entry];
        assert_eq!(block.instructions.len(), 1);
        assert_eq!(block.instruction_spans.len(), 1);
        assert!(matches!(block.instructions[0],
                MirInstruction::FaultFrameEnter { dst, mode: actual }
                if dst == value && actual == mode));
        state.validate(function).unwrap();
        let mut duplicate = function.clone();
        let extra = BasicBlockId::new(9);
        let mut block = BasicBlock::new(extra);
        block.add_instruction(MirInstruction::FaultFrameEnter { dst: value, mode });
        duplicate.add_block(block);
        assert!(state
            .validate(&duplicate)
            .unwrap_err()
            .contains("definition-count-drift"));
        assert!(FunctionFaultFrameV1::borrowed()
            .validate(function)
            .unwrap_err()
            .contains("definition-count-drift"));
    }
}
