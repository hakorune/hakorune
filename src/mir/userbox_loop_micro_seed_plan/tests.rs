use super::*;
use crate::mir::{
    thin_entry::{ThinEntryCurrentCarrier, ThinEntryDemand, ThinEntryValueClass},
    thin_entry_selection::ThinEntrySelectionState,
    BasicBlock, EffectMask, FunctionSignature,
};

fn make_function() -> MirFunction {
    MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    )
}

fn add_point_add_micro_body(function: &mut MirFunction) {
    let entry = function.get_block_mut(BasicBlockId::new(0)).unwrap();
    entry.add_instruction(newbox(8, "Point"));
    entry.add_instruction(const_i(13, 1));
    entry.add_instruction(field_set(8, "x", 13, "IntegerBox"));
    entry.add_instruction(const_i(15, 2));
    entry.add_instruction(field_set(8, "y", 15, "IntegerBox"));
    entry.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(20),
        edge_args: None,
    });

    let mut header = BasicBlock::new(BasicBlockId::new(20));
    header.add_instruction(const_i(40, 2_000_000));
    header.add_instruction(compare(36, CompareOp::Lt, 38, 39));
    header.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(36),
        then_bb: BasicBlockId::new(21),
        else_bb: BasicBlockId::new(23),
        then_edge_args: None,
        else_edge_args: None,
    });
    function.add_block(header);

    let mut body = BasicBlock::new(BasicBlockId::new(21));
    body.add_instruction(field_get(26, 8, "x", "IntegerBox"));
    body.add_instruction(field_get(27, 8, "y", "IntegerBox"));
    body.add_instruction(field_get(29, 8, "x", "IntegerBox"));
    body.add_instruction(binop(47, 48, 49));
    body.add_instruction(field_set(8, "x", 47, "IntegerBox"));
    body.add_instruction(field_get(32, 8, "y", "IntegerBox"));
    body.add_instruction(binop(52, 53, 54));
    body.add_instruction(field_set(8, "y", 52, "IntegerBox"));
    body.add_instruction(binop(56, 57, 58));
    body.add_instruction(binop(35, 55, 56));
    body.add_instruction(binop(21, 65, 66));
    body.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(20),
        edge_args: None,
    });
    function.add_block(body);

    let mut exit = BasicBlock::new(BasicBlockId::new(23));
    exit.add_instruction(field_get(70, 8, "x", "IntegerBox"));
    exit.add_instruction(field_get(75, 8, "y", "IntegerBox"));
    exit.add_instruction(binop(77, 78, 79));
    exit.add_instruction(binop(81, 77, 80));
    exit.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(81)),
    });
    function.add_block(exit);
}

fn add_flag_toggle_micro_body(function: &mut MirFunction) {
    let entry = function.get_block_mut(BasicBlockId::new(0)).unwrap();
    entry.add_instruction(newbox(9, "Flag"));
    entry.add_instruction(const_i(14, 1));
    entry.add_instruction(field_set(9, "enabled", 14, "BoolBox"));
    entry.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(20),
        edge_args: None,
    });

    let mut header = BasicBlock::new(BasicBlockId::new(20));
    header.add_instruction(const_i(34, 2_000_000));
    header.add_instruction(compare(30, CompareOp::Lt, 32, 33));
    header.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(30),
        then_bb: BasicBlockId::new(21),
        else_bb: BasicBlockId::new(23),
        then_edge_args: None,
        else_edge_args: None,
    });
    function.add_block(header);

    let mut body = BasicBlock::new(BasicBlockId::new(21));
    body.add_instruction(field_get(25, 9, "enabled", "BoolBox"));
    body.add_instruction(compare(40, CompareOp::Eq, 41, 42));
    body.add_instruction(binop(28, 39, 40));
    body.add_instruction(const_i(49, 1_000_000));
    body.add_instruction(compare(46, CompareOp::Lt, 47, 48));
    body.add_instruction(field_set(9, "enabled", 46, "BoolBox"));
    body.add_instruction(binop(20, 56, 57));
    body.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(20),
        edge_args: None,
    });
    function.add_block(body);

    let mut exit = BasicBlock::new(BasicBlockId::new(23));
    exit.add_instruction(field_get(61, 9, "enabled", "BoolBox"));
    exit.add_instruction(compare(68, CompareOp::Eq, 69, 70));
    exit.add_instruction(binop(71, 67, 68));
    exit.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(71)),
    });
    function.add_block(exit);
}

fn newbox(dst: u32, box_type: &str) -> MirInstruction {
    MirInstruction::NewBox {
        dst: ValueId::new(dst),
        box_type: box_type.to_string(),
        args: vec![],
    }
}

