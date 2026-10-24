//! Original ordered construction structure, distinct from execution eligibility.
//! The same semantic row retains this source while unqualified calls stay closed.
use super::*;
use crate::mir::resolved_semantics::SourceStmtSiteV1;
use crate::mir::function::TypedObjectFieldStorage;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ConstructionSourceStepV1 {
    Store(SourceStmtSiteV1),
    StoredChildCall(ConstructionChildCallSourceV1),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ConstructionChildCallSourceV1 {
    pub(super) site: OwnedExprSiteV1,
    pub(super) receiver_site: SourceExprSiteV1,
    pub(super) receiver_binding: BindingRefV1,
    pub(super) field: CanonicalFieldRefV1,
    pub(super) child: CanonicalObjectIdV1,
    pub(super) class: Box<str>,
    pub(super) arguments: Box<[SourceExprSiteV1]>,
    pub(super) fault_discharge: Box<[OwnedFieldChildV1]>,
}
/// New completion and field readiness are distinct source obligations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ConstructionCompletionV1 {
    InitializedFields,
    /// Exact declaration fields; none has an initial readable value.
    AllocationOnly { fields: Box<[ConstructionFieldSourceV1]> },
}

/// Original declaration identity and storage vocabulary, not execution authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ConstructionFieldSourceV1 {
    pub(super) name: Box<str>,
    pub(super) field: CanonicalFieldRefV1,
    pub(super) storage: TypedObjectFieldStorage,
}

impl ConstructionFieldSourceV1 {
    pub(crate) fn name(&self) -> &str { &self.name }
    pub(crate) fn field(&self) -> CanonicalFieldRefV1 { self.field }
    pub(crate) fn storage(&self) -> TypedObjectFieldStorage { self.storage }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ConstructionSourceStructureV1 {
    pub(super) object: CanonicalObjectIdV1,
    pub(super) completion: ConstructionCompletionV1,
    pub(super) field_demands: Result<Box<[HomeDemandV1]>, ConstructionUnavailableV1>,
    pub(super) stores: Box<[ConstructionStoreV1]>,
    pub(super) constructor: Option<(crate::parser::ConstructorSourceIdV1, FunctionOwnerIdV1)>,
    pub(super) steps: Box<[ConstructionSourceStepV1]>,
}
impl ConstructionSourceStructureV1 {
    pub(crate) fn completion(&self) -> &ConstructionCompletionV1 { &self.completion }

    pub(crate) fn allocation_only_fields(&self) -> Option<&[ConstructionFieldSourceV1]> {
        match self.completion() {
            ConstructionCompletionV1::AllocationOnly { fields } => Some(fields),
            ConstructionCompletionV1::InitializedFields => None,
        }
    }

    pub(crate) fn is_allocation_only_i64(&self) -> bool {
        self.allocation_only_fields().is_some_and(|fields|
            fields.iter().all(|field| field.storage == TypedObjectFieldStorage::I64))
    }

    /// Allocation-only objects may carry declared Dynamic slots as well as
    /// I64 slots. Their execution still requires the separately sealed field
    /// drain inventory before the local Home can be published.
    pub(crate) fn supports_empty_allocation(&self) -> bool {
        self.allocation_only_fields().is_some_and(|fields| {
            !fields.is_empty()
                && fields.iter().all(|field| matches!(
                    field.storage,
                    TypedObjectFieldStorage::I64 | TypedObjectFieldStorage::Dynamic
                ))
        })
    }

    /// Resolve only within this original declaration, never a global name table.
    pub(crate) fn allocation_only_field(&self, name: &str) -> Option<&ConstructionFieldSourceV1> {
        self.allocation_only_fields()?.iter().find(|field| field.name() == name)
    }

    /// Bounded projection for consumers whose execution contract is still I64-only.
    pub(crate) fn allocation_only_i64_field(&self, name: &str) -> Option<CanonicalFieldRefV1> {
        let field = self.allocation_only_field(name)?;
        (field.storage == TypedObjectFieldStorage::I64).then_some(field.field())
    }

