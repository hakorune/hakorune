//! Selected root emission checked against the same Home order as the raw
//! recorder, while the shared ledger remains untouched until collection.
use super::*;

pub(crate) struct PendingRootHomeExitV1<'ledger> {
    ledger: &'ledger OrdinaryNewClaimLedgerV1,
    owner: FunctionOwnerIdV1,
    site: SourceStmtSiteV1,
    order: RootHomeCleanupOrderV1,
    origins: Vec<RootHomeReleaseEmissionV1>,
    bindings: Vec<(BasicBlockId, MirInstruction)>,
    entry: RootHomeExitEntry,
}

impl PendingRootHomeExitV1<'_> {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn site(&self) -> &SourceStmtSiteV1 {
        &self.site
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn order(&self) -> &RootHomeCleanupOrderV1 {
        &self.order
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn origins(&self) -> &[RootHomeReleaseEmissionV1] {
        &self.origins
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn bindings(&self) -> &[(BasicBlockId, MirInstruction)] {
        &self.bindings
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn entry(&self) -> &RootHomeExitEntry {
        &self.entry
    }

    pub(crate) fn commit_after_collected(self) {
        let mut exits = self.ledger.root_exits.borrow_mut();
        let progress = exits
            .entry((self.owner, self.site))
            .or_insert(RootHomeExitProgress::Unprepared);
        assert!(matches!(progress, RootHomeExitProgress::Unprepared));
        *progress = RootHomeExitProgress::Emitted {
            order: self.order,
            origins: self.origins,
            bindings: self.bindings,
            entry: self.entry,
        };
    }
}

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_root_cleanup_shape(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
    ) -> Result<(), String> {
        self.validate_root_cleanup_shape_with_pending(owner, function, &[])
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_root_cleanup_shape_with_pending(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
        pending: &[PendingRootHomeExitV1<'_>],
    ) -> Result<(), String> {
        let exits = self.root_exits.borrow();
        for (key, progress) in exits.iter() {
            if key.0 != owner {
                continue;
            }
            let RootHomeExitProgress::Emitted { bindings, entry, order, .. } = progress else {
                continue;
            };
            if pending.iter().any(|row| row.owner() == owner && row.site() == &key.1) {
                return Err(freeze("selected-static-root-progress-drift"));
            }
            super::super::root_cleanup_graph::ordered_paths::validate(
                function, bindings, entry, order, None,
            )?;
            super::super::root_cleanup_graph::ordered_structure::validate_original(
                function, bindings, entry, order.ingress_result_kind(),
            )?;
        }
        for row in pending.iter().filter(|row| row.owner() == owner) {
            super::super::root_cleanup_graph::ordered_paths::validate(
                function, row.bindings(), row.entry(), row.order(), None,
            )?;
            super::super::root_cleanup_graph::ordered_structure::validate_original(
                function, row.bindings(), row.entry(), row.order().ingress_result_kind(),
            )?;
        }
        Ok(())
    }

    pub(crate) fn prepare_selected_plain_root_exit_v1(
        &self,
        prepared: PreparedRootHomeOrderV1,
        origins: Vec<(RootHomeReleaseOriginV1, BasicBlockId, MirInstruction)>,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
        group: &PreparedRootLexicalCallBindingGroupV1<'_>,
    ) -> Result<PendingRootHomeExitV1<'_>, String> {
        let owner = prepared.owner;
        let site = prepared.site;
        let local_bindings = vec![group.pending_group().clone()];
        self.validate_local_call_binding_groups(owner, &site, &local_bindings)?;
        self.prepare_selected_root_exit_v1(
            owner,
            site,
            prepared.order,
            origins,
            bindings,
            RootHomeExitEntry::Plain { local_bindings },
        )
    }

    fn prepare_selected_root_exit_v1(
        &self,
        owner: FunctionOwnerIdV1,
        site: SourceStmtSiteV1,
        order: RootHomeCleanupOrderV1,
        origins: Vec<(RootHomeReleaseOriginV1, BasicBlockId, MirInstruction)>,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
        entry: RootHomeExitEntry,
    ) -> Result<PendingRootHomeExitV1<'_>, String> {
        if bindings.is_empty()
            || self.root_exits.borrow().get(&(owner, site.clone())).is_some_and(|progress| {
                !matches!(progress, RootHomeExitProgress::Unprepared)
            })
        {
            return Err(freeze("selected-static-root-progress-drift"));
        }
        if order.normal()
            != origins
                .iter()
                .map(|(origin, _, _)| origin.clone())
                .collect::<Vec<_>>()
        {
            return Err(freeze("root-exit-origin-order"));
        }
        order.validate_direct_entry(&entry, &bindings, None)?;
        order.validate_null_return(&entry, &bindings, None, None)?;
        self.validate_received_return_producer_v1(
            owner, &site, &entry, &bindings, None, None,
        )?;
        self.validate_fresh_return_producer_v1(
            owner, &site, &entry, &bindings, None, None, None,
        )?;
        Ok(PendingRootHomeExitV1 {
            ledger: self,
            owner,
            site,
            order,
            origins: origins
                .into_iter()
                .map(|(origin, block, instruction)| RootHomeReleaseEmissionV1 {
                    origin,
                    block,
                    instruction,
                })
                .collect(),
            bindings,
            entry,
        })
    }
}
