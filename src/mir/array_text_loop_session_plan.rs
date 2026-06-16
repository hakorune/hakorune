//! MIR-owned proof surface for array text loop-session lowering.
//!
//! This module proves only metadata. It is exported to MIR JSON for inspection,
//! but backend routes and runtime loop-session behavior remain disabled until a
//! later row explicitly consumes `ArrayTextLoopSessionPlan`.

use super::array_receiver_proof::{match_array_set_call, same_value_root};
use super::value_origin::build_value_def_map;
use super::{BasicBlockId, MirFunction, MirInstruction, MirModule, ValueId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayTextLoopSessionRejectReason {
    DifferentArrayHandle,
    UnknownLoopRegion,
    ArrayMutationInRegion,
    DropOrPublicationBoundary,
    IndexDomainUnproven,
}

impl ArrayTextLoopSessionRejectReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DifferentArrayHandle => "different_array_handle",
            Self::UnknownLoopRegion => "unknown_loop_region",
            Self::ArrayMutationInRegion => "array_mutation_in_region",
            Self::DropOrPublicationBoundary => "drop_or_publication_boundary",
            Self::IndexDomainUnproven => "index_domain_unproven",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayTextLoopSessionPlan {
    loop_header: BasicBlockId,
    loop_exit: BasicBlockId,
    array_value: ValueId,
    index_value: ValueId,
    len_call_count: usize,
    same_array_handle: bool,
    read_only_region: bool,
    no_mutation_region: bool,
    no_drop_or_publication_boundary: bool,
    index_domain_guarded: bool,
}

impl ArrayTextLoopSessionPlan {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        loop_header: BasicBlockId,
        loop_exit: BasicBlockId,
        array_value: ValueId,
        index_value: ValueId,
        len_call_count: usize,
        same_array_handle: bool,
        read_only_region: bool,
        no_mutation_region: bool,
        no_drop_or_publication_boundary: bool,
        index_domain_guarded: bool,
    ) -> Self {
        Self {
            loop_header,
            loop_exit,
            array_value,
            index_value,
            len_call_count,
            same_array_handle,
            read_only_region,
            no_mutation_region,
            no_drop_or_publication_boundary,
            index_domain_guarded,
        }
    }

    pub fn loop_header(&self) -> BasicBlockId {
        self.loop_header
    }

    pub fn loop_exit(&self) -> BasicBlockId {
        self.loop_exit
    }

    pub fn array_value(&self) -> ValueId {
        self.array_value
    }

    pub fn index_value(&self) -> ValueId {
        self.index_value
    }

    pub fn len_call_count(&self) -> usize {
        self.len_call_count
    }

    pub fn same_array_handle(&self) -> bool {
        self.same_array_handle
    }

    pub fn read_only_region(&self) -> bool {
        self.read_only_region
    }

    pub fn no_mutation_region(&self) -> bool {
        self.no_mutation_region
    }

    pub fn no_drop_or_publication_boundary(&self) -> bool {
        self.no_drop_or_publication_boundary
    }

    pub fn index_domain_guarded(&self) -> bool {
        self.index_domain_guarded
    }

    pub fn backend_session_lowering_allowed(&self) -> bool {
        self.len_call_count > 0
            && self.same_array_handle
            && self.read_only_region
            && self.no_mutation_region
            && self.no_drop_or_publication_boundary
            && self.index_domain_guarded
    }

    pub fn first_reject_reason(&self) -> Option<ArrayTextLoopSessionRejectReason> {
        if !self.same_array_handle {
            return Some(ArrayTextLoopSessionRejectReason::DifferentArrayHandle);
        }
        if !self.read_only_region {
            return Some(ArrayTextLoopSessionRejectReason::UnknownLoopRegion);
        }
        if !self.no_mutation_region {
            return Some(ArrayTextLoopSessionRejectReason::ArrayMutationInRegion);
        }
        if !self.no_drop_or_publication_boundary {
            return Some(ArrayTextLoopSessionRejectReason::DropOrPublicationBoundary);
        }
        if !self.index_domain_guarded {
            return Some(ArrayTextLoopSessionRejectReason::IndexDomainUnproven);
        }
        None
    }
}

pub fn refresh_module_array_text_loop_session_plans(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_array_text_loop_session_plans(function);
    }
}

