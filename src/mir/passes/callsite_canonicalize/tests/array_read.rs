use super::*;

fn array_method_get(receiver: ValueId, certainty: TypeCertainty) -> Callee {
    Callee::Method {
        box_name: "ArrayBox".to_string(),
        method: "get".to_string(),
        receiver: Some(receiver),
        certainty,
        box_kind: CalleeBoxKind::RuntimeData,
    }
}

fn entry_block(func: &mut MirFunction) -> &mut crate::mir::BasicBlock {
    func.blocks
        .get_mut(&BasicBlockId(0))
        .expect("entry block exists")
}

fn new_test_function() -> MirFunction {
    let signature = FunctionSignature {
        name: "main/0".to_string(),
        params: vec![],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    MirFunction::new(signature, BasicBlockId(0))
}

fn block0<'m>(module: &'m MirModule) -> &'m crate::mir::BasicBlock {
    module
        .get_function("main/0")
        .expect("function exists")
        .blocks
        .get(&BasicBlockId(0))
        .expect("entry block exists")
}

#[test]
fn proven_known_array_get_canonicalizes_to_array_element_read() {
    let mut module = MirModule::new("array_read_known".to_string());
    let mut func = new_test_function();
    let block = entry_block(&mut func);
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(1),
        value: crate::mir::ConstValue::Integer(0),
    });
    block.instruction_spans.push(Span::unknown());
    block
        .instructions
        .push(MirInstruction::Call(crate::mir::definitions::MirCall {
            dst: Some(ValueId(2)),
            callee: array_method_get(ValueId(10), TypeCertainty::Known),
            args: vec![ValueId(1)],
            flags: crate::mir::definitions::call_unified::CallFlags::default(),
            effects: EffectMask::PURE,
        }));
    block.instruction_spans.push(Span::unknown());
    module.add_function(func);

    let rewritten = canonicalize_callsites(&mut module);
    assert_eq!(rewritten, 1);
    assert!(matches!(
        &block0(&module).instructions[1],
        MirInstruction::ArrayElementRead {
            site_id,
            dst: Some(ValueId(2)),
            receiver: ValueId(10),
            index: ValueId(1),
        } if site_id.0 == 0
    ));
}

#[test]
fn union_receiver_with_arraybox_origin_canonicalizes() {
    let mut module = MirModule::new("array_read_origin".to_string());
    let mut func = new_test_function();
    let block = entry_block(&mut func);
    block.instructions.push(MirInstruction::NewBox {
        dst: ValueId(10),
        target: crate::mir::ConstructionTarget::Named("ArrayBox".to_string()),
        args: vec![],
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(1),
        value: crate::mir::ConstValue::Integer(0),
    });
    block.instruction_spans.push(Span::unknown());
    block
        .instructions
        .push(MirInstruction::Call(crate::mir::definitions::MirCall {
            dst: Some(ValueId(2)),
            callee: array_method_get(ValueId(10), TypeCertainty::Union),
            args: vec![ValueId(1)],
            flags: crate::mir::definitions::call_unified::CallFlags::default(),
            effects: EffectMask::PURE,
        }));
    block.instruction_spans.push(Span::unknown());
    module.add_function(func);

    let rewritten = canonicalize_callsites(&mut module);
    assert_eq!(rewritten, 1);
    assert!(matches!(
        &block0(&module).instructions[2],
        MirInstruction::ArrayElementRead {
            dst: Some(ValueId(2)),
            receiver: ValueId(10),
            index: ValueId(1),
            ..
        }
    ));
}

