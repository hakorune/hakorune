//! Callable result-class claims for the bounded ordinary-`New` cohort.
//!
//! Source authority: the resolver's sealed `BodyStatementShapeV1::Return`
//! rows and `constructions()` — the same passive facts the birth-site
//! index and field-write claims already walk. Canonical issuer: this
//! module's draft, driven from `issue_ordinary_source_cohort_v1` where
//! the selected-key map is already in scope.
//!
//! A callable claims a result class only when its sealed body-shape
//! inventory exists, every `return` row carries a value that is `new`
//! of one agreed class, and the body's last statement is itself a
//! `return` with a value — so a normal exit can never fall past the
//! observed returns into an unproven unit result. Nested `if`/`loop`
//! returns are part of the same inventory, so early exits participate
//! in the uniform check. Missing inventories, value-less returns,
//! non-`new` return values, and mixed classes all leave the callable
//! unclaimed — additive evidence, never a fallback.

use std::collections::BTreeMap;

use crate::mir::resolved_semantics::{
    BodyStatementShapeV1, VerifiedResolvedBodyShapeInventoryV1, VerifiedResolvedFunctionV1,
};
use crate::parser::ParserOrdinaryBoxSourceCoverageV1;
use hakorune_mir_defs::CanonicalSameModuleCallableKeyV1;

/// Key: the selected canonical callable key. Value: the agreed result
/// class every `return` constructs.
pub(crate) type OrdinaryNewResultClassClaimsV1 =
    BTreeMap<CanonicalSameModuleCallableKeyV1, Box<str>>;

#[derive(Default)]
pub(crate) struct OrdinaryNewResultClassClaimDraftV1 {
    claims: BTreeMap<CanonicalSameModuleCallableKeyV1, Box<str>>,
}

impl OrdinaryNewResultClassClaimDraftV1 {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Observe one selected callable. Only `Some(key)` functions can
    /// carry a claim; a missing body-shape inventory or any non-uniform
    /// evidence simply leaves the key out of the product.
    pub(crate) fn observe_function(
        &mut self,
        function: &VerifiedResolvedFunctionV1,
        body_shape: Option<&VerifiedResolvedBodyShapeInventoryV1>,
        key: &CanonicalSameModuleCallableKeyV1,
    ) {
        let Some(shape) = body_shape else {
            return;
        };
        let statements = shape.statements();
        // The last statement must be a `return` with a value: otherwise a
        // path can fall off the end of the body without constructing the
        // claimed class. Site ordering places a nested statement inside
        // its parent's subtree, so `last` is the final top-level row.
        if !matches!(
            statements.last(),
            Some(BodyStatementShapeV1::Return {
                value: Some(_),
                ..
            })
        ) {
            return;
        }
        let mut class: Option<Box<str>> = None;
        for statement in statements {
            let BodyStatementShapeV1::Return { value, .. } = statement else {
                continue;
            };
            let Some(site) = value else {
                return;
            };
            let Some(construction) = function.expression_source().construction(site) else {
                return;
            };
            match &class {
                None => class = Some(construction.class().into()),
                Some(existing) if existing.as_ref() == construction.class() => {}
                Some(_) => return,
            }
        }
        let Some(class) = class else {
            return;
        };
        self.claims.insert(key.clone(), class);
    }

    /// Seal the observed rows, keeping only claims whose class is an
    /// ordinary box of this package.
    pub(crate) fn finish(
        self,
        ordinary_box_coverage: &ParserOrdinaryBoxSourceCoverageV1,
    ) -> OrdinaryNewResultClassClaimsV1 {
        self.claims
            .into_iter()
            .filter(|(_, class)| {
                ordinary_box_coverage
                    .row_for(class)
                    .ok()
                    .flatten()
                    .is_some()
            })
            .collect()
    }
}
