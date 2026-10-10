//! The selected package's sole source/Home join for variable-bound Mul Loop.
//! Facts are consumed into one retained Recipe product. A complete verified
//! After loan is the only permission for the selected Home walk to continue.

use std::collections::BTreeMap;

use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::variable_bound_mul_recurrence_source::observe_variable_bound_mul_source_v1;
use crate::mir::loop_recipe_contract::{
    produce_variable_bound_mul_recipe_v1, VariableBoundMulRecipeRejectV1,
    VerifiedLoopHomeAfterLoanV1, VerifiedVariableBoundMulRecipeProductV1,
};
use crate::mir::loop_structural_facts::{
    issue_variable_bound_mul_facts_v1, VariableBoundMulFactsIssueV1,
};
use crate::mir::resolved_semantics::{
    home_new_prefix::LoopI64PreStateRequestV1, FunctionOwnerIdV1, SourceStmtSiteV1,
};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum LoopProductUnavailableV1 {
    SourceMembership,
    SourceShape,
    InputClass,
    Recipe(VariableBoundMulRecipeRejectV1),
    Duplicate,
}

pub(crate) type SelectedLoopProductV1 = BTreeMap<
    (FunctionOwnerIdV1, SourceStmtSiteV1),
    Result<VerifiedVariableBoundMulRecipeProductV1, LoopProductUnavailableV1>,
>;

pub(super) fn retain_selected_loop_product_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    prestate: LoopI64PreStateRequestV1,
    rows: &mut SelectedLoopProductV1,
) -> Option<VerifiedLoopHomeAfterLoanV1> {
    let key = (prestate.owner(), prestate.site().clone());
    let candidate = (|| {
        let ledger = input
            .forest()
            .callable_source_ledger(input.owner())
            .map_err(|_| LoopProductUnavailableV1::SourceMembership)?;
        let membership = ledger
            .resolved_loop_source(prestate.site())
            .map_err(|_| LoopProductUnavailableV1::SourceMembership)?;
        let observed = observe_variable_bound_mul_source_v1(input, &ledger, membership)
            .map_err(|_| LoopProductUnavailableV1::SourceShape)?;
        let (
            owner,
            source,
            frame,
            scope_region,
            condition,
            condition_operands,
            operations,
            operation_operands,
            bindings,
            inputs,
        ) = observed.into_parts();
        let facts = issue_variable_bound_mul_facts_v1(
            owner,
            source,
            frame,
            scope_region,
            condition,
            condition_operands,
            operations,
            operation_operands,
            bindings,
            inputs,
            prestate,
        )
        .map_err(|error| match error {
            VariableBoundMulFactsIssueV1::ForeignOwner
            | VariableBoundMulFactsIssueV1::SourceSiteConflict
            | VariableBoundMulFactsIssueV1::ForeignFrame => {
                LoopProductUnavailableV1::SourceMembership
            }
            VariableBoundMulFactsIssueV1::InputClassMissing => LoopProductUnavailableV1::InputClass,
            VariableBoundMulFactsIssueV1::DuplicateOperation => {
                LoopProductUnavailableV1::SourceShape
            }
        })?;
        produce_variable_bound_mul_recipe_v1(facts).map_err(LoopProductUnavailableV1::Recipe)
    })();
    match rows.entry(key) {
        std::collections::btree_map::Entry::Vacant(slot) => {
            slot.insert(candidate).as_ref().ok()?.home_after_loan()
        }
        std::collections::btree_map::Entry::Occupied(mut slot) => {
            let _ = slot.insert(Err(LoopProductUnavailableV1::Duplicate));
            None
        }
    }
}
