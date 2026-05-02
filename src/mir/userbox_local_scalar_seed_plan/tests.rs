use super::*;
use crate::mir::{
    thin_entry::{ThinEntryCurrentCarrier, ThinEntryDemand, ThinEntryValueClass},
    thin_entry_selection::ThinEntrySelectionState,
    EffectMask, FunctionSignature,
};

fn make_function(return_type: MirType) -> MirFunction {
    MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    )
}

fn add_point_local_body(function: &mut MirFunction) {
    let block = function.get_block_mut(BasicBlockId::new(0)).unwrap();
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Integer(41),
    });
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::Integer(2),
    });
    block.add_instruction(newbox(3, "Point"));
    block.add_instruction(field_set(3, "x", 1, "IntegerBox"));
    block.add_instruction(field_set(3, "y", 2, "IntegerBox"));
    block.add_instruction(field_get(4, 3, "x", "IntegerBox"));
    block.add_instruction(field_get(5, 3, "y", "IntegerBox"));
    block.add_instruction(MirInstruction::BinOp {
        dst: ValueId::new(6),
        op: BinaryOp::Add,
        lhs: ValueId::new(4),
        rhs: ValueId::new(5),
    });
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(6)),
    });
}

fn add_point_copy_body(function: &mut MirFunction) {
    let block = function.get_block_mut(BasicBlockId::new(0)).unwrap();
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Integer(41),
    });
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::Integer(2),
    });
    block.add_instruction(newbox(3, "Point"));
    block.add_instruction(field_set(3, "x", 1, "IntegerBox"));
    block.add_instruction(field_set(3, "y", 2, "IntegerBox"));
    block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(6),
        src: ValueId::new(3),
    });
    block.add_instruction(field_get(7, 6, "x", "IntegerBox"));
    block.add_instruction(field_get(8, 6, "y", "IntegerBox"));
    block.add_instruction(MirInstruction::BinOp {
        dst: ValueId::new(9),
        op: BinaryOp::Add,
        lhs: ValueId::new(7),
        rhs: ValueId::new(8),
    });
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(9)),
    });
}

fn add_flag_local_body(function: &mut MirFunction) {
    let block = function.get_block_mut(BasicBlockId::new(0)).unwrap();
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Integer(1),
    });
    block.add_instruction(newbox(2, "Flag"));
    block.add_instruction(field_set(2, "enabled", 1, "BoolBox"));
    block.add_instruction(field_get(3, 2, "enabled", "BoolBox"));
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
}

fn add_flag_copy_body(function: &mut MirFunction) {
    let block = function.get_block_mut(BasicBlockId::new(0)).unwrap();
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Integer(1),
    });
    block.add_instruction(newbox(2, "Flag"));
    block.add_instruction(field_set(2, "enabled", 1, "BoolBox"));
    block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(3),
        src: ValueId::new(2),
    });
    block.add_instruction(field_get(4, 3, "enabled", "BoolBox"));
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });
}

fn add_pointf_local_body(function: &mut MirFunction) {
    let block = function.get_block_mut(BasicBlockId::new(0)).unwrap();
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Float(1.5),
    });
    block.add_instruction(newbox(2, "PointF"));
    block.add_instruction(field_set(2, "x", 1, "FloatBox"));
    block.add_instruction(field_get(3, 2, "x", "FloatBox"));
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
}

fn add_pointf_copy_body(function: &mut MirFunction) {
    let block = function.get_block_mut(BasicBlockId::new(0)).unwrap();
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Float(1.5),
    });
    block.add_instruction(newbox(2, "PointF"));
    block.add_instruction(field_set(2, "x", 1, "FloatBox"));
    block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(3),
        src: ValueId::new(2),
    });
    block.add_instruction(field_get(4, 3, "x", "FloatBox"));
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });
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

fn push_point_metadata(
    function: &mut MirFunction,
    get_x_idx: usize,
    get_y_idx: usize,
    get_x_value: ValueId,
    get_y_value: ValueId,
) {
    function.metadata.thin_entry_selections = vec![
        selection(
            3,
            None,
            ThinEntrySurface::UserBoxFieldSet,
            "Point.x",
            "user_box_field_set.inline_scalar",
            ThinEntryValueClass::InlineI64,
        ),
        selection(
            4,
            None,
            ThinEntrySurface::UserBoxFieldSet,
            "Point.y",
            "user_box_field_set.inline_scalar",
            ThinEntryValueClass::InlineI64,
        ),
        selection(
            get_x_idx,
            Some(get_x_value),
            ThinEntrySurface::UserBoxFieldGet,
            "Point.x",
            "user_box_field_get.inline_scalar",
            ThinEntryValueClass::InlineI64,
        ),
        selection(
            get_y_idx,
            Some(get_y_value),
            ThinEntrySurface::UserBoxFieldGet,
            "Point.y",
            "user_box_field_get.inline_scalar",
            ThinEntryValueClass::InlineI64,
        ),
    ];
}

fn push_single_field_metadata(
    function: &mut MirFunction,
    subject: &str,
    get_idx: usize,
    get_value: ValueId,
    value_class: ThinEntryValueClass,
) {
    function.metadata.thin_entry_selections = vec![
        selection(
            2,
            None,
            ThinEntrySurface::UserBoxFieldSet,
            subject,
            "user_box_field_set.inline_scalar",
            value_class,
        ),
        selection(
            get_idx,
            Some(get_value),
            ThinEntrySurface::UserBoxFieldGet,
            subject,
            "user_box_field_get.inline_scalar",
            value_class,
        ),
    ];
}

