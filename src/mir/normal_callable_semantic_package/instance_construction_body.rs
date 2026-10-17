//! Original ordered construction structure, distinct from execution eligibility.
//! The same semantic row retains this source while unqualified calls stay closed.
use super::*;
use crate::mir::resolved_semantics::SourceStmtSiteV1;

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
    AllocationOnlyI64 { fields: Box<[CanonicalFieldRefV1]> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ConstructionSourceStructureV1 {
    pub(super) object: CanonicalObjectIdV1,
    pub(super) completion: ConstructionCompletionV1,
    pub(super) field_demands: Box<[HomeDemandV1]>,
    pub(super) stores: Box<[ConstructionStoreV1]>,
    pub(super) constructor: Option<(crate::parser::ConstructorSourceIdV1, FunctionOwnerIdV1)>,
    pub(super) steps: Box<[ConstructionSourceStepV1]>,
}
impl ConstructionSourceStructureV1 {
    pub(crate) fn completion(&self) -> &ConstructionCompletionV1 { &self.completion }

    pub(super) fn require_initialized_fields(&self) -> Result<(), ConstructionUnavailableV1> {
        match self.completion() {
            ConstructionCompletionV1::InitializedFields => Ok(()),
            ConstructionCompletionV1::AllocationOnlyI64 { .. } =>
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
    declarations: &[crate::ast::FieldDecl],
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

pub(super) fn prior_owned_fields_v1(
    stores: &[ConstructionStoreV1],
) -> Box<[OwnedFieldChildV1]> {
    stores
            .iter()
            .rev()
            .filter_map(|prior: &ConstructionStoreV1| {
                let kind = match prior.rhs() {
                    ConstructionStoreRhsV1::ProviderConstruction {
                        object: Some(child),
                        ..
                    } => OwnedFieldChildKindV1::Object(child.object()),
                    ConstructionStoreRhsV1::ProviderConstruction {
                        object: None,
                        class,
                        ..
                    } if class.as_ref() == "ArrayBox" => OwnedFieldChildKindV1::Array,
                    // Provided arguments remain caller-owned until Birth Normal.
                    // Fault reclaims only unpublished parent storage.
                    ConstructionStoreRhsV1::Parameter { .. } => return None,
                    _ => return None,
                };
                Some(OwnedFieldChildV1 {
                    field: prior.field(),
                    kind,
                })
            })
            .collect::<Vec<_>>()
            .into_boxed_slice()
}
