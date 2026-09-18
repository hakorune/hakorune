//! Existing owner-indexed root Home progress and physical emission validation.
//!
//! `RootHomeExitProgress` is the sole owner of the retained Home binding,
//! completion exit, object, and physical value until selected emission has
//! recorded and final validation has checked the release operation.

use super::*;

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package) enum RootHomeExitProgress {
    Unprepared,
    Unavailable,
    Prepared(Vec<RootHomeReleaseOriginV1>),
    Emitting,
    Emitted {
        origins: Vec<RootHomeReleaseEmissionV1>,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
        entry: RootHomeExitEntry,
    },
    Finalized,
}

#[derive(Debug)]
pub(crate) enum RootHomeExitEntry {
    /// A non-call terminal exit. `local_bindings` carries the source-ordered
    /// lifecycle binding groups recorded by local Call rows under this owner;
    /// the Plain exit owns them directly because no terminal Call entry
    /// exists to carry them.
    Plain {
        local_bindings: Vec<(OwnedExprSiteV1, Vec<(BasicBlockId, MirInstruction)>)>,
    },
    Call {
        row: crate::mir::normal_callable_semantic_package::RootCallDispositionV1,
        local_bindings: Vec<(OwnedExprSiteV1, Vec<(BasicBlockId, MirInstruction)>)>,
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
        local_bindings: Vec<(OwnedExprSiteV1, Vec<(BasicBlockId, MirInstruction)>)>,
        invoke: (BasicBlockId, MirInstruction),
        projection: (BasicBlockId, MirInstruction),
        frame: (BasicBlockId, MirInstruction),
    },
}

