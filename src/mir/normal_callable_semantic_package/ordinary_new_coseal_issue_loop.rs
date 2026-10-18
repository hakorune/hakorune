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
    Result<SelectedLoopProductRowV1, LoopProductUnavailableV1>,
>;

/// One existing source-product row, with affine physical consumption on the
/// preissued generic arm. This adds no parallel source registry.
#[derive(Debug)]
pub(crate) enum SelectedLoopProductRowV1 {
    VariableBoundMul(VerifiedVariableBoundMulRecipeProductV1),
    SourcePlan(RetainedSourceLoopProductV1),
}
#[derive(Debug)]
pub(crate) struct RetainedSourceLoopProductV1 {
    source: Option<std::rc::Rc<crate::mir::builder::RetainedCallableLoopSourceV1>>,
    plan: std::cell::RefCell<Option<Result<crate::mir::builder::PreissuedCallableLoopSourcePlanV1, String>>>,
    home: std::cell::OnceCell<std::rc::Rc<crate::mir::builder::VerifiedNoExitArrayHomeAfterV1>>,
}
impl SelectedLoopProductRowV1 {
    fn home_after_loan(&self) -> Option<VerifiedLoopHomeAfterLoanV1> {
        match self { Self::VariableBoundMul(row) => row.home_after_loan(), Self::SourcePlan(_) => None }
    }
}

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
            slot.insert(candidate.map(SelectedLoopProductRowV1::VariableBoundMul)).as_ref().ok()?.home_after_loan()
        }
        std::collections::btree_map::Entry::Occupied(mut slot) => {
            let _ = slot.insert(Err(LoopProductUnavailableV1::Duplicate));
            None
        }
    }
}

pub(super) fn issue_selected_home_loop_loan_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    prestate: LoopI64PreStateRequestV1,
    static_sites: &std::collections::BTreeSet<(FunctionOwnerIdV1, SourceStmtSiteV1)>,
    static_products: &BTreeMap<
        FunctionOwnerIdV1,
        Result<crate::mir::builder::StaticI64LoopFunctionEntryV2, String>,
    >,
    variable_rows: &mut SelectedLoopProductV1,
    source_core_calls: Option<&crate::mir::normal_callable_semantic_package::core_method_source::SourceCoreMethodCallRowsV1>,
    policy: crate::mir::builder::LoopFactsPolicyFrameV1,
) -> Option<crate::mir::resolved_semantics::home_new_prefix::SelectedLoopHomeLoanV1> {
    use crate::mir::resolved_semantics::home_new_prefix::SelectedLoopHomeLoanV1;
    if static_sites.contains(&(prestate.owner(), prestate.site().clone())) {
        let product = static_products.get(&prestate.owner())?.as_ref().ok()?;
        let source = product.home_source();
        if source.owner() != prestate.owner()
            || source.loop_site() != prestate.site()
            || !prestate.proves_integer(source.bin_binding())
            || !prestate.proves_integer(source.n_binding())
        {
            return None;
        }
        return Some(SelectedLoopHomeLoanV1::StaticI64Search(source));
    }
    if source_core_calls.is_some_and(|rows| rows.values().any(|row|
        row.contract().owner() == prestate.owner() && row.contract().loop_site() == prestate.site()
            && row.contract().named_array_requirement().and_then(|row| row.residence()).is_some())) {
        let key = (prestate.owner(), prestate.site().clone());
        match variable_rows.entry(key) {
            std::collections::btree_map::Entry::Vacant(slot) => {
                let plan = crate::mir::builder::issue_preissued_callable_loop_source_plan_v1(input, prestate.site(), policy);
                let source = plan.as_ref().ok().map(|plan| std::rc::Rc::clone(plan.original_source_v1()));
                slot.insert(Ok(SelectedLoopProductRowV1::SourcePlan(RetainedSourceLoopProductV1 {
                    source, plan: std::cell::RefCell::new(Some(plan)), home: std::cell::OnceCell::new(),
                })));
            }
            std::collections::btree_map::Entry::Occupied(_) => {}
        }
        let Some(Ok(SelectedLoopProductRowV1::SourcePlan(slot))) = variable_rows.get(&(prestate.owner(), prestate.site().clone())) else { return None; };
        let contracts = || source_core_calls.into_iter().flat_map(|rows| rows.values()).map(|row| row.contract());
        if let Some(home) = slot.home.get() {
            return (home.matches_prestate(&prestate) && home.recheck_source(input, contracts()))
                .then(|| SelectedLoopHomeLoanV1::NoExitArrayAfter(std::rc::Rc::clone(home)));
        }
        let retained = slot.plan.borrow();
        let plan = retained.as_ref()?.as_ref().ok()?;
        let home = std::rc::Rc::new(plan.prove_array_home_after(input, &prestate, contracts())?);
        slot.home.set(std::rc::Rc::clone(&home)).ok()?;
        return Some(SelectedLoopHomeLoanV1::NoExitArrayAfter(home));
    }
    retain_selected_loop_product_v1(input, prestate, variable_rows)
        .map(SelectedLoopHomeLoanV1::VariableAccumAfter)
}

