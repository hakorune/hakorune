//! Existing owner-indexed root Home progress and physical emission validation.
//!
//! `RootHomeExitProgress` is the sole owner of the retained Home binding,
//! completion exit, object, and physical value until selected emission has
//! recorded and final validation has checked the release operation.

use super::*;
#[path = "root_home_cleanup_order.rs"]
mod cleanup_order;
pub(in crate::mir::normal_callable_semantic_package) use cleanup_order::RootHomeCleanupOrderV1;
#[path = "root_home_null_return.rs"]
mod null_return;
pub(crate) use null_return::TerminalNullReturnSourceLoanV1;
#[path = "root_home_fresh_return.rs"]
mod fresh_return;
#[path = "root_home_received_return.rs"]
mod received_return;

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package) enum RootHomeExitProgress {
    Unprepared,
    Unavailable,
    Prepared(RootHomeCleanupOrderV1),
    Emitting(RootHomeCleanupOrderV1),
    Emitted {
        order: RootHomeCleanupOrderV1,
        origins: Vec<RootHomeReleaseEmissionV1>,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
        entry: RootHomeExitEntry,
    },
    Finalized(RootHomeCleanupOrderV1),
}

#[derive(Debug)]
pub(crate) enum RootHomeExitEntry {
    /// A non-call terminal exit. `local_bindings` carries the source-ordered
    /// lifecycle binding groups recorded by local Call rows under this owner;
    /// the Plain exit owns them directly because no terminal Call entry
    /// exists to carry them.
    Plain {
        local_bindings: Vec<super::RootLocalCallBindingGroupV1>,
    },
    Call {
        row: crate::mir::normal_callable_semantic_package::RootCallDispositionV1,
        local_bindings: Vec<super::RootLocalCallBindingGroupV1>,
        arguments: Vec<(BasicBlockId, MirInstruction)>,
        invoke: (BasicBlockId, MirInstruction),
        projection: (BasicBlockId, MirInstruction),
        frame: (BasicBlockId, MirInstruction),
    },
    /// A terminal `return <map>.get("<literal>")` exit. The read invoke and
    /// its i64 projection are retained exactly like a Call ingress; there is
    /// no target row or argument list because the key is sealed inline and
    /// the receiver binding is resolved at prepare time.
    MapGet {
        local_bindings: Vec<super::RootLocalCallBindingGroupV1>,
        invoke: (BasicBlockId, MirInstruction),
        projection: (BasicBlockId, MirInstruction),
        frame: (BasicBlockId, MirInstruction),
    },
}

impl RootHomeExitEntry {
    /// The lifecycle local-call binding groups this exact exit claimed.
    pub(in crate::mir::normal_callable_semantic_package) fn local_call_groups(
        &self,
    ) -> &[super::RootLocalCallBindingGroupV1] {
        match self {
            Self::Plain { local_bindings }
            | Self::Call { local_bindings, .. }
            | Self::MapGet { local_bindings, .. } => local_bindings,
        }
    }
}

/// What one root-exit release operation reclaims. A `Binding` is a live
/// local Home bound before the terminal expression; an `ArgumentMap` is a
/// `%{...}` call-argument map constructed inside it — the caller keeps the
/// lease through the borrowed callee call and Ends it on both exit paths.
/// A `FieldResidence` is one owned `ArrayBox` child of a `Binding` home,
/// released in reverse declaration order before that Home's own release.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum RootHomeReleaseSubjectV1 {
    Binding(BindingRefV1),
    DirectResult {
        site: OwnedExprSiteV1,
    },
    DirectResultField {
        site: OwnedExprSiteV1,
        field: hakorune_mir_defs::CanonicalFieldRefV1,
    },
    ArgumentMap {
        site: OwnedExprSiteV1,
        ordinal: u32,
    },
    FieldResidence {
        binding: BindingRefV1,
        field: hakorune_mir_defs::CanonicalFieldRefV1,
    },
}

/// One source-issued root Home obligation after its existing local value has
/// been physically bound. The source binding and explicit return stay intact;
/// neither block identity nor an emitted instruction issues this origin.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RootHomeReleaseOriginV1 {
    subject: RootHomeReleaseSubjectV1,
    exit: crate::mir::resolved_semantics::SourceStmtSiteV1,
    operation: InvokeOperation,
}

