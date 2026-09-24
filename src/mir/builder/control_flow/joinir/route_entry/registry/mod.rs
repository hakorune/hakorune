//! M10b-I0-R0: the ordered recipe-first route scheduler has been retired.
//!
//! The frozen `route_loop` entry now runs one fixed order — located source
//! -> exact membership -> one policy winner -> verified recipe -> one
//! physical admission -> one canonical physicalizer — and this module keeps
//! only the shared `route_id` compatibility facade for retained consumers.

/// Compatibility facade; producer identity now lives beside the portable
/// artifact and remains non-semantic provenance.
#[allow(dead_code)]
pub(crate) mod route_id {
    pub(crate) use crate::mir::loop_recipe_contract::route_id::{entry_keys, LoopRouteId};
}

pub(crate) mod predicates;
