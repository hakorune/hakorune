use super::generic_string_reject::GenericPureStringReject;
use super::model::GlobalCallReturnContract;
use super::shape_blocker::propagated_unknown_global_target_blocker;
use super::{
    lookup_global_call_target, GlobalCallShapeBlocker, GlobalCallTargetFacts,
    GlobalCallTargetShape, GlobalCallTargetShapeReason,
};
use crate::mir::extern_call_route_plan::{
    classify_extern_call_route, is_hostbridge_extern_invoke_symbol, ExternCallRouteKind,
};
use crate::mir::same_module_body_shape::supported_backend_global;
use crate::mir::string_corridor::StringCorridorOp;
use crate::mir::{BinaryOp, Callee, ConstValue, MirFunction, MirInstruction, MirType, ValueId};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

// Return-profile evidence only: this must not make the target lowerable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GenericStringReturnValueClass {
    Unknown,
    Void,
    String,
    StringOrVoid,
    Object,
    Other,
}

pub(super) struct GenericStringReturnProfile {
    values: BTreeMap<ValueId, GenericStringReturnValueClass>,
    passthrough: BTreeSet<ValueId>,
    blockers: BTreeMap<ValueId, GlobalCallShapeBlocker>,
}

#[derive(Default)]
pub(super) struct GenericStringReturnProfileCache {
    profiles: BTreeMap<String, GenericStringReturnProfile>,
}

include!("string_return_profile_cache.inc");
include!("string_return_profile_analysis.inc");
include!("string_return_profile_query.inc");
