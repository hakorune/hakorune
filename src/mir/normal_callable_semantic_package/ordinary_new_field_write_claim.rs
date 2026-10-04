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
//! the product — additive evidence, never a fallback.

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::resolved_semantics::{
    BindingKindV1, BodyExpressionShapeV1, BodyMeReceiverV1, ResolvedAssignmentTargetV1,
    VerifiedResolvedBodyShapeInventoryV1, VerifiedResolvedFunctionV1,
};
use crate::parser::ParserOrdinaryBoxSourceCoverageV1;

/// Key: (owning box name, field name). Value: the agreed `new` class.
pub(crate) type OrdinaryNewFieldWriteClaimsV1 = BTreeMap<(Box<str>, Box<str>), Box<str>>;

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
pub(crate) type OwnedFieldResidencesV1 =
    BTreeMap<(Box<str>, Box<str>), OwnedFieldResidenceV1>;

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
    writes: BTreeMap<(Box<str>, Box<str>), Vec<ObservedFieldStoreV1>>,
    birth_new_writes: BTreeSet<(Box<str>, Box<str>)>,
    birth_provided_writes: BTreeSet<(Box<str>, Box<str>)>,
    unattributed_fields: BTreeSet<Box<str>>,
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
    /// `in_birth` marks constructor rows: only a birth-side provider
    /// `new` store can seed an owned residence claim.
    pub(crate) fn observe_function(
        &mut self,
        function: &VerifiedResolvedFunctionV1,
        body_shape: Option<&VerifiedResolvedBodyShapeInventoryV1>,
        owner_box: Option<&str>,
        in_birth: bool,
    ) {
        let Some(shape) = body_shape else {
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
            let Some(owner_box) = owner_box else {
                self.unattributed_fields.insert(field.clone());
                continue;
            };
            if !attributed {
                self.unattributed_fields.insert(field.clone());
                continue;
            }
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
            match &store {
                ObservedFieldStoreV1::Construction(class)
                    if in_birth
                        && (class.as_ref() == "ArrayBox"
                            || !crate::box_trait::is_builtin_box(class.as_ref())) =>
                {
                    self.birth_new_writes
                        .insert((owner_box.into(), field.clone()));
                }
                ObservedFieldStoreV1::Provided if in_birth => {
                    self.birth_provided_writes
                        .insert((owner_box.into(), field.clone()));
                }
                _ => {}
            }
            self.writes
                .entry((owner_box.into(), field.clone()))
                .or_default()
                .push(store);
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
        if self.opaque {
            return (BTreeMap::new(), BTreeMap::new());
        }
        let mut residences = BTreeMap::new();
        let claims = self
            .writes
            .into_iter()
            .filter_map(|((owner_box, field), stores)| {
                if self.unattributed_fields.contains(&field) {
                    return None;
                }
                let key = (owner_box.clone(), field.clone());
                match stores.as_slice() {
                    [ObservedFieldStoreV1::Construction(class)]
                        if (class.as_ref() == "ArrayBox"
                            || !crate::box_trait::is_builtin_box(class.as_ref()))
                            && self.birth_new_writes.contains(&key) =>
                    {
                        residences
                            .insert(key, OwnedFieldResidenceV1::Provider(class.clone()));
                    }
                    [ObservedFieldStoreV1::Provided]
                        if self.birth_provided_writes.contains(&key) =>
                    {
                        residences.insert(key, OwnedFieldResidenceV1::Provided);
                    }
                    _ => {}
                }
                let mut unique = BTreeSet::new();
                for store in stores {
                    let ObservedFieldStoreV1::Construction(class) = store else {
                        return None;
                    };
                    unique.insert(class);
                }
                let unique = unique.into_iter().collect::<Vec<_>>();
                let [class] = unique.as_slice() else {
                    return None;
                };
                ordinary_box_coverage
                    .row_for(class)
                    .ok()
                    .flatten()
                    .map(|_| ((owner_box, field), class.clone()))
            })
            .collect();
        (claims, residences)
    }
}
