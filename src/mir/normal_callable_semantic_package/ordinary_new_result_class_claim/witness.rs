//! Immutable source derivations shared by the existing result solver.
//! This graph records source identity; it issues no acquired Home or runtime tag.
use super::ResultValueOriginV1;
use crate::mir::resolved_semantics::{BindingRefV1, OwnedExprSiteV1};
use hakorune_mir_defs::CanonicalSameModuleCallableKeyV1;
use std::rc::Rc;

#[derive(Debug)]
pub(crate) struct ResultFormalSubstitutionV1 {
    pub(crate) argument_site: OwnedExprSiteV1,
    pub(crate) binding: BindingRefV1,
    pub(crate) callee_ordinal: u32,
    pub(crate) caller_ordinal: u32,
}

#[derive(Debug)]
pub(crate) enum ResultWitnessStepV1 {
    NullLiteral,
    FreshConstruction,
    Formal {
        binding: BindingRefV1,
        ordinal: u32,
    },
    Call {
        site: OwnedExprSiteV1,
        key: CanonicalSameModuleCallableKeyV1,
        callee: Rc<ResultOriginWitnessV1>,
        substitution: Option<ResultFormalSubstitutionV1>,
    },
}

#[derive(Debug)]
pub(crate) struct ResultOriginWitnessV1 {
    pub(super) site: OwnedExprSiteV1,
    pub(super) origin: ResultValueOriginV1,
    pub(super) step: ResultWitnessStepV1,
}
impl ResultOriginWitnessV1 {
    pub(crate) fn site(&self) -> &OwnedExprSiteV1 {
        &self.site
    }
    pub(crate) fn origin(&self) -> &ResultValueOriginV1 {
        &self.origin
    }
    pub(crate) fn step(&self) -> &ResultWitnessStepV1 {
        &self.step
    }
    pub(super) fn formal_ordinal(&self) -> Option<u32> {
        match &self.step {
            ResultWitnessStepV1::Formal { ordinal, .. } => Some(*ordinal),
            ResultWitnessStepV1::Call { substitution, .. } => {
                substitution.as_ref().map(|row| row.caller_ordinal)
            }
            _ => None,
        }
    }
}
