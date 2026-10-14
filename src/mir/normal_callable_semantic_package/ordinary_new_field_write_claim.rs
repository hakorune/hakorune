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
    ResolvedAssignmentSourceV1, ResolvedAssignmentTargetV1,
};
use crate::parser::{ConstructorSourceIdV1, ParserOrdinaryBoxSourceCoverageV1};

/// The same sealed writer inventory owns the object-class projection and
/// pending original RHS needs. Retaining a plan grants no numeric value class.
#[derive(Debug, Default)]
pub(crate) struct OrdinaryNewFieldWriteClaimsV1 {
    object_classes: BTreeMap<(Box<str>, Box<str>), Box<str>>,
    writes: Box<[ObservedFieldWriteV1]>,
    unattributed_fields: BTreeSet<Box<str>>,
    opaque: bool,
}
impl OrdinaryNewFieldWriteClaimsV1 {
    pub(crate) fn get(&self, key: &(Box<str>, Box<str>)) -> Option<&Box<str>> {
        self.object_classes.get(key)
    }
    pub(crate) fn writes(&self) -> &[ObservedFieldWriteV1] {
        &self.writes
    }
    /// Missing function inventory and unknown receivers remain explicit vetoes
    /// for every later typed consumer, including when no object claim exists.
    pub(crate) fn has_hidden_writers(&self) -> bool {
        self.opaque
    }
    pub(crate) fn has_unattributed_field(&self, field: &str) -> bool {
        self.unattributed_fields.contains(field)
    }
}

#[derive(Debug)]
pub(crate) struct ObservedFieldWriteV1 {
    owner: FunctionOwnerIdV1,
    owner_box: Option<Box<str>>,
    field: Box<str>,
    attributed: bool,
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
        self.attributed
    }
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
            let attributed = matches!(
                shape.expression_shape(object),
                Some(BodyExpressionShapeV1::Me {
                    receiver: BodyMeReceiverV1::Lexical(receiver),
                    ..
                }) if function
                    .binding(*receiver)
                    .is_some_and(|record| record.kind() == BindingKindV1::Receiver)
            );
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
                attributed,
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
    ) -> (OrdinaryNewFieldWriteClaimsV1, OwnedFieldResidencesV1) {
        let unattributed_fields: BTreeSet<Box<str>> = self
            .writes
            .iter()
            .filter(|write| !write.attributed || write.owner_box.is_none())
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
        (
            OrdinaryNewFieldWriteClaimsV1 {
                object_classes,
                writes: self.writes.into_boxed_slice(),
                unattributed_fields,
                opaque: self.opaque,
            },
            residences,
        )
    }
}
