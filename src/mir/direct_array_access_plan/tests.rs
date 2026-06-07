use super::*;
use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
use crate::mir::function::{DirectArrayExtentProofKind, LoopRangeFact};
use crate::mir::range_index_fact::refresh_function_range_index_facts;
use crate::mir::{
    BasicBlock, BasicBlockId, BinaryOp, Callee, CompareOp, ConstValue, EffectMask,
    FunctionSignature, MirFunction, MirInstruction, MirType,
};

mod caller_precondition;
mod exact_front;
mod range_index;
mod stack_top_pop;

fn make_function() -> MirFunction {
    make_named_function("main", vec![])
}

fn make_named_function(name: &str, params: Vec<MirType>) -> MirFunction {
    MirFunction::new(
        FunctionSignature {
            name: name.to_string(),
            params,
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    )
}

fn method_call(
    dst: Option<u32>,
    box_name: &str,
    method: &str,
    receiver: u32,
    args: Vec<u32>,
) -> MirInstruction {
    MirInstruction::Call {
        dst: dst.map(ValueId::new),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: box_name.to_string(),
            method: method.to_string(),
            receiver: Some(ValueId::new(receiver)),
            certainty: TypeCertainty::Known,
            box_kind: CalleeBoxKind::RuntimeData,
        }),
        args: args.into_iter().map(ValueId::new).collect(),
        effects: EffectMask::PURE,
    }
}

fn refresh_direct_array_plans(function: &mut MirFunction) {
    crate::mir::generic_method_route_plan::refresh_function_generic_method_routes(function);
    refresh_function_range_index_facts(function);
    refresh_function_direct_array_access_plans(function);
}

fn add_unit_range_loop_fact(function: &mut MirFunction, body_bb: BasicBlockId) {
    function.metadata.loop_range_facts.push(LoopRangeFact {
        index_name: "i".to_string(),
        start_value: ValueId::new(10),
        end_value: ValueId::new(11),
        index_phi: ValueId::new(4),
        preheader_bb: BasicBlockId::new(0),
        header_bb: BasicBlockId::new(2),
        body_bb,
        step_bb: BasicBlockId::new(3),
        exit_bb: BasicBlockId::new(4),
        step: 1,
        end_exclusive: true,
        index_read_only: true,
        body_local_writes_supported: true,
        loop_carried_writes_supported: false,
        body_writes_supported: false,
    });
}
