//! Existing owner-indexed root Home progress and physical emission validation.
//!
//! `RootHomeExitProgress` is the sole owner of the retained Home binding,
//! completion exit, object, and physical value until selected emission has
//! recorded and final validation has checked the release operation.

use super::*;
use crate::mir::resolved_semantics::home_new_prefix::RootHomeExitKeyV1;
#[path = "root_home_cleanup_order.rs"]
mod cleanup_order;
#[path = "root_home/prepare.rs"]
mod prepare;
#[path = "root_home/implicit.rs"]
mod implicit;
#[path = "root_home/record.rs"]
mod record;
pub(crate) use prepare::{PreparedRootHomeGraphPlanV1, PreparedRootHomeOrderV1};
#[path = "root_home/pending.rs"]
mod pending;
pub(crate) use pending::PendingRootHomeExitV1;
pub(in crate::mir::normal_callable_semantic_package) use cleanup_order::{RootHomeCleanupOrderV1, source_home_end_plans_v1, source_home_end_selection_v2};
pub(crate) use cleanup_order::fault_indices_after_normal_step_v2;
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
        local_bindings: Vec<super::RootLifecyclePrefixBindingGroupV1>,
    },
    Call {
        row: crate::mir::normal_callable_semantic_package::RootCallDispositionV1,
        local_bindings: Vec<super::RootLifecyclePrefixBindingGroupV1>,
        arguments: Vec<(BasicBlockId, MirInstruction)>,
        invoke: (BasicBlockId, MirInstruction),
        projection: (BasicBlockId, MirInstruction),
        frame: (BasicBlockId, MirInstruction),
    },
    /// One checked terminal Map read, with its original Invoke and projection.
    /// The optional stored-object proof retains the exact get receiver/key scope;
    /// the existing readable-Map I64 relation keeps its sealed inline key.
    MapGet {
        stored: Option<Box<StoredMapGetReturnEmissionV1>>,
        local_bindings: Vec<super::RootLifecyclePrefixBindingGroupV1>,
        invoke: (BasicBlockId, MirInstruction),
        projection: (BasicBlockId, MirInstruction),
        frame: (BasicBlockId, MirInstruction),
    },
}

impl RootHomeExitEntry {
    /// The lifecycle local-call binding groups this exact exit claimed.
    pub(in crate::mir::normal_callable_semantic_package) fn local_call_groups(
        &self,
    ) -> &[super::RootLifecyclePrefixBindingGroupV1] {
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
    Text(crate::mir::resolved_semantics::home_new_prefix::TextResidenceAcquisitionV1),
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
    exit: RootHomeExitKeyV1,
    operation: InvokeOperation,
}

impl RootHomeReleaseOriginV1 {
    pub(crate) const fn subject(&self) -> &RootHomeReleaseSubjectV1 {
        &self.subject
    }

    pub(crate) fn binding(&self) -> Option<BindingRefV1> {
        match &self.subject {
            RootHomeReleaseSubjectV1::Binding(binding)
            | RootHomeReleaseSubjectV1::FieldResidence { binding, .. } => Some(*binding),
            RootHomeReleaseSubjectV1::Text(source) => Some(source.binding()),
            RootHomeReleaseSubjectV1::ArgumentMap { .. }
            | RootHomeReleaseSubjectV1::DirectResult { .. }
            | RootHomeReleaseSubjectV1::DirectResultField { .. } => None,
        }
    }

