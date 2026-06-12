mod array_slot;
mod common;
mod guards;
mod hako_atomic;
mod hako_mem;
mod hako_osvm;
mod hako_tls;
mod hostbridge_stage1;
mod specs;

#[allow(unused_imports)]
pub(crate) use crate::mir::core_method_op::{LoweringPlanEmitKind, LoweringPlanTier};
#[allow(unused_imports)]
pub(crate) use crate::mir::extern_call_route_plan::{
    classify_extern_call_route, extern_call_route_specs, refresh_function_extern_call_routes,
};
#[allow(unused_imports)]
pub(crate) use crate::mir::{
    BasicBlock, BasicBlockId, Callee, ConstValue, EffectMask, FunctionSignature, MirFunction,
    MirInstruction, MirType, ValueId,
};
#[allow(unused_imports)]
pub(crate) use common::make_function_with_call;