pub fn refresh_function_array_text_loop_session_plans(function: &mut MirFunction) {
    let def_map = build_value_def_map(function);
    let mut plans = Vec::new();

    for route in &function.metadata.array_string_len_window_routes {
        let Some((loop_header, loop_exit)) = match_single_body_loop(function, route.block()) else {
            continue;
        };
        let Some(body_block) = function.blocks.get(&route.block()) else {
            continue;
        };
        let same_array_handle = true;
        let read_only_region = true;
        let no_mutation_region =
            !body_mutates_array(function, &def_map, body_block, route.array_value());
        let no_drop_or_publication_boundary = !body_publishes_array(
            function,
            &def_map,
            body_block,
            route.array_value(),
            route.instruction_index(),
            route.len_instruction_index(),
        );
        let index_domain_guarded =
            function.metadata.range_index_facts.iter().any(|fact| {
                fact.body_bb == route.block() && fact.index_value == route.index_value()
            });

        let plan = ArrayTextLoopSessionPlan::new(
            loop_header,
            loop_exit,
            route.array_value(),
            route.index_value(),
            1,
            same_array_handle,
            read_only_region,
            no_mutation_region,
            no_drop_or_publication_boundary,
            index_domain_guarded,
        );
        if plan.backend_session_lowering_allowed() {
            plans.push(plan);
        }
    }

    plans.sort_by_key(|plan| (plan.loop_header.as_u32(), plan.array_value.as_u32()));
    function.metadata.array_text_loop_session_plans = plans;
}

fn match_single_body_loop(
    function: &MirFunction,
    body_block: BasicBlockId,
) -> Option<(BasicBlockId, BasicBlockId)> {
    let body = function.blocks.get(&body_block)?;
    let loop_header = match body.terminator.as_ref()? {
        MirInstruction::Jump { target, .. } => *target,
        _ => return None,
    };
    let header = function.blocks.get(&loop_header)?;
    match header.terminator.as_ref()? {
        MirInstruction::Branch {
            then_bb, else_bb, ..
        } if *then_bb == body_block => Some((loop_header, *else_bb)),
        MirInstruction::Branch {
            then_bb, else_bb, ..
        } if *else_bb == body_block => Some((loop_header, *then_bb)),
        _ => None,
    }
}

fn body_mutates_array(
    function: &MirFunction,
    def_map: &super::value_origin::ValueDefMap,
    body: &super::BasicBlock,
    array_value: ValueId,
) -> bool {
    body.instructions.iter().any(|inst| {
        match_array_set_call(inst).is_some_and(|set_call| {
            same_value_root(function, def_map, set_call.array_value, array_value)
        })
    })
}

fn body_publishes_array(
    function: &MirFunction,
    def_map: &super::value_origin::ValueDefMap,
    body: &super::BasicBlock,
    array_value: ValueId,
    allowed_get_index: usize,
    allowed_len_index: usize,
) -> bool {
    body.instructions
        .iter()
        .enumerate()
        .any(|(instruction_index, inst)| {
            if instruction_index == allowed_get_index || instruction_index == allowed_len_index {
                return false;
            }
            instruction_mentions_value_root(function, def_map, inst, array_value)
        })
}

