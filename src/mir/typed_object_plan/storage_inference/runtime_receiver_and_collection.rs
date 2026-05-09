use super::*;

#[test]
fn build_typed_object_plans_uses_param_box_origins_for_runtime_method_receiver_storage() {
    let mut module = MirModule::new("typed_object_runtime_receiver_param_flow".to_string());
    module
        .metadata
        .user_box_decls
        .insert("Chunker".to_string(), Vec::new());
    module
        .metadata
        .user_box_decls
        .insert("Store".to_string(), Vec::new());
    module
        .metadata
        .user_box_decls
        .insert("Allocator".to_string(), Vec::new());
    module
        .metadata
        .user_box_decls
        .insert("Handle".to_string(), vec!["id".to_string()]);
    module.metadata.user_box_decls.insert(
        "Chunk".to_string(),
        vec!["cid".to_string(), "data".to_string(), "alloc".to_string()],
    );
    module.metadata.user_box_field_decls.insert(
        "Handle".to_string(),
        vec![UserBoxFieldDecl {
            name: "id".to_string(),
            declared_type_name: None,
            is_weak: false,
        }],
    );
    module.metadata.user_box_field_decls.insert(
        "Chunk".to_string(),
        vec![
            UserBoxFieldDecl {
                name: "cid".to_string(),
                declared_type_name: None,
                is_weak: false,
            },
            UserBoxFieldDecl {
                name: "data".to_string(),
                declared_type_name: None,
                is_weak: false,
            },
            UserBoxFieldDecl {
                name: "alloc".to_string(),
                declared_type_name: None,
                is_weak: false,
            },
        ],
    );

    let mut handle_birth = MirFunction::new(
        FunctionSignature {
            name: "Handle.birth/1".to_string(),
            params: vec![MirType::Unknown, MirType::Unknown],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    handle_birth.params = vec![ValueId::new(0), ValueId::new(1)];
    let mut handle_birth_block = BasicBlock::new(BasicBlockId::new(0));
    handle_birth_block.add_instruction(MirInstruction::FieldSet {
        base: ValueId::new(0),
        field: "id".to_string(),
        value: ValueId::new(1),
        declared_type: None,
    });
    handle_birth.add_block(handle_birth_block);

    let mut allocate = MirFunction::new(
        FunctionSignature {
            name: "Allocator.allocate/1".to_string(),
            params: vec![MirType::Unknown, MirType::Unknown],
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    allocate.params = vec![ValueId::new(0), ValueId::new(1)];
    let mut allocate_block = BasicBlock::new(BasicBlockId::new(0));
    allocate_block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(2),
        box_type: "Handle".to_string(),
        args: vec![ValueId::new(1)],
    });
    allocate_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });
    let mut allocate_null_block = BasicBlock::new(BasicBlockId::new(1));
    allocate_null_block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Void,
    });
    allocate_null_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    allocate.add_block(allocate_block);
    allocate.add_block(allocate_null_block);

    let mut chunk_birth = MirFunction::new(
        FunctionSignature {
            name: "Chunk.birth/3".to_string(),
            params: vec![
                MirType::Unknown,
                MirType::Unknown,
                MirType::Unknown,
                MirType::Unknown,
            ],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    chunk_birth.params = vec![
        ValueId::new(0),
        ValueId::new(1),
        ValueId::new(2),
        ValueId::new(3),
    ];
    let mut chunk_birth_block = BasicBlock::new(BasicBlockId::new(0));
    chunk_birth_block.add_instruction(MirInstruction::FieldSet {
        base: ValueId::new(0),
        field: "cid".to_string(),
        value: ValueId::new(1),
        declared_type: None,
    });
    chunk_birth_block.add_instruction(MirInstruction::FieldSet {
        base: ValueId::new(0),
        field: "data".to_string(),
        value: ValueId::new(2),
        declared_type: None,
    });
    chunk_birth_block.add_instruction(MirInstruction::FieldSet {
        base: ValueId::new(0),
        field: "alloc".to_string(),
        value: ValueId::new(3),
        declared_type: None,
    });
    chunk_birth.add_block(chunk_birth_block);

    let mut store_put = MirFunction::new(
        FunctionSignature {
            name: "Store.put/1".to_string(),
            params: vec![MirType::Unknown, MirType::Unknown],
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    store_put.params = vec![ValueId::new(0), ValueId::new(1)];
    let mut store_put_block = BasicBlock::new(BasicBlockId::new(0));
    store_put_block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::String("cid".to_string()),
    });
    store_put_block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(3),
        box_type: "Allocator".to_string(),
        args: vec![],
    });
    store_put_block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(4),
        value: ConstValue::Integer(7),
    });
    store_put_block.add_instruction(MirInstruction::Call {
        dst: Some(ValueId::new(5)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "RuntimeDataBox".to_string(),
            method: "allocate".to_string(),
            receiver: Some(ValueId::new(3)),
            certainty: crate::mir::definitions::call_unified::TypeCertainty::Union,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
        }),
        args: vec![ValueId::new(4)],
        effects: EffectMask::PURE,
    });
    store_put_block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(6),
        box_type: "Chunk".to_string(),
        args: vec![ValueId::new(2), ValueId::new(1), ValueId::new(5)],
    });
    store_put.add_block(store_put_block);

    let mut ingest = MirFunction::new(
        FunctionSignature {
            name: "Chunker.ingest/2".to_string(),
            params: vec![MirType::Unknown, MirType::Unknown, MirType::Unknown],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    ingest.params = vec![ValueId::new(0), ValueId::new(1), ValueId::new(2)];
    let mut ingest_block = BasicBlock::new(BasicBlockId::new(0));
    ingest_block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Integer(0),
    });
    ingest_block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(4),
        value: ConstValue::Integer(4),
    });
    ingest_block.add_instruction(MirInstruction::Call {
        dst: Some(ValueId::new(5)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "RuntimeDataBox".to_string(),
            method: "substring".to_string(),
            receiver: Some(ValueId::new(2)),
            certainty: crate::mir::definitions::call_unified::TypeCertainty::Union,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
        }),
        args: vec![ValueId::new(3), ValueId::new(4)],
        effects: EffectMask::PURE,
    });
    ingest_block.add_instruction(MirInstruction::Call {
        dst: Some(ValueId::new(6)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "RuntimeDataBox".to_string(),
            method: "put".to_string(),
            receiver: Some(ValueId::new(1)),
            certainty: crate::mir::definitions::call_unified::TypeCertainty::Union,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
        }),
        args: vec![ValueId::new(5)],
        effects: EffectMask::PURE,
    });
    ingest.add_block(ingest_block);

    let mut main = MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let mut main_block = BasicBlock::new(BasicBlockId::new(0));
    main_block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(1),
        box_type: "Store".to_string(),
        args: vec![],
    });
    main_block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(2),
        box_type: "Chunker".to_string(),
        args: vec![],
    });
    main_block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::String("payload".to_string()),
    });
    main_block.add_instruction(MirInstruction::Call {
        dst: None,
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "Chunker".to_string(),
            method: "ingest".to_string(),
            receiver: Some(ValueId::new(2)),
            certainty: crate::mir::definitions::call_unified::TypeCertainty::Known,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::UserDefined,
        }),
        args: vec![ValueId::new(1), ValueId::new(3)],
        effects: EffectMask::PURE,
    });
    main.add_block(main_block);

    module.add_function(handle_birth);
    module.add_function(allocate);
    module.add_function(chunk_birth);
    module.add_function(store_put);
    module.add_function(ingest);
    module.add_function(main);

    let plans = build_typed_object_plans(&module);
    let chunk = plans
        .iter()
        .find(|plan| plan.box_name == "Chunk")
        .expect("Chunk typed object plan");
    let handle = plans
        .iter()
        .find(|plan| plan.box_name == "Handle")
        .expect("Handle typed object plan");

    assert_eq!(handle.fields[0].storage, TypedObjectFieldStorage::I64);
    assert_eq!(chunk.fields[0].storage, TypedObjectFieldStorage::Handle);
    assert_eq!(chunk.fields[1].storage, TypedObjectFieldStorage::Handle);
    assert_eq!(chunk.fields[2].storage, TypedObjectFieldStorage::Handle);
}

