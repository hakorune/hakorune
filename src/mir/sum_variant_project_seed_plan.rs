/*!
 * MIR-owned route plan for temporary Sum variant_project exact seed bridges.
 *
 * The route is a backend-consumable view over existing Sum placement metadata.
 * It carries the literal payload required by the current exact helper so `.inc`
 * can emit without rediscovering legality from raw MIR JSON blocks.
 */

use super::{
    sum_placement::SumPlacementState,
    sum_placement_layout::{SumLocalAggregateLayout, SumPlacementLayout},
    sum_placement_selection::{SumPlacementPath, SumPlacementSelection},
    thin_entry::{ThinEntryPreferredEntry, ThinEntrySurface},
    thin_entry_selection::ThinEntrySelection,
    BasicBlock, BasicBlockId, ConstValue, MirFunction, MirInstruction, MirModule, MirType, ValueId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SumVariantProjectSeedKind {
    LocalI64,
    LocalF64,
    LocalHandle,
    CopyLocalI64,
    CopyLocalF64,
    CopyLocalHandle,
}

impl std::fmt::Display for SumVariantProjectSeedKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LocalI64 => f.write_str("variant_project_local_i64"),
            Self::LocalF64 => f.write_str("variant_project_local_f64"),
            Self::LocalHandle => f.write_str("variant_project_local_handle"),
            Self::CopyLocalI64 => f.write_str("variant_project_copy_local_i64"),
            Self::CopyLocalF64 => f.write_str("variant_project_copy_local_f64"),
            Self::CopyLocalHandle => f.write_str("variant_project_copy_local_handle"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SumVariantProjectSeedPayload {
    I64(i64),
    F64(f64),
    String(String),
}

impl SumVariantProjectSeedPayload {
    pub fn kind(&self) -> &'static str {
        match self {
            Self::I64(_) => "i64",
            Self::F64(_) => "f64",
            Self::String(_) => "string",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SumVariantProjectSeedProof {
    LocalAggregateProjectSeed,
}

impl SumVariantProjectSeedProof {
    fn as_str(self) -> &'static str {
        match self {
            Self::LocalAggregateProjectSeed => "sum_variant_project_local_aggregate_seed",
        }
    }
}

impl std::fmt::Display for SumVariantProjectSeedProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SumVariantProjectSeedRoute {
    kind: SumVariantProjectSeedKind,
    enum_name: String,
    variant: String,
    subject: String,
    layout: SumLocalAggregateLayout,
    variant_tag: u32,
    make_block: BasicBlockId,
    make_instruction_index: usize,
    project_block: BasicBlockId,
    project_instruction_index: usize,
    sum_value: ValueId,
    project_value: ValueId,
    project_source_value: ValueId,
    copy_value: Option<ValueId>,
    payload_value: ValueId,
    payload: SumVariantProjectSeedPayload,
    proof: SumVariantProjectSeedProof,
}

impl SumVariantProjectSeedRoute {
    pub fn kind(&self) -> SumVariantProjectSeedKind {
        self.kind
    }

    pub fn enum_name(&self) -> &str {
        self.enum_name.as_str()
    }

    pub fn variant(&self) -> &str {
        self.variant.as_str()
    }

    pub fn subject(&self) -> &str {
        self.subject.as_str()
    }

    pub fn layout(&self) -> SumLocalAggregateLayout {
        self.layout
    }

    pub fn variant_tag(&self) -> u32 {
        self.variant_tag
    }

    pub fn make_block(&self) -> BasicBlockId {
        self.make_block
    }

    pub fn make_instruction_index(&self) -> usize {
        self.make_instruction_index
    }

    pub fn project_block(&self) -> BasicBlockId {
        self.project_block
    }

    pub fn project_instruction_index(&self) -> usize {
        self.project_instruction_index
    }

    pub fn sum_value(&self) -> ValueId {
        self.sum_value
    }

    pub fn project_value(&self) -> ValueId {
        self.project_value
    }

    pub fn project_source_value(&self) -> ValueId {
        self.project_source_value
    }

    pub fn copy_value(&self) -> Option<ValueId> {
        self.copy_value
    }

    pub fn payload_value(&self) -> ValueId {
        self.payload_value
    }

    pub fn payload(&self) -> &SumVariantProjectSeedPayload {
        &self.payload
    }

    pub fn proof(&self) -> &'static str {
        self.proof.as_str()
    }
}

#[cfg(test)]
pub(crate) mod test_support {
    use super::*;

    pub(crate) fn local_i64_result_int_ok() -> SumVariantProjectSeedRoute {
        SumVariantProjectSeedRoute {
            kind: SumVariantProjectSeedKind::LocalI64,
            enum_name: "ResultInt".to_string(),
            variant: "Ok".to_string(),
            subject: "ResultInt::Ok".to_string(),
            layout: SumLocalAggregateLayout::TagI64Payload,
            variant_tag: 0,
            make_block: BasicBlockId::new(0),
            make_instruction_index: 1,
            project_block: BasicBlockId::new(0),
            project_instruction_index: 2,
            sum_value: ValueId::new(2),
            project_value: ValueId::new(3),
            project_source_value: ValueId::new(2),
            copy_value: None,
            payload_value: ValueId::new(1),
            payload: SumVariantProjectSeedPayload::I64(73),
            proof: SumVariantProjectSeedProof::LocalAggregateProjectSeed,
        }
    }

    pub(crate) fn copy_handle_result_handle_ok() -> SumVariantProjectSeedRoute {
        SumVariantProjectSeedRoute {
            kind: SumVariantProjectSeedKind::CopyLocalHandle,
            enum_name: "ResultHandle".to_string(),
            variant: "Ok".to_string(),
            subject: "ResultHandle::Ok".to_string(),
            layout: SumLocalAggregateLayout::TagHandlePayload,
            variant_tag: 0,
            make_block: BasicBlockId::new(0),
            make_instruction_index: 1,
            project_block: BasicBlockId::new(0),
            project_instruction_index: 3,
            sum_value: ValueId::new(2),
            project_value: ValueId::new(4),
            project_source_value: ValueId::new(3),
            copy_value: Some(ValueId::new(3)),
            payload_value: ValueId::new(1),
            payload: SumVariantProjectSeedPayload::String("hako".to_string()),
            proof: SumVariantProjectSeedProof::LocalAggregateProjectSeed,
        }
    }
}

pub fn refresh_module_sum_variant_project_seed_routes(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_sum_variant_project_seed_route(function);
    }
}

pub fn refresh_function_sum_variant_project_seed_route(function: &mut MirFunction) {
    function.metadata.sum_variant_project_seed_route =
        match_sum_variant_project_seed_route(function);
}

fn match_sum_variant_project_seed_route(
    function: &MirFunction,
) -> Option<SumVariantProjectSeedRoute> {
    let blocks = ordered_blocks(function);
    if blocks.len() != 1 {
        return None;
    }
    let block = blocks[0];
    let insts = instructions_with_terminator(block)?;

    if let Some(route) = match_direct_project(function, block.id, &insts) {
        return Some(route);
    }
    match_copy_project(function, block.id, &insts)
}

fn match_direct_project(
    function: &MirFunction,
    block_id: BasicBlockId,
    insts: &[&MirInstruction],
) -> Option<SumVariantProjectSeedRoute> {
    expect_ops(insts, &["const", "variant_make", "variant_project", "ret"])?;
    let payload_site = const_payload(insts[0])?;
    let make = make_info(insts[1])?;
    if make.payload_value != Some(payload_site.value) {
        return None;
    }
    let project = project_info(insts[2])?;
    let ret_value = return_value(insts[3])?;
    if ret_value != project.dst || project.source_value != make.dst {
        return None;
    }
    let (kind, layout) = direct_kind_and_layout(&payload_site.payload, make.payload_type)?;
    build_route(
        function,
        kind,
        &make,
        &project,
        layout,
        block_id,
        1,
        2,
        None,
        payload_site,
    )
}

fn match_copy_project(
    function: &MirFunction,
    block_id: BasicBlockId,
    insts: &[&MirInstruction],
) -> Option<SumVariantProjectSeedRoute> {
    expect_ops(
        insts,
        &["const", "variant_make", "copy", "variant_project", "ret"],
    )?;
    let payload_site = const_payload(insts[0])?;
    let make = make_info(insts[1])?;
    if make.payload_value != Some(payload_site.value) {
        return None;
    }
    let MirInstruction::Copy {
        dst: copy_value,
        src: copy_src,
    } = insts[2]
    else {
        return None;
    };
    if copy_src != &make.dst {
        return None;
    }
    let project = project_info(insts[3])?;
    let ret_value = return_value(insts[4])?;
    if ret_value != project.dst || project.source_value != *copy_value {
        return None;
    }
    let (kind, layout) = copy_kind_and_layout(&payload_site.payload, make.payload_type)?;
    build_route(
        function,
        kind,
        &make,
        &project,
        layout,
        block_id,
        1,
        3,
        Some(*copy_value),
        payload_site,
    )
}

fn build_route(
    function: &MirFunction,
    kind: SumVariantProjectSeedKind,
    make: &VariantMakeInfo<'_>,
    project: &VariantProjectInfo<'_>,
    layout: SumLocalAggregateLayout,
    block: BasicBlockId,
    make_instruction_index: usize,
    project_instruction_index: usize,
    copy_value: Option<ValueId>,
    payload_site: ConstPayload,
) -> Option<SumVariantProjectSeedRoute> {
    if make.enum_name != project.enum_name
        || make.variant != project.variant
        || make.tag != project.tag
        || make.payload_type != project.payload_type
    {
        return None;
    }
    let subject = format!("{}::{}", make.enum_name, make.variant);
    if !has_thin_selection(
        &function.metadata.thin_entry_selections,
        block,
        make_instruction_index,
        Some(make.dst),
        ThinEntrySurface::VariantMake,
        &subject,
        "variant_make.aggregate_local",
    ) || !has_thin_selection(
        &function.metadata.thin_entry_selections,
        block,
        project_instruction_index,
        Some(project.dst),
        ThinEntrySurface::VariantProject,
        &subject,
        "variant_project.payload_local",
    ) {
        return None;
    }
    if !has_sum_fact(
        &function.metadata.sum_placement_facts,
        block,
        make_instruction_index,
        Some(make.dst),
        ThinEntrySurface::VariantMake,
        &subject,
        None,
    ) {
        return None;
    }
    if !has_sum_selection(
        &function.metadata.sum_placement_selections,
        block,
        make_instruction_index,
        Some(make.dst),
        ThinEntrySurface::VariantMake,
        &subject,
        None,
        "variant_make.local_aggregate",
    ) || !has_sum_selection(
        &function.metadata.sum_placement_selections,
        block,
        project_instruction_index,
        Some(project.dst),
        ThinEntrySurface::VariantProject,
        &subject,
        Some(make.dst),
        "variant_project.local_aggregate",
    ) {
        return None;
    }
    if !has_sum_layout(
        &function.metadata.sum_placement_layouts,
        block,
        make_instruction_index,
        Some(make.dst),
        &subject,
        layout,
    ) {
        return None;
    }

    Some(SumVariantProjectSeedRoute {
        kind,
        enum_name: make.enum_name.to_string(),
        variant: make.variant.to_string(),
        subject,
        layout,
        variant_tag: make.tag,
        make_block: block,
        make_instruction_index,
        project_block: block,
        project_instruction_index,
        sum_value: make.dst,
        project_value: project.dst,
        project_source_value: project.source_value,
        copy_value,
        payload_value: payload_site.value,
        payload: payload_site.payload,
        proof: SumVariantProjectSeedProof::LocalAggregateProjectSeed,
    })
}

#[derive(Clone)]
struct ConstPayload {
    value: ValueId,
    payload: SumVariantProjectSeedPayload,
}

struct VariantMakeInfo<'a> {
    dst: ValueId,
    enum_name: &'a str,
    variant: &'a str,
    tag: u32,
    payload_value: Option<ValueId>,
    payload_type: Option<&'a MirType>,
}

