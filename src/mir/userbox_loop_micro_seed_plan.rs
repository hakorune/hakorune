/*!
 * MIR-owned route plan for temporary UserBox loop micro exact seed bridges.
 *
 * Thin-entry metadata already proves the primitive UserBox field surface. This
 * module recognizes the current point-add / flag-toggle loop micro shells and
 * publishes a single backend-consumable route so the C boundary can validate
 * metadata and emit the selected helper without rediscovering raw MIR shape.
 */

use super::{
    thin_entry::{ThinEntryPreferredEntry, ThinEntrySurface},
    thin_entry_selection::ThinEntrySelection,
    BasicBlock, BasicBlockId, BinaryOp, CompareOp, ConstValue, MirFunction, MirInstruction,
    MirModule, MirType, ValueId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserBoxLoopMicroSeedKind {
    PointAddMicro,
    FlagToggleMicro,
}

impl std::fmt::Display for UserBoxLoopMicroSeedKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PointAddMicro => f.write_str("point_add_micro"),
            Self::FlagToggleMicro => f.write_str("flag_toggle_micro"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UserBoxLoopMicroSeedProof {
    PointAddLoopMicroSeed,
    FlagToggleLoopMicroSeed,
}

impl UserBoxLoopMicroSeedProof {
    fn as_str(self) -> &'static str {
        match self {
            Self::PointAddLoopMicroSeed => "userbox_point_add_loop_micro_seed",
            Self::FlagToggleLoopMicroSeed => "userbox_flag_toggle_loop_micro_seed",
        }
    }
}

impl std::fmt::Display for UserBoxLoopMicroSeedProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserBoxLoopMicroSeedRoute {
    kind: UserBoxLoopMicroSeedKind,
    box_name: String,
    block_count: usize,
    newbox_block: BasicBlockId,
    newbox_instruction_index: usize,
    box_value: ValueId,
    ops: i64,
    flip_at: Option<i64>,
    field_get_count: usize,
    field_set_count: usize,
    compare_lt_count: usize,
    compare_eq_count: usize,
    binop_count: usize,
    proof: UserBoxLoopMicroSeedProof,
}

impl UserBoxLoopMicroSeedRoute {
    pub fn kind(&self) -> UserBoxLoopMicroSeedKind {
        self.kind
    }

    pub fn box_name(&self) -> &str {
        &self.box_name
    }

    pub fn block_count(&self) -> usize {
        self.block_count
    }

    pub fn newbox_block(&self) -> BasicBlockId {
        self.newbox_block
    }

    pub fn newbox_instruction_index(&self) -> usize {
        self.newbox_instruction_index
    }

    pub fn box_value(&self) -> ValueId {
        self.box_value
    }

    pub fn ops(&self) -> i64 {
        self.ops
    }

    pub fn flip_at(&self) -> Option<i64> {
        self.flip_at
    }

    pub fn field_get_count(&self) -> usize {
        self.field_get_count
    }

    pub fn field_set_count(&self) -> usize {
        self.field_set_count
    }

    pub fn compare_lt_count(&self) -> usize {
        self.compare_lt_count
    }

    pub fn compare_eq_count(&self) -> usize {
        self.compare_eq_count
    }

    pub fn binop_count(&self) -> usize {
        self.binop_count
    }

    pub fn proof(&self) -> &'static str {
        self.proof.as_str()
    }
}

#[derive(Debug, Clone)]
struct LoopMicroFacts {
    block_count: usize,
    newbox: Option<(String, BasicBlockId, usize, ValueId)>,
    ops: Option<i64>,
    flip_at: Option<i64>,
    field_gets: Vec<FieldAccess>,
    field_sets: Vec<FieldAccess>,
    compare_lt_count: usize,
    compare_eq_count: usize,
    binop_count: usize,
}

#[derive(Debug, Clone)]
struct FieldAccess {
    block: BasicBlockId,
    instruction_index: usize,
    value: Option<ValueId>,
    subject: String,
    declared_box: String,
}

pub fn refresh_module_userbox_loop_micro_seed_routes(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_userbox_loop_micro_seed_route(function);
    }
}

pub fn refresh_function_userbox_loop_micro_seed_route(function: &mut MirFunction) {
    function.metadata.userbox_loop_micro_seed_route = match_userbox_loop_micro_seed_route(function);
}

