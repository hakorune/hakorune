//! One package-owned answer to a callee's result kind.
//!
//! Source solvers contribute evidence; route-specific input and ownership
//! checks grant execution separately. Missing evidence is never a negative kind.
use super::qualified_static_call_claim::QualifiedStaticCallClaimIndexV1;
use super::ordinary_new_coseal::lexical_instance_call::{BorrowedResultCohortV1, PreparedBorrowedFormalIngressV1};
use super::ordinary_new_coseal::result_class_claim::{OrdinaryNewResultClassClaimsV1, OrdinaryNewResultClassV1};
use super::selected_mapping::VerifiedSelectedCallableBatchMapV1;
use crate::mir::builder::{CanonicalSameModuleCallableKeyV1 as Key, SelectedNormalCallableKeyV1};
use crate::mir::callable_result_representation::VerifiedCallableResultDispositionV1 as StaticEvidence;
use crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1;
use crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1 as Class;
use std::{cell::{OnceCell, RefCell}, collections::BTreeMap, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum CalleeResultKindV1 {
    I64,
    Bool,
    Text,
    NullableText,
    Handle(Box<str>),
    Nullable(Box<str>),
    Map,
    Unit,
}
impl CalleeResultKindV1 {
    pub(super) fn local_class(&self) -> Class {
        match self {
            Self::I64 => Class::I64,
            Self::Bool => Class::Bool,
            Self::Text => Class::Text,
            Self::NullableText => Class::Dynamic,
            Self::Handle(_) => Class::Handle,
            Self::Nullable(_) => Class::Nullable,
            Self::Map => Class::Map,
            Self::Unit => Class::Unit,
        }
    }
    fn from_static(row: &StaticEvidence) -> Option<Self> {
        match row {
            StaticEvidence::ExactI64 { .. } => Some(Self::I64),
            StaticEvidence::ExactBool { .. } => Some(Self::Bool),
            StaticEvidence::ExactString => Some(Self::Text),
            StaticEvidence::ExactNominalBox { box_name, .. } => Some(Self::Handle(box_name.clone().into())),
            StaticEvidence::Unavailable(_) => None,
        }
    }
    fn from_source_value_class(class: &crate::mir::callable_result_representation::CallableSourceValueClassV1) -> Self {
        use crate::mir::callable_result_representation::CallableSourceValueClassV1 as Source;
        match class {
            Source::NominalBox(class) => Self::Handle(class.clone().into()),
            Source::Map => Self::Map,
        }
    }
    fn from_object(row: &OrdinaryNewResultClassV1) -> Option<Self> {
        match row {
            OrdinaryNewResultClassV1::Object(class) => Some(Self::Handle(class.clone())),
            OrdinaryNewResultClassV1::NullableObject(class) => Some(Self::Nullable(class.clone())),
            OrdinaryNewResultClassV1::NullableForwarded { .. } => None,
        }
    }
}

/// Input proof availability is independent of the callee's result class.
/// In particular, Unavailable is not a negative object-class fact.
#[derive(Debug, Clone, Copy)]
pub(super) enum StaticInputRequirementsV1<'a> {
    Verified(&'a [u32]),
    Unavailable,
    NoStaticEvidence,
}
impl<'a> StaticInputRequirementsV1<'a> {
    pub(super) fn verified(self) -> Option<&'a [u32]> {
        match self {
            Self::Verified(requirements) => Some(requirements),
            Self::Unavailable | Self::NoStaticEvidence => None,
        }
    }
}

#[derive(Debug)]
struct ResultRowV1 {
    kind: RefCell<Option<CalleeResultKindV1>>,
    // The SAME source projection, including a formal identity without a kind.
    object: Option<OrdinaryNewResultClassV1>,
    normal_i64: bool,
}
impl ResultRowV1 {
    fn corroborate(&self, key: &Key, evidence: Option<CalleeResultKindV1>) -> Result<(), String> {
        let Some(evidence) = evidence else { return Ok(()); };
        let mut current = self.kind.borrow_mut();
        match current.as_ref() {
            Some(kind) if kind != &evidence => Err(format!(
                "[freeze:contract][callee-result-kind/conflict]: callee={key:?} first={kind:?} next={evidence:?}")),
            Some(_) => Ok(()),
            None => { *current = Some(evidence); Ok(()) }
        }
    }
}

