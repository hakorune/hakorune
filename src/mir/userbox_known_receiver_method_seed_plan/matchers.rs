use std::collections::BTreeMap;

use super::{
    BasicBlockId, BinaryOp, Callee, ConstValue, MirFunction, MirInstruction, ThinEntrySurface,
    UserBoxKnownReceiverMethodSeedKind, UserBoxKnownReceiverMethodSeedPayload,
    UserBoxKnownReceiverMethodSeedRoute, ValueId,
};

mod counter;
mod point;
mod shared;

pub(super) fn match_userbox_known_receiver_method_seed_route(
    function: &MirFunction,
    functions: &BTreeMap<String, MirFunction>,
) -> Option<UserBoxKnownReceiverMethodSeedRoute> {
    counter::match_counter_seed_route(function, functions)
        .or_else(|| point::match_point_seed_route(function, functions))
}
