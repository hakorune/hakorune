//! Existing root exit progress retains the same affine Call source row.
//! Physical observations do not issue a target, result or cleanup obligation.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::LifecyclePrefixSourceV1;
use crate::mir::normal_callable_semantic_package::RootCallDispositionV1;

/// A source-ordered group checked without changing the shared ledger.
/// A selected caller may hold this through fallible draft admission; existing
/// callers commit it immediately.
pub(crate) struct PreparedLifecyclePrefixBindingGroupV1<'ledger> {
    ledger: &'ledger OrdinaryNewClaimLedgerV1,
    owner: FunctionOwnerIdV1,
    group: RootLifecyclePrefixBindingGroupV1,
    preceding_sites: Vec<OwnedExprSiteV1>,
}

/// Keep the original emitted packet and its pending group in one owner.
/// The packet becomes visible to the final visitor only after `commit`.
pub(crate) struct PreparedRootLexicalCallBindingGroupV1<'ledger> {
    group: PreparedLifecyclePrefixBindingGroupV1<'ledger>,
    packet: std::rc::Rc<EmittedLexicalCallProjectionV1>,
}

impl PreparedRootLexicalCallBindingGroupV1<'_> {
    pub(crate) fn source_site(&self) -> &OwnedExprSiteV1 {
        self.group.group.site()
    }

    pub(crate) fn original_invoke_binding(&self) -> &(BasicBlockId, MirInstruction) {
        self.packet.invoke_binding()
    }

    pub(crate) fn pending_group(&self) -> &RootLifecyclePrefixBindingGroupV1 {
        &self.group.group
    }

    pub(crate) fn commit(self) -> Result<std::rc::Rc<EmittedLexicalCallProjectionV1>, String> {
        self.group.commit()?;
        Ok(self.packet)
    }

    /// The selected collector has already committed after this group's
    /// source-order preflight; no fallible check may follow that terminal.
    pub(crate) fn commit_after_collected(self) {
        self.group.commit_after_collected();
    }
}

impl PreparedLifecyclePrefixBindingGroupV1<'_> {
    pub(crate) fn commit(self) -> Result<(), String> {
        let expected = self.ledger.expected_local_call_binding_sources(self.owner)?;
        if expected.get(self.preceding_sites.len()).is_none_or(|source| !self.group.matches_source(source)) {
            return Err(freeze("local-call-binding-preflight-stale"));
        }
        let mut rows = self.ledger.root_local_call_bindings.borrow_mut();
        let groups = rows.get(&self.owner).map(Vec::as_slice).unwrap_or(&[]);
        if groups.len() != self.preceding_sites.len()
            || groups
                .iter()
                .map(|recorded| recorded.site())
                .ne(self.preceding_sites.iter())
            || groups
                .iter()
                .any(|recorded| recorded.site() == self.group.site())
        {
            return Err(freeze("local-call-binding-preflight-stale"));
        }
        rows.entry(self.owner).or_default().push(self.group);
        Ok(())
    }

    fn commit_after_collected(self) {
        self.ledger
            .root_local_call_bindings
            .borrow_mut()
            .entry(self.owner)
            .or_default()
            .push(self.group);
    }
}

impl OrdinaryNewClaimLedgerV1 {
    #[cfg(test)]
    pub(in crate::mir) fn selected_static_entry_group_count_for_test(&self) -> usize {
        self.root_local_call_bindings
            .borrow()
            .values()
            .flatten()
            .filter(|group| group.lexical().is_some())
            .count()
    }

    #[cfg(test)]
    pub(in crate::mir) fn emitted_root_exit_count_for_test(&self) -> usize {
        self.root_exits
            .borrow()
            .values()
            .filter(|progress| matches!(progress, RootHomeExitProgress::Emitted { .. }))
            .count()
    }