#[test]
fn union_receiver_without_arraybox_origin_stays_untouched() {
    let mut module = MirModule::new("array_read_union".to_string());
    let mut func = new_test_function();
    let block = entry_block(&mut func);
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(10),
        value: crate::mir::ConstValue::Integer(0),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(1),
        value: crate::mir::ConstValue::Integer(0),
    });
    block.instruction_spans.push(Span::unknown());
    block
        .instructions
        .push(MirInstruction::Call(crate::mir::definitions::MirCall {
            dst: Some(ValueId(2)),
            callee: array_method_get(ValueId(10), TypeCertainty::Union),
            args: vec![ValueId(1)],
            flags: crate::mir::definitions::call_unified::CallFlags::default(),
            effects: EffectMask::PURE,
        }));
    block.instruction_spans.push(Span::unknown());
    module.add_function(func);

    let rewritten = canonicalize_callsites(&mut module);
    assert_eq!(rewritten, 0);
    assert!(matches!(
        &block0(&module).instructions[2],
        MirInstruction::Call(_)
    ));
}

#[test]
fn non_arraybox_receiver_classes_stay_untouched() {
    for box_name in ["DirectArrayI64", "MapBox", "RuntimeDataBox"] {
        let mut module = MirModule::new(format!("array_read_{box_name}"));
        let mut func = new_test_function();
        let block = entry_block(&mut func);
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(1),
            value: crate::mir::ConstValue::Integer(0),
        });
        block.instruction_spans.push(Span::unknown());
        block
            .instructions
            .push(MirInstruction::Call(crate::mir::definitions::MirCall {
                dst: Some(ValueId(2)),
                callee: Callee::Method {
                    box_name: box_name.to_string(),
                    method: "get".to_string(),
                    receiver: Some(ValueId(10)),
                    certainty: TypeCertainty::Known,
                    box_kind: CalleeBoxKind::RuntimeData,
                },
                args: vec![ValueId(1)],
                flags: crate::mir::definitions::call_unified::CallFlags::default(),
                effects: EffectMask::PURE,
            }));
        block.instruction_spans.push(Span::unknown());
        module.add_function(func);

        let rewritten = canonicalize_callsites(&mut module);
        assert_eq!(rewritten, 0, "{box_name} must not canonicalize");
        assert!(matches!(
            &block0(&module).instructions[1],
            MirInstruction::Call(_)
        ));
    }
}

#[test]
fn legacy_call_v0_array_get_is_never_laundered() {
    let mut module = MirModule::new("array_read_legacy".to_string());
    let mut func = new_test_function();
    let block = entry_block(&mut func);
    block.instructions.push(MirInstruction::LegacyCallV0 {
        dst: Some(ValueId(2)),
        func: ValueId::INVALID,
        callee: Some(array_method_get(ValueId(10), TypeCertainty::Known)),
        args: vec![ValueId(1)],
        effects: EffectMask::PURE,
    });
    block.instruction_spans.push(Span::unknown());
    module.add_function(func);

    let rewritten = canonicalize_callsites(&mut module);
    assert_eq!(rewritten, 0);
    assert!(matches!(
        &block0(&module).instructions[0],
        MirInstruction::LegacyCallV0 {
            callee: Some(Callee::Method { .. }),
            ..
        }
    ));
}

#[test]
fn malformed_arity_stays_untouched() {
    let mut module = MirModule::new("array_read_arity".to_string());
    let mut func = new_test_function();
    let block = entry_block(&mut func);
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(1),
        value: crate::mir::ConstValue::Integer(0),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(3),
        value: crate::mir::ConstValue::Integer(1),
    });
    block.instruction_spans.push(Span::unknown());
    block
        .instructions
        .push(MirInstruction::Call(crate::mir::definitions::MirCall {
            dst: Some(ValueId(2)),
            callee: array_method_get(ValueId(10), TypeCertainty::Known),
            args: vec![ValueId(1), ValueId(3)],
            flags: crate::mir::definitions::call_unified::CallFlags::default(),
            effects: EffectMask::PURE,
        }));
    block.instruction_spans.push(Span::unknown());
    module.add_function(func);

    let rewritten = canonicalize_callsites(&mut module);
    assert_eq!(rewritten, 0);
    assert!(matches!(
        &block0(&module).instructions[2],
        MirInstruction::Call(_)
    ));
}

