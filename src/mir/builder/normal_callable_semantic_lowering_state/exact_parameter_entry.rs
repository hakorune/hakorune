//! Mechanical join of original exact-usize source rows to existing entry values.
use super::*;
use crate::mir::{MirBuilder, MirType};

impl CallableSemanticLoweringState {
    pub(in crate::mir::builder) fn stage_exact_usize_entry_formals(
        &mut self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
        rows: Box<[(u32, BindingRefV1)]>,
    ) -> Result<(), String> {
        if owner != self.owner || self.entry_installed || self.exact_usize_entry_formals.is_some() {
            return Err(freeze("exact-usize-entry/source-owner-or-stage-drift"));
        }
        let mut seen = BTreeSet::new();
        for (ordinal, binding) in rows.iter() {
            if binding.owner() != owner
                || self.parameters.get(*ordinal as usize) != Some(binding)
                || !seen.insert(*ordinal)
            {
                return Err(freeze("exact-usize-entry/source-binding-drift"));
            }
        }
        self.exact_usize_entry_formals = Some(rows);
        Ok(())
    }

    /// Preflight all rows before the existing entry installation or type commit.
    pub(in crate::mir::builder) fn prepare_exact_usize_entry_projection(
        &self,
        entry: &PreparedCallableEntryValuesV1,
        builder: &MirBuilder,
    ) -> Result<Vec<(usize, ValueId)>, String> {
        let Some(rows) = &self.exact_usize_entry_formals else {
            return Ok(Vec::new());
        };
        if rows.is_empty() {
            return Ok(Vec::new());
        }
        if self.entry_installed {
            return Err(freeze("exact-usize-entry/repeated-entry"));
        }
        let function = builder
            .function_state
            .current_function
            .as_ref()
            .ok_or_else(|| freeze("exact-usize-entry/function-missing"))?;
        let offset = usize::from(entry.receiver().is_some());
        use crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1 as Carrier;
        let carriers = function
            .metadata
            .physical_param_carriers
            .as_deref()
            .ok_or_else(|| freeze("exact-usize-entry/carrier-missing"))?;
        if carriers.len() != function.params.len()
            || (offset == 1 && carriers.first() != Some(&Carrier::ExistingCallableI64))
        {
            return Err(freeze("exact-usize-entry/carrier-coverage-drift"));
        }
        if function.params.get(offset..) != Some(entry.parameters())
            || function.params.len() != function.signature.params.len()
            || function.params.len() != function.metadata.declared_param_decls.len()
            || (offset == 1 && function.params.first().copied() != entry.receiver())
        {
            return Err(freeze("exact-usize-entry/formal-coverage-drift"));
        }
        let mut projected = Vec::with_capacity(rows.len());
        for (ordinal, binding) in rows.iter() {
            let index = *ordinal as usize + offset;
            if carriers.get(index) != Some(&Carrier::ExistingCallableI64) {
                return Err(freeze("exact-usize-entry/carrier-formal-drift"));
            }
            let value = entry
                .parameters()
                .get(*ordinal as usize)
                .copied()
                .ok_or_else(|| freeze("exact-usize-entry/value-missing"))?;
            let decl = &function.metadata.declared_param_decls[index];
            let original = MirType::Box("usize".into());
            let ty = &function.signature.params[index];
            if decl.implicit_receiver
                || decl.declared_type_name.as_deref() != Some("usize")
                || self.binding_names.get(binding).map(|name| name.as_ref())
                    != Some(decl.name.as_str())
                || self.parameters.get(*ordinal as usize) != Some(binding)
                || Some(value) == entry.receiver()
                || function
                    .params
                    .iter()
                    .filter(|candidate| **candidate == value)
                    .count()
                    != 1
                || (ty != &original && ty != &MirType::Integer)
                || builder.function_state.type_ctx.value_types.get(&value) != Some(ty)
            {
                return Err(freeze("exact-usize-entry/declaration-or-value-drift"));
            }
            projected.push((index, value));
        }
        Ok(projected)
    }
}
