//! Dynamic provider rows are published from canonical declarations and
//! completed source results. Runtime values never issue a source target.

use super::super::{
    physical_abi::HAKO_LLVMC_LIFECYCLE_STORAGE_I64, PublishedLifecyclePhysicalAbiInputV1,
    PublishedMirBackendView, PublishedObjectStorageProfileV1,
};
use super::{
    fault, object_identity, BasicBlockId, MirFunction, MirInstruction, MirModule,
    PublishedLifecyclePhysicalFunctionRoleV1 as Role, PublishedLifecyclePhysicalFunctionV1,
};
use crate::mir::finalized_root_handoff::FinalizedRootHandoffV1;
use crate::mir::function::ObjectDestructionDispositionV1;
use crate::mir::instruction::{DynamicInvocationOperandV1 as Operand, InvokeOperation};
use hakorune_mir_defs::{CanonicalObjectIdV1, CanonicalSameModuleCallableKeyV1 as Key};
use serde_json::{json, Value};
use std::{collections::BTreeSet, rc::Rc};

#[derive(Debug, Clone)]
pub(super) struct ProviderV1 {
    key: Key,
    object: CanonicalObjectIdV1,
    parameters: Box<[u32]>,
}

#[derive(Debug)]
pub(in crate::mir::compiler::normal_default_pipeline) struct ProgramDispatchV1 {
    providers: Box<[ProviderV1]>,
    explicit_typed_input: bool,
}

impl ProgramDispatchV1 {
    fn issue(module: &MirModule, handoff: &FinalizedRootHandoffV1) -> Result<Option<Self>, String> {
        let mut selectors = BTreeSet::new();
        let mut explicit_typed_input = false;
        for term in module
            .functions
            .values()
            .flat_map(|f| f.blocks.values())
            .filter_map(|block| block.terminator.as_ref())
        {
            let MirInstruction::Invoke {
                operation: InvokeOperation::DynamicInvocation(op),
                ..
            } = term
            else {
                continue;
            };
            match op.receiver() {
                Operand::BorrowedTypedObject(_) => explicit_typed_input = true,
                Operand::TaggedDynamic(_) => {}
                // Existing result carriers and primitive receivers cannot
                // hold a program typed-store borrow.
                _ => continue,
            }
            let arity =
                u32::try_from(op.arguments().len()).map_err(|_| fault("dynamic-dispatch-arity"))?;
            selectors.insert((op.selector(), arity));
        }
        if selectors.is_empty() {
            return Ok(None);
        }
        let contracts = handoff
            .callables()
            .ok_or_else(|| fault("dynamic-dispatch-contracts"))?;
        let providers = contracts
            .declared_instance_keys_v1()
            .iter()
            .filter(|key| selectors.contains(&(key.name(), key.arity())))
            .map(|key| admit_provider(module, handoff, key))
            .collect::<Result<_, _>>()?;
        Ok(Some(Self {
            providers,
            explicit_typed_input,
        }))
    }

    pub(super) fn objects(&self) -> impl Iterator<Item = u32> + '_ {
        self.providers
            .iter()
            .map(|row| row.object.declaration_index())
    }

    pub(super) fn wire(
        &self,
        functions: &[PublishedLifecyclePhysicalFunctionV1<'_>],
        eligible_layouts: bool,
    ) -> Result<Option<Vec<Value>>, String> {
        if eligible_layouts {
            return wire(&self.providers, functions).map(Some);
        }
        if self.explicit_typed_input || !self.providers.is_empty() {
            return Err(fault("dynamic-dispatch-layout"));
        }
        Ok(None)
    }
}

impl PublishedMirBackendView<'_> {
    pub(in crate::mir::compiler::normal_default_pipeline) fn retain_program_dispatch_v1(
        &mut self,
    ) -> Result<(), String> {
        self.program_dispatch = self
            .retained_handoff
            .map(|handoff| ProgramDispatchV1::issue(self.module(), handoff))
            .transpose()?
            .flatten()
            .map(Rc::new);
        Ok(())
    }

    pub(in crate::mir::compiler::normal_default_pipeline) fn dynamic_provider_keys_v1(
        &self,
    ) -> impl Iterator<Item = &Key> {
        self.program_dispatch
            .iter()
            .flat_map(|plan| plan.providers.iter().map(|row| &row.key))
    }
}