/// What one root-exit release operation reclaims. A `Binding` is a live
/// local Home bound before the terminal expression; an `ArgumentMap` is a
/// `%{...}` call-argument map constructed inside it — the caller keeps the
/// lease through the borrowed callee call and Ends it on both exit paths.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum RootHomeReleaseSubjectV1 {
    Binding(BindingRefV1),
    ArgumentMap { site: OwnedExprSiteV1, ordinal: u32 },
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
            RootHomeReleaseSubjectV1::Binding(binding) => Some(binding),
            RootHomeReleaseSubjectV1::ArgumentMap { .. } => None,
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

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn root_home_exit_is_complete(
        &self,
    ) -> bool {
        let exits = self.root_exits.borrow();
        let indexed_pending = self.completion_index.values().any(|row| {
            row.as_ref().ok().is_some_and(|completion| {
                matches!(completion.cleanup().terminal_homes(), Some(Ok(_)))
                    && !matches!(
                        exits.get(&completion.owner()),
                        Some(
                            RootHomeExitProgress::Unavailable
                                | RootHomeExitProgress::Emitted { .. }
                                | RootHomeExitProgress::Finalized
                        )
                    )
            })
        });
        let root_pending = self
            .root_completion
            .as_ref()
            .and_then(|row| row.as_ref().ok())
            .is_some_and(|completion| {
                matches!(completion.cleanup().terminal_homes(), Some(Ok(_)))
                    && !matches!(
                        exits.get(&completion.owner()),
                        Some(
                            RootHomeExitProgress::Unavailable
                                | RootHomeExitProgress::Emitted { .. }
                                | RootHomeExitProgress::Finalized,
                        )
                    )
            });
        !indexed_pending && !root_pending
    }

    pub(crate) fn prepare_root_home_exit(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceNodeSiteV1,
    ) -> Result<bool, String> {
        let Some(completion) = self.completion_for_owner(owner) else {
            return Ok(false);
        };
        let Some(Ok(homes)) = completion.cleanup().terminal_homes() else {
            return Ok(false);
        };
        if !completion
            .explicit_site()
            .is_some_and(|expected| expected.node() == site)
        {
            return Err(freeze("root-exit-site-mismatch"));
        }
        let mut exits = self.root_exits.borrow_mut();
        let progress = exits
            .entry(owner)
            .or_insert(RootHomeExitProgress::Unprepared);
        if !matches!(*progress, RootHomeExitProgress::Unprepared) {
            return Err(freeze("duplicate-root-exit-prepare"));
        }
        let rows = self.local_commits.borrow();
        let exit = completion
            .explicit_site()
            .expect("checked explicit root exit")
            .clone();
        let mut origins = Vec::new();
        let mut available = true;
        for binding in homes {
            let row = installed_home(&rows, *binding).map_err(|error| match error {
                HomeLookupError::Missing => freeze("root-home-not-installed"),
                HomeLookupError::Duplicate => freeze("duplicate-root-home"),
            })?;
            available &= row.end_available();
            origins.push(RootHomeReleaseOriginV1 {
                subject: RootHomeReleaseSubjectV1::Binding(*binding),
                exit: exit.clone(),
                operation: row.end_operation(),
            });
        }
        *progress = if available {
            RootHomeExitProgress::Prepared(origins)
        } else {
            RootHomeExitProgress::Unavailable
        };
        Ok(available)
    }

    pub(crate) fn begin_root_home_exit(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Result<Vec<RootHomeReleaseOriginV1>, String> {
        let mut exits = self.root_exits.borrow_mut();
        let progress = exits
            .get_mut(&owner)
            .ok_or_else(|| freeze("root-exit-not-prepared"))?;
        if !matches!(*progress, RootHomeExitProgress::Prepared(_)) {
            return Err(freeze("root-exit-not-prepared"));
        }
        let RootHomeExitProgress::Prepared(operands) =
            std::mem::replace(&mut *progress, RootHomeExitProgress::Emitting)
        else {
            unreachable!()
        };
        drop(exits);
        // Call-argument maps are the youngest caller-owned resources: they
        // are constructed inside the terminal expression, the callee borrows
        // their storage for the call's duration, and the caller Ends them on
        // both exit paths before any older Home. Their leases exist only
        // after argument emission, so the origins attach here — never at
        // prepare time and never re-derived from MIR.
        let mut operations = Vec::new();
        if let Some((completion, terminal)) = self.call_source_completion_for_owner(owner) {
            if completion.owner() == owner && terminal.owner() == owner {
                let exit = completion
                    .explicit_site()
                    .ok_or_else(|| freeze("root-exit-source-missing"))?;
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
                for (ordinal, site) in argument_maps {
                    let Some(LocalCommitV1::Map(row)) = rows.get(site) else {
                        return Err(freeze("root-exit-argument-map-missing"));
                    };
                    if site.owner() != owner || row.binding.is_some() {
                        return Err(freeze("root-exit-argument-map-drift"));
                    }
                    let flow = self.map_flow(site)?;
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
                            site: site.clone(),
                            ordinal,
                        },
                        exit: exit.clone(),
                        operation: InvokeOperation::Map(
                            crate::mir::instruction::MapInvokeOperation::End { map },
                        ),
                    });
                }
            }
        }
        operations.extend(operands);
        Ok(operations)
    }

    pub(crate) fn record_root_home_exit(
        &self,
        owner: FunctionOwnerIdV1,
        origins: Vec<(RootHomeReleaseOriginV1, BasicBlockId, MirInstruction)>,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
    ) -> Result<(), String> {
        // A Plain exit still owns the lifecycle local-call binding groups
        // recorded under this owner: they move into the entry so final
        // validation can match them against the finished function.
        let mut pending = self.root_local_call_bindings.borrow_mut();
        let pending_groups = pending.get(&owner).map(Vec::as_slice).unwrap_or(&[]);
        self.validate_local_call_binding_groups(owner, pending_groups)?;
        let local_bindings = pending.remove(&owner).unwrap_or_default();
        drop(pending);
        self.record_root_home_exit_with_entry(
            owner,
            origins,
            bindings,
            RootHomeExitEntry::Plain { local_bindings },
        )
    }

    fn record_root_home_exit_with_entry(
        &self,
        owner: FunctionOwnerIdV1,
        origins: Vec<(RootHomeReleaseOriginV1, BasicBlockId, MirInstruction)>,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
        entry: RootHomeExitEntry,
    ) -> Result<(), String> {
        let mut exits = self.root_exits.borrow_mut();
        let progress = exits
            .get_mut(&owner)
            .ok_or_else(|| freeze("root-exit-record-without-prepare"))?;
        if !matches!(*progress, RootHomeExitProgress::Emitting) || bindings.is_empty() {
            return Err(freeze("root-exit-record-without-emission"));
        }
        let origins = origins
            .into_iter()
            .map(|(origin, block, instruction)| RootHomeReleaseEmissionV1 {
                origin,
                block,
                instruction,
            })
            .collect();
        *progress = RootHomeExitProgress::Emitted {
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
    ) -> Result<Vec<(BasicBlockId, MirInstruction)>, String> {
        let Some(completion) = self.completion_for_owner(owner) else {
            return Ok(Vec::new());
        };
        if !matches!(completion.cleanup().terminal_homes(), Some(Ok(_))) {
            return Ok(Vec::new());
        }
        let expected_exit = completion
            .explicit_site()
            .ok_or_else(|| freeze("root-exit-source-missing"))?;
        let Some(Ok(expected_homes)) = completion.cleanup().terminal_homes() else {
            return Ok(Vec::new());
        };
        let exits = self.root_exits.borrow();
        match exits.get(&owner) {
            Some(RootHomeExitProgress::Unavailable) => Ok(Vec::new()),
            Some(RootHomeExitProgress::Emitted {
                origins,
                bindings,
                entry,
            }) => {
                self.validate_call_entry(owner, function, projection, entry, bindings)?;
                // Call-argument maps precede the binding origins: they are
                // emitted inside the terminal expression (youngest), in
                // descending argument order. Their expected sequence comes
                // from the sealed terminal Call, never from the MIR.
                let expected_argument_maps = self
                    .call_source_completion_for_owner(owner)
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
                if origins.len() != expected_homes.len() + expected_argument_maps.len() {
                    return Err(freeze("root-exit-origin-count"));
                }
                if projection
                    .is_some_and(|p| bindings.iter().any(|(id, _)| p.destination(*id).is_none()))
                {
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
                let expected_subjects = expected_argument_maps
                    .iter()
                    .map(|(ordinal, site)| RootHomeReleaseSubjectV1::ArgumentMap {
                        site: site.clone(),
                        ordinal: *ordinal,
                    })
                    .chain(
                        expected_homes
                            .iter()
                            .map(|binding| RootHomeReleaseSubjectV1::Binding(*binding)),
                    );
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
                Ok(mapped.unwrap_or_else(|| bindings.clone()))
            }
            _ => Err(freeze("root-exit-unconsumed")),
        }
    }
}

impl OrdinaryNewClaimLedgerV1 {
    pub(super) fn validate_root_cleanup_shape(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
    ) -> Result<(), String> {
        let exits = self.root_exits.borrow();
        if let Some(RootHomeExitProgress::Emitted {
            origins,
            bindings,
            entry,
        }) = exits.get(&owner)
        {
            match entry {
                RootHomeExitEntry::Plain { .. } if !origins.is_empty() => {
                    super::root_cleanup_graph::validate_original(function, bindings, origins.len())?
                }
                RootHomeExitEntry::Call {
                    invoke, projection, ..
                }
                | RootHomeExitEntry::MapGet {
                    invoke, projection, ..
                } => super::root_cleanup_graph::call::validate_original(
                    function,
                    bindings,
                    invoke,
                    projection,
                    &origins
                        .iter()
                        .map(|row| row.origin.operation())
                        .collect::<Vec<_>>(),
                )?,
                _ => {}
            }
        }
        Ok(())
    }
}

#[path = "root_call_entry.rs"]
mod call_entry;
#[path = "root_map_get_entry.rs"]
mod map_get_entry;
