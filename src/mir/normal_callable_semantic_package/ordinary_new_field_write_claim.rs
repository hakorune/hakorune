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

/// One write observation: `Some(class)` for a `new C(...)` value site,
/// `None` for any other stored value (vetoes the field).
#[derive(Default)]
pub(crate) struct OrdinaryNewFieldWriteClaimDraftV1 {
    writes: BTreeMap<(Box<str>, Box<str>), Vec<Option<Box<str>>>>,
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
    pub(crate) fn observe_function(
        &mut self,
        function: &VerifiedResolvedFunctionV1,
        body_shape: Option<&VerifiedResolvedBodyShapeInventoryV1>,
        owner_box: Option<&str>,
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
            let class = function
                .expression_source()
                .construction(assignment.value_site())
                .map(|construction| construction.class().into());
            self.writes
                .entry((owner_box.into(), field.clone()))
                .or_default()
                .push(class);
        }
    }

    /// Seal the observed writes. A field claims a class only when it was
    /// written at least once, every observed write stored `new` of the
    /// same ordinary box, and the field name never appeared on an
    /// unattributed receiver anywhere in the package.
    pub(crate) fn finish(
        self,
        ordinary_box_coverage: &ParserOrdinaryBoxSourceCoverageV1,
    ) -> OrdinaryNewFieldWriteClaimsV1 {
        if self.opaque {
            return BTreeMap::new();
        }
        self.writes
            .into_iter()
            .filter_map(|((owner_box, field), classes)| {
                if self.unattributed_fields.contains(&field) {
                    return None;
                }
                let mut unique = BTreeSet::new();
                for class in classes {
                    unique.insert(class?);
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
            .collect()
    }
}
