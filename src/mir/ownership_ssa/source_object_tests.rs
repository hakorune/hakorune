//! Source lifecycle contracts over the shared ownership test fixtures.
use super::*;

/// One Home: NewBox, Birth, a published alias, one End on Normal and the
/// matching Reclaim on the Birth Fault edge. Block 4 is the Fault End.
fn new_home_fixture() -> (MirFunction, hakorune_mir_defs::CanonicalObjectIdV1) {
    use crate::mir::instruction::{BirthInvokeV1, InvokeOperation};
    let object = hakorune_mir_defs::CanonicalObjectIdV1::from_declaration_index(0).unwrap();
    let (mut function, _) = function(&[], 9);
    function.signature.return_type = MirType::Void;
    function.get_block_mut(bb(0)).unwrap().add_instruction(MirInstruction::FaultFrameEnter {
        dst: value(9), mode: crate::mir::instruction::FaultFrameMode::RootOwned,
    });
    invoke(&mut function, 0, InvokeOperation::NewBox { object }, 1, 2);
    let normal = function.get_block_mut(bb(1)).unwrap();
    normal.add_instruction(MirInstruction::InvokeNormalResult { invoke_block: bb(0), dst: value(10) });
    normal.set_terminator(MirInstruction::Invoke {
        operation: InvokeOperation::Birth(BirthInvokeV1::from_value_arguments(
            hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::birth_constructor("Pair", 0), value(10), vec![], EffectMask::PURE)),
        fault_frame: value(9), normal_landing: bb(3), fault_landing: bb(4),
    });
    let normal = function.get_block_mut(bb(3)).unwrap();
    normal.add_instruction(MirInstruction::Copy { dst: value(11), src: value(10) });
    normal.set_terminator(MirInstruction::Invoke {
        operation: InvokeOperation::HomeRelease { object, value: value(11) }, fault_frame: value(9), normal_landing: bb(5), fault_landing: bb(6),
    });
    invoke(&mut function, 4, InvokeOperation::ReclaimUnpublished { object, value: value(10) }, 7, 8);
    for id in [2, 5, 6, 7, 8] {
        function.get_block_mut(bb(id)).unwrap().set_terminator(MirInstruction::Return { value: None });
    }
    (function, object)
}

