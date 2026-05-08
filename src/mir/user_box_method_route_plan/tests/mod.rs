use super::*;
use crate::mir::definitions::call_unified::TypeCertainty;
use crate::mir::function::{TypedObjectFieldPlan, TypedObjectFieldStorage, TypedObjectPlan};
use crate::mir::generic_method_route_plan::test_support::{
    array_push, runtime_data_map_get_mixed_i64_key_with_result_origin_box,
};
use crate::mir::{BasicBlock, BinaryOp, ConstValue, EffectMask, FunctionSignature, MirInstruction};

mod birth;
mod method_targets;
mod object_handles;
mod param_and_field_origins;
mod receiver_origins;