/// Lend the Home-issued early local return to the existing child retention.
/// The selected V2 product fixes its source identity; the result claim fixes
/// the public I64 class. This does not issue another terminal relation.
pub(super) fn retain_static_i64_search_early_return_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    caller: Option<&crate::mir::builder::CanonicalSameModuleCallableKeyV1>,
    claims: &crate::mir::normal_callable_semantic_package::qualified_static_call_claim::QualifiedStaticCallClaimIndexV1,
    products: &BTreeMap<
        FunctionOwnerIdV1,
        Result<crate::mir::builder::StaticI64LoopFunctionEntryV2, String>,
    >,
    relation: &crate::mir::resolved_semantics::home_new_prefix::TerminalRelationV1,
) -> bool {
    use crate::mir::callable_result_representation::VerifiedCallableResultDispositionV1;
    use crate::mir::resolved_semantics::home_new_prefix::TerminalRelationV1;
    use crate::mir::resolved_semantics::{ResolvedLexicalRefV1, SourcePathSegmentV1, SourcePathV1};
    let TerminalRelationV1::I64Scalar(scalar) = relation else {
        return false;
    };
    let Some(caller) = caller else { return false };
    let Some(product) = products
        .get(&input.owner())
        .and_then(|row| row.as_ref().ok())
    else {
        return false;
    };
    let roles = product.semantic().roles();
    let value = SourcePathV1::from_node(roles.return_site.node())
        .child(SourcePathSegmentV1::Value)
        .expr();
    let entry = product.semantic().source_calls().0;
    scalar.owner() == input.owner()
        && scalar.return_site() == &roles.return_site
        && scalar.value_site() == &value
        && input.function().variable_ref(&value)
            == Some(ResolvedLexicalRefV1::Local(roles.bin_binding))
        && entry.original().caller() == caller
        && matches!(
            claims.result_for_key(caller),
            Some(VerifiedCallableResultDispositionV1::ExactI64 { .. })
        )
}

impl super::OrdinaryNewClaimLedgerV1 {
    /// The existing selected row retains original source after affine plan take.
    /// A selected but missing/drifted source is an error, never an unselected row.
    pub(in crate::mir) fn retained_loop_source_v1(
        &self, owner: FunctionOwnerIdV1, site: &SourceStmtSiteV1,
    ) -> Result<Option<&std::rc::Rc<crate::mir::builder::RetainedCallableLoopSourceV1>>, String> {
        match self.selected_loop_products.get(&(owner, site.clone())) {
            Some(Ok(SelectedLoopProductRowV1::SourcePlan(row))) => {
                let source = row.source.as_ref().ok_or("[freeze:contract][preissued-loop/retained-source-missing]")?;
                if source.owner() != owner || source.site() != site {
                    return Err("[freeze:contract][preissued-loop/retained-source-drift]".into());
                }
                Ok(Some(source))
            }
            _ => Ok(None),
        }
    }

    /// Borrow the SAME source/Home proof before or after affine Recipe take.
    pub(in crate::mir) fn no_exit_array_home_source_v1(
        &self, owner: FunctionOwnerIdV1, site: &SourceStmtSiteV1,
    ) -> Option<&std::rc::Rc<crate::mir::builder::VerifiedNoExitArrayHomeAfterV1>> {
        match self.selected_loop_products.get(&(owner, site.clone())) {
            Some(Ok(SelectedLoopProductRowV1::SourcePlan(row))) => row.home.get(),
            _ => None,
        }
    }

    pub(in crate::mir) fn take_preissued_loop_source_plan_v1(
        &self, owner: FunctionOwnerIdV1, site: &SourceStmtSiteV1,
    ) -> Result<Option<crate::mir::builder::PreissuedCallableLoopSourcePlanV1>, String> {
        match self.selected_loop_products.get(&(owner, site.clone())) {
            Some(Ok(SelectedLoopProductRowV1::SourcePlan(slot))) => slot.plan.borrow_mut().take()
                .ok_or_else(|| "[freeze:contract][preissued-loop/already-taken]".to_owned())?
                .map(Some),
            _ => Ok(None),
        }
    }
}

impl super::OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir) fn validate_preissued_loop_consumption_v1(&self, owner: FunctionOwnerIdV1) -> Result<(), String> {
        if self.selected_loop_products.iter().any(|((source_owner, _), row)| *source_owner == owner
            && matches!(row, Ok(SelectedLoopProductRowV1::SourcePlan(slot)) if slot.plan.borrow().is_some())) {
            return Err("[freeze:contract][preissued-loop/unconsumed]".into());
        }
        if self.selected_loop_products.iter().any(|((source_owner, _), row)| *source_owner == owner
            && matches!(row, Ok(SelectedLoopProductRowV1::SourcePlan(slot))
                if slot.source.as_ref().is_some_and(|source| source.binding_handoff_pending_v1()))) {
            return Err("[freeze:contract][preissued-loop/binding-handoff-unconsumed]".into());
        }
        Ok(())
    }
}
