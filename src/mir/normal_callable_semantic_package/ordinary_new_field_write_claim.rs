//! Field-write claims for the bounded ordinary-`New` cohort.
//!
//! Source authority: the resolver's sealed `assignment_sources` rows,
//! `FieldAccess`/`Me` body shapes, `FieldWrite` assignment targets, and
//! `constructions()` — the same passive facts the birth-site index walks.
//! Canonical issuer: this module's draft, driven from
//! `issue_ordinary_source_cohort_v1` where `instance_constructors` and the
//! selected-key map are already in scope.
//!
//! A field carries a class claim only when every `FieldWrite` in the
//! package resolves to an attributed `me.` write inside a function of the
//! owning box AND every such write stores `new C()` of one agreed ordinary
//! box. A `FieldWrite` whose target object we cannot attribute to a box
//! (non-`me` receiver, unknown owner) vetoes the field name globally:
//! soundness here means the field name is never written outside the
//! observed `me.` writers. A function whose sealed body-shape inventory
//! is missing hides its writes entirely, so one missing inventory empties
//! the object-class projection. The same product retains original writer
//! requirements and their vetoes — additive evidence, never a fallback.

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_value_profile::expression_children::ExpressionChildIssueV1;
use crate::mir::resolved_value_profile::expression_plan::{
    assignment_expression_source_plan_v1, ExpressionSourcePlanV1,
};

use crate::mir::resolved_semantics::{
    BindingKindV1, BodyExpressionShapeV1, BodyMeReceiverV1, FunctionOwnerIdV1,
    ResolvedAssignmentSourceV1, ResolvedAssignmentTargetV1, SourceExprSiteV1,
};
use crate::parser::{ConstructorSourceIdV1, ParserOrdinaryBoxSourceCoverageV1};

#[path = "ordinary_new_field_value_source.rs"]
mod value_source;
pub(crate) use value_source::{DeclaredFieldValueSourceV1, FieldValueSourceIssueV1};

type FieldValueSourcesV1 = BTreeMap<
    (Box<str>, Box<str>), Result<Rc<DeclaredFieldValueSourceV1>, FieldValueSourceIssueV1>,
>;

/// The original all-writer class proof, distinct from numeric value/whole-Birth
/// eligibility. Every consumer shares this row; declaration alone is not class.
#[derive(Debug)]
pub(crate) struct ObjectFieldClassSourceV1 {
    inventory: Rc<OriginalFieldWriterInventoryV1>,
    class: Box<str>,
    declaration: Option<(hakorune_mir_defs::CanonicalFieldRefV1, Option<String>)>,
}
impl ObjectFieldClassSourceV1 {
    pub(crate) fn class(&self) -> &str { &self.class }
    pub(crate) fn field(&self) -> Option<hakorune_mir_defs::CanonicalFieldRefV1> {
        if self.inventory.opaque || self.inventory.value_incomplete { return None; }
        let (field, declared) = self.declaration.as_ref()?;
        (declared.as_deref() == Some(self.class())).then_some(*field)
    }
}
pub(crate) type ObjectFieldClassSourcesV1 =
    Rc<BTreeMap<(Box<str>, Box<str>), Rc<ObjectFieldClassSourceV1>>>;

/// The same sealed writer inventory owns the object-class projection and
/// pending original RHS needs. Retaining a plan grants no numeric value class.
#[derive(Debug, Default)]
pub(crate) struct OrdinaryNewFieldWriteClaimsV1 {
    object_classes: ObjectFieldClassSourcesV1,
    writers: Rc<OriginalFieldWriterInventoryV1>,
    value_sources: FieldValueSourcesV1,
}

/// One immutable original inventory shared by all of its derived field loans.
#[derive(Debug, Default)]
struct OriginalFieldWriterInventoryV1 {
    writes: Box<[ObservedFieldWriteV1]>,
    unattributed_fields: BTreeSet<Box<str>>,
    opaque: bool,
    value_incomplete: bool,
}

