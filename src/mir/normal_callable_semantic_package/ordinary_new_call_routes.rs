//! Which executor owns a statement-level Call, and which writer routed it.
//!
//! A site is routed to the lifecycle lane by exactly one writer; a second
//! write is a contract error. A later stage of the same lane names the stage it
//! continues (`route_local_after`) instead of writing again: the origin stays
//! the single writer. A root terminal packet may take the Invoke over from the
//! lane once; the site then stays claimed.
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, OwnedExprSiteV1};
use std::collections::BTreeMap;

/// The pipeline stage that decided a Call takes the lifecycle lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LocalRouteWriterV1 {
    UnitResult,
    LexicalInstance,
    DirectStatic,
    DirectLifecycle,
    ObjectSlot,
    StaticPacket,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum CallRouteV1 {
    LocalLifecycle(LocalRouteWriterV1),
    RootPacket(LocalRouteWriterV1),
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct CallRouteTableV1 {
    routes: BTreeMap<OwnedExprSiteV1, CallRouteV1>,
}

impl CallRouteTableV1 {
    pub(crate) fn route_local(
        &mut self,
        site: OwnedExprSiteV1,
        writer: LocalRouteWriterV1,
    ) -> Result<(), String> {
        match self.routes.entry(site) {
            std::collections::btree_map::Entry::Vacant(slot) => {
                slot.insert(CallRouteV1::LocalLifecycle(writer));
                Ok(())
            }
            std::collections::btree_map::Entry::Occupied(_) => {
                Err("[freeze:contract][call-route/routed-twice]".into())
            }
        }
    }

    /// A later stage of the same lane: the site is either not routed yet (this
    /// stage is its writer) or was routed by exactly the stage it continues.
    pub(crate) fn route_local_after(
        &mut self,
        site: OwnedExprSiteV1,
        writer: LocalRouteWriterV1,
        after: LocalRouteWriterV1,
    ) -> Result<(), String> {
        match self.routes.get(&site) {
            Some(CallRouteV1::LocalLifecycle(origin)) if *origin == after => Ok(()),
            _ => self.route_local(site, writer),
        }
    }

    pub(crate) fn is_local(&self, site: &OwnedExprSiteV1) -> bool {
        matches!(self.routes.get(site), Some(CallRouteV1::LocalLifecycle(_)))
    }

    pub(crate) fn local_sites(&self) -> impl Iterator<Item = &OwnedExprSiteV1> {
        self.routes
            .iter()
            .filter(|(_, route)| matches!(route, CallRouteV1::LocalLifecycle(_)))
            .map(|(site, _)| site)
    }

    pub(crate) fn local_sites_for(&self, owner: FunctionOwnerIdV1) -> Vec<OwnedExprSiteV1> {
        self.local_sites().filter(|site| site.owner() == owner).cloned().collect()
    }

    /// The root packet takes this site's Invoke from the lifecycle lane.
    pub(crate) fn hand_to_root_packet(&mut self, site: &OwnedExprSiteV1) -> Result<(), String> {
        match self.routes.get_mut(site) {
            Some(route @ CallRouteV1::LocalLifecycle(_)) => {
                let CallRouteV1::LocalLifecycle(writer) = *route else {
                    unreachable!()
                };
                *route = CallRouteV1::RootPacket(writer);
                Ok(())
            }
            _ => Err("[freeze:contract][call-route/root-handoff-without-local-route]".into()),
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.routes.is_empty()
    }

    pub(crate) fn clear(&mut self) {
        self.routes.clear();
    }

    #[cfg(test)]
    pub(crate) fn remove_for_test(&mut self, site: &OwnedExprSiteV1) {
        self.routes.remove(site);
    }
}
