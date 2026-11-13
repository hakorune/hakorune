//! A statement `m.set("<key literal>", <i64 value>)` on a live named-local Map.
//!
//! The store is one more entry of the same `MapHomeFlow` row that constructed
//! the Map, so the Map keeps one owner. Only the entry source is recorded here:
//! the Homes live at the statement (for the Fault cleanup order) and the entry
//! itself. No physical operation, ordinal or runtime policy is issued.
use super::*;
use crate::mir::core_method_result_kind::{lookup_core_method_result_row_v2, CoreMethodSemanticLawV2};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MapLocalStoreV1 {
    statement: SourceStmtSiteV1,
    call: OwnedExprSiteV1,
    receiver: SourceExprSiteV1,
    binding: BindingRefV1,
    entry: MapHomeEntry,
    homes: Box<[LiveHomeV1]>,
}

impl MapLocalStoreV1 {
    pub(crate) fn statement(&self) -> &SourceStmtSiteV1 {
        &self.statement
    }
    pub(crate) fn call(&self) -> &OwnedExprSiteV1 {
        &self.call
    }
    /// The original receiver read and the Map local it names.
    pub(crate) fn receiver(&self) -> (&SourceExprSiteV1, BindingRefV1) {
        (&self.receiver, self.binding)
    }
    pub(crate) fn entry(&self) -> &MapHomeEntry {
        &self.entry
    }
    /// Homes live at the statement in acquisition order, the Map included.
    pub(crate) fn homes(&self) -> &[LiveHomeV1] {
        &self.homes
    }
}

impl MapHomeFlow {
    /// Constructor entries first, then each later store in source order.
    pub(crate) fn all_entries(&self) -> impl Iterator<Item = &MapHomeEntry> + '_ {
        self.entries
            .iter()
            .chain(self.stores.iter().map(MapLocalStoreV1::entry))
    }
    pub(crate) fn stores(&self) -> &[MapLocalStoreV1] {
        &self.stores
    }
}

/// The row's law, not the spelling, decides that this call replaces or inserts
/// one Text-keyed entry. Anything else stays uncovered for the caller.
pub(in crate::mir::resolved_semantics::home_new_prefix) fn observe_local_store_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &crate::mir::compiler::located::LocatedStmtV1<'_>,
    locals: &PrefixLocalFlow<'_>,
    homes: &HomesV1,
    maps: &mut [MapHomeObservation],
) -> bool {
    if !matches!(statement.node(), ASTNode::MethodCall { .. }) {
        return false;
    }
    let site = SourceExprSiteV1::from_node(statement.site().node().clone());
    let Some(row) = input
        .function()
        .method_calls()
        .find(|(call_site, _)| **call_site == site)
        .map(|(_, row)| row)
    else {
        return false;
    };
    let ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(binding)) =
        row.receiver()
    else {
        return false;
    };
    let Some(contract) = lookup_core_method_result_row_v2("MapBox", row.selector(), row.arity())
    else {
        return false;
    };
    if row.owner() != input.owner()
        || !contract.semantic_law.iter().any(|(arity, law)| {
            *arity == row.arity() && *law == CoreMethodSemanticLawV2::MapTextSetReplaceOrInsert
        })
        || !locals.is_map_local(binding)
    {
        return false;
    }
    let [key, value] = row.arguments() else {
        return false;
    };
    let Some(ResolvedLiteralSourceV1::String(key)) =
        input.function().expression_source().literal(key.site())
    else {
        return false;
    };
    let Some(source) = map_value_leaf(input, locals, value.site())
        .filter(|source| source.scalar_kind() == Some(SourceScalarKind::Integer))
    else {
        return false;
    };
    let live = homes.clone().into_boxed_slice();
    if !live.iter().any(|home| home.binding() == binding) {
        return false;
    }
    let Some(flow) = maps.iter_mut().find_map(|observation| match observation {
        MapHomeObservation::Complete(flow) if flow.local_binding() == Some(binding) => Some(flow),
        _ => None,
    }) else {
        return false;
    };
    // Equal keys follow the literal key law: the earlier entry is displaced.
    let displaced = flow
        .all_entries()
        .filter(|entry| entry.key == *key)
        .last()
        .map(|entry| entry.site.clone());
    flow.stores.push(MapLocalStoreV1 {
        statement: statement.site().clone(),
        call: OwnedExprSiteV1::new(input.owner(), site),
        receiver: row.receiver_site().clone(),
        binding,
        entry: MapHomeEntry {
            site: value.site().clone(),
            key: key.clone(),
            ownership: MapEntryOwnership::Value(source),
            replaced_at: None,
            displaced,
        },
        homes: live,
    });
    true
}