fn instruction_mentions_value_root(
    function: &MirFunction,
    def_map: &super::value_origin::ValueDefMap,
    inst: &MirInstruction,
    target: ValueId,
) -> bool {
    if matches!(inst, MirInstruction::Copy { .. }) {
        return false;
    }
    let target_root = super::array_receiver_proof::value_root(function, def_map, target);
    inst.used_values().into_iter().any(|value| {
        super::array_receiver_proof::value_root(function, def_map, value) == target_root
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::array_string_len_window_plan::refresh_function_array_string_len_window_routes;
    use crate::mir::function::{FunctionSignature, RangeIndexFact, RangeIndexFactOriginKind};
    use crate::mir::{BasicBlock, Callee, EffectMask, MirType};
    use hakorune_mir_defs::{CalleeBoxKind, TypeCertainty};

    fn plan_with_flags(
        same_array_handle: bool,
        read_only_region: bool,
        no_mutation_region: bool,
        no_drop_or_publication_boundary: bool,
        index_domain_guarded: bool,
    ) -> ArrayTextLoopSessionPlan {
        ArrayTextLoopSessionPlan::new(
            BasicBlockId::new(10),
            BasicBlockId::new(20),
            ValueId::new(1),
            ValueId::new(2),
            3,
            same_array_handle,
            read_only_region,
            no_mutation_region,
            no_drop_or_publication_boundary,
            index_domain_guarded,
        )
    }

    #[test]
    fn complete_plan_allows_backend_session_lowering() {
        let plan = plan_with_flags(true, true, true, true, true);
        assert!(plan.backend_session_lowering_allowed());
        assert_eq!(plan.first_reject_reason(), None);
    }

    #[test]
    fn mutation_rejects_backend_session_lowering() {
        let plan = plan_with_flags(true, true, false, true, true);
        assert!(!plan.backend_session_lowering_allowed());
        assert_eq!(
            plan.first_reject_reason(),
            Some(ArrayTextLoopSessionRejectReason::ArrayMutationInRegion)
        );
    }

    #[test]
    fn unguarded_index_rejects_backend_session_lowering() {
        let plan = plan_with_flags(true, true, true, true, false);
        assert!(!plan.backend_session_lowering_allowed());
        assert_eq!(
            plan.first_reject_reason(),
            Some(ArrayTextLoopSessionRejectReason::IndexDomainUnproven)
        );
    }

    fn test_function_with_array_get_len_loop() -> MirFunction {
        let signature = FunctionSignature {
            name: "test".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId::new(0));

        let header_id = BasicBlockId::new(1);
        let body_id = BasicBlockId::new(2);
        let exit_id = BasicBlockId::new(3);
        let mut header = BasicBlock::new(header_id);
        header.add_instruction(MirInstruction::Branch {
            condition: ValueId::new(10),
            then_bb: body_id,
            else_bb: exit_id,
            then_edge_args: None,
            else_edge_args: None,
        });

        let mut body = BasicBlock::new(body_id);
        body.add_instruction(MirInstruction::Call {
            dst: Some(ValueId::new(30)),
            func: ValueId::new(0),
            callee: Some(Callee::Method {
                box_name: "ArrayBox".to_string(),
                method: "get".to_string(),
                receiver: Some(ValueId::new(20)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(21)],
            effects: EffectMask::PURE,
        });
        body.add_instruction(MirInstruction::Call {
            dst: Some(ValueId::new(31)),
            func: ValueId::new(0),
            callee: Some(Callee::Method {
                box_name: "StringBox".to_string(),
                method: "length".to_string(),
                receiver: Some(ValueId::new(30)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![],
            effects: EffectMask::PURE,
        });
        body.add_instruction(MirInstruction::Jump {
            target: header_id,
            edge_args: None,
        });

        let mut exit = BasicBlock::new(exit_id);
        exit.add_instruction(MirInstruction::Return {
            value: Some(ValueId::new(31)),
        });

        function.blocks.insert(header_id, header);
        function.blocks.insert(body_id, body);
        function.blocks.insert(exit_id, exit);
        function
    }

    fn add_index_fact(function: &mut MirFunction) {
        function.metadata.range_index_facts.push(RangeIndexFact {
            fact_id: 0,
            origin_kind: RangeIndexFactOriginKind::ModuloOfRangeIndex,
            index_value: ValueId::new(21),
            lower_value: ValueId::new(40),
            upper_exclusive_value: ValueId::new(41),
            body_bb: BasicBlockId::new(2),
            step: 0,
            end_exclusive: true,
            index_body_read_only: true,
            loop_carried_writes_supported: false,
        });
    }

    #[test]
    fn producer_builds_plan_from_window_route_and_index_fact() {
        let mut function = test_function_with_array_get_len_loop();
        refresh_function_array_string_len_window_routes(&mut function);
        add_index_fact(&mut function);

        refresh_function_array_text_loop_session_plans(&mut function);

        assert_eq!(function.metadata.array_text_loop_session_plans.len(), 1);
        let plan = &function.metadata.array_text_loop_session_plans[0];
        assert_eq!(plan.loop_header(), BasicBlockId::new(1));
        assert_eq!(plan.loop_exit(), BasicBlockId::new(3));
        assert_eq!(plan.array_value(), ValueId::new(20));
        assert_eq!(plan.index_value(), ValueId::new(21));
        assert!(plan.backend_session_lowering_allowed());
    }

    #[test]
    fn producer_rejects_window_without_index_fact() {
        let mut function = test_function_with_array_get_len_loop();
        refresh_function_array_string_len_window_routes(&mut function);

        refresh_function_array_text_loop_session_plans(&mut function);

        assert!(function.metadata.array_text_loop_session_plans.is_empty());
    }
}