struct VariantProjectInfo<'a> {
    dst: ValueId,
    source_value: ValueId,
    enum_name: &'a str,
    variant: &'a str,
    tag: u32,
    payload_type: Option<&'a MirType>,
}

fn const_payload(inst: &MirInstruction) -> Option<ConstPayload> {
    match inst {
        MirInstruction::Const {
            dst,
            value: ConstValue::Integer(value),
        } => Some(ConstPayload {
            value: *dst,
            payload: SumVariantProjectSeedPayload::I64(*value),
        }),
        MirInstruction::Const {
            dst,
            value: ConstValue::Float(value),
        } => Some(ConstPayload {
            value: *dst,
            payload: SumVariantProjectSeedPayload::F64(*value),
        }),
        MirInstruction::Const {
            dst,
            value: ConstValue::String(value),
        } => Some(ConstPayload {
            value: *dst,
            payload: SumVariantProjectSeedPayload::String(value.clone()),
        }),
        _ => None,
    }
}

fn make_info(inst: &MirInstruction) -> Option<VariantMakeInfo<'_>> {
    match inst {
        MirInstruction::VariantMake {
            dst,
            enum_name,
            variant,
            tag,
            payload,
            payload_type,
        } => Some(VariantMakeInfo {
            dst: *dst,
            enum_name,
            variant,
            tag: *tag,
            payload_value: *payload,
            payload_type: payload_type.as_ref(),
        }),
        _ => None,
    }
}

