//! Final-view physical inputs required by the dedicated lifecycle C consumer.
//!
//! This is a physical join of already-issued products.  It neither allocates a
//! layout nor interprets source meaning.

use std::collections::{BTreeMap, BTreeSet};
#[path = "physical_abi_birth_actuals.rs"]
mod birth_actuals;
use birth_actuals::{BorrowedBirthActualTransportV1, ObjectBirthActualTransportV1};

#[path = "physical_abi_row_obligations.rs"]
mod row_obligations;
use row_obligations::{issue_diagnostic_sites, issue_exact_numeric_checks};

use super::compiled_entry_contract::CompiledEntryFormalKindV1;
use crate::mir::function::TypedObjectFieldStorage;
use crate::mir::instruction::{InvokeOperation, MapInvokeOperation};
use crate::mir::resolved_semantics::OwnedExprSiteV1;
use crate::mir::MirInstruction;

use super::{
    physical_program::PublishedLifecyclePhysicalProgramV1, CompiledEntryContractV1,
    CompiledEntryRootResultV1, PublishedMirBackendView,
};

/// Physical-program.v2 field storage tag; mirrored by the C ABI header.
const HAKO_LLVMC_LIFECYCLE_STORAGE_I64: u32 = 1;
const HAKO_LLVMC_LIFECYCLE_STORAGE_DYNAMIC: u32 = 12;

/// Runtime diagnostic operation kinds admitted by the selected lifecycle ABI.
/// These are physical runtime calls, never source-level diagnostic sites.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PublishedLifecycleCheckedOperationKindV1 {
    NewBox,
    MapNew,
    MapFieldHas,
    MapFieldKeys,
    MapFieldGetIndexed,
    MapFieldInstallIndexed,
    MapFieldDeleteIndexed,
    MapPrepareKey,
    MapInstall,
    MapCheckedGet,
    MapArrayLength,
    MapArrayIndex,
    MapGetText,
    MapEndOutcome,
    MapEnd,
    StringAdd,
    StringMaterialize,
    StringLength,
    StringCompare,
    StringSubstring,
    StringIndexOf,
    StringPrint,
    StringEnd,
    DynamicInvocation,
    DynamicArrayGet,
    DynamicCarrierInteger,
    DynamicCarrierSubI64,
    DynamicCarrierEnd,
    DynamicCarrierArrayClaim,
    DynamicCarrierArrayAppend,
    DynamicCarrierArrayEnd,
    ArrayNew,
    ScriptArgumentsNew,
    ArrayClaim,
    ArrayWrite,
    OwnedArrayClaim,
    OwnedArrayAppend,
    OwnedArrayGet,
    OwnedArrayEnd,
    FieldGet,
    ObjectFieldBorrow,
    FieldSet,
    ObjectFieldSet,
    HomeRelease,
    HomeReleaseIfLive,
    ObjectEndIfOwned,
    FieldResidenceRelease,
    ObjectFieldRelease,
    ReclaimUnpublished,
}

