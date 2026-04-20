/*!
 * MIR-owned array/text residence session plans.
 *
 * H25 keeps this metadata-only: it proves where a future backend may hold an
 * array text residence session, but it does not change lowering or runtime
 * behavior. Runtime remains executor-only; legality lives here.
 */

use std::collections::HashSet;

use super::{
    array_text_loopcarry_plan::ArrayTextLoopCarryLenStoreRoute, BasicBlockId, MirFunction,
    MirInstruction, MirModule, ValueId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayTextResidenceSessionScope {
    LoopBackedgeSingleBody,
}

impl std::fmt::Display for ArrayTextResidenceSessionScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LoopBackedgeSingleBody => f.write_str("loop_backedge_single_body"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayTextResidenceSessionProof {
    LoopcarryLenStoreOnly,
}

impl std::fmt::Display for ArrayTextResidenceSessionProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LoopcarryLenStoreOnly => f.write_str("loopcarry_len_store_only"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayTextResidenceSessionRoute {
    pub header_block: BasicBlockId,
    pub body_block: BasicBlockId,
    pub exit_block: BasicBlockId,
    pub route_instruction_index: usize,
    pub array_value: ValueId,
    pub index_value: ValueId,
    pub source_value: ValueId,
    pub result_len_value: ValueId,
    pub middle_value: ValueId,
    pub middle_length: i64,
    pub scope: ArrayTextResidenceSessionScope,
    pub proof: ArrayTextResidenceSessionProof,
}

pub fn refresh_module_array_text_residence_session_routes(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_array_text_residence_session_routes(function);
    }
}

pub fn refresh_function_array_text_residence_session_routes(function: &mut MirFunction) {
    let routes = function
        .metadata
        .array_text_loopcarry_len_store_routes
        .iter()
        .filter_map(|route| derive_loopcarry_session(function, route))
        .collect();
    function.metadata.array_text_residence_sessions = routes;
}

fn derive_loopcarry_session(
    function: &MirFunction,
    route: &ArrayTextLoopCarryLenStoreRoute,
) -> Option<ArrayTextResidenceSessionRoute> {
    if count_routes_in_body(function, route.block) != 1 {
        return None;
    }

    let body = function.blocks.get(&route.block)?;
    let header_block = match body.terminator.as_ref()? {
        MirInstruction::Jump { target, .. } => *target,
        _ => return None,
    };

    let header = function.blocks.get(&header_block)?;
    let exit_block = match header.terminator.as_ref()? {
        MirInstruction::Branch {
            then_bb, else_bb, ..
        } if *then_bb == route.block => *else_bb,
        MirInstruction::Branch {
            then_bb, else_bb, ..
        } if *else_bb == route.block => *then_bb,
        _ => return None,
    };

    if !body_has_only_covered_route_and_pure_loop_bookkeeping(body, route) {
        return None;
    }

    Some(ArrayTextResidenceSessionRoute {
        header_block,
        body_block: route.block,
        exit_block,
        route_instruction_index: route.instruction_index,
        array_value: route.array_value,
        index_value: route.index_value,
        source_value: route.source_value,
        result_len_value: route.result_len_value,
        middle_value: route.middle_value,
        middle_length: route.middle_length,
        scope: ArrayTextResidenceSessionScope::LoopBackedgeSingleBody,
        proof: ArrayTextResidenceSessionProof::LoopcarryLenStoreOnly,
    })
}

fn count_routes_in_body(function: &MirFunction, body_block: BasicBlockId) -> usize {
    function
        .metadata
        .array_text_loopcarry_len_store_routes
        .iter()
        .filter(|route| route.block == body_block)
        .count()
}

fn body_has_only_covered_route_and_pure_loop_bookkeeping(
    body: &super::BasicBlock,
    route: &ArrayTextLoopCarryLenStoreRoute,
) -> bool {
    let covered: HashSet<usize> = std::iter::once(route.instruction_index)
        .chain(route.skip_instruction_indices.iter().copied())
        .collect();

    body.instructions
        .iter()
        .enumerate()
        .all(|(index, inst)| covered.contains(&index) || is_session_safe_bookkeeping(inst))
}

fn is_session_safe_bookkeeping(inst: &MirInstruction) -> bool {
    matches!(
        inst,
        MirInstruction::Const { .. }
            | MirInstruction::Copy { .. }
            | MirInstruction::BinOp { .. }
            | MirInstruction::Compare { .. }
    )
}
