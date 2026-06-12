use super::super::make_function_with_global_call_args;
use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
use crate::mir::global_call_route_plan::refresh_module_global_call_routes;
use crate::mir::{
    BasicBlock, BasicBlockId, BinaryOp, Callee, CompareOp, ConstValue, EffectMask,
    FunctionSignature, MirFunction, MirInstruction, MirModule, MirType, ValueId,
};

mod loop_scalar_phi_substring_void_sentinel_body;
mod mixed_param_substring_void_sentinel_body;
mod scalar_void_guard_string_or_void_body;
mod unknown_param_or_void_sentinel_body;
mod unknown_return_void_sentinel_body;
mod unknown_wrapper_string_or_void_child;
