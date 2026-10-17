//! Retain the same Main wrapper source/emission through the existing root ledger.
use super::*;
use crate::mir::{BasicBlockId, MirFunction, MirInstruction};
impl OrdinaryNewClaimLedgerV1 {
    /// Lend the same source-only policy to ABI selection and physical emission.
    pub(in crate::mir) fn main_arguments_source_v1(
        &self,
        input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
        identity: &crate::parser::CallableDeclarationIdentityV1,
    ) -> Result<Rc<MainArgumentsSourceV1>, String> {
        let source = self.main_arguments_source.as_ref().ok_or_else(|| fault("source-missing"))?;
        let key = self.app_main_catalog_key.as_ref().ok_or_else(|| fault("source-key"))?;
        let original = self.app_main_identity.as_ref().ok_or_else(|| fault("root-identity"))?;
        if !original.same_as(identity) || !source.corroborates_anchor(input.owner(), original, key)
            || !source.corroborates_input(input, identity) {
            return Err(fault("source-mismatch"));
        }
        Ok(Rc::clone(source))
    }
    pub(in crate::mir) fn retain_main_arguments_v1(
        &self,
        emission: crate::mir::builder::MainArgumentsEmissionV1,
    ) -> Result<(), String> {
        let owner = self
            .root_completion
            .as_ref()
            .and_then(|row| row.as_ref().ok())
            .map(|row| row.owner())
            .ok_or_else(|| fault("root-completion"))?;
        let source = self.main_arguments_source.as_ref().ok_or_else(|| fault("source-missing"))?;
        if !emission.corroborates_source(owner, source) {
            return Err(fault("source-mismatch"));
        }
        let mut slot = self.main_arguments.borrow_mut();
        if slot.is_some() {
            return Err(fault("duplicate-entry"));
        }
        *slot = Some(Rc::new(emission));
        Ok(())
    }
    pub(super) fn main_arguments_original_bindings_v1(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Vec<(BasicBlockId, MirInstruction)> {
        self.main_arguments
            .borrow()
            .as_ref()
            .filter(|row| row.owner() == owner)
            .map_or_else(Vec::new, |row| row.original_bindings())
    }
    pub(crate) fn main_arguments_release_suffix_v1(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
    ) -> Result<Vec<(BasicBlockId, MirInstruction)>, String> {
        if !self
            .main_arguments
            .borrow()
            .as_ref()
            .is_some_and(|row| row.owner() == owner)
        {
            return Ok(Vec::new());
        }
        Ok(self
            .main_arguments_bindings_v1(function)?
            .into_iter()
            .filter(|(_, row)| matches!(row, MirInstruction::ArrayResidenceRelease { .. }))
            .collect())
    }
    pub(crate) fn main_arguments_bindings_v1(
        &self,
        function: &MirFunction,
    ) -> Result<Vec<(BasicBlockId, MirInstruction)>, String> {
        self.main_arguments_projected_bindings_v1(function, |binding| Ok(binding.clone()))
    }
    pub(super) fn main_arguments_projected_bindings_v1(
        &self,
        function: &MirFunction,
        map: impl FnMut(
            &(BasicBlockId, MirInstruction),
        ) -> Result<(BasicBlockId, MirInstruction), String>,
    ) -> Result<Vec<(BasicBlockId, MirInstruction)>, String> {
        let slot = self.main_arguments.borrow();
        let Some(emission) = slot.as_ref() else {
            return Ok(Vec::new());
        };
        let owner = self
            .root_completion
            .as_ref()
            .and_then(|row| row.as_ref().ok())
            .map(|row| row.owner())
            .ok_or_else(|| fault("root-completion"))?;
        let source = self.main_arguments_source.as_ref().ok_or_else(|| fault("source-missing"))?;
        if !emission.corroborates_source(owner, source) {
            return Err(fault("source-mismatch"));
        }
        emission.projected_bindings(function, map)
    }
}
fn fault(reason: &str) -> String {
    format!("[freeze:contract][main-arguments/{reason}]")
}
