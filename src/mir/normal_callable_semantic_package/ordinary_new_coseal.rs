//! Source-bound admission claims for the first Raw ordinary-`New` cohort.
//!
//! The claim is issued from the parser's final ordinary-box coverage and the
//! resolver's exact direct-body allocation site.  Builder headers, symbol
//! scans, and post-lowering target inference are deliberately outside this
//! owner.

use std::{cell::RefCell, collections::BTreeMap};

use super::selected_mapping::VerifiedSelectedCallableBatchMapV1;
use crate::ast::ASTNode;
use crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1;
use crate::mir::resolved_semantics::{
    BodyEffectKindV1, OwnedExprSiteV1, SourceExprSiteV1, SourcePathSegmentV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OrdinaryNewAdmissionClaimV1 {
    site: OwnedExprSiteV1,
    class: Box<str>,
    arity: usize,
    birth: Option<Box<str>>,
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

    pub(crate) fn birth(&self) -> Option<&str> {
        self.birth.as_deref()
    }

    pub(crate) const fn arity(&self) -> usize {
        self.arity
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
    BoxDeclarationMissing {
        site: OwnedExprSiteV1,
        class: Box<str>,
    },
    BoxDeclarationNotOrdinary {
        site: OwnedExprSiteV1,
        class: Box<str>,
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

            if !batch.ordinary_box_coverage().contains_box(class.as_ref()) {
                return Err(OrdinaryNewCoSealIssueV1::OrdinaryBoxCoverageMissing { site, class });
            }
            let birth = source_birth_target(batch.source_ast(), &site, &class, arity)?;
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
                birth,
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

fn source_birth_target(
    source_ast: &ASTNode,
    site: &OwnedExprSiteV1,
    class: &str,
    arity: usize,
) -> Result<Option<Box<str>>, OrdinaryNewCoSealIssueV1> {
    let ASTNode::Program { statements, .. } = source_ast else {
        return Err(OrdinaryNewCoSealIssueV1::BoxDeclarationMissing {
            site: site.clone(),
            class: class.into(),
        });
    };
    let Some(ASTNode::BoxDeclaration {
        constructors,
        is_interface,
        is_record,
        is_static,
        name,
        ..
    }) = statements.iter().find(|statement| {
        matches!(statement, ASTNode::BoxDeclaration { name: declaration, .. } if declaration == class)
    })
    else {
        return Err(OrdinaryNewCoSealIssueV1::BoxDeclarationMissing {
            site: site.clone(),
            class: class.into(),
        });
    };
    if *is_interface || *is_record || *is_static || name != class {
        return Err(OrdinaryNewCoSealIssueV1::BoxDeclarationNotOrdinary {
            site: site.clone(),
            class: class.into(),
        });
    }
    let key = format!("birth/{arity}");
    if constructors.contains_key(&key) {
        return Ok(Some(format!("{class}.{key}").into_boxed_str()));
    }
    if arity == 0 {
        return Ok(None);
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
            birth: None,
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
    fn source_birth_projection_is_exact_and_missing_arity_rejects() {
        let source =
            crate::parser::NyashParser::parse_from_string("box Page { birth() { return 0 } }")
                .expect("ordinary box source");
        let site = test_site();
        assert_eq!(
            source_birth_target(&source, &site, "Page", 0)
                .expect("zero-arity birth projection")
                .as_deref(),
            Some("Page.birth/0")
        );
        assert!(matches!(
            source_birth_target(&source, &site, "Page", 1),
            Err(OrdinaryNewCoSealIssueV1::BirthConstructorMissing {
                class,
                arity: 1,
                ..
            }) if class.as_ref() == "Page"
        ));
    }
}