#[test]
fn build_typed_object_plans_infers_birth_param_storage_from_collection_get_element_storage() {
    let mut module = MirModule::new("typed_object_collection_element_storage".to_string());
    module
        .metadata
        .user_box_decls
        .insert("Handle".to_string(), vec!["block_id".to_string()]);
    module.metadata.user_box_field_decls.insert(
        "Handle".to_string(),
        vec![UserBoxFieldDecl {
            name: "block_id".to_string(),
            declared_type_name: None,
            is_weak: false,
        }],
    );
    module
        .metadata
        .user_box_decls
        .insert("Page".to_string(), vec!["free_stack".to_string()]);
    module.metadata.user_box_field_decls.insert(
        "Page".to_string(),
        vec![UserBoxFieldDecl {
            name: "free_stack".to_string(),
            declared_type_name: None,
            is_weak: false,
        }],
    );

    let mut handle_birth = MirFunction::new(
        FunctionSignature {
            name: "Handle.birth/1".to_string(),
            params: vec![MirType::Box("Handle".to_string()), MirType::Unknown],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    handle_birth.params = vec![ValueId::new(0), ValueId::new(1)];
    let mut handle_birth_block = BasicBlock::new(BasicBlockId::new(0));
    handle_birth_block.add_instruction(MirInstruction::FieldSet {
        base: ValueId::new(0),
        field: "block_id".to_string(),
        value: ValueId::new(1),
        declared_type: None,
    });
    handle_birth.add_block(handle_birth_block);

    let mut page_birth = MirFunction::new(
        FunctionSignature {
            name: "Page.birth/0".to_string(),
            params: vec![MirType::Box("Page".to_string())],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    page_birth.params = vec![ValueId::new(0)];
    let mut page_birth_block = BasicBlock::new(BasicBlockId::new(0));
    page_birth_block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(1),
        box_type: "ArrayBox".to_string(),
        args: Vec::new(),
    });
    page_birth_block.add_instruction(MirInstruction::FieldSet {
        base: ValueId::new(0),
        field: "free_stack".to_string(),
        value: ValueId::new(1),
        declared_type: None,
    });
    page_birth.add_block(page_birth_block);

    let mut seed = MirFunction::new(
        FunctionSignature {
            name: "Page.seed/0".to_string(),
            params: vec![MirType::Box("Page".to_string())],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    seed.params = vec![ValueId::new(0)];
    let mut seed_block = BasicBlock::new(BasicBlockId::new(0));
    seed_block.add_instruction(MirInstruction::FieldGet {
        dst: ValueId::new(1),
        base: ValueId::new(0),
        field: "free_stack".to_string(),
        declared_type: None,
    });
    seed_block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::Integer(7),
    });
    seed_block.add_instruction(MirInstruction::Call {
        dst: Some(ValueId::new(3)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "RuntimeDataBox".to_string(),
            method: "push".to_string(),
            receiver: Some(ValueId::new(1)),
            certainty: crate::mir::definitions::call_unified::TypeCertainty::Union,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
        }),
        args: vec![ValueId::new(2)],
        effects: EffectMask::PURE,
    });
    seed.add_block(seed_block);

    let mut allocate = MirFunction::new(
        FunctionSignature {
            name: "Page.allocate/0".to_string(),
            params: vec![MirType::Box("Page".to_string())],
            return_type: MirType::Box("Handle".to_string()),
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    allocate.params = vec![ValueId::new(0)];
    let mut allocate_block = BasicBlock::new(BasicBlockId::new(0));
    allocate_block.add_instruction(MirInstruction::FieldGet {
        dst: ValueId::new(1),
        base: ValueId::new(0),
        field: "free_stack".to_string(),
        declared_type: None,
    });
    allocate_block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::Integer(0),
    });
    allocate_block.add_instruction(MirInstruction::Call {
        dst: Some(ValueId::new(3)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "RuntimeDataBox".to_string(),
            method: "get".to_string(),
            receiver: Some(ValueId::new(1)),
            certainty: crate::mir::definitions::call_unified::TypeCertainty::Union,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
        }),
        args: vec![ValueId::new(2)],
        effects: EffectMask::PURE,
    });
    allocate_block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(4),
        box_type: "Handle".to_string(),
        args: vec![ValueId::new(3)],
    });
    allocate.add_block(allocate_block);

    module.add_function(handle_birth);
    module.add_function(page_birth);
    module.add_function(seed);
    module.add_function(allocate);

    let plans = build_typed_object_plans(&module);
    let handle = plans
        .iter()
        .find(|plan| plan.box_name == "Handle")
        .expect("Handle plan");
    assert_eq!(handle.fields[0].storage, TypedObjectFieldStorage::I64);
}