impl PublishedLifecycleCheckedOperationKindV1 {
    pub(crate) const fn from_instruction(instruction: &MirInstruction) -> Option<Self> {
        // A routed bare FieldSet row is a checked store: the kernel call can
        // record a source Fault, so it carries the same diagnostic site as
        // the invoke form.
        if let MirInstruction::FieldSet { .. } = instruction {
            return Some(Self::FieldSet);
        }
        // A routed bare `.set` element write is a checked kernel call on the
        // same authority: its recorded Fault needs the same site identity.
        if let MirInstruction::ArrayElementWrite {
            kind: crate::mir::ArrayElementWriteKind::Set,
            producer: crate::mir::ArrayWriteProducerKind::MethodCall,
            index: Some(_),
            ..
        } = instruction
        {
            return Some(Self::ArrayWrite);
        }
        let MirInstruction::Invoke { operation, .. } = instruction else {
            return None;
        };
        match operation {
            InvokeOperation::OwnedArray(operation) => Some(match operation {
                crate::mir::instruction::OwnedArrayInvokeOperation::Claim { .. } => Self::OwnedArrayClaim,
                crate::mir::instruction::OwnedArrayInvokeOperation::Append { .. } => Self::OwnedArrayAppend,
                crate::mir::instruction::OwnedArrayInvokeOperation::Get { .. } => Self::OwnedArrayGet,
                crate::mir::instruction::OwnedArrayInvokeOperation::End { .. } => Self::OwnedArrayEnd,
            }),
            InvokeOperation::String(operation) => Some(match operation {
                crate::mir::instruction::StringInvokeOperation::Materialize { .. } | crate::mir::instruction::StringInvokeOperation::FieldMaterialize { .. } => Self::StringMaterialize,
                crate::mir::instruction::StringInvokeOperation::Add { .. } => Self::StringAdd,
                crate::mir::instruction::StringInvokeOperation::Length { .. } => Self::StringLength,
                crate::mir::instruction::StringInvokeOperation::Compare { .. } => Self::StringCompare,
                crate::mir::instruction::StringInvokeOperation::Substring { .. } => Self::StringSubstring,
                crate::mir::instruction::StringInvokeOperation::IndexOf { .. } => Self::StringIndexOf,
                crate::mir::instruction::StringInvokeOperation::Print { .. } => Self::StringPrint,
                crate::mir::instruction::StringInvokeOperation::End { .. }
                    | crate::mir::instruction::StringInvokeOperation::EndIfOwned { .. } => Self::StringEnd,
            }),
            InvokeOperation::DynamicInvocation(_) => Some(Self::DynamicInvocation),
            InvokeOperation::DynamicArrayGet { .. } => Some(Self::DynamicArrayGet),
            InvokeOperation::DynamicCarrierInteger { .. } => Some(Self::DynamicCarrierInteger),
            InvokeOperation::DynamicCarrierSubI64 { .. } => Some(Self::DynamicCarrierSubI64),
            InvokeOperation::DynamicCarrierEnd { .. } => Some(Self::DynamicCarrierEnd),
            InvokeOperation::DynamicCarrierArray(operation) => Some(match operation {
                crate::mir::instruction::DynamicCarrierArrayInvokeOperation::Claim { .. } => Self::DynamicCarrierArrayClaim,
                crate::mir::instruction::DynamicCarrierArrayInvokeOperation::Append { .. } => Self::DynamicCarrierArrayAppend,
                crate::mir::instruction::DynamicCarrierArrayInvokeOperation::End { .. } => Self::DynamicCarrierArrayEnd,
            }),
            InvokeOperation::Map(operation) => Some(match operation {
                MapInvokeOperation::New => Self::MapNew,
                MapInvokeOperation::FieldKeys { .. } => Self::MapFieldKeys,
                MapInvokeOperation::FieldHas { .. } => Self::MapFieldHas,
                MapInvokeOperation::FieldGetIndexed { .. } => Self::MapFieldGetIndexed,
                MapInvokeOperation::FieldInstallIndexed { .. } => Self::MapFieldInstallIndexed,
                MapInvokeOperation::FieldDeleteIndexed { .. } => Self::MapFieldDeleteIndexed,
                MapInvokeOperation::PrepareKey { .. } => Self::MapPrepareKey,
                MapInvokeOperation::InstallIndexed { .. }
                | MapInvokeOperation::InstallValue { .. }
                | MapInvokeOperation::InstallText { .. }
                | MapInvokeOperation::InstallEmptyArray { .. }
                | MapInvokeOperation::InstallBorrowedArray { .. } => Self::MapInstall,
                MapInvokeOperation::CheckedGetI64 { .. } => Self::MapCheckedGet,
                MapInvokeOperation::ArrayLength { .. } => Self::MapArrayLength,
                MapInvokeOperation::ArrayIndexMap { .. } => Self::MapArrayIndex,
                MapInvokeOperation::MapGetText { .. } => Self::MapGetText,
                MapInvokeOperation::EndOutcome { .. } => Self::MapEndOutcome,
                MapInvokeOperation::End { .. } => Self::MapEnd,
            }),
            InvokeOperation::NewBox { .. } => Some(Self::NewBox),
            InvokeOperation::FieldGet { .. } => Some(Self::FieldGet),
            InvokeOperation::ObjectFieldBorrow { .. } => Some(Self::ObjectFieldBorrow),
            InvokeOperation::FieldSet { .. } => Some(Self::FieldSet),
            InvokeOperation::ObjectFieldSet { .. } => Some(Self::ObjectFieldSet),
            InvokeOperation::HomeRelease { .. } => Some(Self::HomeRelease),
            InvokeOperation::HomeReleaseIfLive { .. } => Some(Self::HomeReleaseIfLive),
            InvokeOperation::ObjectEndIfOwned { .. } => Some(Self::ObjectEndIfOwned),
            InvokeOperation::OwnedFieldResidenceRelease { .. } => {
                Some(Self::FieldResidenceRelease)
            }
            InvokeOperation::OwnedObjectFieldRelease { .. } => Some(Self::ObjectFieldRelease),
            InvokeOperation::ReclaimUnpublished { .. } => Some(Self::ReclaimUnpublished),
            InvokeOperation::IntrinsicArrayNew => Some(Self::ArrayNew),
            InvokeOperation::ScriptArgumentsNew { .. } => Some(Self::ScriptArgumentsNew),
            InvokeOperation::ArrayStateContractClaim { .. } => Some(Self::ArrayClaim),
            InvokeOperation::ArrayElementWrite { .. } => Some(Self::ArrayWrite),
            InvokeOperation::Birth(_) | InvokeOperation::Call { .. } => None,
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
    pub(crate) const fn function(&self) -> u32 {
        self.function
    }
    pub(crate) const fn block(&self) -> u32 {
        self.block
    }
    pub(crate) const fn instruction(&self) -> u32 {
        self.instruction
    }
    pub(crate) const fn kind(&self) -> PublishedLifecycleCheckedOperationKindV1 {
        self.kind
    }
    pub(crate) const fn site(&self) -> u64 {
        self.site
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PublishedLifecyclePhysicalFieldLayoutV1 {
    object_id: u32,
    declaration_ordinal: u32,
    runtime_slot: u32,
    storage_kind: u32,
}

impl PublishedLifecyclePhysicalFieldLayoutV1 {
    pub(crate) const fn object_id(&self) -> u32 {
        self.object_id
    }
    pub(crate) const fn declaration_ordinal(&self) -> u32 {
        self.declaration_ordinal
    }
    pub(crate) const fn runtime_slot(&self) -> u32 {
        self.runtime_slot
    }
    pub(crate) const fn storage_kind(&self) -> u32 {
        self.storage_kind
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PublishedLifecyclePhysicalObjectLayoutV1 {
    object_id: u32,
    runtime_type_id: u32,
    field_count: u32,
    fields: Box<[PublishedLifecyclePhysicalFieldLayoutV1]>,
    /// Declaration ordinals of this object's owned `ArrayBox` residences,
    /// in declaration order — the same declaration rows the destruction
    /// disposition and residence seal classify. The physical release
    /// consumer walks them newest-first before the object Home; a plain
    /// object carries an empty list. Placement is untouched: no slot or
    /// storage kind changes here.
    owned_residences: Box<[u32]>,
    /// Source-sealed user-object slot and canonical child, in declaration order.
    owned_object_residences: Box<[(u32, u32)]>,
}

impl PublishedLifecyclePhysicalObjectLayoutV1 {
    pub(crate) const fn object_id(&self) -> u32 {
        self.object_id
    }
    pub(crate) const fn runtime_type_id(&self) -> u32 {
        self.runtime_type_id
    }
    pub(crate) const fn field_count(&self) -> u32 {
        self.field_count
    }
    pub(crate) fn fields(&self) -> &[PublishedLifecyclePhysicalFieldLayoutV1] {
        &self.fields
    }
    pub(crate) fn owned_residences(&self) -> &[u32] {
        &self.owned_residences
    }
    pub(crate) fn owned_object_residences(&self) -> &[(u32, u32)] {
        &self.owned_object_residences
    }
}

/// Physical runtime dependency of the retained root cohort.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PublishedLifecycleRuntimeRequirementsV1 {
    TypedObject { storage_profile: u32 },
    NativeArray,
}

/// One final-view-issued exact-numeric runtime-check obligation at an exact
/// physical row. Only bare routed `FieldSet` rows carry one today.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PublishedLifecycleExactNumericCheckV1 {
    function: u32,
    block: u32,
    instruction: u32,
    declared_type_name: String,
}

/// One C-consumer input whose parts were issued by the same final view.
#[derive(Debug, Clone)]
pub(crate) struct PublishedLifecyclePhysicalAbiInputV1<'module> {
    entry: CompiledEntryContractV1<'module>,
    layouts: Box<[PublishedLifecyclePhysicalObjectLayoutV1]>,
    diagnostic_sites: Box<[PublishedLifecycleOperationDiagnosticSiteV1]>,
    exact_numeric_checks: Box<[PublishedLifecycleExactNumericCheckV1]>,
    /// Dominated `new`-argument uses admitted by the finalized draft, keyed
    /// by the argument's own (new site, ordinal) identity. Each admitted
    /// row retains the receiver's exact pair/i64 transport for that same view.
    borrowed_birth_actuals: BTreeMap<(OwnedExprSiteV1, u32), BorrowedBirthActualTransportV1>,
    /// Object-field formal actuals admitted by the sealed
    /// `ObjectFieldStores` contract, keyed like `borrowed_birth_actuals`.
    /// Each row rides the `nullable_kind_payload_v1` lane: `Null` spells
    /// the (0,0) pair, an owned `Handle` actual the (3,payload) move.
    object_birth_actuals: BTreeMap<(OwnedExprSiteV1, u32), ObjectBirthActualTransportV1>,
    process_result_site: u64,
    fault_abi_version: u32,
    runtime_requirements: PublishedLifecycleRuntimeRequirementsV1,
}

impl<'module> PublishedLifecyclePhysicalAbiInputV1<'module> {
    pub(crate) fn entry(&self) -> &CompiledEntryContractV1<'module> {
        &self.entry
    }
    pub(crate) fn program(&self) -> &PublishedLifecyclePhysicalProgramV1<'module> {
        self.entry.program()
    }
    pub(crate) fn layouts(&self) -> &[PublishedLifecyclePhysicalObjectLayoutV1] {
        &self.layouts
    }
    pub(crate) fn diagnostic_sites(&self) -> &[PublishedLifecycleOperationDiagnosticSiteV1] {
        &self.diagnostic_sites
    }
    /// Exact-numeric range-check obligation issued at one physical row.
    /// The declared source spelling is the single authority; each consumer
    /// lowers it to its own range proof instead of re-deriving meaning.
    pub(crate) fn exact_numeric_check_at(
        &self,
        function: u32,
        block: u32,
        instruction: u32,
    ) -> Option<&str> {
        self.exact_numeric_checks
            .iter()
            .find(|check| {
                check.function == function
                    && check.block == block
                    && check.instruction == instruction
            })
            .map(|check| check.declared_type_name.as_str())
    }

    pub(crate) fn diagnostic_site_at(
        &self,
        function: u32,
        block: u32,
        instruction: u32,
    ) -> Option<PublishedLifecycleOperationDiagnosticSiteV1> {
        self.diagnostic_sites.iter().copied().find(|site| {
            site.function == function && site.block == block && site.instruction == instruction
        })
    }
    /// The same admitted Integer view, projected to its receiving Birth ABI.
    pub(in crate::mir) fn borrowed_birth_actual(&self, site: &OwnedExprSiteV1, ordinal: u32) -> Option<BorrowedBirthActualTransportV1> {
        self.borrowed_birth_actuals.get(&(site.clone(), ordinal)).copied()
    }
    /// Whether this exact (new site, ordinal) actual binds an
    /// object-field formal through the `nullable_kind_payload_v1` lane:
    /// `Null` encodes the (0,0) pair, an owned `Handle` the (3,payload)
    /// move — never a scalar kind.
    pub(crate) fn object_birth_actual(&self, site: &OwnedExprSiteV1, ordinal: u32) -> Option<ObjectBirthActualTransportV1> {
        self.object_birth_actuals.get(&(site.clone(), ordinal)).copied()
    }
    pub(crate) const fn process_result_site(&self) -> u64 {
        self.process_result_site
    }
    pub(crate) const fn fault_abi_version(&self) -> u32 {
        self.fault_abi_version
    }
    pub(crate) const fn runtime_requirements(&self) -> PublishedLifecycleRuntimeRequirementsV1 {
        self.runtime_requirements
    }
    pub(crate) const fn storage_profile(&self) -> Option<u32> {
        match self.runtime_requirements {
            PublishedLifecycleRuntimeRequirementsV1::TypedObject { storage_profile } => {
                Some(storage_profile)
            }
            PublishedLifecycleRuntimeRequirementsV1::NativeArray => None,
        }
    }
}

impl<'module> PublishedMirBackendView<'module> {
    /// Joins selected program coordinates to installed runtime layouts.
    pub(crate) fn issue_lifecycle_physical_abi_input(
        &self,
    ) -> Result<PublishedLifecyclePhysicalAbiInputV1<'module>, String> {
        let entry = self.issue_lifecycle_compiled_entry_contract()?;
        if !entry.program().is_native_array()
            && entry.root_result() != CompiledEntryRootResultV1::I64
        {
            return Err(fault("root-result-unavailable"));
        }
        use crate::mir::normal_callable_semantic_package::{
            BirthFormalDeclarationClassV1 as Declaration, BirthFormalUseCoverageV1 as Uses,
        };
        for birth in entry.births() {
            let function = entry.program().functions().get(birth.function_index() as usize)
                .ok_or_else(|| fault("birth-caller-missing"))?;
            let super::physical_program::PublishedLifecyclePhysicalFunctionRoleV1::BirthUnit { abi } = function.role()
                else { return Err(fault("formal-contract-missing")) };
            let mut text_birth = false;
            for formal in birth.formals() {
                if formal.kind() == CompiledEntryFormalKindV1::Receiver {
                    continue;
                }
                if abi.text_formal_source_v1(formal.source_ordinal().ok_or_else(|| fault("formal-contract-missing"))?)?.is_some() {
                    text_birth = true;
                    continue;
                }
                let contract = formal
                    .contract()
                    .ok_or_else(|| fault("formal-contract-missing"))?;
                if contract.declaration() != Declaration::Unannotated {
                    return Err(fault("formal-declaration-unavailable"));
                }
                let source_ordinal = formal.source_ordinal()
                    .ok_or_else(|| fault("formal-contract-missing"))?;
                let source_qualified_untyped = abi.dynamic_object_formal_source_v1(source_ordinal)?.is_some()
                    || abi.dynamic_i64_formal_source_v1(source_ordinal)?;
                if !matches!(
                    contract.uses(),
                    Uses::NoUse | Uses::I64FieldStores { .. } | Uses::ObjectFieldStores { .. }
                ) && !matches!(contract.uses(), Uses::UnqualifiedFieldStores { .. } if source_qualified_untyped) {
                    return Err(fault("formal-use-unavailable"));
                }
            }
            if text_birth {
                // Recheck even when a cloned module lost its demand marker.
                let callee = self.module().functions.get(function.name()).ok_or_else(|| fault("birth-caller-draft-missing"))?;
                entry.program().handoff().root_source().ok_or_else(|| fault("formal-contract-missing"))?
                    .validate_birth_text_entry_v1(abi, callee)?;
            }
        }
        // Exact call identity, arity and ordering were checked by compiled-entry.
        // Inspect every actual, including unused formals; never specialize a body.
        let (borrowed_birth_actuals, object_birth_actuals) =
            self.issue_borrowed_birth_actuals(&entry)?;
        let diagnostic_sites = issue_diagnostic_sites(entry.program())?;
        let exact_numeric_checks =
            issue_exact_numeric_checks(self.module(), entry.program())?;
        // The process projection is an entry epilogue, not a MIR Invoke.
        let process_result_site = u64::try_from(diagnostic_sites.len())
            .map_err(|_| fault("process-result-site-overflow"))?;
        if entry.program().is_native_array() {
            return Ok(PublishedLifecyclePhysicalAbiInputV1 {
                entry,
                layouts: Box::new([]),
                diagnostic_sites: diagnostic_sites.into_boxed_slice(),
                exact_numeric_checks: exact_numeric_checks.into_boxed_slice(),
                borrowed_birth_actuals,
                object_birth_actuals,
                process_result_site,
                fault_abi_version: 1,
                runtime_requirements: PublishedLifecycleRuntimeRequirementsV1::NativeArray,
            });
        }
        let storage_profile = self
            .lifecycle_storage_profile()
            .ok_or_else(|| fault("storage-profile-missing"))? as u32;
        let mut ids = referenced_objects(entry.program())?;
        // A sealed parameter class/null domain references its layout even when
        // the formal is ignored and the only incoming value is null.
        for (index, function) in entry.program().functions().iter().enumerate() {
            for param in function.params() {
                if let Some(object) = entry.borrowed_object_view(index as u32, param.0) {
                    ids.insert(object);
                }
            }
        }
        let definitions = self
            .module()
            .canonical_object_definitions()
            .ok_or_else(|| fault("object-definitions-missing"))?;
        let mut layouts = Vec::with_capacity(ids.len());
        for object_id in ids {
            let definition = definitions
                .get(object_id as usize)
                .ok_or_else(|| fault("object-definition-missing"))?;
            if !definition.destruction_disposition().has_no_hook() {
                return Err(fault("object-destruction"));
            }
            let layout = definition
                .runtime_layout()
                .ok_or_else(|| fault("layout-not-issued"))?
                .as_ref()
                .map_err(|_| fault("layout-unavailable"))?;
            if layout.field_count as usize != layout.fields.len() || definition.fields().len() != layout.fields.len() {
                return Err(fault("layout-field-count"));
            }
            if definition.fields().iter().enumerate().any(|(ordinal, field)| field.declared_type_name.is_none() && !definition.is_array_field(ordinal)) {
                use crate::mir::normal_callable_semantic_package::OwnedFieldChildKindV1 as Child;
                use crate::mir::instruction::FieldResidenceEndModeV1 as Mode;
                let object = hakorune_mir_defs::CanonicalObjectIdV1::from_declaration_index(object_id as usize)
                    .ok_or_else(|| fault("owned-object-identity"))?;
                let source = entry.program().handoff().root_source()
                    .ok_or_else(|| fault("owned-object-source-missing"))?;
                let inventory = source.owned_field_inventory_v1(object)?
                    .ok_or_else(|| fault("object-destruction"))?;
                let required: Vec<_> = definition.fields().iter().enumerate()
                    .filter(|(ordinal, field)| field.declared_type_name.is_none() && !definition.is_array_field(*ordinal))
                    .map(|(ordinal, _)| hakorune_mir_defs::CanonicalFieldRefV1::from_declaration_ordinal(object, ordinal)
                        .expect("complete canonical declaration census")).collect();
                let retained: Vec<_> = inventory.iter().filter(|row| row.kind == Child::Residence(Mode::DynamicResidence))
                    .map(|row| row.field).collect();
                if retained != required { return Err(fault("object-destruction")); }
            }
            let fields = layout
                .fields
                .iter()
                .enumerate()
                .map(|(ordinal, field)| {
                    // Retain the canonical physical storage. Dynamic is never
                    // reinterpreted as an i64 slot; its current residence has
                    // the explicit source-issued view/replace/End protocol.
                    if field.slot != ordinal as u32
                        || (field.storage == TypedObjectFieldStorage::Dynamic)
                            != (definition.fields()[ordinal].declared_type_name.is_none() && !definition.is_array_field(ordinal))
                        || !matches!(
                            field.storage,
                            TypedObjectFieldStorage::I8
                                | TypedObjectFieldStorage::I16
                                | TypedObjectFieldStorage::I32
                                | TypedObjectFieldStorage::I64
                                | TypedObjectFieldStorage::ISize
                                | TypedObjectFieldStorage::U8
                                | TypedObjectFieldStorage::U16
                                | TypedObjectFieldStorage::U32
                                | TypedObjectFieldStorage::U64
                                | TypedObjectFieldStorage::USize
                                | TypedObjectFieldStorage::Handle
                                | TypedObjectFieldStorage::Dynamic
                        )
                    {
                        return Err(fault("layout-field-drift"));
                    }
                    Ok(PublishedLifecyclePhysicalFieldLayoutV1 {
                        object_id,
                        declaration_ordinal: ordinal as u32,
                        runtime_slot: field.slot,
                        storage_kind: if field.storage == TypedObjectFieldStorage::Dynamic {
                            HAKO_LLVMC_LIFECYCLE_STORAGE_DYNAMIC
                        } else { HAKO_LLVMC_LIFECYCLE_STORAGE_I64 },
                    })
                })
                .collect::<Result<Vec<_>, String>>()?;
            // The owned-residence mark is the declared `ArrayBox` field
            // set of this exact object — the same declaration rows the
            // destruction disposition and the sealed residence claim
            // consume. It is a teardown inventory, not a storage fact:
            // ordinals only, in declaration order.
            let owned_residences = definition
                .fields()
                .iter()
                .enumerate()
                .filter(|(ordinal, _)| definition.is_array_field(*ordinal))
                .map(|(ordinal, _)| ordinal as u32)
                .collect::<Vec<_>>()
                .into_boxed_slice();
            let owned_object_residences = if definition.destruction_disposition().supports_static_object_fields()
                || definition.fields().iter().any(|field| field.declared_type_name.is_none())
            {
                let object = hakorune_mir_defs::CanonicalObjectIdV1::from_declaration_index(
                    object_id as usize,
                )
                .ok_or_else(|| fault("owned-object-identity"))?;
                let source = entry
                    .program()
                    .handoff()
                    .root_source()
                    .ok_or_else(|| fault("owned-object-source-missing"))?;
                source
                    .owned_field_inventory_v1(object)?
                    .into_iter()
                    .flatten()
                    .filter_map(|child| {
                        match child.kind {
                            crate::mir::normal_callable_semantic_package::OwnedFieldChildKindV1::Object(id) =>
                                Some((child.field.declaration_ordinal(), id.declaration_index())),
                            _ => None,
                        }
                    })
                    .collect::<Vec<_>>()
                    .into_boxed_slice()
            } else {
                Box::new([])
            };
            layouts.push(PublishedLifecyclePhysicalObjectLayoutV1 {
                object_id,
                runtime_type_id: layout.type_id,
                field_count: layout.field_count,
                fields: fields.into_boxed_slice(),
                owned_residences,
                owned_object_residences,
            });
        }
        Ok(PublishedLifecyclePhysicalAbiInputV1 {
            entry,
            layouts: layouts.into_boxed_slice(),
            diagnostic_sites: diagnostic_sites.into_boxed_slice(),
            exact_numeric_checks: exact_numeric_checks.into_boxed_slice(),
            borrowed_birth_actuals,
            object_birth_actuals,
            process_result_site,
            fault_abi_version: 1,
            runtime_requirements: PublishedLifecycleRuntimeRequirementsV1::TypedObject {
                storage_profile,
            },
        })
    }


}

