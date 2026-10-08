//! Complete original callee exits and ordered Facts edges for return handoff.
//! No input, caller Home, Normal projection or artifact capability is issued.
use super::super::lexical_instance_call::LexicalInstanceCallSourceTargetV1;
use super::super::{result_class_claim, OrdinaryNewClaimLedgerV1};
use crate::mir::resolved_semantics::home_new_prefix::{TerminalRelationV1, TerminalValueReturnV1};
use crate::mir::resolved_semantics::OwnedExprSiteV1;
use result_class_claim::{
    ObjectReturnCallQualificationV1, ResultOriginWitnessV1, ResultWitnessStepV1,
};
use std::collections::BTreeSet;
use std::rc::Rc;

type CalleeExits = Box<[(Rc<ResultOriginWitnessV1>, TerminalValueReturnV1)]>;

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn checked_completed_object_callee_exits_v1(
        &self,
        target: &LexicalInstanceCallSourceTargetV1,
        loan: &ObjectReturnCallQualificationV1,
    ) -> Result<Option<CalleeExits>, String> {
        self.check_original_callee_exits(target, Some(loan))
    }

    pub(in crate::mir::normal_callable_semantic_package) fn checked_completed_object_callee_target_exits_v1(
        &self,
        target: &LexicalInstanceCallSourceTargetV1,
    ) -> Result<Option<CalleeExits>, String> {
        self.check_original_callee_exits(target, None)
    }

    fn check_original_callee_exits(
        &self,
        target: &LexicalInstanceCallSourceTargetV1,
        loan: Option<&ObjectReturnCallQualificationV1>,
    ) -> Result<Option<CalleeExits>, String> {
        if let Some(loan) = loan {
            if loan.key() != target.target()
                || loan.call() != target.call_site()
                || self
                    .callable_result_classes
                    .object_return_qualification(loan.value())
                    .as_ref()
                    != Some(loan)
                || !target
                    .object_return_sources()
                    .is_some_and(|rows| rows.contains(loan))
            {
                return Err(freeze("callee-call-qualification"));
            }
        }
        let Some(completion) = self.completion_index.get(&target.callee_owner()) else {
            return Ok(None);
        };
        let completion = completion
            .as_ref()
            .map_err(|issue| format!("{}: {issue:?}", freeze("callee-completion")))?;
        if completion.owner() != target.callee_owner() {
            return Err(freeze("callee-completion-owner"));
        }
        if !completion.returns_value() || completion.implicit_body_end().is_some() {
            return Ok(None);
        }
        let outcomes = self
            .callable_result_classes
            .outcomes(target.target())
            .ok_or_else(|| freeze("callee-facts-missing"))?;
        let mut fact_sites = BTreeSet::new();
        for row in outcomes {
            if row.site().owner() != target.callee_owner()
                || !fact_sites.insert(row.site().clone())
                || row.witnesses().is_empty()
                || row
                    .witnesses()
                    .iter()
                    .any(|witness| witness.site() != row.site())
            {
                return Err(freeze("callee-facts-identity"));
            }
            let origins: BTreeSet<_> = row
                .witnesses()
                .iter()
                .map(|witness| witness.origin().clone())
                .collect();
            if &origins != row.alternatives() {
                return Err(freeze("callee-facts-alternatives"));
            }
        }
        let roots: Vec<_> = outcomes.iter().flat_map(|row| row.witnesses()).collect();
        if let Some(loan) = loan {
            if loan.witnesses().len() != roots.len() {
                return Err(freeze("callee-witness-coverage"));
            }
            for (caller, callee) in loan.witnesses().iter().zip(&roots) {
                let ResultWitnessStepV1::Call {
                    site,
                    key,
                    callee: child,
                    substitution,
                } = caller.step()
                else {
                    return Err(freeze("callee-witness-role"));
                };
                if caller.site() != loan.value()
                    || site != loan.call()
                    || key != loan.key()
                    || substitution.is_some()
                    || !Rc::ptr_eq(child, callee)
                    || caller.origin() != callee.origin()
                {
                    return Err(freeze("callee-witness-identity"));
                }
            }
        }
        let mut mapped = BTreeSet::new();
        let mut exits = BTreeSet::new();
        let mut terminals = Vec::new();
        let mut missing = false;
        for exit in completion.explicit_sites() {
            if !exits.insert(exit.clone()) {
                return Err(freeze("callee-exit-duplicate"));
            }
            match completion
                .cleanup()
                .root_flow()
                .and_then(|flow| flow.exit_row(exit))
            {
                Some(Err(issue)) => return Err(format!("{}: {issue:?}", freeze("callee-exit"))),
                None => missing = true,
                Some(Ok(_)) => {}
            }
            let Some(relation) = self.terminal_relation_for_owner_at(completion.owner(), exit)
            else {
                missing = true;
                continue;
            };
            if relation.owner() != completion.owner() {
                return Err(freeze("callee-return-identity"));
            }
            let TerminalRelationV1::Value(value) = relation else {
                missing = true;
                continue;
            };
            if value.owner() != completion.owner() || value.return_site() != exit {
                return Err(freeze("callee-return-identity"));
            }
            let site = OwnedExprSiteV1::new(value.owner(), value.value_site().clone());
            if !fact_sites.contains(&site) || !mapped.insert(site.clone()) {
                return Err(freeze("callee-value-coverage"));
            }
            terminals.push((site, value));
        }
        if missing {
            return Ok(None);
        }
        if mapped != fact_sites || exits.is_empty() {
            return Err(freeze("callee-return-coverage"));
        }
        let mut rows = Vec::new();
        for witness in roots {
            let (_, terminal) = terminals
                .iter()
                .find(|(site, _)| site == witness.site())
                .ok_or_else(|| freeze("callee-witness-terminal"))?;
            rows.push((Rc::clone(witness), (*terminal).clone()));
        }
        Ok(Some(rows.into_boxed_slice()))
    }
}
fn freeze(reason: &str) -> String {
    format!("[freeze:contract][ordinary-new/object-return/{reason}]")
}

#[cfg(test)]
#[path = "ordinary_new_return_callee_tests.rs"]
mod tests;