fn selection(
    instruction_index: usize,
    value: Option<ValueId>,
    surface: ThinEntrySurface,
    subject: &str,
    manifest_row: &'static str,
    value_class: ThinEntryValueClass,
) -> ThinEntrySelection {
    ThinEntrySelection {
        block: BasicBlockId::new(0),
        instruction_index,
        value,
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
fn userbox_local_scalar_seed_detects_point_local_i64() {
    let mut function = make_function(MirType::Integer);
    add_point_local_body(&mut function);
    push_point_metadata(&mut function, 5, 6, ValueId::new(4), ValueId::new(5));

    refresh_function_userbox_local_scalar_seed_route(&mut function);

    let route = function
        .metadata
        .userbox_local_scalar_seed_route
        .expect("userbox local scalar route");
    assert_eq!(route.kind, UserBoxLocalScalarSeedKind::PointLocalI64);
    assert_eq!(route.copy_value, None);
    assert_eq!(route.result_value, ValueId::new(6));
    match route.payload {
        UserBoxLocalScalarSeedPayload::PointI64Pair { x_i64, y_i64, .. } => {
            assert_eq!(x_i64, 41);
            assert_eq!(y_i64, 2);
        }
        UserBoxLocalScalarSeedPayload::SingleField { .. } => panic!("point payload expected"),
    }
}

#[test]
fn userbox_local_scalar_seed_detects_point_copy_local_i64() {
    let mut function = make_function(MirType::Integer);
    add_point_copy_body(&mut function);
    push_point_metadata(&mut function, 6, 7, ValueId::new(7), ValueId::new(8));

    refresh_function_userbox_local_scalar_seed_route(&mut function);

    let route = function
        .metadata
        .userbox_local_scalar_seed_route
        .expect("userbox local scalar route");
    assert_eq!(route.kind, UserBoxLocalScalarSeedKind::PointCopyLocalI64);
    assert_eq!(route.copy_value, Some(ValueId::new(6)));
    assert_eq!(route.result_value, ValueId::new(9));
}

#[test]
fn userbox_local_scalar_seed_detects_flag_local_bool() {
    let mut function = make_function(MirType::Bool);
    add_flag_local_body(&mut function);
    push_single_field_metadata(
        &mut function,
        "Flag.enabled",
        3,
        ValueId::new(3),
        ThinEntryValueClass::InlineBool,
    );

    refresh_function_userbox_local_scalar_seed_route(&mut function);

    let route = function
        .metadata
        .userbox_local_scalar_seed_route
        .expect("userbox local scalar route");
    assert_eq!(route.kind, UserBoxLocalScalarSeedKind::FlagLocalBool);
    assert_eq!(route.box_value, ValueId::new(2));
    assert_eq!(route.copy_value, None);
    assert_eq!(route.result_value, ValueId::new(3));
}

#[test]
fn userbox_local_scalar_seed_detects_flag_copy_local_bool() {
    let mut function = make_function(MirType::Bool);
    add_flag_copy_body(&mut function);
    push_single_field_metadata(
        &mut function,
        "Flag.enabled",
        4,
        ValueId::new(4),
        ThinEntryValueClass::InlineBool,
    );

    refresh_function_userbox_local_scalar_seed_route(&mut function);

    let route = function
        .metadata
        .userbox_local_scalar_seed_route
        .expect("userbox local scalar route");
    assert_eq!(route.kind, UserBoxLocalScalarSeedKind::FlagCopyLocalBool);
    assert_eq!(route.copy_value, Some(ValueId::new(3)));
    assert_eq!(route.result_value, ValueId::new(4));
}

#[test]
fn userbox_local_scalar_seed_detects_pointf_local_f64() {
    let mut function = make_function(MirType::Float);
    add_pointf_local_body(&mut function);
    push_single_field_metadata(
        &mut function,
        "PointF.x",
        3,
        ValueId::new(3),
        ThinEntryValueClass::InlineF64,
    );

    refresh_function_userbox_local_scalar_seed_route(&mut function);

    let route = function
        .metadata
        .userbox_local_scalar_seed_route
        .expect("userbox local scalar route");
    assert_eq!(route.kind, UserBoxLocalScalarSeedKind::PointFLocalF64);
    match route.payload {
        UserBoxLocalScalarSeedPayload::SingleField {
            payload: UserBoxLocalScalarSeedSinglePayload::F64Bits(bits),
            ..
        } => assert_eq!(f64::from_bits(bits), 1.5),
        _ => panic!("pointf single-field payload expected"),
    }
}

#[test]
fn userbox_local_scalar_seed_detects_pointf_copy_local_f64() {
    let mut function = make_function(MirType::Float);
    add_pointf_copy_body(&mut function);
    push_single_field_metadata(
        &mut function,
        "PointF.x",
        4,
        ValueId::new(4),
        ThinEntryValueClass::InlineF64,
    );

    refresh_function_userbox_local_scalar_seed_route(&mut function);

    let route = function
        .metadata
        .userbox_local_scalar_seed_route
        .expect("userbox local scalar route");
    assert_eq!(route.kind, UserBoxLocalScalarSeedKind::PointFCopyLocalF64);
    assert_eq!(route.copy_value, Some(ValueId::new(3)));
    assert_eq!(route.result_value, ValueId::new(4));
}

#[test]
fn userbox_local_scalar_seed_stays_absent_without_thin_selections() {
    let mut function = make_function(MirType::Integer);
    add_point_local_body(&mut function);

    refresh_function_userbox_local_scalar_seed_route(&mut function);

    assert!(function.metadata.userbox_local_scalar_seed_route.is_none());
}