    #[cfg(test)]
    pub(in crate::mir) fn emitted_root_exits_share_one_lexical_group_for_test(&self) -> bool {
        let exits = self.root_exits.borrow();
        let mut sites = exits.values()
            .filter_map(|progress| {
                let RootHomeExitProgress::Emitted { entry, .. } = progress else {
                    return None;
                };
                let groups = match entry {
                    RootHomeExitEntry::Plain { local_bindings }
                    | RootHomeExitEntry::Call { local_bindings, .. }
                    | RootHomeExitEntry::MapGet { local_bindings, .. } => local_bindings,
                };
                Some(
                    groups
                        .iter()
                        .filter(|group| group.lexical().is_some())
                        .map(|group| group.site().clone()),
                )
            })
            .flatten();
        let Some(first) = sites.next() else {
            return false;
        };
        sites.all(|site| site == first)
    }

    #[cfg(test)]
    pub(in crate::mir::normal_callable_semantic_package) fn with_local_call_binding_groups_for_test<
        R,
    >(
        &self,
        owner: FunctionOwnerIdV1,
        visit: impl FnOnce(&[RootLifecyclePrefixBindingGroupV1]) -> R,
    ) -> R {
        let groups = self.root_local_call_bindings.borrow();
        visit(groups.get(&owner).map(Vec::as_slice).unwrap_or(&[]))
    }

    /// Simulate the pending pool moving into its finalized artifact owner.
    #[cfg(test)]
    pub(crate) fn validate_new_emissions_after_local_pool_move_for_test(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
    ) -> Result<(), String> {
        let groups = self
            .root_local_call_bindings
            .borrow_mut()
            .remove(&owner)
            .ok_or_else(|| freeze("local-call-pool-missing"))?;
        let result = self.validate_new_emissions(owner, function);
        self.root_local_call_bindings
            .borrow_mut()
            .insert(owner, groups);
        result
    }

    pub(crate) fn record_map_read_bindings(
        &self,
        owner: FunctionOwnerIdV1,
        site: OwnedExprSiteV1,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
    ) -> Result<(), String> {
        if site.owner() != owner || bindings.is_empty() {
            return Err(freeze("map-read-binding-shape"));
        }
        let mut rows = self.map_read_bindings.borrow_mut();
        let groups = rows.entry(owner).or_default();
        if groups.iter().any(|(recorded, _)| recorded == &site) {
            return Err(freeze("map-read-binding-duplicate"));
        }
        groups.push((site, bindings));
        Ok(())
    }

    pub(crate) fn record_root_local_call_bindings(
        &self,
        owner: FunctionOwnerIdV1,
        site: OwnedExprSiteV1,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
    ) -> Result<(), String> {
        self.record_local_call_binding_group(
            owner,
            RootLifecyclePrefixBindingGroupV1::new(site, bindings, None)?,
        )
    }

    pub(crate) fn record_root_lexical_call_bindings(
        &self,
        owner: FunctionOwnerIdV1,
        site: OwnedExprSiteV1,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
        packet: EmittedLexicalCallProjectionV1,
    ) -> Result<std::rc::Rc<EmittedLexicalCallProjectionV1>, String> {
        self.prepare_root_lexical_call_bindings(owner, site, bindings, packet)?
            .commit()
    }

