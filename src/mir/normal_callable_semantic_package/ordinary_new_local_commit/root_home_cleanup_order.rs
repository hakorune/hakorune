//! Retain original exit obligations across the existing affine root progress.
//! Sequence membership uses exact source origins, never emitted MIR discovery.
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use super::{freeze, RootHomeReleaseOriginV1};
use crate::mir::resolved_semantics::home_new_prefix::RootHomeExitKeyV1;

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package) struct RootHomeCleanupOrderV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) implicit_void: Option<(
        std::rc::Rc<crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1>,
        (crate::mir::BasicBlockId, crate::mir::MirInstruction),
    )>,
    pub(super) null_return: Option<super::null_return::TerminalNullReturnProducerV1>,
    pub(super) return_expression: Option<Box<super::received_return::ReturnExpressionEmissionV1>>,
    pub(super) text_return_hold: Option<(crate::mir::BasicBlockId, crate::mir::MirInstruction)>,
    direct: Option<(
        super::super::super::completion_index::DirectRootCleanupSourceV1,
        Option<crate::mir::ValueId>,
    )>,
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) stored_map_get: Option<crate::mir::normal_callable_semantic_package::StoredMapGetReturnSourceLoanV1>,
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
    rows: &std::collections::BTreeMap<super::OwnedExprSiteV1, super::LocalCommitV1>,
    homes: &[LiveHomeV1],
) -> Result<Option<Vec<(LiveHomeV1, super::RootHomeReleaseSubjectV1, crate::mir::instruction::InvokeOperation)>>, String> {
    for (index, binding) in homes.iter().enumerate() {
        if homes[..index].contains(binding) { return Err(freeze("root-cleanup-order/duplicate-home")); }
    }
    let mut result = Vec::new();
    let mut available = true;
    for binding in homes {
        let home = super::installed_source_home(rows, binding).map_err(|error| match error {
            super::HomeLookupError::Missing => freeze("root-home-not-installed"),
            super::HomeLookupError::Duplicate => freeze("duplicate-root-home"),
        })?;
        if let super::LocalCommitV1::CallReceived(row) = home {
            if let Some(parents) = &row.field_support_parents {
                for parent in parents.iter() {
                    if homes.iter().filter(|home| home.binding() == *parent).count() != 1
                        || homes.iter().position(|home| home.binding() == *parent)
                            <= homes.iter().position(|candidate| candidate == binding) {
                        return Err(freeze("field-end/anchor-cleanup-order"));
                    }
                }
            }
        }
        available &= home.end_available();
        if !home.end_available() { continue; }
        result.extend(home.end_plan().into_vec().into_iter().map(|(subject, operation)| {
            let subject = match binding {
                LiveHomeV1::Text(acquisition) => super::RootHomeReleaseSubjectV1::Text(acquisition.clone()),
                LiveHomeV1::Stable(_) => subject,
            };
            (binding.clone(), subject, operation)
        }));
    }
    Ok(available.then_some(result))
}

/// Select complete Home plans with the same subset, order and retained-anchor
/// policy for root exits and Loop ports. No synthetic exit identity is needed.
pub(in crate::mir::normal_callable_semantic_package) fn source_home_end_selection_v2(
    rows: &std::collections::BTreeMap<super::OwnedExprSiteV1, super::LocalCommitV1>,
    full_homes: &[LiveHomeV1], normal_homes: &[LiveHomeV1],
) -> Result<Option<(Vec<(LiveHomeV1, super::RootHomeReleaseSubjectV1,
    crate::mir::instruction::InvokeOperation)>, Vec<usize>)>, String> {
    for (index, binding) in full_homes.iter().enumerate() {
        if full_homes[..index].contains(binding) {
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
            if full_homes.iter().any(|home| home.binding() == row.binding) && !normal_homes.iter().any(|home| home.binding() == row.binding)
                && row.field_support_parents.as_ref().is_some_and(|parents| parents.iter().any(|parent| normal_homes.iter().any(|home| home.binding() == *parent))) {
                return Err(freeze("field-end/escaping-local-anchor"));
            }
        }
    }
    let Some(full) = source_home_end_plans_v1(rows, full_homes)? else { return Ok(None) };
    for (index, (_, subject, _)) in full.iter().enumerate() {
        if full[..index].iter().any(|(_, prior, _)| prior == subject) {
            return Err(freeze("root-cleanup-order/duplicate-subject"));
        }
    }
    let normal = full.iter().enumerate().filter_map(|(index, (binding, _, _))|
        normal_homes.contains(binding).then_some(index)).collect();
    Ok(Some((full, normal)))
}