#[test]
fn receiver_duplicated_arg_surface_uses_the_real_index() {
    let mut module = MirModule::new("array_read_dup".to_string());
    let mut func = new_test_function();
    let block = entry_block(&mut func);
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(1),
        value: crate::mir::ConstValue::Integer(0),
    });
    block.instruction_spans.push(Span::unknown());
    block
        .instructions
        .push(MirInstruction::Call(crate::mir::definitions::MirCall {
            dst: Some(ValueId(2)),
            callee: array_method_get(ValueId(10), TypeCertainty::Known),
            args: vec![ValueId(10), ValueId(1)],
            flags: crate::mir::definitions::call_unified::CallFlags::default(),
            effects: EffectMask::PURE,
        }));
    block.instruction_spans.push(Span::unknown());
    module.add_function(func);

    let rewritten = canonicalize_callsites(&mut module);
    assert_eq!(rewritten, 1);
    assert!(matches!(
        &block0(&module).instructions[1],
        MirInstruction::ArrayElementRead {
            receiver: ValueId(10),
            index: ValueId(1),
            ..
        }
    ));
}

#[test]
fn canonicalized_read_reaches_the_sole_published_consumer() {
    use crate::mir::function::{PublishedMirBackendView, PublishedStaticMethodRouteV1};

    let mut module = MirModule::new("array_read_view".to_string());
    let mut func = new_test_function();
    let block = entry_block(&mut func);
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(1),
        value: crate::mir::ConstValue::Integer(0),
    });
    block.instruction_spans.push(Span::unknown());
    block
        .instructions
        .push(MirInstruction::Call(crate::mir::definitions::MirCall {
            dst: Some(ValueId(2)),
            callee: array_method_get(ValueId(10), TypeCertainty::Known),
            args: vec![ValueId(1)],
            flags: crate::mir::definitions::call_unified::CallFlags::default(),
            effects: EffectMask::PURE,
        }));
    block.instruction_spans.push(Span::unknown());
    module.add_function(func);

    assert_eq!(canonicalize_callsites(&mut module), 1);

    let view = PublishedMirBackendView::try_new(&module).expect("canonicalized read view");
    assert_eq!(view.route(), PublishedStaticMethodRouteV1::CanonicalTyped);
    assert_eq!(view.array_element_reads().len(), 1);
    assert_eq!(view.array_element_reads()[0].index(), ValueId(1));
    assert_eq!(view.array_element_reads()[0].dst(), Some(ValueId(2)));
}

#[test]
fn site_id_continues_above_existing_read_sites() {
    let mut module = MirModule::new("array_read_site".to_string());
    let mut func = new_test_function();
    let block = entry_block(&mut func);
    block.instructions.push(MirInstruction::ArrayElementRead {
        site_id: crate::mir::ArrayReadSiteId::new(5),
        dst: Some(ValueId(20)),
        receiver: ValueId(11),
        index: ValueId(12),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(1),
        value: crate::mir::ConstValue::Integer(0),
    });
    block.instruction_spans.push(Span::unknown());
    block
        .instructions
        .push(MirInstruction::Call(crate::mir::definitions::MirCall {
            dst: Some(ValueId(2)),
            callee: array_method_get(ValueId(10), TypeCertainty::Known),
            args: vec![ValueId(1)],
            flags: crate::mir::definitions::call_unified::CallFlags::default(),
            effects: EffectMask::PURE,
        }));
    block.instruction_spans.push(Span::unknown());
    module.add_function(func);

    let rewritten = canonicalize_callsites(&mut module);
    assert_eq!(rewritten, 1);
    assert!(matches!(
        &block0(&module).instructions[2],
        MirInstruction::ArrayElementRead {
            site_id,
            index: ValueId(1),
            ..
        } if site_id.0 == 6
    ));
}
