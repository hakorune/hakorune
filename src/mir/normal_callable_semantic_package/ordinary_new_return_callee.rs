//! Complete original callee exits and ordered Facts edges for return handoff.
//! No input, caller Home, Normal projection or artifact capability is issued.
use super::super::lexical_instance_call::LexicalInstanceCallSourceTargetV1;
use super::super::{result_class_claim, OrdinaryNewClaimLedgerV1};
use crate::mir::resolved_semantics::home_new_prefix::{TerminalRelationV1, TerminalValueReturnV1};
use crate::mir::resolved_semantics::OwnedExprSiteV1;
use result_class_claim::{ObjectReturnCallQualificationV1, ResultOriginWitnessV1};
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
        let roots = self
            .callable_result_classes
            .checked_original_source_roots_v1(target.target(), target.callee_owner())
            .map_err(freeze)?;
        let fact_sites: BTreeSet<_> = roots.iter().map(|row| row.site().clone()).collect();
        if let Some(loan) = loan {
            self.callable_result_classes
                .check_original_call_root_coverage_v1(loan, &roots)
                .map_err(freeze)?;
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
