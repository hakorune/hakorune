//! Existing descriptor ownership before the caller source walk.
//! This private candidate issues no availability, transfer or physical progress.
use super::*;

pub(super) struct OrdinaryNewCandidate {
    pub(super) site: OwnedExprSiteV1,
    pub(super) box_source: crate::parser::ParserOrdinaryBoxSourceRowV1,
    pub(super) class: Box<str>,
    pub(super) arity: usize,
    pub(super) destination: BindingRefV1,
    pub(super) declaration: SourceBindingSiteV1,
    pub(super) construction: ConstructionEligibilityV1,
    pub(super) object: CanonicalObjectIdV1,
    pub(super) destruction: ObjectDestructionDispositionV1,
    pub(super) constructor: OrdinaryNewConstructorDispositionV1,
    pub(super) birth_handoff: Option<BirthAbiHandoffV1>,
}

impl OrdinaryNewCandidate {
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
                    if row.box_name() != class.as_ref()
                        || usize::try_from(row.source_arity()).ok() != Some(arity)
                    {
                        return Err(OrdinaryNewCoSealIssueV1::ConstructorRelationMismatch {
                            site,
                            class,
                            arity,
                        });
                    }
                    let abi = InstanceConstructorAbiV1::issue(arity).map_err(|error| {
                        OrdinaryNewCoSealIssueV1::ConstructorAbi {
                            site: site.clone(),
                            class: class.clone(),
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
                            class: class.clone(),
                            arity,
                        })?
                        .clone();
                    row.birth_completion()
                        .filter(|completion| {
                            row.forest().roots() == [completion.owner()]
                                && !completion.returns_value()
                        })
                        .ok_or_else(|| OrdinaryNewCoSealIssueV1::BirthCompletionNotUnit {
                            site: site.clone(),
                            class: class.clone(),
                        })?;
                    let effect = row
                        .birth_effect()
                        .filter(|effect| {
                            *effect == DeclaredInstanceCallSemanticEffectV1::OpaqueObservable
                        })
                        .ok_or_else(|| OrdinaryNewCoSealIssueV1::BirthEffectUnsupported {
                            site: site.clone(),
                            class: class.clone(),
                        })?;
                    let birth_abi =
                        BirthAbiHandoffV1::issue(row, target.clone(), abi).map_err(|_| {
                            OrdinaryNewCoSealIssueV1::ConstructorRelationMismatch {
                                site: site.clone(),
                                class: class.clone(),
                                arity,
                            }
                        })?;
                    birth_handoff = Some(birth_abi);
                    OrdinaryNewConstructorDispositionV1::Birth(VerifiedOrdinaryNewBirthRecipeV1 {
                        source_id: row.source_id().clone(),
                        target,
                        effect,
                        abi,
                    })
                }
                None => no_birth_constructor_disposition(&site, &class, arity)?,
            };
        Ok(Some(Self {
            site,
            box_source: box_source.clone(),
            class,
            arity,
            destination,
            declaration,
            construction,
            object,
            destruction,
            constructor,
            birth_handoff,
        }))
    }
}
