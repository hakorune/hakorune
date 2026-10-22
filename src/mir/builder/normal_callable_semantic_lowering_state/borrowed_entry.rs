//! Join selected source formals to values allocated by the existing entry.
use super::*;

type PreparedEntryValues = Option<Result<Box<[(u32, BindingRefV1, ValueId)]>, String>>;
type PreparedCarriers = (
    Box<[crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1]>,
    Vec<(usize, ValueId)>,
);

impl CallableSemanticLoweringState {
    /// Lends the existing installed raw callable entry identity; no BindingId
    /// authority is installed or synthesized for this legacy physical shell.
    pub(in crate::mir::builder) fn checked_compare_entry_owner_v1(
        &self,
    ) -> Result<crate::mir::resolved_semantics::FunctionOwnerIdV1, String> {
        if !self.entry_installed {
            return Err(freeze("borrowed-compare/entry-not-installed"));
        }
        Ok(self.owner)
    }

    /// Pre-effect projection onto the ordinary signature's existing column.
    /// This does not authorize backend expansion or a borrowed Invoke.
    pub(in crate::mir::builder) fn prepare_borrowed_entry_carriers(
        &self,
        entry: &PreparedCallableEntryValuesV1,
        builder: &crate::mir::MirBuilder,
    ) -> Result<Option<PreparedCarriers>, String> {
        self.prepare_borrowed_entry_carriers_for_shape(entry, builder, true)
    }

    pub(in crate::mir::builder) fn prepare_borrowed_entry_carriers_for_shape(
        &self,
        entry: &PreparedCallableEntryValuesV1,
        builder: &crate::mir::MirBuilder,
        has_physical_formals: bool,
    ) -> Result<Option<PreparedCarriers>, String> {
        use crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1 as Carrier;
        let prepared = self.prepare_borrowed_entry_values(entry)?;
        if matches!(&prepared, Some(Err(_))) { return Ok(None); }
        if !has_physical_formals {
            if matches!(&prepared, Some(Ok(rows)) if !rows.is_empty()) {
                return Err(freeze("injected-entry/borrowed-formal-lane"));
            }
            let function = builder.function_state.current_function.as_ref()
                .ok_or_else(|| freeze("dynamic-entry/no-current-function"))?;
            if self.receiver.is_some() || entry.receiver().is_some() || entry.parameters().is_empty()
                || !function.params.is_empty() || !function.signature.params.is_empty()
                || !function.metadata.declared_param_decls.is_empty()
                || function.metadata.physical_param_carriers.is_some() {
                return Err(freeze("injected-entry/physical-shape-drift"));
            }
            return Ok(None);
        }
        let Some(Ok(_)) = &prepared else {
            // Keep a pending source rejection in the existing entry ledger.
            return self.prepare_dynamic_entry_carriers(entry, builder);
        };
        let row = prepared.as_ref().unwrap();
        let declared = self
            .ordinary_new_claim_ledger
            .as_ref()
            .ok_or_else(|| freeze("borrowed-entry/source-ledger-missing"))?
            .borrowed_ordinary_entry_declared_classes_v1(self.owner, row)?;
        let rows = row.as_ref().unwrap();
        let function = builder
            .function_state
            .current_function
            .as_ref()
            .ok_or_else(|| freeze("borrowed-entry/no-current-function"))?;
        let receiver_required = self
            .ordinary_new_claim_ledger
            .as_ref()
            .unwrap()
            .borrowed_ordinary_entry_receiver_required_v1(self.owner)?;
        let offset = usize::from(receiver_required);
        if (receiver_required
            && (!matches!(
                function.signature.params.first(),
                Some(crate::mir::MirType::Box(_))
            ) || function.params.first().copied() != entry.receiver()))
            || (!receiver_required
                && function
                    .metadata
                    .declared_param_decls
                    .iter()
                    .any(|row| row.implicit_receiver))
            || function.params.get(offset..) != Some(entry.parameters())
            || function.signature.params.len() != function.params.len()
        {
            return Err(freeze("borrowed-entry/physical-signature-drift"));
        }
        let original = function
            .metadata
            .physical_param_carriers
            .as_deref()
            .ok_or_else(|| freeze("borrowed-entry/carrier-column-missing"))?;
        if original.len() != function.params.len()
            || (receiver_required && original.first() != Some(&Carrier::ExistingCallableI64))
            || original.contains(&Carrier::BorrowedTaggedValue)
        {
            return Err(freeze("borrowed-entry/carrier-column-drift"));
        }
        let mut carriers = original.to_vec();
        let mut projected = Vec::new();
        for (ordinal, binding, value) in rows.iter() {
            let index = *ordinal as usize + offset;
            if let Some(class) = declared.get(binding) {
                let declaration = function
                    .metadata
                    .declared_param_decls
                    .get(index)
                    .ok_or_else(|| freeze("borrowed-entry/declared-header-missing"))?;
                let original_type = crate::mir::MirType::Box((*class).into());
                if function.metadata.declared_param_decls.len() != function.params.len()
                    || declaration.implicit_receiver
                    || declaration.declared_type_name.as_deref() != Some(*class)
                    || self.binding_names.get(binding).map(|name| name.as_ref())
                        != Some(declaration.name.as_str())
                    || function.signature.params.get(index) != Some(&original_type)
                    || builder.function_state.type_ctx.value_types.get(value)
                        != Some(&original_type)
                {
                    return Err(freeze("borrowed-entry/declared-header-or-value-drift"));
                }
                projected.push((index, *value));
            } else if !matches!(
                function.signature.params.get(index),
                Some(crate::mir::MirType::Unknown | crate::mir::MirType::Integer)
            ) || function
                .metadata
                .declared_param_decls
                .get(index)
                .is_some_and(|row| row.declared_type_name.is_some())
            {
                return Err(freeze("borrowed-entry/carrier-formal-drift"));
            }
            if function.params.get(index) != Some(value)
                || carriers.get(index) != Some(&Carrier::ExistingCallableI64)
            {
                return Err(freeze("borrowed-entry/carrier-formal-drift"));
            }
            carriers[index] = Carrier::BorrowedTaggedValue;
        }
        Ok(Some((carriers.into_boxed_slice(), projected)))
    }

