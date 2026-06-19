/*!
 * Passive dead text-region plans for substring/concat rotation loops.
 *
 * This module owns the MIR-side observation for loops where the carried string
 * content is rebuilt each iteration but only its length contributes to the
 * final result. It does not mutate MIR and it does not enable backend lowering.
 */

use crate::mir::array_receiver_proof::value_root;
use crate::mir::string_corridor_names::{
    is_len_method_name, is_lowered_len_global, is_runtime_len_handle_export,
};
use crate::mir::string_corridor_recognizer::{
    match_substring_call, match_substring_concat3_helper_call,
};
use crate::mir::value_origin::{build_value_def_map, ValueDefMap};
use crate::mir::{
    BasicBlock, BasicBlockId, BinaryOp, Callee, CompareOp, ConstValue, MirFunction, MirInstruction,
    ValueId,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringDeadTextRegionPlan {
    pub loop_header: BasicBlockId,
    pub loop_body: BasicBlockId,
    pub loop_exit: BasicBlockId,
    pub text_phi_value: ValueId,
    pub text_initial_value: ValueId,
    pub loop_index_phi_value: ValueId,
    pub loop_index_initial_value: ValueId,
    pub loop_index_initial_const: i64,
    pub loop_index_next_value: ValueId,
    pub loop_bound_value: ValueId,
    pub loop_bound_const: i64,
    pub base_len_value: ValueId,
    pub base_len_const: i64,
    pub inserted_text_value: ValueId,
    pub inserted_text: String,
    pub inserted_len_const: i64,
    pub accumulator_phi_value: ValueId,
    pub accumulator_initial_value: ValueId,
    pub accumulator_initial_const: i64,
    pub accumulator_next_value: ValueId,
    pub accumulator_delta_value: ValueId,
    pub accumulator_delta_const: i64,
    pub exit_accumulator_value: ValueId,
    pub final_len_value: ValueId,
    pub final_return_value: i64,
    pub substring_left_value: ValueId,
    pub substring_right_value: ValueId,
    pub concat_result_value: ValueId,
    pub rotation_start_value: ValueId,
    pub rotation_end_value: ValueId,
    pub split_value: ValueId,
    pub publication_boundary: &'static str,
    pub final_text_content_observed: bool,
    pub lowering_consumer_enabled: bool,
}

impl StringDeadTextRegionPlan {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        loop_header: BasicBlockId,
        loop_body: BasicBlockId,
        loop_exit: BasicBlockId,
        text_phi_value: ValueId,
        text_initial_value: ValueId,
        loop_index_phi_value: ValueId,
        loop_index_initial_value: ValueId,
        loop_index_initial_const: i64,
        loop_index_next_value: ValueId,
        loop_bound_value: ValueId,
        loop_bound_const: i64,
        base_len_value: ValueId,
        base_len_const: i64,
        inserted_text_value: ValueId,
        inserted_text: String,
        inserted_len_const: i64,
        accumulator_phi_value: ValueId,
        accumulator_initial_value: ValueId,
        accumulator_initial_const: i64,
        accumulator_next_value: ValueId,
        accumulator_delta_value: ValueId,
        accumulator_delta_const: i64,
        exit_accumulator_value: ValueId,
        final_len_value: ValueId,
        final_return_value: i64,
        substring_left_value: ValueId,
        substring_right_value: ValueId,
        concat_result_value: ValueId,
        rotation_start_value: ValueId,
        rotation_end_value: ValueId,
        split_value: ValueId,
    ) -> Self {
        Self {
            loop_header,
            loop_body,
            loop_exit,
            text_phi_value,
            text_initial_value,
            loop_index_phi_value,
            loop_index_initial_value,
            loop_index_initial_const,
            loop_index_next_value,
            loop_bound_value,
            loop_bound_const,
            base_len_value,
            base_len_const,
            inserted_text_value,
            inserted_text,
            inserted_len_const,
            accumulator_phi_value,
            accumulator_initial_value,
            accumulator_initial_const,
            accumulator_next_value,
            accumulator_delta_value,
            accumulator_delta_const,
            exit_accumulator_value,
            final_len_value,
            final_return_value,
            substring_left_value,
            substring_right_value,
            concat_result_value,
            rotation_start_value,
            rotation_end_value,
            split_value,
            publication_boundary: "none",
            final_text_content_observed: false,
            lowering_consumer_enabled: false,
        }
    }
}

