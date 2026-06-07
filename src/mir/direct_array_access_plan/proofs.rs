use crate::mir::value_origin::{build_value_def_map, resolve_value_origin, ValueDefMap};
use crate::mir::{
    BasicBlock, BasicBlockId, BinaryOp, CompareOp, ConstValue, MirFunction, MirInstruction, ValueId,
};

#[derive(Debug, Default)]
pub(super) struct StackTopPopStoreIndexOrigins {
    origins: Vec<ValueId>,
}

pub(super) fn stack_top_pop_store_index_origins() -> StackTopPopStoreIndexOrigins {
    StackTopPopStoreIndexOrigins::default()
}

impl StackTopPopStoreIndexOrigins {
    pub(super) fn record(&mut self, function: &MirFunction, value: ValueId) {
        let def_map = build_value_def_map(function);
        self.origins
            .push(resolve_value_origin(function, &def_map, value));
    }

    pub(super) fn contains(&self, function: &MirFunction, value: ValueId) -> bool {
        let def_map = build_value_def_map(function);
        let origin = resolve_value_origin(function, &def_map, value);
        self.origins.iter().any(|known| *known == origin)
    }
}

pub(super) fn stack_top_pop_proves_branchless_load(
    function: &MirFunction,
    block_id: BasicBlockId,
    index_value: ValueId,
) -> bool {
    let def_map = build_value_def_map(function);
    let Some(stack_top_value) = binop_sub_one_lhs(function, index_value) else {
        return false;
    };
    predecessor_branch_proves_nonzero_on_edge(function, &def_map, block_id, stack_top_value)
}

pub(super) fn caller_precondition_proves_branchless_store(
    function: &MirFunction,
    receiver_value: ValueId,
    index_value: ValueId,
    value_value: ValueId,
) -> bool {
    if function.signature.name != "HakoAllocPageModel.releaseLocalKnownLive/1" {
        return false;
    }
    let Some(me_value) = function.params.first().copied() else {
        return false;
    };
    let Some(block_id_value) = function.params.get(1).copied() else {
        return false;
    };
    let def_map = build_value_def_map(function);
    if value_origin_is_field_get(function, &def_map, receiver_value, me_value, "block_used") {
        return resolve_value_origin(function, &def_map, index_value)
            == resolve_value_origin(function, &def_map, block_id_value)
            && value_is_integer_const(function, value_value, 0);
    }
    if value_origin_is_field_get(function, &def_map, receiver_value, me_value, "local_free") {
        return value_origin_is_field_get(
            function,
            &def_map,
            index_value,
            me_value,
            "local_free_top",
        ) && resolve_value_origin(function, &def_map, value_value)
            == resolve_value_origin(function, &def_map, block_id_value);
    }
    false
}

fn value_origin_is_field_get(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value_id: ValueId,
    base_value: ValueId,
    expected_field: &str,
) -> bool {
    let value_origin = resolve_value_origin(function, def_map, value_id);
    let base_origin = resolve_value_origin(function, def_map, base_value);
    function.blocks.values().any(|block| {
        block.instructions.iter().any(|instruction| {
            matches!(
                instruction,
                MirInstruction::FieldGet {
                    dst,
                    base,
                    field,
                    ..
                } if resolve_value_origin(function, def_map, *dst) == value_origin
                    && resolve_value_origin(function, def_map, *base) == base_origin
                    && field == expected_field
            )
        })
    })
}

fn binop_sub_one_lhs(function: &MirFunction, value_id: ValueId) -> Option<ValueId> {
    let def_map = build_value_def_map(function);
    let origin = resolve_value_origin(function, &def_map, value_id);
    function.blocks.values().find_map(|block| {
        block
            .instructions
            .iter()
            .find_map(|instruction| match instruction {
                MirInstruction::BinOp {
                    dst,
                    op: BinaryOp::Sub,
                    lhs,
                    rhs,
                } if resolve_value_origin(function, &def_map, *dst) == origin
                    && value_is_integer_const(function, *rhs, 1) =>
                {
                    Some(*lhs)
                }
                _ => None,
            })
    })
}

fn predecessor_branch_proves_nonzero_on_edge(
    function: &MirFunction,
    def_map: &ValueDefMap,
    target_block: BasicBlockId,
    value: ValueId,
) -> bool {
    let value_origin = resolve_value_origin(function, def_map, value);
    function.blocks.values().any(|pred| {
        let Some((condition, then_bb, else_bb)) = block_branch_edges(pred) else {
            return false;
        };
        let Some((op, lhs, rhs)) = compare_def(function, condition) else {
            return false;
        };
        let lhs_matches = resolve_value_origin(function, def_map, lhs) == value_origin
            && value_is_integer_const(function, rhs, 0);
        let rhs_matches = resolve_value_origin(function, def_map, rhs) == value_origin
            && value_is_integer_const(function, lhs, 0);
        if !(lhs_matches || rhs_matches) {
            return false;
        }
        match op {
            CompareOp::Eq => else_bb == target_block,
            CompareOp::Ne => then_bb == target_block,
            CompareOp::Lt => {
                // `0 < value` or `value < 0` proves non-zero on the true edge.
                (lhs_matches || rhs_matches) && then_bb == target_block
            }
            CompareOp::Gt => {
                // `value > 0` or `0 > value` proves non-zero on the true edge.
                (lhs_matches || rhs_matches) && then_bb == target_block
            }
            CompareOp::Le => {
                // `value <= 0` / `0 <= value` proves non-zero only on the false
                // edge, where the value is respectively positive/negative.
                (lhs_matches || rhs_matches) && else_bb == target_block
            }
            CompareOp::Ge => {
                // `value >= 0` / `0 >= value` proves non-zero only on the false
                // edge, where the value is respectively negative/positive.
                (lhs_matches || rhs_matches) && else_bb == target_block
            }
        }
    })
}

