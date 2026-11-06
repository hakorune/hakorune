//! Existing descriptor ownership before the caller source walk.
//! This private candidate issues no availability, transfer or physical progress.
use super::*;

#[path = "ordinary_new_local_new_source.rs"]
mod source;
pub(in crate::mir::normal_callable_semantic_package) use source::LocalNewRootSourceV1;

/// Site-level resolution products shared by the local-initializer claim
/// lane and the return-position result-new claim lane. Destination and
/// declaration are deliberately absent: return-position `new` installs no
/// local and transfers its fresh object to the caller.
pub(super) struct OrdinaryNewSiteResolutionV1 {
    pub(super) site: OwnedExprSiteV1,
    pub(super) box_source: crate::parser::ParserOrdinaryBoxSourceRowV1,
    pub(super) class: Box<str>,
    pub(super) arity: usize,
    pub(super) construction: ConstructionEligibilityV1,
    pub(super) construction_source: ConstructionSourceEligibilityV1,
    pub(super) object: CanonicalObjectIdV1,
    pub(super) destruction: ObjectDestructionDispositionV1,
    pub(super) constructor: OrdinaryNewConstructorDispositionV1,
    pub(super) birth_handoff: Option<BirthAbiHandoffV1>,
}

pub(super) struct OrdinaryNewCandidate {
    pub(super) site: OwnedExprSiteV1,
    pub(super) box_source: crate::parser::ParserOrdinaryBoxSourceRowV1,
    pub(super) class: Box<str>,
    pub(super) arity: usize,
    pub(super) destination: BindingRefV1,
    pub(super) declaration: SourceBindingSiteV1,
    pub(super) construction: ConstructionEligibilityV1,
    pub(super) construction_source: ConstructionSourceEligibilityV1,
    pub(super) allocation_execution: Option<AllocationOnlyNewExecutionV1>,
    pub(super) object: CanonicalObjectIdV1,
    pub(super) destruction: ObjectDestructionDispositionV1,
    pub(super) constructor: OrdinaryNewConstructorDispositionV1,
    pub(super) birth_handoff: Option<BirthAbiHandoffV1>,
}

impl OrdinaryNewCandidate {
    /// Resolve the position-independent site products: coverage, object
    /// identity, destruction disposition, construction eligibility and the
    /// verified `Birth` recipe/handoff. Membership position — initializer
    /// destination or return boundary — stays the caller's concern.
    pub(super) fn resolve_site(
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        instance_constructors: &VerifiedInstanceConstructorSemanticBatchV1,
        site: OwnedExprSiteV1,
        class: Box<str>,
        arity: usize,
        has_overrides: bool,
    ) -> Result<Option<OrdinaryNewSiteResolutionV1>, OrdinaryNewCoSealIssueV1> {
        let Some(box_source) = batch
            .ordinary_box_coverage()
            .row_for(class.as_ref())
            .map_err(|_| OrdinaryNewCoSealIssueV1::OrdinaryBoxCoverageDuplicate {
                site: site.clone(),
                class: class.clone(),
            })?
        else {
            // Builtin/plugin constructors retain their existing
            // compatibility owner.  They are deliberately outside the
            // source-backed ordinary-Box claim ledger; only an unknown
            // user Box is a coverage error here.
            if crate::box_trait::is_builtin_box(class.as_ref()) {
                return Ok(None);
            }
            return Err(OrdinaryNewCoSealIssueV1::OrdinaryBoxCoverageMissing { site, class });
        };
        let (object, destruction) =
            instance_constructors
                .destruction_for(box_source)
                .map_err(|error| OrdinaryNewCoSealIssueV1::ConstructorLookup {
                    site: site.clone(),
                    class: class.clone(),
                    error,
                })?;
        let construction = if has_overrides {
            Err(ConstructionUnavailableV1::OverrideUnsupported)
        } else {
            instance_constructors
                .construction_for(box_source, arity)
                .map_err(|error| OrdinaryNewCoSealIssueV1::ConstructorLookup {
                    site: site.clone(),
                    class: class.clone(),
                    error,
                })?
                .clone()
        };
        if matches!(&construction, Ok(plan) if plan.object() != object) {
            return Err(OrdinaryNewCoSealIssueV1::ConstructorRelationMismatch {
                site,
                class,
                arity,
            });
        }
        let mut birth_handoff = None;
        let constructor =
            match instance_constructors
                .birth_for(box_source, arity)
                .map_err(|error| OrdinaryNewCoSealIssueV1::ConstructorLookup {
                    site: site.clone(),
                    class: class.clone(),
                    error,
                })? {
                Some(row) => {
                    let (recipe, handoff) =
                        verified_birth_recipe_for_site_v1(&site, class.as_ref(), arity, row)?;
                    birth_handoff = Some(handoff);
                    OrdinaryNewConstructorDispositionV1::Birth(recipe)
                }
                None => no_birth_constructor_disposition(&site, &class, arity)?,
            };
        // Preserve missing-Birth diagnostic ownership above before borrowing
        // source. Retention grants neither execution nor return/escape permission.
        let construction_source = if has_overrides {
            Err(ConstructionUnavailableV1::OverrideUnsupported)
        } else {
            instance_constructors.construction_source_for(box_source, arity)
                .map_err(|error| OrdinaryNewCoSealIssueV1::ConstructorLookup {
                    site: site.clone(), class: class.clone(), error,
                })?.clone()
        };
        if matches!(&construction_source, Ok(source) if source.object() != object) {
            return Err(OrdinaryNewCoSealIssueV1::ConstructorRelationMismatch { site, class, arity });
        }
        Ok(Some(OrdinaryNewSiteResolutionV1 {
            site,
            box_source: box_source.clone(),
            class,
            arity,
            construction,
            construction_source,
            object,
            destruction,
            constructor,
            birth_handoff,
        }))
    }

