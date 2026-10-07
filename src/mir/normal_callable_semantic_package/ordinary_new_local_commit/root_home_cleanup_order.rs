//! Retain original exit obligations across the existing affine root progress.
//! Sequence membership uses exact source origins, never emitted MIR discovery.
use super::{freeze, RootHomeReleaseOriginV1};

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package) struct RootHomeCleanupOrderV1 {
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
            full: full.into_boxed_slice(),
            normal,
            acquisition_fault,
        })
    }

    pub(super) fn prepend_argument_maps(
        &mut self,
        prefix: Vec<RootHomeReleaseOriginV1>,
    ) -> Result<(), String> {
        let mut full = prefix.clone();
        full.extend_from_slice(&self.full);
        let mut normal = prefix.clone();
        normal.extend(self.normal());
        let mut acquisition_fault = prefix;
        acquisition_fault.extend(self.acquisition_fault());
        *self = Self::from_sequences(full, &normal, &acquisition_fault)?;
        Ok(())
    }

    pub(super) fn normal(&self) -> Vec<RootHomeReleaseOriginV1> {
        self.normal
            .iter()
            .map(|index| self.full[*index].clone())
            .collect()
    }

    pub(super) fn full(&self) -> &[RootHomeReleaseOriginV1] {
        &self.full
    }

    pub(super) fn acquisition_fault(&self) -> Vec<RootHomeReleaseOriginV1> {
        self.acquisition_fault
            .iter()
            .map(|index| self.full[*index].clone())
            .collect()
    }

    pub(super) fn fault_after_normal_step(
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