fn project_info(inst: &MirInstruction) -> Option<VariantProjectInfo<'_>> {
    match inst {
        MirInstruction::VariantProject {
            dst,
            value,
            enum_name,
            variant,
            tag,
            payload_type,
        } => Some(VariantProjectInfo {
            dst: *dst,
            source_value: *value,
            enum_name,
            variant,
            tag: *tag,
            payload_type: payload_type.as_ref(),
        }),
        _ => None,
    }
}

fn direct_kind_and_layout(
    payload: &SumVariantProjectSeedPayload,
    payload_type: Option<&MirType>,
) -> Option<(SumVariantProjectSeedKind, SumLocalAggregateLayout)> {
    match (payload, payload_type) {
        (SumVariantProjectSeedPayload::I64(_), Some(MirType::Integer)) => Some((
            SumVariantProjectSeedKind::LocalI64,
            SumLocalAggregateLayout::TagI64Payload,
        )),
        (SumVariantProjectSeedPayload::F64(_), Some(MirType::Float)) => Some((
            SumVariantProjectSeedKind::LocalF64,
            SumLocalAggregateLayout::TagF64Payload,
        )),
        (SumVariantProjectSeedPayload::String(_), Some(MirType::String)) => Some((
            SumVariantProjectSeedKind::LocalHandle,
            SumLocalAggregateLayout::TagHandlePayload,
        )),
        _ => None,
    }
}