/// Loan of one original writer, without attributing its receiver or field class.
#[derive(Debug, Clone)]
pub(crate) struct OriginalFieldWriteSourceLoanV1 {
    inventory: Rc<OriginalFieldWriterInventoryV1>,
    index: usize,
}
impl PartialEq for OriginalFieldWriteSourceLoanV1 {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.inventory, &other.inventory) && self.index == other.index
    }
}
impl Eq for OriginalFieldWriteSourceLoanV1 {}
/// Exact selected LOCAL New and declaration field, paired with one original writer.
/// No RHS value, Normal readiness or physical write permission is issued here.
#[derive(Debug, Clone)]
pub(crate) struct OwnedLocalFieldWriteSourceV1 {
    acquisition: crate::mir::resolved_semantics::OwnedExprSiteV1,
    root: crate::mir::resolved_semantics::BindingRefV1,
    construction: Rc<super::super::ConstructionSourceStructureV1>,
    field: hakorune_mir_defs::CanonicalFieldRefV1,
    original: OriginalFieldWriteSourceLoanV1,
}
impl PartialEq for OwnedLocalFieldWriteSourceV1 {
    fn eq(&self, other: &Self) -> bool {
        self.acquisition == other.acquisition && self.root == other.root && self.field == other.field
            && Rc::ptr_eq(&self.construction, &other.construction) && self.original == other.original
    }
}
impl Eq for OwnedLocalFieldWriteSourceV1 {}
impl OwnedLocalFieldWriteSourceV1 {
    pub(crate) fn acquisition(&self) -> &crate::mir::resolved_semantics::OwnedExprSiteV1 { &self.acquisition }
    pub(crate) fn root(&self) -> crate::mir::resolved_semantics::BindingRefV1 { self.root }
    pub(crate) fn construction(&self) -> &Rc<super::super::ConstructionSourceStructureV1> { &self.construction }
    pub(crate) fn field(&self) -> hakorune_mir_defs::CanonicalFieldRefV1 { self.field }
    pub(crate) fn original(&self) -> &OriginalFieldWriteSourceLoanV1 { &self.original }
    pub(crate) fn source(&self) -> &Rc<ExpressionSourcePlanV1> {
        self.original.write().source().expect("sealed original writer source")
    }
}
impl OriginalFieldWriteSourceLoanV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn owned_local_v1(
        &self, input: ResolvedFunctionLoweringInputV1<'_>,
        acquisition: &crate::mir::resolved_semantics::OwnedExprSiteV1,
        root: crate::mir::resolved_semantics::BindingRefV1,
        construction: Rc<super::super::ConstructionSourceStructureV1>,
    ) -> Option<OwnedLocalFieldWriteSourceV1> {
        let write = self.write();
        let source = write.source().ok()?;
        let shape = input.body_shape()?;
        let assignment = write.assignment();
        let ResolvedAssignmentTargetV1::FieldWrite { receiver } = input.function().assignment_target(assignment.target_site())? else { return None };
        let BodyExpressionShapeV1::FieldAccess { object, field, .. } = shape.expression_shape(assignment.target_site())? else { return None };
        let field_ref = construction.allocation_only_i64_field(field)?;
        if input.owner() != write.owner() || source.owner() != input.owner()
            || acquisition.owner() != input.owner() || root.owner() != input.owner()
            || write.in_birth() || object != receiver || receiver != write.receiver_site()
            || field.as_ref() != write.field()
            || assignment.form() != crate::mir::resolved_semantics::ResolvedAssignmentFormV1::Plain
            || !shape.assignment_sources().iter().any(|row| row == assignment)
            || !matches!(source.root(), crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::Assignment { assignment: own } if own == assignment)
            || input.function().variable_ref(receiver) != Some(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(root))
            || !matches!(input.function().binding(root).map(|row| row.kind()), Some(BindingKindV1::Local { .. }))
            || !input.function().expression_source().initializers().any(|row|
                row.binding() == root && row.initializer_site() == Some(acquisition.site()))
        { return None; }
        Some(OwnedLocalFieldWriteSourceV1 { acquisition: acquisition.clone(), root,
            construction, field: field_ref, original: self.clone() })
    }

    pub(crate) fn write(&self) -> &ObservedFieldWriteV1 {
        &self.inventory.writes[self.index]
    }

    /// A closed original writer census, not an actual child-class or Home proof.
    pub(crate) fn corroborates_sole_provided_birth_v1(
        &self,
        owner_box: &str,
        constructor: &ConstructorSourceIdV1,
        owner: FunctionOwnerIdV1,
        assignment: &ResolvedAssignmentSourceV1,
    ) -> bool {
        let write = self.write();
        if self.inventory.opaque || self.inventory.value_incomplete
            || self.inventory.unattributed_fields.contains(write.field())
            || !write.is_attributed() || write.owner_box() != Some(owner_box)
            || write.owner() != owner || write.assignment() != assignment
            || !write.birth_source().is_some_and(|source| source.same_as(constructor))
            || !matches!(write.object_store, ObservedFieldStoreV1::Provided)
            || write.source().is_err() {
            return false;
        }
        let mut writers = self.inventory.writes.iter().enumerate().filter(|(_, row)|
            row.owner_box() == Some(owner_box) && row.field() == write.field());
        writers.next().is_some_and(|(index, _)| index == self.index) && writers.next().is_none()
    }
}
impl OrdinaryNewFieldWriteClaimsV1 {
    pub(crate) fn original_write_source_v1(
        &self,
        owner: FunctionOwnerIdV1,
        target: &SourceExprSiteV1,
    ) -> Result<Option<OriginalFieldWriteSourceLoanV1>, &'static str> {
        let mut rows = self.writers.writes.iter().enumerate().filter(|(_, row)|
            row.owner() == owner && row.assignment().target_site() == target);
        let Some((index, row)) = rows.next() else { return Ok(None); };
        if rows.next().is_some() { return Err("field-write/source-duplicate"); }
        row.source().map_err(|_| "field-write/source-plan")?;
        Ok(Some(OriginalFieldWriteSourceLoanV1 { inventory: self.writers.clone(), index }))
    }

    pub(crate) fn recheck_original_write_source_v1(
        &self, loan: &OriginalFieldWriteSourceLoanV1,
    ) -> Result<(), &'static str> {
        if !Rc::ptr_eq(&self.writers, &loan.inventory)
            || loan.index >= self.writers.writes.len()
        { return Err("field-write/source-inventory"); }
        loan.write().source().map_err(|_| "field-write/source-plan")?;
        Ok(())
    }
    pub(crate) fn get(&self, key: &(Box<str>, Box<str>)) -> Option<&Box<str>> {
        self.object_classes.get(key).map(|source| &source.class)
    }
    pub(crate) fn object_sources(&self) -> &ObjectFieldClassSourcesV1 { &self.object_classes }
    pub(crate) fn writes(&self) -> &[ObservedFieldWriteV1] {
        &self.writers.writes
    }
    pub(crate) fn value_source(&self, class: &str, field: &str) -> Option<&Result<Rc<DeclaredFieldValueSourceV1>, FieldValueSourceIssueV1>> {
        self.value_sources.get(&(class.into(), field.into()))
    }
    pub(crate) fn value_sources(&self) -> impl Iterator<Item = (&str, &str, &Result<Rc<DeclaredFieldValueSourceV1>, FieldValueSourceIssueV1>)> {
        self.value_sources.iter().map(|((class, name), source)| (class.as_ref(), name.as_ref(), source))
    }
    /// Missing function inventory and unknown receivers remain explicit vetoes
    /// for every later typed consumer, including when no object claim exists.
    pub(crate) fn has_hidden_writers(&self) -> bool {
        self.writers.opaque
    }
    pub(crate) fn has_unattributed_field(&self, field: &str) -> bool {
        self.writers.unattributed_fields.contains(field)
    }
}