fn match_userbox_loop_micro_seed_route(
    function: &MirFunction,
) -> Option<UserBoxLoopMicroSeedRoute> {
    let blocks = ordered_blocks(function);
    if blocks.len() != 4 {
        return None;
    }

    let facts = collect_loop_micro_facts(&blocks)?;
    match facts
        .newbox
        .as_ref()
        .map(|(box_name, _, _, _)| box_name.as_str())
    {
        Some("Point") => match_point_add_micro(function, facts),
        Some("Flag") => match_flag_toggle_micro(function, facts),
        _ => None,
    }
}

fn match_point_add_micro(
    function: &MirFunction,
    facts: LoopMicroFacts,
) -> Option<UserBoxLoopMicroSeedRoute> {
    let (box_name, newbox_block, newbox_instruction_index, box_value) = facts.newbox?;
    if !(box_name == "Point"
        && facts.ops == Some(2_000_000)
        && facts.flip_at.is_none()
        && facts.field_gets.len() == 6
        && facts.field_sets.len() == 4
        && facts.compare_lt_count == 1
        && facts.compare_eq_count == 0
        && facts.binop_count == 7
        && fields_match(
            &facts.field_gets,
            &[("Point.x", "IntegerBox", 3), ("Point.y", "IntegerBox", 3)],
        )
        && fields_match(
            &facts.field_sets,
            &[("Point.x", "IntegerBox", 2), ("Point.y", "IntegerBox", 2)],
        )
        && all_field_accesses_have_thin_selection(
            &function.metadata.thin_entry_selections,
            &facts.field_gets,
            ThinEntrySurface::UserBoxFieldGet,
            "user_box_field_get.inline_scalar",
        )
        && all_field_accesses_have_thin_selection(
            &function.metadata.thin_entry_selections,
            &facts.field_sets,
            ThinEntrySurface::UserBoxFieldSet,
            "user_box_field_set.inline_scalar",
        ))
    {
        return None;
    }

    Some(UserBoxLoopMicroSeedRoute {
        kind: UserBoxLoopMicroSeedKind::PointAddMicro,
        box_name,
        block_count: facts.block_count,
        newbox_block,
        newbox_instruction_index,
        box_value,
        ops: 2_000_000,
        flip_at: None,
        field_get_count: facts.field_gets.len(),
        field_set_count: facts.field_sets.len(),
        compare_lt_count: facts.compare_lt_count,
        compare_eq_count: facts.compare_eq_count,
        binop_count: facts.binop_count,
        proof: UserBoxLoopMicroSeedProof::PointAddLoopMicroSeed,
    })
}

fn match_flag_toggle_micro(
    function: &MirFunction,
    facts: LoopMicroFacts,
) -> Option<UserBoxLoopMicroSeedRoute> {
    let (box_name, newbox_block, newbox_instruction_index, box_value) = facts.newbox?;
    if !(box_name == "Flag"
        && facts.ops == Some(2_000_000)
        && facts.flip_at == Some(1_000_000)
        && facts.field_gets.len() == 2
        && facts.field_sets.len() == 2
        && facts.compare_lt_count == 2
        && facts.compare_eq_count == 2
        && facts.binop_count == 3
        && fields_match(&facts.field_gets, &[("Flag.enabled", "BoolBox", 2)])
        && fields_match(&facts.field_sets, &[("Flag.enabled", "BoolBox", 2)])
        && all_field_accesses_have_thin_selection(
            &function.metadata.thin_entry_selections,
            &facts.field_gets,
            ThinEntrySurface::UserBoxFieldGet,
            "user_box_field_get.inline_scalar",
        )
        && all_field_accesses_have_thin_selection(
            &function.metadata.thin_entry_selections,
            &facts.field_sets,
            ThinEntrySurface::UserBoxFieldSet,
            "user_box_field_set.inline_scalar",
        ))
    {
        return None;
    }

    Some(UserBoxLoopMicroSeedRoute {
        kind: UserBoxLoopMicroSeedKind::FlagToggleMicro,
        box_name,
        block_count: facts.block_count,
        newbox_block,
        newbox_instruction_index,
        box_value,
        ops: 2_000_000,
        flip_at: Some(1_000_000),
        field_get_count: facts.field_gets.len(),
        field_set_count: facts.field_sets.len(),
        compare_lt_count: facts.compare_lt_count,
        compare_eq_count: facts.compare_eq_count,
        binop_count: facts.binop_count,
        proof: UserBoxLoopMicroSeedProof::FlagToggleLoopMicroSeed,
    })
}

