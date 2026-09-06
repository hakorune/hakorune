//! Final-view physical inputs required by the dedicated lifecycle C consumer.
//!
//! This is a physical join of already-issued products.  It neither allocates a
//! layout nor interprets source meaning.

use std::collections::BTreeSet;

use crate::mir::MirInstruction;
use crate::mir::instruction::InvokeOperation;
use crate::mir::function::{ObjectDestructionDispositionV1, TypedObjectFieldStorage};

use super::{PublishedMirBackendView, physical_program::PublishedLifecyclePhysicalProgramV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PublishedLifecyclePhysicalFieldLayoutV1 {
    object_id: u32,
    declaration_ordinal: u32,
    runtime_slot: u32,
    storage_kind: u32,
}

impl PublishedLifecyclePhysicalFieldLayoutV1 {
    pub(crate) const fn object_id(&self) -> u32 { self.object_id }
    pub(crate) const fn declaration_ordinal(&self) -> u32 { self.declaration_ordinal }
    pub(crate) const fn runtime_slot(&self) -> u32 { self.runtime_slot }
    pub(crate) const fn storage_kind(&self) -> u32 { self.storage_kind }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PublishedLifecyclePhysicalObjectLayoutV1 {
    object_id: u32,
    runtime_type_id: u32,
    field_count: u32,
    fields: Box<[PublishedLifecyclePhysicalFieldLayoutV1]>,
}

impl PublishedLifecyclePhysicalObjectLayoutV1 {
    pub(crate) const fn object_id(&self) -> u32 { self.object_id }
    pub(crate) const fn runtime_type_id(&self) -> u32 { self.runtime_type_id }
    pub(crate) const fn field_count(&self) -> u32 { self.field_count }
    pub(crate) fn fields(&self) -> &[PublishedLifecyclePhysicalFieldLayoutV1] { &self.fields }
}

/// One C-consumer input whose parts were issued by the same final view.
#[derive(Debug, Clone)]
pub(crate) struct PublishedLifecyclePhysicalAbiInputV1<'module> {
    program: PublishedLifecyclePhysicalProgramV1<'module>,
    layouts: Box<[PublishedLifecyclePhysicalObjectLayoutV1]>,
    fault_abi_version: u32,
    storage_profile: u32,
}

impl<'module> PublishedLifecyclePhysicalAbiInputV1<'module> {
    pub(crate) fn program(&self) -> &PublishedLifecyclePhysicalProgramV1<'module> { &self.program }
    pub(crate) fn layouts(&self) -> &[PublishedLifecyclePhysicalObjectLayoutV1] { &self.layouts }
    pub(crate) const fn fault_abi_version(&self) -> u32 { self.fault_abi_version }
    pub(crate) const fn storage_profile(&self) -> u32 { self.storage_profile }
}

impl<'module> PublishedMirBackendView<'module> {
    /// Joins selected program coordinates to installed runtime layouts.
    pub(crate) fn issue_lifecycle_physical_abi_input(
        &self,
    ) -> Result<PublishedLifecyclePhysicalAbiInputV1<'module>, String> {
        let program = self.issue_lifecycle_physical_program()?;
        let storage_profile = self.lifecycle_storage_profile()
            .ok_or_else(|| fault("storage-profile-missing"))? as u32;
        let ids = referenced_objects(&program);
        let definitions = self.module().canonical_object_definitions()
            .ok_or_else(|| fault("object-definitions-missing"))?;
        let mut layouts = Vec::with_capacity(ids.len());
        for object_id in ids {
            let definition = definitions.get(object_id as usize)
                .ok_or_else(|| fault("object-definition-missing"))?;
            if definition.destruction_disposition() != ObjectDestructionDispositionV1::PlainI64NoHook {
                return Err(fault("object-destruction"));
            }
            let layout = definition.runtime_layout().ok_or_else(|| fault("layout-not-issued"))?
                .as_ref().map_err(|_| fault("layout-unavailable"))?;
            if layout.field_count as usize != layout.fields.len() {
                return Err(fault("layout-field-count"));
            }
            let fields = layout.fields.iter().enumerate().map(|(ordinal, field)| {
                if field.storage != TypedObjectFieldStorage::I64 || field.slot != ordinal as u32 {
                    return Err(fault("layout-field-drift"));
                }
                Ok(PublishedLifecyclePhysicalFieldLayoutV1 {
                    object_id, declaration_ordinal: ordinal as u32,
                    runtime_slot: field.slot, storage_kind: 1,
                })
            }).collect::<Result<Vec<_>, String>>()?;
            layouts.push(PublishedLifecyclePhysicalObjectLayoutV1 {
                object_id, runtime_type_id: layout.type_id, field_count: layout.field_count,
                fields: fields.into_boxed_slice(),
            });
        }
        Ok(PublishedLifecyclePhysicalAbiInputV1 {
            program, layouts: layouts.into_boxed_slice(), fault_abi_version: 1,
            storage_profile,
        })
    }
}

fn referenced_objects(program: &PublishedLifecyclePhysicalProgramV1<'_>) -> BTreeSet<u32> {
    let mut ids = BTreeSet::new();
    for function in program.functions() {
        for block in function.blocks() {
            for row in block.instructions().iter().copied().chain(std::iter::once(block.terminator())) {
                match row.instruction() {
                    MirInstruction::ObjectFieldGet { field, .. } => { ids.insert(field.object().declaration_index()); }
                    MirInstruction::Invoke { operation, .. } => match operation {
                        InvokeOperation::NewBox { object }
                        | InvokeOperation::HomeRelease { object, .. }
                        | InvokeOperation::ReclaimUnpublished { object, .. } => { ids.insert(object.declaration_index()); }
                        InvokeOperation::FieldSet { field, .. } => { ids.insert(field.object().declaration_index()); }
                        InvokeOperation::Call(_) => {}
                    },
                    _ => {}
                }
            }
        }
    }
    ids
}

fn fault(reason: &str) -> String {
    format!("[freeze:contract][published-lifecycle-physical-abi/{reason}]")
}
