//! Retain original exit obligations across the existing affine root progress.
//! Sequence membership uses exact source origins, never emitted MIR discovery.
use super::{freeze, RootHomeReleaseOriginV1, FunctionOwnerIdV1, SourceStmtSiteV1,
    TerminalRelationV1, TerminalReturnedSourceV1, CallValidationContextV1, RootHomeExitProgress, ValueId};
use crate::mir::instruction::InvokeOperation;
use crate::mir::{BasicBlockId, MirInstruction};
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use crate::mir::resolved_semantics::home_new_prefix::RootHomeExitKeyV1;

fn conflicting_home(earlier: &LiveHomeV1, current: &LiveHomeV1) -> bool {
    if earlier == current { return true; }
    let involves_move = matches!(earlier, LiveHomeV1::Moved { .. })
        || matches!(current, LiveHomeV1::Moved { .. });
    involves_move && (earlier.binding() == current.binding()
        || current.stable_origin().is_some_and(|origin| earlier.stable_origin() == Some(origin)))
}

/// Physical cleanup selected by a retained source owner. This passive action
/// never issues ownership; a plain Array End has no semantic Fault edge.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum RootCleanupActionV1 {
    Invoke(InvokeOperation),
    ArrayRelease { value: crate::mir::ValueId },
}

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package) struct RootHomeCleanupOrderV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) original_frame: Option<(crate::mir::BasicBlockId, crate::mir::MirInstruction)>,
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) implicit_void:
        Option<(
            std::rc::Rc<crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1>,
            (crate::mir::BasicBlockId, crate::mir::MirInstruction),
        )>,
    pub(super) installed_map_return: Option<super::map_field_install::InstalledMapReturnEmissionV1>,
    pub(super) null_return: Option<super::null_return::TerminalNullReturnProducerV1>,
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) scalar_return_read: Option<super::call_entry::ScalarBindingReadProofV1>,
    pub(super) return_expression: Option<Box<super::received_return::ReturnExpressionEmissionV1>>,
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) return_read: Option<crate::mir::builder::ExactLexicalReadV1>,
    pub(super) text_return_hold: Option<(crate::mir::BasicBlockId, crate::mir::MirInstruction)>,
    direct: Option<(
        super::super::super::completion_index::DirectRootCleanupSourceV1,
        Option<crate::mir::ValueId>,
    )>,
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) stored_map_get:
        Option<crate::mir::normal_callable_semantic_package::StoredMapGetReturnSourceLoanV1>,
    scalar: Option<(
        super::SourceStmtSiteV1,
        crate::mir::normal_callable_semantic_package::BorrowedScalarResultSourceLoanV1,
        Option<crate::mir::ValueId>,
    )>,
    full: Box<[RootHomeReleaseOriginV1]>,
    normal: Box<[usize]>,
    acquisition_fault: Box<[usize]>,
}
/// Expand complete installed End plans in the original caller-supplied order.
/// No exit identity is invented for a Call Fault; unavailable never becomes empty.
pub(in crate::mir::normal_callable_semantic_package) fn source_home_end_plans_v1(
    ledger: &super::OrdinaryNewClaimLedgerV1,
    rows: &std::collections::BTreeMap<super::OwnedExprSiteV1, super::LocalCommitV1>,
    homes: &[LiveHomeV1],
) -> Result<
    Option<
        Vec<(
            LiveHomeV1,
            super::RootHomeReleaseSubjectV1,
            crate::mir::instruction::InvokeOperation,
        )>,
    >,
    String,
