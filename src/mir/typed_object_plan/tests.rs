use super::*;
use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
use crate::mir::function::UserBoxFieldDecl;
use crate::mir::{
    BasicBlockId, Callee, ConstValue, EffectMask, FunctionSignature, MirFunction,
    MirInstruction, MirModule,
};

#[test]
fn publishes_integer_result_for_typed_object_array_field_get() {
    let mut module = MirModule::new("typed_object_array_element_type_test".to_string());
    module
        .metadata
        .user_box_decls
        .insert("Holder".to_string(), vec!["values".to_string()]);
    module.metadata.user_box_field_decls.insert(
        "Holder".to_string(),
        vec![UserBoxFieldDecl {
            name: "values".to_string(),
            declared_type_name: Some("ArrayBox".to_string()),
            is_weak: false,
        }],
    );
    refresh_module_typed_object_plans(&mut module).expect("layout refresh");

    let mut push_fn = MirFunction::new(
        FunctionSignature {
            name: "Holder.push_value/1".to_string(),
            params: vec![MirType::Unknown, MirType::Integer],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    push_fn.params = vec![ValueId::new(0), ValueId::new(1)];
    push_fn
        .metadata
        .value_types
        .insert(ValueId::new(1), MirType::Integer);
    let push_block = push_fn
        .get_block_mut(BasicBlockId::new(0))
        .expect("push entry");
    push_block.add_instruction(MirInstruction::FieldGet {
        dst: ValueId::new(2),
        base: ValueId::new(0),
        field: "values".to_string(),
        declared_type: Some(MirType::Box("ArrayBox".to_string())),
    });
    push_block.add_instruction(MirInstruction::LegacyCallV0 {
        dst: None,
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "ArrayBox".to_string(),
            method: "push".to_string(),
            receiver: Some(ValueId::new(2)),
            certainty: TypeCertainty::Known,
            box_kind: CalleeBoxKind::RuntimeData,
        }),
        args: vec![ValueId::new(1)],
        effects: EffectMask::IO,
    });

    let mut read_fn = MirFunction::new(
        FunctionSignature {
            name: "Holder.read_first/0".to_string(),
            params: vec![MirType::Unknown],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    read_fn.params = vec![ValueId::new(0)];
    let read_block = read_fn
        .get_block_mut(BasicBlockId::new(0))
        .expect("read entry");
    read_block.add_instruction(MirInstruction::FieldGet {
        dst: ValueId::new(2),
        base: ValueId::new(0),
        field: "values".to_string(),
        declared_type: Some(MirType::Box("ArrayBox".to_string())),
    });
    read_block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Integer(0),
    });
    read_block.add_instruction(MirInstruction::LegacyCallV0 {
        dst: Some(ValueId::new(4)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "RuntimeDataBox".to_string(),
            method: "get".to_string(),
            receiver: Some(ValueId::new(2)),
            certainty: TypeCertainty::Union,
            box_kind: CalleeBoxKind::RuntimeData,
        }),
        args: vec![ValueId::new(3)],
        effects: EffectMask::PURE,
    });

    module.add_function(push_fn);
    module.add_function(read_fn);
    refresh_module_typed_object_field_value_types(&mut module);
    refresh_module_typed_object_collection_field_element_value_types(&mut module);

    let read_fn = module
        .get_function("Holder.read_first/0")
        .expect("read function");
    assert_eq!(
        read_fn.metadata.value_types.get(&ValueId::new(4)),
        Some(&MirType::Integer)
    );
}