pub fn refresh_module_string_dead_text_region_plans(module: &mut crate::mir::MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_string_dead_text_region_plans(function);
    }
}

pub fn refresh_function_string_dead_text_region_plans(function: &mut MirFunction) {
    let def_map = build_value_def_map(function);
    refresh_function_string_dead_text_region_plans_with_def_map(function, &def_map);
}

pub(crate) fn refresh_function_string_dead_text_region_plans_with_def_map(
    function: &mut MirFunction,
    def_map: &ValueDefMap,
) {
    function.metadata.string_dead_text_region_plans =
        derive_string_dead_text_region_plans(function, def_map);
}

fn derive_string_dead_text_region_plans(
    function: &MirFunction,
    def_map: &ValueDefMap,
) -> Vec<StringDeadTextRegionPlan> {
    let mut plans = Vec::new();
    for (header_id, header) in &function.blocks {
        if let Some(plan) = match_header(function, def_map, *header_id, header) {
            plans.push(plan);
        }
    }
    plans
}

fn match_header(
    function: &MirFunction,
    def_map: &ValueDefMap,
    header_id: BasicBlockId,
    header: &BasicBlock,
) -> Option<StringDeadTextRegionPlan> {
    let (first_target, second_target, condition) = match header.terminator.as_ref()? {
        MirInstruction::Branch {
            condition,
            then_bb,
            else_bb,
            ..
        } => (*then_bb, *else_bb, *condition),
        _ => return None,
    };
    let (body_id, exit_id) =
        select_loop_body_exit(function, header_id, first_target, second_target)?;
    let body = function.blocks.get(&body_id)?;
    let exit = function.blocks.get(&exit_id)?;
    let preheader_id = single_preheader(function, header_id, body_id)?;

    let (loop_index_phi, loop_bound_value) =
        match_loop_condition(function, def_map, header, condition)?;
    let loop_index_initial = phi_input_from(header, loop_index_phi, preheader_id)?;
    let loop_index_next = phi_input_from(header, loop_index_phi, body_id)?;
    if const_i64(function, def_map, loop_index_initial)? != 0 {
        return None;
    }
    if !is_add_const_from(function, def_map, body, loop_index_next, loop_index_phi, 1) {
        return None;
    }
    let loop_bound_const = const_i64(function, def_map, loop_bound_value)?;

    let concat = body
        .instructions
        .iter()
        .find_map(match_substring_concat3_helper_call)?;
    let text_phi = header.instructions.iter().find_map(|inst| {
        let MirInstruction::Phi { dst, .. } = inst else {
            return None;
        };
        (phi_input_from(header, *dst, body_id).map(|v| value_root(function, def_map, v))
            == Some(value_root(function, def_map, concat.dst)))
        .then_some(*dst)
    })?;
    let text_initial = phi_input_from(header, text_phi, preheader_id)?;
    let text_initial_const = const_text(function, def_map, text_initial)?;
    let base_len_const = text_initial_const.len() as i64;

    let left = match_substring_producer(function, def_map, body, concat.left)?;
    let right = match_substring_producer(function, def_map, body, concat.right)?;
    let text_root = value_root(function, def_map, text_phi);
    if value_root(function, def_map, left.source) != text_root
        || value_root(function, def_map, right.source) != text_root
        || const_i64(function, def_map, left.start)? != 0
        || value_root(function, def_map, left.end) != value_root(function, def_map, right.start)
    {
        return None;
    }
    let split_value = value_root(function, def_map, left.end);
    let base_len_value = value_root(function, def_map, right.end);
    if !is_div_const_from(function, def_map, body, split_value, base_len_value, 2) {
        return None;
    }
    if !is_len_value_for_source(function, def_map, base_len_value, text_initial) {
        return None;
    }

    let inserted_text = const_text(function, def_map, concat.middle)?;
    let inserted_len_const = inserted_text.len() as i64;
    if const_i64(function, def_map, concat.start)? != 1 {
        return None;
    }
    if !is_add_const_from(function, def_map, body, concat.end, base_len_value, 1) {
        return None;
    }

    let (
        accumulator_phi,
        accumulator_initial,
        accumulator_next,
        accumulator_delta_value,
        accumulator_delta_const,
    ) = match_accumulator(
        function,
        def_map,
        header,
        body,
        preheader_id,
        body_id,
        base_len_value,
    )?;
    let accumulator_initial_const = const_i64(function, def_map, accumulator_initial)?;
    if accumulator_delta_const != base_len_const + inserted_len_const {
        return None;
    }
    let exit_accumulator = value_root(function, def_map, accumulator_phi);
    let (final_len_value, final_return_value) = match_exit_return(
        function,
        def_map,
        exit,
        exit_accumulator,
        text_root,
        accumulator_initial_const,
        loop_bound_const,
        accumulator_delta_const,
        base_len_const,
    )?;

    if text_content_observed_after_loop(function, def_map, exit, text_root, final_len_value) {
        return None;
    }

    Some(StringDeadTextRegionPlan::new(
        header_id,
        body_id,
        exit_id,
        text_phi,
        value_root(function, def_map, text_initial),
        loop_index_phi,
        value_root(function, def_map, loop_index_initial),
        0,
        value_root(function, def_map, loop_index_next),
        loop_bound_value,
        loop_bound_const,
        base_len_value,
        base_len_const,
        value_root(function, def_map, concat.middle),
        inserted_text,
        inserted_len_const,
        accumulator_phi,
        value_root(function, def_map, accumulator_initial),
        accumulator_initial_const,
        value_root(function, def_map, accumulator_next),
        value_root(function, def_map, accumulator_delta_value),
        accumulator_delta_const,
        exit_accumulator,
        final_len_value,
        final_return_value,
        value_root(function, def_map, left.dst),
        value_root(function, def_map, right.dst),
        value_root(function, def_map, concat.dst),
        value_root(function, def_map, concat.start),
        value_root(function, def_map, concat.end),
        split_value,
    ))
}