#[test]
fn new_home_alias_ends_once_across_normal_and_fault_edges() {
    use crate::mir::instruction::InvokeOperation;
    let (mut function, object) = new_home_fixture();
    let owner = crate::mir::resolved_semantics::FunctionOwnerIssuerV1::new_for_compilation().unwrap().issue().unwrap();
    let abi = OwnershipFunctionAbiV1::new(OwnershipFunctionOwnerV1::source(owner), vec![], FunctionResultOwnershipV1::None)
        .with_source_aliases([(value(11), value(10))].into_iter().collect());
    assert!(verify_ownership_ssa_v1(&function, &abi).is_ok());
    let mut same_edge = function.clone();
    if let Some(MirInstruction::Invoke { fault_landing, .. }) = &mut same_edge.get_block_mut(bb(1)).unwrap().terminator {
        *fault_landing = bb(3);
    }
    assert!(matches!(verify_ownership_ssa_v1(&same_edge, &abi), Err(OwnershipSsaErrorV1::InvalidObjectLifecycle { .. })));
    let mut wrong_fault_end = function.clone();
    invoke(&mut wrong_fault_end, 4, InvokeOperation::HomeRelease { object, value: value(10) }, 7, 8);
    assert!(matches!(verify_ownership_ssa_v1(&wrong_fault_end, &abi), Err(OwnershipSsaErrorV1::InvalidObjectLifecycle { .. })));
    let mut scalar_end = function.clone();
    invoke(&mut scalar_end, 3, InvokeOperation::HomeRelease { object, value: value(9) }, 5, 6);
    assert!(matches!(verify_ownership_ssa_v1(&scalar_end, &abi), Err(OwnershipSsaErrorV1::InvalidObjectLifecycle { .. })));
    let mut escaped = function.clone();
    escaped.get_block_mut(bb(3)).unwrap().add_instruction(MirInstruction::Store { value: value(11), ptr: value(9) });
    assert!(matches!(verify_ownership_ssa_v1(&escaped, &abi), Err(OwnershipSsaErrorV1::UnsupportedManagedInstruction { .. })));
    invoke(&mut function, 5, InvokeOperation::HomeRelease { object, value: value(10) }, 6, 8);
    assert!(matches!(verify_ownership_ssa_v1(&function, &abi), Err(OwnershipSsaErrorV1::InvalidObjectLifecycle { .. })));
    // A take parameter starts as a published object; no synthetic allocation.
    let (mut entry, _) = self::function(&[MirOwnershipKindV1::Owned], 3);
    let parameter = entry.params[0];
    let entry_abi = OwnershipFunctionAbiV1::new(OwnershipFunctionOwnerV1::source(owner),
        vec![MirOwnershipKindV1::Owned], FunctionResultOwnershipV1::None)
        .with_source_owned_parameters([(parameter, object)].into_iter().collect());
    entry.get_block_mut(bb(0)).unwrap().add_instruction(MirInstruction::FaultFrameEnter {
        dst: value(9), mode: crate::mir::instruction::FaultFrameMode::RootOwned,
    });
    invoke(&mut entry, 0, InvokeOperation::HomeRelease { object, value: parameter }, 1, 2);
    for block in [1, 2] { entry.get_block_mut(bb(block)).unwrap().set_terminator(MirInstruction::Return { value: None }); }
    assert!(verify_ownership_ssa_v1(&entry, &entry_abi).is_ok());
    let target = hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::static_box_method("Consumer", "run", 1);
    let call_abi = entry_abi.clone().with_source_call_takes([(target.clone(), [(0, object)].into_iter().collect())].into_iter().collect());
    let mut caller = entry.clone();
    invoke(&mut caller, 0, InvokeOperation::Call {
            call: crate::mir::definitions::call_unified::MirCall::global(None, target.canonical_global_target_v1().unwrap(), vec![parameter]),
            result: crate::mir::instruction::InvokeCallResultKind::I64,
        }, 1, 2);
    assert!(verify_ownership_ssa_v1(&caller, &call_abi).is_ok());
    let r = verify_ownership_ssa_v1(&caller, &entry_abi);
    assert!(matches!(r, Err(OwnershipSsaErrorV1::ManagedCallOwnershipUnsupported { .. })), "a call without a declared take cannot move the parameter: {r:?}");
    // Entry transfer consumes on Fault as well: the caller cannot End again.
    invoke(&mut caller, 2, InvokeOperation::HomeRelease { object, value: parameter }, 1, 1);
    let r = verify_ownership_ssa_v1(&caller, &call_abi);
    assert!(matches!(r, Err(OwnershipSsaErrorV1::InvalidObjectLifecycle { .. })), "a moved parameter cannot be Ended on the Fault edge: {r:?}");
    let missing_class = entry_abi.clone().with_source_owned_parameters(Default::default());
    assert!(matches!(verify_ownership_ssa_v1(&entry, &missing_class), Err(OwnershipSsaErrorV1::InvalidObjectLifecycle { .. })));
    let mut wrong_class = entry.clone();
    if let Some(MirInstruction::Invoke { operation: InvokeOperation::HomeRelease { object, .. }, .. }) =
        &mut wrong_class.get_block_mut(bb(0)).unwrap().terminator {
        *object = hakorune_mir_defs::CanonicalObjectIdV1::from_declaration_index(1).unwrap();
    }
    assert!(matches!(verify_ownership_ssa_v1(&wrong_class, &entry_abi), Err(OwnershipSsaErrorV1::InvalidObjectLifecycle { .. })));
    entry.get_block_mut(bb(0)).unwrap().set_terminator(MirInstruction::Return { value: None });
    entry.blocks.retain(|id, _| *id == bb(0));
    assert!(matches!(verify_ownership_ssa_v1(&entry, &entry_abi), Err(OwnershipSsaErrorV1::MissingDispositionAtExit { .. })));

}