fn referenced_objects(
    program: &PublishedLifecyclePhysicalProgramV1<'_>,
) -> Result<BTreeSet<u32>, String> {
    let mut ids = BTreeSet::new();
    for function in program.functions() {
        for block in function.blocks() {
            for row in block
                .instructions()
                .iter()
                .copied()
                .chain(std::iter::once(block.terminator()))
            {
                match row.instruction() {
                    MirInstruction::ObjectFieldGet { field, .. } => {
                        ids.insert(field.object().declaration_index());
                    }
                    MirInstruction::FieldGet { .. } => {
                        let field = row
                            .field_ref()
                            .ok_or_else(|| fault("field-get-route-missing"))?;
                        ids.insert(field.object().declaration_index());
                    }
                    MirInstruction::Invoke { operation, .. } => match operation {
                        InvokeOperation::Map(MapInvokeOperation::InstallIndexed {
                            object, ..
                        })
                        | InvokeOperation::NewBox { object }
                        | InvokeOperation::HomeRelease { object, .. }
                        | InvokeOperation::HomeReleaseIfLive { object, .. }
                        | InvokeOperation::ReclaimUnpublished { object, .. } => {
                            ids.insert(object.declaration_index());
                        }
                        InvokeOperation::Map(MapInvokeOperation::FieldInstallIndexed { field, object, .. } | MapInvokeOperation::FieldDeleteIndexed { field, object, .. }) => {
                            ids.insert(field.object().declaration_index());
                            ids.insert(object.declaration_index());
                        }
                        InvokeOperation::ObjectEndIfOwned { object, end, .. } => {
                            ids.insert(object.declaration_index());
                            for step in end.iter() {
                                match step {
                                    crate::mir::instruction::ObjectEndStepV1::FieldResidence { field, .. } => { ids.insert(field.object().declaration_index()); }
                                    crate::mir::instruction::ObjectEndStepV1::ObjectField { field, child } => {
                                        ids.insert(field.object().declaration_index());
                                        ids.insert(child.declaration_index());
                                    }
                                    crate::mir::instruction::ObjectEndStepV1::ReleaseOuter => {},
                                }
                            }
                        }
                        InvokeOperation::FieldGet { field, .. }
                        | InvokeOperation::FieldSet { field, .. }
                        | InvokeOperation::OwnedFieldResidenceRelease { field, .. } => {
                            ids.insert(field.object().declaration_index());
                        }
                        InvokeOperation::OwnedObjectFieldRelease { field, child, .. }
                        | InvokeOperation::ObjectFieldBorrow { field, child, .. }
                        | InvokeOperation::ObjectFieldSet { field, child, .. } => {
                            ids.insert(field.object().declaration_index());
                            ids.insert(child.declaration_index());
                        }
                        InvokeOperation::OwnedArray(operation) => {
                            ids.insert(operation.descriptor().object().declaration_index());
                        }
                        InvokeOperation::Birth(_)
                        | InvokeOperation::Call { .. }
                        | InvokeOperation::Map(_)
                        | InvokeOperation::String(_)
                        | InvokeOperation::DynamicInvocation(_)
                        | InvokeOperation::DynamicArrayGet { .. }
                        | InvokeOperation::DynamicCarrierInteger { .. }
                        | InvokeOperation::DynamicCarrierSubI64 { .. }
                        | InvokeOperation::DynamicCarrierEnd { .. }
                        | InvokeOperation::DynamicCarrierArray(_) => {}
                        InvokeOperation::IntrinsicArrayNew
                        | InvokeOperation::ScriptArgumentsNew { .. }
                        | InvokeOperation::ArrayStateContractClaim { .. }
                        | InvokeOperation::ArrayElementWrite { .. } => {}
                    },
                    _ => {}
                }
            }
        }
    }
    Ok(ids)
}

