//! Retain original exit obligations across the existing affine root progress.
//! Sequence membership uses exact source origins, never emitted MIR discovery.
use super::{freeze, RootHomeReleaseOriginV1};

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package) struct RootHomeCleanupOrderV1 {
    pub(super) null_return: Option<super::null_return::TerminalNullReturnProducerV1>,
    direct: Option<(
        super::super::super::completion_index::DirectRootCleanupSourceV1,
        Option<crate::mir::ValueId>,
    )>,
    full: Box<[RootHomeReleaseOriginV1]>,
    normal: Box<[usize]>,
    acquisition_fault: Box<[usize]>,
}
impl RootHomeCleanupOrderV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn ordinary(
        full: Vec<RootHomeReleaseOriginV1>,
    ) -> Result<Self, String> {
        Self::from_sequences(full.clone(), &full, &full)
    }

    /// Expand original full exit Homes once; Normal selects whole Home plans.
    pub(super) fn from_home_end_plans(
        rows: &std::collections::BTreeMap<super::OwnedExprSiteV1, super::LocalCommitV1>,
        exit: &super::SourceStmtSiteV1,
        full_homes: &[super::BindingRefV1],
        normal_homes: &[super::BindingRefV1],
    ) -> Result<Option<Self>, String> {
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
        let mut full = Vec::new();
        let mut normal = Vec::new();
        let mut available = true;
        for binding in full_homes {
            let home = super::installed_home(rows, *binding).map_err(|error| match error {
                super::HomeLookupError::Missing => freeze("root-home-not-installed"),
                super::HomeLookupError::Duplicate => freeze("duplicate-root-home"),
            })?;
            let end_available = home.end_available();
            available &= end_available;
            if !end_available {
                continue;
            }
            for (subject, operation) in home.end_plan().into_vec() {
                let origin = RootHomeReleaseOriginV1 {
                    subject,
                    exit: exit.clone(),
                    operation,
                };
                if normal_homes.contains(binding) {
                    normal.push(origin.clone());
                }
                full.push(origin);
            }
        }
        if !available {
            return Ok(None);
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
            null_return: None,
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
        {
            return Err(freeze("direct-result-arguments-after-bind"));
        }
        let mut full = prefix.clone();
        full.extend_from_slice(&self.full);
        let mut normal = prefix.clone();
        normal.extend(self.normal());
        let mut acquisition_fault = prefix;
        acquisition_fault.extend(self.acquisition_fault());
        let mut next = Self::from_sequences(full, &normal, &acquisition_fault)?;
        next.direct = self.direct.take();
        next.null_return = self.null_return.take();
        *self = next;
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

    pub(super) fn full(&self) -> &[RootHomeReleaseOriginV1] {
        &self.full
    }

    pub(super) fn acquisition_fault_indices(&self) -> &[usize] {
        &self.acquisition_fault
    }

    pub(super) fn fault_indices_after_normal_step(
        &self,
        step: usize,
    ) -> Result<Vec<usize>, String> {
        let attempted = self
            .normal
            .get(..=step)
            .ok_or_else(|| freeze("root-cleanup-order/normal-step"))?;
        Ok((0..self.full.len())
            .filter(|index| !attempted.contains(index))
            .collect())
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
        let attempted = self
            .normal
            .get(..=step)
            .ok_or_else(|| freeze("root-cleanup-order/normal-step"))?;
        Ok(self
            .full
            .iter()
            .enumerate()
            .filter(|(index, _)| !attempted.contains(index))
            .map(|(_, origin)| origin.clone())
            .collect())
    }
}

#[cfg(test)]
#[path = "root_home_cleanup_order_tests.rs"]
mod tests;

#[path = "root_home_direct_result.rs"]
mod direct_result;