#[derive(Debug)]
pub(super) struct CalleeResultKindCatalogV1 {
    // Own the original rows rather than maintaining a second static index.
    static_rows: BTreeMap<Key, StaticEvidence>,
    static_value_classes: BTreeMap<Key, crate::mir::callable_result_representation::CallableSourceValueClassV1>,
    resolved: OnceCell<BTreeMap<Key, ResultRowV1>>,
}
impl CalleeResultKindCatalogV1 {
    pub(super) fn new(static_rows: BTreeMap<Key, StaticEvidence>, static_value_classes: BTreeMap<Key, crate::mir::callable_result_representation::CallableSourceValueClassV1>) -> Self {
        Self { static_rows, static_value_classes, resolved: OnceCell::new() }
    }
    pub(super) fn static_evidence(&self, key: &Key) -> Option<&StaticEvidence> {
        self.static_rows.get(key)
    }
    pub(super) fn static_input_requirements(&self, key: &Key) -> StaticInputRequirementsV1<'_> {
        match self.static_evidence(key) {
            Some(StaticEvidence::ExactI64 { required_i64_arguments }
                | StaticEvidence::ExactBool { required_i64_arguments }
                | StaticEvidence::ExactNominalBox { required_i64_arguments, .. }) =>
                StaticInputRequirementsV1::Verified(required_i64_arguments),
            Some(StaticEvidence::ExactString) => StaticInputRequirementsV1::Verified(&[]),
            Some(StaticEvidence::Unavailable(_)) => StaticInputRequirementsV1::Unavailable,
            None => StaticInputRequirementsV1::NoStaticEvidence,
        }
    }
    pub(super) fn kind(&self, key: &Key) -> Option<CalleeResultKindV1> {
        match self.resolved.get() {
            Some(rows) => rows.get(key)?.kind.borrow().clone(),
            None => self.static_evidence(key).and_then(CalleeResultKindV1::from_static)
                .or_else(|| self.static_value_classes.get(key).map(CalleeResultKindV1::from_source_value_class)),
        }
    }
    pub(super) fn object(&self, key: &Key) -> Option<&OrdinaryNewResultClassV1> {
        self.resolved.get()?.get(key)?.object.as_ref()
    }
    pub(super) fn normal_i64(&self, key: &Key) -> bool {
        self.resolved.get().and_then(|rows| rows.get(key)).is_some_and(|row| row.normal_i64)
    }
    pub(super) fn claim_row(&self, key: &Key) -> Option<(&[u32], Class)> {
        // Existing claim membership is an independent input/ABI restriction.
        self.static_evidence(key)?.admits_static_claim().then_some(())?;
        let requirements = self.static_input_requirements(key).verified()?;
        Some((requirements, self.kind(key)?.local_class()))
    }
    /// Source-kind evidence may become available after Completion. The
    /// independently verified transfer stays an admission prerequisite.
    pub(super) fn corroborated_invoke_result<'a>(
        &self, key: &Key, text: bool, nullable_text: bool, source_bool: bool, retained_bool: bool,
        relations: impl IntoIterator<Item = &'a crate::mir::resolved_semantics::home_new_prefix::TerminalRelationV1>,
    ) -> Result<Option<crate::mir::instruction::InvokeCallResultKind>, String> {
        let row = self.resolved.get().and_then(|rows| rows.get(key))
            .ok_or_else(|| format!("[freeze:contract][callee-result-kind/missing]: callee={key:?}"))?;
        if text { row.corroborate(key, Some(CalleeResultKindV1::Text))?; }
        if nullable_text { row.corroborate(key, Some(CalleeResultKindV1::NullableText))?; }
        if source_bool { row.corroborate(key, Some(CalleeResultKindV1::Bool))?; }
        let independently_covered = if text || nullable_text { true }
            else if source_bool { retained_bool }
            else {
                let checked = super::direct_call_loan::lifecycle::uniform_call_result_kind(relations);
                checked.is_some() && self.kind(key).and_then(|kind| kind.local_class().invoke_result()) == checked
            };
        Ok(independently_covered.then(|| self.kind(key)?.local_class().invoke_result()).flatten())
    }
    pub(super) fn bind(
        self: &Rc<Self>, batch: &VerifiedResolvedCallableSemanticBatchV1,
        selected: &VerifiedSelectedCallableBatchMapV1, facts: &OrdinaryNewResultClassClaimsV1,
        scalar: &BorrowedResultCohortV1, ingress: &Result<PreparedBorrowedFormalIngressV1, String>,
    ) -> Result<(), String> {
        let mut rows: BTreeMap<_, _> = self.static_rows.iter().map(|(key, proof)| (key.clone(), ResultRowV1 {
            kind: RefCell::new(CalleeResultKindV1::from_static(proof)), object: None, normal_i64: false,
        })).collect();
        for (key, class) in &self.static_value_classes {
            let row = rows.entry(key.clone()).or_insert(ResultRowV1 { kind: RefCell::new(None), object: None, normal_i64: false });
            row.corroborate(key, Some(CalleeResultKindV1::from_source_value_class(class)))?;
        }
        for (key, object) in facts.iter() {
            let row = rows.entry(key.clone()).or_insert(ResultRowV1 { kind: RefCell::new(None), object: None, normal_i64: false });
            row.corroborate(key, CalleeResultKindV1::from_object(object))?;
            row.object = Some(object.clone());
        }
        for declaration in batch.declarations() {
            let Some(SelectedNormalCallableKeyV1::Cataloged(key)) = selected.key_for_batch_slot(declaration.batch_slot()) else { continue; };
            let row = rows.entry(key.clone()).or_insert(ResultRowV1 { kind: RefCell::new(None), object: None, normal_i64: false });
            row.normal_i64 = scalar.is_normal_i64_result_v1(&declaration.owner());
            row.corroborate(key, scalar.source_result_class_v1(&declaration.owner()).and_then(|class| match class {
                Class::I64 => Some(CalleeResultKindV1::I64),
                Class::Bool => Some(CalleeResultKindV1::Bool),
                _ => None,
            }))?;
            if let Ok(Some(original)) = scalar.original_return_source_v1(&declaration.owner()) {
                if ingress.as_ref().is_ok_and(|ingress| original.is_text_return_v1(ingress)) {
                    row.corroborate(key, Some(CalleeResultKindV1::Text))?;
                }
                if original.is_unit_candidate_v1() { row.corroborate(key, Some(CalleeResultKindV1::Unit))?; }
            }
            if super::direct_call_loan::lifecycle::map_result_owner(batch, declaration.owner()) {
                row.corroborate(key, Some(CalleeResultKindV1::Map))?;
            }
        }
        self.resolved.set(rows).map_err(|_| "[freeze:contract][callee-result-kind/already-bound]".to_owned())?;
        facts.bind_result_catalog_v1(Rc::clone(self))
    }
}

impl QualifiedStaticCallClaimIndexV1 {
    pub(super) fn bind_result_kinds_v1(
        &self, batch: &VerifiedResolvedCallableSemanticBatchV1, selected: &VerifiedSelectedCallableBatchMapV1,
        facts: &OrdinaryNewResultClassClaimsV1, scalar: &BorrowedResultCohortV1,
        ingress: &Result<PreparedBorrowedFormalIngressV1, String>,
    ) -> Result<(), String> {
        self.result_catalog().bind(batch, selected, facts, scalar, ingress)
    }
}
