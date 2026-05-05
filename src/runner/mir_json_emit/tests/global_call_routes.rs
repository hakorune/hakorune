use super::super::build_mir_json_root;
use super::make_function;
use crate::mir::global_call_route_plan::{
    refresh_function_global_call_routes, refresh_module_global_call_routes, GlobalCallRoute,
    GlobalCallRouteSite, GlobalCallTargetFacts,
};
use crate::mir::{
    BasicBlock, BasicBlockId, BinaryOp, Callee, CompareOp, ConstValue, EffectMask,
    FunctionSignature, MirFunction, MirInstruction, MirType, ValueId,
};

mod blockers;
mod box_type_inspector_describe;
mod core;
mod generic_i64;
mod parser_program_json;
mod pattern_util_local_value_probe;
mod static_string_array;
mod tail;
mod void_logging;
mod void_sentinel;