fn block_branch_edges(block: &BasicBlock) -> Option<(ValueId, BasicBlockId, BasicBlockId)> {
    if let Some(MirInstruction::Branch {
        condition,
        then_bb,
        else_bb,
        ..
    }) = &block.terminator
    {
        return Some((*condition, *then_bb, *else_bb));
    }
    match block.instructions.last() {
        Some(MirInstruction::Branch {
            condition,
            then_bb,
            else_bb,
            ..
        }) => Some((*condition, *then_bb, *else_bb)),
        _ => None,
    }
}

fn compare_def(function: &MirFunction, value_id: ValueId) -> Option<(CompareOp, ValueId, ValueId)> {
    function.blocks.values().find_map(|block| {
        block
            .instructions
            .iter()
            .find_map(|instruction| match instruction {
                MirInstruction::Compare { dst, op, lhs, rhs } if *dst == value_id => {
                    Some((*op, *lhs, *rhs))
                }
                _ => None,
            })
    })
}

pub(super) fn range_index_proves_branchless_append_or_overwrite_store(
    function: &MirFunction,
    block_id: BasicBlockId,
    receiver_value: ValueId,
    index_value: ValueId,
) -> bool {
    let def_map = build_value_def_map(function);
    let index_origin = resolve_value_origin(function, &def_map, index_value);
    let receiver_origin = resolve_value_origin(function, &def_map, receiver_value);
    function.metadata.range_index_facts.iter().any(|fact| {
        fact.body_bb == block_id
            && resolve_value_origin(function, &def_map, fact.index_value) == index_origin
            && fact.step == 1
            && fact.end_exclusive
            && fact.index_body_read_only
            && !fact.loop_carried_writes_supported
            && value_is_integer_const(function, fact.lower_value, 0)
            && direct_array_extent_v0_proves_upper_bound(
                function,
                receiver_origin,
                fact.upper_exclusive_value,
            )
    })
}

fn value_is_integer_const(function: &MirFunction, value_id: ValueId, expected: i64) -> bool {
    integer_const_value(function, value_id)
        .map(|actual| actual == expected)
        .unwrap_or(false)
}

fn direct_array_extent_v0_proves_upper_bound(
    function: &MirFunction,
    receiver_value: ValueId,
    end_value: ValueId,
) -> bool {
    let def_map = build_value_def_map(function);
    let receiver_origin = resolve_value_origin(function, &def_map, receiver_value);
    let end_origin = resolve_value_origin(function, &def_map, end_value);
    function
        .metadata
        .direct_array_extent_facts
        .iter()
        .any(|fact| {
            resolve_value_origin(function, &def_map, fact.receiver_value) == receiver_origin
                && resolve_value_origin(function, &def_map, fact.lower_bound_value) == end_origin
                && fact.stable_in_region
                && direct_array_region_stability_v0_proves(
                    function,
                    fact.region_stability_fact_id,
                    receiver_origin,
                )
        })
}

fn direct_array_region_stability_v0_proves(
    function: &MirFunction,
    fact_id: u32,
    receiver_origin: ValueId,
) -> bool {
    let def_map = build_value_def_map(function);
    function.metadata.region_stability_facts.iter().any(|fact| {
        fact.fact_id == fact_id
            && fact.stable_in_region
            && resolve_value_origin(function, &def_map, fact.region_value) == receiver_origin
    })
}

fn integer_const_value(function: &MirFunction, value_id: ValueId) -> Option<i64> {
    let def_map = build_value_def_map(function);
    let origin = resolve_value_origin(function, &def_map, value_id);
    function.blocks.values().find_map(|block| {
        block
            .instructions
            .iter()
            .find_map(|instruction| match instruction {
                MirInstruction::Const {
                    dst,
                    value: ConstValue::Integer(actual),
                } if resolve_value_origin(function, &def_map, *dst) == origin => Some(*actual),
                _ => None,
            })
    })
}

pub(super) fn checked_direct_array_lowering_site_is_cfg_safe(
    function: &MirFunction,
    block_id: BasicBlockId,
) -> bool {
    let Some(block) = function.blocks.get(&block_id) else {
        return false;
    };
    // The current checked DirectArray C lowering emits a local success/fail
    // branch and rejoins before the original block terminator.  That changes
    // the concrete predecessor label seen by successor PHIs, so v0 only marks
    // checked plans for blocks whose direct successors do not begin with PHI.
    // Proved-unchecked plans are branchless and are handled in a later slice.
    block.successors_from_terminator().iter().all(|successor| {
        function
            .blocks
            .get(successor)
            .map(|successor_block| {
                !matches!(
                    successor_block.instructions.first(),
                    Some(MirInstruction::Phi { .. })
                )
            })
            .unwrap_or(false)
    })
}

pub(super) fn array_store_value_arg(
    function: &MirFunction,
    block: BasicBlockId,
    instruction_index: usize,
    key_value: ValueId,
) -> Option<ValueId> {
    let block = function.blocks.get(&block)?;
    let def_map = build_value_def_map(function);
    let key_origin = resolve_value_origin(function, &def_map, key_value);
    match block.instructions.get(instruction_index)? {
        MirInstruction::Call { args, .. } => args
            .iter()
            .position(|arg| resolve_value_origin(function, &def_map, *arg) == key_origin)
            .and_then(|index| args.get(index + 1).copied()),
        _ => None,
    }
}
