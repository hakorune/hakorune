/*!
 * MIR-owned route plan for temporary UserBox known-receiver method exact seeds.
 *
 * Thin-entry metadata already proves the known receiver method surface and the
 * primitive field lanes. This module binds the local/copy `Counter.step/1` and
 * `Point.sum/1` exact seed shells to a backend route so the C boundary can
 * validate metadata and emit without rescanning raw MIR JSON.
 */

mod ir_match;
mod main_facts;
mod matchers;
mod model;

#[allow(unused_imports)]
pub(crate) use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
#[allow(unused_imports)]
pub(crate) use crate::mir::thin_entry::{
    ThinEntryCurrentCarrier, ThinEntryDemand, ThinEntryPreferredEntry, ThinEntrySurface,
    ThinEntryValueClass,
};
#[allow(unused_imports)]
pub(crate) use crate::mir::thin_entry_selection::{ThinEntrySelection, ThinEntrySelectionState};
#[allow(unused_imports)]
pub(crate) use crate::mir::{
    BasicBlockId, BinaryOp, Callee, ConstValue, EffectMask, FunctionSignature, MirFunction,
    MirInstruction, MirModule, MirType, ValueId,
};

pub use self::model::{
    UserBoxKnownReceiverMethodSeedKind, UserBoxKnownReceiverMethodSeedPayload,
    UserBoxKnownReceiverMethodSeedRoute,
};

use self::matchers::match_userbox_known_receiver_method_seed_route;

pub fn refresh_module_userbox_known_receiver_method_seed_routes(module: &mut super::MirModule) {
    let routes: Vec<(String, Option<UserBoxKnownReceiverMethodSeedRoute>)> = module
        .functions
        .iter()
        .map(|(name, function)| {
            (
                name.clone(),
                match_userbox_known_receiver_method_seed_route(function, &module.functions),
            )
        })
        .collect();

    for (name, route) in routes {
        if let Some(function) = module.functions.get_mut(&name) {
            function.metadata.userbox_known_receiver_method_seed_route = route;
        }
    }
}

#[cfg(test)]
pub(crate) mod test_support;
#[cfg(test)]
mod tests;