impl RootHomeReleaseOriginV1 {
    pub(crate) const fn subject(&self) -> &RootHomeReleaseSubjectV1 {
        &self.subject
    }

    pub(crate) fn binding(&self) -> Option<BindingRefV1> {
        match self.subject {
            RootHomeReleaseSubjectV1::Binding(binding)
            | RootHomeReleaseSubjectV1::FieldResidence { binding, .. } => Some(binding),
            RootHomeReleaseSubjectV1::ArgumentMap { .. }
            | RootHomeReleaseSubjectV1::DirectResult { .. }
            | RootHomeReleaseSubjectV1::DirectResultField { .. } => None,
        }
    }

    pub(crate) fn exit(&self) -> &crate::mir::resolved_semantics::SourceStmtSiteV1 {
        &self.exit
    }

    pub(crate) fn operation(&self) -> &InvokeOperation {
        &self.operation
    }
}

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package) struct RootHomeReleaseEmissionV1 {
    origin: RootHomeReleaseOriginV1,
    block: BasicBlockId,
    instruction: MirInstruction,
}

impl RootHomeReleaseEmissionV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn origin(
        &self,
    ) -> &RootHomeReleaseOriginV1 {
        &self.origin
    }
}

/// Temporary immutable emission projection of the SAME retained root order.
/// Indices identify original obligations, never operation-payload equivalence.
pub(crate) struct RootHomeFaultSequencesV1 {
    pub(crate) full: Vec<RootHomeReleaseOriginV1>,
    pub(crate) acquisition: Vec<usize>,
    pub(crate) after_normal: Vec<Vec<usize>>,
}

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn root_home_exit_is_complete(
        &self,
    ) -> bool {
        let exits = self.root_exits.borrow();
        let pending =
            |completion: &crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1| {
                completion.cleanup().root_flow().is_some_and(|flow| {
                    flow.all_exits_ready()
                        && !completion.explicit_sites().iter().all(|site| {
                            matches!(
                                exits.get(&(completion.owner(), site.clone())),
                                Some(
                                    RootHomeExitProgress::Unavailable
                                        | RootHomeExitProgress::Emitted { .. }
                                        | RootHomeExitProgress::Finalized(_)
                                )
                            )
                        })
                })
            };
        let indexed_pending = self.completion_index.values().any(|row| {
            row.as_ref()
                .ok()
                .is_some_and(|completion| pending(completion))
        });
        let root_pending = self
            .root_completion
            .as_ref()
            .and_then(|row| row.as_ref().ok())
            .is_some_and(|completion| pending(completion));
        !indexed_pending && !root_pending
    }

    /// Every recorded lifecycle local-call binding group must be claimed by
    /// at least one emitted exit entry, or sit on paths whose exits are all
    /// retained `Unavailable`. A recorded-but-never-claimed site is physical
    /// evidence nobody consumed — a violation, not a benign leftover.
    pub(in crate::mir::normal_callable_semantic_package) fn local_call_bindings_consumed(
        &self,
    ) -> bool {
        let pending = self.root_local_call_bindings.borrow();
        let exits = self.root_exits.borrow();
        pending.iter().all(|(owner, groups)| {
            groups.iter().all(|group| {
                let site = group.site();
                let Some(completion) = self.completion_for_owner(*owner) else {
                    return false;
                };
                let Some(flow) = completion.cleanup().root_flow() else {
                    return false;
                };
                completion.explicit_sites().iter().any(|exit| {
                    let covers = flow
                        .exit_row(exit)
                        .and_then(|row| row.ok())
                        .is_some_and(|row| row.covered_calls().contains(site));
                    if !covers {
                        return false;
                    }
                    match exits.get(&(*owner, exit.clone())) {
                        Some(RootHomeExitProgress::Emitted { entry, .. }) => entry
                            .local_call_groups()
                            .iter()
                            .any(|group| group.site() == site),
                        Some(RootHomeExitProgress::Unavailable)
                        | Some(RootHomeExitProgress::Finalized(_)) => true,
                        _ => false,
                    }
                })
            })
        })
    }

    pub(crate) fn prepare_root_home_exit(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceNodeSiteV1,
    ) -> Result<bool, String> {
        if !self.object_return_construction_ready_v1(owner)? {
            return Ok(false);
        }
        let Some(completion) = self.completion_for_owner(owner) else {
            return Ok(false);
        };
        let Some(flow) = completion.cleanup().root_flow() else {
            return Ok(false);
        };
        // Per-exit admission is all-or-nothing per function: a sibling exit
        // whose row is unavailable keeps this exit on the generic path too,
        // so no `return` can silently skip proven release evidence.
        if !flow.all_exits_ready() {
            return Ok(false);
        }
        let exit = SourceStmtSiteV1::from_node(site.clone());
        if !completion.explicit_sites().contains(&exit) {
            return Err(freeze("root-exit-site-mismatch"));
        }
        let Some(row) = self.normal_exit_projection_v1(owner, &exit)? else {
            return Ok(false);
        };
        let mut exits = self.root_exits.borrow_mut();
        let progress = exits
            .entry((owner, exit.clone()))
            .or_insert(RootHomeExitProgress::Unprepared);
        if !matches!(*progress, RootHomeExitProgress::Unprepared) {
            return Err(freeze("duplicate-root-exit-prepare"));
        }
        let rows = self.local_commits.borrow();
        let order = RootHomeCleanupOrderV1::from_home_end_plans(
            &rows,
            &exit,
            row.fault_homes(),
            row.homes(),
        )?;
        let available = order.is_some();
        *progress = match order {
            Some(mut order) => {
                if let Some(view) = self.verified_direct_object_return_source_v1(owner, &exit)? {
                    let Some(source) = view.cleanup_source() else {
                        *progress = RootHomeExitProgress::Unavailable;
                        return Ok(false);
                    };
                    order.retain_direct_source(source)?;
                }
                RootHomeExitProgress::Prepared(order)
            }
            None => RootHomeExitProgress::Unavailable,
        };
        Ok(available)
    }

    pub(crate) fn bind_root_direct_result_v1(
        &self,
        owner: FunctionOwnerIdV1,
        exit: &SourceStmtSiteV1,
        value: ValueId,
        kind: crate::mir::instruction::InvokeCallResultKind,
    ) -> Result<(), String> {
        let mut exits = self.root_exits.borrow_mut();
        let Some(RootHomeExitProgress::Emitting(order)) = exits.get_mut(&(owner, exit.clone()))
        else {
            return Err(freeze("direct-result-not-emitting"));
        };
        order.bind_direct_result(value, kind)
    }

    pub(crate) fn root_home_fault_sequences(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceStmtSiteV1,
    ) -> Result<RootHomeFaultSequencesV1, String> {
        let exits = self.root_exits.borrow();
        let Some(RootHomeExitProgress::Emitting(order)) = exits.get(&(owner, site.clone())) else {
            return Err(freeze("root-exit-not-emitting"));
        };
        Ok(RootHomeFaultSequencesV1 {
            full: order.full().to_vec(),
            acquisition: order.acquisition_fault_indices().to_vec(),
            after_normal: (0..order.normal().len())
                .map(|index| order.fault_indices_after_normal_step(index))
                .collect::<Result<_, _>>()?,
        })
    }

    pub(crate) fn begin_root_home_exit(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceStmtSiteV1,
    ) -> Result<Vec<RootHomeReleaseOriginV1>, String> {
        let mut exits = self.root_exits.borrow_mut();
        let progress = exits
            .get_mut(&(owner, site.clone()))
            .ok_or_else(|| freeze("root-exit-not-prepared"))?;
        if !matches!(*progress, RootHomeExitProgress::Prepared(_)) {
            return Err(freeze("root-exit-not-prepared"));
        }
        let RootHomeExitProgress::Prepared(order) =
            std::mem::replace(&mut *progress, RootHomeExitProgress::Unprepared)
        else {
            unreachable!()
        };
        *progress = RootHomeExitProgress::Emitting(order);
        drop(exits);
        // Call-argument maps are the youngest caller-owned resources: they
        // are constructed inside the terminal expression, the callee borrows
        // their storage for the call's duration, and the caller Ends them on
        // both exit paths before any older Home. Their leases exist only
        // after argument emission, so the origins attach here — never at
        // prepare time and never re-derived from MIR.
        let mut operations = Vec::new();
        if let Some((completion, terminal)) = self.call_source_completion_for_owner_at(owner, site)
        {
            if completion.owner() == owner && terminal.owner() == owner {
                let rows = self.local_commits.borrow();
                let mut argument_maps: Vec<(u32, &OwnedExprSiteV1)> = terminal
                    .arguments()
                    .iter()
                    .enumerate()
                    .filter_map(|(ordinal, argument)| {
                        argument.map_site().map(|site| (ordinal as u32, site))
                    })
                    .collect();
                argument_maps.sort_by_key(|(ordinal, _)| std::cmp::Reverse(*ordinal));
                for (ordinal, map_site) in argument_maps {
                    let Some(LocalCommitV1::Map(row)) = rows.get(map_site) else {
                        return Err(freeze("root-exit-argument-map-missing"));
                    };
                    if map_site.owner() != owner || row.binding.is_some() {
                        return Err(freeze("root-exit-argument-map-drift"));
                    }
                    let flow = self.map_flow(map_site)?;
                    if !matches!(
                        flow.destination(),
                        crate::mir::resolved_semantics::home_new_prefix::MapDestinationV1::CallArgument {
                            call,
                            ordinal: expected,
                        } if call.site() == terminal.call_site() && *expected == ordinal
                    ) {
                        return Err(freeze("root-exit-argument-map-drift"));
                    }
                    let map = row
                        .emitted_value()
                        .ok_or_else(|| freeze("root-exit-argument-map-missing"))?;
                    operations.push(RootHomeReleaseOriginV1 {
                        subject: RootHomeReleaseSubjectV1::ArgumentMap {
                            site: map_site.clone(),
                            ordinal,
                        },
                        exit: site.clone(),
                        operation: InvokeOperation::Map(
                            crate::mir::instruction::MapInvokeOperation::End { map },
                        ),
                    });
                }
            }
        }
        let mut exits = self.root_exits.borrow_mut();
        let Some(RootHomeExitProgress::Emitting(order)) = exits.get_mut(&(owner, site.clone()))
        else {
            return Err(freeze("root-exit-not-emitting"));
        };
        order.prepend_argument_maps(operations)?;
        let operations = order.normal();
        Ok(operations)
    }

    pub(crate) fn record_root_home_exit(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceStmtSiteV1,
        origins: Vec<(RootHomeReleaseOriginV1, BasicBlockId, MirInstruction)>,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
    ) -> Result<(), String> {
        // A Plain exit claims exactly the lifecycle local-call binding
        // groups this exit's `covered_calls` names — never the whole owner
        // pool, which sibling exits still need.
        let local_bindings = self.select_local_call_binding_groups(owner, site)?;
        self.record_root_home_exit_with_entry(
            owner,
            site,
            origins,
            bindings,
            RootHomeExitEntry::Plain { local_bindings },
        )
    }

    fn record_root_home_exit_with_entry(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceStmtSiteV1,
        origins: Vec<(RootHomeReleaseOriginV1, BasicBlockId, MirInstruction)>,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
        entry: RootHomeExitEntry,
    ) -> Result<(), String> {
        let mut exits = self.root_exits.borrow_mut();
        let progress = exits
            .get_mut(&(owner, site.clone()))
            .ok_or_else(|| freeze("root-exit-record-without-prepare"))?;
        if !matches!(*progress, RootHomeExitProgress::Emitting(_)) || bindings.is_empty() {
            return Err(freeze("root-exit-record-without-emission"));
        }
        let RootHomeExitProgress::Emitting(order) = &*progress else {
            unreachable!()
        };
        if order.normal()
            != origins
                .iter()
                .map(|(origin, _, _)| origin.clone())
                .collect::<Vec<_>>()
        {
            return Err(freeze("root-exit-origin-order"));
        }
        order.validate_direct_entry(&entry, &bindings, None)?;
        if self.terminal_relation_index.get(&owner).and_then(|rows| rows.get(site)).is_some_and(|row| {
            matches!(row, TerminalRelationV1::Value(value) if matches!(value.returned(), TerminalReturnedSourceV1::NullLiteral))
        }) && order.null_return_binding().is_none() {
            return Err(freeze("terminal-null/producer-missing"));
        }
        order.validate_null_return(&entry, &bindings, None, None)?;
        self.validate_received_return_producer_v1(owner, site, &entry, &bindings, None, None)?;
        self.validate_fresh_return_producer_v1(owner, site, &entry, &bindings, None, None, None)?;
        let RootHomeExitProgress::Emitting(order) =
            std::mem::replace(progress, RootHomeExitProgress::Unprepared)
        else {
            unreachable!()
        };
        let origins = origins
            .into_iter()
            .map(|(origin, block, instruction)| RootHomeReleaseEmissionV1 {
                origin,
                block,
                instruction,
            })
            .collect();
        *progress = RootHomeExitProgress::Emitted {
            order,
            origins,
            bindings,
            entry,
        };
        Ok(())
    }

    pub(super) fn validate_root_home_exit(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
        projection: Option<&super::physical_boundary::FinishedBindings>,
    ) -> Result<(), String> {
        if !self.object_return_construction_ready_v1(owner)? {
            return Ok(());
        }
        let Some(completion) = self.completion_for_owner(owner) else {
            return Ok(());
        };
        let Some(flow) = completion.cleanup().root_flow() else {
            return Ok(());
        };
        if !flow.all_exits_ready() {
            return Ok(());
        }
        let exits = self.root_exits.borrow();
        for expected_exit in completion.explicit_sites() {
            let Some(exit_row) = self.normal_exit_projection_v1(owner, expected_exit)? else {
                return Err(freeze("root-exit-source-missing"));
            };
            let expected_homes = exit_row.homes();
            let Some(progress) = exits.get(&(owner, expected_exit.clone())) else {
                return Err(freeze("root-exit-unconsumed"));
            };
            match progress {
                RootHomeExitProgress::Unavailable => {}
                RootHomeExitProgress::Emitted {
                    order,
                    origins,
                    bindings,
                    entry,
                } => {
                    if order.normal()
                        != origins
                            .iter()
                            .map(|row| row.origin.clone())
                            .collect::<Vec<_>>()
                    {
                        return Err(freeze("root-exit-origin-order"));
                    }
                    self.validate_call_entry(
                        owner,
                        expected_exit,
                        function,
                        projection,
                        entry,
                        bindings,
                    )?;
                    // Call-argument maps precede the binding origins: they are
                    // emitted inside the terminal expression (youngest), in
                    // descending argument order. Their expected sequence comes
                    // from the sealed terminal Call, never from the MIR.
                    let expected_argument_maps = self
                        .call_source_completion_for_owner_at(owner, expected_exit)
                        .filter(|(completion, _)| completion.owner() == owner)
                        .map(|(_, terminal)| {
                            terminal
                                .arguments()
                                .iter()
                                .enumerate()
                                .filter_map(|(ordinal, argument)| {
                                    argument
                                        .map_site()
                                        .map(|site| (ordinal as u32, site.clone()))
                                })
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_default();
                    let mut expected_argument_maps = expected_argument_maps;
                    expected_argument_maps.sort_by_key(|(ordinal, _)| std::cmp::Reverse(*ordinal));
                    if projection.is_some_and(|p| {
                        bindings.iter().any(|(id, _)| p.destination(*id).is_none())
                    }) {
                        return Err(freeze("root-cleanup-graph/residual-node"));
                    }
                    let mapped = projection.map(|p| p.bindings(bindings)).transpose()?;
                    if let (RootHomeExitEntry::Plain { .. }, Some(projection), Some(mapped)) =
                        (entry, projection, mapped.as_ref())
                    {
                        if let Some((entry, _)) = bindings.last() {
                            super::root_cleanup_graph::validate_projected_ingress(
                                function,
                                mapped,
                                projection
                                    .destination(*entry)
                                    .expect("checked root mapping"),
                            )?;
                        }
                    }
                    let rows = self.local_commits.borrow();
                    let mut expected_subjects: Vec<RootHomeReleaseSubjectV1> =
                        expected_argument_maps
                            .iter()
                            .map(|(ordinal, site)| RootHomeReleaseSubjectV1::ArgumentMap {
                                site: site.clone(),
                                ordinal: *ordinal,
                            })
                            .collect();
                    // Each installed Home expands to its sealed teardown
                    // plan's subjects: owned `ArrayBox` residences precede
                    // the Home's own Binding release.
                    for binding in expected_homes {
                        let home =
                            installed_home(&rows, *binding).map_err(|error| match error {
                                HomeLookupError::Missing => freeze("root-home-not-installed"),
                                HomeLookupError::Duplicate => freeze("duplicate-root-home"),
                            })?;
                        expected_subjects.extend(
                            home.end_plan()
                                .into_vec()
                                .into_iter()
                                .map(|(subject, _)| subject),
                        );
                    }
                    if origins.len() != expected_subjects.len() {
                        return Err(freeze("root-exit-origin-count"));
                    }
                    let expected_subjects = expected_subjects.into_iter();
                    for (emitted, expected_subject) in origins.iter().zip(expected_subjects) {
                        if emitted.origin.subject() != &expected_subject
                            || emitted.origin.exit() != expected_exit
                        {
                            return Err(freeze("root-exit-origin-drift"));
                        }
                        if !matches!(
                            &emitted.instruction,
                            MirInstruction::Invoke { operation, .. }
                                if operation == emitted.origin.operation()
                        ) {
                            return Err(freeze("root-exit-operation-drift"));
                        }
                        if bindings
                            .iter()
                            .filter(|(id, instruction)| {
                                *id == emitted.block && *instruction == emitted.instruction
                            })
                            .count()
                            != 1
                        {
                            return Err(freeze("root-exit-origin-binding-drift"));
                        }
                        let (actual_block, expected_instruction) = match projection {
                            Some(p) => p
                                .binding(emitted.block, &emitted.instruction)?
                                .ok_or_else(|| freeze("root-exit-origin-binding-drift"))?,
                            None => (emitted.block, emitted.instruction.clone()),
                        };
                        if !function.blocks.get(&actual_block).is_some_and(|block| {
                            block.all_instructions().any(|actual| {
                                matches!(
                                    actual,
                                    MirInstruction::Invoke { operation, .. }
                                        if operation == emitted.origin.operation()
                                )
                            })
                        }) {
                            return Err(freeze("root-exit-operation-drift"));
                        }
                        if !function.blocks.get(&actual_block).is_some_and(|block| {
                            block
                                .all_instructions()
                                .any(|actual| actual == &expected_instruction)
                        }) {
                            return Err(freeze("root-exit-origin-binding-drift"));
                        }
                    }
                    for (id, expected) in bindings {
                        if !super::physical_boundary::check_binding(
                            function, projection, *id, expected,
                        )? {
                            return Err(freeze("root-exit-binding-drift"));
                        }
                    }
                    order.validate_direct_entry(entry, bindings, projection)?;
                    order.validate_null_return(entry, bindings, projection, projection)?;
                    if let Some(producer) = self.validate_received_return_producer_v1(
                        owner,
                        expected_exit,
                        entry,
                        bindings,
                        projection,
                        projection,
                    )? {
                        // Projection precedes validate_complete's registration.
                        // Final collector checks mandatory membership after that
                        // existing owner has installed the recorded bindings.
                        if !super::physical_boundary::check_binding(
                            function,
                            None,
                            producer.0,
                            &producer.1,
                        )? {
                            return Err(freeze("received-return/producer-actual-drift"));
                        }
                    }
                    self.validate_fresh_return_producer_v1(
                        owner,
                        expected_exit,
                        entry,
                        bindings,
                        projection,
                        projection,
                        Some(function),
                    )?;
                    super::root_cleanup_graph::ordered_paths::validate(
                        function, bindings, entry, order, projection,
                    )?;
                    if let Some(projection) = projection {
                        super::root_cleanup_graph::ordered_structure::validate_projected(
                            function,
                            bindings,
                            entry,
                            projection,
                            order.ingress_result_kind(),
                        )?;
                    }
                    let _ = mapped;
                }
                _ => return Err(freeze("root-exit-unconsumed")),
            }
        }
        Ok(())
    }
}

impl OrdinaryNewClaimLedgerV1 {
    pub(super) fn validate_root_cleanup_shape(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
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
            super::root_cleanup_graph::ordered_paths::validate(
                function, bindings, entry, order, None,
            )?;
            super::root_cleanup_graph::ordered_structure::validate_original(
                function,
                bindings,
                entry,
                order.ingress_result_kind(),
            )?;
        }
        Ok(())
    }
}

#[path = "root_call_entry.rs"]
mod call_entry;
#[path = "root_map_get_entry.rs"]
mod map_get_entry;

pub(in crate::mir) use call_entry::{
    CallPacketSourceLoanV1, CallPacketSourceV1, EmittedLexicalCallProjectionV1,
    LexicalCallArgumentProjectionV1, PreparedLexicalCallProjectionV1,
};

pub(crate) use call_entry::RootLocalCallBindingGroupV1;