    pub(super) fn require_initialized_fields(&self) -> Result<(), ConstructionUnavailableV1> {
        match self.completion() {
            ConstructionCompletionV1::InitializedFields => self.field_demands.as_ref().map(|_| ()).map_err(|reason| *reason),
            ConstructionCompletionV1::AllocationOnly { .. } =>
                Err(ConstructionUnavailableV1::InitializationContractMissing),
        }
    }
    pub(crate) fn child_call_source_v1(self: &Rc<Self>, site: &OwnedExprSiteV1) -> Result<Option<ConstructionChildCallSourceLoanV1>, String> {
        let mut matching = self.steps.iter().enumerate().filter_map(|(index, step)| match step {
            ConstructionSourceStepV1::StoredChildCall(call) if &call.site == site => Some(index),
            _ => None,
        });
        let Some(index) = matching.next() else { return Ok(None); };
        if matching.next().is_some() || self.constructor.as_ref().map(|(_, owner)| *owner) != Some(site.owner()) {
            return Err("[freeze:contract][construction-source/child-call-identity]".into());
        }
        Ok(Some(ConstructionChildCallSourceLoanV1 { source: Rc::clone(self), index }))
    }
    pub(crate) fn steps(&self) -> &[ConstructionSourceStepV1] { &self.steps }
    pub(crate) fn stores(&self) -> &[ConstructionStoreV1] { &self.stores }
    pub(crate) fn object(&self) -> CanonicalObjectIdV1 { self.object }
    pub(crate) fn constructor(&self) -> Option<&(crate::parser::ConstructorSourceIdV1, FunctionOwnerIdV1)> {
        self.constructor.as_ref()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ConstructionChildCallSourceLoanV1 {
    source: Rc<ConstructionSourceStructureV1>,
    index: usize,
}
impl PartialEq for ConstructionChildCallSourceLoanV1 {
    fn eq(&self, other: &Self) -> bool { self.index == other.index && Rc::ptr_eq(&self.source, &other.source) }
}
impl Eq for ConstructionChildCallSourceLoanV1 {}
impl ConstructionChildCallSourceLoanV1 {
    pub(crate) fn source(&self) -> &Rc<ConstructionSourceStructureV1> { &self.source }
    pub(crate) fn call(&self) -> &ConstructionChildCallSourceV1 {
        match &self.source.steps[self.index] {
            ConstructionSourceStepV1::StoredChildCall(call) => call,
            _ => unreachable!("private original child-call loan"),
        }
    }
}
impl ConstructionChildCallSourceV1 {
    pub(crate) fn site(&self) -> &OwnedExprSiteV1 { &self.site }
    pub(crate) fn receiver_site(&self) -> &SourceExprSiteV1 { &self.receiver_site }
    pub(crate) fn receiver_binding(&self) -> BindingRefV1 { self.receiver_binding }
    pub(crate) fn field(&self) -> CanonicalFieldRefV1 { self.field }
    pub(crate) fn child(&self) -> CanonicalObjectIdV1 { self.child }
    pub(crate) fn class(&self) -> &str { &self.class }
    pub(crate) fn arguments(&self) -> &[SourceExprSiteV1] { &self.arguments }
    pub(crate) fn fault_discharge(&self) -> &[OwnedFieldChildV1] { &self.fault_discharge }
}

pub(super) fn capture_child_call_source_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    shape: &crate::mir::resolved_semantics::VerifiedResolvedBodyShapeInventoryV1,
    receiver: BindingRefV1,
    object: CanonicalObjectIdV1,
    fields: &[String],
    declarations: &[crate::mir::function::UserBoxFieldDecl],
    objects: &[(crate::parser::ParserOrdinaryBoxSourceRowV1, CanonicalObjectIdV1)],
    stores: &[ConstructionStoreV1],
    statement: &SourceStmtSiteV1,
) -> Result<ConstructionChildCallSourceV1, ConstructionUnavailableV1> {
    use ConstructionUnavailableV1 as U;
    let site = SourceExprSiteV1::from_node(statement.node().clone());
    let call = input.function().method_call(&site).ok_or(U::SourceRelationMissing)?;
    let (field, _) = super::scalar::me_field_read_row(shape, receiver, call.receiver_site(), object, fields)?;
    let Some(ConstructionStoreRhsV1::ProviderConstruction { object: Some(child), class, .. }) =
        super::scalar::prior_store_rhs(stores, field) else { return Err(U::BodyCoverageUnsupported); };
    if child.object() == object || declarations.get(field.declaration_ordinal() as usize)
        .and_then(|declaration| declaration.declared_type_name.as_deref()) != Some(class.as_ref()) {
        return Err(U::BodyCoverageUnsupported);
    }
    // The original provider already resolved this exact class/object pair.
    // Method selection, inputs, Completion and result remain late obligations.
    Ok(ConstructionChildCallSourceV1 {
        site: OwnedExprSiteV1::new(input.owner(), site),
        receiver_site: call.receiver_site().clone(), receiver_binding: receiver,
        field, child: child.object(), class: class.clone(),
        arguments: call.arguments().iter().map(|argument| argument.site().clone()).collect(),
        fault_discharge: prior_owned_fields_v1(stores),
    })
}

/// Shared source/qualified-plan residence law. Provided arguments remain
/// caller-owned until Birth Normal; only independently acquired residences
/// join a successful store prefix.
fn owned_field_after_store_v1(
    store: &ConstructionStoreV1,
    mode: crate::mir::instruction::FieldSetModeV1,
) -> Option<OwnedFieldChildV1> {
    use crate::mir::instruction::{FieldResidenceEndModeV1 as End, FieldSetModeV1 as Mode};
    let kind = match mode {
        Mode::DynamicI64Initialize | Mode::DynamicMapInitialize | Mode::DynamicTextInitialize
        | Mode::DynamicValue(_) => OwnedFieldChildKindV1::Residence(End::DynamicResidence),
        _ => match store.rhs() {
            ConstructionStoreRhsV1::ProviderConstruction { object: Some(child), .. } => OwnedFieldChildKindV1::Object(child.object()),
            ConstructionStoreRhsV1::ProviderConstruction { object: None, class, .. } if class.as_ref() == "ArrayBox" =>
                OwnedFieldChildKindV1::Residence(End::ArrayResidence),
            _ => return None,
        },
    };
    Some(OwnedFieldChildV1 { field: store.field(), kind })
}

pub(super) fn prior_owned_fields_v1(stores: &[ConstructionStoreV1]) -> Box<[OwnedFieldChildV1]> {
    stores.iter().rev().filter_map(|store| owned_field_after_store_v1(store, store.mode().ok()?)).collect()
}

impl ConstructionPlanV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn qualify_ordered_unit_calls_v1(
        source: &ConstructionSourceEligibilityV1,
        mut qualify: impl FnMut(&Rc<ConstructionSourceStructureV1>, &OwnedExprSiteV1) -> Result<bool, String>,
    ) -> Result<ConstructionEligibilityV1, String> {
        let source = match source { Ok(source) => source, Err(reason) => return Ok(Err(*reason)) };
        if let Err(reason) = source.require_initialized_fields() { return Ok(Err(reason)); }
        let mut saw_call = false;
        for step in source.steps() {
            match step {
                ConstructionSourceStepV1::Store(_) if saw_call => {
                    // Existing literal folds do not prove reads across opaque
                    // effects. Keep this source form unavailable until its owner does.
                    return Ok(Err(ConstructionUnavailableV1::BodyCoverageUnsupported));
                }
                ConstructionSourceStepV1::StoredChildCall(call) => {
                    saw_call = true;
                    if !qualify(source, call.site())? { return Ok(Err(ConstructionUnavailableV1::BodyCoverageUnsupported)); }
                }
                _ => {}
            }
        }
        Ok(Self::seal_original_execution_v1(source))
    }