    pub(super) fn resolve(
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        instance_constructors: &VerifiedInstanceConstructorSemanticBatchV1,
        site: OwnedExprSiteV1,
        class: Box<str>,
        arity: usize,
        destination: BindingRefV1,
        declaration: SourceBindingSiteV1,
        has_overrides: bool,
    ) -> Result<Option<Self>, OrdinaryNewCoSealIssueV1> {
        let Some(resolution) = Self::resolve_site(
            batch,
            instance_constructors,
            site,
            class,
            arity,
            has_overrides,
        )?
        else {
            return Ok(None);
        };
        let allocation_execution = AllocationOnlyNewExecutionV1::issue_local(
            &resolution.construction, &resolution.construction_source, &resolution.constructor,
            resolution.arity, resolution.object, resolution.destruction);
        Ok(Some(Self {
            allocation_execution,
            site: resolution.site,
            box_source: resolution.box_source,
            class: resolution.class,
            arity: resolution.arity,
            destination,
            declaration,
            construction: resolution.construction,
            construction_source: resolution.construction_source,
            object: resolution.object,
            destruction: resolution.destruction,
            constructor: resolution.constructor,
            birth_handoff: resolution.birth_handoff,
        }))
    }
}

/// Verify one constructor row into the `Birth` recipe + ABI handoff for a
/// `new` site.  Shared by the local-initializer claim census and the
/// destination-less birth-site index; it issues no site authority itself.
pub(super) fn verified_birth_recipe_for_site_v1(
    site: &OwnedExprSiteV1,
    class: &str,
    arity: usize,
    row: &VerifiedInstanceConstructorSemanticRowV1,
) -> Result<(VerifiedOrdinaryNewBirthRecipeV1, BirthAbiHandoffV1), OrdinaryNewCoSealIssueV1> {
    if row.box_name() != class || usize::try_from(row.source_arity()).ok() != Some(arity) {
        return Err(OrdinaryNewCoSealIssueV1::ConstructorRelationMismatch {
            site: site.clone(),
            class: class.into(),
            arity,
        });
    }
    let abi = InstanceConstructorAbiV1::issue(arity).map_err(|error| {
        OrdinaryNewCoSealIssueV1::ConstructorAbi {
            site: site.clone(),
            class: class.into(),
            error,
        }
    })?;
    let target = row
        .published_birth_key()
        .filter(|key| {
            key.namespace() == SameModuleCallableNamespaceV1::BirthConstructor
                && key.owner() == row.box_name()
                && key.arity() == row.source_arity()
        })
        .ok_or_else(|| OrdinaryNewCoSealIssueV1::BirthTargetInvalid {
            site: site.clone(),
            class: class.into(),
            arity,
        })?
        .clone();
    row.birth_completion()
        .filter(|completion| {
            row.forest().roots() == [completion.owner()] && !completion.returns_value()
        })
        .ok_or_else(|| OrdinaryNewCoSealIssueV1::BirthCompletionNotUnit {
            site: site.clone(),
            class: class.into(),
        })?;
    let effect = row
        .birth_effect()
        .filter(|effect| *effect == DeclaredInstanceCallSemanticEffectV1::OpaqueObservable)
        .ok_or_else(|| OrdinaryNewCoSealIssueV1::BirthEffectUnsupported {
            site: site.clone(),
            class: class.into(),
        })?;
    let handoff = BirthAbiHandoffV1::issue(row, target.clone(), abi).map_err(|_| {
        OrdinaryNewCoSealIssueV1::ConstructorRelationMismatch {
            site: site.clone(),
            class: class.into(),
            arity,
        }
    })?;
    Ok((
        VerifiedOrdinaryNewBirthRecipeV1 {
            source_id: row.source_id().clone(),
            target,
            effect,
            abi,
        },
        handoff,
    ))
}

