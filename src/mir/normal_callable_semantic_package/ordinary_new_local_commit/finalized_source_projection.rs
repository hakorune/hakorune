//! Read-only original source and finished-coordinate loans from the same owner.
use super::super::lexical_instance_call::PreparedBorrowedFormalActualV1;
use super::*;
use crate::mir::normal_callable_semantic_package::LexicalInstanceCallDispositionRowV1;

type Binding = (BasicBlockId, MirInstruction);

impl FinalizedRootSourceHandoffV1 {
    /// Only an exact producer from an original retained node can use this map.
    /// Cleanup/frame membership and a matching finished instruction are insufficient.
    pub(in crate::mir) fn finished_local_call_producer_v1(
        &self,
        owner: FunctionOwnerIdV1,
        group_site: &OwnedExprSiteV1,
        node_site: &OwnedExprSiteV1,
        original: &Binding,
        function: &MirFunction,
    ) -> Result<(BasicBlockId, usize), String> {
        if group_site.owner() != owner || node_site.owner() != owner {
            return Err(freeze("finished-local-call/owner"));
        }
        let mut groups = self
            .local_calls
            .get(&owner)
            .into_iter()
            .flatten()
            .filter(|group| group.site() == group_site);
        let group = groups
            .next()
            .ok_or_else(|| freeze("finished-local-call/group-missing"))?;
        if groups.next().is_some() {
            return Err(freeze("finished-local-call/group-duplicate"));
        }
        let packet = group
            .lexical()
            .ok_or_else(|| freeze("finished-local-call/packet-missing"))?;
        if !packet.has_producer_at(node_site, original) {
            return Err(freeze("finished-local-call/original-producer"));
        }
        let (symbol, finished) = self.ledger.finished_binding_for_owner(owner, original)?;
        if function.signature.name != symbol {
            return Err(freeze("finished-local-call/function"));
        }
        let block = function
            .blocks
            .get(&finished.0)
            .ok_or_else(|| freeze("finished-local-call/block-missing"))?;
        let mut matches = block
            .all_instructions()
            .enumerate()
            .filter(|(_, instruction)| *instruction == &finished.1);
        let (index, _) = matches
            .next()
            .ok_or_else(|| freeze("finished-local-call/producer-missing"))?;
        if matches.next().is_some() {
            return Err(freeze("finished-local-call/producer-duplicate"));
        }
        Ok((finished.0, index))
    }

    pub(in crate::mir) fn borrowed_call_actuals_v1(
        &self,
        row: &LexicalInstanceCallDispositionRowV1,
    ) -> Result<Option<&[PreparedBorrowedFormalActualV1]>, String> {
        self.ledger.borrowed_call_actuals_v1(row)
    }

    pub(in crate::mir) fn borrowed_ordinary_entry_values_v1(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Result<Box<[(u32, BindingRefV1, ValueId)]>, String> {
        self.ledger.borrowed_ordinary_entry_values_v1(owner)
    }
}

impl OrdinaryNewClaimLedgerV1 {
    fn finished_binding_for_owner(
        &self,
        owner: FunctionOwnerIdV1,
        original: &Binding,
    ) -> Result<(String, Binding), String> {
        let root = self.root_validation.borrow();
        if let RootNewValidation::ArtifactFinalized {
            owner: root_owner,
            symbol,
            projection,
        } = &*root
        {
            if *root_owner == owner {
                return project_recorded(symbol, projection, original);
            }
        } else {
            return Err(freeze("finished-local-call/root-not-finalized"));
        }
        let children = self.child_physical_validation.borrow();
        match children.get(&owner) {
            Some(ChildPhysicalValidation::FinishingChecked { symbol, projection }) => {
                project_recorded(symbol, projection, original)
            }
            _ => Err(freeze("finished-local-call/child-not-finished")),
        }
    }
}

fn project_recorded(
    symbol: &str,
    projection: &physical_boundary::FinishedBindings,
    original: &Binding,
) -> Result<(String, Binding), String> {
    let finished = projection
        .binding(original.0, &original.1)?
        .ok_or_else(|| freeze("finished-local-call/producer-removed"))?;
    if !projection.recorded().contains(&finished) {
        return Err(freeze("finished-local-call/producer-unrecorded"));
    }
    Ok((symbol.to_owned(), finished))
}