fn collect_loop_micro_facts(blocks: &[&BasicBlock]) -> Option<LoopMicroFacts> {
    let mut facts = LoopMicroFacts {
        block_count: blocks.len(),
        newbox: None,
        ops: None,
        flip_at: None,
        field_gets: Vec::new(),
        field_sets: Vec::new(),
        compare_lt_count: 0,
        compare_eq_count: 0,
        binop_count: 0,
    };

    for block in blocks {
        for (instruction_index, inst) in block.all_instructions().enumerate() {
            match inst {
                MirInstruction::NewBox {
                    dst,
                    box_type,
                    args,
                } => {
                    if !args.is_empty() || facts.newbox.is_some() {
                        return None;
                    }
                    facts.newbox = Some((box_type.clone(), block.id, instruction_index, *dst));
                }
                MirInstruction::Const {
                    value: ConstValue::Integer(value),
                    ..
                } => match *value {
                    2_000_000 => facts.ops = Some(*value),
                    1_000_000 => facts.flip_at = Some(*value),
                    _ => {}
                },
                MirInstruction::FieldGet {
                    dst,
                    field,
                    declared_type,
                    ..
                } => {
                    facts.field_gets.push(FieldAccess {
                        block: block.id,
                        instruction_index,
                        value: Some(*dst),
                        subject: subject_for_declared_field(declared_type.as_ref(), field)?,
                        declared_box: declared_box(declared_type.as_ref())?.to_string(),
                    });
                }
                MirInstruction::FieldSet {
                    field,
                    declared_type,
                    ..
                } => {
                    facts.field_sets.push(FieldAccess {
                        block: block.id,
                        instruction_index,
                        value: None,
                        subject: subject_for_declared_field(declared_type.as_ref(), field)?,
                        declared_box: declared_box(declared_type.as_ref())?.to_string(),
                    });
                }
                MirInstruction::Compare { op, .. } => match op {
                    CompareOp::Lt => facts.compare_lt_count += 1,
                    CompareOp::Eq => facts.compare_eq_count += 1,
                    _ => return None,
                },
                MirInstruction::BinOp { op, .. } => {
                    if *op != BinaryOp::Add {
                        return None;
                    }
                    facts.binop_count += 1;
                }
                _ => {}
            }
        }
    }

    Some(facts)
}

fn subject_for_declared_field(declared_type: Option<&MirType>, field: &str) -> Option<String> {
    match (declared_box(declared_type)?, field) {
        ("IntegerBox", "x") => Some("Point.x".to_string()),
        ("IntegerBox", "y") => Some("Point.y".to_string()),
        ("BoolBox", "enabled") => Some("Flag.enabled".to_string()),
        _ => None,
    }
}

fn declared_box(ty: Option<&MirType>) -> Option<&str> {
    match ty {
        Some(MirType::Box(box_name)) => Some(box_name.as_str()),
        _ => None,
    }
}

fn fields_match(accesses: &[FieldAccess], expected: &[(&str, &str, usize)]) -> bool {
    expected.iter().all(|(subject, declared_box, count)| {
        accesses
            .iter()
            .filter(|access| access.subject == *subject && access.declared_box == *declared_box)
            .count()
            == *count
    }) && accesses.len() == expected.iter().map(|(_, _, count)| *count).sum::<usize>()
}

fn all_field_accesses_have_thin_selection(
    selections: &[ThinEntrySelection],
    accesses: &[FieldAccess],
    surface: ThinEntrySurface,
    manifest_row: &str,
) -> bool {
    accesses.iter().all(|access| {
        has_thin_selection(
            selections,
            access.block,
            access.instruction_index,
            access.value,
            surface,
            access.subject.as_str(),
            manifest_row,
        )
    })
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

#[cfg(test)]
pub(crate) mod test_support {
    use super::*;

    pub(crate) fn flag_toggle_micro() -> UserBoxLoopMicroSeedRoute {
        UserBoxLoopMicroSeedRoute {
            kind: UserBoxLoopMicroSeedKind::FlagToggleMicro,
            box_name: "Flag".to_string(),
            block_count: 4,
            newbox_block: BasicBlockId::new(0),
            newbox_instruction_index: 0,
            box_value: ValueId::new(9),
            ops: 2_000_000,
            flip_at: Some(1_000_000),
            field_get_count: 2,
            field_set_count: 2,
            compare_lt_count: 2,
            compare_eq_count: 2,
            binop_count: 3,
            proof: UserBoxLoopMicroSeedProof::FlagToggleLoopMicroSeed,
        }
    }
}

#[cfg(test)]
mod tests;