    fn prepare_dynamic_entry_carriers(
        &self,
        entry: &PreparedCallableEntryValuesV1,
        builder: &crate::mir::MirBuilder,
    ) -> Result<Option<PreparedCarriers>, String> {
        use crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1 as Carrier;
        use crate::mir::MirType;
        let Some(formals) = &self.dynamic_entry_formals else { return Ok(None) };
        if formals.is_empty() { return Ok(None); }
        let receiver_required = self.receiver.is_some();
        let offset = usize::from(receiver_required);
        let function = builder.function_state.current_function.as_ref()
            .ok_or_else(|| freeze("dynamic-entry/no-current-function"))?;
        let original = function.metadata.physical_param_carriers.as_deref()
            .ok_or_else(|| freeze("dynamic-entry/carrier-column-missing"))?;
        if entry.receiver().is_some() != receiver_required
            || self.parameters.len() != entry.parameters().len()
            || function.params.get(offset..) != Some(entry.parameters())
            || function.params.len() != self.parameters.len() + offset
            || function.signature.params.len() != function.params.len()
            || function.metadata.declared_param_decls.len() != function.params.len()
            || original.len() != function.params.len()
            || original.iter().any(|carrier| *carrier != Carrier::ExistingCallableI64)
            || function.params.iter().copied().collect::<BTreeSet<_>>().len() != function.params.len()
            || function.blocks.values().flat_map(|block| block.all_instructions()).any(|row|
                row.dst_value().is_some_and(|dst| function.params.contains(&dst))) {
            return Err(freeze("dynamic-entry/physical-signature-drift"));
        }
        if receiver_required != function.metadata.declared_param_decls.first()
            .is_some_and(|row| row.implicit_receiver)
            || function.metadata.declared_param_decls.iter().skip(offset).any(|row| row.implicit_receiver) {
            return Err(freeze("dynamic-entry/receiver-layout"));
        }
        let mut carriers = original.to_vec();
        let mut projected = Vec::with_capacity(formals.len());
        for (ordinal, binding) in formals.iter() {
            let index = usize::try_from(*ordinal).map_err(|_| freeze("dynamic-entry/formal-ordinal"))?;
            let lane = index.checked_add(offset).ok_or_else(|| freeze("dynamic-entry/formal-lane"))?;
            let value = *entry.parameters().get(index).ok_or_else(|| freeze("dynamic-entry/formal-value"))?;
            let declaration = function.metadata.declared_param_decls.get(lane)
                .ok_or_else(|| freeze("dynamic-entry/declared-formal"))?;
            if self.parameters.get(index) != Some(binding)
                || binding.owner() != self.owner
                || function.params.get(lane) != Some(&value)
                || declaration.implicit_receiver
                || declaration.declared_type_name.is_some()
                || self.binding_names.get(binding).map(|name| name.as_ref()) != Some(declaration.name.as_str())
                || !matches!(function.signature.params.get(lane), Some(MirType::Unknown | MirType::Integer))
                || builder.function_state.type_ctx.value_types.get(&value)
                    .is_some_and(|kind| !matches!(kind, MirType::Unknown | MirType::Integer)) {
                return Err(freeze("dynamic-entry/formal-source-drift"));
            }
            carriers[lane] = Carrier::BorrowedTaggedValue;
            projected.push((lane, value));
        }
        Ok(Some((carriers.into_boxed_slice(), projected)))
    }

