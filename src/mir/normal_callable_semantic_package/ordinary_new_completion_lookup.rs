//! Borrow one already-issued Completion by its exact callable owner.

use super::lexical_instance_call::freeze;
use super::{
    OrdinaryNewClaimLedgerV1, OrdinaryNewTrivialArgumentKindV1, OrdinaryNewTrivialArgumentV1,
};
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;
use crate::mir::resolved_semantics::home_new_prefix::{
    CheckedExpressionEvaluationV1, HomeExpressionPlanV1, MapHomeObservation,
    NamedArrayHomeObservationV1, TerminalRelationV1, TerminalReturnedSourceV1,
};
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, SourceExprSiteV1};
use std::rc::Rc;

impl OrdinaryNewClaimLedgerV1 {
    /// Borrow canonical bodies from all retained owners. `false` is a consumer's
    /// incomplete source-use closure, never permission to skip an invalid body.
    pub(in crate::mir) fn visit_expression_evaluations_v1(
        &self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
        visitor: &mut impl FnMut(&CheckedExpressionEvaluationV1) -> Result<bool, String>,
    ) -> Result<bool, String> {
        let mut seen = std::collections::BTreeSet::new();
        let mut lend = |evaluation: &CheckedExpressionEvaluationV1| {
            if evaluation.source().owner() != owner {
                return Err(freeze("static-input/qualified-use-identity"));
            }
            if seen.insert(evaluation as *const CheckedExpressionEvaluationV1) {
                visitor(evaluation)
            } else {
                Ok(true)
            }
        };
        fn plan(
            original: &HomeExpressionPlanV1,
            lend: &mut impl FnMut(&CheckedExpressionEvaluationV1) -> Result<bool, String>,
        ) -> Result<bool, String> {
            if let Some(evaluation) = original.evaluation() {
                if !lend(evaluation)? {
                    return Ok(false);
                }
            }
            if let Some(product) = original.loop_recipe_v2() {
                if !lend(product.source.condition.evaluation())? {
                    return Ok(false);
                }
                for body in &product.source.body_expressions {
                    if !plan(body, lend)? {
                        return Ok(false);
                    }
                }
            }
            Ok(true)
        }
        if let Some(flow) = self
            .completion_for_owner(owner)
            .and_then(|row| row.cleanup().root_flow())
        {
            for original in flow.expression_plans() {
                if !plan(original, &mut lend)? {
                    return Ok(false);
                }
            }
            for original in flow.maps() {
                let key = match original {
                    MapHomeObservation::FieldInstall(frame) => Some(frame.key()),
                    MapHomeObservation::FieldGet(frame) => Some(frame.key()),
                    _ => None,
                };
                if let Some(key) = key {
                    if !lend(key)? {
                        return Ok(false);
                    }
                }
            }
            for original in flow.named_arrays() {
                if let NamedArrayHomeObservationV1::FieldTextAppend(frame) = original {
                    if !lend(frame.evaluation())? {
                        return Ok(false);
                    }
                }
            }
        }
        for original in self.terminal_relations_for_owner(owner) {
            if let Some(evaluation) =
                self.borrow_return_expression_evaluation_v1(owner, original.return_site())?
            {
                if !lend(evaluation)? {
                    return Ok(false);
                }
            }
            if let TerminalRelationV1::Value(value) = original {
                if let TerminalReturnedSourceV1::StoredMapGet { key, .. } = value.returned() {
                    if value.owner() != owner {
                        return Err(freeze("static-input/qualified-use-identity"));
                    }
                    if !lend(key)? {
                        return Ok(false);
                    }
                }
            }
        }
        self.visit_new_input_arguments_v1(owner, &mut |argument| match argument.kind() {
            OrdinaryNewTrivialArgumentKindV1::Expression { evaluation } => lend(evaluation),
            _ => Ok(true),
        })
    }

