//! Physical progress of the stores a named-local Map receives after its
//! construction, and the Map value a later checked read names. A store is an entry of the Map's own `MapHomeFlow` row, so its
//! emitted blocks join that same Map progress row; no second ledger exists.
use super::map::MapProgress;
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::{MapHomeObservation, MapLocalStoreV1};

impl OrdinaryNewClaimLedgerV1 {
    /// The sealed store at this exact call site and the site of the Map row
    /// that owns it, if any.
    pub(crate) fn map_local_store_at_v1(
        &self,
        site: &OwnedExprSiteV1,
    ) -> Result<Option<(MapLocalStoreV1, OwnedExprSiteV1)>, String> {
        let Some(flow) = self
            .completion_for_owner(site.owner())
            .and_then(|completion| completion.cleanup().root_flow())
        else {
            return Ok(None);
        };
        let mut found = None;
        for map in flow.maps().iter().filter_map(MapHomeObservation::complete) {
            for store in map.stores().iter().filter(|store| store.call() == site) {
                if found.replace((store.clone(), map.site().clone())).is_some() {
                    return Err(freeze("map-store-duplicate"));
                }
            }
        }
        Ok(found)
    }

    /// The emitted Map value a later store installs into. It is the value the
    /// constructing emission produced; no read or copy is made.
    pub(crate) fn map_local_store_map_value_v1(
        &self,
        map_site: &OwnedExprSiteV1,
    ) -> Result<ValueId, String> {
        match self.local_commits.borrow().get(map_site) {
            Some(LocalCommitV1::Map(row)) => row
                .emitted_value()
                .ok_or_else(|| freeze("map-store-map-not-emitted")),
            _ => Err(freeze("map-store-map-missing")),
        }
    }

    pub(crate) fn record_map_local_store_v1(
        &self,
        map_site: &OwnedExprSiteV1,
        blocks: Vec<(BasicBlockId, MirInstruction)>,
    ) -> Result<(), String> {
        let mut rows = self.local_commits.borrow_mut();
        let Some(LocalCommitV1::Map(row)) = rows.get_mut(map_site) else {
            return Err(freeze("map-store-record-without-map"));
        };
        match &mut row.progress {
            MapProgress::Emitted { bindings, .. } if !blocks.is_empty() => {
                bindings.extend(blocks);
                Ok(())
            }
            _ => Err(freeze("map-store-record-state")),
        }
    }

    /// The Map value a checked `get` expression reads: the original receiver
    /// read must name the local's own constructing emission, nothing else.
    pub(crate) fn local_map_receiver_read_v1(
        &self,
        core: &crate::mir::normal_callable_semantic_package::ExpressionCoreSourceLoanV1,
        read: &crate::mir::builder::ExactLexicalReadV1,
    ) -> Result<ValueId, String> {
        let binding = core
            .local_map()
            .ok_or_else(|| freeze("map-read-receiver-source"))?;
        let site = core.source().source().receiver_site();
        let value = read
            .value_for(core.source().source().owner(), site.node(), binding)
            .map_err(|error| error.to_string())?;
        let rows = self.local_commits.borrow();
        let mut maps = rows.values().filter_map(|row| match row {
            LocalCommitV1::Map(map) if map.binding == Some(binding) => map.emitted_value(),
            _ => None,
        });
        match (maps.next(), maps.next()) {
            (Some(emitted), None) if emitted == value => Ok(value),
            _ => Err(freeze("map-read-receiver-drift")),
        }
    }
}
