/*!
 * MIR-owned route plan for temporary Sum variant_tag exact seed bridges.
 *
 * Sum placement metadata already proves the local aggregate route. This module
 * only recognizes the current whole-function exact seed shells and binds them
 * to one backend route so the C boundary can validate metadata and emit the
 * selected helper without rescanning raw MIR JSON.
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
pub enum SumVariantTagSeedKind {
    LocalI64,
    LocalTagOnly,
    LocalF64,
    LocalHandle,
    CopyLocalI64,
}

impl std::fmt::Display for SumVariantTagSeedKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LocalI64 => f.write_str("variant_tag_local_i64"),
            Self::LocalTagOnly => f.write_str("variant_tag_local_tag_only"),
            Self::LocalF64 => f.write_str("variant_tag_local_f64"),
            Self::LocalHandle => f.write_str("variant_tag_local_handle"),
            Self::CopyLocalI64 => f.write_str("variant_tag_copy_local_i64"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SumVariantTagSeedProof {
    LocalAggregateTagSeed,
}

impl SumVariantTagSeedProof {
    fn as_str(self) -> &'static str {
        match self {
            Self::LocalAggregateTagSeed => "sum_variant_tag_local_aggregate_seed",
        }
    }
}

impl std::fmt::Display for SumVariantTagSeedProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SumVariantTagSeedRoute {
    kind: SumVariantTagSeedKind,
    enum_name: String,
    variant: String,
    subject: String,
    layout: SumLocalAggregateLayout,
    variant_tag: u32,
    make_block: BasicBlockId,
    make_instruction_index: usize,
    tag_block: BasicBlockId,
    tag_instruction_index: usize,
    sum_value: ValueId,
    tag_value: ValueId,
    tag_source_value: ValueId,
    copy_value: Option<ValueId>,
    payload_value: Option<ValueId>,
    proof: SumVariantTagSeedProof,
}

impl SumVariantTagSeedRoute {
    pub fn kind(&self) -> SumVariantTagSeedKind {
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

    pub fn tag_block(&self) -> BasicBlockId {
        self.tag_block
    }

    pub fn tag_instruction_index(&self) -> usize {
        self.tag_instruction_index
    }

    pub fn sum_value(&self) -> ValueId {
        self.sum_value
    }

    pub fn tag_value(&self) -> ValueId {
        self.tag_value
    }

    pub fn tag_source_value(&self) -> ValueId {
        self.tag_source_value
    }

    pub fn copy_value(&self) -> Option<ValueId> {
        self.copy_value
    }

    pub fn payload_value(&self) -> Option<ValueId> {
        self.payload_value
    }

    pub fn proof(&self) -> &'static str {
        self.proof.as_str()
    }
}

#[cfg(test)]
pub(crate) mod test_support {
    use super::*;

    pub(crate) fn local_i64_result_ok() -> SumVariantTagSeedRoute {
        SumVariantTagSeedRoute {
            kind: SumVariantTagSeedKind::LocalI64,
            enum_name: "Result".to_string(),
            variant: "Ok".to_string(),
            subject: "Result::Ok".to_string(),
            layout: SumLocalAggregateLayout::TagI64Payload,
            variant_tag: 0,
            make_block: BasicBlockId::new(0),
            make_instruction_index: 1,
            tag_block: BasicBlockId::new(0),
            tag_instruction_index: 2,
            sum_value: ValueId::new(2),
            tag_value: ValueId::new(3),
            tag_source_value: ValueId::new(2),
            copy_value: None,
            payload_value: Some(ValueId::new(1)),
            proof: SumVariantTagSeedProof::LocalAggregateTagSeed,
        }
    }
}

pub fn refresh_module_sum_variant_tag_seed_routes(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_sum_variant_tag_seed_route(function);
    }
}

pub fn refresh_function_sum_variant_tag_seed_route(function: &mut MirFunction) {
    function.metadata.sum_variant_tag_seed_route = match_sum_variant_tag_seed_route(function);
}

fn match_sum_variant_tag_seed_route(function: &MirFunction) -> Option<SumVariantTagSeedRoute> {
    let blocks = ordered_blocks(function);
    if blocks.len() != 1 {
        return None;
    }
    let block = blocks[0];
    let insts = instructions_with_terminator(block)?;

    if let Some(route) = match_tag_only(function, block.id, &insts) {
        return Some(route);
    }
    if let Some(route) = match_direct_payload(function, block.id, &insts) {
        return Some(route);
    }
    match_copy_i64_payload(function, block.id, &insts)
}

fn match_tag_only(
    function: &MirFunction,
    block_id: BasicBlockId,
    insts: &[&MirInstruction],
) -> Option<SumVariantTagSeedRoute> {
    expect_ops(insts, &["variant_make", "variant_tag", "ret"])?;
    let MirInstruction::VariantMake {
        dst: sum_value,
        enum_name,
        variant,
        tag,
        payload,
        payload_type,
    } = insts[0]
    else {
        return None;
    };
    if payload.is_some() || payload_type.is_some() {
        return None;
    }
    let tag_info = variant_tag_info(insts[1])?;
    let ret_value = return_value(insts[2])?;
    if tag_info.enum_name != enum_name
        || tag_info.source_value != *sum_value
        || ret_value != tag_info.dst
    {
        return None;
    }
    build_route(
        function,
        SumVariantTagSeedKind::LocalTagOnly,
        enum_name,
        variant,
        SumLocalAggregateLayout::TagOnly,
        *tag,
        block_id,
        0,
        1,
        *sum_value,
        tag_info.dst,
        tag_info.source_value,
        None,
        None,
    )
}

fn match_direct_payload(
    function: &MirFunction,
    block_id: BasicBlockId,
    insts: &[&MirInstruction],
) -> Option<SumVariantTagSeedRoute> {
    expect_ops(insts, &["const", "variant_make", "variant_tag", "ret"])?;
    let const_dst = const_dst_for_payload(insts[0])?;
    let MirInstruction::VariantMake {
        dst: sum_value,
        enum_name,
        variant,
        tag,
        payload,
        payload_type,
    } = insts[1]
    else {
        return None;
    };
    if payload != &Some(const_dst) {
        return None;
    }
    let (kind, layout) = direct_payload_kind_and_layout(insts[0], payload_type.as_ref())?;
    let tag_info = variant_tag_info(insts[2])?;
    let ret_value = return_value(insts[3])?;
    if tag_info.enum_name != enum_name
        || tag_info.source_value != *sum_value
        || ret_value != tag_info.dst
    {
        return None;
    }
    build_route(
        function,
        kind,
        enum_name,
        variant,
        layout,
        *tag,
        block_id,
        1,
        2,
        *sum_value,
        tag_info.dst,
        tag_info.source_value,
        None,
        Some(const_dst),
    )
}

fn match_copy_i64_payload(
    function: &MirFunction,
    block_id: BasicBlockId,
    insts: &[&MirInstruction],
) -> Option<SumVariantTagSeedRoute> {
    expect_ops(
        insts,
        &["const", "variant_make", "copy", "variant_tag", "ret"],
    )?;
    let const_dst = const_dst_for_payload(insts[0])?;
    if !const_is_i64(insts[0]) {
        return None;
    }
    let MirInstruction::VariantMake {
        dst: sum_value,
        enum_name,
        variant,
        tag,
        payload,
        payload_type,
    } = insts[1]
    else {
        return None;
    };
    if payload != &Some(const_dst) || !matches!(payload_type, Some(MirType::Integer)) {
        return None;
    }
    let MirInstruction::Copy {
        dst: copy_value,
        src: copy_src,
    } = insts[2]
    else {
        return None;
    };
    if copy_src != sum_value {
        return None;
    }
    let tag_info = variant_tag_info(insts[3])?;
    let ret_value = return_value(insts[4])?;
    if tag_info.enum_name != enum_name
        || tag_info.source_value != *copy_value
        || ret_value != tag_info.dst
    {
        return None;
    }
    build_route(
        function,
        SumVariantTagSeedKind::CopyLocalI64,
        enum_name,
        variant,
        SumLocalAggregateLayout::TagI64Payload,
        *tag,
        block_id,
        1,
        3,
        *sum_value,
        tag_info.dst,
        tag_info.source_value,
        Some(*copy_value),
        Some(const_dst),
    )
}

fn build_route(
    function: &MirFunction,
    kind: SumVariantTagSeedKind,
    enum_name: &str,
    variant: &str,
    layout: SumLocalAggregateLayout,
    variant_tag: u32,
    block: BasicBlockId,
    make_instruction_index: usize,
    tag_instruction_index: usize,
    sum_value: ValueId,
    tag_value: ValueId,
    tag_source_value: ValueId,
    copy_value: Option<ValueId>,
    payload_value: Option<ValueId>,
) -> Option<SumVariantTagSeedRoute> {
    let subject = format!("{enum_name}::{variant}");
    if !has_thin_selection(
        &function.metadata.thin_entry_selections,
        block,
        make_instruction_index,
        Some(sum_value),
        ThinEntrySurface::VariantMake,
        &subject,
        "variant_make.aggregate_local",
    ) || !has_thin_selection(
        &function.metadata.thin_entry_selections,
        block,
        tag_instruction_index,
        Some(tag_value),
        ThinEntrySurface::VariantTag,
        enum_name,
        "variant_tag.tag_local",
    ) {
        return None;
    }
    if !has_sum_fact(
        &function.metadata.sum_placement_facts,
        block,
        make_instruction_index,
        Some(sum_value),
        ThinEntrySurface::VariantMake,
        &subject,
        None,
    ) || !has_sum_fact(
        &function.metadata.sum_placement_facts,
        block,
        tag_instruction_index,
        Some(tag_value),
        ThinEntrySurface::VariantTag,
        enum_name,
        Some(sum_value),
    ) {
        return None;
    }
    if !has_sum_selection(
        &function.metadata.sum_placement_selections,
        block,
        make_instruction_index,
        Some(sum_value),
        ThinEntrySurface::VariantMake,
        &subject,
        None,
        "variant_make.local_aggregate",
    ) || !has_sum_selection(
        &function.metadata.sum_placement_selections,
        block,
        tag_instruction_index,
        Some(tag_value),
        ThinEntrySurface::VariantTag,
        enum_name,
        Some(sum_value),
        "variant_tag.local_aggregate",
    ) {
        return None;
    }
    if !has_sum_layout(
        &function.metadata.sum_placement_layouts,
        block,
        make_instruction_index,
        Some(sum_value),
        ThinEntrySurface::VariantMake,
        &subject,
        layout,
    ) {
        return None;
    }

    Some(SumVariantTagSeedRoute {
        kind,
        enum_name: enum_name.to_string(),
        variant: variant.to_string(),
        subject,
        layout,
        variant_tag,
        make_block: block,
        make_instruction_index,
        tag_block: block,
        tag_instruction_index,
        sum_value,
        tag_value,
        tag_source_value,
        copy_value,
        payload_value,
        proof: SumVariantTagSeedProof::LocalAggregateTagSeed,
    })
}

struct VariantTagInfo<'a> {
    dst: ValueId,
    source_value: ValueId,
    enum_name: &'a str,
}

fn variant_tag_info(inst: &MirInstruction) -> Option<VariantTagInfo<'_>> {
    match inst {
        MirInstruction::VariantTag {
            dst,
            value,
            enum_name,
        } => Some(VariantTagInfo {
            dst: *dst,
            source_value: *value,
            enum_name,
        }),
        _ => None,
    }
}

fn direct_payload_kind_and_layout(
    const_inst: &MirInstruction,
    payload_type: Option<&MirType>,
) -> Option<(SumVariantTagSeedKind, SumLocalAggregateLayout)> {
    match (const_inst, payload_type) {
        (
            MirInstruction::Const {
                value: ConstValue::Integer(_),
                ..
            },
            Some(MirType::Integer),
        ) => Some((
            SumVariantTagSeedKind::LocalI64,
            SumLocalAggregateLayout::TagI64Payload,
        )),
        (
            MirInstruction::Const {
                value: ConstValue::Float(_),
                ..
            },
            Some(MirType::Float),
        ) => Some((
            SumVariantTagSeedKind::LocalF64,
            SumLocalAggregateLayout::TagF64Payload,
        )),
        (
            MirInstruction::Const {
                value: ConstValue::String(_),
                ..
            },
            Some(MirType::String),
        ) => Some((
            SumVariantTagSeedKind::LocalHandle,
            SumLocalAggregateLayout::TagHandlePayload,
        )),
        _ => None,
    }
}

fn const_dst_for_payload(inst: &MirInstruction) -> Option<ValueId> {
    match inst {
        MirInstruction::Const { dst, .. } => Some(*dst),
        _ => None,
    }
}

fn const_is_i64(inst: &MirInstruction) -> bool {
    matches!(
        inst,
        MirInstruction::Const {
            value: ConstValue::Integer(_),
            ..
        }
    )
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
    surface: ThinEntrySurface,
    subject: &str,
    layout: SumLocalAggregateLayout,
) -> bool {
    layouts.iter().any(|candidate| {
        candidate.block == block
            && candidate.instruction_index == instruction_index
            && candidate.value == value
            && candidate.surface == surface
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
        MirInstruction::VariantTag { .. } => "variant_tag",
        MirInstruction::Copy { .. } => "copy",
        MirInstruction::Return { .. } => "ret",
        _ => "other",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{
        sum_placement::{SumObjectizationBarrier, SumPlacementFact},
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

    fn push_metadata(
        function: &mut MirFunction,
        make_idx: usize,
        tag_idx: usize,
        enum_name: &str,
        variant: &str,
        sum_value: ValueId,
        tag_value: ValueId,
        layout: SumLocalAggregateLayout,
    ) {
        let subject = format!("{enum_name}::{variant}");
        function.metadata.thin_entry_selections = vec![
            ThinEntrySelection {
                block: BasicBlockId::new(0),
                instruction_index: make_idx,
                value: Some(sum_value),
                surface: ThinEntrySurface::VariantMake,
                subject: subject.clone(),
                manifest_row: "variant_make.aggregate_local",
                selected_entry: ThinEntryPreferredEntry::ThinInternalEntry,
                state: ThinEntrySelectionState::Candidate,
                current_carrier: ThinEntryCurrentCarrier::CompatBox,
                value_class: ThinEntryValueClass::AggLocal,
                demand: ThinEntryDemand::LocalAggregate,
                reason: "test make".to_string(),
            },
            ThinEntrySelection {
                block: BasicBlockId::new(0),
                instruction_index: tag_idx,
                value: Some(tag_value),
                surface: ThinEntrySurface::VariantTag,
                subject: enum_name.to_string(),
                manifest_row: "variant_tag.tag_local",
                selected_entry: ThinEntryPreferredEntry::ThinInternalEntry,
                state: ThinEntrySelectionState::Candidate,
                current_carrier: ThinEntryCurrentCarrier::CompatBox,
                value_class: ThinEntryValueClass::InlineI64,
                demand: ThinEntryDemand::InlineScalar,
                reason: "test tag".to_string(),
            },
        ];
        function.metadata.sum_placement_facts = vec![
            SumPlacementFact {
                block: BasicBlockId::new(0),
                instruction_index: make_idx,
                value: Some(sum_value),
                surface: ThinEntrySurface::VariantMake,
                subject: subject.clone(),
                source_sum: None,
                value_class: ThinEntryValueClass::AggLocal,
                state: SumPlacementState::LocalAggregateCandidate,
                tag_reads: 1,
                project_reads: 0,
                barriers: Vec::<SumObjectizationBarrier>::new(),
                reason: "test make fact".to_string(),
            },
            SumPlacementFact {
                block: BasicBlockId::new(0),
                instruction_index: tag_idx,
                value: Some(tag_value),
                surface: ThinEntrySurface::VariantTag,
                subject: enum_name.to_string(),
                source_sum: Some(sum_value),
                value_class: ThinEntryValueClass::InlineI64,
                state: SumPlacementState::LocalAggregateCandidate,
                tag_reads: 1,
                project_reads: 0,
                barriers: Vec::<SumObjectizationBarrier>::new(),
                reason: "test tag fact".to_string(),
            },
        ];
        function.metadata.sum_placement_selections = vec![
            SumPlacementSelection {
                block: BasicBlockId::new(0),
                instruction_index: make_idx,
                value: Some(sum_value),
                surface: ThinEntrySurface::VariantMake,
                subject: subject.clone(),
                source_sum: None,
                manifest_row: "variant_make.local_aggregate",
                selected_path: SumPlacementPath::LocalAggregate,
                reason: "test make selection".to_string(),
            },
            SumPlacementSelection {
                block: BasicBlockId::new(0),
                instruction_index: tag_idx,
                value: Some(tag_value),
                surface: ThinEntrySurface::VariantTag,
                subject: enum_name.to_string(),
                source_sum: Some(sum_value),
                manifest_row: "variant_tag.local_aggregate",
                selected_path: SumPlacementPath::LocalAggregate,
                reason: "test tag selection".to_string(),
            },
        ];
        function.metadata.sum_placement_layouts = vec![SumPlacementLayout {
            block: BasicBlockId::new(0),
            instruction_index: make_idx,
            value: Some(sum_value),
            surface: ThinEntrySurface::VariantMake,
            subject,
            source_sum: None,
            layout,
            reason: "test layout".to_string(),
        }];
    }

    #[test]
    fn sum_variant_tag_seed_detects_i64_route_from_metadata() {
        let mut function = make_function();
        let block = function.get_block_mut(BasicBlockId::new(0)).unwrap();
        block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::Integer(41),
        });
        block.add_instruction(MirInstruction::VariantMake {
            dst: ValueId::new(2),
            enum_name: "Result".to_string(),
            variant: "Ok".to_string(),
            tag: 0,
            payload: Some(ValueId::new(1)),
            payload_type: Some(MirType::Integer),
        });
        block.add_instruction(MirInstruction::VariantTag {
            dst: ValueId::new(3),
            value: ValueId::new(2),
            enum_name: "Result".to_string(),
        });
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(3)),
        });
        push_metadata(
            &mut function,
            1,
            2,
            "Result",
            "Ok",
            ValueId::new(2),
            ValueId::new(3),
            SumLocalAggregateLayout::TagI64Payload,
        );

        refresh_function_sum_variant_tag_seed_route(&mut function);

        let route = function
            .metadata
            .sum_variant_tag_seed_route
            .expect("sum variant tag route");
        assert_eq!(route.kind, SumVariantTagSeedKind::LocalI64);
        assert_eq!(route.subject, "Result::Ok");
        assert_eq!(route.layout, SumLocalAggregateLayout::TagI64Payload);
        assert_eq!(route.variant_tag, 0);
        assert_eq!(route.payload_value, Some(ValueId::new(1)));
        assert_eq!(route.copy_value, None);
    }

    #[test]
    fn sum_variant_tag_seed_detects_copy_i64_route_from_metadata() {
        let mut function = make_function();
        let block = function.get_block_mut(BasicBlockId::new(0)).unwrap();
        block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::Integer(41),
        });
        block.add_instruction(MirInstruction::VariantMake {
            dst: ValueId::new(2),
            enum_name: "Result".to_string(),
            variant: "Ok".to_string(),
            tag: 0,
            payload: Some(ValueId::new(1)),
            payload_type: Some(MirType::Integer),
        });
        block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(4),
            src: ValueId::new(2),
        });
        block.add_instruction(MirInstruction::VariantTag {
            dst: ValueId::new(3),
            value: ValueId::new(4),
            enum_name: "Result".to_string(),
        });
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(3)),
        });
        push_metadata(
            &mut function,
            1,
            3,
            "Result",
            "Ok",
            ValueId::new(2),
            ValueId::new(3),
            SumLocalAggregateLayout::TagI64Payload,
        );

        refresh_function_sum_variant_tag_seed_route(&mut function);

        let route = function
            .metadata
            .sum_variant_tag_seed_route
            .expect("sum variant tag route");
        assert_eq!(route.kind, SumVariantTagSeedKind::CopyLocalI64);
        assert_eq!(route.copy_value, Some(ValueId::new(4)));
        assert_eq!(route.tag_source_value, ValueId::new(4));
    }

    #[test]
    fn sum_variant_tag_seed_rejects_missing_metadata() {
        let mut function = make_function();
        let block = function.get_block_mut(BasicBlockId::new(0)).unwrap();
        block.add_instruction(MirInstruction::VariantMake {
            dst: ValueId::new(1),
            enum_name: "ResultUnit".to_string(),
            variant: "Ok".to_string(),
            tag: 0,
            payload: None,
            payload_type: None,
        });
        block.add_instruction(MirInstruction::VariantTag {
            dst: ValueId::new(2),
            value: ValueId::new(1),
            enum_name: "ResultUnit".to_string(),
        });
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(2)),
        });

        refresh_function_sum_variant_tag_seed_route(&mut function);

        assert!(function.metadata.sum_variant_tag_seed_route.is_none());
    }
}
