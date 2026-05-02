use super::*;
use crate::mir::{
    definitions::call_unified::{CalleeBoxKind, TypeCertainty},
    thin_entry::{
        ThinEntryCurrentCarrier, ThinEntryDemand, ThinEntryPreferredEntry, ThinEntryValueClass,
    },
    thin_entry_selection::{ThinEntrySelection, ThinEntrySelectionState},
    EffectMask, FunctionSignature, MirType,
};

fn make_function(name: &str) -> MirFunction {
    MirFunction::new(
        FunctionSignature {
            name: name.to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    )
}

fn make_method(name: &str) -> MirFunction {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: name.to_string(),
            params: vec![MirType::Box("Self".to_string())],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(1),
    );
    function.params = vec![ValueId::new(0)];
    function
}

fn add_counter_step_main(function: &mut MirFunction, copy: bool) {
    let block = function.get_block_mut(BasicBlockId::new(0)).unwrap();
    block.add_instruction(const_i(1, 41));
    block.add_instruction(newbox(2, "Counter"));
    block.add_instruction(field_set(2, "value", 1, "IntegerBox"));
    let receiver = if copy {
        block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(3),
            src: ValueId::new(2),
        });
        ValueId::new(3)
    } else {
        ValueId::new(2)
    };
    let result = if copy { 4 } else { 3 };
    block.add_instruction(method_call_inst(result, "Counter", "step", receiver));
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(result)),
    });
    function.metadata.thin_entry_selections = vec![
        selection(
            0,
            2,
            None,
            ThinEntrySurface::UserBoxFieldSet,
            "Counter.value",
            "user_box_field_set.inline_scalar",
            ThinEntryValueClass::InlineI64,
        ),
        selection(
            0,
            if copy { 4 } else { 3 },
            Some(result),
            ThinEntrySurface::UserBoxMethod,
            "Counter.step",
            "user_box_method.known_receiver",
            ThinEntryValueClass::Unknown,
        ),
    ];
}

fn counter_step_method() -> MirFunction {
    let mut function = make_method("Counter.step/1");
    let block = function.get_block_mut(BasicBlockId::new(1)).unwrap();
    block.add_instruction(field_get(1, 0, "value", "IntegerBox"));
    block.add_instruction(const_i(2, 2));
    block.add_instruction(binop(3, 1, 2));
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    function.metadata.thin_entry_selections = vec![selection(
        1,
        0,
        Some(1),
        ThinEntrySurface::UserBoxFieldGet,
        "Counter.value",
        "user_box_field_get.inline_scalar",
        ThinEntryValueClass::InlineI64,
    )];
    function
}

fn add_point_sum_main(function: &mut MirFunction, copy: bool) {
    let block = function.get_block_mut(BasicBlockId::new(0)).unwrap();
    block.add_instruction(const_i(1, 1));
    block.add_instruction(const_i(2, 2));
    block.add_instruction(newbox(3, "Point"));
    block.add_instruction(field_set(3, "x", 1, "IntegerBox"));
    block.add_instruction(field_set(3, "y", 2, "IntegerBox"));
    let receiver = if copy {
        block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(4),
            src: ValueId::new(3),
        });
        ValueId::new(4)
    } else {
        ValueId::new(3)
    };
    let result = if copy { 5 } else { 4 };
    block.add_instruction(method_call_inst(result, "Point", "sum", receiver));
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(result)),
    });
    function.metadata.thin_entry_selections = vec![
        selection(
            0,
            3,
            None,
            ThinEntrySurface::UserBoxFieldSet,
            "Point.x",
            "user_box_field_set.inline_scalar",
            ThinEntryValueClass::InlineI64,
        ),
        selection(
            0,
            4,
            None,
            ThinEntrySurface::UserBoxFieldSet,
            "Point.y",
            "user_box_field_set.inline_scalar",
            ThinEntryValueClass::InlineI64,
        ),
        selection(
            0,
            if copy { 6 } else { 5 },
            Some(result),
            ThinEntrySurface::UserBoxMethod,
            "Point.sum",
            "user_box_method.known_receiver",
            ThinEntryValueClass::Unknown,
        ),
    ];
}

fn point_sum_method() -> MirFunction {
    let mut function = make_method("Point.sum/1");
    let block = function.get_block_mut(BasicBlockId::new(1)).unwrap();
    block.add_instruction(field_get(1, 0, "x", "IntegerBox"));
    block.add_instruction(field_get(2, 0, "y", "IntegerBox"));
    block.add_instruction(binop(3, 1, 2));
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    function.metadata.thin_entry_selections = vec![
        selection(
            1,
            0,
            Some(1),
            ThinEntrySurface::UserBoxFieldGet,
            "Point.x",
            "user_box_field_get.inline_scalar",
            ThinEntryValueClass::InlineI64,
        ),
        selection(
            1,
            1,
            Some(2),
            ThinEntrySurface::UserBoxFieldGet,
            "Point.y",
            "user_box_field_get.inline_scalar",
            ThinEntryValueClass::InlineI64,
        ),
    ];
    function
}