fn const_i(dst: u32, value: i64) -> MirInstruction {
    MirInstruction::Const {
        dst: ValueId::new(dst),
        value: ConstValue::Integer(value),
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

fn compare(dst: u32, op: CompareOp, lhs: u32, rhs: u32) -> MirInstruction {
    MirInstruction::Compare {
        dst: ValueId::new(dst),
        op,
        lhs: ValueId::new(lhs),
        rhs: ValueId::new(rhs),
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

fn push_point_selections(function: &mut MirFunction) {
    function.metadata.thin_entry_selections = vec![
        selection(
            0,
            2,
            None,
            ThinEntrySurface::UserBoxFieldSet,
            "Point.x",
            ThinEntryValueClass::InlineI64,
        ),
        selection(
            0,
            4,
            None,
            ThinEntrySurface::UserBoxFieldSet,
            "Point.y",
            ThinEntryValueClass::InlineI64,
        ),
        selection(
            21,
            0,
            Some(26),
            ThinEntrySurface::UserBoxFieldGet,
            "Point.x",
            ThinEntryValueClass::InlineI64,
        ),
        selection(
            21,
            1,
            Some(27),
            ThinEntrySurface::UserBoxFieldGet,
            "Point.y",
            ThinEntryValueClass::InlineI64,
        ),
        selection(
            21,
            2,
            Some(29),
            ThinEntrySurface::UserBoxFieldGet,
            "Point.x",
            ThinEntryValueClass::InlineI64,
        ),
        selection(
            21,
            4,
            None,
            ThinEntrySurface::UserBoxFieldSet,
            "Point.x",
            ThinEntryValueClass::InlineI64,
        ),
        selection(
            21,
            5,
            Some(32),
            ThinEntrySurface::UserBoxFieldGet,
            "Point.y",
            ThinEntryValueClass::InlineI64,
        ),
        selection(
            21,
            7,
            None,
            ThinEntrySurface::UserBoxFieldSet,
            "Point.y",
            ThinEntryValueClass::InlineI64,
        ),
        selection(
            23,
            0,
            Some(70),
            ThinEntrySurface::UserBoxFieldGet,
            "Point.x",
            ThinEntryValueClass::InlineI64,
        ),
        selection(
            23,
            1,
            Some(75),
            ThinEntrySurface::UserBoxFieldGet,
            "Point.y",
            ThinEntryValueClass::InlineI64,
        ),
    ];
}

fn push_flag_selections(function: &mut MirFunction) {
    function.metadata.thin_entry_selections = vec![
        selection(
            0,
            2,
            None,
            ThinEntrySurface::UserBoxFieldSet,
            "Flag.enabled",
            ThinEntryValueClass::InlineBool,
        ),
        selection(
            21,
            0,
            Some(25),
            ThinEntrySurface::UserBoxFieldGet,
            "Flag.enabled",
            ThinEntryValueClass::InlineBool,
        ),
        selection(
            21,
            5,
            None,
            ThinEntrySurface::UserBoxFieldSet,
            "Flag.enabled",
            ThinEntryValueClass::InlineBool,
        ),
        selection(
            23,
            0,
            Some(61),
            ThinEntrySurface::UserBoxFieldGet,
            "Flag.enabled",
            ThinEntryValueClass::InlineBool,
        ),
    ];
}

fn selection(
    block: u32,
    instruction_index: usize,
    value: Option<u32>,
    surface: ThinEntrySurface,
    subject: &str,
    value_class: ThinEntryValueClass,
) -> ThinEntrySelection {
    let manifest_row = match surface {
        ThinEntrySurface::UserBoxFieldGet => "user_box_field_get.inline_scalar",
        ThinEntrySurface::UserBoxFieldSet => "user_box_field_set.inline_scalar",
        _ => unreachable!("test only uses user-box field surfaces"),
    };
    ThinEntrySelection {
        block: BasicBlockId::new(block),
        instruction_index,
        value: value.map(ValueId::new),
        surface,
        subject: subject.to_string(),
        manifest_row,
        selected_entry: ThinEntryPreferredEntry::ThinInternalEntry,
        state: ThinEntrySelectionState::AlreadySatisfied,
        current_carrier: ThinEntryCurrentCarrier::BackendTyped,
        value_class,
        demand: ThinEntryDemand::InlineScalar,
        reason: "test selection".to_string(),
    }
}

#[test]
fn userbox_loop_micro_seed_detects_point_add() {
    let mut function = make_function();
    add_point_add_micro_body(&mut function);
    push_point_selections(&mut function);

    refresh_function_userbox_loop_micro_seed_route(&mut function);

    let route = function
        .metadata
        .userbox_loop_micro_seed_route
        .expect("userbox loop micro route");
    assert_eq!(route.kind, UserBoxLoopMicroSeedKind::PointAddMicro);
    assert_eq!(route.box_name, "Point");
    assert_eq!(route.block_count, 4);
    assert_eq!(route.ops, 2_000_000);
    assert_eq!(route.flip_at, None);
    assert_eq!(route.field_get_count, 6);
    assert_eq!(route.field_set_count, 4);
    assert_eq!(route.compare_lt_count, 1);
    assert_eq!(route.compare_eq_count, 0);
    assert_eq!(route.binop_count, 7);
}

#[test]
fn userbox_loop_micro_seed_detects_flag_toggle() {
    let mut function = make_function();
    add_flag_toggle_micro_body(&mut function);
    push_flag_selections(&mut function);

    refresh_function_userbox_loop_micro_seed_route(&mut function);

    let route = function
        .metadata
        .userbox_loop_micro_seed_route
        .expect("userbox loop micro route");
    assert_eq!(route.kind, UserBoxLoopMicroSeedKind::FlagToggleMicro);
    assert_eq!(route.box_name, "Flag");
    assert_eq!(route.block_count, 4);
    assert_eq!(route.ops, 2_000_000);
    assert_eq!(route.flip_at, Some(1_000_000));
    assert_eq!(route.field_get_count, 2);
    assert_eq!(route.field_set_count, 2);
    assert_eq!(route.compare_lt_count, 2);
    assert_eq!(route.compare_eq_count, 2);
    assert_eq!(route.binop_count, 3);
}

#[test]
fn userbox_loop_micro_seed_stays_absent_without_thin_selections() {
    let mut function = make_function();
    add_point_add_micro_body(&mut function);

    refresh_function_userbox_loop_micro_seed_route(&mut function);

    assert!(function.metadata.userbox_loop_micro_seed_route.is_none());
}
