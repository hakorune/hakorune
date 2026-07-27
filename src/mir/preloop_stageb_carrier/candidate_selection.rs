//! Consuming 0/1/many projection for the bounded Stage-B candidate inventory.
//!
//! Prepared rows and construction-only catalog identity never cross this
//! boundary. A single candidate is immediately paired with the exact boxed
//! declaration catalog that produced it.

use crate::mir::builder::VerifiedSameModuleCallableDeclarationCatalogV1;

use super::activation::{
    PreloopStageBCarrierActivationErrorV1, PreloopStageBCarrierActivationStageV1,
    RejectedPreloopStageBCarrierActivationPlanV1, VerifiedPreloopStageBCarrierActivationPlanV1,
};
use super::source_inventory::{
    PreloopStageBCandidateCardinalityV1, PreloopStageBCandidateIdentityV1,
    VerifiedPreloopStageBCandidateInventoryV1,
};

#[derive(Debug)]
pub(crate) struct VerifiedPreloopStageBNoCandidateV1 {
    declaration_catalog: Box<VerifiedSameModuleCallableDeclarationCatalogV1>,
    inventory: VerifiedPreloopStageBCandidateInventoryV1,
}

impl VerifiedPreloopStageBNoCandidateV1 {
    pub(crate) fn discard(self) {
        let Self {
            declaration_catalog,
            inventory,
        } = self;
        let _ = declaration_catalog;
        inventory.discard();
    }
}

#[derive(Debug)]
pub(crate) struct VerifiedPreloopStageBSelectedCandidateV1 {
    activation: VerifiedPreloopStageBCarrierActivationPlanV1,
}

impl VerifiedPreloopStageBSelectedCandidateV1 {
    pub(crate) fn into_activation(self) -> VerifiedPreloopStageBCarrierActivationPlanV1 {
        self.activation
    }
}

#[derive(Debug)]
pub(crate) struct VerifiedPreloopStageBAmbiguousCandidatesV1 {
    declaration_catalog: Box<VerifiedSameModuleCallableDeclarationCatalogV1>,
    inventory: VerifiedPreloopStageBCandidateInventoryV1,
}

impl VerifiedPreloopStageBAmbiguousCandidatesV1 {
    pub(crate) const fn candidate_count(&self) -> usize {
        self.inventory.candidate_count()
    }

    pub(crate) fn candidate_identities(
        &self,
    ) -> impl Iterator<Item = &PreloopStageBCandidateIdentityV1> {
        self.inventory.candidate_identities()
    }

    pub(crate) fn is_branded_by_exact_catalog(&self) -> bool {
        self.inventory
            .is_branded_by(self.declaration_catalog.as_ref())
    }

    pub(crate) fn discard(self) {}
}

#[derive(Debug)]
pub(crate) enum VerifiedPreloopStageBCandidateSelectionV1 {
    Zero(VerifiedPreloopStageBNoCandidateV1),
    One(VerifiedPreloopStageBSelectedCandidateV1),
    Many(VerifiedPreloopStageBAmbiguousCandidatesV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PreloopStageBCandidateSelectionErrorV1 {
    CatalogAllocationMismatch,
    Activation {
        stage: PreloopStageBCarrierActivationStageV1,
        cause: PreloopStageBCarrierActivationErrorV1,
    },
}

#[derive(Debug)]
enum RetainedPreloopStageBCandidateSelectionOwnerV1 {
    CatalogMismatch {
        declaration_catalog: Box<VerifiedSameModuleCallableDeclarationCatalogV1>,
        inventory: VerifiedPreloopStageBCandidateInventoryV1,
    },
    Activation(RejectedPreloopStageBCarrierActivationPlanV1),
}

#[derive(Debug)]
pub(crate) struct RejectedPreloopStageBCandidateSelectionV1 {
    owner: RetainedPreloopStageBCandidateSelectionOwnerV1,
    cause: PreloopStageBCandidateSelectionErrorV1,
}

impl RejectedPreloopStageBCandidateSelectionV1 {
    pub(crate) const fn cause(&self) -> &PreloopStageBCandidateSelectionErrorV1 {
        &self.cause
    }

    pub(crate) fn discard(self) {
        match self.owner {
            RetainedPreloopStageBCandidateSelectionOwnerV1::CatalogMismatch {
                declaration_catalog,
                inventory,
            } => {
                let _ = declaration_catalog;
                inventory.discard();
            }
            RetainedPreloopStageBCandidateSelectionOwnerV1::Activation(rejected) => {
                rejected.discard();
            }
        }
    }
}

pub(crate) fn seal_preloop_stageb_candidate_selection_v1(
    declaration_catalog: Box<VerifiedSameModuleCallableDeclarationCatalogV1>,
    inventory: VerifiedPreloopStageBCandidateInventoryV1,
) -> Result<VerifiedPreloopStageBCandidateSelectionV1, RejectedPreloopStageBCandidateSelectionV1> {
    if !inventory.is_branded_by(declaration_catalog.as_ref()) {
        return Err(RejectedPreloopStageBCandidateSelectionV1 {
            owner: RetainedPreloopStageBCandidateSelectionOwnerV1::CatalogMismatch {
                declaration_catalog,
                inventory,
            },
            cause: PreloopStageBCandidateSelectionErrorV1::CatalogAllocationMismatch,
        });
    }

    match inventory.classify() {
        PreloopStageBCandidateCardinalityV1::Zero(inventory) => {
            let no_candidate = VerifiedPreloopStageBNoCandidateV1 {
                declaration_catalog,
                inventory,
            };
            Ok(VerifiedPreloopStageBCandidateSelectionV1::Zero(
                no_candidate,
            ))
        }
        PreloopStageBCandidateCardinalityV1::One { identity: _, rows } => {
            match VerifiedPreloopStageBCarrierActivationPlanV1::seal(declaration_catalog, rows) {
                Ok(activation) => Ok(VerifiedPreloopStageBCandidateSelectionV1::One(
                    VerifiedPreloopStageBSelectedCandidateV1 { activation },
                )),
                Err(rejected) => {
                    let cause = PreloopStageBCandidateSelectionErrorV1::Activation {
                        stage: rejected.stage(),
                        cause: rejected.cause().clone(),
                    };
                    Err(RejectedPreloopStageBCandidateSelectionV1 {
                        owner: RetainedPreloopStageBCandidateSelectionOwnerV1::Activation(rejected),
                        cause,
                    })
                }
            }
        }
        PreloopStageBCandidateCardinalityV1::Many(inventory) => {
            Ok(VerifiedPreloopStageBCandidateSelectionV1::Many(
                VerifiedPreloopStageBAmbiguousCandidatesV1 {
                    declaration_catalog,
                    inventory,
                },
            ))
        }
    }
}