    /// Canonical New rows remain in their existing construction/return/take
    /// owners. Validate source identity while those rows are immutably borrowed.
    pub(in crate::mir::normal_callable_semantic_package) fn visit_new_input_arguments_v1(
        &self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
        visitor: &mut impl FnMut(&OrdinaryNewTrivialArgumentV1) -> Result<bool, String>,
    ) -> Result<bool, String> {
        use crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1;
        let local = self.claims.borrow();
        let returned = self.result_claims.borrow();
        let taken = self.local_commits.borrow();
        let mut sites = std::collections::BTreeSet::new();
        for (site, rows) in local
            .values()
            .map(|claim| (claim.site(), claim.argument_rows()))
            .chain(
                returned
                    .values()
                    .map(|claim| (claim.site(), claim.argument_rows())),
            )
            .chain(taken.iter().filter_map(|(site, row)| {
                row.new_argument_rows().map(|rows| (site, rows.as_deref()))
            }))
            .filter(|(site, _)| site.owner() == owner)
        {
            if !sites.insert(site) {
                return Err(freeze("static-input/qualified-use-identity"));
            }
            let Ok(rows) = rows else {
                continue;
            };
            for (index, argument) in rows.iter().enumerate() {
                if argument.owner() != owner
                    || argument.new_site() != site
                    || argument.ordinal() as usize != index
                {
                    return Err(freeze("static-input/qualified-use-identity"));
                }
                if let OrdinaryNewTrivialArgumentKindV1::Expression { evaluation } = argument.kind()
                {
                    let ExpressionPlanRootV1::NewArgument {
                        construction,
                        ordinal,
                        ..
                    } = evaluation.source().root()
                    else {
                        return Err(freeze("static-input/qualified-use-identity"));
                    };
                    if evaluation.source().owner() != owner
                        || construction.site() != site.site()
                        || *ordinal != argument.ordinal()
                        || construction.arguments().get(index) != Some(argument.site())
                        || evaluation.source().nodes().first().map(|node| node.site())
                            != Some(argument.site())
                    {
                        return Err(freeze("static-input/qualified-use-identity"));
                    }
                }
                if !visitor(argument)? {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }

    pub(in crate::mir::normal_callable_semantic_package) fn field_array_text_append_sources_v1(
        &self,
    ) -> Result<
        Vec<crate::mir::normal_callable_semantic_package::FieldArrayAppendSourceLoanV1>,
        String,
    > {
        use crate::mir::resolved_semantics::home_new_prefix::NamedArrayHomeObservationV1;
        let mut sources = Vec::new();
        // Only successful original Completions contain source frames. The
        // owner's execution gate still rejects error-bearing Completions.
        let originals = self
            .completion_index
            .values()
            .filter_map(|row| row.as_ref().ok())
            .chain(
                self.root_completion
                    .iter()
                    .filter_map(|row| row.as_ref().ok())
                    .filter(|root| !self.completion_index.contains_key(&root.owner())),
            );
        for completion in originals {
            let Some(flow) = completion.cleanup().root_flow() else {
                continue;
            };
            for row in flow.named_arrays() {
                if let NamedArrayHomeObservationV1::FieldTextAppend(frame) = row {
                    sources.push(self.field_array_text_append_source_v1(frame.source().call())?);
                }
            }
        }
        Ok(sources)
    }
    /// Presence selects the existing source-scoped consumer, never an empty
    /// cleanup inference. Error-bearing original completions remain errors.
    pub(in crate::mir) fn requires_source_home_scope_v1(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Result<bool, String> {
        if let Some(Err(issue)) = self.completion_index.get(&owner) {
            return Err(format!(
                "[freeze:contract][callable-home/source-completion] {issue:?}"
            ));
        }
        Ok(self
            .completion_for_owner(owner)
            .is_some_and(|completion| completion.cleanup().root_flow().is_some()))
    }

    pub(in crate::mir) fn raw_null_condition_v1(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceExprSiteV1,
    ) -> Result<
        Option<crate::mir::resolved_semantics::home_new_prefix::ComparisonExecutionPlanV1>,
        String,
    > {
        let Some(flow) = self
            .completion_for_owner(owner)
            .and_then(|completion| completion.cleanup().root_flow())
        else {
            return Ok(None);
        };
        let mut plans = flow
            .expression_plans()
            .iter()
            .filter(|plan| plan.source().nodes()[0].site() == site)
            .filter_map(|plan| plan.raw_null_condition_v1());
        let plan = plans.next();
        if plans.next().is_some() {
            return Err("[freeze:contract][bool-condition/duplicate-source-frame]".to_owned());
        }
        Ok(plan)
    }

    pub(in crate::mir) fn completion_for_owner(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Option<&VerifiedFunctionCompletionV1> {
        self.completion_source_for_owner_v1(owner).map(Rc::as_ref)
    }

    pub(in crate::mir::normal_callable_semantic_package) fn completion_source_for_owner_v1(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Option<&Rc<VerifiedFunctionCompletionV1>> {
        self.completion_index
            .get(&owner)
            .and_then(|row| row.as_ref().ok())
            .or_else(|| {
                self.root_completion
                    .as_ref()
                    .and_then(|row| row.as_ref().ok())
                    .filter(|completion| completion.owner() == owner)
            })
    }
}