/// Hand-assembled MIR (not the real pipeline): the verifier admits a published object into a Birth only through a declared `take` ordinal; the permission is
/// the declared ordinal, a single transfer, and nothing may End the same value twice.
#[test]
fn hand_assembled_birth_take_permission_is_the_declared_ordinal_only() {
    use crate::mir::instruction::{BirthArgumentV1, BirthInvokeV1, InvokeOperation};
    let object = hakorune_mir_defs::CanonicalObjectIdV1::from_declaration_index(0).unwrap();
    let holder = hakorune_mir_defs::CanonicalObjectIdV1::from_declaration_index(1).unwrap();
    let key = hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::birth_constructor("Holder", 1);
    let owner = crate::mir::resolved_semantics::FunctionOwnerIssuerV1::new_for_compilation().unwrap().issue().unwrap();
    let (mut function, _) = self::function(&[MirOwnershipKindV1::Owned], 9);
    let parameter = function.params[0];
    let base_abi = OwnershipFunctionAbiV1::new(OwnershipFunctionOwnerV1::source(owner),
        vec![MirOwnershipKindV1::Owned], FunctionResultOwnershipV1::None)
        .with_source_owned_parameters([(parameter, object)].into_iter().collect())
        .with_source_aliases([(value(11), value(10))].into_iter().collect());
    function.get_block_mut(bb(0)).unwrap().add_instruction(MirInstruction::FaultFrameEnter {
        dst: value(9), mode: crate::mir::instruction::FaultFrameMode::RootOwned,
    });
    invoke(&mut function, 0, InvokeOperation::NewBox { object: holder }, 1, 2);
    function.get_block_mut(bb(1)).unwrap().add_instruction(MirInstruction::InvokeNormalResult { invoke_block: bb(0), dst: value(10) });
    invoke(&mut function, 1, InvokeOperation::Birth(BirthInvokeV1 {
        key: key.clone(), receiver: value(10), arguments: vec![BirthArgumentV1::Value(parameter)].into_boxed_slice(),
        effects: EffectMask::PURE,
    }), 3, 4);
    // Normal: the holder (which owns the moved object through its field) is Ended once.
    function.get_block_mut(bb(3)).unwrap().add_instruction(MirInstruction::Copy { dst: value(11), src: value(10) });
    invoke(&mut function, 3, InvokeOperation::HomeRelease { object: holder, value: value(11) }, 5, 6);
    // Birth Fault: the unpublished holder is reclaimed and the caller-owned actual is Ended.
    invoke(&mut function, 4, InvokeOperation::ReclaimUnpublished { object: holder, value: value(10) }, 7, 7);
    invoke(&mut function, 7, InvokeOperation::HomeRelease { object, value: parameter }, 8, 8);
    // The allocation Fault precedes any move: the caller still owns the actual and Ends it.
    invoke(&mut function, 2, InvokeOperation::HomeRelease { object, value: parameter }, 5, 6);
    for id in [5, 6, 8] {
        function.get_block_mut(bb(id)).unwrap().set_terminator(MirInstruction::Return { value: None });
    }
    let take_abi = base_abi.clone().with_source_birth_takes([(key.clone(), [0u32].into_iter().collect())].into_iter().collect());
    let accepted = verify_ownership_ssa_v1(&function, &take_abi);
    // The declared `take` ordinal admits the transfer; without the declaration it is refused.
    assert!(accepted.is_ok(), "declared take must admit the transfer: {accepted:?}");
    assert!(matches!(verify_ownership_ssa_v1(&function, &base_abi), Err(OwnershipSsaErrorV1::InvalidObjectLifecycle { .. })),
        "an undeclared formal gets no take permission");
    let wrong_ordinal = base_abi.clone().with_source_birth_takes([(key.clone(), [1u32].into_iter().collect())].into_iter().collect());
    assert!(matches!(verify_ownership_ssa_v1(&function, &wrong_ordinal), Err(OwnershipSsaErrorV1::InvalidObjectLifecycle { .. })),
        "a take on another ordinal does not cover this formal");
    // Ending the moved value again on the Normal path is a double End.
    let mut double_end = function.clone();
    invoke(&mut double_end, 5, InvokeOperation::HomeRelease { object, value: parameter }, 6, 6);
    assert!(matches!(verify_ownership_ssa_v1(&double_end, &take_abi), Err(OwnershipSsaErrorV1::InvalidObjectLifecycle { .. })), "a moved value cannot be Ended again");
    // The same value in two take positions is moved twice.
    let mut twice = function.clone();
    if let Some(MirInstruction::Invoke { operation: InvokeOperation::Birth(row), .. }) = &mut twice.get_block_mut(bb(1)).unwrap().terminator {
        row.arguments = vec![BirthArgumentV1::Value(parameter), BirthArgumentV1::Value(parameter)].into_boxed_slice();
    }
    let both = base_abi.with_source_birth_takes([(key, [0u32, 1].into_iter().collect())].into_iter().collect());
    assert!(matches!(verify_ownership_ssa_v1(&twice, &both), Err(OwnershipSsaErrorV1::InvalidObjectLifecycle { .. })), "one value cannot be taken twice");
}

