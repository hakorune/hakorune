//! Existing root exit progress retains the same affine Call source row.
//! Physical observations do not issue a target, result or cleanup obligation.
use super::*;
use crate::mir::normal_callable_semantic_package::RootCallDispositionV1;

impl OrdinaryNewClaimLedgerV1 {
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
            RootLocalCallBindingGroupV1::new(site, bindings, None)?,
        )
    }

    pub(crate) fn record_root_lexical_call_bindings(
        &self,
        owner: FunctionOwnerIdV1,
        site: OwnedExprSiteV1,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
        packet: EmittedLexicalCallProjectionV1,
    ) -> Result<(), String> {
        self.record_local_call_binding_group(
            owner,
            RootLocalCallBindingGroupV1::new(site, bindings, Some(std::rc::Rc::new(packet)))?,
        )
    }

    fn record_local_call_binding_group(
        &self,
        owner: FunctionOwnerIdV1,
        group: RootLocalCallBindingGroupV1,
    ) -> Result<(), String> {
        let site = group.site().clone();
        if site.owner() != owner {
            return Err(freeze("local-call-binding-owner-drift"));
        }
        let expected = self.expected_local_call_binding_sites(owner)?;
        let mut rows = self.root_local_call_bindings.borrow_mut();
        let groups = rows.entry(owner).or_default();
        if groups.iter().any(|recorded| recorded.site() == &site) {
            return Err(freeze("duplicate-local-call-bindings"));
        }
        if groups.len() >= expected.len() {
            return Err(freeze("local-call-bindings-overflow"));
        }
        if expected.get(groups.len()) != Some(&site) {
            return Err(freeze("local-call-binding-site-order"));
        }
        groups.push(group);
        Ok(())
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

    fn expected_local_call_binding_sites(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Result<Vec<OwnedExprSiteV1>, String> {
        let completion = self
            .completion_for_owner(owner)
            .ok_or_else(|| freeze("local-call-source-missing"))?;
        let flow = completion
            .cleanup()
            .root_flow()
            .ok_or_else(|| freeze("local-call-source-missing"))?;
        let routed = self.lifecycle_local_call_sites.borrow();
        let routed = routed.get(&owner);
        Ok(flow
            .local_calls()
            .iter()
            .filter(|call| {
                call.result()
                    == crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::I64
                    && routed.is_some_and(|sites| sites.contains(call.site()))
            })
            .map(|call| call.site().clone())
            .collect())
    }

    /// The routed lifecycle calls this exact exit covers, in `local_calls()`
    /// source order. Sibling exits sharing a prefix each name the same
    /// call sites; entry construction claims the recorded groups through
    /// `select_local_call_binding_groups` instead of draining the pool.
    fn expected_local_call_binding_sites_for_exit(
        &self,
        owner: FunctionOwnerIdV1,
        exit: &SourceStmtSiteV1,
    ) -> Result<Vec<OwnedExprSiteV1>, String> {
        let completion = self
            .completion_for_owner(owner)
            .ok_or_else(|| freeze("local-call-source-missing"))?;
        let flow = completion
            .cleanup()
            .root_flow()
            .ok_or_else(|| freeze("local-call-source-missing"))?;
        let Some(Ok(row)) = flow.exit_row(exit) else {
            return Err(freeze("local-call-source-missing"));
        };
        let covered = row.covered_calls();
        let routed = self.lifecycle_local_call_sites.borrow();
        let routed = routed.get(&owner);
        Ok(flow
            .local_calls()
            .iter()
            .filter(|call| {
                call.result()
                    == crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::I64
                    && covered.contains(call.site())
                    && routed.is_some_and(|sites| sites.contains(call.site()))
            })
            .map(|call| call.site().clone())
            .collect())
    }

    /// Select this exit's claimed binding-group rows from the recorded pool.
    /// The pending map stays whole: a call site covered by several exits
    /// must be claimable by each of them, and `local_call_bindings_consumed`
    /// is the final arbiter that nothing recorded was left unclaimed.
    pub(super) fn select_local_call_binding_groups(
        &self,
        owner: FunctionOwnerIdV1,
        exit: &SourceStmtSiteV1,
    ) -> Result<Vec<RootLocalCallBindingGroupV1>, String> {
        let expected = self.expected_local_call_binding_sites_for_exit(owner, exit)?;
        let pending = self.root_local_call_bindings.borrow();
        let recorded = pending.get(&owner).map(Vec::as_slice).unwrap_or(&[]);
        let mut selected = Vec::with_capacity(expected.len());
        for site in &expected {
            let Some(group) = recorded.iter().find(|recorded| recorded.site() == site) else {
                return Err(freeze("local-call-binding-sequence"));
            };
            selected.push(group.clone());
        }
        Ok(selected)
    }

    pub(super) fn validate_local_call_binding_groups(
        &self,
        owner: FunctionOwnerIdV1,
        exit: &SourceStmtSiteV1,
        groups: &[RootLocalCallBindingGroupV1],
    ) -> Result<(), String> {
        let expected = self.expected_local_call_binding_sites_for_exit(owner, exit)?;
        if groups.len() != expected.len()
            || groups.iter().zip(expected.iter()).any(|(group, expected)| {
                group.site().owner() != owner
                    || group.site() != expected
                    || group.bindings().is_empty()
            })
        {
            return Err(freeze("local-call-binding-sequence"));
        }
        Ok(())
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
        if let RootCallDispositionV1::Lexical(packet) = &row {
            self.validate_lexical_terminal_packet(owner, site, packet)?;
            let (original_invoke, original_projection) = packet.outer_bindings();
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
        let Some(progress) = exits.get_mut(&(owner, site.clone())) else {
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

    pub(crate) fn take_finalized_root_call(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceStmtSiteV1,
    ) -> Result<Option<(RootHomeExitEntry, Vec<(BasicBlockId, MirInstruction)>)>, String> {
        let mut exits = self.root_exits.borrow_mut();
        let Some(progress) = exits.get_mut(&(owner, site.clone())) else {
            return Ok(None);
        };
        let state = std::mem::replace(progress, RootHomeExitProgress::Finalized);
        match state {
            RootHomeExitProgress::Emitted {
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
            } => Ok(Some((
                RootHomeExitEntry::Call {
                    row,
                    local_bindings,
                    arguments,
                    invoke,
                    projection,
                    frame,
                },
                bindings,
            ))),
            RootHomeExitProgress::Emitted {
                origins,
                bindings,
                entry: RootHomeExitEntry::Plain { local_bindings },
            } => {
                *progress = RootHomeExitProgress::Emitted {
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
            RootHomeExitProgress::Finalized => Err(freeze("root-call-already-finalized")),
            other => {
                *progress = other;
                Err(freeze("root-call-entry-unavailable"))
            }
        }
    }

    /// Match one exit's claimed lifecycle local-call binding groups against
    /// the finished function: the source-ordered site sequence first, then
    /// every recorded instruction at its projected block.
    pub(super) fn check_local_call_binding_groups(
        &self,
        owner: FunctionOwnerIdV1,
        exit: &SourceStmtSiteV1,
        function: &MirFunction,
        finishing: Option<&super::super::physical_boundary::FinishedBindings>,
        groups: &[RootLocalCallBindingGroupV1],
    ) -> Result<(), String> {
        self.validate_local_call_binding_groups(owner, exit, groups)?;
        for group in groups {
            for (id, instruction) in group.bindings() {
                if !super::super::physical_boundary::check_binding(
                    function,
                    finishing,
                    *id,
                    instruction,
                )? {
                    return Err(freeze("local-call-binding-drift"));
                }
            }
        }
        Ok(())
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
    EmittedLexicalCallProjectionV1, LexicalCallArgumentProjectionV1,
    PreparedLexicalCallProjectionV1,
};

#[path = "root_call_entry/local_binding_group.rs"]
mod local_binding_group;
pub(crate) use local_binding_group::RootLocalCallBindingGroupV1;

#[path = "root_call_entry/lexical_terminal.rs"]
mod lexical_terminal;