/// Fault keeps every obligation except Normal operations already attempted.
pub(crate) fn fault_indices_after_normal_step_v2(
    full_len: usize, normal: &[usize], step: usize,
) -> Result<Vec<usize>, String> {
    let attempted = normal.get(..=step).ok_or_else(|| freeze("root-cleanup-order/normal-step"))?;
    Ok((0..full_len).filter(|index| !attempted.contains(index)).collect())
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
        rows: &std::collections::BTreeMap<super::OwnedExprSiteV1, super::LocalCommitV1>,
        exit: &super::SourceStmtSiteV1,
        full_homes: &[LiveHomeV1],
        normal_homes: &[LiveHomeV1],
    ) -> Result<Option<Self>, String> {
        Self::from_home_end_plans_for_exit(rows, &RootHomeExitKeyV1::Explicit(exit.clone()), full_homes, normal_homes)
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn from_home_end_plans_for_exit(
        rows: &std::collections::BTreeMap<super::OwnedExprSiteV1, super::LocalCommitV1>,
        exit: &RootHomeExitKeyV1,
        full_homes: &[LiveHomeV1], normal_homes: &[LiveHomeV1],
    ) -> Result<Option<Self>, String> {
        let mut full = Vec::new();
        let mut normal = Vec::new();
        let Some((plans, selected)) = source_home_end_selection_v2(rows, full_homes, normal_homes)? else { return Ok(None) };
        for (index, (_, subject, operation)) in plans.into_iter().enumerate() {
            let origin = RootHomeReleaseOriginV1 { subject, exit: exit.clone(), operation };
            if selected.contains(&index) { normal.push(origin.clone()); }
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
            scalar: None, stored_map_get: None,
            null_return: None,
            return_expression: None,
            text_return_hold: None,
            implicit_void: None,
            full: full.into_boxed_slice(),
            normal,
            acquisition_fault,
        })
    }

    pub(super) fn prepend_argument_maps(
        &mut self,
        prefix: Vec<RootHomeReleaseOriginV1>,
    ) -> Result<(), String> {
        if self
            .direct
            .as_ref()
            .is_some_and(|(_, value)| value.is_some())
            || self.scalar.as_ref().is_some_and(|(_, _, value)| value.is_some())
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

    /// A completed pending Return result transfers only after prior cleanup.
    /// Its End belongs to later Fault residuals, never Normal/acquisition.
    pub(super) fn prepend_fault_only_result(
        &mut self,
        prefix: Vec<RootHomeReleaseOriginV1>,
    ) -> Result<(), String> {
        if prefix.is_empty() { return Err(freeze("result-cleanup/empty-plan")); }
        let mut full = prefix;
        full.extend_from_slice(&self.full);
        self.replace_sequences(full, &self.normal(), &self.acquisition_fault())
    }

    /// Reorder obligations atomically without replacing their original source
    /// products. New source fields must not require another hand-copy list.
    fn replace_sequences(
        &mut self, full: Vec<RootHomeReleaseOriginV1>, normal: &[RootHomeReleaseOriginV1],
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
        if expected.is_empty() || !self.full.starts_with(expected)
            || self.normal().iter().chain(self.acquisition_fault().iter()).any(|origin|
                expected.iter().any(|pending| pending.subject() == origin.subject()))
        { return Err(freeze("result-cleanup/order-drift")); }
        Ok(())
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn normal(
        &self,
    ) -> Vec<RootHomeReleaseOriginV1> {
        self.normal
            .iter()
            .map(|index| self.full[*index].clone())
            .collect()
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn full(&self) -> &[RootHomeReleaseOriginV1] {
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
        Ok(self.fault_indices_after_normal_step(step)?.into_iter()
            .map(|index| self.full[index].clone()).collect())
    }
}

#[cfg(test)]
#[path = "root_home_cleanup_order_tests.rs"]
mod tests;

#[path = "root_home_direct_result.rs"]
mod direct_result;