/// The published-MIR check derives its own aliases and publication from the
/// NewBox, Birth and Copy instructions: no generator type feeds it. A Fault
/// End that is missing or doubled, or a Normal End that is missing, is refused
/// on the hand-assembled MIR; the unmodified MIR passes.
#[test]
fn published_homes_derive_their_inputs_and_refuse_a_missing_or_doubled_end() {
    use crate::mir::instruction::InvokeOperation;
    use super::published_homes::{verify_published_object_homes_v1 as check, PublishedHomeVerdictV1 as Verdict};
    let owner = crate::mir::resolved_semantics::FunctionOwnerIssuerV1::new_for_compilation().unwrap().issue().unwrap();
    let (births, calls) = (Default::default(), Default::default());
    let (function, object) = new_home_fixture();
    assert_eq!(check(&function, owner, &births, &calls, None), Ok(Verdict::Verified));

    let mut missing_fault_end = function.clone();
    missing_fault_end.get_block_mut(bb(4)).unwrap().set_terminator(MirInstruction::Return { value: None });
    missing_fault_end.blocks.retain(|id, _| *id != bb(7) && *id != bb(8));
    assert!(matches!(check(&missing_fault_end, owner, &births, &calls, None), Err(OwnershipSsaErrorV1::MissingDispositionAtExit { .. })));

    let mut doubled_fault_end = function.clone();
    invoke(&mut doubled_fault_end, 7, InvokeOperation::ReclaimUnpublished { object, value: value(10) }, 8, 8);
    assert!(matches!(check(&doubled_fault_end, owner, &births, &calls, None), Err(OwnershipSsaErrorV1::InvalidObjectLifecycle { .. })));

    let mut missing_normal_end = function.clone();
    missing_normal_end.get_block_mut(bb(3)).unwrap().set_terminator(MirInstruction::Return { value: None });
    missing_normal_end.blocks.retain(|id, _| *id != bb(5) && *id != bb(6));
    assert!(matches!(check(&missing_normal_end, owner, &births, &calls, None), Err(OwnershipSsaErrorV1::MissingDispositionAtExit { .. })));

    // No claim is made about a function the verifier cannot model.
    let mut float_parameter = function.clone();
    float_parameter.signature.params.push(MirType::Float);
    assert_eq!(check(&float_parameter, owner, &births, &calls, None), Ok(Verdict::OutsideModel));
    // A Home with a second copy (a read temporary) cannot be derived uniquely.
    let mut second_copy = function.clone();
    second_copy.get_block_mut(bb(3)).unwrap().add_instruction(MirInstruction::Copy { dst: value(12), src: value(10) });
    assert_eq!(check(&second_copy, owner, &births, &calls, None), Ok(Verdict::OutsideModel));
    let (without_home, _) = self::function(&[], 2);
    assert_eq!(check(&without_home, owner, &births, &calls, None), Ok(Verdict::NoHomes));
}