fn copy_kind_and_layout(
    payload: &SumVariantProjectSeedPayload,
    payload_type: Option<&MirType>,
) -> Option<(SumVariantProjectSeedKind, SumLocalAggregateLayout)> {
    match (payload, payload_type) {
        (SumVariantProjectSeedPayload::I64(_), Some(MirType::Integer)) => Some((
            SumVariantProjectSeedKind::CopyLocalI64,
            SumLocalAggregateLayout::TagI64Payload,
        )),
        (SumVariantProjectSeedPayload::F64(_), Some(MirType::Float)) => Some((
            SumVariantProjectSeedKind::CopyLocalF64,
            SumLocalAggregateLayout::TagF64Payload,
        )),
        (SumVariantProjectSeedPayload::String(_), Some(MirType::String)) => Some((
            SumVariantProjectSeedKind::CopyLocalHandle,
            SumLocalAggregateLayout::TagHandlePayload,
        )),
        _ => None,
    }
}

fn return_value(inst: &MirInstruction) -> Option<ValueId> {
    match inst {
        MirInstruction::Return { value: Some(value) } => Some(*value),
        _ => None,
    }
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

fn has_sum_fact(
    facts: &[super::sum_placement::SumPlacementFact],
    block: BasicBlockId,
    instruction_index: usize,
    value: Option<ValueId>,
    surface: ThinEntrySurface,
    subject: &str,
    source_sum: Option<ValueId>,
) -> bool {
    facts.iter().any(|fact| {
        fact.block == block
            && fact.instruction_index == instruction_index
            && fact.value == value
            && fact.surface == surface
            && fact.subject == subject
            && fact.source_sum == source_sum
            && fact.state == SumPlacementState::LocalAggregateCandidate
    })
}

fn has_sum_selection(
    selections: &[SumPlacementSelection],
    block: BasicBlockId,
    instruction_index: usize,
    value: Option<ValueId>,
    surface: ThinEntrySurface,
    subject: &str,
    source_sum: Option<ValueId>,
    manifest_row: &str,
) -> bool {
    selections.iter().any(|selection| {
        selection.block == block
            && selection.instruction_index == instruction_index
            && selection.value == value
            && selection.surface == surface
            && selection.subject == subject
            && selection.source_sum == source_sum
            && selection.manifest_row == manifest_row
            && selection.selected_path == SumPlacementPath::LocalAggregate
    })
}

fn has_sum_layout(
    layouts: &[SumPlacementLayout],
    block: BasicBlockId,
    instruction_index: usize,
    value: Option<ValueId>,
    subject: &str,
    layout: SumLocalAggregateLayout,
) -> bool {
    layouts.iter().any(|candidate| {
        candidate.block == block
            && candidate.instruction_index == instruction_index
            && candidate.value == value
            && candidate.surface == ThinEntrySurface::VariantMake
            && candidate.subject == subject
            && candidate.source_sum.is_none()
            && candidate.layout == layout
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
        MirInstruction::VariantMake { .. } => "variant_make",
        MirInstruction::VariantProject { .. } => "variant_project",
        MirInstruction::Copy { .. } => "copy",
        MirInstruction::Return { .. } => "ret",
        _ => "other",
    }
}

#[cfg(test)]
mod tests;
