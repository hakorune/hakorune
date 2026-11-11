//! Existing owner-indexed root Home progress and physical emission validation.
//!
//! `RootHomeExitProgress` is the sole owner of the retained Home binding,
//! completion exit, object, and physical value until selected emission has
//! recorded and final validation has checked the release operation.

use super::*;
#[path = "root_home/validation.rs"]
mod validation;
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
pub(crate) use cleanup_order::{displaced_replacement_fault_sequences_v2, fault_indices_after_normal_step_v2};
pub(crate) use cleanup_order::RootCleanupActionV1;
#[path = "root_home_null_return.rs"]
mod null_return;
pub(crate) use null_return::TerminalNullReturnSourceLoanV1;
#[path = "root_home_fresh_return.rs"]
mod fresh_return;
#[path = "root_home_received_return.rs"]
mod received_return;
#[path = "root_scalar_read.rs"]
mod scalar_read;

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
    /// A Loop Text carrier with no Home of its own at an in-loop `return`.
    LoopTextCarrier(BindingRefV1),
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
            | RootHomeReleaseSubjectV1::FieldResidence { binding, .. }
            | RootHomeReleaseSubjectV1::LoopTextCarrier(binding) => Some(*binding),
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
#[derive(Clone, PartialEq)]
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
                completion.cleanup().root_flow().is_some_and(|_| {
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

    pub(in crate::mir::normal_callable_semantic_package) fn root_home_exit_not_selected_v1(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceNodeSiteV1,
    ) -> Result<bool, String> {
        Ok(matches!(
            self.preview_root_home_exit_v1(owner, site)?,
            prepare::RootHomePreparationV1::NotSelected
        ))
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

    /// Borrow the SAME older Home order for a scalar expression with no
    /// independent argument/result cleanup. This lends no affine progress.
    pub(crate) fn expression_root_cleanup_v1(
        &self,
        frame: &crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1,
    ) -> Result<Option<(Vec<RootHomeReleaseOriginV1>, RootHomeFaultSequencesV1)>, String> {
        use crate::mir::resolved_value_profile::{expression_class::ExpressionValueClassV1 as Class,
            expression_projection::ExpressionLeafClassV1 as Leaf};
        if !matches!(frame.root_class(), Leaf::Value(Class::I64 | Class::Bool))
            || !frame.calls().is_empty() || !frame.owned_result_nodes().is_empty()
            || frame.cores().values().any(|core| core.source().target().frame_free_read_law_v1().is_none()) {
            return Ok(None);
        }
        let owner = frame.source().owner();
        let site = frame.source().root().statement();
        if self.return_expression_evaluation_v1(owner, site)?.is_none_or(|source| !std::ptr::eq(source.as_ref(), frame)) {
            return Ok(None);
        }
        let exits = self.root_exits.borrow();
        let order = match exits.get(&(owner, site.clone().into())) {
            Some(RootHomeExitProgress::Prepared(order) | RootHomeExitProgress::Emitting(order)
                | RootHomeExitProgress::Finalized(order))
            | Some(RootHomeExitProgress::Emitted { order, .. }) => order,
            _ => return Ok(None),
        };
        if order.normal().is_empty() || order.text_return_hold.is_some() { return Ok(None); }
        let faults = RootHomeFaultSequencesV1 {
            full: order.full().to_vec(), acquisition: order.acquisition_fault_indices().to_vec(),
            after_normal: (0..order.normal().len()).map(|index| order.fault_indices_after_normal_step(index)).collect::<Result<_, _>>()?,
        };
        let older: Vec<_> = faults.acquisition.iter().map(|index| faults.full[*index].operation().clone()).collect();
        if older != self.prior_home_unwind_for(frame.prior_homes())? { return Ok(None); }
        Ok(Some((order.normal().to_vec(), faults)))
    }

    pub(in crate::mir::normal_callable_semantic_package) fn expression_root_continuation_v1(
        &self, frame: &crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1,
        continuation: &(BasicBlockId, MirInstruction),
    ) -> Result<bool, String> {
        let Some((normal, _)) = self.expression_root_cleanup_v1(frame)? else { return Ok(false); };
        Ok(matches!(&continuation.1, MirInstruction::Invoke { operation, .. }
            if normal.first().is_some_and(|origin| origin.operation() == operation)))
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

    /// Record the Header/Residence Text carriers an in-loop `return` still owes
    /// (older than the loop's own Homes, younger than its prior Homes).
    pub(crate) fn attach_loop_return_text_carrier_ends_v2(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceStmtSiteV1,
        prior: Vec<BindingRefV1>,
        ends: Vec<(BindingRefV1, InvokeOperation)>,
    ) -> Result<(), String> {
        if ends.is_empty() {
            return Ok(());
        }
        let exit: RootHomeExitKeyV1 = site.clone().into();
        let origins = ends
            .into_iter()
            .map(|(binding, operation)| RootHomeReleaseOriginV1 {
                subject: RootHomeReleaseSubjectV1::LoopTextCarrier(binding),
                exit: exit.clone(),
                operation,
            })
            .collect();
        if self.loop_return_text_ends.borrow_mut().insert((owner, exit), (prior, origins)).is_some() {
            return Err(freeze("loop-return-text-ends/duplicate"));
        }
        Ok(())
    }

    pub(crate) fn record_loop_call_text_carrier_ends_v2(
        &self,
        call: OwnedExprSiteV1,
        prior_len: usize,
        ends: Vec<InvokeOperation>,
    ) -> Result<(), String> {
        if crate::config::env::builder_debug_enabled() {
            eprintln!("[loop-call-ends] record call={call:?} prior_len={prior_len} ends={ends:?}");
        }
        if ends.is_empty() {
            return Ok(());
        }
        if self.loop_call_text_ends.borrow_mut().insert(call, (prior_len, ends)).is_some() {
            return Err(freeze("loop-call-text-ends/duplicate"));
        }
        Ok(())
    }

    /// Place the recorded carrier Ends just before the loop's prior-Home
    /// suffix of `unwind` (oldest-first `homes`, newest-first operations).
    pub(crate) fn with_loop_call_text_ends_v2(
        &self,
        call: &OwnedExprSiteV1,
        homes: &[crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1],
        unwind: &mut Vec<InvokeOperation>,
    ) -> Result<(), String> {
        self.with_loop_call_text_ends_at_state_v1(call, homes, homes, unwind)
    }

    /// The recorded prefix identifies the loop boundary, while only the
    /// evaluation's current responsibilities contribute cleanup operations.
    pub(crate) fn with_loop_call_text_ends_at_state_v1(
        &self,
        call: &OwnedExprSiteV1,
        original: &[crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1],
        current: &[crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1],
        unwind: &mut Vec<InvokeOperation>,
    ) -> Result<(), String> {
        let recorded = self.loop_call_text_ends.borrow();
        let Some((prior_len, ends)) = recorded.get(call) else {
            return Ok(());
        };
        if crate::config::env::builder_debug_enabled() {
            eprintln!("[loop-call-ends] apply call={call:?} unwind={unwind:?}");
        }
        let prefix = original.get(..*prior_len)
            .ok_or_else(|| freeze("loop-call-text-ends/homes"))?;
        let retained: Vec<_> = current.iter().filter(|home| prefix.contains(home)).cloned().collect();
        let suffix = self.prior_home_unwind_for(&retained)?;
        if !unwind.ends_with(&suffix) {
            return Err(freeze("loop-call-text-ends/suffix"));
        }
        let at = unwind.len() - suffix.len();
        unwind.splice(at..at, ends.iter().cloned());
        Ok(())
    }

    /// Project source-owned responsibilities; the optional override retains
    /// loop carrier cleanup, which is not an expression Home transfer.
    pub(crate) fn expression_home_unwind_at_v1(
        &self,
        frame: &crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1,
        current: &[crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1],
        override_unwind: Option<&[InvokeOperation]>,
    ) -> Result<Vec<InvokeOperation>, String> {
        if current.iter().any(|home| !frame.prior_homes().contains(home)) {
            return Err(freeze("borrowed-actual/source-identity"));
        }
        if let Some(original) = override_unwind {
            let mut projected = original.to_vec();
            for home in frame.prior_homes().iter().filter(|home| !current.contains(home)) {
                for operation in self.prior_home_unwind_for(std::slice::from_ref(home))? {
                    let mut positions = projected.iter().enumerate()
                        .filter_map(|(index, candidate)| (candidate == &operation).then_some(index));
                    let position = positions.next().ok_or_else(|| freeze("borrowed-actual/source-identity"))?;
                    if positions.next().is_some() { return Err(freeze("borrowed-actual/source-identity")); }
                    projected.remove(position);
                }
            }
            return Ok(projected);
        }
        let mut unwind = self.prior_home_unwind_for(current)?;
        use crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1 as Root;
        if let Root::CallArgument { call, .. } = frame.source().root() {
            let site = OwnedExprSiteV1::new(frame.source().owner(), call.site().clone());
            self.with_loop_call_text_ends_at_state_v1(&site, frame.prior_homes(), current, &mut unwind)?;
        }
        Ok(unwind)
    }

    /// Call-argument expressions Fault with their enclosing Call's carriers.
    pub(crate) fn with_loop_call_text_ends_for_frame_v2(
        &self,
        frame: &crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1,
        unwind: &mut Vec<InvokeOperation>,
    ) -> Result<(), String> {
        use crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1 as Root;
        if let Root::CallArgument { call, .. } = frame.source().root() {
            let site = OwnedExprSiteV1::new(frame.source().owner(), call.site().clone());
            self.with_loop_call_text_ends_v2(&site, frame.prior_homes(), unwind)?;
        }
        Ok(())
    }

    /// Apply the recorded loop-return carrier Ends to `order`; validators
    /// rebuild their expected order through this same insertion.
    pub(in crate::mir::normal_callable_semantic_package) fn adopt_loop_return_text_ends_v1(
        &self,
        owner: FunctionOwnerIdV1,
        exit: &RootHomeExitKeyV1,
        order: &mut RootHomeCleanupOrderV1,
    ) -> Result<(), String> {
        let recorded = self.loop_return_text_ends.borrow();
        match recorded.get(&(owner, exit.clone())) {
            Some((prior, ends)) => order.insert_loop_text_carrier_ends(prior, ends),
            None => Ok(()),
        }
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
        self.adopt_loop_return_text_ends_v1(owner, exit, order)?;
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
        let terminal_matches = match (
            &source.terminal,
            self.terminal_relation_for_owner_at(owner, &source.site),
        ) {
            (
                TerminalRelationV1::Scalar(expected),
                Some(TerminalRelationV1::Scalar(original)),
            ) => expected == original,
            (
                TerminalRelationV1::IntegerLiteral(expected),
                Some(TerminalRelationV1::IntegerLiteral(original)),
            ) => expected == original,
            (TerminalRelationV1::Value(expected), Some(TerminalRelationV1::Value(original))) => expected == original,
            _ => false,
        };
        if self.retained_source_loop_recipe_v2(owner, product.source.source.site())?
            .is_none_or(|original| !std::rc::Rc::ptr_eq(original, product))
            || !product.schedule().returns_v2().any(|row| std::rc::Rc::ptr_eq(row, source))
            || !terminal_matches
            || self.completion_for_owner(owner).and_then(|completion| completion.cleanup().root_flow())
                .and_then(|flow| flow.exit_row(&source.site)).and_then(Result::ok) != Some(&source.homes) {
            return Err(freeze("source-loop/frame-source"));
        }
        let rows = self.root_exits.borrow();
        let Some(RootHomeExitProgress::Emitted { order, bindings, entry: RootHomeExitEntry::Plain { .. }, .. })
            = rows.get(&(owner, source.site.clone().into())) else { return Err(freeze("source-loop/original-write")); };
        let mut bindings = bindings.clone();
        let literal = self.terminal_literal_producer_v1(owner, &source.site)?;
        // Null is already retained by the same prepared exit's source loan;
        // include that exact append in Loop accounting, without re-emission.
        for producer in literal.as_ref().into_iter().chain(order.null_return_binding()) {
            if !bindings.contains(producer) {
                let entry_jump = bindings
                    .len()
                    .checked_sub(1)
                    .ok_or_else(|| freeze("source-loop/original-write"))?;
                bindings.insert(entry_jump, producer.clone());
            }
        }
        Ok(bindings)
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


}

#[path = "root_call_entry.rs"]
mod call_entry;
#[path = "root_map_field_install.rs"]
mod map_field_install;
pub(crate) use map_field_install::{PreparedMapFieldInstallV1, MapFieldInstallEmissionV1};
#[path = "root_map_key_read.rs"]
mod map_key_read;
#[path = "root_map_get_entry.rs"]
mod map_get_entry;
pub(crate) use map_get_entry::{StoredMapGetReturnEmissionV1, StoredMapGetReadEmissionV1, StoredMapGetLocalEmissionV1};

pub(in crate::mir) use call_entry::{
    CallPacketSourceLoanV1, CallPacketSourceV1, EmittedLexicalCallProjectionV1,
    LexicalCallArgumentProjectionV1, TakenObjectArgumentOriginV1, PendingStaticCallProjectionV1,
    PreparedLexicalCallProjectionV1, StoredReceiverReadV1, StoredReceiverFaultV1, PreparedSelectedStaticLoopCallProjectionV1,
};
pub(in crate::mir) use call_entry::EmittedDynamicInvocationCallProjectionV1;
pub(crate) use call_entry::PreparedStaticTerminalPacketV1;

pub(crate) use call_entry::RootLifecyclePrefixBindingGroupV1;
pub(crate) use call_entry::PreparedRootLexicalCallBindingGroupV1;
pub(crate) use call_entry::{FrameFreeExpressionReadV1, checked_expression_core_operation_v1, checked_scalar_invoke_operation_v1};
pub(in crate::mir) use call_entry::{EvaluatedExpressionValueV1, BranchScalarPhiObservationV1, BranchTextPhiObservationV1, BranchReadPhiObservationV1, ExpressionReferenceReadV1, ExpressionCallTransientPrefixV1, ExpressionCallArgumentSourceV1, validate_expression_acquisition_prefix_v1, validate_expression_acquisition_scope_v1};
pub(in crate::mir) use call_entry::ExpressionPhysicalProjectionRefV1;
pub(crate) use call_entry::{EmittedFieldArrayAppendV1, FieldAppendTextProjectionV1, EmittedOwnedArrayAppendV1, OwnedArrayAppendCandidateV1, EmittedPrintExpressionProjectionV1, EmittedBindingRebindProjectionV1, EmittedInitializerExpressionProjectionV1};

pub(in crate::mir) use call_entry::{SourceLoopCleanupPlanV2, SourceLoopPhysicalProofV2, EmittedBoolConditionProjectionV1, BoolConditionSealV1, BoolConditionCursorV1, BoolConditionNodeProjectionV1, BoolConditionValueProofV1, borrowed_source_loop_phi_relations_v2};

pub(crate) use call_entry::{EmittedBorrowedFieldWriteV1, CheckedFieldWriteGraphV1};
pub(in crate::mir) use call_entry::{EmittedFieldWriteRhsV1, FieldWriteValueV1, FieldWriteRhsNodeV1, FieldWriteRhsProofV1};