/// The public field declaration and a closed read interval grant a view, not
/// an owner. No generator descriptor or value_types hint participates.
#[test]
fn raw_child_view_requires_public_nonnullable_field_and_closed_live_interval() {
    use crate::mir::function::{CanonicalObjectDefinitionV1, ObjectDestructionDispositionV1, UserBoxFieldDecl};
    use hakorune_mir_defs::{CanonicalFieldRefV1, CanonicalObjectIdV1};
    let owner = crate::mir::resolved_semantics::FunctionOwnerIssuerV1::new_for_compilation().unwrap().issue().unwrap();
    let parent = CanonicalObjectIdV1::from_declaration_index(0).unwrap();
    let child = CanonicalObjectIdV1::from_declaration_index(1).unwrap();
    let child_field = CanonicalFieldRefV1::from_declaration_ordinal(parent, 0).unwrap();
    let scalar_field = CanonicalFieldRefV1::from_declaration_ordinal(child, 0).unwrap();
    let module = |declared: &str, weak: bool, scalar: &str| {
        let mut module = crate::mir::MirModule::new("borrow_interval".into());
        let definitions = [("Pair", "child", declared, weak), ("Leaf", "value", scalar, false)]
            .into_iter().map(|(class, name, declared, is_weak)| {
                let fields = vec![UserBoxFieldDecl { name: name.into(), declared_type_name: Some(declared.into()), is_weak }];
                module.metadata.user_box_decls.insert(class.into(), vec![name.into()]);
                module.metadata.user_box_field_decls.insert(class.into(), fields.clone());
                CanonicalObjectDefinitionV1::from_source_declaration(class.into(), fields.into_boxed_slice(),
                    Ok(()), ObjectDestructionDispositionV1::NUMERIC_NO_HOOK, Box::new([]))
            }).collect::<Vec<_>>();
        module.install_object_definitions_preflighted(definitions.into_boxed_slice());
        module.metadata.canonical_object_membership = Some([
            ("Pair".into(), parent), ("Leaf".into(), child),
        ].into_iter().collect());
        module.validate_object_definition_membership().unwrap();
        module
    };
    let (mut original, _) = new_home_fixture();
    original.get_block_mut(bb(3)).unwrap().add_instruction(MirInstruction::ObjectFieldGet {
        dst: value(12), base: value(11), field: child_field,
    });
    original.get_block_mut(bb(3)).unwrap().add_instruction(MirInstruction::ObjectFieldGet {
        dst: value(13), base: value(12), field: scalar_field,
    });
    let abi = OwnershipFunctionAbiV1::new(OwnershipFunctionOwnerV1::source(owner), vec![], FunctionResultOwnershipV1::None)
        .with_source_aliases([(value(11), value(10))].into_iter().collect());
    assert!(matches!(verify_ownership_ssa_v1(&original, &abi), Err(OwnershipSsaErrorV1::InvalidObjectLifecycle { .. })),
        "an absent module cannot grant a child type");
    let valid = module("Leaf", false, "i64");
    assert!(super::super::verify_ownership_ssa_with_module_v1(&original, &abi, &valid).is_ok());
    for bad in [module("Leaf?", false, "i64"), module("Leaf", true, "i64"),
        module("Leaf", false, "String"), module("Leaf", false, "ArrayBox")] {
        assert!(matches!(super::super::verify_ownership_ssa_with_module_v1(&original, &abi, &bad),
            Err(OwnershipSsaErrorV1::InvalidObjectLifecycle { .. })));
    }
    let mut bad_membership = valid.clone();
    bad_membership.metadata.canonical_object_membership = None;
    assert!(super::super::verify_ownership_ssa_with_module_v1(&original, &abi, &bad_membership).is_err());
    for mutation in 0..6 {
        let mut changed = original.clone();
        let body = &mut changed.get_block_mut(bb(3)).unwrap().instructions;
        match mutation {
            0 => body[2] = MirInstruction::ObjectFieldGet { dst: value(13), base: value(12), field: child_field },
            1 => body.push(MirInstruction::Copy { dst: value(14), src: value(12) }),
            2 => body.push(MirInstruction::Const { dst: value(12), value: crate::mir::ConstValue::Integer(0) }),
            3 => body.swap(1, 2),
            4 => body.insert(2, MirInstruction::Store { value: value(9), ptr: value(9) }),
            5 => {
                let read = body.remove(2);
                changed.get_block_mut(bb(5)).unwrap().add_instruction(read);
            }
            _ => unreachable!(),
        }
        assert!(super::super::verify_ownership_ssa_with_module_v1(&changed, &abi, &valid).is_err(),
            "invalid field view mutation {mutation}");
    }
    let mut after_end = original.clone();
    after_end.get_block_mut(bb(5)).unwrap().add_instruction(MirInstruction::ObjectFieldGet {
        dst: value(14), base: value(12), field: scalar_field,
    });
    assert!(super::super::verify_ownership_ssa_with_module_v1(&after_end, &abi, &valid).is_err(),
        "a released parent's child cannot be read on its continuation");
    let mut parameter_clash = original.clone();
    parameter_clash.params.push(value(12));
    parameter_clash.signature.params.push(MirType::Integer);
    let parameter_abi = OwnershipFunctionAbiV1::new(OwnershipFunctionOwnerV1::source(owner),
        vec![MirOwnershipKindV1::None], FunctionResultOwnershipV1::None)
        .with_source_aliases([(value(11), value(10))].into_iter().collect());
    assert!(super::super::verify_ownership_ssa_with_module_v1(&parameter_clash, &parameter_abi, &valid).is_err(),
        "a borrowed view cannot redefine a parameter");
}