    pub(in crate::mir::builder) fn stage_borrowed_entry_formals(
        &mut self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
        formals: Result<Option<Box<[(u32, BindingRefV1)]>>, String>,
    ) -> Result<(), String> {
        if owner != self.owner {
            return Err(freeze("borrowed-entry/source-owner-drift"));
        }
        if self.entry_installed || self.borrowed_entry_formals.is_some() {
            return Err(freeze("borrowed-entry/duplicate-source-preparation"));
        }
        self.borrowed_entry_formals = Some(formals);
        Ok(())
    }

    pub(super) fn prepare_borrowed_entry_values(
        &self,
        entry: &PreparedCallableEntryValuesV1,
    ) -> Result<PreparedEntryValues, String> {
        let formals = match &self.borrowed_entry_formals {
            None | Some(Ok(None)) => return Ok(None),
            Some(Err(issue)) => return Ok(Some(Err(issue.clone()))),
            Some(Ok(Some(formals))) => formals,
        };
        if self.ordinary_new_claim_ledger.is_none() {
            return Err(freeze("borrowed-entry/source-ledger-missing"));
        }
        let receiver_required = self
            .ordinary_new_claim_ledger
            .as_ref()
            .unwrap()
            .borrowed_ordinary_entry_receiver_required_v1(self.owner)?;
        if self.receiver.is_some() != receiver_required
            || entry.receiver().is_some() != receiver_required
            || formals.is_empty()
        {
            return Err(freeze("borrowed-entry/instance-shape"));
        }
        let mut seen = BTreeSet::new();
        let mut values = BTreeSet::new();
        let mut rows = Vec::new();
        for (ordinal, binding) in formals.iter() {
            let index = *ordinal as usize;
            if binding.owner() != self.owner
                || self.parameters.get(index) != Some(binding)
                || !seen.insert(*ordinal)
            {
                return Err(freeze("borrowed-entry/formal-value-identity"));
            }
            let value = entry
                .parameters()
                .get(index)
                .copied()
                .ok_or_else(|| freeze("borrowed-entry/formal-value-missing"))?;
            if Some(value) == entry.receiver()
                || entry
                    .parameters()
                    .iter()
                    .filter(|candidate| **candidate == value)
                    .count()
                    != 1
                || !values.insert(value)
            {
                return Err(freeze("borrowed-entry/formal-value-collision"));
            }
            rows.push((*ordinal, *binding, value));
        }
        let result = Ok(rows.into_boxed_slice());
        Ok(Some(result))
    }
}

#[cfg(test)]
#[path = "borrowed_entry_tests.rs"]
mod tests;