    pub(in crate::mir::normal_callable_semantic_package) fn qualify_source_dynamic_stores_v1(
        source: &ConstructionSourceEligibilityV1,
        declarations: &[crate::mir::function::UserBoxFieldDecl],
        objects: &[(crate::parser::ParserOrdinaryBoxSourceRowV1, CanonicalObjectIdV1)],
        definitions: &[crate::mir::function::CanonicalObjectDefinitionV1],
        mut qualify_dynamic: impl FnMut(&ConstructionStoreV1) -> Option<crate::mir::instruction::FieldSetModeV1>,
        mut qualify: impl FnMut(&ConstructionStoreV1) -> Option<Rc<super::super::SourceValueSumV1>>,
    ) -> ConstructionEligibilityV1 {
        use ConstructionUnavailableV1 as U;
        let source = source.as_ref().map_err(|reason| *reason)?;
        if !matches!(source.completion(), ConstructionCompletionV1::InitializedFields)
            || source.steps().iter().any(|step| matches!(step, ConstructionSourceStepV1::StoredChildCall(_))) {
            return Err(U::BodyCoverageUnsupported);
        }
        let mut qualified = Vec::with_capacity(source.stores.len());
        for store in source.stores.iter() {
            let field = declarations.get(store.field().declaration_ordinal() as usize).ok_or(U::SourceRelationMissing)?;
            if store.field().object() != source.object() { return Err(U::SourceRelationMissing); }
            let next = match store.mode() {
                Ok(mode) => (mode, None),
                Err(U::FieldContractUnsupported) if field.declared_type_name.is_none() && !field.is_weak
                    && matches!(store.rhs(), ConstructionStoreRhsV1::Parameter { provided: None, .. }) => {
                    let source_dynamic = qualify_dynamic(store);
                    let source_dynamic_available = source_dynamic.is_some();
                    let dynamic = source_dynamic.filter(|mode| match mode {
                        crate::mir::instruction::FieldSetModeV1::DynamicI64Initialize => true,
                        crate::mir::instruction::FieldSetModeV1::DynamicValue(contract) => {
                            use crate::mir::instruction::{DynamicFieldResidenceV1 as Residence,
                                DynamicFieldValueTagV1 as Tag, DynamicFieldWriteIntentV1 as Intent};
                            contract.intent() == Intent::Initialize
                                && matches!((contract.value_tag(), contract.residence()),
                                    (Tag::Object | Tag::NullableObject, Residence::ObjectHome(_)))
                        }
                        _ => false,
                    });
                    if let Some(mode) = dynamic {
                        (mode, None)
                    } else {
                        let sum = qualify(store);
                        let sum = sum.filter(|sum| sum.text_only()).ok_or(U::FieldContractUnsupported)?;
                        (crate::mir::instruction::FieldSetModeV1::DynamicTextInitialize, Some(sum))
                    }
                }
                Err(reason) => return Err(reason),
            };
            qualified.push(next);
        }
        let modes = qualified.iter().map(|(mode, _)| Ok(*mode)).collect::<Vec<_>>();
        let demands = qualified_field_demands_v1(declarations, &source.stores, &modes, objects, definitions)?;
        Self::seal_execution_v1(source, demands, qualified)
    }

