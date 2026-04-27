/*!
 * MIR-owned route plan for temporary UserBox local scalar exact seed bridges.
 *
 * Thin-entry metadata already proves the primitive field surface. This module
 * recognizes the current local/copy exact seed shells and binds them to one
 * backend route so the C boundary can validate metadata and emit the selected
 * helper without rescanning raw MIR JSON.
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
    FlagLocalBool,
    FlagCopyLocalBool,
    PointFLocalF64,
    PointFCopyLocalF64,
}

impl std::fmt::Display for UserBoxLocalScalarSeedKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PointLocalI64 => f.write_str("point_local_i64"),
            Self::PointCopyLocalI64 => f.write_str("point_copy_local_i64"),
            Self::FlagLocalBool => f.write_str("flag_local_bool"),
            Self::FlagCopyLocalBool => f.write_str("flag_copy_local_bool"),
            Self::PointFLocalF64 => f.write_str("pointf_local_f64"),
            Self::PointFCopyLocalF64 => f.write_str("pointf_copy_local_f64"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UserBoxLocalScalarSeedProof {
    PointFieldLocalScalarSeed,
    FlagFieldLocalScalarSeed,
    PointFFieldLocalScalarSeed,
}

impl UserBoxLocalScalarSeedProof {
    fn as_str(self) -> &'static str {
        match self {
            Self::PointFieldLocalScalarSeed => "userbox_point_field_local_scalar_seed",
            Self::FlagFieldLocalScalarSeed => "userbox_flag_field_local_scalar_seed",
            Self::PointFFieldLocalScalarSeed => "userbox_pointf_field_local_scalar_seed",
        }
    }
}

impl std::fmt::Display for UserBoxLocalScalarSeedProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserBoxLocalScalarSeedPayload {
    PointI64Pair {
        x_field: String,
        y_field: String,
        set_x_instruction_index: usize,
        set_y_instruction_index: usize,
        get_x_instruction_index: usize,
        get_y_instruction_index: usize,
        x_value: ValueId,
        y_value: ValueId,
        get_x_value: ValueId,
        get_y_value: ValueId,
        x_i64: i64,
        y_i64: i64,
    },
    SingleField {
        field: String,
        set_instruction_index: usize,
        get_instruction_index: usize,
        field_value: ValueId,
        get_field_value: ValueId,
        payload: UserBoxLocalScalarSeedSinglePayload,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserBoxLocalScalarSeedSinglePayload {
    I64(i64),
    F64Bits(u64),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserBoxLocalScalarSeedRoute {
    kind: UserBoxLocalScalarSeedKind,
    box_name: String,
    block: BasicBlockId,
    newbox_instruction_index: usize,
    box_value: ValueId,
    copy_value: Option<ValueId>,
    result_value: ValueId,
    proof: UserBoxLocalScalarSeedProof,
    payload: UserBoxLocalScalarSeedPayload,
}

impl UserBoxLocalScalarSeedRoute {
    pub fn kind(&self) -> UserBoxLocalScalarSeedKind {
        self.kind
    }

    pub fn box_name(&self) -> &str {
        &self.box_name
    }

    pub fn block(&self) -> BasicBlockId {
        self.block
    }

    pub fn newbox_instruction_index(&self) -> usize {
        self.newbox_instruction_index
    }

    pub fn box_value(&self) -> ValueId {
        self.box_value
    }

    pub fn copy_value(&self) -> Option<ValueId> {
        self.copy_value
    }

    pub fn result_value(&self) -> ValueId {
        self.result_value
    }

    pub fn proof(&self) -> &'static str {
        self.proof.as_str()
    }

    pub fn payload(&self) -> &UserBoxLocalScalarSeedPayload {
        &self.payload
    }
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
    if let Some(route) = match_point_copy_local_i64(function, block.id, &insts) {
        return Some(route);
    }
    if let Some(route) = match_flag_local_bool(function, block.id, &insts) {
        return Some(route);
    }
    if let Some(route) = match_flag_copy_local_bool(function, block.id, &insts) {
        return Some(route);
    }
    if let Some(route) = match_pointf_local_f64(function, block.id, &insts) {
        return Some(route);
    }
    match_pointf_copy_local_f64(function, block.id, &insts)
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
    let point_value = newbox_named(insts[2], ValueId::new(3), "Point")?;
    field_set_declared(insts[3], point_value, "x", x_value, "IntegerBox")?;
    field_set_declared(insts[4], point_value, "y", y_value, "IntegerBox")?;
    let get_x_value =
        field_get_declared(insts[5], point_value, "x", ValueId::new(4), "IntegerBox")?;
    let get_y_value =
        field_get_declared(insts[6], point_value, "y", ValueId::new(5), "IntegerBox")?;
    let result_value = add_result(insts[7], ValueId::new(6), get_x_value, get_y_value)?;
    return_value(insts[8], result_value)?;
    build_point_route(
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
    let point_value = newbox_named(insts[2], ValueId::new(3), "Point")?;
    field_set_declared(insts[3], point_value, "x", x_value, "IntegerBox")?;
    field_set_declared(insts[4], point_value, "y", y_value, "IntegerBox")?;
    let copy_value = copy_from(insts[5], ValueId::new(6), point_value)?;
    let get_x_value = field_get_declared(insts[6], copy_value, "x", ValueId::new(7), "IntegerBox")?;
    let get_y_value = field_get_declared(insts[7], copy_value, "y", ValueId::new(8), "IntegerBox")?;
    let result_value = add_result(insts[8], ValueId::new(9), get_x_value, get_y_value)?;
    return_value(insts[9], result_value)?;
    build_point_route(
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

fn match_flag_local_bool(
    function: &MirFunction,
    block: BasicBlockId,
    insts: &[&MirInstruction],
) -> Option<UserBoxLocalScalarSeedRoute> {
    expect_ops(insts, &["const", "newbox", "field_set", "field_get", "ret"])?;
    let (field_value, payload_i64) = const_i64(insts[0], ValueId::new(1), 1)?;
    let box_value = newbox_named(insts[1], ValueId::new(2), "Flag")?;
    field_set_declared(insts[2], box_value, "enabled", field_value, "BoolBox")?;
    let get_field_value =
        field_get_declared(insts[3], box_value, "enabled", ValueId::new(3), "BoolBox")?;
    return_value(insts[4], get_field_value)?;
    build_single_field_route(
        function,
        UserBoxLocalScalarSeedKind::FlagLocalBool,
        UserBoxLocalScalarSeedProof::FlagFieldLocalScalarSeed,
        block,
        "Flag",
        "enabled",
        "Flag.enabled",
        box_value,
        None,
        field_value,
        get_field_value,
        get_field_value,
        UserBoxLocalScalarSeedSinglePayload::I64(payload_i64),
        1,
        2,
        3,
    )
}

fn match_flag_copy_local_bool(
    function: &MirFunction,
    block: BasicBlockId,
    insts: &[&MirInstruction],
) -> Option<UserBoxLocalScalarSeedRoute> {
    expect_ops(
        insts,
        &["const", "newbox", "field_set", "copy", "field_get", "ret"],
    )?;
    let (field_value, payload_i64) = const_i64(insts[0], ValueId::new(1), 1)?;
    let box_value = newbox_named(insts[1], ValueId::new(2), "Flag")?;
    field_set_declared(insts[2], box_value, "enabled", field_value, "BoolBox")?;
    let copy_value = copy_from(insts[3], ValueId::new(3), box_value)?;
    let get_field_value =
        field_get_declared(insts[4], copy_value, "enabled", ValueId::new(4), "BoolBox")?;
    return_value(insts[5], get_field_value)?;
    build_single_field_route(
        function,
        UserBoxLocalScalarSeedKind::FlagCopyLocalBool,
        UserBoxLocalScalarSeedProof::FlagFieldLocalScalarSeed,
        block,
        "Flag",
        "enabled",
        "Flag.enabled",
        box_value,
        Some(copy_value),
        field_value,
        get_field_value,
        get_field_value,
        UserBoxLocalScalarSeedSinglePayload::I64(payload_i64),
        1,
        2,
        4,
    )
}

fn match_pointf_local_f64(
    function: &MirFunction,
    block: BasicBlockId,
    insts: &[&MirInstruction],
) -> Option<UserBoxLocalScalarSeedRoute> {
    expect_ops(insts, &["const", "newbox", "field_set", "field_get", "ret"])?;
    let (field_value, payload_bits) = const_f64_bits(insts[0], ValueId::new(1))?;
    let box_value = newbox_named(insts[1], ValueId::new(2), "PointF")?;
    field_set_declared(insts[2], box_value, "x", field_value, "FloatBox")?;
    let get_field_value =
        field_get_declared(insts[3], box_value, "x", ValueId::new(3), "FloatBox")?;
    return_value(insts[4], get_field_value)?;
    build_single_field_route(
        function,
        UserBoxLocalScalarSeedKind::PointFLocalF64,
        UserBoxLocalScalarSeedProof::PointFFieldLocalScalarSeed,
        block,
        "PointF",
        "x",
        "PointF.x",
        box_value,
        None,
        field_value,
        get_field_value,
        get_field_value,
        UserBoxLocalScalarSeedSinglePayload::F64Bits(payload_bits),
        1,
        2,
        3,
    )
}

fn match_pointf_copy_local_f64(
    function: &MirFunction,
    block: BasicBlockId,
    insts: &[&MirInstruction],
) -> Option<UserBoxLocalScalarSeedRoute> {
    expect_ops(
        insts,
        &["const", "newbox", "field_set", "copy", "field_get", "ret"],
    )?;
    let (field_value, payload_bits) = const_f64_bits(insts[0], ValueId::new(1))?;
    let box_value = newbox_named(insts[1], ValueId::new(2), "PointF")?;
    field_set_declared(insts[2], box_value, "x", field_value, "FloatBox")?;
    let copy_value = copy_from(insts[3], ValueId::new(3), box_value)?;
    let get_field_value =
        field_get_declared(insts[4], copy_value, "x", ValueId::new(4), "FloatBox")?;
    return_value(insts[5], get_field_value)?;
    build_single_field_route(
        function,
        UserBoxLocalScalarSeedKind::PointFCopyLocalF64,
        UserBoxLocalScalarSeedProof::PointFFieldLocalScalarSeed,
        block,
        "PointF",
        "x",
        "PointF.x",
        box_value,
        Some(copy_value),
        field_value,
        get_field_value,
        get_field_value,
        UserBoxLocalScalarSeedSinglePayload::F64Bits(payload_bits),
        1,
        2,
        4,
    )
}

#[allow(clippy::too_many_arguments)]
fn build_point_route(
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
        block,
        newbox_instruction_index,
        box_value: point_value,
        copy_value,
        result_value,
        proof: UserBoxLocalScalarSeedProof::PointFieldLocalScalarSeed,
        payload: UserBoxLocalScalarSeedPayload::PointI64Pair {
            x_field: "x".to_string(),
            y_field: "y".to_string(),
            set_x_instruction_index,
            set_y_instruction_index,
            get_x_instruction_index,
            get_y_instruction_index,
            x_value,
            y_value,
            get_x_value,
            get_y_value,
            x_i64,
            y_i64,
        },
    })
}

#[allow(clippy::too_many_arguments)]
fn build_single_field_route(
    function: &MirFunction,
    kind: UserBoxLocalScalarSeedKind,
    proof: UserBoxLocalScalarSeedProof,
    block: BasicBlockId,
    box_name: &str,
    field: &str,
    subject: &str,
    box_value: ValueId,
    copy_value: Option<ValueId>,
    field_value: ValueId,
    get_field_value: ValueId,
    result_value: ValueId,
    payload: UserBoxLocalScalarSeedSinglePayload,
    newbox_instruction_index: usize,
    set_instruction_index: usize,
    get_instruction_index: usize,
) -> Option<UserBoxLocalScalarSeedRoute> {
    if !has_thin_selection(
        &function.metadata.thin_entry_selections,
        block,
        set_instruction_index,
        None,
        ThinEntrySurface::UserBoxFieldSet,
        subject,
        "user_box_field_set.inline_scalar",
    ) || !has_thin_selection(
        &function.metadata.thin_entry_selections,
        block,
        get_instruction_index,
        Some(get_field_value),
        ThinEntrySurface::UserBoxFieldGet,
        subject,
        "user_box_field_get.inline_scalar",
    ) {
        return None;
    }

    Some(UserBoxLocalScalarSeedRoute {
        kind,
        box_name: box_name.to_string(),
        block,
        newbox_instruction_index,
        box_value,
        copy_value,
        result_value,
        proof,
        payload: UserBoxLocalScalarSeedPayload::SingleField {
            field: field.to_string(),
            set_instruction_index,
            get_instruction_index,
            field_value,
            get_field_value,
            payload,
        },
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

fn const_f64_bits(inst: &MirInstruction, expected_dst: ValueId) -> Option<(ValueId, u64)> {
    let MirInstruction::Const {
        dst,
        value: ConstValue::Float(value),
    } = inst
    else {
        return None;
    };
    (*dst == expected_dst).then_some((*dst, value.to_bits()))
}

fn newbox_named(
    inst: &MirInstruction,
    expected_dst: ValueId,
    expected_box: &str,
) -> Option<ValueId> {
    let MirInstruction::NewBox {
        dst,
        box_type,
        args,
    } = inst
    else {
        return None;
    };
    (*dst == expected_dst && box_type == expected_box && args.is_empty()).then_some(*dst)
}

fn field_set_declared(
    inst: &MirInstruction,
    expected_base: ValueId,
    expected_field: &str,
    expected_value: ValueId,
    expected_declared_box: &str,
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
        && declared_box(declared_type.as_ref(), expected_declared_box))
    .then_some(())
}

fn field_get_declared(
    inst: &MirInstruction,
    expected_base: ValueId,
    expected_field: &str,
    expected_dst: ValueId,
    expected_declared_box: &str,
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
        && declared_box(declared_type.as_ref(), expected_declared_box))
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

fn declared_box(ty: Option<&MirType>, expected_box: &str) -> bool {
    matches!(ty, Some(MirType::Box(box_name)) if box_name == expected_box)
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
pub(crate) mod test_support {
    use super::*;

    pub(crate) fn point_local_i64() -> UserBoxLocalScalarSeedRoute {
        UserBoxLocalScalarSeedRoute {
            kind: UserBoxLocalScalarSeedKind::PointLocalI64,
            box_name: "Point".to_string(),
            block: BasicBlockId::new(0),
            newbox_instruction_index: 2,
            box_value: ValueId::new(3),
            copy_value: None,
            result_value: ValueId::new(6),
            proof: UserBoxLocalScalarSeedProof::PointFieldLocalScalarSeed,
            payload: UserBoxLocalScalarSeedPayload::PointI64Pair {
                x_field: "x".to_string(),
                y_field: "y".to_string(),
                set_x_instruction_index: 3,
                set_y_instruction_index: 4,
                get_x_instruction_index: 5,
                get_y_instruction_index: 6,
                x_value: ValueId::new(1),
                y_value: ValueId::new(2),
                get_x_value: ValueId::new(4),
                get_y_value: ValueId::new(5),
                x_i64: 41,
                y_i64: 2,
            },
        }
    }

    pub(crate) fn point_copy_local_i64() -> UserBoxLocalScalarSeedRoute {
        UserBoxLocalScalarSeedRoute {
            kind: UserBoxLocalScalarSeedKind::PointCopyLocalI64,
            box_name: "Point".to_string(),
            block: BasicBlockId::new(0),
            newbox_instruction_index: 2,
            box_value: ValueId::new(3),
            copy_value: Some(ValueId::new(6)),
            result_value: ValueId::new(9),
            proof: UserBoxLocalScalarSeedProof::PointFieldLocalScalarSeed,
            payload: UserBoxLocalScalarSeedPayload::PointI64Pair {
                x_field: "x".to_string(),
                y_field: "y".to_string(),
                set_x_instruction_index: 3,
                set_y_instruction_index: 4,
                get_x_instruction_index: 6,
                get_y_instruction_index: 7,
                x_value: ValueId::new(1),
                y_value: ValueId::new(2),
                get_x_value: ValueId::new(7),
                get_y_value: ValueId::new(8),
                x_i64: 41,
                y_i64: 2,
            },
        }
    }

    pub(crate) fn flag_copy_local_bool() -> UserBoxLocalScalarSeedRoute {
        UserBoxLocalScalarSeedRoute {
            kind: UserBoxLocalScalarSeedKind::FlagCopyLocalBool,
            box_name: "Flag".to_string(),
            block: BasicBlockId::new(0),
            newbox_instruction_index: 1,
            box_value: ValueId::new(2),
            copy_value: Some(ValueId::new(3)),
            result_value: ValueId::new(4),
            proof: UserBoxLocalScalarSeedProof::FlagFieldLocalScalarSeed,
            payload: UserBoxLocalScalarSeedPayload::SingleField {
                field: "enabled".to_string(),
                set_instruction_index: 2,
                get_instruction_index: 4,
                field_value: ValueId::new(1),
                get_field_value: ValueId::new(4),
                payload: UserBoxLocalScalarSeedSinglePayload::I64(1),
            },
        }
    }

    pub(crate) fn pointf_copy_local_f64() -> UserBoxLocalScalarSeedRoute {
        UserBoxLocalScalarSeedRoute {
            kind: UserBoxLocalScalarSeedKind::PointFCopyLocalF64,
            box_name: "PointF".to_string(),
            block: BasicBlockId::new(0),
            newbox_instruction_index: 1,
            box_value: ValueId::new(2),
            copy_value: Some(ValueId::new(3)),
            result_value: ValueId::new(4),
            proof: UserBoxLocalScalarSeedProof::PointFFieldLocalScalarSeed,
            payload: UserBoxLocalScalarSeedPayload::SingleField {
                field: "x".to_string(),
                set_instruction_index: 2,
                get_instruction_index: 4,
                field_value: ValueId::new(1),
                get_field_value: ValueId::new(4),
                payload: UserBoxLocalScalarSeedSinglePayload::F64Bits(1.5f64.to_bits()),
            },
        }
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
}
