//! Corroborate one source-owned store after the independent use closures.
use super::*;

impl CompiledEntryContractV1<'_> {
    /// The exact source-owned whole RHS discharges the signed range premise.
    /// Its source SUM and physical producer checks are replayed by the shared
    /// final projection; neither MIR Integer nor a coordinate alone suffices.
    pub(crate) fn corroborate_qualified_i64_field_write_v1(
        &self, module: &crate::mir::MirModule, function_index: u32,
        block: crate::mir::BasicBlockId, instruction_index: usize, instruction: &MirInstruction,
    ) -> Result<bool, String> {
        let physical = self.program.functions().get(function_index as usize)
            .ok_or_else(|| fault("borrowed-write/function-index"))?;
        let source = self.program.handoff().root_source().ok_or_else(|| fault("borrowed-write/source-missing"))?;
        let owner = if let Some(key) = physical.role().ordinary_target() {
            if module.canonical_callable_definition_symbol(key) != Some(physical.name()) {
                return Err(fault("borrowed-write/function-membership"));
            }
            self.program.handoff().callables().and_then(|callables| callables.owner_for_canonical_key(key))
                .ok_or_else(|| fault("borrowed-write/source-owner"))?
        } else if matches!(physical.role(), super::super::physical_program::PublishedLifecyclePhysicalFunctionRoleV1::Root { .. }) {
            source.owner()
        } else { return Ok(false) };
        let function = module.functions.get(physical.name()).ok_or_else(|| fault("borrowed-write/function-missing"))?;
        let actual = function.blocks.get(&block).and_then(|body| body.all_instructions().nth(instruction_index))
            .ok_or_else(|| fault("borrowed-write/instruction-missing"))?;
        if !std::ptr::eq(actual, instruction) { return Err(fault("borrowed-write/function-membership")); }
        let mut found = 0;
        source.with_finished_owned_field_writes_v1(owner, function, |row| {
            if row.invoke.1 == (block, instruction_index) {
                if &row.invoke.0.1 != instruction { return Err(fault("borrowed-write/source-row")); }
                found += 1;
            }
            Ok(())
        })?;
        if found > 1 { return Err(fault("borrowed-write/row-uncovered")); }
        Ok(found == 1)
    }
    pub(crate) fn corroborate_borrowed_usize_field_write_v1(
        &self, module: &crate::mir::MirModule, function_index: u32,
        block: crate::mir::BasicBlockId, instruction_index: usize,
        instruction: &MirInstruction,
    ) -> Result<(), String> {
        if !self.has_borrowed_entry(function_index) {
            return Err(fault("borrowed-write/entry-unclosed"));
        }
        let physical = self.program.functions().get(function_index as usize)
            .ok_or_else(|| fault("borrowed-write/function-index"))?;
        let key = physical.role().ordinary_target().ok_or_else(|| fault("borrowed-write/function-role"))?;
        let owner = self.program.handoff().callables().and_then(|callables| callables.owner_for_canonical_key(key))
            .ok_or_else(|| fault("borrowed-write/source-owner"))?;
        let source = self.program.handoff().root_source().ok_or_else(|| fault("borrowed-write/source-missing"))?;
        let function = module.functions.get(physical.name()).ok_or_else(|| fault("borrowed-write/function-missing"))?;
        let actual = function.blocks.get(&block).and_then(|body| body.all_instructions().nth(instruction_index))
            .ok_or_else(|| fault("borrowed-write/instruction-missing"))?;
        if !std::ptr::eq(actual, instruction) || module.canonical_callable_definition_symbol(key) != Some(physical.name()) {
            return Err(fault("borrowed-write/function-membership"));
        }
        let mut found = 0;
        source.with_finished_borrowed_field_writes_v1(owner, function, |loan, invoke, _, _| {
            if invoke.1 == (block, instruction_index) {
                if &invoke.0.1 != instruction || loan.field_source().declared_type_name() != Some("usize") {
                    return Err(fault("borrowed-write/source-row"));
                }
                found += 1;
            }
            Ok(())
        })?;
        if found != 1 { return Err(fault("borrowed-write/row-uncovered")); }
        Ok(())
    }
}