fn select_loop_body_exit(
    function: &MirFunction,
    header_id: BasicBlockId,
    first_target: BasicBlockId,
    second_target: BasicBlockId,
) -> Option<(BasicBlockId, BasicBlockId)> {
    let first_returns = function
        .blocks
        .get(&first_target)
        .is_some_and(|block| block_successors(block).contains(&header_id));
    let second_returns = function
        .blocks
        .get(&second_target)
        .is_some_and(|block| block_successors(block).contains(&header_id));
    match (first_returns, second_returns) {
        (true, false) => Some((first_target, second_target)),
        (false, true) => Some((second_target, first_target)),
        _ => None,
    }
}

#[derive(Debug, Clone, Copy)]
struct SubstringProducer {
    dst: ValueId,
    source: ValueId,
    start: ValueId,
    end: ValueId,
}

fn match_substring_producer(
    function: &MirFunction,
    def_map: &ValueDefMap,
    body: &BasicBlock,
    value: ValueId,
) -> Option<SubstringProducer> {
    let value = value_root(function, def_map, value);
    body.instructions.iter().find_map(|inst| {
        let (dst, source, start, end, _) = match_substring_call(inst)?;
        (value_root(function, def_map, dst) == value).then_some(SubstringProducer {
            dst,
            source,
            start,
            end,
        })
    })
}

fn single_preheader(
    function: &MirFunction,
    header_id: BasicBlockId,
    body_id: BasicBlockId,
) -> Option<BasicBlockId> {
    let mut preheaders = function
        .blocks
        .iter()
        .filter_map(|(candidate_id, block)| {
            block_successors(block)
                .contains(&header_id)
                .then_some(*candidate_id)
        })
        .filter(|predecessor| *predecessor != body_id);
    let preheader = preheaders.next()?;
    if preheaders.next().is_some() {
        return None;
    }
    let preheader_block = function.blocks.get(&preheader)?;
    matches!(
        preheader_block.terminator.as_ref()?,
        MirInstruction::Jump { target, .. } if *target == header_id
    )
    .then_some(preheader)
}

