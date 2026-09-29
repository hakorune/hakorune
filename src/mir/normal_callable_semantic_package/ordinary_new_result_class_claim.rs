//! Callable result-class claims for the bounded ordinary-`New` cohort.
//!
//! Source authority: the resolver's sealed `BodyStatementShapeV1::Return`
//! rows and `constructions()`/`literal()` — the same passive facts the
//! birth-site index and field-write claims already walk. Canonical issuer:
//! this module's draft, driven from `issue_ordinary_source_cohort_v1`
//! where the selected-key map is already in scope.
//!
//! A callable claims a definite object result only when its sealed
//! body-shape inventory exists, every `return` row carries a value that
//! is `new` of one agreed class, and the body's last statement is itself
//! a `return` with a value — so a normal exit can never fall past the
//! observed returns into an unproven unit result. When every exit is
//! either `new` of the agreed class or the exact `null` literal, the
//! claim degrades to `NullableObject`: the class is proven, but the
//! caller may observe `null`, so Handle/lifecycle consumers must not
//! treat it as an owned object. Nested `if`/`loop` returns are part of
//! the same inventory, so early exits participate in the uniform check.
//! Missing inventories, value-less returns, non-`new` non-`null` return
//! values (forwarded calls, locals, parameters), and mixed classes all
//! leave the callable unclaimed — additive evidence, never a fallback.

use std::collections::BTreeMap;

use crate::mir::resolved_semantics::{
    BodyStatementShapeV1, ResolvedLiteralSourceV1, VerifiedResolvedBodyShapeInventoryV1,
    VerifiedResolvedFunctionV1,
};
use crate::parser::ParserOrdinaryBoxSourceCoverageV1;
use hakorune_mir_defs::CanonicalSameModuleCallableKeyV1;

/// The proven result class of a selected callable. The class name is the
/// agreed `new` class; the arm records whether a `null` literal exit
/// also exists. `NullableObject` is never a Handle authorization — it
/// only states that every sealed exit is `new C(...)` or `null`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum OrdinaryNewResultClassV1 {
    /// Every `return` constructs `new` of the agreed class — definite object.
    Object(Box<str>),
    /// Exits are `new` of the agreed class or the exact `null` literal.
    NullableObject(Box<str>),
}

impl OrdinaryNewResultClassV1 {
    pub(crate) fn class(&self) -> &str {
        match self {
            Self::Object(class) | Self::NullableObject(class) => class.as_ref(),
        }
    }
}

/// Key: the selected canonical callable key. Value: the agreed result
/// class claim every `return` satisfies.
pub(crate) type OrdinaryNewResultClassClaimsV1 =
    BTreeMap<CanonicalSameModuleCallableKeyV1, OrdinaryNewResultClassV1>;

#[derive(Default)]
pub(crate) struct OrdinaryNewResultClassClaimDraftV1 {
    claims: BTreeMap<CanonicalSameModuleCallableKeyV1, OrdinaryNewResultClassV1>,
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
            Some(BodyStatementShapeV1::Return { value: Some(_), .. })
        ) {
            return;
        }
        let mut class: Option<Box<str>> = None;
        let mut nullable = false;
        for statement in statements {
            let BodyStatementShapeV1::Return { value, .. } = statement else {
                continue;
            };
            let Some(site) = value else {
                return;
            };
            // `return null`: the sealed literal row proves the exact null
            // exit. At least one `new` exit must still agree on the class —
            // a null-only callee carries no object claim.
            if matches!(
                function.expression_source().literal(site),
                Some(ResolvedLiteralSourceV1::Null)
            ) {
                nullable = true;
                continue;
            }
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
        self.claims.insert(
            key.clone(),
            if nullable {
                OrdinaryNewResultClassV1::NullableObject(class)
            } else {
                OrdinaryNewResultClassV1::Object(class)
            },
        );
    }

    /// Seal the observed rows, keeping only claims whose class is an
    /// ordinary box of this package.
    pub(crate) fn finish(
        self,
        ordinary_box_coverage: &ParserOrdinaryBoxSourceCoverageV1,
    ) -> OrdinaryNewResultClassClaimsV1 {
        self.claims
            .into_iter()
            .filter(|(_, claim)| {
                ordinary_box_coverage
                    .row_for(claim.class())
                    .ok()
                    .flatten()
                    .is_some()
            })
            .collect()
    }
}
