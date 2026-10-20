//! Finished qualified entry declarations borrowed by final consumers.
use super::*;

impl FinalizedRootSourceHandoffV1 {
    /// Original declaration kind, logical ordinal and entry Value remain one
    /// loan. An inferred object view cannot authorize a declared header.
    pub(in crate::mir) fn with_borrowed_parameter_declarations_v1(
        &self, owner: FunctionOwnerIdV1, function: &MirFunction,
        requests: &[(u32, ValueId, Option<&str>)],
        mut visit: impl FnMut(ValueId, Option<hakorune_mir_defs::CanonicalObjectIdV1>, Option<&crate::mir::normal_callable_semantic_package::BorrowedFormalDomainV1>, bool) -> Result<(), String>,
    ) -> Result<(), String> {
        use crate::mir::callable_parameter_contract::CallableParameterContractKindV1 as Kind;
        let entry = self.borrowed_ordinary_entry_source_for_function_v1(owner, function)?;
        let catalog = self.ledger.original_parameter_contracts.as_deref()
            .ok_or_else(|| freeze("borrowed-parameter/catalog"))?;
        let mut owners = catalog.iter().filter(|row| row.owner == owner);
        let declaration = owners.next().ok_or_else(|| freeze("borrowed-parameter/owner"))?;
        if owners.next().is_some() { return Err(freeze("borrowed-parameter/duplicate-owner")); }
        let values = self.borrowed_ordinary_entry_values_v1(owner)?;
        let offset = usize::from(self.ledger.borrowed_ordinary_entry_receiver_required_v1(owner)?);
        let mut seen = std::collections::BTreeSet::new();
        for &(ordinal, value, declared_type) in requests {
            if !seen.insert(ordinal) { return Err(freeze("borrowed-parameter/duplicate-ordinal")); }
            let mut parameters = declaration.parameters.iter().filter(|row| row.ordinal == ordinal);
            let parameter = parameters.next().ok_or_else(|| freeze("borrowed-parameter/ordinal"))?;
            if parameters.next().is_some()
                || entry.formals().iter().filter(|row| **row == (ordinal, parameter.binding)).count() != 1
                || values.iter().filter(|row| **row == (ordinal, parameter.binding, value)).count() != 1
                || function.params.get(ordinal as usize + offset) != Some(&value)
            { return Err(freeze("borrowed-parameter/entry-binding")); }
            let object = match (&parameter.kind, declared_type) {
                (Kind::OpaqueHandle, None) => Ok(None),
                (Kind::DeclaredObject(class), Some(name)) if class.as_ref() == name => {
                    let view = entry.formal_object_view(parameter.binding)?
                        .ok_or_else(|| freeze("borrowed-parameter/declared-view-missing"))?;
                    if !view.is_declared() || view.class() != class.as_ref() {
                        return Err(freeze("borrowed-parameter/declared-view-drift"));
                    }
                    Ok(Some(view.object()))
                }
                _ => Err(freeze("borrowed-parameter/declaration-drift")),
            }?;
            visit(value, object, entry.formal_domain_v1(ordinal, parameter.binding)?, entry.uses_finite_input_v1())?;
        }
        Ok(())
    }

    /// Final zero-call declaration correspondence, shared by independent consumers.
    /// The original entry owner proves its profile and complete-empty incoming.
    pub(in crate::mir) fn borrowed_declaration_for_function_v1(
        &self,
        owner: FunctionOwnerIdV1,
        key: &hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
        function: &MirFunction,
    ) -> Result<(), String> {
        use crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1 as Carrier;
        let entry = self.ledger.checked_borrowed_declaration_source_v1(owner).map_err(|error|
            format!("{error} owner={owner:?} key={key:?} function={}", function.signature.name))?;
        self.borrowed_ordinary_entry_source_for_function_v1(owner, function)?;
        if !self
            .ledger
            .borrowed_ordinary_entry_receiver_required_v1(owner)?
        {
            return Err(freeze("borrowed-declaration/receiver-mode"));
        }
        let values = self.borrowed_ordinary_entry_values_v1(owner)?;
        let carriers = function
            .metadata
            .physical_param_carriers
            .as_deref()
            .ok_or_else(|| freeze("borrowed-declaration/carriers-missing"))?;
        if function.params.len() != entry.formals().len() + 1
            || function.signature.params.len() != function.params.len()
            || carriers.len() != function.params.len()
            || carriers.first() != Some(&Carrier::ExistingCallableI64)
            || key.namespace()
                != hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod
            || key.arity() as usize != entry.formals().len()
            || !matches!(function.signature.params.first(),
                Some(crate::mir::MirType::Box(class)) if class == key.owner())
            || function
                .params
                .iter()
                .copied()
                .collect::<std::collections::BTreeSet<_>>()
                .len()
                != function.params.len()
            || values.iter().any(|(ordinal, _, value)| {
                let slot = *ordinal as usize + 1;
                function.params.get(slot) != Some(value)
                    || carriers.get(slot) != Some(&Carrier::BorrowedTaggedValue)
            })
            || carriers
                .iter()
                .skip(1)
                .any(|carrier| *carrier != Carrier::BorrowedTaggedValue)
            || function.signature.params.iter().skip(1).any(|kind| {
                !matches!(
                    kind,
                    crate::mir::MirType::Unknown | crate::mir::MirType::Integer
                )
            })
        {
            return Err(freeze("borrowed-declaration/entry-correspondence"));
        }
        Ok(())
    }
}
