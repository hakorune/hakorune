//! Source-bound admission claims for the first Raw ordinary-`New` cohort.
//!
//! The claim is issued from the parser's final ordinary-box coverage and the
//! resolver's exact direct-body allocation site.  Builder headers, symbol
//! scans, and post-lowering target inference are deliberately outside this
//! owner.

use std::{cell::RefCell, collections::BTreeMap};

use super::instance_constructor_semantic::{
    InstanceConstructorBirthLookupErrorV1, VerifiedInstanceConstructorSemanticBatchV1,
};
use super::selected_mapping::VerifiedSelectedCallableBatchMapV1;
use crate::ast::ASTNode;
use crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1;
use crate::mir::instance_constructor_abi::{
    InstanceConstructorAbiErrorV1, InstanceConstructorAbiV1,
};
use crate::mir::resolved_semantics::{
    BodyEffectKindV1, OwnedExprSiteV1, SourceExprSiteV1, SourcePathSegmentV1,
};
use hakorune_mir_defs::{CanonicalGlobalTargetConstructionErrorV1, CanonicalGlobalTargetV1};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedOrdinaryNewBirthRecipeV1 {
    source_id: crate::parser::ConstructorSourceIdV1,
    target: CanonicalGlobalTargetV1,
    abi: InstanceConstructorAbiV1,
}

impl VerifiedOrdinaryNewBirthRecipeV1 {
    pub(crate) fn source_id(&self) -> &crate::parser::ConstructorSourceIdV1 {
        &self.source_id
    }

    pub(crate) fn target(self) -> CanonicalGlobalTargetV1 {
        self.target
    }