> {
    for (index, binding) in homes.iter().enumerate() {
        if homes[..index].iter().any(|earlier| conflicting_home(earlier, binding)) {
            return Err(freeze("root-cleanup-order/duplicate-home"));
        }
    }
    let mut result = Vec::new();
    let mut available = true;
    for binding in homes {
        let plans = if let Some(plans) = ledger.take_entry_home_end_plan_v1(binding)? {
            plans
        } else {
        let home = super::installed_source_home(rows, binding).map_err(|error| match error {
            super::HomeLookupError::Missing => freeze("root-home-not-installed"),
            super::HomeLookupError::Duplicate => freeze("duplicate-root-home"),
        })?;
        if let super::LocalCommitV1::CallReceived(row) = home {
            if let Some(parents) = &row.field_support_parents {
                for parent in parents.iter() {
                    if homes
                        .iter()
                        .filter(|home| home.binding() == *parent)
                        .count()
                        != 1
                        || homes.iter().position(|home| home.binding() == *parent)
                            <= homes.iter().position(|candidate| candidate == binding)
                    {
                        return Err(freeze("field-end/anchor-cleanup-order"));
                    }
                }
            }
        }
        available &= home.end_available();
        if !home.end_available() {
            continue;
        }
        home.end_plan().into_vec()
        };
        for (subject, operation) in plans {
            let subject = match binding {
                LiveHomeV1::Text(acquisition) => super::RootHomeReleaseSubjectV1::Text(acquisition.clone()),
                LiveHomeV1::Stable(_) => subject,
                LiveHomeV1::Moved { binding, .. } => match subject {
                    super::RootHomeReleaseSubjectV1::Binding(_) => super::RootHomeReleaseSubjectV1::Binding(*binding),
                    super::RootHomeReleaseSubjectV1::FieldResidence { field, .. } => super::RootHomeReleaseSubjectV1::FieldResidence { binding: *binding, field },
                    _ => return Err(freeze("moved-home/unexpected-end-subject")),
                },
            };
            result.push((binding.clone(), subject, operation));
        }
    }
    Ok(available.then_some(result))
}

/// Select complete Home plans with the same subset, order and retained-anchor
/// policy for root exits and Loop ports. No synthetic exit identity is needed.
pub(in crate::mir::normal_callable_semantic_package) fn source_home_end_selection_v2(
    ledger: &super::OrdinaryNewClaimLedgerV1,
    rows: &std::collections::BTreeMap<super::OwnedExprSiteV1, super::LocalCommitV1>,
    full_homes: &[LiveHomeV1],
    normal_homes: &[LiveHomeV1],
) -> Result<
    Option<(
        Vec<(
            LiveHomeV1,
            super::RootHomeReleaseSubjectV1,
            crate::mir::instruction::InvokeOperation,
        )>,
        Vec<usize>,
    )>,
    String,
> {
    for (index, binding) in full_homes.iter().enumerate() {
        if full_homes[..index].iter().any(|earlier| conflicting_home(earlier, binding)) {
            return Err(freeze("root-cleanup-order/duplicate-home"));
        }
    }
    let mut previous = None;
    for binding in normal_homes {
        let index = full_homes
            .iter()
            .position(|home| home == binding)
            .ok_or_else(|| freeze("root-cleanup-order/foreign-home"))?;
        if previous.is_some_and(|prior| prior >= index) {
            return Err(freeze("root-cleanup-order/home-order"));
        }
        previous = Some(index);
    }
    for row in rows.values() {
        if let super::LocalCommitV1::CallReceived(row) = row {
            if full_homes.iter().any(|home| home.binding() == row.binding)
                && !normal_homes
                    .iter()
                    .any(|home| home.binding() == row.binding)
                && row.field_support_parents.as_ref().is_some_and(|parents| {
                    parents
                        .iter()
                        .any(|parent| normal_homes.iter().any(|home| home.binding() == *parent))
                })
            {
                return Err(freeze("field-end/escaping-local-anchor"));
            }
        }
    }
    let Some(full) = source_home_end_plans_v1(ledger, rows, full_homes)? else {
        return Ok(None);
    };
    for (index, (_, subject, _)) in full.iter().enumerate() {
        if full[..index].iter().any(|(_, prior, _)| prior == subject) {
            return Err(freeze("root-cleanup-order/duplicate-subject"));
        }
    }
    let normal = full
        .iter()
        .enumerate()
        .filter_map(|(index, (binding, _, _))| normal_homes.contains(binding).then_some(index))
        .collect();
    Ok(Some((full, normal)))
}

