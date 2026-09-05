//! Source-bound admission claims for the first Raw ordinary-`New` cohort.
//!
//! The claim is issued from the parser's final ordinary-box coverage and the
//! resolver's exact direct-local initializer relation. Builder headers, symbol
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
    BindingKindV1, BindingRefV1, OwnedExprSiteV1, SourceBindingSiteV1,
    SourceExprSiteV1, SourcePathSegmentV1,
};
use hakorune_mir_defs::{CanonicalSameModuleCallableKeyV1, SameModuleCallableNamespaceV1};
use crate::mir::resolved_semantics::DeclaredInstanceCallSemanticEffectV1;
use crate::mir::{Effect, EffectMask};
use crate::mir::resolved_semantics::home_new_prefix::{
    issue_new_home_prefixes_v1, CallerNewHomePrefixV1, HomePrefixUnavailableV1,
};

#[path = "ordinary_new_local_commit.rs"]
mod local_commit;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedOrdinaryNewBirthRecipeV1 {
    source_id: crate::parser::ConstructorSourceIdV1,
    target: CanonicalSameModuleCallableKeyV1,
    effect: DeclaredInstanceCallSemanticEffectV1,
    abi: InstanceConstructorAbiV1,
}

impl VerifiedOrdinaryNewBirthRecipeV1 {
    pub(crate) fn source_id(&self) -> &crate::parser::ConstructorSourceIdV1 {
        &self.source_id
    }

    pub(crate) fn target(self) -> CanonicalSameModuleCallableKeyV1 {
        self.target
    }

    pub(crate) const fn abi(&self) -> InstanceConstructorAbiV1 {
        self.abi
    }