fn fault(reason: &str) -> String {
    format!("[freeze:contract][published-lifecycle-physical-abi/{reason}]")
}

/// Explicit physical ABI mapping; source enum discriminants are not wire tags.
pub(super) fn array_element_tag(
    spec: crate::typed_array_contract_spec::ArrayElementContractSpec,
) -> u32 {
    use crate::typed_array_contract_spec::ExactArrayElementType;
    match spec.element {
        ExactArrayElementType::I8 => 1,
        ExactArrayElementType::I16 => 2,
        ExactArrayElementType::I32 => 3,
        ExactArrayElementType::I64 => 4,
        ExactArrayElementType::U8 => 5,
        ExactArrayElementType::U16 => 6,
        ExactArrayElementType::U32 => 7,
    }
}

/// Sole source-kind to physical-tag projection for this bounded input.
/// A non-literal scalar actual — a local/bound read or a proven `i64`
/// field read — spells the integer payload tag only when the caller's own
/// value table pins the emitted ValueId to `MirType::Integer`; `Null` and
/// `Handle` actuals still carry no scalar tag.
pub(super) fn scalar_actual_kind(
    kind: &crate::mir::normal_callable_semantic_package::OrdinaryNewTrivialArgumentKindV1,
    value: crate::mir::ValueId,
    caller: &std::collections::BTreeMap<crate::mir::ValueId, crate::mir::MirType>,
) -> Result<u32, String> {
    use crate::mir::normal_callable_semantic_package::OrdinaryNewTrivialArgumentKindV1 as Kind;
    match kind {
        Kind::Null => Ok(0),
        Kind::Integer(_) => Ok(1),
        Kind::Bool(_) => Ok(2),
        Kind::Local { .. }
        | Kind::BoundValue { .. }
        | Kind::I64Field { .. }
        | Kind::QualifiedStaticCall { .. }
            if caller.get(&value) == Some(&crate::mir::MirType::Integer) =>
        {
            Ok(1)
        }
        Kind::Local { .. }
        | Kind::Handle { .. }
        | Kind::BoundValue { .. }
        | Kind::I64Field { .. }
        | Kind::QualifiedStaticCall { .. }
        | Kind::Expression { .. } => Err(fault("actual-kind-unavailable")),
    }
}

#[cfg(test)]
#[path = "physical_abi_tests.rs"]
mod tests;