#[test]
fn userbox_known_receiver_method_seed_detects_counter_step_local_and_copy() {
    for copy in [false, true] {
        let mut module = MirModule::new("counter_step_route_test".to_string());
        let mut main = make_function("main");
        add_counter_step_main(&mut main, copy);
        module.add_function(main);
        module.add_function(counter_step_method());

        refresh_module_userbox_known_receiver_method_seed_routes(&mut module);
        let route = module
            .functions
            .get("main")
            .and_then(|function| {
                function
                    .metadata
                    .userbox_known_receiver_method_seed_route
                    .as_ref()
            })
            .expect("counter step route");

        assert_eq!(route.box_name, "Counter");
        assert_eq!(route.method, "step");
        assert_eq!(route.copy_value.is_some(), copy);
        assert_eq!(
            route.kind,
            if copy {
                UserBoxKnownReceiverMethodSeedKind::CounterStepCopyLocalI64
            } else {
                UserBoxKnownReceiverMethodSeedKind::CounterStepLocalI64
            }
        );
        assert_eq!(
            route.payload,
            UserBoxKnownReceiverMethodSeedPayload::CounterStepI64 {
                base_i64: 41,
                delta_i64: 2
            }
        );
    }
}

#[test]
fn userbox_known_receiver_method_seed_detects_point_sum_local_and_copy() {
    for copy in [false, true] {
        let mut module = MirModule::new("point_sum_route_test".to_string());
        let mut main = make_function("main");
        add_point_sum_main(&mut main, copy);
        module.add_function(main);
        module.add_function(point_sum_method());

        refresh_module_userbox_known_receiver_method_seed_routes(&mut module);
        let route = module
            .functions
            .get("main")
            .and_then(|function| {
                function
                    .metadata
                    .userbox_known_receiver_method_seed_route
                    .as_ref()
            })
            .expect("point sum route");

        assert_eq!(route.box_name, "Point");
        assert_eq!(route.method, "sum");
        assert_eq!(route.copy_value.is_some(), copy);
        assert_eq!(
            route.kind,
            if copy {
                UserBoxKnownReceiverMethodSeedKind::PointSumCopyLocalI64
            } else {
                UserBoxKnownReceiverMethodSeedKind::PointSumLocalI64
            }
        );
        assert_eq!(
            route.payload,
            UserBoxKnownReceiverMethodSeedPayload::PointSumI64 { x_i64: 1, y_i64: 2 }
        );
    }
}

#[test]
fn userbox_known_receiver_method_seed_requires_thin_method_selection() {
    let mut module = MirModule::new("counter_step_route_negative_test".to_string());
    let mut main = make_function("main");
    add_counter_step_main(&mut main, false);
    main.metadata
        .thin_entry_selections
        .retain(|selection| selection.surface != ThinEntrySurface::UserBoxMethod);
    module.add_function(main);
    module.add_function(counter_step_method());

    refresh_module_userbox_known_receiver_method_seed_routes(&mut module);
    assert!(module
        .functions
        .get("main")
        .unwrap()
        .metadata
        .userbox_known_receiver_method_seed_route
        .is_none());
}

fn const_i(dst: u32, value: i64) -> MirInstruction {
    MirInstruction::Const {
        dst: ValueId::new(dst),
        value: ConstValue::Integer(value),
    }
}

fn newbox(dst: u32, box_type: &str) -> MirInstruction {
    MirInstruction::NewBox {
        dst: ValueId::new(dst),
        box_type: box_type.to_string(),
        args: vec![],
    }
}

fn field_set(base: u32, field: &str, value: u32, declared_box: &str) -> MirInstruction {
    MirInstruction::FieldSet {
        base: ValueId::new(base),
        field: field.to_string(),
        value: ValueId::new(value),
        declared_type: Some(MirType::Box(declared_box.to_string())),
    }
}

fn field_get(dst: u32, base: u32, field: &str, declared_box: &str) -> MirInstruction {
    MirInstruction::FieldGet {
        dst: ValueId::new(dst),
        base: ValueId::new(base),
        field: field.to_string(),
        declared_type: Some(MirType::Box(declared_box.to_string())),
    }
}

fn binop(dst: u32, lhs: u32, rhs: u32) -> MirInstruction {
    MirInstruction::BinOp {
        dst: ValueId::new(dst),
        op: BinaryOp::Add,
        lhs: ValueId::new(lhs),
        rhs: ValueId::new(rhs),
    }
}

fn method_call_inst(dst: u32, box_name: &str, method: &str, receiver: ValueId) -> MirInstruction {
    MirInstruction::Call {
        dst: Some(ValueId::new(dst)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: box_name.to_string(),
            method: method.to_string(),
            receiver: Some(receiver),
            certainty: TypeCertainty::Known,
            box_kind: CalleeBoxKind::UserDefined,
        }),
        args: vec![],
        effects: EffectMask::PURE,
    }
}

fn selection(
    block: u32,
    instruction_index: usize,
    value: Option<u32>,
    surface: ThinEntrySurface,
    subject: &str,
    manifest_row: &'static str,
    value_class: ThinEntryValueClass,
) -> ThinEntrySelection {
    ThinEntrySelection {
        block: BasicBlockId::new(block),
        instruction_index,
        value: value.map(ValueId::new),
        surface,
        subject: subject.to_string(),
        manifest_row,
        selected_entry: ThinEntryPreferredEntry::ThinInternalEntry,
        state: ThinEntrySelectionState::Candidate,
        current_carrier: ThinEntryCurrentCarrier::BackendTyped,
        value_class,
        demand: ThinEntryDemand::InlineScalar,
        reason: "test".to_string(),
    }
}
