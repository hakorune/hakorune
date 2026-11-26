//! Retains the original loop source, its preissued plan and the sealed recipe.

use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::SourceStmtSiteV1;

#[derive(Debug)]
pub(crate) struct RetainedSourceLoopProductV1 {
    source: Option<std::rc::Rc<crate::mir::builder::RetainedCallableLoopSourceV1>>,
    plan: std::cell::RefCell<
        Option<Result<crate::mir::builder::PreissuedCallableLoopSourcePlanV1, String>>,
    >,
    source_recipe: std::cell::OnceCell<
        Result<
            std::rc::Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>,
            String,
        >,
    >,
}
impl RetainedSourceLoopProductV1 {
    pub(in crate::mir) fn issue(
        input: ResolvedFunctionLoweringInputV1<'_>,
        site: &SourceStmtSiteV1,
        policy: crate::mir::builder::LoopFactsPolicyFrameV1,
    ) -> std::rc::Rc<Self> {
        let plan =
            crate::mir::builder::issue_preissued_callable_loop_source_plan_v1(input, site, policy);
        if let Err(reason) = &plan {
            if crate::config::env::builder_debug_enabled() {
                eprintln!("[source-loop] owner={:?} site={site:?} original planner issuance failed: {reason}", input.owner());
            }
        }
        let source = plan
            .as_ref()
            .ok()
            .map(|plan| std::rc::Rc::clone(plan.original_source_v1()));
        std::rc::Rc::new(Self {
            source,
            plan: std::cell::RefCell::new(Some(plan)),
            source_recipe: std::cell::OnceCell::new(),
        })
    }
}
impl RetainedSourceLoopProductV1 {
    pub(crate) fn observation_source_v1(
        &self,
        input: ResolvedFunctionLoweringInputV1<'_>,
    ) -> Option<std::rc::Rc<crate::mir::builder::RetainedCallableLoopSourceV1>> {
        let source = self.source.as_ref()?;
        source.condition_source_v1(input)?;
        Some(std::rc::Rc::clone(source))
    }
    /// Retain one consuming source projection in the SAME selected row. Source
    /// failure is terminal, and this logical product grants no Home After.
    pub(crate) fn retain_source_recipe_v2(
        &self,
        input: ResolvedFunctionLoweringInputV1<'_>,
        summary: crate::mir::resolved_semantics::home_new_prefix::SourceLoopScanSummaryV1,
    ) -> Result<
        std::rc::Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>,
        String,
    > {
        let source = self
            .source
            .as_ref()
            .ok_or("[freeze:contract][source-loop/row-source-missing]")?;
        if !std::rc::Rc::ptr_eq(source, &summary.source) {
            return Err("[freeze:contract][source-loop/row-source-drift]".into());
        }
        if self.source_recipe.get().is_some() {
            return Err("[freeze:contract][source-loop/product-already-retained]".into());
        }
        // Recheck the original Resolve source after the body scan.
        if self
            .observation_source_v1(input)
            .as_ref()
            .is_none_or(|original| !std::rc::Rc::ptr_eq(source, original))
        {
            return Err("[freeze:contract][source-loop/row-plan-drift]".into());
        }
        let product = crate::mir::loop_recipe_contract::produce_source_loop_recipe_v2(
            input, summary,
        )
        .map(std::rc::Rc::new)
        .map_err(|reason| {
            let issue = format!(
                "{reason}; owner={:?}; function={:?}; loop={:?}",
                input.owner(),
                crate::mir::resolved_semantics::CallableHeaderSyntaxViewV1::from_function_ast(
                    input.source().root()
                )
                .map(|header| header.name()),
                source.site()
            );
            if crate::config::env::builder_debug_enabled() {
                eprintln!("[source-loop] original Recipe projection failed: {issue}");
            }
            issue
        });
        self.source_recipe
            .set(product)
            .map_err(|_| "[freeze:contract][source-loop/product-already-retained]".to_owned())?;
        self.source_recipe
            .get()
            .unwrap()
            .as_ref()
            .map(std::rc::Rc::clone)
            .map_err(Clone::clone)
    }
}
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
