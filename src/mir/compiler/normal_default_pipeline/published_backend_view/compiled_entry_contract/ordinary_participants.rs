//! Source declaration membership is independent of real ordinary call coordinates.
use super::*;

impl PublishedMirBackendView<'_> {
    pub(super) fn verify_ordinary_function_membership_v1(
        &self,
        program: &super::super::physical_program::PublishedLifecyclePhysicalProgramV1<'_>,
        referenced: &BTreeSet<hakorune_mir_defs::CanonicalSameModuleCallableKeyV1>,
        functions: &BTreeMap<hakorune_mir_defs::CanonicalSameModuleCallableKeyV1, u32>,
    ) -> Result<(), String> {
        let participants = self.retained_borrowed_participants_v1()?;
        let mut covered = referenced.clone();
        for (key, result) in &participants {
            let index = *functions
                .get(key)
                .ok_or_else(|| fault("compiled-entry-participant-missing"))?;
            let function = &program.functions()[index as usize];
            let expected = function.role().ordinary_result()
                .ok_or_else(|| fault("compiled-entry-participant-role"))?;
            if expected != *result
                || function.role().ordinary_target() != Some(key)
                || self.module().canonical_callable_definition_symbol(key) != Some(function.name())
                || function.params().len()
                    != key.arity() as usize + usize::from(function.role().has_receiver())
            {
                return Err(fault("compiled-entry-participant-drift"));
            }
            covered.insert(key.clone());
        }
        if covered != functions.keys().cloned().collect() {
            return Err(fault("compiled-entry-ordinary-unissued"));
        }
        Ok(())
    }
}
