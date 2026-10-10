//! The selected package's sole source/Home join for variable-bound Mul Loop.
//! A missing or duplicate candidate remains a retained refusal; no Home After
//! or builder fallback is issued by this prerequisite.

use std::collections::BTreeMap;

use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::variable_bound_mul_recurrence_source::observe_variable_bound_mul_source_v1;
use crate::mir::loop_structural_facts::{
    issue_variable_bound_mul_facts_v1, VariableBoundMulFactsIssueV1,
    VerifiedVariableBoundMulFactsV1,
};
use crate::mir::resolved_semantics::{
    home_new_prefix::LoopI64PreStateRequestV1, FunctionOwnerIdV1, SourceStmtSiteV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopFactsUnavailableV1 {
    SourceMembership,
    SourceShape,
    InputClass,
    Duplicate,
}

pub(crate) type SelectedLoopFactsV1 = BTreeMap<
    (FunctionOwnerIdV1, SourceStmtSiteV1),
    Result<VerifiedVariableBoundMulFactsV1, LoopFactsUnavailableV1>,
>;

pub(super) fn retain_selected_loop_facts_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    prestate: LoopI64PreStateRequestV1,
    rows: &mut SelectedLoopFactsV1,
) {
    let key = (prestate.owner(), prestate.site().clone());
    let candidate = (|| {
        let ledger = input
            .forest()
            .callable_source_ledger(input.owner())
            .map_err(|_| LoopFactsUnavailableV1::SourceMembership)?;
        let membership = ledger
            .resolved_loop_source(prestate.site())
            .map_err(|_| LoopFactsUnavailableV1::SourceMembership)?;
        let observed = observe_variable_bound_mul_source_v1(input, &ledger, membership)
            .map_err(|_| LoopFactsUnavailableV1::SourceShape)?;
        let (owner, source, frame, scope_region, condition, operations, bindings) =
            observed.into_parts();
        issue_variable_bound_mul_facts_v1(
            owner,
            source,
            frame,
            scope_region,
            condition,
            operations,
            bindings,
            prestate,
        )
        .map_err(|error| match error {
            VariableBoundMulFactsIssueV1::ForeignOwner
            | VariableBoundMulFactsIssueV1::SourceSiteConflict
            | VariableBoundMulFactsIssueV1::ForeignFrame => {
                LoopFactsUnavailableV1::SourceMembership
            }
            VariableBoundMulFactsIssueV1::InputClassMissing => LoopFactsUnavailableV1::InputClass,
            VariableBoundMulFactsIssueV1::DuplicateOperation => LoopFactsUnavailableV1::SourceShape,
        })
    })();
    match rows.entry(key) {
        std::collections::btree_map::Entry::Vacant(slot) => {
            slot.insert(candidate);
        }
        std::collections::btree_map::Entry::Occupied(mut slot) => {
            let _ = slot.insert(Err(LoopFactsUnavailableV1::Duplicate));
        }
    }
}