/// Object ownership delegates Text only for the same source module image.
/// Affine String End checks remain in the mandatory independent String tests.
#[test]
fn source_module_text_domain_does_not_grant_standalone_or_carrier_permission() {
    use crate::mir::instruction::{InvokeOperation as Op, StringInvokeOperand as Text, StringInvokeOperation as StringOp};
    let (mut function, _) = function(&[], 5);
    function.signature.return_type = MirType::Void;
    function.get_block_mut(bb(0)).unwrap().add_instruction(MirInstruction::FaultFrameEnter {
        dst: value(9), mode: crate::mir::instruction::FaultFrameMode::RootOwned,
    });
    invoke(&mut function, 0, Op::String(StringOp::Add {
        left: Text::StaticText { utf8: "a".into() }, right: Text::StaticText { utf8: "b".into() },
    }), 1, 2);
    function.get_block_mut(bb(1)).unwrap().add_instruction(MirInstruction::InvokeNormalResult {
        invoke_block: bb(0), dst: value(10),
    });
    invoke(&mut function, 1, Op::String(StringOp::End { value: value(10) }), 3, 4);
    for block in [2, 3, 4] {
        function.get_block_mut(bb(block)).unwrap().set_terminator(MirInstruction::Return { value: None });
    }
    let owner = crate::mir::resolved_semantics::FunctionOwnerIssuerV1::new_for_compilation().unwrap().issue().unwrap();
    let abi = OwnershipFunctionAbiV1::new(OwnershipFunctionOwnerV1::source(owner), vec![], FunctionResultOwnershipV1::None);
    assert!(matches!(verify_ownership_ssa_v1(&function, &abi), Err(OwnershipSsaErrorV1::InvalidAcquisition { .. })));
    let name = function.signature.name.clone();
    let mut module = crate::mir::MirModule::new("object-domain".into());
    module.functions.insert(name.clone(), function);
    let function = &module.functions[&name];
    assert!(super::super::verify_ownership_ssa_with_module_v1(function, &abi, &module).is_ok());
    let foreign = function.clone();
    assert!(matches!(super::super::verify_ownership_ssa_with_module_v1(&foreign, &abi, &module),
        Err(OwnershipSsaErrorV1::InvalidAcquisition { .. })));
    let function = module.functions.get_mut(&name).unwrap();
    if let Some(MirInstruction::Invoke { normal_landing, .. }) = &mut function.get_block_mut(bb(0)).unwrap().terminator {
        *normal_landing = bb(3);
    }
    assert!(matches!(super::super::verify_ownership_ssa_with_module_v1(&module.functions[&name], &abi, &module),
        Err(OwnershipSsaErrorV1::InvalidAcquisition { .. })));
    let function = module.functions.get_mut(&name).unwrap();
    invoke(function, 0, Op::DynamicCarrierSubI64 { carrier: value(7), rhs: value(8) }, 1, 2);
    assert!(matches!(super::super::verify_ownership_ssa_with_module_v1(&module.functions[&name], &abi, &module),
        Err(OwnershipSsaErrorV1::InvalidAcquisition { .. })));
}
