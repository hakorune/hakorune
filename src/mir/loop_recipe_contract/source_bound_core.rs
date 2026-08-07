//! Caller-zero source-bound core for the portable Loop contract.
//!
//! This module co-seals already verified Recipe/JoinSig/source-wire products
//! with resolver-issued source relations.  It does not issue Recipe keys,
//! inspect AST, select a family, or allocate physical identities.

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::resolved_semantics::{
    BindingOriginV1, BindingRefV1, FunctionOwnerIdV1, OwnedExprSiteV1, SourceStmtSiteV1,
};

use super::error::LoopRecipeRejectReasonV1 as Reject;
use super::ids::{LoopBindingKeyV1, LoopCarrierKeyV1};
use super::join_sig::VerifiedLoopJoinSigV1;
use super::schema::{LoopRecipeV1, LoopValueClassV1};
use super::source_binding::StructurallyVerifiedLoopRecipeSourceClaimV1;
#[cfg(test)]
use super::verify::LoopRecipeVerifierV1;
use super::verify::{VerifiedLoopRecipeArtifactV1, VerifiedLoopRecipeV1};

/// Unsealed source relation DTO supplied by a source projector or a later
/// profile producer.  This type carries no authority until the core issuer
/// validates the complete relation set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoopRecipeBindingRelationV1 {
    recipe_binding: LoopBindingKeyV1,
    source_binding: BindingRefV1,
    class: LoopValueClassV1,
    declaration: BindingOriginV1,
}

impl LoopRecipeBindingRelationV1 {
    pub(crate) fn new(
        recipe_binding: LoopBindingKeyV1,
        source_binding: BindingRefV1,
        class: LoopValueClassV1,
        declaration: BindingOriginV1,
    ) -> Self {
        Self {
            recipe_binding,
            source_binding,
            class,
            declaration,
        }
    }
}

/// Typed source anchor for one effect.  Derived carrier entries are anchored
/// at a source loop statement, not fabricated as expression sites.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum LoopBindingEffectAnchorV1 {
    Expr(OwnedExprSiteV1),
    DerivedCarrierEntry {
        owner: FunctionOwnerIdV1,
        source_loop: SourceStmtSiteV1,
        carrier: LoopCarrierKeyV1,
    },
}

