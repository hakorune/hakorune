//! Claim-backed static observations from the sole incoming batch walk.
//! Missing caller/index authority stays visible; this is not Static ABI coverage.
use super::super::super::freeze;
use super::*;
use crate::mir::normal_callable_semantic_package::qualified_static_call_claim::{
    caller_key_for_function, incoming_source::StaticIncomingSourceV1,
    QualifiedStaticCallClaimIndexV1,
};
use crate::mir::resolved_semantics::{
    ResolvedMethodCallReceiverSourceV1, VerifiedResolvedMethodCallSourceV1,
};
use std::rc::Rc;

pub(super) type StaticIncomingObservationV1 = Result<Rc<StaticIncomingSourceV1>, String>;

pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call)
struct StaticIncomingContextV1
<'a> {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) claims:
        &'a QualifiedStaticCallClaimIndexV1,
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) arguments:
        &'a BTreeMap<
            (OwnedExprSiteV1, u32),
            super::super::super::borrowed_static_argument::StaticArgumentSourceV1,
        >,
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) main:
        Option<&'a super::super::super::BorrowedAppMainSourceLoanV1<'a>>,
}

impl StaticIncomingContextV1<'_> {
    pub(super) fn observe(
        &self,
        caller_batch_slot: u32,
        site: &OwnedExprSiteV1,
        source: &VerifiedResolvedMethodCallSourceV1,
        selected: &super::super::super::VerifiedSelectedCallableBatchMapV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        operand_retained: Option<&Rc<StaticIncomingSourceV1>>,
    ) -> Result<Option<StaticIncomingObservationV1>, BorrowedIncomingDraftErrorV1> {
        let mut existing = self
            .arguments
            .range((site.clone(), 0)..=(site.clone(), u32::MAX))
            .map(|(_, row)| row);
        let retained = existing.next().map(|row| row.retained_call_source());
        if existing
            .any(|row| retained.is_none_or(|old| !Rc::ptr_eq(old, row.retained_call_source())))
        {
            return Err(BorrowedIncomingDraftErrorV1::CallIdentity(site.clone()));
        }
        if retained
            .zip(operand_retained)
            .is_some_and(|(argument, operand)| !Rc::ptr_eq(argument, operand))
        {
            return Err(BorrowedIncomingDraftErrorV1::CallIdentity(site.clone()));
        }
        let retained = retained.or(operand_retained);
        if !matches!(
            source.receiver(),
            ResolvedMethodCallReceiverSourceV1::QualifiedUnbound
                | ResolvedMethodCallReceiverSourceV1::CurrentOwner
        ) {
            return if retained.is_some() {
                Err(BorrowedIncomingDraftErrorV1::CallIdentity(site.clone()))
            } else {
                Ok(None)
            };
        }
        let mut callers = contracts.iter().filter(|row| row.owner == site.owner());
        let contract = callers.next();
        let observation = (|| {
            let contract = contract.ok_or_else(|| freeze("borrowed-static/caller-contract"))?;
            if callers.next().is_some() {
                return Err(freeze("borrowed-static/duplicate-caller-contract"));
            }
            if contract.batch_slot != caller_batch_slot {
                return Err(freeze("borrowed-static/caller-slot-identity"));
            }
            let main = self.main.filter(|main| main.matches_contract(contract));
            let key = caller_key_for_function(
                selected,
                contract.batch_slot,
                main.is_some(),
                main.map(|main| main.catalog_key()),
            )
            .ok_or_else(|| freeze("borrowed-static/caller-key-unavailable"))?;
            let loan = self
                .claims
                .incoming_source(&key, site, source, selected, contracts, self.main)?
                .ok_or_else(|| freeze("borrowed-static/claim-unavailable"))?;
            match retained {
                Some(retained) if loan.corroborates_retained(retained) => Ok(Rc::clone(retained)),
                Some(_) => Err(freeze("borrowed-static/retained-source-identity")),
                None => Ok(loan.retain()),
            }
        })();
        if retained.is_some() && observation.is_err() {
            return Err(BorrowedIncomingDraftErrorV1::CallIdentity(site.clone()));
        }
        Ok(Some(observation))
    }
}