#[derive(Debug)]
pub(crate) struct ObservedFieldWriteV1 {
    owner: FunctionOwnerIdV1,
    owner_box: Option<Box<str>>,
    field: Box<str>,
    receiver: Option<crate::mir::resolved_semantics::BindingRefV1>,
    receiver_site: SourceExprSiteV1,
    birth_source: Option<ConstructorSourceIdV1>,
    assignment: ResolvedAssignmentSourceV1,
    source: Result<Rc<ExpressionSourcePlanV1>, ExpressionChildIssueV1>,
    object_store: ObservedFieldStoreV1,
}
impl ObservedFieldWriteV1 {
    pub(crate) fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }
    pub(crate) fn owner_box(&self) -> Option<&str> {
        self.owner_box.as_deref()
    }
    pub(crate) fn field(&self) -> &str {
        &self.field
    }
    pub(crate) fn is_attributed(&self) -> bool {
        self.receiver.is_some()
    }
    pub(crate) fn receiver(&self) -> Option<crate::mir::resolved_semantics::BindingRefV1> { self.receiver }
    pub(crate) fn receiver_site(&self) -> &SourceExprSiteV1 { &self.receiver_site }
    pub(crate) fn in_birth(&self) -> bool {
        self.birth_source.is_some()
    }
    pub(crate) fn birth_source(&self) -> Option<&ConstructorSourceIdV1> {
        self.birth_source.as_ref()
    }
    pub(crate) fn assignment(&self) -> &ResolvedAssignmentSourceV1 {
        &self.assignment
    }
    pub(crate) fn source(&self) -> Result<&Rc<ExpressionSourcePlanV1>, ExpressionChildIssueV1> {
        self.source.as_ref().map_err(|issue| *issue)
    }
}