impl LoopBindingEffectAnchorV1 {
    pub(crate) fn owner(&self) -> FunctionOwnerIdV1 {
        match self {
            Self::Expr(site) => site.owner(),
            Self::DerivedCarrierEntry { owner, .. } => *owner,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum LoopBindingEffectRoleV1 {
    SourceRead { ordinal: u32 },
    SourceWrite { ordinal: u32 },
    DerivedCarrierEntry,
}

/// Unsealed effect relation DTO.  Multiple reads/writes of one binding are
/// allowed; the role/anchor pair must still be unique.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoopBindingEffectRelationV1 {
    role: LoopBindingEffectRoleV1,
    recipe_binding: LoopBindingKeyV1,
    source_binding: BindingRefV1,
    class: LoopValueClassV1,
    anchor: LoopBindingEffectAnchorV1,
}

impl LoopBindingEffectRelationV1 {
    pub(crate) fn new(
        role: LoopBindingEffectRoleV1,
        recipe_binding: LoopBindingKeyV1,
        source_binding: BindingRefV1,
        class: LoopValueClassV1,
        anchor: LoopBindingEffectAnchorV1,
    ) -> Self {
        Self {
            role,
            recipe_binding,
            source_binding,
            class,
            anchor,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedLoopRecipeBindingRelationV1(LoopRecipeBindingRelationV1);

impl VerifiedLoopRecipeBindingRelationV1 {
    pub(crate) fn recipe_binding(&self) -> LoopBindingKeyV1 {
        self.0.recipe_binding
    }

    pub(crate) fn source_binding(&self) -> BindingRefV1 {
        self.0.source_binding
    }

    pub(crate) fn class(&self) -> LoopValueClassV1 {
        self.0.class
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedLoopBindingEffectRelationV1(LoopBindingEffectRelationV1);

impl VerifiedLoopBindingEffectRelationV1 {
    pub(crate) fn role(&self) -> LoopBindingEffectRoleV1 {
        self.0.role
    }

    pub(crate) fn recipe_binding(&self) -> LoopBindingKeyV1 {
        self.0.recipe_binding
    }

    pub(crate) fn source_binding(&self) -> BindingRefV1 {
        self.0.source_binding
    }

    /// Read-only view used by the operation/effect join. The effect relation
    /// remains the sole owner of this anchor; callers must not copy it into a
    /// second effect catalog.
    pub(crate) fn anchor(&self) -> &LoopBindingEffectAnchorV1 {
        &self.0.anchor
    }

    pub(crate) fn class(&self) -> LoopValueClassV1 {
        self.0.class
    }
}

/// The sole source-bound logical co-seal product.  It is move-only and keeps
/// the structural source claim opaque to later Recipe/physical consumers.
#[derive(Debug)]
pub(crate) struct VerifiedLoopCoreProductV1 {
    owner: FunctionOwnerIdV1,
    recipe: VerifiedLoopRecipeV1,
    join_sig: VerifiedLoopJoinSigV1,
    source_claim: StructurallyVerifiedLoopRecipeSourceClaimV1,
    binding_relations: Box<[VerifiedLoopRecipeBindingRelationV1]>,
    effect_relations: Box<[VerifiedLoopBindingEffectRelationV1]>,
}

impl VerifiedLoopCoreProductV1 {
    pub(crate) fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) fn recipe(&self) -> &VerifiedLoopRecipeV1 {
        &self.recipe
    }

    pub(crate) fn join_sig(&self) -> &VerifiedLoopJoinSigV1 {
        &self.join_sig
    }

    pub(crate) fn binding_relations(&self) -> &[VerifiedLoopRecipeBindingRelationV1] {
        &self.binding_relations
    }

    pub(crate) fn effect_relations(&self) -> &[VerifiedLoopBindingEffectRelationV1] {
        &self.effect_relations
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        FunctionOwnerIdV1,
        VerifiedLoopRecipeV1,
        VerifiedLoopJoinSigV1,
        StructurallyVerifiedLoopRecipeSourceClaimV1,
        Box<[VerifiedLoopRecipeBindingRelationV1]>,
        Box<[VerifiedLoopBindingEffectRelationV1]>,
    ) {
        (
            self.owner,
            self.recipe,
            self.join_sig,
            self.source_claim,
            self.binding_relations,
            self.effect_relations,
        )
    }
}

/// Co-seals one already verified artifact and one already verified JoinSig.
/// The Generic producer remains the sole issuer of real relation instances;
/// this issuer only verifies and seals the shared transport contract.
pub(super) fn issue_source_bound_core_v1(
    artifact: VerifiedLoopRecipeArtifactV1,
    join_sig: VerifiedLoopJoinSigV1,
    owner: FunctionOwnerIdV1,
    bindings: Vec<LoopRecipeBindingRelationV1>,
    effects: Vec<LoopBindingEffectRelationV1>,
) -> Result<VerifiedLoopCoreProductV1, Reject> {
    let (source_claim, recipe) = artifact.into_source_bound_parts();
    verify_join_sig_pair(recipe.as_recipe(), &join_sig)?;
    let binding_relations = verify_binding_relations(recipe.as_recipe(), owner, bindings)?;
    let effect_relations = verify_effect_relations(recipe.as_recipe(), owner, effects)?;
    Ok(VerifiedLoopCoreProductV1 {
        owner,
        recipe,
        join_sig,
        source_claim,
        binding_relations: binding_relations.into_boxed_slice(),
        effect_relations: effect_relations.into_boxed_slice(),
    })
}

/// Test-only bridge that keeps the source artifact verifier as the sole wire
/// claim issuer while exercising the core co-seal boundary.
#[cfg(test)]
pub(crate) fn issue_source_bound_core_for_test(
    artifact: super::schema::LoopRecipeArtifactV1,
    join_sig: VerifiedLoopJoinSigV1,
    owner: FunctionOwnerIdV1,
    bindings: Vec<LoopRecipeBindingRelationV1>,
    effects: Vec<LoopBindingEffectRelationV1>,
) -> Result<VerifiedLoopCoreProductV1, Reject> {
    let verified = LoopRecipeVerifierV1::verify_artifact(artifact)?;
    issue_source_bound_core_v1(verified, join_sig, owner, bindings, effects)
}

fn verify_binding_relations(
    recipe: &LoopRecipeV1,
    owner: FunctionOwnerIdV1,
    rows: Vec<LoopRecipeBindingRelationV1>,
) -> Result<Vec<VerifiedLoopRecipeBindingRelationV1>, Reject> {
    if rows.len() != recipe.bindings.len() {
        return Err(Reject::SourceBoundBindingCoverageMismatch {
            expected: recipe.bindings.len(),
            found: rows.len(),
        });
    }
    let expected = recipe
        .bindings
        .iter()
        .map(|row| (row.key, row.class))
        .collect::<BTreeMap<_, _>>();
    let mut seen_keys = BTreeSet::new();
    let mut seen_sources = BTreeSet::new();
    let mut verified = Vec::with_capacity(rows.len());
    for row in rows {
        if !seen_keys.insert(row.recipe_binding) {
            return Err(Reject::SourceBoundDuplicateRecipeBinding {
                key: row.recipe_binding,
            });
        }
        let Some(expected_class) = expected.get(&row.recipe_binding).copied() else {
            return Err(Reject::SourceBoundUnknownRecipeBinding {
                key: row.recipe_binding,
            });
        };
        if row.source_binding.owner() != owner {
            return Err(Reject::SourceBoundForeignBinding {
                key: row.recipe_binding,
            });
        }
        if !seen_sources.insert(row.source_binding) {
            return Err(Reject::SourceBoundDuplicateSourceBinding {
                binding: row.source_binding,
            });
        }
        if row.class != expected_class {
            return Err(Reject::SourceBoundBindingClassMismatch {
                key: row.recipe_binding,
            });
        }
        if !matches!(row.declaration, BindingOriginV1::Source(_)) {
            return Err(Reject::SourceBoundSyntheticDeclaration {
                key: row.recipe_binding,
            });
        }
        verified.push(VerifiedLoopRecipeBindingRelationV1(row));
    }
    if seen_keys.len() != expected.len() {
        return Err(Reject::SourceBoundBindingCoverageMismatch {
            expected: expected.len(),
            found: seen_keys.len(),
        });
    }
    verified.sort_by_key(|row| row.recipe_binding());
    Ok(verified)
}

fn verify_effect_relations(
    recipe: &LoopRecipeV1,
    owner: FunctionOwnerIdV1,
    rows: Vec<LoopBindingEffectRelationV1>,
) -> Result<Vec<VerifiedLoopBindingEffectRelationV1>, Reject> {
    let bindings = recipe
        .bindings
        .iter()
        .map(|row| (row.key, row.class))
        .collect::<BTreeMap<_, _>>();
    let carriers = recipe
        .carriers
        .iter()
        .map(|row| {
            (
                row.key,
                (row.owner_loop, row.binding, row.class, row.entry_value),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let mut seen = BTreeSet::new();
    let mut seen_anchors = BTreeSet::new();
    let mut verified = Vec::with_capacity(rows.len());
    for row in rows {
        let Some(expected_class) = bindings.get(&row.recipe_binding).copied() else {
            return Err(Reject::SourceBoundUnknownRecipeBinding {
                key: row.recipe_binding,
            });
        };
        if row.source_binding.owner() != owner {
            return Err(Reject::SourceBoundForeignEffectAnchor {
                key: row.recipe_binding,
            });
        }
        if row.class != expected_class {
            return Err(Reject::SourceBoundEffectClassMismatch {
                key: row.recipe_binding,
            });
        }
        if !seen.insert((row.role, row.recipe_binding, row.anchor.clone())) {
            return Err(Reject::SourceBoundDuplicateEffect {
                key: row.recipe_binding,
            });
        }
        if !seen_anchors.insert(row.anchor.clone()) {
            return Err(Reject::SourceBoundDuplicateEffect {
                key: row.recipe_binding,
            });
        }
        match (&row.role, &row.anchor) {
            (
                LoopBindingEffectRoleV1::SourceRead { .. }
                | LoopBindingEffectRoleV1::SourceWrite { .. },
                LoopBindingEffectAnchorV1::Expr(site),
            ) if site.owner() == owner => {}
            (
                LoopBindingEffectRoleV1::DerivedCarrierEntry,
                LoopBindingEffectAnchorV1::DerivedCarrierEntry {
                    owner: anchor_owner,
                    source_loop,
                    carrier,
                },
            ) => {
                if *anchor_owner != owner || !is_loop_statement_site(source_loop) {
                    return Err(Reject::SourceBoundDerivedAnchorEmpty { carrier: *carrier });
                }
                let Some((owner_loop, binding, class, _entry_value)) = carriers.get(carrier) else {
                    return Err(Reject::SourceBoundDerivedCarrierMismatch { carrier: *carrier });
                };
                if *binding != row.recipe_binding || *class != row.class {
                    return Err(Reject::SourceBoundDerivedCarrierMismatch { carrier: *carrier });
                }
                if owner_loop.raw() as usize >= recipe.loops.len() {
                    return Err(Reject::SourceBoundDerivedCarrierMismatch { carrier: *carrier });
                }
            }
            _ => {
                return Err(Reject::SourceBoundEffectRoleMismatch {
                    key: row.recipe_binding,
                });
            }
        }
        verified.push(VerifiedLoopBindingEffectRelationV1(row));
    }
    verified.sort_by_key(|row| (row.recipe_binding(), row.role()));
    Ok(verified)
}

fn is_loop_statement_site(site: &SourceStmtSiteV1) -> bool {
    site.node().segments().iter().any(|segment| {
        matches!(
            segment,
            crate::mir::resolved_semantics::SourcePathSegmentV1::LoopCondition
                | crate::mir::resolved_semantics::SourcePathSegmentV1::LoopBodyRoot
                | crate::mir::resolved_semantics::SourcePathSegmentV1::LoopBody(_)
                // Declared-callable resolver products retain a root loop as
                // a function-body statement. Membership was already sealed
                // by the resolver token before this structural check.
                | crate::mir::resolved_semantics::SourcePathSegmentV1::Body(_)
        )
    })
}

fn verify_join_sig_pair(
    recipe: &LoopRecipeV1,
    join_sig: &VerifiedLoopJoinSigV1,
) -> Result<(), Reject> {
    let sig = join_sig.as_sig();
    if sig.loops.len() != recipe.loops.len() {
        return Err(Reject::SourceBoundJoinSigMismatch);
    }
    let binding_classes = recipe
        .bindings
        .iter()
        .map(|row| (row.key, row.class))
        .collect::<BTreeMap<_, _>>();
    let value_classes = recipe
        .values
        .iter()
        .map(|row| (row.key, row.class))
        .collect::<BTreeMap<_, _>>();
    let loop_rows = recipe
        .loops
        .iter()
        .map(|row| (row.key, row.parent))
        .collect::<BTreeMap<_, _>>();
    let mut seen_loops = BTreeSet::new();
    for row in &sig.loops {
        if !seen_loops.insert(row.key) || loop_rows.get(&row.key) != Some(&row.parent) {
            return Err(Reject::SourceBoundJoinSigMismatch);
        }
        for payload in &row.carriers {
            if binding_classes.get(&payload.binding) != Some(&payload.class)
                || value_classes.get(&payload.value) != Some(&payload.class)
            {
                return Err(Reject::SourceBoundJoinSigMismatch);
            }
        }
        for edge in &row.edges {
            for payload in &edge.payload {
                if binding_classes.get(&payload.binding) != Some(&payload.class)
                    || value_classes.get(&payload.value) != Some(&payload.class)
                {
                    return Err(Reject::SourceBoundJoinSigMismatch);
                }
            }
        }
    }
    let port_bindings = sig
        .port_bindings
        .iter()
        .map(|row| row.binding)
        .collect::<BTreeSet<_>>();
    if seen_loops.len() != loop_rows.len()
        || binding_classes
            .keys()
            .any(|binding| !port_bindings.contains(binding))
        || sig.port_bindings.iter().any(|row| {
            row.loop_key.raw() as usize >= recipe.loops.len()
                || binding_classes.get(&row.binding) != Some(&row.class)
        })
    {
        return Err(Reject::SourceBoundJoinSigMismatch);
    }
    Ok(())
}
