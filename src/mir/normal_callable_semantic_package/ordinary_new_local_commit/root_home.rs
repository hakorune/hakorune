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
    Plain,
    Call {
        row: crate::mir::normal_callable_semantic_package::AppMainDirectCallDispositionRowV1,
        local_bindings: Vec<(BasicBlockId, MirInstruction)>,
        arguments: Vec<(BasicBlockId, MirInstruction)>,
        invoke: (BasicBlockId, MirInstruction),
        projection: (BasicBlockId, MirInstruction),
        frame: (BasicBlockId, MirInstruction),
    },
}

/// One source-issued root Home obligation after its existing local value has
/// been physically bound. The source binding and explicit return stay intact;
/// neither block identity nor an emitted instruction issues this origin.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RootHomeReleaseOriginV1 {
    binding: BindingRefV1,
    exit: crate::mir::resolved_semantics::SourceStmtSiteV1,
    operation: InvokeOperation,
}

impl RootHomeReleaseOriginV1 {
    pub(crate) const fn binding(&self) -> BindingRefV1 {
        self.binding
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
                binding: *binding,
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
        Ok(operands)
    }

    pub(crate) fn record_root_home_exit(
        &self,
        owner: FunctionOwnerIdV1,
        origins: Vec<(RootHomeReleaseOriginV1, BasicBlockId, MirInstruction)>,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
    ) -> Result<(), String> {
        self.record_root_home_exit_with_entry(owner, origins, bindings, RootHomeExitEntry::Plain)
    }

    fn record_root_home_exit_with_entry(
        &self,
        owner: FunctionOwnerIdV1,
        origins: Vec<(RootHomeReleaseOriginV1, BasicBlockId, MirInstruction)>,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
        entry: RootHomeExitEntry,
    ) -> Result<(), String> {
        if matches!(entry, RootHomeExitEntry::Plain)
            && self
                .root_local_call_bindings
                .borrow()
                .get(&owner)
                .is_some_and(|bindings| !bindings.is_empty())
        {
            return Err(freeze("local-call-without-terminal"));
        }
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
                if origins.len() != expected_homes.len() {
                    return Err(freeze("root-exit-origin-count"));
                }
                if projection
                    .is_some_and(|p| bindings.iter().any(|(id, _)| p.destination(*id).is_none()))
                {
                    return Err(freeze("root-cleanup-graph/residual-node"));
                }
                let mapped = projection.map(|p| p.bindings(bindings)).transpose()?;
                if let (RootHomeExitEntry::Plain, Some(projection), Some(mapped)) =
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
                for (emitted, expected_binding) in origins.iter().zip(expected_homes) {
                    if emitted.origin.binding() != *expected_binding
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
                RootHomeExitEntry::Plain if !origins.is_empty() => {
                    super::root_cleanup_graph::validate_original(function, bindings, origins.len())?
                }
                RootHomeExitEntry::Call {
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