    pub(in crate::mir::normal_callable_semantic_package) fn without_child_calls(source: &ConstructionSourceEligibilityV1) -> ConstructionEligibilityV1 {
        let source = source.as_ref().map_err(|reason| *reason)?;
        source.require_initialized_fields()?;
        if source.steps.iter().any(|step| matches!(step, ConstructionSourceStepV1::StoredChildCall(_))) {
            return Err(ConstructionUnavailableV1::BodyCoverageUnsupported);
        }
        Self::seal_original_execution_v1(source)
    }

    /// Move existing source qualification into the sole execution owner. The
    /// prefix is sealed in original store order, never BTreeMap/field order.
    fn seal_original_execution_v1(source: &Rc<ConstructionSourceStructureV1>) -> ConstructionEligibilityV1 {
        source.require_initialized_fields()?;
        Self::seal_execution_v1(source, source.field_demands.as_ref().map_err(|reason| *reason)?.clone(),
            source.stores.iter().map(|store| store.mode().map(|mode| (mode, None))).collect::<Result<Vec<_>, _>>()?)
    }

    fn seal_execution_v1(
        source: &Rc<ConstructionSourceStructureV1>,
        field_demands: Box<[HomeDemandV1]>,
        qualified: Vec<(crate::mir::instruction::FieldSetModeV1, Option<Rc<super::super::SourceValueSumV1>>)>,
    ) -> ConstructionEligibilityV1 {
        if qualified.len() != source.stores.len() { return Err(ConstructionUnavailableV1::SourceRelationMissing); }
        let mut prefix = Vec::new();
        let mut stores = Vec::with_capacity(source.stores.len());
        for (index, (store, (mode, text_input))) in source.stores.iter().zip(qualified).enumerate() {
            if !match (mode, text_input.as_ref()) {
                (crate::mir::instruction::FieldSetModeV1::DynamicTextInitialize, Some(sum)) => sum.text_only(),
                (crate::mir::instruction::FieldSetModeV1::DynamicTextInitialize, None) => false,
                (_, loan) => loan.is_none(),
            } {
                return Err(ConstructionUnavailableV1::SourceRelationMissing);
            }
            let fault_discharge: Box<[_]> = prefix.iter().rev().cloned().collect();
            // Retained source cutpoints and the shared residence law must
            // agree before any physical consumer receives this projection.
            if store.fault_discharge() != prior_owned_fields_v1(&source.stores[..index]).as_ref() {
                return Err(ConstructionUnavailableV1::SourceRelationMissing);
            }
            stores.push(ConstructionStoreExecutionV1 { mode, fault_discharge, text_input });
            if let Some(field) = owned_field_after_store_v1(store, mode) { prefix.push(field); }
        }
        Ok(Self {
            source: Rc::clone(source),
            execution: Rc::new(ConstructionExecutionV1 {
                field_demands,
                stores: stores.into_boxed_slice(),
            }),
        })
    }
}

/// Full declaration census shared by initial and late-qualified execution.
pub(super) fn qualified_field_demands_v1(
    field_decls: &[crate::mir::function::UserBoxFieldDecl],
    stores: &[ConstructionStoreV1],
    modes: &[Result<crate::mir::instruction::FieldSetModeV1, ConstructionUnavailableV1>],
    objects: &[(crate::parser::ParserOrdinaryBoxSourceRowV1, CanonicalObjectIdV1)],
    definitions: &[crate::mir::function::CanonicalObjectDefinitionV1],
) -> Result<Box<[HomeDemandV1]>, ConstructionUnavailableV1> {
    use ConstructionUnavailableV1 as U;
    let mut demands = vec![HomeDemandV1::Trivial; field_decls.len()];
    for (ordinal, field) in field_decls.iter().enumerate() {
        let (index, store) = stores
            .iter().enumerate()
            .find(|(_, store)| store.field().declaration_ordinal() as usize == ordinal)
            .ok_or(U::InitializationContractMissing)?;
        let mode = modes.get(index).ok_or(U::SourceRelationMissing)?;
        if let Err(reason) = mode {
            return Err(*reason);
        }
        use crate::mir::instruction::{DynamicFieldResidenceV1 as Residence,
            DynamicFieldValueTagV1 as Tag, DynamicFieldWriteIntentV1 as Intent,
            FieldSetModeV1 as Mode};
        let text = modes[index] == Ok(Mode::DynamicTextInitialize);
        let dynamic_i64 = modes[index] == Ok(Mode::DynamicI64Initialize)
            && field.declared_type_name.is_none() && !field.is_weak
            && matches!(store.rhs(), ConstructionStoreRhsV1::Parameter { provided: None, .. });
        let dynamic_object = matches!(modes[index], Ok(Mode::DynamicValue(contract))
            if contract.intent() == Intent::Initialize
                && matches!((contract.value_tag(), contract.residence()),
                    (Tag::Object | Tag::NullableObject, Residence::ObjectHome(child))
                        if objects.iter().any(|(_, object)| object == &child)
                            && definitions.get(child.declaration_index() as usize)
                                // Dynamic field installation carries the child's exact End
                                // descriptor in its residence transaction. The child may
                                // itself own Dynamic fields, provided its source End is
                                // hook-free and those fields have their own complete drain.
                                .is_some_and(|definition| definition.destruction_disposition().has_no_hook())))
            && field.declared_type_name.is_none() && !field.is_weak
            && matches!(store.rhs(), ConstructionStoreRhsV1::Parameter { provided: None, .. });
        let supported = if text || dynamic_i64 || dynamic_object { field.declared_type_name.is_none() && !field.is_weak
            && matches!(store.rhs(), ConstructionStoreRhsV1::Parameter { provided: None, .. })
        } else { match field.declared_type_name.as_deref() {
            Some("i64") | Some("usize") => matches!(
                store.rhs(),
                ConstructionStoreRhsV1::LiteralI64(_) | ConstructionStoreRhsV1::Parameter { .. }
            ),
            Some(name) => match store.rhs() {
                ConstructionStoreRhsV1::ProviderConstruction {
                    class, object, ..
                } => {
                    class.as_ref() == name
                        && object.is_some()
                            == (crate::runtime::CoreBoxId::from_name(name).is_none())
                }
                // Caller-provided object store: birth formals are
                // unannotated, so the declared field class is the sole
                // class authority and it must resolve to a releasable
                // user class — self-reference included: the emitted
                // teardown call graph recurses on the instance, not the
                // type. The disposition bound matches the release op's
                // child admission exactly.
                ConstructionStoreRhsV1::Parameter { .. } => {
                    if crate::runtime::CoreBoxId::from_name(name).is_some() {
                        false
                    } else {
                        let mut resolved = objects
                            .iter()
                            .filter_map(|(own, id)| (own.name() == name).then_some(*id));
                        match (resolved.next(), resolved.next()) {
                            (Some(child), None) => definitions
                                .get(child.declaration_index() as usize)
                                .map(|definition| definition.destruction_disposition())
                                .is_some_and(|disposition| {
                                    (disposition).supports_static_no_hook()
                                }),
                            _ => false,
                        }
                    }
                }
                _ => false,
            },
            None => matches!(
                store.rhs(),
                ConstructionStoreRhsV1::ProviderConstruction { .. } | ConstructionStoreRhsV1::LiteralI64(_)
            ),
        } };
        if !supported {
            return Err(U::FieldContractUnsupported);
        }
        let object_field = !dynamic_i64 && !matches!(
            field.declared_type_name.as_deref(),
            Some("i64") | Some("usize")
        );
        if object_field && !matches!(store.rhs(), ConstructionStoreRhsV1::LiteralI64(_)) {
            demands[ordinal] = HomeDemandV1::Handle;
        }
    }
    Ok(demands.into_boxed_slice())
}
