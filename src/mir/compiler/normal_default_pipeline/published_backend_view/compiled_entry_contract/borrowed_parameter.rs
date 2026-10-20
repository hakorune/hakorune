//! Join the original formal declaration to the already-closed operand image.
use super::*;

impl CompiledEntryContractV1<'_> {
    pub(crate) fn corroborate_borrowed_parameters_v1(
        &self, module: &crate::mir::MirModule, function_index: u32,
        requests: &[(u32, ValueId, Option<&str>)],
    ) -> Result<(), String> {
        if !self.has_borrowed_entry(function_index) {
            return Err(fault("borrowed-parameter/entry-unclosed"));
        }
        let physical = self.program.functions().get(function_index as usize)
            .ok_or_else(|| fault("borrowed-parameter/function-index"))?;
        let key = physical.role().ordinary_target()
            .ok_or_else(|| fault("borrowed-parameter/function-role"))?;
        let function = module.functions.get(physical.name())
            .ok_or_else(|| fault("borrowed-parameter/function-missing"))?;
        if module.canonical_callable_definition_symbol(key) != Some(physical.name()) {
            return Err(fault("borrowed-parameter/function-membership"));
        }
        let owner = self.program.handoff().callables().and_then(|callables|
            callables.owner_for_canonical_key(key))
            .ok_or_else(|| fault("borrowed-parameter/source-owner"))?;
        let source = self.program.handoff().root_source()
            .ok_or_else(|| fault("borrowed-parameter/source-missing"))?;
        source.with_borrowed_parameter_declarations_v1(owner, function, requests, |value, object, domain, finite_input| {
            if self.borrowed_formal_domain_v1(function_index, value.0) != domain
                || self.has_finite_borrowed_input_v1(function_index) != finite_input {
                return Err(fault("borrowed-parameter/closed-domain"));
            }
            if let Some(object) = object {
                // The closed use map is keyed by ValueId, never formal ordinal.
                if self.borrowed_object_view(function_index, value.0) != Some(object.declaration_index()) {
                    return Err(fault("borrowed-parameter/closed-object-view"));
                }
            }
            Ok(())
        })
    }
}
