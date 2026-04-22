/*!
 * MIR-owned route plan for temporary UserBox local scalar exact seed bridges.
 *
 * Thin-entry metadata already proves the primitive field surface. This module
 * only recognizes the current Point local/copy exact seed shells and binds them
 * to one backend route so the C boundary can validate metadata and emit the
 * selected helper without rescanning raw MIR JSON.
 */

use super::{
    thin_entry::{ThinEntryPreferredEntry, ThinEntrySurface},
    thin_entry_selection::ThinEntrySelection,
    BasicBlock, BasicBlockId, BinaryOp, ConstValue, MirFunction, MirInstruction, MirModule,
    MirType, ValueId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserBoxLocalScalarSeedKind {
    PointLocalI64,
    PointCopyLocalI64,
}

impl std::fmt::Display for UserBoxLocalScalarSeedKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PointLocalI64 => f.write_str("point_local_i64"),
            Self::PointCopyLocalI64 => f.write_str("point_copy_local_i64"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserBoxLocalScalarSeedProof {
    PointFieldLocalScalarSeed,
}

impl std::fmt::Display for UserBoxLocalScalarSeedProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PointFieldLocalScalarSeed => f.write_str("userbox_point_field_local_scalar_seed"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserBoxLocalScalarSeedRoute {
    pub kind: UserBoxLocalScalarSeedKind,
    pub box_name: String,
    pub x_field: String,
    pub y_field: String,
    pub block: BasicBlockId,
    pub newbox_instruction_index: usize,
    pub set_x_instruction_index: usize,
    pub set_y_instruction_index: usize,
    pub get_x_instruction_index: usize,
    pub get_y_instruction_index: usize,
    pub point_value: ValueId,
    pub copy_value: Option<ValueId>,
    pub x_value: ValueId,
    pub y_value: ValueId,
    pub get_x_value: ValueId,
    pub get_y_value: ValueId,
    pub result_value: ValueId,
    pub x_i64: i64,
    pub y_i64: i64,
    pub proof: UserBoxLocalScalarSeedProof,
}

pub fn refresh_module_userbox_local_scalar_seed_routes(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_userbox_local_scalar_seed_route(function);
    }
}

pub fn refresh_function_userbox_local_scalar_seed_route(function: &mut MirFunction) {
    function.metadata.userbox_local_scalar_seed_route =
        match_userbox_local_scalar_seed_route(function);
}

fn match_userbox_local_scalar_seed_route(
    function: &MirFunction,
) -> Option<UserBoxLocalScalarSeedRoute> {
    let blocks = ordered_blocks(function);
    if blocks.len() != 1 {
        return None;
    }
    let block = blocks[0];
    let insts = instructions_with_terminator(block)?;

    if let Some(route) = match_point_local_i64(function, block.id, &insts) {
        return Some(route);
    }
    match_point_copy_local_i64(function, block.id, &insts)
}

fn match_point_local_i64(
    function: &MirFunction,
    block: BasicBlockId,
    insts: &[&MirInstruction],
) -> Option<UserBoxLocalScalarSeedRoute> {
    expect_ops(
        insts,
        &[
            "const",
            "const",
            "newbox",
            "field_set",
            "field_set",
            "field_get",
            "field_get",
            "binop",
            "ret",
        ],
    )?;
    let (x_value, x_i64) = const_i64(insts[0], ValueId::new(1), 41)?;
    let (y_value, y_i64) = const_i64(insts[1], ValueId::new(2), 2)?;
    let point_value = newbox_point(insts[2], ValueId::new(3))?;
    field_set_point_i64(insts[3], point_value, "x", x_value)?;
    field_set_point_i64(insts[4], point_value, "y", y_value)?;
    let get_x_value = field_get_point_i64(insts[5], point_value, "x", ValueId::new(4))?;
    let get_y_value = field_get_point_i64(insts[6], point_value, "y", ValueId::new(5))?;
    let result_value = add_result(insts[7], ValueId::new(6), get_x_value, get_y_value)?;
    return_value(insts[8], result_value)?;
    build_route(
        function,
        UserBoxLocalScalarSeedKind::PointLocalI64,
        block,
        point_value,
        None,
        x_value,
        y_value,
        get_x_value,
        get_y_value,
        result_value,
        x_i64,
        y_i64,
        2,
        3,
        4,
        5,
        6,
    )
}

fn match_point_copy_local_i64(
    function: &MirFunction,
    block: BasicBlockId,
    insts: &[&MirInstruction],
) -> Option<UserBoxLocalScalarSeedRoute> {
    expect_ops(
        insts,
        &[
            "const",
            "const",
            "newbox",
            "field_set",
            "field_set",
            "copy",
            "field_get",
            "field_get",
            "binop",
            "ret",
        ],
    )?;
    let (x_value, x_i64) = const_i64(insts[0], ValueId::new(1), 41)?;
    let (y_value, y_i64) = const_i64(insts[1], ValueId::new(2), 2)?;
    let point_value = newbox_point(insts[2], ValueId::new(3))?;
    field_set_point_i64(insts[3], point_value, "x", x_value)?;
    field_set_point_i64(insts[4], point_value, "y", y_value)?;
    let copy_value = copy_from(insts[5], ValueId::new(6), point_value)?;
    let get_x_value = field_get_point_i64(insts[6], copy_value, "x", ValueId::new(7))?;
    let get_y_value = field_get_point_i64(insts[7], copy_value, "y", ValueId::new(8))?;
    let result_value = add_result(insts[8], ValueId::new(9), get_x_value, get_y_value)?;
    return_value(insts[9], result_value)?;
    build_route(
        function,
        UserBoxLocalScalarSeedKind::PointCopyLocalI64,
        block,
        point_value,
        Some(copy_value),
        x_value,
        y_value,
        get_x_value,
        get_y_value,
        result_value,
        x_i64,
        y_i64,
        2,
        3,
        4,
        6,
        7,
    )
}

#[allow(clippy::too_many_arguments)]
fn build_route(
    function: &MirFunction,
    kind: UserBoxLocalScalarSeedKind,
    block: BasicBlockId,
    point_value: ValueId,
    copy_value: Option<ValueId>,
    x_value: ValueId,
    y_value: ValueId,
    get_x_value: ValueId,
    get_y_value: ValueId,
    result_value: ValueId,
    x_i64: i64,
    y_i64: i64,
    newbox_instruction_index: usize,
    set_x_instruction_index: usize,
    set_y_instruction_index: usize,
    get_x_instruction_index: usize,
    get_y_instruction_index: usize,
) -> Option<UserBoxLocalScalarSeedRoute> {
    if !has_thin_selection(
        &function.metadata.thin_entry_selections,
        block,
        set_x_instruction_index,
        None,
        ThinEntrySurface::UserBoxFieldSet,
        "Point.x",
        "user_box_field_set.inline_scalar",
    ) || !has_thin_selection(
        &function.metadata.thin_entry_selections,
        block,
        set_y_instruction_index,
        None,
        ThinEntrySurface::UserBoxFieldSet,
        "Point.y",
        "user_box_field_set.inline_scalar",
    ) || !has_thin_selection(
        &function.metadata.thin_entry_selections,
        block,
        get_x_instruction_index,
        Some(get_x_value),
        ThinEntrySurface::UserBoxFieldGet,
        "Point.x",
        "user_box_field_get.inline_scalar",
    ) || !has_thin_selection(
        &function.metadata.thin_entry_selections,
        block,
        get_y_instruction_index,
        Some(get_y_value),
        ThinEntrySurface::UserBoxFieldGet,
        "Point.y",
        "user_box_field_get.inline_scalar",
    ) {
        return None;
    }

    Some(UserBoxLocalScalarSeedRoute {
        kind,
        box_name: "Point".to_string(),
        x_field: "x".to_string(),
        y_field: "y".to_string(),
        block,
        newbox_instruction_index,
        set_x_instruction_index,
        set_y_instruction_index,
        get_x_instruction_index,
        get_y_instruction_index,
        point_value,
        copy_value,
        x_value,
        y_value,
        get_x_value,
        get_y_value,
        result_value,
        x_i64,
        y_i64,
        proof: UserBoxLocalScalarSeedProof::PointFieldLocalScalarSeed,
    })
}

fn const_i64(
    inst: &MirInstruction,
    expected_dst: ValueId,
    expected_value: i64,
) -> Option<(ValueId, i64)> {
    let MirInstruction::Const {
        dst,
        value: ConstValue::Integer(value),
    } = inst
    else {
        return None;
    };
    (*dst == expected_dst && *value == expected_value).then_some((*dst, *value))
}

fn newbox_point(inst: &MirInstruction, expected_dst: ValueId) -> Option<ValueId> {
    let MirInstruction::NewBox {
        dst,
        box_type,
        args,
    } = inst
    else {
        return None;
    };
    (*dst == expected_dst && box_type == "Point" && args.is_empty()).then_some(*dst)
}

fn field_set_point_i64(
    inst: &MirInstruction,
    expected_base: ValueId,
    expected_field: &str,
    expected_value: ValueId,
) -> Option<()> {
    let MirInstruction::FieldSet {
        base,
        field,
        value,
        declared_type,
    } = inst
    else {
        return None;
    };
    (*base == expected_base
        && field == expected_field
        && *value == expected_value
        && declared_integer_box(declared_type.as_ref()))
    .then_some(())
}

fn field_get_point_i64(
    inst: &MirInstruction,
    expected_base: ValueId,
    expected_field: &str,
    expected_dst: ValueId,
) -> Option<ValueId> {
    let MirInstruction::FieldGet {
        dst,
        base,
        field,
        declared_type,
    } = inst
    else {
        return None;
    };
    (*dst == expected_dst
        && *base == expected_base
        && field == expected_field
        && declared_integer_box(declared_type.as_ref()))
    .then_some(*dst)
}

fn copy_from(
    inst: &MirInstruction,
    expected_dst: ValueId,
    expected_src: ValueId,
) -> Option<ValueId> {
    let MirInstruction::Copy { dst, src } = inst else {
        return None;
    };
    (*dst == expected_dst && *src == expected_src).then_some(*dst)
}

fn add_result(
    inst: &MirInstruction,
    expected_dst: ValueId,
    expected_lhs: ValueId,
    expected_rhs: ValueId,
) -> Option<ValueId> {
    let MirInstruction::BinOp { dst, op, lhs, rhs } = inst else {
        return None;
    };
    (*dst == expected_dst && *op == BinaryOp::Add && *lhs == expected_lhs && *rhs == expected_rhs)
        .then_some(*dst)
}

fn return_value(inst: &MirInstruction, expected_value: ValueId) -> Option<()> {
    let MirInstruction::Return { value } = inst else {
        return None;
    };
    (*value == Some(expected_value)).then_some(())
}

fn declared_integer_box(ty: Option<&MirType>) -> bool {
    matches!(ty, Some(MirType::Box(box_name)) if box_name == "IntegerBox")
}

fn has_thin_selection(
    selections: &[ThinEntrySelection],
    block: BasicBlockId,
    instruction_index: usize,
    value: Option<ValueId>,
    surface: ThinEntrySurface,
    subject: &str,
    manifest_row: &str,
) -> bool {
    selections.iter().any(|selection| {
        selection.block == block
            && selection.instruction_index == instruction_index
            && selection.value == value
            && selection.surface == surface
            && selection.subject == subject
            && selection.manifest_row == manifest_row
            && selection.selected_entry == ThinEntryPreferredEntry::ThinInternalEntry
    })
}

fn ordered_blocks(function: &MirFunction) -> Vec<&BasicBlock> {
    let mut ids: Vec<BasicBlockId> = function.blocks.keys().copied().collect();
    ids.sort();
    ids.into_iter()
        .filter_map(|id| function.blocks.get(&id))
        .collect()
}

fn instructions_with_terminator(block: &BasicBlock) -> Option<Vec<&MirInstruction>> {
    let mut insts: Vec<&MirInstruction> = block.instructions.iter().collect();
    insts.push(block.terminator.as_ref()?);
    Some(insts)
}

fn expect_ops(insts: &[&MirInstruction], expected: &[&str]) -> Option<()> {
    if insts.len() != expected.len() {
        return None;
    }
    for (inst, expected) in insts.iter().zip(expected.iter().copied()) {
        if op_name(inst) != expected {
            return None;
        }
    }
    Some(())
}

fn op_name(inst: &MirInstruction) -> &'static str {
    match inst {
        MirInstruction::Const { .. } => "const",
        MirInstruction::NewBox { .. } => "newbox",
        MirInstruction::FieldSet { .. } => "field_set",
        MirInstruction::FieldGet { .. } => "field_get",
        MirInstruction::Copy { .. } => "copy",
        MirInstruction::BinOp { .. } => "binop",
        MirInstruction::Return { .. } => "ret",
        _ => "other",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{
        thin_entry::{ThinEntryCurrentCarrier, ThinEntryDemand, ThinEntryValueClass},
        thin_entry_selection::ThinEntrySelectionState,
        EffectMask, FunctionSignature,
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
        block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(3),
            box_type: "Point".to_string(),
            args: vec![],
        });
        block.add_instruction(field_set(3, "x", 1));
        block.add_instruction(field_set(3, "y", 2));
        block.add_instruction(field_get(4, 3, "x"));
        block.add_instruction(field_get(5, 3, "y"));
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
        block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(3),
            box_type: "Point".to_string(),
            args: vec![],
        });
        block.add_instruction(field_set(3, "x", 1));
        block.add_instruction(field_set(3, "y", 2));
        block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(6),
            src: ValueId::new(3),
        });
        block.add_instruction(field_get(7, 6, "x"));
        block.add_instruction(field_get(8, 6, "y"));
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

    fn field_set(base: u32, field: &str, value: u32) -> MirInstruction {
        MirInstruction::FieldSet {
            base: ValueId::new(base),
            field: field.to_string(),
            value: ValueId::new(value),
            declared_type: Some(MirType::Box("IntegerBox".to_string())),
        }
    }

    fn field_get(dst: u32, base: u32, field: &str) -> MirInstruction {
        MirInstruction::FieldGet {
            dst: ValueId::new(dst),
            base: ValueId::new(base),
            field: field.to_string(),
            declared_type: Some(MirType::Box("IntegerBox".to_string())),
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
            ),
            selection(
                4,
                None,
                ThinEntrySurface::UserBoxFieldSet,
                "Point.y",
                "user_box_field_set.inline_scalar",
            ),
            selection(
                get_x_idx,
                Some(get_x_value),
                ThinEntrySurface::UserBoxFieldGet,
                "Point.x",
                "user_box_field_get.inline_scalar",
            ),
            selection(
                get_y_idx,
                Some(get_y_value),
                ThinEntrySurface::UserBoxFieldGet,
                "Point.y",
                "user_box_field_get.inline_scalar",
            ),
        ];
    }

    fn selection(
        instruction_index: usize,
        value: Option<ValueId>,
        surface: ThinEntrySurface,
        subject: &str,
        manifest_row: &'static str,
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
            value_class: ThinEntryValueClass::InlineI64,
            demand: ThinEntryDemand::InlineScalar,
            reason: "test selection".to_string(),
        }
    }

    #[test]
    fn userbox_local_scalar_seed_detects_point_local_i64() {
        let mut function = make_function();
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
        assert_eq!(route.x_i64, 41);
        assert_eq!(route.y_i64, 2);
    }

    #[test]
    fn userbox_local_scalar_seed_detects_point_copy_local_i64() {
        let mut function = make_function();
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
    fn userbox_local_scalar_seed_stays_absent_without_thin_selections() {
        let mut function = make_function();
        add_point_local_body(&mut function);

        refresh_function_userbox_local_scalar_seed_route(&mut function);

        assert!(function.metadata.userbox_local_scalar_seed_route.is_none());
    }
}
