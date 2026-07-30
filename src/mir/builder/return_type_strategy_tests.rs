use super::{
    primary_hint, resolve_known_instance_method_return_type, resolve_known_typeop_return_type,
    uniform_phi,
};
use crate::mir::{
    BasicBlock, BasicBlockId, ConstValue, EffectMask, FunctionSignature, MirFunction,
    MirInstruction, MirType,
};
use std::collections::BTreeMap;

#[test]
fn primary_hint_and_uniform_fallback_partition_function_names() {
    for name in [
        "IfSelectTest.simple/0",
        "IfMergeTest.simple/0",
        "read_quoted_from/1",
        "NewBoxTest.array/0",
    ] {
        assert!(primary_hint::is_primary_target(name));
        assert!(!primary_hint::is_uniform_phi_fallback_target(name));
    }

    for name in ["Main.main/0", "ArrayProcessor.process/1"] {
        assert!(!primary_hint::is_primary_target(name));
        assert!(primary_hint::is_uniform_phi_fallback_target(name));
    }
    assert!(!primary_hint::is_uniform_phi_fallback_target(""));
}

#[test]
fn uniform_phi_fallback_requires_one_incoming_type() {
    let entry = BasicBlockId::new(0);
    let left = BasicBlockId::new(1);
    let right = BasicBlockId::new(2);
    let merge = BasicBlockId::new(3);
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "ArrayProcessor.process/1".into(),
            params: vec![],
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        },
        entry,
    );
    function.add_block(BasicBlock::new(left));
    function.add_block(BasicBlock::new(right));
    function.add_block(BasicBlock::new(merge));

    let left_value = function.next_value_id();
    let right_value = function.next_value_id();
    let return_value = function.next_value_id();
    function
        .get_block_mut(left)
        .unwrap()
        .add_instruction(MirInstruction::Const {
            dst: left_value,
            value: ConstValue::Integer(1),
        });
    function
        .get_block_mut(right)
        .unwrap()
        .add_instruction(MirInstruction::Const {
            dst: right_value,
            value: ConstValue::Integer(2),
        });
    function
        .get_block_mut(merge)
        .unwrap()
        .add_instruction(MirInstruction::Phi {
            dst: return_value,
            inputs: vec![(left, left_value), (right, right_value)],
            type_hint: None,
        });

    let mut types = BTreeMap::new();
    types.insert(left_value, MirType::Integer);
    types.insert(right_value, MirType::Integer);
    assert_eq!(
        uniform_phi::resolve_from_phi(&function, return_value, &types),
        Some(MirType::Integer)
    );
    types.insert(right_value, MirType::Bool);
    assert_eq!(
        uniform_phi::resolve_from_phi(&function, return_value, &types),
        None
    );
}

#[test]
fn known_instance_method_return_types_use_existing_annotation_policy() {
    for (box_name, method, expected) in [
        ("StringBox", "length", Some(MirType::Integer)),
        ("ArrayBox", "push", Some(MirType::Void)),
        ("IntegerBox", "str", Some(MirType::String)),
        ("MapBox", "has", Some(MirType::Bool)),
        ("UnknownBox", "unknown_method", None),
    ] {
        assert_eq!(
            resolve_known_instance_method_return_type(Some(&MirType::Box(box_name.into())), method),
            expected,
        );
    }
}

#[test]
fn known_return_definition_typeop_policy_is_exact() {
    assert_eq!(
        resolve_known_typeop_return_type(&crate::mir::TypeOpKind::Check, &MirType::Integer),
        MirType::Bool,
    );
    assert_eq!(
        resolve_known_typeop_return_type(&crate::mir::TypeOpKind::Cast, &MirType::String),
        MirType::String,
    );
}