    pub(crate) fn prepare_root_lexical_call_bindings(
        &self,
        owner: FunctionOwnerIdV1,
        site: OwnedExprSiteV1,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
        packet: EmittedLexicalCallProjectionV1,
    ) -> Result<PreparedRootLexicalCallBindingGroupV1<'_>, String> {
        let packet = std::rc::Rc::new(packet);
        let group = self.prepare_local_call_binding_group(
            owner,
            RootLifecyclePrefixBindingGroupV1::new(site, bindings, Some(std::rc::Rc::clone(&packet)))?,
        )?;
        Ok(PreparedRootLexicalCallBindingGroupV1 { group, packet })
    }

    pub(crate) fn print_expression_frame_v1(
        &self, owner: FunctionOwnerIdV1, statement: &SourceStmtSiteV1,
    ) -> Result<Option<std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedPrintExpressionFrameV1>>, String> {
        let Some(flow) = self.completion_for_owner(owner).and_then(|completion| completion.cleanup().root_flow())
            else { return Ok(None) };
        let mut frames = flow.expression_plans().iter().filter_map(|plan| plan.print_frame())
            .filter(|frame| frame.source().root().statement() == statement);
        let frame = frames.next().cloned();
        if frame.is_none() && flow.statement_prefix_unavailable(statement) {
            return Err(freeze("print-expression/source-unavailable"));
        }
        if frames.next().is_some() { return Err(freeze("print-expression/duplicate-source-frame")); }
        Ok(frame)
    }

    pub(crate) fn validate_print_expression_source_v1(
        &self, frame: &std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedPrintExpressionFrameV1>,
    ) -> Result<(), String> {
        let owner = frame.source().owner();
        let sources = self.expected_local_call_binding_sources(owner)?;
        let groups = self.root_local_call_bindings.borrow();
        let count = groups.get(&owner).map_or(0, Vec::len);
        if !matches!(sources.get(count), Some(LifecyclePrefixSourceV1::PrintExpression { frame: expected, .. })
            if std::rc::Rc::ptr_eq(frame, expected)) {
            return Err(freeze("print-expression/source-prefix-order"));
        }
        Ok(())
    }

    /// The same pool owns a complete expression prefix. Source/frame identity
    /// and ordering are checked before this pending group can be published.
    pub(crate) fn prepare_print_expression_bindings(
        &self, owner: FunctionOwnerIdV1,
        packet: std::rc::Rc<EmittedPrintExpressionProjectionV1>,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
    ) -> Result<PreparedLifecyclePrefixBindingGroupV1<'_>, String> {
        self.prepare_local_call_binding_group(owner,
            RootLifecyclePrefixBindingGroupV1::new_print(packet, bindings)?)
    }

    fn record_local_call_binding_group(
        &self,
        owner: FunctionOwnerIdV1,
        group: RootLifecyclePrefixBindingGroupV1,
    ) -> Result<(), String> {
        self.prepare_local_call_binding_group(owner, group)?
            .commit()
    }

    fn prepare_local_call_binding_group(
        &self,
        owner: FunctionOwnerIdV1,
        group: RootLifecyclePrefixBindingGroupV1,
    ) -> Result<PreparedLifecyclePrefixBindingGroupV1<'_>, String> {
        let site = group.site().clone();
        if site.owner() != owner {
            return Err(freeze("local-call-binding-owner-drift"));
        }
        let expected = self.expected_local_call_binding_sources(owner)?;
        let rows = self.root_local_call_bindings.borrow();
        let groups = rows.get(&owner).map(Vec::as_slice).unwrap_or(&[]);
        if groups.iter().any(|recorded| recorded.site() == &site) {
            return Err(freeze("duplicate-local-call-bindings"));
        }
        if groups.len() >= expected.len() {
            return Err(freeze("local-call-bindings-overflow"));
        }
        if expected.get(groups.len()).is_none_or(|source| !group.matches_source(source)) {
            return Err(freeze("local-call-binding-site-order"));
        }
        Ok(PreparedLifecyclePrefixBindingGroupV1 {
            ledger: self,
            owner,
            group,
            preceding_sites: groups
                .iter()
                .map(|recorded| recorded.site().clone())
                .collect(),
        })
    }

    /// `co_seal_lifecycle` is the routing authority: it marks exactly the
    /// sealed local-call sites whose affine rows take the lifecycle Invoke
    /// lane. The binding-group expectation is that marked subset — a sealed
    /// I64 local call that keeps the scalar Call route emits a plain `Call`
    /// instruction and owes no lifecycle bindings.
    pub(crate) fn record_lifecycle_local_call_site(
        &self,
        owner: FunctionOwnerIdV1,
        site: OwnedExprSiteV1,
    ) {
        let mut routed = self.lifecycle_local_call_sites.borrow_mut();
        let sites = routed.entry(owner).or_default();
        if !sites.contains(&site) {
            sites.push(site);
        }
    }

    /// Select the local emitter without stealing receiver-owned Object rows.
    pub(crate) fn is_lifecycle_local_call_site_v1(&self, site: &OwnedExprSiteV1) -> Result<bool, String> {
        if !self.lifecycle_local_call_sites.borrow().get(&site.owner())
            .is_some_and(|sites| sites.contains(site)) { return Ok(false); }
        use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::LexicalInstanceCallDispositionSlotV1;
        let slots = self.lexical_instance_calls.borrow();
        match slots.get(site) {
            Some(LexicalInstanceCallDispositionSlotV1::Ready(row)) if row.source_target().borrowed_nullable_local_source().is_some() => {
                self.corroborate_borrowed_nullable_call_v1(row)?;
                Ok(true)
            }
            Some(LexicalInstanceCallDispositionSlotV1::Ready(row)) if row.source_target().mixed_local_source().is_some() => {
                self.corroborate_mixed_call_v1(row)?;
                Ok(true)
            }
            Some(LexicalInstanceCallDispositionSlotV1::Ready(row)) if row.source_target().is_self_receiver() => {
                let Some(loan) = self.lexical_i64_call_source(site)
                    .and_then(|observation| observation.borrowed_scalar()) else { return Ok(false); };
                if loan.source() != row.source_target() || row.scalar_result_source_v1() != Some(loan) {
                    return Err(freeze("lexical-instance-call/self-scalar-source-drift"));
                }
                self.checked_local_scalar_call_result_v1(row)?;
                Ok(true)
            }
            // Selected but unavailable/consumed remains selected: the affine
            // take below supplies the existing fail-fast diagnostic.
            _ => Ok(true),
        }
    }

    pub(super) fn expected_local_call_binding_sources(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Result<Vec<LifecyclePrefixSourceV1>, String> {
        let completion = self.completion_for_owner(owner)
            .ok_or_else(|| freeze("local-call-source-missing"))?;
        let flow = completion.cleanup().root_flow()
            .ok_or_else(|| freeze("local-call-source-missing"))?;
        let routed = self.lifecycle_local_call_sites.borrow();
        flow.lifecycle_prefixes(routed.get(&owner).map(Vec::as_slice).unwrap_or(&[]))
            .map_err(freeze)
    }

    /// Test-only sole-exit shorthand — production resolves through
    /// `terminal_call_arguments_for_owner_at`.
    #[cfg(test)]
    pub(crate) fn terminal_call_arguments(
        &self,
    ) -> Option<&[crate::mir::resolved_semantics::home_new_prefix::TerminalCallArgumentV1]> {
        self.call_source_completion()
            .map(|(_, terminal)| terminal.arguments())
    }

    /// Borrow the terminal Call seated at this exact exit site. A sibling
    /// exit's Call relation is never a substitute — the argument evidence is
    /// bound to its own `return` statement.
    pub(crate) fn terminal_call_arguments_for_owner_at(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceStmtSiteV1,
    ) -> Option<&[crate::mir::resolved_semantics::home_new_prefix::TerminalCallArgumentV1]> {
        self.call_source_completion_for_owner_at(owner, site)
            .map(|(_, terminal)| terminal.arguments())
    }

    pub(crate) fn record_root_call_exit(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceStmtSiteV1,
        row: RootCallDispositionV1,
        arguments: Vec<(BasicBlockId, MirInstruction)>,
        invoke: (BasicBlockId, MirInstruction),
        projection: (BasicBlockId, MirInstruction),
        frame: (BasicBlockId, MirInstruction),
        origins: Vec<(RootHomeReleaseOriginV1, BasicBlockId, MirInstruction)>,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
    ) -> Result<(), String> {
        if matches!(&row, RootCallDispositionV1::Direct(_)) {
            if let RootCallDispositionV1::Direct(direct) = &row {
                let _ = direct.physical_emission();
            }
        }
        if matches!(&row, RootCallDispositionV1::PendingStatic(_)) {
            return Err(freeze("pending-static-entry"));
        }
        if let RootCallDispositionV1::Lexical(packet) = &row {
            self.validate_lexical_terminal_packet(owner, site, packet)?;
            let (original_invoke, original_projection) = packet.outer_bindings()?;
            if original_invoke != &invoke || original_projection != &projection {
                return Err(freeze("lexical-terminal/original-outer-drift"));
            }
            let mut recorded = arguments.clone();
            recorded.extend([invoke.clone(), projection.clone()]);
            packet.validate_recorded(&recorded)?;
        }
        let local_bindings = self.select_local_call_binding_groups(owner, site)?;
        self.record_root_home_exit_with_entry(
            owner,
            site,
            origins,
            bindings,
            RootHomeExitEntry::Call {
                row,
                local_bindings,
                arguments,
                invoke,
                projection,
                frame,
            },
        )
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn rebind_root_call_entry(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceStmtSiteV1,
        projection: &super::super::physical_boundary::FinishedBindings,
    ) -> Result<(), String> {
        let mut exits = self.root_exits.borrow_mut();
        let Some(progress) = exits.get_mut(&(owner, site.clone().into())) else {
            return Err(freeze("root-call-entry-missing"));
        };
        // A Call terminal whose receiver home can never reach `end_available`
        // is recorded `Unavailable` by `prepare_root_home_exit` and lowered
        // through the generic value path — there is no entry to rebind, the
        // same non-Call tolerance `Emitted{Plain}`/`Emitted{MapGet}` already
        // get. The sealing lanes stay the sole rejection authority.
        if matches!(progress, RootHomeExitProgress::Unavailable) {
            return Ok(());
        }
        let RootHomeExitProgress::Emitted {
            bindings, entry, ..
        } = progress
        else {
            return Err(freeze("root-call-entry-missing"));
        };
        let RootHomeExitEntry::Call {
            local_bindings,
            arguments,
            invoke,
            projection: result_projection,
            frame,
            ..
        } = entry
        else {
            return Ok(());
        };
        let map = |binding: &(BasicBlockId, MirInstruction)| {
            projection
                .binding(binding.0, &binding.1)?
                .ok_or_else(|| freeze("root-call-binding-contracted"))
        };
        let mapped_local_bindings = local_bindings
            .iter()
            .map(|group| {
                Ok::<_, String>(
                    group.with_bindings(
                        group
                            .bindings()
                            .iter()
                            .map(map)
                            .collect::<Result<Vec<_>, _>>()?,
                    ),
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        let mapped_arguments = arguments.iter().map(map).collect::<Result<Vec<_>, _>>()?;
        let mapped_invoke = map(invoke)?;
        let mapped_projection = map(result_projection)?;
        let mapped_frame = map(frame)?;
        let mapped_cleanup = projection.bindings(bindings)?;
        *local_bindings = mapped_local_bindings;
        *arguments = mapped_arguments;
        *invoke = mapped_invoke;
        *result_projection = mapped_projection;
        *frame = mapped_frame;
        *bindings = mapped_cleanup;
        Ok(())
    }

    /// Check the whole selected set under one borrow before moving any entry.
    /// Semantic and finished-binding preflight stays with the collector.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn take_finalized_root_calls(
        &self,
        owner: FunctionOwnerIdV1,
        sites: &std::collections::BTreeSet<SourceStmtSiteV1>,
    ) -> Result<
        std::collections::BTreeMap<
            SourceStmtSiteV1,
            (RootHomeExitEntry, Vec<(BasicBlockId, MirInstruction)>),
        >,
        String,
    > {
        let mut exits = self.root_exits.borrow_mut();
        for site in sites {
            match exits.get(&(owner, site.clone().into())) {
                Some(RootHomeExitProgress::Emitted {
                    entry: RootHomeExitEntry::Call { .. },
                    ..
                }) => {}
                Some(RootHomeExitProgress::Finalized(_)) => {
                    return Err(freeze("root-call-already-finalized"));
                }
                _ => return Err(freeze("artifact-call-physical-missing")),
            }
        }
        let mut taken = std::collections::BTreeMap::new();
        for site in sites {
            let progress = exits
                .get_mut(&(owner, site.clone().into()))
                .expect("whole batch checked under the same borrow");
            let RootHomeExitProgress::Emitted {
                order,
                bindings,
                entry,
                ..
            } = std::mem::replace(progress, RootHomeExitProgress::Unprepared)
            else {
                unreachable!("whole batch checked under the same borrow");
            };
            *progress = RootHomeExitProgress::Finalized(order);
            taken.insert(site.clone(), (entry, bindings));
        }
        Ok(taken)
    }

    pub(crate) fn take_finalized_root_call(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceStmtSiteV1,
    ) -> Result<Option<(RootHomeExitEntry, Vec<(BasicBlockId, MirInstruction)>)>, String> {
        let mut exits = self.root_exits.borrow_mut();
        let Some(progress) = exits.get_mut(&(owner, site.clone().into())) else {
            return Ok(None);
        };
        let state = std::mem::replace(progress, RootHomeExitProgress::Unprepared);
        match state {
            RootHomeExitProgress::Emitted {
                order,
                bindings,
                entry:
                    RootHomeExitEntry::Call {
                        row,
                        local_bindings,
                        arguments,
                        invoke,
                        projection,
                        frame,
                    },
                ..
            } => {
                *progress = RootHomeExitProgress::Finalized(order);
                Ok(Some((
                    RootHomeExitEntry::Call {
                        row,
                        local_bindings,
                        arguments,
                        invoke,
                        projection,
                        frame,
                    },
                    bindings,
                )))
            }
            RootHomeExitProgress::Emitted {
                order,
                origins,
                bindings,
                entry: RootHomeExitEntry::Plain { local_bindings },
            } => {
                *progress = RootHomeExitProgress::Emitted {
                    order,
                    origins,
                    bindings,
                    entry: RootHomeExitEntry::Plain { local_bindings },
                };
                Ok(None)
            }
            // The MapGet payload stays internal like a Plain entry: the
            // physical instructions already live in the finished function
            // body, so the handoff carries no separate read receipt.
            RootHomeExitProgress::Emitted {
                order,
                origins,
                bindings,
                entry:
                    RootHomeExitEntry::MapGet {
                        local_bindings,
                        invoke,
                        projection,
                        frame,
                    },
            } => {
                *progress = RootHomeExitProgress::Emitted {
                    order,
                    origins,
                    bindings,
                    entry: RootHomeExitEntry::MapGet {
                        local_bindings,
                        invoke,
                        projection,
                        frame,
                    },
                };
                Ok(None)
            }
            RootHomeExitProgress::Finalized(order) => {
                *progress = RootHomeExitProgress::Finalized(order);
                Err(freeze("root-call-already-finalized"))
            }
            other => {
                *progress = other;
                Err(freeze("root-call-entry-unavailable"))
            }
        }
    }


}

impl RootHomeExitEntry {
    pub(crate) fn call_row(
        &self,
    ) -> Option<&crate::mir::normal_callable_semantic_package::RootCallDispositionV1> {
        match self {
            Self::Call { row, .. } => Some(row),
            Self::Plain { .. } | Self::MapGet { .. } => None,
        }
    }

    pub(crate) fn call_arguments(&self) -> Option<&[(BasicBlockId, MirInstruction)]> {
        match self {
            Self::Call { arguments, .. } => Some(arguments),
            Self::Plain { .. } | Self::MapGet { .. } => None,
        }
    }

    pub(crate) fn call_invoke(&self) -> Option<&(BasicBlockId, MirInstruction)> {
        match self {
            Self::Call { invoke, .. } => Some(invoke),
            Self::Plain { .. } | Self::MapGet { .. } => None,
        }
    }

    pub(crate) fn call_projection(&self) -> Option<&(BasicBlockId, MirInstruction)> {
        match self {
            Self::Call { projection, .. } => Some(projection),
            Self::Plain { .. } | Self::MapGet { .. } => None,
        }
    }

    pub(crate) fn call_frame(&self) -> Option<&(BasicBlockId, MirInstruction)> {
        match self {
            Self::Call { frame, .. } => Some(frame),
            Self::Plain { .. } | Self::MapGet { .. } => None,
        }
    }
}

#[cfg(test)]
#[path = "root_call_entry/tests.rs"]
mod tests;

impl RootHomeExitEntry {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn copy_dependencies(
        &self,
        ledger: &OrdinaryNewClaimLedgerV1,
    ) -> Result<Vec<(BasicBlockId, MirInstruction)>, String> {
        let groups = match self {
            Self::Plain { local_bindings }
            | Self::Call { local_bindings, .. }
            | Self::MapGet { local_bindings, .. } => local_bindings,
        };
        let mut copies = Vec::new();
        for packet in groups.iter().flat_map(|group| group.lexical_packets()) {
            copies.extend(
                packet
                    .copy_dependencies(ledger)?
                    .into_iter()
                    .map(|(_, copy)| copy),
            );
        }
        if let Self::Call {
            row: RootCallDispositionV1::Lexical(packet),
            ..
        } = self
        {
            copies.extend(
                packet
                    .copy_dependencies(ledger)?
                    .into_iter()
                    .map(|(_, copy)| copy),
            );
        }
        Ok(copies)
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn append_bindings(
        &self,
        bindings: &mut Vec<(BasicBlockId, MirInstruction)>,
    ) {
        let local_groups = match self {
            Self::Plain { local_bindings } => local_bindings,
            Self::Call { local_bindings, .. } | Self::MapGet { local_bindings, .. } => {
                local_bindings
            }
        };
        for group in local_groups {
            bindings.extend_from_slice(group.bindings());
        }
        if let Self::Call {
            arguments,
            invoke,
            projection,
            frame,
            ..
        } = self
        {
            bindings.extend_from_slice(arguments);
            bindings.push(invoke.clone());
            bindings.push(projection.clone());
            bindings.push(frame.clone());
        }
        if let Self::MapGet {
            invoke,
            projection,
            frame,
            ..
        } = self
        {
            bindings.push(invoke.clone());
            bindings.push(projection.clone());
            bindings.push(frame.clone());
        }
    }
}

#[path = "root_call_entry/validation.rs"]
mod validation;

#[path = "root_call_entry/lexical_projection.rs"]
mod lexical_projection;
pub(in crate::mir) use lexical_projection::{
    CallPacketSourceLoanV1, CallPacketSourceV1, EmittedLexicalCallProjectionV1,
    LexicalCallArgumentProjectionV1, PendingStaticCallProjectionV1,
    PreparedLexicalCallProjectionV1, PreparedSelectedStaticLoopCallProjectionV1,
};
pub(crate) use lexical_projection::PreparedStaticTerminalPacketV1;

#[path = "root_call_entry/local_binding_group.rs"]
mod local_binding_group;
pub(crate) use local_binding_group::RootLifecyclePrefixBindingGroupV1;

#[path = "root_call_entry/lexical_terminal.rs"]
mod lexical_terminal;

#[path = "root_call_entry/expression_projection.rs"]
mod expression_projection;

#[path = "root_call_entry/print_expression_projection.rs"]
mod print_expression_projection;
pub(crate) use expression_projection::checked_expression_core_operation_v1;
#[cfg(test)]
pub(in crate::mir) use expression_projection::ExpressionPhysicalProjectionRefV1;
pub(crate) use print_expression_projection::EmittedPrintExpressionProjectionV1;

#[path = "root_call_entry/owned_array_append_projection.rs"]
mod owned_array_append_projection;
pub(crate) use owned_array_append_projection::{EmittedOwnedArrayAppendV1, OwnedArrayAppendCandidateV1};

#[path = "root_call_entry/bool_condition_projection.rs"]
mod bool_condition_projection;
pub(in crate::mir) use bool_condition_projection::{EmittedBoolConditionProjectionV1, BoolConditionSealV1, BoolConditionCursorV1, BoolConditionNodeProjectionV1, BoolConditionValueProofV1};

#[path = "root_call_entry/field_array_append_projection.rs"]
mod field_array_append_projection;
pub(crate) use field_array_append_projection::EmittedFieldArrayAppendV1;

#[path = "root_call_entry/borrowed_field_write_projection.rs"]
mod borrowed_field_write_projection;
pub(crate) use borrowed_field_write_projection::{EmittedBorrowedFieldWriteV1, CheckedFieldWriteGraphV1};
pub(in crate::mir) use borrowed_field_write_projection::{EmittedFieldWriteRhsV1, FieldWriteValueV1, FieldWriteRhsNodeV1, FieldWriteRhsProofV1};

#[path = "root_call_entry/initializer_expression_projection.rs"]
mod initializer_expression_projection;