    pub(crate) fn exit(&self) -> &RootHomeExitKeyV1 {
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
    pub(in crate::mir::normal_callable_semantic_package) fn binding(&self) -> (BasicBlockId, MirInstruction) {
        (self.block, self.instruction.clone())
    }
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
                    self.root_home_flow_ready_v1(completion).unwrap_or(false)
                        && !completion.explicit_sites().iter().all(|site| {
                            matches!(
                                exits.get(&(completion.owner(), site.clone().into())),
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
        let implicit_pending = self.completion_index.keys().copied()
            .chain(self.root_completion.iter().filter_map(|row| row.as_ref().ok().map(|row| row.owner())))
            .any(|owner| match self.implicit_unit_completion_source_v1(owner) {
                Ok(Some(_)) => !matches!(exits.get(&(owner, RootHomeExitKeyV1::ImplicitBodyEnd)),
                    Some(RootHomeExitProgress::Emitted { .. })),
                Ok(None) => false,
                Err(_) => true,
            });
        !indexed_pending && !root_pending && !implicit_pending
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
                let expected = match self.expected_local_call_binding_sources(*owner) {
                    Ok(expected) => expected,
                    Err(_) => return false,
                };
                let Some(source) = expected.iter().find(|source| group.matches_source(source)) else {
                    return false;
                };
                let exits_to_check = completion.explicit_sites().iter().cloned().map(RootHomeExitKeyV1::Explicit)
                    .chain(completion.implicit_body_end().map(|_| RootHomeExitKeyV1::ImplicitBodyEnd))
                    .collect::<Vec<_>>();
                let consumed = exits_to_check.iter().any(|exit| {
                    let covers = match &exit {
                        RootHomeExitKeyV1::Explicit(site) => flow.exit_row(site),
                        RootHomeExitKeyV1::ImplicitBodyEnd => flow.implicit_exit(),
                    }.and_then(|row| row.ok())
                        .is_some_and(|row| source.covered_by(row));
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
                });
                consumed
            })
        })
    }

    pub(crate) fn prepare_root_home_exit(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceNodeSiteV1,
    ) -> Result<bool, String> {
        let preview = self.preview_root_home_exit_v1(owner, site)?;
        if matches!(preview, prepare::RootHomePreparationV1::NotSelected) {
            return Ok(false);
        }
        let exit = SourceStmtSiteV1::from_node(site.clone());
        let mut exits = self.root_exits.borrow_mut();
        let progress = exits
            .entry((owner, exit.clone().into()))
            .or_insert(RootHomeExitProgress::Unprepared);
        if !matches!(*progress, RootHomeExitProgress::Unprepared) {
            return Err(freeze("duplicate-root-exit-prepare"));
        }
        *progress = match preview {
            prepare::RootHomePreparationV1::NotSelected => unreachable!(),
            prepare::RootHomePreparationV1::Unavailable => {
                RootHomeExitProgress::Unavailable
            }
            prepare::RootHomePreparationV1::Prepared { order, .. } => {
                RootHomeExitProgress::Prepared(order)
            }
        };
        Ok(matches!(*progress, RootHomeExitProgress::Prepared(_)))
    }

    pub(crate) fn bind_root_direct_result_v1(
        &self,
        owner: FunctionOwnerIdV1,
        exit: &SourceStmtSiteV1,
        value: ValueId,
        kind: crate::mir::instruction::InvokeCallResultKind,
    ) -> Result<(), String> {
        let mut exits = self.root_exits.borrow_mut();
        let Some(RootHomeExitProgress::Emitting(order)) = exits.get_mut(&(owner, exit.clone().into()))
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
        self.root_home_fault_sequences_for_exit_v1(owner, &site.clone().into())
    }