/// A `take` formal owns exactly one non-optional user box. This is a property of
/// the declaration alone, so it is judged before any actual is classified.
pub(super) fn validate_take_declarations_v1(
    site: &OwnedExprSiteV1,
    class: &str,
    row: &VerifiedInstanceConstructorSemanticRowV1,
) -> Result<(), OrdinaryNewCoSealIssueV1> {
    
    match row.formal_contracts().iter().find(|formal| formal.is_take() && !formal.declares_exact_object()) {
        Some(formal) => Err(OrdinaryNewCoSealIssueV1::TakeFormalDeclarationUnavailable {
            site: site.clone(),
            class: class.into(),
            ordinal: formal.ordinal(),
        }),
        None => Ok(()),
    }
}

/// Every `take` birth formal is fed by an explicit `move x` of a handle local. A
/// plain `x`, a literal, or any other actual has no ownership to hand over.
pub(super) fn validate_take_actuals_v1(
    site: &OwnedExprSiteV1,
    class: &str,
    row: &VerifiedInstanceConstructorSemanticRowV1,
    arguments: &[OrdinaryNewTrivialArgumentV1],
) -> Result<(), OrdinaryNewCoSealIssueV1> {
    
    use crate::mir::normal_callable_semantic_package::instance_construction::ConstructionUnavailableV1 as Unavailable;
    // A birth with a `take` formal has no legacy route (it cannot lower `move`), so every
    // reason its source or plan is unavailable is a refusal, not only the two spelling errors.
    if let Some(issue) = row.construction_source().as_ref().err().or(row.construction().as_ref().err()) {
        let (site, class, issue) = (site.clone(), class.into(), *issue);
        if matches!(issue, Unavailable::TakeStoreRequiresMove | Unavailable::MoveRequiresTakeFormal) {
            return Err(OrdinaryNewCoSealIssueV1::BirthStoreOwnershipMismatch { site, class, issue });
        }
        if row.formal_contracts().iter().any(|formal| formal.is_take()) {
            return Err(OrdinaryNewCoSealIssueV1::TakeBirthSourceUnconnected { site, class, issue });
        }
    }
    for formal in row.formal_contracts() {
        if !formal.declares_exact_object() {
            continue;
        }
        let moved = arguments.iter().any(|argument| argument.ordinal() == formal.ordinal()
            && argument.explicit_move()
            && matches!(argument.kind(), OrdinaryNewTrivialArgumentKindV1::Handle { .. }));
        if !moved {
            return Err(OrdinaryNewCoSealIssueV1::TakeActualRequiresMove {
                site: site.clone(),
                class: class.into(),
                ordinal: formal.ordinal(),
            });
        }
        if matches!(formal.uses(), crate::mir::normal_callable_semantic_package::BirthFormalUseCoverageV1::NoUse) {
            return Err(OrdinaryNewCoSealIssueV1::TakeFormalNotStored {
                site: site.clone(),
                class: class.into(),
                ordinal: formal.ordinal(),
            });
        }
    }
    Ok(())
}
