/*!
 * Nested executor contract for array/text observer routes.
 *
 * This is implementation support for `array_text_observer_routes`, not a new
 * metadata family. MIR owns the legality/proof; backends only consume the
 * nested contract.
 */

use super::{
    array_text_observer_plan::{ArrayTextObserverPublicationBoundary, ArrayTextObserverRoute},
    definitions::Callee,
    resolve_value_origin, BasicBlockId, BinaryOp, CompareOp, ConstValue, MirFunction,
    MirInstruction, ValueDefMap, ValueId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayTextObserverExecutorExecutionMode {
    SingleRegionExecutor,
}

impl std::fmt::Display for ArrayTextObserverExecutorExecutionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SingleRegionExecutor => f.write_str("single_region_executor"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayTextObserverExecutorProofRegion {
    LoopBackedgeSingleBody,
}

impl std::fmt::Display for ArrayTextObserverExecutorProofRegion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LoopBackedgeSingleBody => f.write_str("loop_backedge_single_body"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayTextObserverExecutorCarrier {
    ArrayLaneTextCell,
}

impl std::fmt::Display for ArrayTextObserverExecutorCarrier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ArrayLaneTextCell => f.write_str("array_lane_text_cell"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayTextObserverExecutorEffect {
    ObserveIndexOf,
    StoreCell,
}

impl std::fmt::Display for ArrayTextObserverExecutorEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ObserveIndexOf => f.write_str("observe.indexof"),
            Self::StoreCell => f.write_str("store.cell"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayTextObserverExecutorConsumerCapability {
    CompareOnly,
    SinkStore,
}

impl std::fmt::Display for ArrayTextObserverExecutorConsumerCapability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CompareOnly => f.write_str("compare_only"),
            Self::SinkStore => f.write_str("sink_store"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayTextObserverExecutorMaterializationPolicy {
    TextResidentOrStringlikeSlot,
}

impl std::fmt::Display for ArrayTextObserverExecutorMaterializationPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TextResidentOrStringlikeSlot => f.write_str("text_resident_or_stringlike_slot"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayTextObserverStoreRegionMapping {
    pub array_root_value: ValueId,
    pub loop_index_phi_value: ValueId,
    pub loop_index_initial_value: ValueId,
    pub loop_index_initial_const: i64,
    pub loop_index_next_value: ValueId,
    pub loop_bound_value: ValueId,
    pub loop_bound_const: i64,
    pub begin_block: BasicBlockId,
    pub begin_to_header_block: BasicBlockId,
    pub header_block: BasicBlockId,
    pub observer_block: BasicBlockId,
    pub observer_instruction_index: usize,
    pub predicate_value: ValueId,
    pub then_store_block: BasicBlockId,
    pub store_instruction_index: usize,
    pub suffix_value: ValueId,
    pub suffix_text: String,
    pub suffix_byte_len: usize,
    pub latch_block: BasicBlockId,
    pub exit_block: BasicBlockId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayTextObserverExecutorContract {
    pub execution_mode: ArrayTextObserverExecutorExecutionMode,
    pub proof_region: ArrayTextObserverExecutorProofRegion,
    pub publication_boundary: ArrayTextObserverPublicationBoundary,
    pub carrier: ArrayTextObserverExecutorCarrier,
    pub effects: Vec<ArrayTextObserverExecutorEffect>,
    pub consumer_capabilities: Vec<ArrayTextObserverExecutorConsumerCapability>,
    pub materialization_policy: ArrayTextObserverExecutorMaterializationPolicy,
    pub region_mapping: Option<ArrayTextObserverStoreRegionMapping>,
}

impl ArrayTextObserverExecutorContract {
    pub(crate) fn conditional_suffix_store_single_region(
        region_mapping: ArrayTextObserverStoreRegionMapping,
    ) -> Self {
        Self {
            execution_mode: ArrayTextObserverExecutorExecutionMode::SingleRegionExecutor,
            proof_region: ArrayTextObserverExecutorProofRegion::LoopBackedgeSingleBody,
            publication_boundary: ArrayTextObserverPublicationBoundary::None,
            carrier: ArrayTextObserverExecutorCarrier::ArrayLaneTextCell,
            effects: vec![
                ArrayTextObserverExecutorEffect::ObserveIndexOf,
                ArrayTextObserverExecutorEffect::StoreCell,
            ],
            consumer_capabilities: vec![
                ArrayTextObserverExecutorConsumerCapability::CompareOnly,
                ArrayTextObserverExecutorConsumerCapability::SinkStore,
            ],
            materialization_policy:
                ArrayTextObserverExecutorMaterializationPolicy::TextResidentOrStringlikeSlot,
            region_mapping: Some(region_mapping),
        }
    }
}

pub(crate) fn derive_observer_store_region_contract(
    function: &MirFunction,
    def_map: &ValueDefMap,
    route: &ArrayTextObserverRoute,
) -> Option<ArrayTextObserverStoreRegionMapping> {
    if !route.has_found_predicate_consumer() {
        return None;
    }
    if !route.observer_arg0_is_const_utf8() {
        return None;
    }
    if route.keep_get_live() || route.observer_arg0_keep_live() {
        return None;
    }

    let observer_block = function.blocks.get(&route.block())?;
    let (predicate_value, branch_then_block, branch_else_block) =
        match_found_predicate_branch(function, def_map, observer_block, route.result_value())?;
    let (then_block, latch_block, store_instruction_index, suffix_value, suffix_text) =
        if let Some((store_instruction_index, suffix_value, suffix_text)) =
            match_then_same_slot_suffix_store(function, def_map, branch_then_block, route)
        {
            (
                branch_then_block,
                branch_else_block,
                store_instruction_index,
                suffix_value,
                suffix_text,
            )
        } else if let Some((store_instruction_index, suffix_value, suffix_text)) =
            match_then_same_slot_suffix_store(function, def_map, branch_else_block, route)
        {
            (
                branch_else_block,
                branch_then_block,
                store_instruction_index,
                suffix_value,
                suffix_text,
            )
        } else {
            return None;
        };
    let then = function.blocks.get(&then_block)?;
    if !matches!(
        then.terminator.as_ref()?,
        MirInstruction::Jump { target, .. } if *target == latch_block
    ) {
        return None;
    }
    let latch = function.blocks.get(&latch_block)?;
    let header_block = match latch.terminator.as_ref()? {
        MirInstruction::Jump { target, .. } => *target,
        _ => return None,
    };
    let header = function.blocks.get(&header_block)?;
    if !header.predecessors.contains(&latch_block) {
        return None;
    }
    let exit_block = match header.terminator.as_ref()? {
        MirInstruction::Branch {
            then_bb, else_bb, ..
        } if *then_bb == route.block() => *else_bb,
        MirInstruction::Branch {
            then_bb, else_bb, ..
        } if *else_bb == route.block() => *then_bb,
        _ => return None,
    };
    let exit = function.blocks.get(&exit_block)?;
    if exit
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Phi { .. }))
    {
        return None;
    }
    if !observer_block.predecessors.contains(&header_block)
        || !latch.predecessors.contains(&route.block())
        || !latch.predecessors.contains(&then_block)
    {
        return None;
    }
    let preheader = single_preheader_jump_to_header(function, header_block, latch_block)?;
    let (loop_index_phi_value, loop_bound_value) =
        match_loop_index_condition(function, def_map, header)?;
    if loop_index_phi_value != root(function, def_map, route.index_value()) {
        return None;
    }
    let loop_index_initial_value = phi_input_from(header, loop_index_phi_value, preheader)?;
    let loop_index_initial_const = const_i64(function, def_map, loop_index_initial_value)?;
    if loop_index_initial_const != 0 {
        return None;
    }
    let loop_index_next_value = phi_input_from(header, loop_index_phi_value, latch_block)?;
    if !is_add_const_one_from(
        function,
        def_map,
        latch,
        loop_index_next_value,
        loop_index_phi_value,
    ) {
        return None;
    }
    let loop_bound_const = const_i64(function, def_map, loop_bound_value)?;

    Some(ArrayTextObserverStoreRegionMapping {
        array_root_value: root(function, def_map, route.array_value()),
        loop_index_phi_value,
        loop_index_initial_value,
        loop_index_initial_const,
        loop_index_next_value,
        loop_bound_value,
        loop_bound_const,
        begin_block: preheader,
        begin_to_header_block: header_block,
        header_block,
        observer_block: route.block(),
        observer_instruction_index: route.observer_instruction_index(),
        predicate_value,
        then_store_block: then_block,
        store_instruction_index,
        suffix_value,
        suffix_byte_len: suffix_text.len(),
        suffix_text,
        latch_block,
        exit_block,
    })
}

fn root(function: &MirFunction, def_map: &ValueDefMap, value: ValueId) -> ValueId {
    resolve_value_origin(function, def_map, value)
}

fn const_i64(function: &MirFunction, def_map: &ValueDefMap, value: ValueId) -> Option<i64> {
    let value = root(function, def_map, value);
    let (block, index) = def_map.get(&value).copied()?;
    match function.blocks.get(&block)?.instructions.get(index)? {
        MirInstruction::Const {
            value: ConstValue::Integer(actual),
            ..
        } => Some(*actual),
        _ => None,
    }
}

fn const_utf8(function: &MirFunction, def_map: &ValueDefMap, value: ValueId) -> Option<String> {
    let value = root(function, def_map, value);
    let (block, index) = def_map.get(&value).copied()?;
    match function.blocks.get(&block)?.instructions.get(index)? {
        MirInstruction::Const {
            value: ConstValue::String(actual),
            ..
        } => Some(actual.clone()),
        _ => None,
    }
}

fn match_found_predicate_branch(
    function: &MirFunction,
    def_map: &ValueDefMap,
    block: &super::BasicBlock,
    result_value: ValueId,
) -> Option<(ValueId, BasicBlockId, BasicBlockId)> {
    let result_root = root(function, def_map, result_value);
    let predicate_value = block.instructions.iter().find_map(|inst| match inst {
        MirInstruction::Compare { dst, .. }
            if compare_is_found_predicate(function, def_map, inst, result_root) =>
        {
            Some(*dst)
        }
        _ => None,
    })?;
    let MirInstruction::Branch {
        condition,
        then_bb,
        else_bb,
        ..
    } = block.terminator.as_ref()?
    else {
        return None;
    };
    if root(function, def_map, *condition) != root(function, def_map, predicate_value) {
        return None;
    }
    Some((predicate_value, *then_bb, *else_bb))
}

fn compare_is_found_predicate(
    function: &MirFunction,
    def_map: &ValueDefMap,
    inst: &MirInstruction,
    result_root: ValueId,
) -> bool {
    let MirInstruction::Compare { op, lhs, rhs, .. } = inst else {
        return false;
    };
    let lhs_root = root(function, def_map, *lhs);
    let rhs_root = root(function, def_map, *rhs);
    if lhs_root == result_root {
        return match const_i64(function, def_map, *rhs) {
            Some(0) => matches!(op, CompareOp::Ge),
            Some(-1) => matches!(op, CompareOp::Gt | CompareOp::Ne),
            _ => false,
        };
    }
    if rhs_root == result_root {
        return match const_i64(function, def_map, *lhs) {
            Some(0) => matches!(op, CompareOp::Le),
            Some(-1) => matches!(op, CompareOp::Lt | CompareOp::Ne),
            _ => false,
        };
    }
    false
}

fn match_then_same_slot_suffix_store(
    function: &MirFunction,
    def_map: &ValueDefMap,
    block_id: BasicBlockId,
    route: &ArrayTextObserverRoute,
) -> Option<(usize, ValueId, String)> {
    let block = function.blocks.get(&block_id)?;
    let source_root = root(function, def_map, route.source_value());
    let array_root = root(function, def_map, route.array_value());
    let index_root = root(function, def_map, route.index_value());
    for inst in &block.instructions {
        let Some((concat_value, suffix_value, suffix_text)) =
            const_suffix_concat_details(function, def_map, inst, source_root)
        else {
            continue;
        };
        for (store_instruction_index, store) in block.instructions.iter().enumerate() {
            if is_same_slot_set_consumer(
                function,
                def_map,
                store,
                concat_value,
                array_root,
                index_root,
            ) {
                return Some((store_instruction_index, suffix_value, suffix_text));
            }
        }
    }
    None
}

fn const_suffix_concat_details(
    function: &MirFunction,
    def_map: &ValueDefMap,
    inst: &MirInstruction,
    source_root: ValueId,
) -> Option<(ValueId, ValueId, String)> {
    let MirInstruction::BinOp {
        dst,
        op: BinaryOp::Add,
        lhs,
        rhs,
        ..
    } = inst
    else {
        return None;
    };
    let lhs_root = root(function, def_map, *lhs);
    let rhs_root = root(function, def_map, *rhs);
    if lhs_root == source_root {
        let suffix_text = const_utf8(function, def_map, rhs_root)?;
        return Some((*dst, rhs_root, suffix_text));
    }
    if rhs_root == source_root {
        let suffix_text = const_utf8(function, def_map, lhs_root)?;
        return Some((*dst, lhs_root, suffix_text));
    }
    None
}

fn is_same_slot_set_consumer(
    function: &MirFunction,
    def_map: &ValueDefMap,
    inst: &MirInstruction,
    value_root: ValueId,
    array_root: ValueId,
    index_root: ValueId,
) -> bool {
    match inst {
        MirInstruction::Call {
            callee:
                Some(Callee::Method {
                    box_name,
                    method,
                    receiver: Some(receiver),
                    ..
                }),
            args,
            ..
        } if method == "set"
            && args.len() == 2
            && matches!(box_name.as_str(), "RuntimeDataBox" | "ArrayBox") =>
        {
            root(function, def_map, *receiver) == array_root
                && root(function, def_map, args[0]) == index_root
                && root(function, def_map, args[1]) == value_root
        }
        _ => false,
    }
}

fn single_preheader_jump_to_header(
    function: &MirFunction,
    header_block: BasicBlockId,
    latch_block: BasicBlockId,
) -> Option<BasicBlockId> {
    let header = function.blocks.get(&header_block)?;
    let mut non_latch_predecessors = header
        .predecessors
        .iter()
        .copied()
        .filter(|predecessor| *predecessor != latch_block);
    let preheader = non_latch_predecessors.next()?;
    if non_latch_predecessors.next().is_some() {
        return None;
    }
    let preheader_block = function.blocks.get(&preheader)?;
    match preheader_block.terminator.as_ref()? {
        MirInstruction::Jump { target, .. } if *target == header_block => Some(preheader),
        _ => None,
    }
}

fn match_loop_index_condition(
    function: &MirFunction,
    def_map: &ValueDefMap,
    header: &super::BasicBlock,
) -> Option<(ValueId, ValueId)> {
    let condition = match header.terminator.as_ref()? {
        MirInstruction::Branch { condition, .. } => *condition,
        _ => return None,
    };
    let condition = root(function, def_map, condition);
    let compare = header.instructions.iter().find_map(|inst| match inst {
        MirInstruction::Compare {
            dst,
            op: CompareOp::Lt,
            lhs,
            rhs,
        } if root(function, def_map, *dst) == condition => Some((*lhs, *rhs)),
        _ => None,
    })?;
    let loop_index_phi_value = root(function, def_map, compare.0);
    if !is_phi_dst(header, loop_index_phi_value) {
        return None;
    }
    Some((loop_index_phi_value, root(function, def_map, compare.1)))
}

fn is_phi_dst(block: &super::BasicBlock, value: ValueId) -> bool {
    block.instructions.iter().any(|inst| {
        matches!(
            inst,
            MirInstruction::Phi { dst, .. } if *dst == value
        )
    })
}

fn phi_input_from(
    block: &super::BasicBlock,
    phi_value: ValueId,
    predecessor: BasicBlockId,
) -> Option<ValueId> {
    block.instructions.iter().find_map(|inst| match inst {
        MirInstruction::Phi { dst, inputs, .. } if *dst == phi_value => inputs
            .iter()
            .find_map(|(block, value)| (*block == predecessor).then_some(*value)),
        _ => None,
    })
}

fn is_add_const_one_from(
    function: &MirFunction,
    def_map: &ValueDefMap,
    block: &super::BasicBlock,
    next_value: ValueId,
    source_value: ValueId,
) -> bool {
    let next_value = root(function, def_map, next_value);
    block.instructions.iter().any(|inst| match inst {
        MirInstruction::BinOp {
            dst,
            op: BinaryOp::Add,
            lhs,
            rhs,
        } if *dst == next_value => {
            (root(function, def_map, *lhs) == source_value
                && const_i64(function, def_map, *rhs) == Some(1))
                || (root(function, def_map, *rhs) == source_value
                    && const_i64(function, def_map, *lhs) == Some(1))
        }
        _ => false,
    })
}