#[test]
fn publishes_integer_result_from_observed_method_param_array_write() {
    let mut module = MirModule::new("typed_object_array_observed_param_test".to_string());
    module
        .metadata
        .user_box_decls
        .insert("Holder".to_string(), vec!["values".to_string()]);
    module.metadata.user_box_field_decls.insert(
        "Holder".to_string(),
        vec![UserBoxFieldDecl {
            name: "values".to_string(),
            declared_type_name: Some("ArrayBox".to_string()),
            is_weak: false,
        }],
    );
    refresh_module_typed_object_plans(&mut module).expect("layout refresh");

    let mut set_fn = MirFunction::new(
        FunctionSignature {
            name: "Holder.set/1".to_string(),
            params: vec![MirType::Unknown, MirType::Unknown],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    set_fn.params = vec![ValueId::new(0), ValueId::new(1)];
    let set_block = set_fn
        .get_block_mut(BasicBlockId::new(0))
        .expect("set entry");
    set_block.add_instruction(MirInstruction::FieldGet {
        dst: ValueId::new(2),
        base: ValueId::new(0),
        field: "values".to_string(),
        declared_type: Some(MirType::Box("ArrayBox".to_string())),
    });
    set_block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(3),
        src: ValueId::new(1),
    });
    set_block.add_instruction(MirInstruction::LegacyCallV0 {
        dst: None,
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "ArrayBox".to_string(),
            method: "push".to_string(),
            receiver: Some(ValueId::new(2)),
            certainty: TypeCertainty::Known,
            box_kind: CalleeBoxKind::RuntimeData,
        }),
        args: vec![ValueId::new(2), ValueId::new(3)],
        effects: EffectMask::IO,
    });
    set_block.add_instruction(MirInstruction::FieldGet {
        dst: ValueId::new(4),
        base: ValueId::new(0),
        field: "values".to_string(),
        declared_type: Some(MirType::Box("ArrayBox".to_string())),
    });
    set_block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(5),
        value: ConstValue::Integer(0),
    });
    set_block.add_instruction(MirInstruction::LegacyCallV0 {
        dst: Some(ValueId::new(6)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "RuntimeDataBox".to_string(),
            method: "get".to_string(),
            receiver: Some(ValueId::new(4)),
            certainty: TypeCertainty::Union,
            box_kind: CalleeBoxKind::RuntimeData,
        }),
        args: vec![ValueId::new(5)],
        effects: EffectMask::PURE,
    });
    set_block.add_instruction(MirInstruction::LegacyCallV0 {
        dst: None,
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "ArrayBox".to_string(),
            method: "set".to_string(),
            receiver: Some(ValueId::new(4)),
            certainty: TypeCertainty::Known,
            box_kind: CalleeBoxKind::RuntimeData,
        }),
        args: vec![ValueId::new(5), ValueId::new(6)],
        effects: EffectMask::IO,
    });

    let mut caller = MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let caller_block = caller
        .get_block_mut(BasicBlockId::new(0))
        .expect("caller entry");
    caller_block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(10),
        box_type: "Holder".to_string(),
        args: vec![],
    });
    caller_block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(11),
        value: ConstValue::Integer(7),
    });
    caller_block.add_instruction(MirInstruction::LegacyCallV0 {
        dst: None,
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "Holder".to_string(),
            method: "set".to_string(),
            receiver: Some(ValueId::new(10)),
            certainty: TypeCertainty::Known,
            box_kind: CalleeBoxKind::UserDefined,
        }),
        args: vec![ValueId::new(11)],
        effects: EffectMask::IO,
    });

    let mut read_fn = MirFunction::new(
        FunctionSignature {
            name: "Holder.read_first/0".to_string(),
            params: vec![MirType::Unknown],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    read_fn.params = vec![ValueId::new(0)];
    let read_block = read_fn
        .get_block_mut(BasicBlockId::new(0))
        .expect("read entry");
    read_block.add_instruction(MirInstruction::FieldGet {
        dst: ValueId::new(2),
        base: ValueId::new(0),
        field: "values".to_string(),
        declared_type: Some(MirType::Box("ArrayBox".to_string())),
    });
    read_block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Integer(0),
    });
    read_block.add_instruction(MirInstruction::LegacyCallV0 {
        dst: Some(ValueId::new(4)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "RuntimeDataBox".to_string(),
            method: "get".to_string(),
            receiver: Some(ValueId::new(2)),
            certainty: TypeCertainty::Union,
            box_kind: CalleeBoxKind::RuntimeData,
        }),
        args: vec![ValueId::new(3)],
        effects: EffectMask::PURE,
    });

    module.add_function(set_fn);
    module.add_function(caller);
    module.add_function(read_fn);
    refresh_module_typed_object_field_value_types(&mut module);
    refresh_module_typed_object_collection_field_element_value_types(&mut module);

    let read_fn = module
        .get_function("Holder.read_first/0")
        .expect("read function");
    assert_eq!(
        read_fn.metadata.value_types.get(&ValueId::new(4)),
        Some(&MirType::Integer)
    );
}
