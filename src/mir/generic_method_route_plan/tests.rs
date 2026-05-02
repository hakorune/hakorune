use super::*;
use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
use crate::mir::global_call_route_plan::{
    GlobalCallRoute, GlobalCallRouteSite, GlobalCallTargetFacts, GlobalCallTargetShape,
};
use crate::mir::{BasicBlock, BasicBlockId, EffectMask, FunctionSignature, MirType};

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

fn route_for<'a>(
    function: &'a MirFunction,
    box_name: &str,
    method: &str,
    result: Option<u32>,
) -> &'a GenericMethodRoute {
    let result_value = result.map(ValueId::new);
    function
            .metadata
            .generic_method_routes
            .iter()
            .find(|route| {
                route.box_name() == box_name
                    && route.method() == method
                    && route.result_value() == result_value
            })
            .unwrap_or_else(|| {
                panic!(
                    "missing generic method route box={box_name} method={method} result={result_value:?}"
                )
            })
}

mod core_routes;
mod map_set_routes;
mod scalar_proof;
mod string_routes;
mod tail;