    pub(crate) const fn abi(&self) -> InstanceConstructorAbiV1 {
        self.abi
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum OrdinaryNewConstructorDispositionV1 {
    NoBirthZero,
    Birth(VerifiedOrdinaryNewBirthRecipeV1),
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct OrdinaryNewAdmissionClaimV1 {
    site: OwnedExprSiteV1,
    class: Box<str>,
    arity: usize,
    constructor: OrdinaryNewConstructorDispositionV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OrdinaryNewClaimTakeErrorV1 {
    Unavailable,
    Mismatch,
}

#[derive(Debug)]
pub(crate) struct OrdinaryNewClaimLedgerV1 {
    claims: RefCell<BTreeMap<OwnedExprSiteV1, OrdinaryNewAdmissionClaimV1>>,
    ordinary_box_names: Box<[Box<str>]>,
}

impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn issue(
        claims: Box<[OrdinaryNewAdmissionClaimV1]>,
        ordinary_box_names: Box<[Box<str>]>,
    ) -> Self {
        Self {
            claims: RefCell::new(
                claims
                    .into_vec()
                    .into_iter()
                    .map(|claim| (claim.site().clone(), claim))
                    .collect(),
            ),
            ordinary_box_names,
        }
    }

    pub(crate) fn try_take(
        &self,
        site: &OwnedExprSiteV1,
        class: &str,
        arity: usize,
    ) -> Result<Option<OrdinaryNewAdmissionClaimV1>, OrdinaryNewClaimTakeErrorV1> {
        if !self
            .ordinary_box_names
            .iter()
            .any(|name| name.as_ref() == class)
        {
            return Ok(None);
        }
        let mut claims = self.claims.borrow_mut();
        let claim = claims
            .get(site)
            .ok_or(OrdinaryNewClaimTakeErrorV1::Unavailable)?;
        if claim.class() != class || claim.arity() != arity {
            return Err(OrdinaryNewClaimTakeErrorV1::Mismatch);
        }
        Ok(Some(
            claims
                .remove(site)
                .expect("claim remained present after the checked lookup"),
        ))
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.claims.borrow().is_empty()
    }
}

impl OrdinaryNewAdmissionClaimV1 {
    pub(crate) fn site(&self) -> &OwnedExprSiteV1 {
        &self.site
    }

    pub(crate) fn class(&self) -> &str {
        &self.class
    }

    pub(crate) const fn arity(&self) -> usize {
        self.arity
    }

    pub(crate) fn constructor(self) -> OrdinaryNewConstructorDispositionV1 {
        self.constructor
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum OrdinaryNewCoSealIssueV1 {
    BatchLoan,
    SourceNavigation {
        site: OwnedExprSiteV1,
    },
    AllocationSiteNotDirectLocal {
        site: SourceExprSiteV1,
    },
    AllocationSiteNotNew {
        site: OwnedExprSiteV1,
    },
    OrdinaryBoxCoverageMissing {
        site: OwnedExprSiteV1,
        class: Box<str>,
    },
    OrdinaryBoxCoverageDuplicate {
        site: OwnedExprSiteV1,
        class: Box<str>,
    },
    ConstructorSourceOrdinalOverflow {
        site: OwnedExprSiteV1,
        class: Box<str>,
    },
    ConstructorLookup {
        site: OwnedExprSiteV1,
        class: Box<str>,
        error: InstanceConstructorBirthLookupErrorV1,
    },
    ConstructorAbi {
        site: OwnedExprSiteV1,
        class: Box<str>,
        error: InstanceConstructorAbiErrorV1,
    },
    BirthTargetInvalid {
        site: OwnedExprSiteV1,
        class: Box<str>,
        arity: usize,
        error: CanonicalGlobalTargetConstructionErrorV1,
    },
    ConstructorRelationMismatch {
        site: OwnedExprSiteV1,
        class: Box<str>,
        arity: usize,
    },
    BirthConstructorMissing {
        site: OwnedExprSiteV1,
        class: Box<str>,
        arity: usize,
    },
    DuplicateSite {
        site: OwnedExprSiteV1,
    },
}

pub(crate) fn issue_ordinary_new_claims_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    excluded_dynamic_batch_slot: Option<u32>,
    instance_constructors: &VerifiedInstanceConstructorSemanticBatchV1,
) -> Result<Box<[OrdinaryNewAdmissionClaimV1]>, OrdinaryNewCoSealIssueV1> {
    let mut claims = Vec::new();
    for declaration in batch.declarations() {
        let owner = declaration.owner();
        let batch_slot = declaration.batch_slot();
        if selected.role_for_batch_slot(batch_slot).is_none() {
            continue;
        }
        if excluded_dynamic_batch_slot == Some(batch_slot) {
            continue;
        }
        for effect in declaration
            .body_shape()
            .effects()
            .iter()
            .filter(|effect| effect.kind == BodyEffectKindV1::Allocation)
        {
            if !is_direct_local_initializer(effect.site.node().segments()) {
                continue;
            }
            let site = OwnedExprSiteV1::new(owner, effect.site.clone());
            let (class, arity) = batch
                .with_lowering_input(batch_slot, |input| {
                    let located = input.source().expr_at(&site).map_err(|_| {
                        OrdinaryNewCoSealIssueV1::SourceNavigation { site: site.clone() }
                    })?;
                    match located.node() {
                        ASTNode::New {
                            class, arguments, ..
                        } => Ok((class.clone().into_boxed_str(), arguments.len())),
                        _ => Err(OrdinaryNewCoSealIssueV1::AllocationSiteNotNew {
                            site: site.clone(),
                        }),
                    }
                })
                .map_err(|_| OrdinaryNewCoSealIssueV1::BatchLoan)??;

            let Some(box_source) = batch
                .ordinary_box_coverage()
                .row_for(class.as_ref())
                .map_err(|_| OrdinaryNewCoSealIssueV1::OrdinaryBoxCoverageDuplicate {
                    site: site.clone(),
                    class: class.clone(),
                })?
            else {
                return Err(OrdinaryNewCoSealIssueV1::OrdinaryBoxCoverageMissing { site, class });
            };
            let final_box_ordinal =
                u32::try_from(box_source.final_box_ordinal()).map_err(|_| {
                    OrdinaryNewCoSealIssueV1::ConstructorSourceOrdinalOverflow {
                        site: site.clone(),
                        class: class.clone(),
                    }
                })?;
            let constructor = match instance_constructors
                .birth_for(final_box_ordinal, arity)
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
                    let target = CanonicalGlobalTargetV1::new_static_box_method(
                        row.box_name().into(),
                        "birth".into(),
                        row.source_arity(),
                    )
                    .map_err(|error| {
                        OrdinaryNewCoSealIssueV1::BirthTargetInvalid {
                            site: site.clone(),
                            class: class.clone(),
                            arity,
                            error,
                        }
                    })?;
                    OrdinaryNewConstructorDispositionV1::Birth(VerifiedOrdinaryNewBirthRecipeV1 {
                        source_id: row.source_id().clone(),
                        target,
                        abi,
                    })
                }
                None => no_birth_constructor_disposition(&site, &class, arity)?,
            };
            if claims
                .iter()
                .any(|claim: &OrdinaryNewAdmissionClaimV1| claim.site == site)
            {
                return Err(OrdinaryNewCoSealIssueV1::DuplicateSite { site });
            }
            claims.push(OrdinaryNewAdmissionClaimV1 {
                site,
                class,
                arity,
                constructor,
            });
        }
    }
    Ok(claims.into_boxed_slice())
}

fn is_direct_local_initializer(segments: &[SourcePathSegmentV1]) -> bool {
    matches!(
        segments,
        [
            SourcePathSegmentV1::Body(_),
            SourcePathSegmentV1::Initializer(_)
        ]
    )
}

fn no_birth_constructor_disposition(
    site: &OwnedExprSiteV1,
    class: &str,
    arity: usize,
) -> Result<OrdinaryNewConstructorDispositionV1, OrdinaryNewCoSealIssueV1> {
    if arity == 0 {
        return Ok(OrdinaryNewConstructorDispositionV1::NoBirthZero);
    }
    Err(OrdinaryNewCoSealIssueV1::BirthConstructorMissing {
        site: site.clone(),
        class: class.into(),
        arity,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::resolved_semantics::{
        FunctionOwnerIssuerV1, SourceNodeSiteV1, SourcePathSegmentV1,
    };

    fn test_site() -> OwnedExprSiteV1 {
        let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().expect("owner issuer");
        let owner = issuer.issue().expect("owner");
        OwnedExprSiteV1::new(
            owner,
            SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::Initializer(0),
            ])),
        )
    }

    fn claim(site: OwnedExprSiteV1, arity: usize) -> OrdinaryNewAdmissionClaimV1 {
        OrdinaryNewAdmissionClaimV1 {
            site,
            class: "Page".into(),
            arity,
            constructor: OrdinaryNewConstructorDispositionV1::NoBirthZero,
        }
    }

    #[test]
    fn exact_claim_is_consumed_once() {
        let site = test_site();
        let ledger = OrdinaryNewClaimLedgerV1::issue(
            vec![claim(site.clone(), 0)].into_boxed_slice(),
            vec!["Page".into()].into_boxed_slice(),
        );

        let taken = ledger
            .try_take(&site, "Page", 0)
            .expect("exact claim should be available")
            .expect("ordinary class should return a claim");
        assert_eq!(taken.class(), "Page");
        assert_eq!(taken.arity(), 0);
        assert!(ledger.is_empty());
        assert_eq!(
            ledger.try_take(&site, "Page", 0),
            Err(OrdinaryNewClaimTakeErrorV1::Unavailable)
        );
    }

    #[test]
    fn nonordinary_class_does_not_consume_claim() {
        let site = test_site();
        let ledger = OrdinaryNewClaimLedgerV1::issue(
            vec![claim(site.clone(), 0)].into_boxed_slice(),
            vec!["Page".into()].into_boxed_slice(),
        );

        assert_eq!(ledger.try_take(&site, "Plugin", 0), Ok(None));
        assert!(!ledger.is_empty());
        assert!(ledger.try_take(&site, "Page", 0).unwrap().is_some());
        assert!(ledger.is_empty());
    }

    #[test]
    fn mismatched_shape_preserves_claim_for_the_correct_consumer() {
        let site = test_site();
        let ledger = OrdinaryNewClaimLedgerV1::issue(
            vec![claim(site.clone(), 0)].into_boxed_slice(),
            vec!["Page".into()].into_boxed_slice(),
        );

        assert_eq!(
            ledger.try_take(&site, "Page", 1),
            Err(OrdinaryNewClaimTakeErrorV1::Mismatch)
        );
        assert!(!ledger.is_empty());
        assert!(ledger.try_take(&site, "Page", 0).unwrap().is_some());
        assert!(ledger.is_empty());
    }

    #[test]
    fn ordinary_new_recipe_preserves_exact_birth_source() {
        let source_id = crate::parser::ConstructorSourceIdV1::test_new(7);
        let abi = InstanceConstructorAbiV1::issue(2).expect("constructor ABI");
        let target =
            CanonicalGlobalTargetV1::new_static_box_method("Page".into(), "birth".into(), 2)
                .expect("birth target");
        let recipe = VerifiedOrdinaryNewBirthRecipeV1 {
            source_id: source_id.clone(),
            target: target.clone(),
            abi,
        };
        let OrdinaryNewConstructorDispositionV1::Birth(recipe) =
            OrdinaryNewConstructorDispositionV1::Birth(recipe)
        else {
            unreachable!()
        };
        assert!(recipe.source_id().same_as(&source_id));
        assert_eq!(recipe.abi(), abi);
        assert_eq!(recipe.target(), target);
    }

    #[test]
    fn ordinary_new_constructor_disposition_accepts_zero_arity_without_birth() {
        let site = test_site();
        let disposition = no_birth_constructor_disposition(&site, "Page", 0)
            .expect("zero-arity allocation may omit birth");
        assert!(matches!(
            disposition,
            OrdinaryNewConstructorDispositionV1::NoBirthZero
        ));
    }

    #[test]
    fn ordinary_new_constructor_disposition_rejects_nonzero_without_birth() {
        let site = test_site();
        let error = no_birth_constructor_disposition(&site, "Page", 1)
            .expect_err("nonzero allocation requires an exact birth row");
        assert!(matches!(
            error,
            OrdinaryNewCoSealIssueV1::BirthConstructorMissing {
                class,
                arity: 1,
                ..
            } if class.as_ref() == "Page"
        ));
    }
}
