//! Final-view physical inputs required by the dedicated lifecycle C consumer.
//!
//! This is a physical join of already-issued products.  It neither allocates a
//! layout nor interprets source meaning.

use std::collections::BTreeSet;

use crate::mir::MirInstruction;
use crate::mir::instruction::InvokeOperation;
use crate::mir::function::{ObjectDestructionDispositionV1, TypedObjectFieldStorage};

use super::{
    CompiledEntryContractV1, CompiledEntryRootResultV1, PublishedMirBackendView,
    physical_program::PublishedLifecyclePhysicalProgramV1,
};

/// Runtime diagnostic operation kinds admitted by the selected lifecycle ABI.
/// These are physical runtime calls, never source-level diagnostic sites.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PublishedLifecycleCheckedOperationKindV1 {
    NewBox,
    FieldSet,
    HomeRelease,
    ReclaimUnpublished,
}

impl PublishedLifecycleCheckedOperationKindV1 {
    pub(crate) const fn from_instruction(instruction: &MirInstruction) -> Option<Self> {
        let MirInstruction::Invoke { operation, .. } = instruction else { return None };
        match operation {
            InvokeOperation::NewBox { .. } => Some(Self::NewBox),
            InvokeOperation::FieldSet { .. } => Some(Self::FieldSet),
            InvokeOperation::HomeRelease { .. } => Some(Self::HomeRelease),
            InvokeOperation::ReclaimUnpublished { .. } => Some(Self::ReclaimUnpublished),
            InvokeOperation::Call(_) => None,
        }
    }
}

/// One final-view-issued runtime diagnostic identity at an exact physical row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PublishedLifecycleOperationDiagnosticSiteV1 {
    function: u32,
    block: u32,
    instruction: u32,
    kind: PublishedLifecycleCheckedOperationKindV1,
    site: u64,
}

impl PublishedLifecycleOperationDiagnosticSiteV1 {
    pub(crate) const fn function(&self) -> u32 { self.function }
    pub(crate) const fn block(&self) -> u32 { self.block }
    pub(crate) const fn instruction(&self) -> u32 { self.instruction }
    pub(crate) const fn kind(&self) -> PublishedLifecycleCheckedOperationKindV1 { self.kind }
    pub(crate) const fn site(&self) -> u64 { self.site }
}

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
    entry: CompiledEntryContractV1<'module>,
    layouts: Box<[PublishedLifecyclePhysicalObjectLayoutV1]>,
    diagnostic_sites: Box<[PublishedLifecycleOperationDiagnosticSiteV1]>,
    fault_abi_version: u32,
    storage_profile: u32,
}

impl<'module> PublishedLifecyclePhysicalAbiInputV1<'module> {
    pub(crate) fn entry(&self) -> &CompiledEntryContractV1<'module> { &self.entry }
    pub(crate) fn program(&self) -> &PublishedLifecyclePhysicalProgramV1<'module> {
        self.entry.program()
    }
    pub(crate) fn layouts(&self) -> &[PublishedLifecyclePhysicalObjectLayoutV1] { &self.layouts }
    pub(crate) fn diagnostic_sites(&self) -> &[PublishedLifecycleOperationDiagnosticSiteV1] {
        &self.diagnostic_sites
    }
    pub(crate) fn diagnostic_site_at(
        &self, function: u32, block: u32, instruction: u32,
    ) -> Option<PublishedLifecycleOperationDiagnosticSiteV1> {
        self.diagnostic_sites.iter().copied().find(|site|
            site.function == function && site.block == block && site.instruction == instruction)
    }
    pub(crate) const fn fault_abi_version(&self) -> u32 { self.fault_abi_version }
    pub(crate) const fn storage_profile(&self) -> u32 { self.storage_profile }
}

impl<'module> PublishedMirBackendView<'module> {
    /// Joins selected program coordinates to installed runtime layouts.
    pub(crate) fn issue_lifecycle_physical_abi_input(
        &self,
    ) -> Result<PublishedLifecyclePhysicalAbiInputV1<'module>, String> {
        let entry = self.issue_lifecycle_compiled_entry_contract()?;
        if entry.root_result() != CompiledEntryRootResultV1::I64 {
            return Err(fault("root-result-unavailable"));
        }
        let diagnostic_sites = issue_diagnostic_sites(entry.program())?;
        let storage_profile = self.lifecycle_storage_profile()
            .ok_or_else(|| fault("storage-profile-missing"))? as u32;
        let ids = referenced_objects(entry.program());
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
            entry, layouts: layouts.into_boxed_slice(),
            diagnostic_sites: diagnostic_sites.into_boxed_slice(), fault_abi_version: 1,
            storage_profile,
        })
    }
}

fn issue_diagnostic_sites(
    program: &PublishedLifecyclePhysicalProgramV1<'_>,
) -> Result<Vec<PublishedLifecycleOperationDiagnosticSiteV1>, String> {
    let mut sites = Vec::new();
    let mut coordinates = BTreeSet::new();
    for (function_ordinal, physical_function) in program.functions().iter().enumerate() {
        let function = u32::try_from(function_ordinal).map_err(|_| fault("site-function-overflow"))?;
        for block in physical_function.blocks() {
            for row in block.instructions().iter().copied().chain(std::iter::once(block.terminator())) {
                let Some(kind) = PublishedLifecycleCheckedOperationKindV1::from_instruction(row.instruction()) else {
                    continue;
                };
                let coordinate = (function, block.id().0, row.index());
                if !coordinates.insert(coordinate) {
                    return Err(fault("site-coordinate-duplicate"));
                }
                let site = u64::try_from(sites.len()).map_err(|_| fault("site-overflow"))?;
                sites.push(PublishedLifecycleOperationDiagnosticSiteV1 {
                    function, block: block.id().0, instruction: row.index(), kind, site,
                });
            }
        }
    }
    if sites.is_empty() { return Err(fault("site-missing")); }
    Ok(sites)
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