    /// Explicit conservative physical policy, not an effect inferred from MIR
    /// or source event counts. Completion and FieldSet Fault remain separate.
    pub(crate) fn physical_effect_mask(&self) -> EffectMask {
        EffectMask::MUT.union(EffectMask::IO).union(EffectMask::WRITE)
            .add(Effect::Control).add(Effect::P2P).add(Effect::FFI)
            .add(Effect::Panic).add(Effect::Alloc).add(Effect::Global)
            .add(Effect::Async).add(Effect::Unsafe).add(Effect::Debug)
            .add(Effect::Barrier)
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
    destination: BindingRefV1,
    declaration: SourceBindingSiteV1,
    home_prefix: Result<CallerNewHomePrefixV1, HomePrefixUnavailableV1>,
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
    local_commits: RefCell<BTreeMap<OwnedExprSiteV1, local_commit::NewLocalCommitV1>>,
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
            local_commits: RefCell::new(BTreeMap::new()),
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
        let mut commits = self.local_commits.borrow_mut();
        if let Ok(prefix) = &claim.home_prefix {
            if prefix.destination() != claim.destination || prefix.required_unwind() != site
                || prefix.prior_homes().iter().any(|binding|
                    !commits.values().any(|row| row.installs(*binding)))
            {
                return Err(OrdinaryNewClaimTakeErrorV1::Mismatch);
            }
        }
        commits.insert(
            site.clone(),
            local_commit::NewLocalCommitV1::pending(claim.destination, claim.declaration.clone(), claim.home_prefix.clone()),
        );
        Ok(Some(
            claims
                .remove(site)
                .expect("claim remained present after the checked lookup"),
        ))
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.claims.borrow().is_empty()
            && self.local_commits.borrow().values().all(|row| row.is_complete())
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

    pub(crate) fn home_prefix(&self) -> Result<&CallerNewHomePrefixV1, &HomePrefixUnavailableV1> {
        self.home_prefix.as_ref()
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
    InitializerBindingMismatch {
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
    },
    BirthCompletionNotUnit { site: OwnedExprSiteV1, class: Box<str> },
    BirthEffectUnsupported { site: OwnedExprSiteV1, class: Box<str> },
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
    app_main_batch_slot: Option<u32>,
    excluded_dynamic_batch_slot: Option<u32>,
    instance_constructors: &VerifiedInstanceConstructorSemanticBatchV1,
) -> Result<Box<[OrdinaryNewAdmissionClaimV1]>, OrdinaryNewCoSealIssueV1> {
    let mut claims = Vec::new();
    for declaration in batch.declarations() {
        let owner = declaration.owner();
        let batch_slot = declaration.batch_slot();
        // App Main is intentionally omitted from the generic selected-role
        // map; its exact parser identity is still admitted through the
        // source-backed batch slot supplied by the package issuer.
        let is_app_main = app_main_batch_slot == Some(batch_slot);
        if selected.role_for_batch_slot(batch_slot).is_none() && !is_app_main {
            continue;
        }
        if excluded_dynamic_batch_slot == Some(batch_slot) {
            continue;
        }
        // One source loan covers both initializer membership and binding
        // validation. Its order is not a Home availability/execution timeline.
        let (candidates, mut home_prefixes) = batch
            .with_lowering_input(batch_slot, |input| -> Result<_, OrdinaryNewCoSealIssueV1> {
                let function = input.function();
                let mut candidates = Vec::new();
                for initializer in function.expression_source().initializers() {
                    let Some(initializer_site) = initializer.initializer_site() else {
                        continue;
                    };
                    if !is_direct_local_initializer(initializer_site.node().segments()) {
                        continue;
                    }
                    let site = OwnedExprSiteV1::new(owner, initializer_site.clone());
                    let located = input.source().expr_at(&site).map_err(|_| {
                        OrdinaryNewCoSealIssueV1::SourceNavigation { site: site.clone() }
                    })?;
                    let ASTNode::New { class, arguments, .. } = located.node() else {
                        continue;
                    };
                    if initializer.binding().owner() != owner
                        || function.declaration_binding(initializer.declaration_site())
                            != Some(initializer.binding())
                        || !matches!(function.binding(initializer.binding()).map(|row| row.kind()),
                            Some(BindingKindV1::Local { .. }))
                    {
                        return Err(OrdinaryNewCoSealIssueV1::InitializerBindingMismatch { site });
                    }
                    candidates.push((site, class.clone().into_boxed_str(), arguments.len(),
                        initializer.binding(), initializer.declaration_site().clone()));
                }
                let selected = candidates.iter().filter(|(_, class, _, _, _)|
                    matches!(batch.ordinary_box_coverage().row_for(class.as_ref()), Ok(Some(_))))
                    .map(|(site, _, _, binding, _)| (site.clone(), *binding)).collect();
                let home_prefixes = issue_new_home_prefixes_v1(input, &selected);
                Ok((candidates, home_prefixes))
            })
            .map_err(|_| OrdinaryNewCoSealIssueV1::BatchLoan)??;
        for (site, class, arity, destination, declaration) in candidates {
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
                    continue;
                }
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
                    let target = row.published_birth_key().filter(|key| {
                        key.namespace() == SameModuleCallableNamespaceV1::BirthConstructor
                            && key.owner() == row.box_name()
                            && key.arity() == row.source_arity()
                    }).ok_or_else(|| OrdinaryNewCoSealIssueV1::BirthTargetInvalid {
                            site: site.clone(),
                            class: class.clone(),
                            arity,
                    })?.clone();
                    row.birth_completion().filter(|completion| {
                        row.forest().roots() == [completion.owner()]
                            && !completion.returns_value()
                    }).ok_or_else(|| OrdinaryNewCoSealIssueV1::BirthCompletionNotUnit {
                        site: site.clone(), class: class.clone(),
                    })?;
                    let effect = row.birth_effect().filter(|effect| {
                        *effect == DeclaredInstanceCallSemanticEffectV1::OpaqueObservable
                    }).ok_or_else(|| OrdinaryNewCoSealIssueV1::BirthEffectUnsupported {
                        site: site.clone(), class: class.clone(),
                    })?;
                    OrdinaryNewConstructorDispositionV1::Birth(VerifiedOrdinaryNewBirthRecipeV1 {
                        source_id: row.source_id().clone(),
                        target,
                        effect,
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
            let home_prefix = home_prefixes.remove(&site).ok_or_else(||
                OrdinaryNewCoSealIssueV1::InitializerBindingMismatch { site: site.clone() })?;
            claims.push(OrdinaryNewAdmissionClaimV1 {
                site,
                class,
                arity,
                constructor,
                destination,
                declaration,
                home_prefix,
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
        let destination = BindingRefV1::new(site.owner(), hakorune_mir_core::BindingId::new(0));
        let declaration = SourceBindingSiteV1::Local {
            statement: crate::mir::resolved_semantics::SourceStmtSiteV1::from_node(
                crate::mir::resolved_semantics::SourceNodeSiteV1::from_segments(vec![SourcePathSegmentV1::Body(0)])),
            ordinal: 0,
        };
        OrdinaryNewAdmissionClaimV1 {
            site,
            class: "Page".into(),
            arity,
            constructor: OrdinaryNewConstructorDispositionV1::NoBirthZero,
            destination,
            declaration,
            home_prefix: Err(HomePrefixUnavailableV1::SourceMismatch),
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
        assert!(!ledger.is_empty(), "target take is not local completion");
        let SourceBindingSiteV1::Local { statement, ordinal } = &taken.declaration else {
            panic!("local claim");
        };
        ledger.complete_new_expression(&site, "Page", crate::mir::ValueId(0)).unwrap();
        assert!(!ledger.is_empty(), "whole New is not local installation");
        ledger.complete_local_installation(site.owner(), statement.node(), &[
            (taken.destination, *ordinal, crate::mir::ValueId(0), crate::mir::ValueId(1)),
        ]).unwrap();
        assert!(ledger.is_empty(), "ValueId zero is a valid completed initializer");
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
        assert!(!ledger.is_empty(), "target take is not local completion");
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
        assert!(!ledger.is_empty(), "target take is not local completion");
    }

    #[test]
    fn ordinary_new_recipe_preserves_exact_birth_source() {
        let source_id = crate::parser::ConstructorSourceIdV1::test_new(7);
        let abi = InstanceConstructorAbiV1::issue(2).expect("constructor ABI");
        let target = CanonicalSameModuleCallableKeyV1::birth_constructor("Page", 2);
        let recipe = VerifiedOrdinaryNewBirthRecipeV1 {
            source_id: source_id.clone(),
            target: target.clone(),
            effect: DeclaredInstanceCallSemanticEffectV1::OpaqueObservable,
            abi,
        };
        let OrdinaryNewConstructorDispositionV1::Birth(recipe) =
            OrdinaryNewConstructorDispositionV1::Birth(recipe)
        else {
            unreachable!()
        };
        assert!(recipe.source_id().same_as(&source_id));
        assert_eq!(recipe.abi(), abi);
        assert_eq!(recipe.effect, DeclaredInstanceCallSemanticEffectV1::OpaqueObservable);
        assert_eq!(
            recipe.physical_effect_mask(),
            crate::mir::canonical_direct_call::materialize_direct_call_effect_v1(
                crate::mir::canonical_direct_call_contract::VerifiedDirectCallEffectV1::ConservativeBarrier,
            ),
            "physical barrier bit policy must not drift; target issuers remain separate"
        );
        assert_eq!(recipe.target(), target);
    }

    #[test]
    fn ordinary_new_recipe_target_is_selected_before_consumption() {
        let source_id = crate::parser::ConstructorSourceIdV1::test_new(8);
        let abi = InstanceConstructorAbiV1::issue(1).expect("constructor ABI");
        let target = CanonicalSameModuleCallableKeyV1::birth_constructor("Pair", 1);
        let recipe = VerifiedOrdinaryNewBirthRecipeV1 {
            source_id,
            target: target.clone(),
            effect: DeclaredInstanceCallSemanticEffectV1::OpaqueObservable,
            abi,
        };

        assert_eq!(&recipe.target, &target);
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

    #[test]
    fn ordinary_new_local_commit_rejects_drift_without_consuming_pending_installation() {
        use crate::mir::ValueId;
        let site = test_site();
        let claim = claim(site.clone(), 0);
        let destination = claim.destination;
        let SourceBindingSiteV1::Local { statement, ordinal } = claim.declaration.clone() else {
            panic!("local claim");
        };
        let ledger = OrdinaryNewClaimLedgerV1::issue(
            vec![claim].into_boxed_slice(), vec!["Page".into()].into_boxed_slice());
        let good = (destination, ordinal, ValueId(8), ValueId(9));
        assert!(ledger.complete_local_installation(site.owner(), statement.node(), &[good])
            .unwrap_err().contains("local-before-target-take"));
        assert!(ledger.complete_new_expression(&site, "Page", ValueId(8))
            .unwrap_err().contains("expression-without-target-take"));
        ledger.try_take(&site, "Page", 0).unwrap().unwrap();
        assert!(ledger.complete_local_installation(site.owner(), statement.node(), &[good])
            .unwrap_err().contains("local-initializer-mismatch"));
        ledger.complete_new_expression(&site, "Page", ValueId(8)).unwrap();
        assert!(ledger.complete_new_expression(&site, "Page", ValueId(8))
            .unwrap_err().contains("duplicate-expression-completion"));
        for bad in [
            (destination, ordinal + 1, ValueId(8), ValueId(9)),
            (BindingRefV1::new(site.owner(), hakorune_mir_core::BindingId::new(1)),
                ordinal, ValueId(8), ValueId(9)),
            (destination, ordinal, ValueId(7), ValueId(9)),
            (destination, ordinal, ValueId(8), ValueId(8)),
        ] {
            assert!(ledger.complete_local_installation(site.owner(), statement.node(), &[bad]).is_err());
            assert!(!ledger.is_empty());
        }
        let foreign = test_site().owner();
        assert!(ledger.complete_local_installation(foreign, statement.node(), &[good])
            .unwrap_err().contains("foreign-or-duplicate-local"));
        assert!(ledger.complete_local_installation(site.owner(), statement.node(), &[good, good])
            .unwrap_err().contains("foreign-or-duplicate-local"));
        let wrong_statement = SourceNodeSiteV1::from_segments(vec![SourcePathSegmentV1::Body(1)]);
        ledger.complete_local_installation(site.owner(), &wrong_statement, &[good]).unwrap();
        assert!(!ledger.is_empty(), "unrelated local cannot discharge selected obligation");
        ledger.complete_local_installation(site.owner(), statement.node(), &[good]).unwrap();
        assert!(ledger.is_empty());
        assert!(ledger.complete_local_installation(site.owner(), statement.node(), &[good])
            .unwrap_err().contains("duplicate-local-installation"));
    }
}
