//! Original ordinary Core source issuance; no Loop or physical capability.
use std::rc::Rc;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::ClosedOriginalCoreSourceV1;
use crate::mir::resolved_semantics::{
    CoreMethodInstanceTargetIssuerV1, ResolverCoreMethodCallableContractIssuerV1,
    VerifiedResolverOrdinaryCoreMethodSourceV1,
};

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package) struct OrdinaryCoreSourceCallV1 {
    operands: Rc<ClosedOriginalCoreSourceV1>,
    source: VerifiedResolverOrdinaryCoreMethodSourceV1,
}
impl OrdinaryCoreSourceCallV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn source(&self) -> &VerifiedResolverOrdinaryCoreMethodSourceV1 { &self.source }
    pub(in crate::mir::normal_callable_semantic_package) fn operands(&self) -> &Rc<ClosedOriginalCoreSourceV1> { &self.operands }
}

pub(in crate::mir::normal_callable_semantic_package) fn issue_ordinary_core_source_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    operands: &Rc<ClosedOriginalCoreSourceV1>,
) -> Result<Rc<OrdinaryCoreSourceCallV1>, String> {
    let reject = || "[freeze:contract][ordinary-core/source-identity]".to_owned();
    let method = operands.method();
    if input.owner() != method.owner()
        || !input.function().method_call_source_v1(method.site()).is_some_and(|original| std::sync::Arc::ptr_eq(original, method))
    { return Err(reject()); }
    let ledger = input.forest().callable_source_ledger(input.owner()).map_err(|_| reject())?;
    let mut issuer = CoreMethodInstanceTargetIssuerV1::string_box_text(operands.manifest().brand())
        .map_err(|error| format!("[freeze:contract][ordinary-core/target] {error:?}"))?;
    let target = issuer.issue(operands.manifest())
        .map_err(|error| format!("[freeze:contract][ordinary-core/target] {error:?}"))?;
    if target.checked_code_point_law_v1().is_none() {
        return Err("[freeze:contract][ordinary-core/semantic-law]".to_owned());
    }
    let source = ResolverCoreMethodCallableContractIssuerV1::issue_ordinary_source(&ledger, method, target)
        .map_err(|error| format!("[freeze:contract][ordinary-core/source] {error:?}"))?;
    if !std::sync::Arc::ptr_eq(source.source(), method) { return Err(reject()); }
    Ok(Rc::new(OrdinaryCoreSourceCallV1 { source, operands: Rc::clone(operands) }))
}


/// A loan of one canonical source receipt inside the SAME original plan.
/// This is neither an executable call packet nor an acquisition/entry grant.
#[derive(Debug, Clone)]
pub(crate) struct ExpressionCoreSourceLoanV1 {
    plan: Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>,
    node: usize,
    original: Rc<OrdinaryCoreSourceCallV1>,
}
impl PartialEq for ExpressionCoreSourceLoanV1 {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node && Rc::ptr_eq(&self.plan, &other.plan)
            && Rc::ptr_eq(&self.original, &other.original)
    }
}
impl Eq for ExpressionCoreSourceLoanV1 {}
impl ExpressionCoreSourceLoanV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn lend(
        input: ResolvedFunctionLoweringInputV1<'_>,
        plan: &Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>,
        node: usize, original: &Rc<OrdinaryCoreSourceCallV1>,
    ) -> Result<Self, String> {
        let loan = Self { plan: Rc::clone(plan), node, original: Rc::clone(original) };
        if input.owner() != plan.owner() || !loan.matches_plan(plan, node)
            || !input.function().method_call_source_v1(loan.source().source().site())
                .is_some_and(|method| std::sync::Arc::ptr_eq(method, loan.source().source())) {
            return Err("[freeze:contract][expression-core/source-identity]".to_owned());
        }
        Ok(loan)
    }
    pub(crate) fn source(&self) -> &VerifiedResolverOrdinaryCoreMethodSourceV1 { self.original.source() }
    pub(crate) fn node(&self) -> usize { self.node }
    pub(crate) fn plan(&self) -> &Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1> { &self.plan }
    pub(crate) fn matches_plan(&self,
        plan: &Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>, node: usize,
    ) -> bool {
        use crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1 as Node;
        Rc::ptr_eq(plan, &self.plan) && node == self.node && self.source().source().owner() == plan.owner()
            && plan.nodes().get(node).is_some_and(|row| row.site() == self.source().source().site()
                && matches!(row.source(), Node::MethodCall))
            && self.source().target().checked_code_point_law_v1().is_some()
            && std::iter::once(self.source().source().receiver_site())
                .chain(self.source().source().arguments().iter().map(|arg| arg.site()))
                .all(|site| plan.nodes().iter().filter(|row| row.site() == site).count() == 1)
    }
    pub(crate) fn owns_result(&self) -> bool {
        self.source().target().result() == crate::mir::resolved_semantics::CoreMethodHomeResultRelationV1::TextToCaller
    }
}