    pub(crate) fn root_home_fault_sequences_for_exit_v1(
        &self, owner: FunctionOwnerIdV1, exit: &RootHomeExitKeyV1,
    ) -> Result<RootHomeFaultSequencesV1, String> {
        let exits = self.root_exits.borrow();
        let Some(RootHomeExitProgress::Emitting(order)) = exits.get(&(owner, exit.clone())) else {
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
        self.begin_root_home_exit_for_key_v1(owner, &site.clone().into())
    }

    pub(crate) fn begin_root_home_exit_for_key_v1(
        &self, owner: FunctionOwnerIdV1, exit: &RootHomeExitKeyV1,
    ) -> Result<Vec<RootHomeReleaseOriginV1>, String> {
        let mut exits = self.root_exits.borrow_mut();
        let progress = exits
            .get_mut(&(owner, exit.clone()))
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
        if let Some((completion, terminal)) = exit.explicit_site()
            .and_then(|site| self.call_source_completion_for_owner_at(owner, site))
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
                        exit: exit.clone(),
                        operation: InvokeOperation::Map(
                            crate::mir::instruction::MapInvokeOperation::End { map },
                        ),
                    });
                }
            }
        }
        let mut exits = self.root_exits.borrow_mut();
        let Some(RootHomeExitProgress::Emitting(order)) = exits.get_mut(&(owner, exit.clone()))
        else {
            return Err(freeze("root-exit-not-emitting"));
        };
        order.prepend_argument_maps(operations)?;
        let operations = order.normal();
        Ok(operations)
    }

    /// Borrow the same Plain FunctionExit graph; its normal/fault cleanup is
    /// still independently checked by the existing root final consumers.
    pub(in crate::mir) fn source_loop_return_bindings_v2(&self,
        product: &std::rc::Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>,
        source: &std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::SourceLoopReturnV1>,
    ) -> Result<Vec<(BasicBlockId, MirInstruction)>, String> {
        let owner = source.terminal.owner();
        if self.retained_source_loop_recipe_v2(owner, product.source.source.site())?
            .is_none_or(|original| !std::rc::Rc::ptr_eq(original, product))
            || !product.schedule().returns_v2().any(|row| std::rc::Rc::ptr_eq(row, source))
            || !matches!(self.terminal_relation_for_owner_at(owner, &source.site),
                Some(TerminalRelationV1::I64Scalar(original)) if original == &source.terminal)
            || self.completion_for_owner(owner).and_then(|completion| completion.cleanup().root_flow())
                .and_then(|flow| flow.exit_row(&source.site)).and_then(Result::ok) != Some(&source.homes) {
            return Err(freeze("source-loop/frame-source"));
        }
        let rows = self.root_exits.borrow();
        let Some(RootHomeExitProgress::Emitted { bindings, entry: RootHomeExitEntry::Plain { .. }, .. })
            = rows.get(&(owner, source.site.clone().into())) else { return Err(freeze("source-loop/original-write")); };
        Ok(bindings.clone())
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

    pub(super) fn validate_root_home_exit(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
        projection: Option<&super::physical_boundary::FinishedBindings>,
    ) -> Result<(), String> {
        self.validate_root_home_exit_view(owner, function, projection, None)
    }

    pub(super) fn validate_root_home_exit_with_pending(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
        projection: Option<&super::physical_boundary::FinishedBindings>,
        pending: &super::PendingSelectedStaticLoopViewV1<'_, '_>,
    ) -> Result<(), String> {
        self.validate_root_home_exit_view(owner, function, projection, Some(pending))
    }

    fn validate_root_home_exit_view(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
        projection: Option<&super::physical_boundary::FinishedBindings>,
        pending: Option<&super::PendingSelectedStaticLoopViewV1<'_, '_>>,
    ) -> Result<(), String> {
        self.validate_implicit_root_progress_v1(owner, function, projection)?;
        if !self.object_return_construction_ready_v1(owner)? {
            return Ok(());
        }
        let Some(completion) = self.completion_for_owner(owner) else {
            return Ok(());
        };
        let Some(flow) = completion.cleanup().root_flow() else {
            return Ok(());
        };
        if !self.root_home_flow_ready_v1(completion)? {
            return Ok(());
        }
        let exits = self.root_exits.borrow();
        for expected_exit in completion.explicit_sites() {
            let Some(exit_row) = self.normal_exit_projection_v1(owner, expected_exit)? else {
                return Err(format!("{}: owner={owner:?} exit={expected_exit:?} dispositions={} flow_exit={} terminal={:?}",
                    freeze("root-exit-source-missing"), self.normal_return_dispositions.is_some(),
                    flow.exit_row(expected_exit).is_some(), self.terminal_relation_for_owner_at(owner, expected_exit)));
            };
            let expected_homes = exit_row.homes();
            let pending_row = pending.and_then(|pending| pending.root(owner, expected_exit));
            let progress = exits.get(&(owner, expected_exit.clone().into()));
            let (order, origins, bindings, entry) = match (pending_row, progress) {
                (Some(row), None | Some(RootHomeExitProgress::Unprepared)) => {
                    (row.order(), row.origins(), row.bindings(), row.entry())
                }
                (None, Some(RootHomeExitProgress::Unavailable)) => continue,
                (None, Some(RootHomeExitProgress::Emitted {
                    order, origins, bindings, entry,
                })) => (order, origins.as_slice(), bindings.as_slice(), entry),
                _ => return Err(freeze("root-exit-unconsumed")),
            };
            {
                    if order.normal()
                        != origins
                            .iter()
                            .map(|row| row.origin.clone())
                            .collect::<Vec<_>>()
                    {
                        return Err(freeze("root-exit-origin-order"));
                    }
                    self.validate_call_entry_with_pending(
                        owner,
                        expected_exit,
                        function,
                        projection,
                        pending.map_or(&[][..], |pending| pending.excluded_routes()),
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
                    let expected_plans = source_home_end_plans_v1(&rows, expected_homes)?
                        .ok_or_else(|| freeze("root-home-end-unavailable"))?;
                    expected_subjects.extend(expected_plans.iter().map(|(_, subject, _)| subject.clone()));
                    if origins.len() != expected_subjects.len() {
                        return Err(freeze("root-exit-origin-count"));
                    }
                    let expected_subjects = expected_subjects.into_iter();
                    for (emitted, expected_subject) in origins.iter().zip(expected_subjects) {
                        if emitted.origin.subject() != &expected_subject
                            || expected_plans.iter().find(|(_, subject, _)| subject == &expected_subject)
                                .is_some_and(|(_, _, operation)| emitted.origin.operation() != operation)
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
                    order.validate_direct_entry(self, owner, expected_exit, entry, bindings, projection)?;
                    order.validate_null_return(entry, bindings, projection, projection)?;
                    if let Some(producers) = self.validate_return_expression_producer_v1(CallValidationContextV1::Lowering(self),
                        owner, expected_exit, order, entry, bindings, projection, projection, Some(function))? {
                        for producer in producers {
                            if !super::physical_boundary::check_binding(function, None, producer.0, &producer.1)? {
                                return Err(freeze("root-exit-binding-drift"));
                            }
                        }
                    }
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
        }
        Ok(())
    }
}

#[path = "root_call_entry.rs"]
mod call_entry;
#[path = "root_map_get_entry.rs"]
mod map_get_entry;
pub(crate) use map_get_entry::{StoredMapGetReturnEmissionV1, StoredMapGetReadEmissionV1, StoredMapGetLocalEmissionV1};

pub(in crate::mir) use call_entry::{
    CallPacketSourceLoanV1, CallPacketSourceV1, EmittedLexicalCallProjectionV1,
    LexicalCallArgumentProjectionV1, PendingStaticCallProjectionV1,
    PreparedLexicalCallProjectionV1, PreparedSelectedStaticLoopCallProjectionV1,
};
pub(crate) use call_entry::PreparedStaticTerminalPacketV1;

pub(crate) use call_entry::RootLifecyclePrefixBindingGroupV1;
pub(crate) use call_entry::PreparedRootLexicalCallBindingGroupV1;
pub(crate) use call_entry::{checked_expression_core_operation_v1, checked_scalar_invoke_operation_v1};
pub(in crate::mir) use call_entry::{EvaluatedExpressionValueV1, BranchScalarPhiObservationV1, ExpressionReferenceReadV1, ExpressionCallTransientPrefixV1, ExpressionCallArgumentSourceV1, validate_expression_acquisition_prefix_v1, validate_expression_acquisition_scope_v1};
pub(in crate::mir) use call_entry::ExpressionPhysicalProjectionRefV1;
pub(crate) use call_entry::{EmittedFieldArrayAppendV1, FieldAppendTextProjectionV1, EmittedOwnedArrayAppendV1, OwnedArrayAppendCandidateV1, EmittedPrintExpressionProjectionV1, EmittedBindingRebindProjectionV1, EmittedInitializerExpressionProjectionV1, PreparedLifecyclePrefixBindingGroupV1};

pub(in crate::mir) use call_entry::{SourceLoopCleanupPlanV2, SourceLoopPhysicalProofV2, PreparedSourceLoopPhysicalCompletionV2, EmittedBoolConditionProjectionV1, BoolConditionSealV1, BoolConditionCursorV1, BoolConditionNodeProjectionV1, BoolConditionValueProofV1, borrowed_source_loop_phi_relations_v2};

pub(crate) use call_entry::{EmittedBorrowedFieldWriteV1, CheckedFieldWriteGraphV1};
pub(in crate::mir) use call_entry::{EmittedFieldWriteRhsV1, FieldWriteValueV1, FieldWriteRhsNodeV1, FieldWriteRhsProofV1};
