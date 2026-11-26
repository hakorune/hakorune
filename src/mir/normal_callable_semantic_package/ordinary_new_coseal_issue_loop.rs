//! The selected package retains original Resolve evidence for each Loop.
//! A complete verified source product permits the Home walk to continue.

use std::collections::BTreeMap;

use crate::mir::loop_recipe_contract::RetainedSourceLoopProductV1;

use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::{
    home_new_prefix::{LoopI64PreStateRequestV1, SelectedLoopHomeLoanV1},
    FunctionOwnerIdV1, SourceStmtSiteV1,
};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum LoopProductUnavailableV1 {
    Duplicate,
}

pub(crate) type SelectedLoopProductV1 = BTreeMap<
    (FunctionOwnerIdV1, SourceStmtSiteV1),
    Result<std::rc::Rc<RetainedSourceLoopProductV1>, LoopProductUnavailableV1>,
>;

pub(super) fn retain_selected_loop_product_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    prestate: LoopI64PreStateRequestV1,
    rows: &mut SelectedLoopProductV1,
    policy: crate::mir::builder::LoopFactsPolicyFrameV1,
) -> Option<SelectedLoopHomeLoanV1> {
    let key = (prestate.owner(), prestate.site().clone());
    match rows.entry(key) {
        std::collections::btree_map::Entry::Vacant(slot) => {
            let row = RetainedSourceLoopProductV1::issue(input, prestate.site(), policy);
            let loan = row.observation_source_v1(input)
                .map(|_| SelectedLoopHomeLoanV1::SourceObservation(std::rc::Rc::clone(&row)));
            slot.insert(Ok(row));
            loan
        }
        std::collections::btree_map::Entry::Occupied(mut slot) => {
            let _ = slot.insert(Err(LoopProductUnavailableV1::Duplicate));
            None
        }
    }
}

impl super::OrdinaryNewClaimLedgerV1 {
    /// Borrow branch joins from the SAME selected source-loop product when a
    /// Loop header reads a pre-loop BranchNormal, or its schedule joins a nested If.
    /// Root-flow plans do not flatten these source-loop observations.
    pub(in crate::mir) fn selected_source_loop_branch_joins_v2(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceStmtSiteV1,
    ) -> Vec<std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::BranchNormalScalarSourceV1>>
    {
        let branch_path = site.node().segments();
        let mut output: Vec<
            std::rc::Rc<
                crate::mir::resolved_semantics::home_new_prefix::BranchNormalScalarSourceV1,
            >,
        > = Vec::new();
        for ((actual_owner, loop_site), row) in &self.selected_loop_products {
            if *actual_owner != owner {
                continue;
            }
            let Ok(retained) = row else {
                continue;
            };
            let Some(Ok(product)) = retained.source_recipe() else {
                continue;
            };
            for basis in product.source.headers.values() {
                if let crate::mir::resolved_semantics::home_new_prefix::SourceLoopValueProducerV1::Scalar(
                    crate::mir::resolved_semantics::home_new_prefix::CurrentScalarProducerV1::BranchNormal(source),
                ) = basis.entry_producer()
                {
                    if source.site() == site
                        && !output.iter().any(|old| old.as_ref() == source.as_ref())
                    {
                        output.push(std::rc::Rc::clone(source));
                    }
                }
            }
            if !branch_path.starts_with(loop_site.node().segments()) {
                continue;
            }
            for join in product
                .schedule()
                .branch_joins_v2()
                .filter(|join| join.source.site() == site)
            {
                if !output
                    .iter()
                    .any(|old| old.as_ref() == join.source.as_ref())
                {
                    output.push(std::rc::Rc::clone(&join.source));
                }
            }
        }
        output
    }

    pub(in crate::mir) fn selected_source_loop_text_branch_joins_v2(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceStmtSiteV1,
    ) -> Vec<std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::BranchNormalTextSourceV1>> {
        let branch_path = site.node().segments();
        let mut output: Vec<std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::BranchNormalTextSourceV1>> = Vec::new();
        for ((actual_owner, loop_site), row) in &self.selected_loop_products {
            if *actual_owner != owner || !branch_path.starts_with(loop_site.node().segments()) {
                continue;
            }
            let Ok(retained) = row else {
                continue;
            };
            let Some(Ok(product)) = retained.source_recipe() else {
                continue;
            };
            for join in product.schedule().text_branch_joins_v2()
                .filter(|join| join.source.site() == site)
            {
                if !output.iter().any(|old| old.as_ref() == join.source.as_ref()) {
                    output.push(std::rc::Rc::clone(&join.source));
                }
            }
        }
        output
    }

    /// The existing selected row retains original source after affine plan take.
    /// A selected but missing/drifted source is an error, never an unselected row.
    pub(in crate::mir) fn retained_loop_source_v1(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceStmtSiteV1,
    ) -> Result<Option<&std::rc::Rc<crate::mir::builder::RetainedCallableLoopSourceV1>>, String>
    {
        match self.selected_loop_products.get(&(owner, site.clone())) {
            Some(Ok(row)) => {
                let source = row
                    .retained_source()
                    .ok_or("[freeze:contract][preissued-loop/retained-source-missing]")?;
                if source.owner() != owner || source.site() != site {
                    return Err("[freeze:contract][preissued-loop/retained-source-drift]".into());
                }
                Ok(Some(source))
            }
            _ => Ok(None),
        }
    }

    /// Borrow the same source-tree Recipe produced during the selected Home walk.
    /// This remains logical evidence until Core/cleanup/physical closure lands.
    pub(in crate::mir) fn retained_source_loop_recipe_v2(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceStmtSiteV1,
    ) -> Result<
        Option<&std::rc::Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>>,
        String,
    > {
        match self.selected_loop_products.get(&(owner, site.clone())) {
            Some(Ok(row)) => {
                row.source_recipe().map_or(Ok(None), |product| {
                    product.as_ref().map(Some).map_err(Clone::clone)
                })
            }
            _ => Ok(None),
        }
    }

    pub(in crate::mir) fn take_preissued_loop_source_handoff_v2(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceStmtSiteV1,
    ) -> Result<Option<crate::mir::builder::PreissuedCallableLoopSourcePlanV1>, String> {
        match self.selected_loop_products.get(&(owner, site.clone())) {
            Some(Ok(slot)) => {
                slot.take_plan_v2(owner, site).map(Some)
            }
            _ => Ok(None),
        }
    }
}

impl super::OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir) fn validate_preissued_loop_consumption_v1(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Result<(), String> {
        if self.selected_loop_products.iter().any(|((source_owner, _), row)| *source_owner == owner
            && matches!(row, Ok(slot) if slot.has_plan())) {
            return Err("[freeze:contract][preissued-loop/unconsumed]".into());
        }
        if self
            .selected_loop_products
            .iter()
            .any(|((source_owner, _), row)| {
                *source_owner == owner
                    && matches!(row, Ok(slot)
                if slot.retained_source().is_some_and(|source| source.binding_handoff_pending_v1()))
            })
        {
            return Err("[freeze:contract][preissued-loop/binding-handoff-unconsumed]".into());
        }
        Ok(())
    }
}