/// Fault keeps every obligation except Normal operations already attempted.
pub(crate) fn fault_indices_after_normal_step_v2(
    full_len: usize,
    normal: &[usize],
    step: usize,
) -> Result<Vec<usize>, String> {
    let attempted = normal
        .get(..=step)
        .ok_or_else(|| freeze("root-cleanup-order/normal-step"))?;
    Ok((0..full_len)
        .filter(|index| !attempted.contains(index))
        .collect())
}

/// Fault order after a displaced End of a replaced Home. `full` is
/// [displaced, temporaries.., replacement, homes..] and `old_position` is the
/// displaced Home's slot among those homes. A Fault of the displaced End itself
/// keeps acquisition order (replacement first); once it succeeded, the
/// replacement holds the displaced slot, so younger homes unwind before it.
pub(crate) fn displaced_replacement_fault_sequences_v2<T: Clone>(
    full: &mut Vec<T>,
    normal_len: usize,
    old_position: usize,
) -> Result<Vec<Vec<usize>>, String> {
    let installed = normal_len;
    let homes = installed + 1;
    if normal_len < 2 || full.len() <= installed || old_position > full.len() - homes {
        return Err(freeze("root-cleanup-order/displaced-shape"));
    }
    let duplicate = full.len();
    full.push(full[installed].clone());
    Ok((0..normal_len)
        .map(|step| {
            if step == 0 {
                (1..duplicate).collect()
            } else {
                (step + 1..installed)
                    .chain(homes..homes + old_position)
                    .chain(std::iter::once(duplicate))
                    .chain(homes + old_position..duplicate)
                    .collect()
            }
        })
        .collect())
}