fn admit_provider(
    module: &MirModule,
    handoff: &FinalizedRootHandoffV1,
    key: &Key,
) -> Result<ProviderV1, String> {
    let refuse = |reason| format!("{} {key:?}", fault(reason));
    if module.canonical_callable_definition_symbol(key).is_none() {
        return Err(refuse("dynamic-dispatch-definition"));
    }
    let object = object_identity::ordinary_receiver_object(module, key)?
        .ok_or_else(|| refuse("dynamic-dispatch-receiver"))?;
    let definition = module
        .canonical_object_definition(object)
        .ok_or_else(|| refuse("dynamic-dispatch-definition"))?;
    if definition.destruction_disposition() != ObjectDestructionDispositionV1::NUMERIC_NO_HOOK {
        return Err(refuse("dynamic-dispatch-receiver-layout"));
    }
    let contracts = handoff
        .callables()
        .ok_or_else(|| refuse("dynamic-dispatch-contracts"))?;
    let selected = crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key.clone());
    let result = contracts
        .completed_result(&selected)
        .ok_or_else(|| refuse("dynamic-dispatch-result-source"))?;
    if !result.has_complete_i64_result_v1() {
        return Err(refuse("dynamic-dispatch-result-i64-exits"));
    }
    let parameters = contracts
        .dynamic_dispatch_parameters_v1(key)
        .ok_or_else(|| refuse("dynamic-dispatch-parameters"))?;
    if parameters.len() != key.arity() as usize {
        return Err(refuse("dynamic-dispatch-arity"));
    }
    Ok(ProviderV1 {
        key: key.clone(),
        object,
        parameters: parameters.into_boxed_slice(),
    })
}

fn wire(
    rows: &[ProviderV1],
    functions: &[PublishedLifecyclePhysicalFunctionV1<'_>],
) -> Result<Vec<Value>, String> {
    let mut keys = BTreeSet::new();
    rows.iter().map(|row| {
        let mut physical = functions.iter().enumerate().filter(|(_, function)| {
            matches!(function.role(), super::PublishedLifecyclePhysicalFunctionRoleV1::OrdinaryI64 { key, receiver_object }
                if key == &row.key && *receiver_object == Some(row.object))
        });
        let (target, _) = physical.next().ok_or_else(|| fault("dynamic-dispatch-target"))?;
        if physical.next().is_some() || !keys.insert((row.object, row.key.name(), row.key.arity())) {
            return Err(fault("dynamic-dispatch-duplicate"));
        }
        Ok(json!({
            "object_id": row.object.declaration_index(), "selector": row.key.name(),
            "arity": row.key.arity(), "parameters": row.parameters, "target": target,
        }))
    }).collect()
}

impl PublishedLifecyclePhysicalAbiInputV1<'_> {
    pub(in crate::mir::compiler::normal_default_pipeline::published_backend_view) fn has_program_dispatch_layouts_v1(
        &self,
    ) -> bool {
        self.storage_profile() == Some(PublishedObjectStorageProfileV1::SafeMutex.wire_id())
            && !self.layouts().is_empty()
            && self.layouts().iter().all(|layout| {
                layout.direct_placement().is_none()
                    && layout.owned_residences().is_empty()
                    && layout.owned_object_residences().is_empty()
                    && layout
                        .fields()
                        .iter()
                        .all(|field| field.storage_kind() == HAKO_LLVMC_LIFECYCLE_STORAGE_I64)
            })
    }
}

/// Physical instruction admission lends the exact checked source coordinates.
/// Result and cleanup liveness are independently checked by the V4 flow.
pub(super) fn published_operations_v1(
    view: Option<&PublishedMirBackendView<'_>>,
    function: &MirFunction,
    role: &Role,
) -> Result<BTreeSet<(BasicBlockId, usize)>, String> {
    let mut operations = BTreeSet::new();
    if !function.blocks.values().any(|block| {
        matches!(
            block.terminator.as_ref(),
            Some(MirInstruction::Invoke {
                operation: InvokeOperation::DynamicInvocation(_),
                ..
            })
        )
    }) {
        return Ok(operations);
    }
    let Some(view) = view.filter(|view| view.program_dispatch.is_some()) else {
        return Ok(operations);
    };
    let Some(source) = view.retained_root_source() else {
        return Ok(operations);
    };
    let owner = match role {
        Role::Root { .. } => Some(source.owner()),
        Role::BirthUnit { abi } => Some(abi.owner()),
        role => role.ordinary_target().and_then(|key| {
            view.retained_handoff?
                .callables()?
                .owner_for_canonical_key(key)
        }),
    };
    if let Some(owner) = owner {
        source.with_finished_dynamic_receiver_uses_v1(
            owner,
            function,
            |_, _, coordinate, _, _, scalar| {
                operations.insert(coordinate);
                operations.extend(scalar);
                Ok(())
            },
        )?;
    }
    Ok(operations)
}