fn match_loop_condition(
    function: &MirFunction,
    def_map: &ValueDefMap,
    header: &BasicBlock,
    condition: ValueId,
) -> Option<(ValueId, ValueId)> {
    let condition = value_root(function, def_map, condition);
    header.instructions.iter().find_map(|inst| match inst {
        MirInstruction::Compare {
            dst,
            op: CompareOp::Lt,
            lhs,
            rhs,
        } if value_root(function, def_map, *dst) == condition => {
            let lhs = value_root(function, def_map, *lhs);
            if is_phi_dst(header, lhs) {
                Some((lhs, value_root(function, def_map, *rhs)))
            } else {
                None
            }
        }
        _ => None,
    })
}

fn is_phi_dst(block: &BasicBlock, value: ValueId) -> bool {
    block
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Phi { dst, .. } if *dst == value))
}

fn phi_input_from(
    block: &BasicBlock,
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

fn is_add_const_from(
    function: &MirFunction,
    def_map: &ValueDefMap,
    body: &BasicBlock,
    value: ValueId,
    source: ValueId,
    constant: i64,
) -> bool {
    let value = value_root(function, def_map, value);
    let source = value_root(function, def_map, source);
    body.instructions.iter().any(|inst| match inst {
        MirInstruction::BinOp {
            dst,
            op: BinaryOp::Add,
            lhs,
            rhs,
        } if value_root(function, def_map, *dst) == value => {
            (value_root(function, def_map, *lhs) == source
                && const_i64(function, def_map, *rhs) == Some(constant))
                || (value_root(function, def_map, *rhs) == source
                    && const_i64(function, def_map, *lhs) == Some(constant))
        }
        _ => false,
    })
}

fn is_div_const_from(
    function: &MirFunction,
    def_map: &ValueDefMap,
    body: &BasicBlock,
    value: ValueId,
    source: ValueId,
    constant: i64,
) -> bool {
    let value = value_root(function, def_map, value);
    let source = value_root(function, def_map, source);
    body.instructions.iter().any(|inst| match inst {
        MirInstruction::BinOp {
            dst,
            op: BinaryOp::Div,
            lhs,
            rhs,
        } if value_root(function, def_map, *dst) == value => {
            value_root(function, def_map, *lhs) == source
                && const_i64(function, def_map, *rhs) == Some(constant)
        }
        _ => false,
    })
}

fn match_accumulator(
    function: &MirFunction,
    def_map: &ValueDefMap,
    header: &BasicBlock,
    body: &BasicBlock,
    preheader_id: BasicBlockId,
    body_id: BasicBlockId,
    base_len_value: ValueId,
) -> Option<(ValueId, ValueId, ValueId, ValueId, i64)> {
    header.instructions.iter().find_map(|inst| {
        let MirInstruction::Phi { dst, inputs, .. } = inst else {
            return None;
        };
        let initial = inputs
            .iter()
            .find_map(|(block, value)| (*block == preheader_id).then_some(*value))?;
        let next = inputs
            .iter()
            .find_map(|(block, value)| (*block == body_id).then_some(*value))?;
        let next_root = value_root(function, def_map, next);
        let acc_root = value_root(function, def_map, *dst);
        body.instructions.iter().find_map(|body_inst| {
            let delta_value = match body_inst {
                MirInstruction::BinOp {
                    dst: add_dst,
                    op: BinaryOp::Add,
                    lhs,
                    rhs,
                } if value_root(function, def_map, *add_dst) == next_root => {
                    let lhs = value_root(function, def_map, *lhs);
                    let rhs = value_root(function, def_map, *rhs);
                    if lhs == acc_root {
                        rhs
                    } else if rhs == acc_root {
                        lhs
                    } else {
                        return None;
                    }
                }
                _ => return None,
            };
            let delta_const =
                match_accumulator_delta(function, def_map, body, delta_value, base_len_value)?;
            Some((*dst, initial, next, delta_value, delta_const))
        })
    })
}

fn match_accumulator_delta(
    function: &MirFunction,
    def_map: &ValueDefMap,
    body: &BasicBlock,
    delta_value: ValueId,
    base_len_value: ValueId,
) -> Option<i64> {
    let delta = value_root(function, def_map, delta_value);
    let base_len = value_root(function, def_map, base_len_value);
    body.instructions.iter().find_map(|inst| match inst {
        MirInstruction::BinOp {
            dst,
            op: BinaryOp::Add,
            lhs,
            rhs,
        } if value_root(function, def_map, *dst) == delta => {
            let lhs_root = value_root(function, def_map, *lhs);
            let rhs_root = value_root(function, def_map, *rhs);
            let base_len_const = const_len(function, def_map, base_len)?;
            if lhs_root == base_len {
                const_i64(function, def_map, *rhs).map(|inserted| inserted + base_len_const)
            } else if rhs_root == base_len {
                const_i64(function, def_map, *lhs).map(|inserted| inserted + base_len_const)
            } else {
                None
            }
        }
        _ => None,
    })
}

fn const_len(function: &MirFunction, def_map: &ValueDefMap, len_value: ValueId) -> Option<i64> {
    function.blocks.values().find_map(|block| {
        block.instructions.iter().find_map(|inst| {
            let (dst, sources) = match_dead_text_len_call(inst)?;
            if value_root(function, def_map, dst) != value_root(function, def_map, len_value) {
                return None;
            }
            sources.into_iter().find_map(|source| {
                const_text(function, def_map, source).map(|text| text.len() as i64)
            })
        })
    })
}

fn is_len_value_for_source(
    function: &MirFunction,
    def_map: &ValueDefMap,
    len_value: ValueId,
    source_value: ValueId,
) -> bool {
    let len_value = value_root(function, def_map, len_value);
    let source_value = value_root(function, def_map, source_value);
    function.blocks.values().any(|block| {
        block.instructions.iter().any(|inst| {
            let Some((dst, sources)) = match_dead_text_len_call(inst) else {
                return false;
            };
            value_root(function, def_map, dst) == len_value
                && sources
                    .into_iter()
                    .any(|source| value_root(function, def_map, source) == source_value)
        })
    })
}

fn match_exit_return(
    function: &MirFunction,
    def_map: &ValueDefMap,
    exit: &BasicBlock,
    exit_accumulator_value: ValueId,
    text_value: ValueId,
    accumulator_initial_const: i64,
    loop_bound_const: i64,
    accumulator_delta_const: i64,
    base_len_const: i64,
) -> Option<(ValueId, i64)> {
    let ret_value = match exit.terminator.as_ref()? {
        MirInstruction::Return { value: Some(value) } => value_root(function, def_map, *value),
        _ => return None,
    };
    let final_len_value = exit.instructions.iter().find_map(|inst| {
        let (dst, sources) = match_dead_text_len_call(inst)?;
        sources
            .into_iter()
            .any(|source| value_root(function, def_map, source) == text_value)
            .then_some(value_root(function, def_map, dst))
    })?;
    let matches_return = exit.instructions.iter().any(|inst| match inst {
        MirInstruction::BinOp {
            dst,
            op: BinaryOp::Add,
            lhs,
            rhs,
        } if value_root(function, def_map, *dst) == ret_value => {
            let lhs = value_root(function, def_map, *lhs);
            let rhs = value_root(function, def_map, *rhs);
            (lhs == exit_accumulator_value && rhs == final_len_value)
                || (rhs == exit_accumulator_value && lhs == final_len_value)
        }
        _ => false,
    });
    if !matches_return {
        return None;
    }
    Some((
        final_len_value,
        accumulator_initial_const + loop_bound_const * accumulator_delta_const + base_len_const,
    ))
}

fn text_content_observed_after_loop(
    function: &MirFunction,
    def_map: &ValueDefMap,
    exit: &BasicBlock,
    text_value: ValueId,
    final_len_value: ValueId,
) -> bool {
    exit.instructions
        .iter()
        .chain(exit.terminator.iter())
        .any(|inst| {
            if let MirInstruction::Copy { dst, src } = inst {
                let dst_is_text = value_root(function, def_map, *dst) == text_value;
                let src_is_text = value_root(function, def_map, *src) == text_value;
                let observed = (dst_is_text || src_is_text) && !(dst_is_text && src_is_text);
                return observed;
            }
            if let Some((dst, sources)) = match_dead_text_len_call(inst) {
                let reads_text = sources
                    .into_iter()
                    .any(|source| value_root(function, def_map, source) == text_value);
                let allowed_final_len =
                    reads_text && value_root(function, def_map, dst) == final_len_value;
                return !allowed_final_len;
            }
            let observed = inst
                .used_values()
                .into_iter()
                .any(|value| value_root(function, def_map, value) == text_value);
            observed
        })
}

fn block_successors(block: &BasicBlock) -> Vec<BasicBlockId> {
    match block.terminator.as_ref() {
        Some(MirInstruction::Jump { target, .. }) => vec![*target],
        Some(MirInstruction::Branch {
            then_bb, else_bb, ..
        }) => vec![*then_bb, *else_bb],
        _ => Vec::new(),
    }
}

fn match_dead_text_len_call(inst: &MirInstruction) -> Option<(ValueId, Vec<ValueId>)> {
    match inst {
        MirInstruction::Call {
            dst: Some(dst),
            callee:
                Some(Callee::Method {
                    method,
                    receiver: Some(receiver),
                    ..
                }),
            args,
            ..
        } if is_len_method_name(method) => {
            let mut values = vec![*receiver];
            values.extend(args.iter().copied());
            Some((*dst, values))
        }
        MirInstruction::Call {
            dst: Some(dst),
            callee: Some(Callee::Extern(name)),
            args,
            ..
        } if args.len() == 1 && is_runtime_len_handle_export(name) => {
            Some((*dst, args.iter().copied().collect::<Vec<_>>()))
        }
        MirInstruction::Call {
            dst: Some(dst),
            callee: Some(Callee::Global(name)),
            args,
            ..
        } if args.len() == 1 && is_lowered_len_global(name) => {
            Some((*dst, args.iter().copied().collect::<Vec<_>>()))
        }
        _ => None,
    }
}

fn const_i64(function: &MirFunction, def_map: &ValueDefMap, value: ValueId) -> Option<i64> {
    let value = value_root(function, def_map, value);
    let (block, index) = def_map.get(&value).copied()?;
    match function.blocks.get(&block)?.instructions.get(index)? {
        MirInstruction::Const {
            value: ConstValue::Integer(actual),
            ..
        } => Some(*actual),
        _ => None,
    }
}

fn const_text(function: &MirFunction, def_map: &ValueDefMap, value: ValueId) -> Option<String> {
    let value = value_root(function, def_map, value);
    let (block, index) = def_map.get(&value).copied()?;
    match function.blocks.get(&block)?.instructions.get(index)? {
        MirInstruction::Const {
            value: ConstValue::String(text),
            ..
        } => Some(text.clone()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::mir::MirCompiler;
    use crate::runner::modes::common_util::source_hint::prepare_source_minimal;
    use crate::NyashParser;

    fn ensure_ring0_initialized() {
        use crate::runtime::ring0::{default_ring0, init_global_ring0};
        let _ = std::panic::catch_unwind(|| {
            init_global_ring0(default_ring0());
        });
    }

    #[test]
    fn refresh_function_detects_substring_concat_dead_text_region() {
        ensure_ring0_initialized();
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/benchmarks/bench_kilo_micro_substring_concat.hako"
        );
        let source = std::fs::read_to_string(path).expect("benchmark source");
        let prepared = prepare_source_minimal(&source, path).expect("prepare benchmark source");
        let ast = NyashParser::parse_from_string(&prepared).expect("parse benchmark");
        let mut compiler = MirCompiler::with_options(true);
        let result = compiler
            .compile_with_source(ast, Some(path))
            .expect("compile benchmark");
        let main = result.module.functions.get("main").expect("main");
        let plans = &main.metadata.string_dead_text_region_plans;

        assert_eq!(plans.len(), 1);
        let plan = &plans[0];
        assert_ne!(plan.loop_header, plan.loop_body);
        assert_ne!(plan.loop_header, plan.loop_exit);
        assert_ne!(plan.loop_body, plan.loop_exit);
        assert_eq!(plan.loop_bound_const, 300000);
        assert_eq!(plan.base_len_const, 16);
        assert_eq!(plan.inserted_text, "xx");
        assert_eq!(plan.inserted_len_const, 2);
        assert_eq!(plan.accumulator_delta_const, 18);
        assert_eq!(plan.final_return_value, 5_400_016);
        assert_eq!(plan.publication_boundary, "none");
        assert!(!plan.final_text_content_observed);
        assert!(!plan.lowering_consumer_enabled);
    }
}
