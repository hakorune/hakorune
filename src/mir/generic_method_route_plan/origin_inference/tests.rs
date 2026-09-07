use super::*;
use crate::mir::{BasicBlock, EffectMask, FunctionSignature};

#[test]
fn collection_field_key_returns_none_for_self_referential_phi() {
    let signature = FunctionSignature {
        name: "cycle".to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId::new(0));
    let mut block = BasicBlock::new(BasicBlockId::new(0));
    block.add_instruction(MirInstruction::Phi {
        dst: ValueId::new(1),
        inputs: vec![
            (BasicBlockId::new(0), ValueId::new(1)),
            (BasicBlockId::new(0), ValueId::new(2)),
        ],
        type_hint: None,
    });
    block.add_instruction(MirInstruction::FieldGet {
        dst: ValueId::new(2),
        base: ValueId::new(3),
        field: "items".to_string(),
        declared_type: None,
    });
    function.add_block(block);

    let def_map = build_value_def_map(&function);
    assert_eq!(
        typed_object_collection_field_key(&function, &def_map, ValueId::new(1)),
        None
    );
}

#[test]
fn handle_value_origin_box_name_returns_none_for_self_referential_phi() {
    let signature = FunctionSignature {
        name: "cycle_origin".to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId::new(0));
    let mut block = BasicBlock::new(BasicBlockId::new(0));
    block.add_instruction(MirInstruction::Phi {
        dst: ValueId::new(1),
        inputs: vec![
            (BasicBlockId::new(0), ValueId::new(1)),
            (BasicBlockId::new(0), ValueId::new(2)),
        ],
        type_hint: None,
    });
    block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(2),
        target: crate::mir::ConstructionTarget::Named("ArrayBox".to_string()),
        args: vec![],
    });
    function.add_block(block);

    let def_map = build_value_def_map(&function);
    assert_eq!(
        handle_value_origin_box_name_with_context(
            &MirModule::new(String::new()),
            &function,
            &def_map,
            ValueId::new(1),
            &MethodParamBoxOriginMap::new(),
        ),
        None
    );
}