impl RootHomeCleanupOrderV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn text_return_hold_binding(
        &self,
    ) -> Option<&(crate::mir::BasicBlockId, crate::mir::MirInstruction)> {
        self.text_return_hold.as_ref()
    }

    pub(in crate::mir::normal_callable_semantic_package) fn ordinary(
        full: Vec<RootHomeReleaseOriginV1>,
    ) -> Result<Self, String> {
        Self::from_sequences(full.clone(), &full, &full)
    }

    /// Expand original full exit Homes once; Normal selects whole Home plans.
    pub(super) fn from_home_end_plans(
        ledger: &super::OrdinaryNewClaimLedgerV1,
        rows: &std::collections::BTreeMap<super::OwnedExprSiteV1, super::LocalCommitV1>,
        exit: &super::SourceStmtSiteV1,
        full_homes: &[LiveHomeV1],
        normal_homes: &[LiveHomeV1],
    ) -> Result<Option<Self>, String> {
        Self::from_home_end_plans_for_exit(
            ledger,
            rows,
            &RootHomeExitKeyV1::Explicit(exit.clone()),
            full_homes,
            normal_homes,
        )
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn from_home_end_plans_for_exit(
        ledger: &super::OrdinaryNewClaimLedgerV1,
        rows: &std::collections::BTreeMap<super::OwnedExprSiteV1, super::LocalCommitV1>,
        exit: &RootHomeExitKeyV1,
        full_homes: &[LiveHomeV1],
        normal_homes: &[LiveHomeV1],
    ) -> Result<Option<Self>, String> {
        let mut full = Vec::new();
        let mut normal = Vec::new();
        let Some((plans, selected)) = source_home_end_selection_v2(ledger, rows, full_homes, normal_homes)?
        else {
            return Ok(None);
        };
        if crate::config::env::builder_debug_enabled() {
            eprintln!("[root-home] plans exit={exit:?} {:?}", plans.iter().map(|(home, _, operation)| (home.binding(), operation)).collect::<Vec<_>>());
        }
        for (index, (_, subject, operation)) in plans.into_iter().enumerate() {
            let origin = RootHomeReleaseOriginV1 {
                subject,
                exit: exit.clone(),
                operation,
            };
            if selected.contains(&index) {
                normal.push(origin.clone());
            }
            full.push(origin);
        }
        Self::from_sequences(full.clone(), &normal, &full).map(Some)
    }

    fn from_sequences(
        full: Vec<RootHomeReleaseOriginV1>,
        normal: &[RootHomeReleaseOriginV1],
        acquisition_fault: &[RootHomeReleaseOriginV1],
    ) -> Result<Self, String> {
        for (index, origin) in full.iter().enumerate() {
            if full[..index]
                .iter()
                .any(|prior| prior.subject() == origin.subject())
            {
                return Err(freeze("root-cleanup-order/duplicate-subject"));
            }
            if full
                .first()
                .is_some_and(|first| first.exit() != origin.exit())
            {
                return Err(freeze("root-cleanup-order/foreign-exit"));
            }
        }
        let indices = |sequence: &[RootHomeReleaseOriginV1]| {
            let mut result = Vec::with_capacity(sequence.len());
            for origin in sequence {
                let index = full
                    .iter()
                    .position(|original| original == origin)
                    .ok_or_else(|| freeze("root-cleanup-order/foreign-origin"))?;
                if result.last().is_some_and(|prior| *prior >= index) {
                    return Err(freeze("root-cleanup-order/sequence-order"));
                }
                result.push(index);
            }
            Ok::<_, String>(result.into_boxed_slice())
        };
        let normal = indices(normal)?;
        let acquisition_fault = indices(acquisition_fault)?;
        Ok(Self {
            direct: None,
            scalar: None,
            stored_map_get: None,
            installed_map_return: None,
            null_return: None,
            return_expression: None,
            scalar_return_read: None,
            return_read: None,
            text_return_hold: None,
            implicit_void: None,
            original_frame: None,
            full: full.into_boxed_slice(),
            normal,
            acquisition_fault,
        })
    }

    pub(in crate::mir) fn result_cancellation_v1(
        &self,
    ) -> Result<Vec<crate::mir::instruction::InvokeOperation>, String> {
        super::RootHomeFaultSequencesV1::result_cancellation_from_sources_v1(
            &self.full, &self.acquisition_fault)
    }

    pub(super) fn prepend_argument_maps(
        &mut self,
        prefix: Vec<RootHomeReleaseOriginV1>,
    ) -> Result<(), String> {
        if self
            .direct
            .as_ref()
            .is_some_and(|(_, value)| value.is_some())
            || self
                .scalar
                .as_ref()
                .is_some_and(|(_, _, value)| value.is_some())
        {
            return Err(freeze("direct-result-arguments-after-bind"));
        }
        let mut full = prefix.clone();
        full.extend_from_slice(&self.full);
        let mut normal = prefix.clone();
        normal.extend(self.normal());
        let mut acquisition_fault = prefix;
        acquisition_fault.extend(self.acquisition_fault());
        self.replace_sequences(full, &normal, &acquisition_fault)
    }

    /// Younger than the loop's prior Homes, older than its own: the carrier
    /// Ends sit just before the first prior-Home obligation (or last).
    pub(super) fn insert_loop_text_carrier_ends(
        &mut self,
        prior: &[crate::mir::resolved_semantics::BindingRefV1],
        ends: &[RootHomeReleaseOriginV1],
    ) -> Result<(), String> {
        let at = self
            .full
            .iter()
            .position(|origin| origin.binding().is_some_and(|binding| prior.contains(&binding)))
            .unwrap_or(self.full.len());
        let mut full = self.full.to_vec();
        full.splice(at..at, ends.iter().cloned());
        let with_ends = |indices: &[usize]| {
            let mut sequence = Vec::with_capacity(indices.len() + ends.len());
            let mut placed = false;
            for index in indices {
                if *index >= at && !placed {
                    sequence.extend_from_slice(ends);
                    placed = true;
                }
                sequence.push(self.full[*index].clone());
            }
            if !placed {
                sequence.extend_from_slice(ends);
            }
            sequence
        };
        let normal = with_ends(&self.normal);
        let acquisition_fault = with_ends(&self.acquisition_fault);
        self.replace_sequences(full, &normal, &acquisition_fault)
    }

    /// A completed pending Return result transfers only after prior cleanup.
    /// Its End belongs to later Fault residuals, never Normal/acquisition.
    pub(super) fn prepend_fault_only_result(
        &mut self,
        prefix: Vec<RootHomeReleaseOriginV1>,
    ) -> Result<(), String> {
        if prefix.is_empty() {
            return Err(freeze("result-cleanup/empty-plan"));
        }
        let mut full = prefix;
        full.extend_from_slice(&self.full);
        self.replace_sequences(full, &self.normal(), &self.acquisition_fault())
    }

    /// Reorder obligations atomically without replacing their original source
    /// products. New source fields must not require another hand-copy list.
    fn replace_sequences(
        &mut self,
        full: Vec<RootHomeReleaseOriginV1>,
        normal: &[RootHomeReleaseOriginV1],
        acquisition_fault: &[RootHomeReleaseOriginV1],
    ) -> Result<(), String> {
        let next = Self::from_sequences(full, normal, acquisition_fault)?;
        self.full = next.full;
        self.normal = next.normal;
        self.acquisition_fault = next.acquisition_fault;
        Ok(())
    }

    pub(super) fn validate_fault_only_result(
        &self,
        expected: &[RootHomeReleaseOriginV1],
    ) -> Result<(), String> {
        if expected.is_empty()
            || !self.full.starts_with(expected)
            || self
                .normal()
                .iter()
                .chain(self.acquisition_fault().iter())
                .any(|origin| {
                    expected
                        .iter()
                        .any(|pending| pending.subject() == origin.subject())
                })
        {
            return Err(freeze("result-cleanup/order-drift"));
        }
        Ok(())
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn borrowed_expression_fault_v1(&self)
        -> Option<(BasicBlockId, &[(BasicBlockId, MirInstruction)])> {
        self.return_expression.as_ref().and_then(|row| row.borrowed_root_boundary_v1())
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn normal(
        &self,
    ) -> Vec<RootHomeReleaseOriginV1> {
        self.normal
            .iter()
            .map(|index| self.full[*index].clone())
            .collect()
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn full(
        &self,
    ) -> &[RootHomeReleaseOriginV1] {
        &self.full
    }

    pub(super) fn acquisition_fault_indices(&self) -> &[usize] {
        &self.acquisition_fault
    }

    pub(super) fn fault_indices_after_normal_step(
        &self,
        step: usize,
    ) -> Result<Vec<usize>, String> {
        fault_indices_after_normal_step_v2(self.full.len(), &self.normal, step)
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn acquisition_fault(
        &self,
    ) -> Vec<RootHomeReleaseOriginV1> {
        self.acquisition_fault
            .iter()
            .map(|index| self.full[*index].clone())
            .collect()
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn fault_after_normal_step(
        &self,
        step: usize,
    ) -> Result<Vec<RootHomeReleaseOriginV1>, String> {
        Ok(self
            .fault_indices_after_normal_step(step)?
            .into_iter()
            .map(|index| self.full[index].clone())
            .collect())
    }
}

#[cfg(test)]
#[path = "root_home_cleanup_order_tests.rs"]
mod tests;

#[path = "root_home_direct_result.rs"]
mod direct_result;

impl super::RootHomeFaultSequencesV1 {
    /// The result is acquired only on Call Normal. Its source-issued prefix
    /// precedes the unchanged acquisition-Fault Home suffix on cleanup Fault.
    pub(in crate::mir) fn result_cancellation_v1(&self) -> Result<Vec<InvokeOperation>, String> {
        Self::result_cancellation_from_sources_v1(&self.full, &self.acquisition)
    }
    pub(super) fn result_cancellation_from_sources_v1(
        full: &[RootHomeReleaseOriginV1], acquisition: &[usize],
    ) -> Result<Vec<InvokeOperation>, String> {
        let split = full.len().checked_sub(acquisition.len())
            .ok_or_else(|| freeze("root-fault-sequence-shape"))?;
        if acquisition.iter().copied().ne(split..full.len())
            || full[..split].iter().any(|origin| !matches!(origin.subject(),
                super::RootHomeReleaseSubjectV1::DirectResult { .. }
                | super::RootHomeReleaseSubjectV1::DirectResultField { .. })) {
            return Err(freeze("root-fault-sequence-shape"));
        }
        Ok(full[..split].iter().map(|origin| origin.operation().clone()).collect())
    }
}

// Entry values are the same once-recorded parameter correspondence used by
// reads. Only the original take declaration grants this Home and complete End.
impl super::OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn source_home_installed_v1(&self,
        rows: &std::collections::BTreeMap<super::OwnedExprSiteV1, super::LocalCommitV1>, home: &LiveHomeV1,
    ) -> Result<bool, String> {
        match self.take_entry_home_end_plan_v1(home)? {
            Some(_) => Ok(true),
            None => Ok(super::installed_source_home(rows, home).is_ok()),
        }
    }

    pub(in crate::mir::normal_callable_semantic_package) fn take_entry_home_end_plan_v1(&self, home: &LiveHomeV1) -> Result<
        Option<Vec<(super::RootHomeReleaseSubjectV1, InvokeOperation)>>, String,
    > {
        let Some(binding) = home.stable_origin() else { return Ok(None); };
        let Some(catalog) = self.original_parameter_contracts.as_deref() else { return Ok(None); };
        let mut declarations = catalog.iter().filter(|row| row.owner == binding.owner());
        let Some(declaration) = declarations.next() else { return Ok(None); };
        if declarations.next().is_some() { return Err(freeze("borrowed-entry/original-contract-duplicate")); }
        let mut parameters = declaration.parameters.iter().filter(|row| row.binding == binding);
        let Some(parameter) = parameters.next() else { return Ok(None); };
        if parameters.next().is_some() { return Err(freeze("borrowed-entry/actuals-identity")); }
        if parameter.demand != crate::mir::resolved_semantics::HomeDemandV1::Home { return Ok(None); }
        let crate::mir::callable_parameter_contract::CallableParameterContractKindV1::DeclaredObject(class) = &parameter.kind
            else { return Err(freeze("root-home-not-installed")); };
        let values = self.borrowed_ordinary_entry_values_v1(binding.owner())?;
        let mut actuals = values.iter().filter(|(ordinal, actual, _)| *ordinal == parameter.ordinal && *actual == binding);
        let Some((_, _, value)) = actuals.next() else { return Err(freeze("borrowed-entry/entry-values-missing")); };
        if actuals.next().is_some() { return Err(freeze("borrowed-entry/actuals-identity")); }
        let entry = self.finalized_borrowed_ordinary_entry_source_v1(binding.owner())?;
        let view = entry.formal_object_view(binding)?.ok_or_else(|| freeze("root-home-not-installed"))?;
        if !view.is_declared() || view.class() != class.as_ref() || !view.destruction_disposition().has_no_hook() {
            return Err(freeze("root-home-not-installed"));
        }
        let children = match self.owned_field_children_for(view.object()) {
            Some(Some(children)) => children,
            None if !view.destruction_disposition().requires_field_end() => &[],
            _ => return Err(freeze("root-home-not-installed")),
        };
        Ok(Some(super::super::super::object_end::operations_v1(view.object(), false, children, *value)
            .map(|(field, operation)| (match field {
                Some(field) => super::RootHomeReleaseSubjectV1::FieldResidence { binding, field },
                None => super::RootHomeReleaseSubjectV1::Binding(binding),
            }, operation)).collect()))
    }
}

impl super::OrdinaryNewClaimLedgerV1 {
    pub(crate) fn check_declared_return_value_v1(&self, owner: FunctionOwnerIdV1,
        exit: &SourceStmtSiteV1, value: ValueId,
    ) -> Result<(), String> {
        let rows = self.root_exits.borrow();
        let Some(RootHomeExitProgress::Prepared(order)) = rows.get(&(owner, exit.clone().into()))
            else { return Err(freeze("root-exit-record-without-prepare")); };
        if order.return_read.as_ref().is_some_and(|read|
            read.binding_observation_v2().binding_value().1 != value) {
            return Err(freeze("borrowed-return/source-kind"));
        }
        Ok(())
    }

    /// Keep the actual one-shot terminal read in the existing exit owner.
    /// Nonterminal reads and legacy returns retain their original lowering.
    pub(crate) fn emit_return_read_v1(
        &self, owner: FunctionOwnerIdV1,
        site: &crate::mir::resolved_semantics::SourceNodeSiteV1,
        emit: impl FnOnce() -> Result<crate::mir::builder::ExactLexicalReadV1, String>,
    ) -> Result<Option<ValueId>, String> {
        use crate::mir::resolved_semantics::{SourceNodeSiteV1, SourcePathSegmentV1};
        let Some((SourcePathSegmentV1::Value, parent)) = site.segments().split_last() else { return Ok(None); };
        let exit = SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(parent.to_vec()));
        let terminal = self.terminal_relation_for_owner_at(owner, &exit);
        let declared = match terminal {
            Some(TerminalRelationV1::Value(terminal)) => {
                let TerminalReturnedSourceV1::BorrowedNullableLocal(source) = terminal.returned() else { return Ok(None); };
                let Some(view) = source.read_v1() else { return Ok(None); };
                if terminal.value_site().node() != site || view.read().site().node() != site
                    || self.checked_borrowed_nullable_return_source_v1(owner, &exit)? != Some(source.as_ref()) {
                    return Err(freeze("borrowed-return/source-kind"));
                }
                Some(view)
            }
            Some(TerminalRelationV1::Scalar(terminal)) if terminal.evaluation().is_none() => {
                let Some(read) = terminal.original_read_v1() else { return Ok(None); };
                if terminal.value_site().node() != site || read.read().node() != site
                    || read.binding().owner() != owner || read.kind() != crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind::Integer {
                    return Err(freeze("expression/scalar-binding-observation"));
                }
                None
            }
            _ => return Ok(None),
        };
        {
            let rows = self.root_exits.borrow();
            if !matches!(rows.get(&(owner, exit.clone().into())),
                Some(RootHomeExitProgress::Prepared(order)) if order.return_read.is_none()) {
                return Err(freeze("root-exit-record-without-emission"));
            }
        }
        let read = emit()?;
        let value = match declared {
            Some(view) => CallValidationContextV1::Lowering(self).declared_result_read_v1(view, &read)?.0,
            None => {
                let Some(TerminalRelationV1::Scalar(terminal)) = terminal else { unreachable!() };
                let source = terminal.original_read_v1().unwrap();
                read.value_for(owner, site, source.binding()).map_err(|error| error.to_string())?
            }
        };
        let mut rows = self.root_exits.borrow_mut();
        let Some(RootHomeExitProgress::Prepared(order)) = rows.get_mut(&(owner, exit.into()))
            else { return Err(freeze("root-exit-record-without-emission")); };
        if order.return_read.is_some() { return Err(freeze("expression/scalar-binding-observation")); }
        order.return_read = Some(read);
        Ok(Some(value))
    }

}
