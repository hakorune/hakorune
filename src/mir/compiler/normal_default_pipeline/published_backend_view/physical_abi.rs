//! Final-view physical inputs required by the dedicated lifecycle C consumer.
//!
//! This is a physical join of already-issued products.  It neither allocates a
//! layout nor interprets source meaning.

use std::collections::BTreeSet;

use super::compiled_entry_contract::CompiledEntryFormalKindV1;
use crate::mir::function::{ObjectDestructionDispositionV1, TypedObjectFieldStorage};
use crate::mir::instruction::{InvokeOperation, MapInvokeOperation};
use crate::mir::resolved_semantics::OwnedExprSiteV1;
use crate::mir::MirInstruction;

use super::{
    physical_program::PublishedLifecyclePhysicalProgramV1, CompiledEntryContractV1,
    CompiledEntryRootResultV1, PublishedMirBackendView,
};

/// Physical-program.v2 field storage tag; mirrored by the C ABI header.
const HAKO_LLVMC_LIFECYCLE_STORAGE_I64: u32 = 1;

/// Runtime diagnostic operation kinds admitted by the selected lifecycle ABI.
/// These are physical runtime calls, never source-level diagnostic sites.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PublishedLifecycleCheckedOperationKindV1 {
    NewBox,
    MapNew,
    MapPrepareKey,
    MapInstall,
    MapCheckedGet,
    MapArrayLength,
    MapArrayIndex,
    MapGetText,
    MapEndOutcome,
    MapEnd,
    ArrayNew,
    ArrayClaim,
    ArrayWrite,
    FieldSet,
    ObjectFieldSet,
    HomeRelease,
    HomeReleaseIfLive,
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
            InvokeOperation::Map(operation) => Some(match operation {
                MapInvokeOperation::New => Self::MapNew,
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
            InvokeOperation::FieldSet { .. } => Some(Self::FieldSet),
            InvokeOperation::ObjectFieldSet { .. } => Some(Self::ObjectFieldSet),
            InvokeOperation::HomeRelease { .. } => Some(Self::HomeRelease),
            InvokeOperation::HomeReleaseIfLive { .. } => Some(Self::HomeReleaseIfLive),
            InvokeOperation::OwnedFieldResidenceRelease { .. } => {
                Some(Self::FieldResidenceRelease)
            }
            InvokeOperation::OwnedObjectFieldRelease { .. } => Some(Self::ObjectFieldRelease),
            InvokeOperation::ReclaimUnpublished { .. } => Some(Self::ReclaimUnpublished),
            InvokeOperation::IntrinsicArrayNew => Some(Self::ArrayNew),
            InvokeOperation::ArrayStateContractClaim { .. } => Some(Self::ArrayClaim),
            InvokeOperation::ArrayElementWrite { .. } => Some(Self::ArrayWrite),
            InvokeOperation::Call { .. } => None,
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
    /// row spells `"tagged"` so the callee sees the proven kind==1 payload.
    tagged_birth_actuals: BTreeSet<(OwnedExprSiteV1, u32)>,
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
    /// Whether this exact (new site, ordinal) actual is an admitted
    /// dominated `new`-argument view: the birth transport spells it
    /// `"tagged"` so the callee sees the proven kind==1 payload.
    pub(crate) fn tagged_birth_actual(&self, site: &OwnedExprSiteV1, ordinal: u32) -> bool {
        self.tagged_birth_actuals
            .contains(&(site.clone(), ordinal))
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
        for formal in entry.births().iter().flat_map(|birth| birth.formals()) {
            if formal.kind() == CompiledEntryFormalKindV1::Receiver {
                continue;
            }
            let contract = formal
                .contract()
                .ok_or_else(|| fault("formal-contract-missing"))?;
            if contract.declaration() != Declaration::Unannotated {
                return Err(fault("formal-declaration-unavailable"));
            }
            if !matches!(contract.uses(), Uses::NoUse | Uses::I64FieldStores { .. }) {
                return Err(fault("formal-use-unavailable"));
            }
        }
        // Exact call identity, arity and ordering were checked by compiled-entry.
        // Inspect every actual, including unused formals; never specialize a body.
        let tagged_birth_actuals = self.issue_tagged_birth_actuals(&entry)?;
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
                tagged_birth_actuals,
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
            if !matches!(
                definition.destruction_disposition(),
                ObjectDestructionDispositionV1::PlainI64NoHook
                    | ObjectDestructionDispositionV1::OwnedArrayFieldsNoHook
                    | ObjectDestructionDispositionV1::OwnedObjectFieldsNoHook
            ) {
                return Err(fault("object-destruction"));
            }
            let layout = definition
                .runtime_layout()
                .ok_or_else(|| fault("layout-not-issued"))?
                .as_ref()
                .map_err(|_| fault("layout-unavailable"))?;
            if layout.field_count as usize != layout.fields.len() {
                return Err(fault("layout-field-count"));
            }
            let fields = layout
                .fields
                .iter()
                .enumerate()
                .map(|(ordinal, field)| {
                    // Every plan storage kind projects onto the same i64
                    // wire lane — `Handle` fields carry a residence handle
                    // in the integer lane. Any future non-lane storage is
                    // rejected here rather than silently tagged I64.
                    if field.slot != ordinal as u32
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
                        )
                    {
                        return Err(fault("layout-field-drift"));
                    }
                    Ok(PublishedLifecyclePhysicalFieldLayoutV1 {
                        object_id,
                        declaration_ordinal: ordinal as u32,
                        runtime_slot: field.slot,
                        storage_kind: HAKO_LLVMC_LIFECYCLE_STORAGE_I64,
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
                .filter(|(_, field)| {
                    field.declared_type_name.as_deref() == Some("ArrayBox")
                })
                .map(|(ordinal, _)| ordinal as u32)
                .collect::<Vec<_>>()
                .into_boxed_slice();
            let owned_object_residences = if definition.destruction_disposition()
                == ObjectDestructionDispositionV1::OwnedObjectFieldsNoHook
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
            tagged_birth_actuals,
            process_result_site,
            fault_abi_version: 1,
            runtime_requirements: PublishedLifecycleRuntimeRequirementsV1::TypedObject {
                storage_profile,
            },
        })
    }

    /// The finalized draft admits a dominated `new`-argument lane per
    /// exact (site, ordinal): that emitted `Handle{binding}` actual is the
    /// same lent Integer view and is transported as `"tagged"`. Every
    /// other actual must still prove its scalar lane here — `null` and
    /// non-admitted handle actuals fail closed.
    fn issue_tagged_birth_actuals(
        &self,
        entry: &CompiledEntryContractV1<'_>,
    ) -> Result<BTreeSet<(OwnedExprSiteV1, u32)>, String> {
        use crate::mir::normal_callable_semantic_package::OrdinaryNewTrivialArgumentKindV1 as Kind;
        let mut tagged = BTreeSet::new();
        for call in entry.birth_calls() {
            let caller = entry
                .program()
                .functions()
                .get(call.caller_function_index() as usize)
                .ok_or_else(|| fault("birth-caller-missing"))?;
            let uses = entry
                .program()
                .handoff()
                .root_source()
                .map(|source| {
                    // Callers outside the closed borrowed-entry profile
                    // cannot carry admitted `new`-argument uses; asking for
                    // their projection would demand entry values that were
                    // never recorded.
                    if !source.has_borrowed_ordinary_entry_v1(call.actual().owner()) {
                        return Ok(Vec::new().into_boxed_slice());
                    }
                    let caller_function = self
                        .module()
                        .functions
                        .get(caller.name())
                        .ok_or_else(|| fault("birth-caller-draft-missing"))?;
                    source.borrowed_ordinary_new_argument_uses_v1(
                        call.actual().owner(),
                        caller_function,
                    )
                })
                .transpose()?
                .unwrap_or_default();
            for actual in call.actual().arguments() {
                let admitted = uses.iter().any(|(_, formal, site, ordinal)| {
                    *ordinal == actual.source().ordinal()
                        && site == actual.source().new_site()
                        && matches!(
                            actual.source().kind(),
                            Kind::Handle { binding } if binding == formal
                        )
                });
                if admitted {
                    if !tagged.insert((
                        actual.source().new_site().clone(),
                        actual.source().ordinal(),
                    )) {
                        return Err(fault("birth-actual-tagged-duplicate"));
                    }
                    continue;
                }
                scalar_actual_kind(actual.source().kind(), actual.value(), caller.value_types())?;
            }
        }
        Ok(tagged)
    }
}

/// Binds every issued dynamic-integer-range contract to its physical row.
/// Only bare routed `FieldSet` rows enter the transport set; invoke-form
/// stores are covered by the separate lifecycle enforce arm instead.
fn issue_exact_numeric_checks(
    module: &crate::mir::MirModule,
    program: &PublishedLifecyclePhysicalProgramV1<'_>,
) -> Result<Vec<PublishedLifecycleExactNumericCheckV1>, String> {
    let mut checks = Vec::new();
    let mut coordinates = BTreeSet::new();
    for (name, function) in &module.functions {
        for contract in &function.metadata.exact_numeric_runtime_check_contracts {
            if contract.kind
                != crate::mir::function::ExactNumericRuntimeCheckContractKind::DynamicIntegerRange
            {
                continue;
            }
            let (ordinal, physical) = program
                .functions()
                .iter()
                .enumerate()
                .find(|(_, p)| p.name() == name.as_str())
                .ok_or_else(|| fault("check-uncovered-function"))?;
            let block = physical
                .blocks()
                .iter()
                .find(|b| b.id() == contract.block)
                .ok_or_else(|| fault("check-uncovered-block"))?;
            let row = block
                .instructions()
                .iter()
                .copied()
                .chain(std::iter::once(block.terminator()))
                .find(|r| r.index() as usize == contract.instruction_index)
                .ok_or_else(|| fault("check-uncovered-instruction"))?;
            let MirInstruction::FieldSet { .. } = row.instruction() else {
                continue;
            };
            let function_ordinal =
                u32::try_from(ordinal).map_err(|_| fault("check-function-overflow"))?;
            if !coordinates.insert((function_ordinal, contract.block.as_u32(), row.index())) {
                return Err(fault("check-coordinate-duplicate"));
            }
            checks.push(PublishedLifecycleExactNumericCheckV1 {
                function: function_ordinal,
                block: contract.block.as_u32(),
                instruction: row.index(),
                declared_type_name: contract.declared_type_name.clone(),
            });
        }
    }
    Ok(checks)
}

fn issue_diagnostic_sites(
    program: &PublishedLifecyclePhysicalProgramV1<'_>,
) -> Result<Vec<PublishedLifecycleOperationDiagnosticSiteV1>, String> {
    let mut sites = Vec::new();
    let mut coordinates = BTreeSet::new();
    for (function_ordinal, physical_function) in program.functions().iter().enumerate() {
        let function =
            u32::try_from(function_ordinal).map_err(|_| fault("site-function-overflow"))?;
        for block in physical_function.blocks() {
            for row in block
                .instructions()
                .iter()
                .copied()
                .chain(std::iter::once(block.terminator()))
            {
                let Some(kind) =
                    PublishedLifecycleCheckedOperationKindV1::from_instruction(row.instruction())
                else {
                    continue;
                };
                let coordinate = (function, block.id().0, row.index());
                if !coordinates.insert(coordinate) {
                    return Err(fault("site-coordinate-duplicate"));
                }
                let site = u64::try_from(sites.len()).map_err(|_| fault("site-overflow"))?;
                sites.push(PublishedLifecycleOperationDiagnosticSiteV1 {
                    function,
                    block: block.id().0,
                    instruction: row.index(),
                    kind,
                    site,
                });
            }
        }
    }
    // Ordinary Calls may require this ABI without any checked operation.
    // The root epilogue still owns process-result site zero in that case.
    Ok(sites)
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
                        InvokeOperation::FieldSet { field, .. }
                        | InvokeOperation::OwnedFieldResidenceRelease { field, .. } => {
                            ids.insert(field.object().declaration_index());
                        }
                        InvokeOperation::OwnedObjectFieldRelease { field, child, .. }
                        | InvokeOperation::ObjectFieldSet { field, child, .. } => {
                            ids.insert(field.object().declaration_index());
                            ids.insert(child.declaration_index());
                        }
                        InvokeOperation::Call { .. } | InvokeOperation::Map(_) => {}
                        InvokeOperation::IntrinsicArrayNew
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
        Kind::Null
        | Kind::Local { .. }
        | Kind::Handle { .. }
        | Kind::BoundValue { .. }
        | Kind::I64Field { .. }
        | Kind::QualifiedStaticCall { .. } => Err(fault("actual-kind-unavailable")),
    }
}

#[cfg(test)]
#[path = "physical_abi_tests.rs"]
mod tests;
