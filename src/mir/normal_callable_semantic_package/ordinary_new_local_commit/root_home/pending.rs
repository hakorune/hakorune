//! Selected root emission checked against the same Home order as the raw
//! recorder, while the shared ledger remains untouched until collection.
use super::super::super::RootCallDispositionV1;
use super::*;
use crate::mir::callable_result_representation::VerifiedStaticCallResultPublicationHandoffV1;
use crate::mir::instruction::InvokeCallResultKind;
use std::rc::Rc;

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
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn owner(
        &self,
    ) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn site(
        &self,
    ) -> &SourceStmtSiteV1 {
        &self.site
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn order(
        &self,
    ) -> &RootHomeCleanupOrderV1 {
        &self.order
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn origins(
        &self,
    ) -> &[RootHomeReleaseEmissionV1] {
        &self.origins
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn bindings(
        &self,
    ) -> &[(BasicBlockId, MirInstruction)] {
        &self.bindings
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn entry(
        &self,
    ) -> &RootHomeExitEntry {
        &self.entry
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn append_lifecycle_bindings(
        &self,
        bindings: &mut Vec<(BasicBlockId, MirInstruction)>,
    ) -> Result<(), String> {
        let mut push = |binding: &(BasicBlockId, MirInstruction)| {
            if !bindings.contains(binding) {
                bindings.push(binding.clone());
            }
        };
        for binding in self.bindings() {
            push(binding);
        }
        let mut entry_bindings = Vec::new();
        self.entry.append_bindings(&mut entry_bindings);
        for binding in &entry_bindings {
            push(binding);
        }
        for original in self.order.text_expression_bindings_v1() { push(original); }
        if let Some(original) = &self.order.text_return_hold {
            push(original);
        }
        if let Some(original) = self.order.null_return_binding() {
            push(original);
        }
        for dependency in self.entry.copy_dependencies(self.ledger)? {
            push(&dependency);
        }
        Ok(())
    }

    pub(crate) fn commit_after_collected(self) {
        let Self {
            ledger,
            owner,
            site,
            order,
            origins,
            bindings,
            entry,
        } = self;
        assert!(!matches!(
            &entry,
            RootHomeExitEntry::Call {
                row: RootCallDispositionV1::PendingStatic(_),
                ..
            }
        ));
        Self::commit_entry(ledger, owner, site, order, origins, bindings, entry);
    }

    pub(crate) fn commit_static_after_collected(
        self,
        publication: VerifiedStaticCallResultPublicationHandoffV1,
    ) {
        let Self {
            ledger,
            owner,
            site,
            order,
            origins,
            bindings,
            entry,
        } = self;
        let entry = match entry {
            RootHomeExitEntry::Call {
                row: RootCallDispositionV1::PendingStatic(pending),
                local_bindings,
                arguments,
                invoke,
                projection,
                frame,
            } => {
                assert!(pending
                    .packet()
                    .original()
                    .corroborates_publication_handoff(&publication));
                let (row, prepared) = pending.into_packet(publication);
                let packet = EmittedLexicalCallProjectionV1::from_source(
                    row,
                    prepared,
                    invoke.clone(),
                    projection.clone(),
                );
                RootHomeExitEntry::Call {
                    row: RootCallDispositionV1::Lexical(Rc::new(packet)),
                    local_bindings,
                    arguments,
                    invoke,
                    projection,
                    frame,
                }
            }
            _ => panic!("selected Static root exit requires its pending packet"),
        };
        Self::commit_entry(ledger, owner, site, order, origins, bindings, entry);
    }

    fn commit_entry(
        ledger: &OrdinaryNewClaimLedgerV1,
        owner: FunctionOwnerIdV1,
        site: SourceStmtSiteV1,
        order: RootHomeCleanupOrderV1,
        origins: Vec<RootHomeReleaseEmissionV1>,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
        entry: RootHomeExitEntry,
    ) {
        let mut exits = ledger.root_exits.borrow_mut();
        let progress = exits
            .entry((owner, site.into()))
            .or_insert(RootHomeExitProgress::Unprepared);
        assert!(matches!(progress, RootHomeExitProgress::Unprepared));
        *progress = RootHomeExitProgress::Emitted {
            order,
            origins,
            bindings,
            entry,
        };
    }
}

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_root_cleanup_shape(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
    ) -> Result<(), String> {
        self.validate_root_cleanup_shape_rows(owner, function, &[])
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_root_cleanup_shape_with_pending(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
        pending: &super::super::PendingSelectedStaticLoopViewV1<'_, '_>,
    ) -> Result<(), String> {
        self.validate_root_cleanup_shape_rows(owner, function, &pending.roots())
    }

    fn validate_root_cleanup_shape_rows(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
        pending: &[&PendingRootHomeExitV1<'_>],
    ) -> Result<(), String> {
        let exits = self.root_exits.borrow();
        for (key, progress) in exits.iter() {
            if key.0 != owner {
                continue;
            }
            let RootHomeExitProgress::Emitted {
                bindings,
                entry,
                order,
                ..
            } = progress
            else {
                continue;
            };
            if pending
                .iter()
                .any(|row| row.owner() == owner && &key.1 == row.site())
            {
                return Err(freeze("selected-static-root-progress-drift"));
            }
            super::super::root_cleanup_graph::ordered_paths::validate(
                function, bindings, entry, order, None,
            )?;
            super::super::root_cleanup_graph::ordered_structure::validate_original_with_wrapper(
                function,
                bindings,
                entry,
                order.ingress_result_kind(),
                &self.main_arguments_release_suffix_v1(owner, function)?,
            )?;
        }
        for row in pending.iter().filter(|row| row.owner() == owner) {
            super::super::root_cleanup_graph::ordered_paths::validate(
                function,
                row.bindings(),
                row.entry(),
                row.order(),
                None,
            )?;
            super::super::root_cleanup_graph::ordered_structure::validate_original_with_wrapper(
                function,
                row.bindings(),
                row.entry(),
                row.order().ingress_result_kind(),
                &self.main_arguments_release_suffix_v1(owner, function)?,
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
        pending_routes: &[OwnedExprSiteV1],
    ) -> Result<PendingRootHomeExitV1<'_>, String> {
        let owner = prepared.owner;
        let site = prepared.site;
        let local_bindings = vec![group.pending_group().clone()];
        self.validate_local_call_binding_groups_with_pending(
            owner,
            &site,
            &local_bindings,
            pending_routes,
        )?;
        self.prepare_selected_root_exit_v1(
            owner,
            site,
            prepared.order,
            origins,
            bindings,
            RootHomeExitEntry::Plain { local_bindings },
        )
    }

    pub(crate) fn prepare_selected_static_root_exit_v1(
        &self,
        prepared: PreparedRootHomeOrderV1,
        origins: Vec<(RootHomeReleaseOriginV1, BasicBlockId, MirInstruction)>,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
        group: &PreparedRootLexicalCallBindingGroupV1<'_>,
        route: &super::super::super::PreparedStaticTerminalRootRouteV1<'_>,
        pending_routes: &[OwnedExprSiteV1],
        packet: PreparedStaticTerminalPacketV1,
        prepared_projection: PreparedLexicalCallProjectionV1,
        publication: &VerifiedStaticCallResultPublicationHandoffV1,
        invoke: (BasicBlockId, MirInstruction),
        projection: (BasicBlockId, MirInstruction),
        frame: (BasicBlockId, MirInstruction),
    ) -> Result<PendingRootHomeExitV1<'_>, String> {
        let owner = prepared.owner;
        let site = prepared.site;
        let local_bindings = vec![group.pending_group().clone()];
        if !pending_routes.contains(route.site()) {
            return Err(freeze("selected-static-root-route-drift"));
        }
        self.validate_local_call_binding_groups_with_pending(
            owner,
            &site,
            &local_bindings,
            pending_routes,
        )?;
        let call = prepared_projection.materialize_using(
            owner,
            packet.loan(publication),
            packet.observation().arguments(),
            Some(self),
        )?;
        if !matches!(&invoke.1,
            MirInstruction::Invoke {
                operation: InvokeOperation::Call { call: actual, result },
                normal_landing,
                ..
            } if actual == &call
                && *result == InvokeCallResultKind::I64
                && *normal_landing == projection.0)
            || !matches!(&projection.1,
                MirInstruction::InvokeNormalResult { invoke_block, .. }
                    if *invoke_block == invoke.0)
        {
            return Err(freeze("selected-static-root-call-drift"));
        }
        let pending = PendingStaticCallProjectionV1::new(packet, prepared_projection, call);
        self.prepare_selected_root_exit_v1(
            owner,
            site,
            prepared.order,
            origins,
            bindings,
            RootHomeExitEntry::Call {
                row: RootCallDispositionV1::PendingStatic(pending),
                local_bindings,
                arguments: Vec::new(),
                invoke,
                projection,
                frame,
            },
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
            || self
                .root_exits
                .borrow()
                .get(&(owner, (site.clone()).into()))
                .is_some_and(|progress| !matches!(progress, RootHomeExitProgress::Unprepared))
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
        order.validate_direct_entry(self, owner, &site, &entry, &bindings, None)?;
        order.validate_null_return(&entry, &bindings, None, None)?;
        self.validate_text_return_producer_v1(CallValidationContextV1::Lowering(self), owner, &site, &order, &entry, &bindings, None, None, None)?;
        self.validate_received_return_producer_v1(owner, &site, &entry, &bindings, None, None)?;
        self.validate_fresh_return_producer_v1(owner, &site, &entry, &bindings, None, None, None)?;
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

#[cfg(test)]
#[path = "pending_order_tests.rs"]
mod order_tests;