/// The sole observed birth-side write proves an owned field residence.
/// `Provider` stores `new <class>()` at the store site; `Provided`
/// stores a `Parameter` binding — the declared field type is the sole
/// class authority for the transferred object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum OwnedFieldResidenceV1 {
    Provider(Box<str>),
    Provided,
}

/// Key: (owning box name, field name). The field's sole observed write is
/// a birth-side `me.<field> = <residence value>` store; re-assignment,
/// non-birth writers and unattributed writers all veto the residence.
pub(crate) type OwnedFieldResidencesV1 = BTreeMap<(Box<str>, Box<str>), OwnedFieldResidenceV1>;

/// One write observation: `Construction(class)` for a `new C(...)` value
/// site, `Provided` for a `Parameter` binding store, `Other` for any
/// other stored value (vetoes the field).
#[derive(Debug)]
enum ObservedFieldStoreV1 {
    Construction(Box<str>),
    Provided,
    Other,
}

#[derive(Default)]
pub(crate) struct OrdinaryNewFieldWriteClaimDraftV1 {
    writes: Vec<ObservedFieldWriteV1>,
    opaque: bool,
    required_writers: BTreeSet<(FunctionOwnerIdV1, SourceExprSiteV1, SourceExprSiteV1)>,
}

impl OrdinaryNewFieldWriteClaimDraftV1 {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Observe every field-write assignment row in one resolved function.
    /// `owner_box` is the box this function belongs to when its selected
    /// key is `InstanceBoxMethod` (or the constructor row's `box_name`);
    /// a `me.` write with no attributable owner vetoes the field name.
    /// `birth_source` retains the original constructor identity: only a birth-side provider
    /// `new` store can seed an owned residence claim.
    pub(crate) fn observe_function(
        &mut self,
        input: ResolvedFunctionLoweringInputV1<'_>,
        owner_box: Option<&str>,
        birth_source: Option<&ConstructorSourceIdV1>,
    ) {
        // Resolver targets are the inverse completeness authority. Include all
        // semantic owners: an unobserved nested owner cannot disappear merely
        // because the enclosing root has a complete body shape.
        for (owner, product) in input.forest().semantic_owners() {
            for (target, assignment) in product.assignment_targets() {
                if let ResolvedAssignmentTargetV1::FieldWrite { receiver } = assignment {
                    self.required_writers.insert((owner, target.clone(), receiver.clone()));
                }
            }
        }
        let function = input.function();
        let Some(shape) = input.body_shape() else {
            // Writes we cannot observe could hold any value; no field
            // claim may stand on a package with hidden writers.
            self.opaque = true;
            return;
        };
        for assignment in shape.assignment_sources() {
            let Some(BodyExpressionShapeV1::FieldAccess { object, field, .. }) =
                shape.expression_shape(assignment.target_site())
            else {
                continue;
            };
            if !matches!(
                function.assignment_target(assignment.target_site()),
                Some(ResolvedAssignmentTargetV1::FieldWrite { .. })
            ) {
                continue;
            }
            let receiver = match shape.expression_shape(object) {
                Some(BodyExpressionShapeV1::Me {
                    receiver: BodyMeReceiverV1::Lexical(receiver), ..
                }) if function.binding(*receiver)
                    .is_some_and(|record| record.kind() == BindingKindV1::Receiver) => Some(*receiver),
                _ => None,
            };
            let store = match function
                .expression_source()
                .construction(assignment.value_site())
            {
                Some(construction) => {
                    ObservedFieldStoreV1::Construction(construction.class().into())
                }
                None => {
                    let provided = matches!(
                        shape.expression_shape(assignment.value_site()),
                        Some(BodyExpressionShapeV1::Variable {
                            resolved: crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding),
                            ..
                        }) if matches!(
                            function.binding(*binding).map(|record| record.kind()),
                            Some(BindingKindV1::Parameter { .. })
                        )
                    );
                    if provided {
                        ObservedFieldStoreV1::Provided
                    } else {
                        ObservedFieldStoreV1::Other
                    }
                }
            };
            // Retain a failed plan as an observed writer, never as absence.
            // Foreign receivers likewise survive in this one inventory and
            // veto every same-named field in the object and future typed views.
            let source = input
                .forest()
                .callable_source_ledger(input.owner())
                .map_err(|_| ExpressionChildIssueV1::MissingSource)
                .and_then(|ledger| {
                    assignment_expression_source_plan_v1(&ledger, shape, assignment)
                });
            self.writes.push(ObservedFieldWriteV1 {
                owner: input.owner(),
                owner_box: owner_box.map(Into::into),
                field: field.clone(),
                receiver,
                receiver_site: object.clone(),
                birth_source: birth_source.cloned(),
                assignment: assignment.clone(),
                source,
                object_store: store,
            });
        }
    }

    /// Seal the observed writes. A field claims a class only when it was
    /// written at least once, every observed write stored `new` of the
    /// same ordinary box, and the field name never appeared on an
    /// unattributed receiver anywhere in the package. An owned field
    /// residence is stronger still: exactly one write, a birth-side
    /// provider `new` or `Parameter` store, nothing else. A `Provided`
    /// residence carries no class — children sealing resolves the
    /// declared field type, and it never mints a stored-`new` claim.
    pub(crate) fn finish(
        self,
        ordinary_box_coverage: &ParserOrdinaryBoxSourceCoverageV1,
        constructors: &super::VerifiedInstanceConstructorSemanticBatchV1,
    ) -> (OrdinaryNewFieldWriteClaimsV1, OwnedFieldResidencesV1) {
        let mut observed = BTreeMap::new();
        for write in &self.writes {
            *observed.entry((write.owner(), write.assignment().target_site().clone(),
                write.receiver_site().clone())).or_insert(0usize) += 1;
        }
        let value_incomplete = observed.len() != self.required_writers.len()
            || self.required_writers.iter().any(|key| observed.get(key) != Some(&1));
        let unattributed_fields: BTreeSet<Box<str>> = self
            .writes
            .iter()
            .filter(|write| !write.is_attributed() || write.owner_box.is_none())
            .map(|write| write.field.clone())
            .collect();
        // Group references to the original observations, not a second scan of
        // source or an independent numeric classifier.
        let mut fields: BTreeMap<(Box<str>, Box<str>), Vec<&ObservedFieldWriteV1>> =
            BTreeMap::new();
        for write in &self.writes {
            if let Some(owner_box) = &write.owner_box {
                fields
                    .entry((owner_box.clone(), write.field.clone()))
                    .or_default()
                    .push(write);
            }
        }
        let mut residences = BTreeMap::new();
        let mut object_classes = BTreeMap::new();
        for ((owner_box, field), stores) in fields {
            if self.opaque || unattributed_fields.contains(&field) {
                continue;
            }
            let key = (owner_box.clone(), field.clone());
            if let [write] = stores.as_slice() {
                if write.in_birth() {
                    match &write.object_store {
                        ObservedFieldStoreV1::Construction(class)
                            if class.as_ref() == "ArrayBox"
                                || !crate::box_trait::is_builtin_box(class.as_ref()) =>
                        {
                            residences.insert(key, OwnedFieldResidenceV1::Provider(class.clone()));
                        }
                        ObservedFieldStoreV1::Provided => {
                            residences.insert(key, OwnedFieldResidenceV1::Provided);
                        }
                        _ => {}
                    }
                }
            }
            let mut unique = BTreeSet::new();
            let mut complete = true;
            for write in stores {
                if let ObservedFieldStoreV1::Construction(class) = &write.object_store {
                    unique.insert(class);
                } else {
                    complete = false;
                }
            }
            if !complete || unique.len() != 1 {
                continue;
            }
            let class = *unique.first().unwrap();
            if ordinary_box_coverage
                .row_for(class)
                .ok()
                .flatten()
                .is_some()
            {
                object_classes.insert((owner_box, field), class.clone());
            }
        }
        let writers = Rc::new(OriginalFieldWriterInventoryV1 {
            writes: self.writes.into_boxed_slice(), unattributed_fields, opaque: self.opaque, value_incomplete,
        });
        let object_classes = Rc::new(object_classes.into_iter().map(|(key, class)| {
            let declaration = ordinary_box_coverage.row_for(&key.0).ok().flatten()
                .and_then(|source| writers.writes.iter()
                    .find(|write| write.owner_box() == Some(key.0.as_ref()) && write.field() == key.1.as_ref())
                    .and_then(|write| super::source_declared_field(constructors, source,
                        &crate::mir::resolved_semantics::OwnedExprSiteV1::new(write.owner(), write.assignment().target_site().clone()),
                        &key.1).ok().flatten()));
            (key, Rc::new(ObjectFieldClassSourceV1 { inventory: Rc::clone(&writers), class, declaration }))
        }).collect());
        let mut claims = OrdinaryNewFieldWriteClaimsV1 {
            object_classes, writers, value_sources: BTreeMap::new(),
        };
        claims.value_sources = value_source::issue_value_sources_v1(
            &claims.writers, ordinary_box_coverage, constructors,
        );
        (claims, residences)
    }
}
