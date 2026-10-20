//! Original finite entry reads. Source proof only; current state is separate.
use super::*;
use super::super::super::borrowed_formal_source::{SourceValueSumV1, SourceValueAlternativeV1};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::{SourceExprSiteV1, ResolvedLexicalRefV1};
use crate::mir::resolved_semantics::source_value_flow::{VerifiedSourceValueFlowV1, SourceValueRefV1};
use crate::mir::resolved_value_profile::expression_plan::{ExpressionSourcePlanV1, ExpressionPlanRootV1};
use crate::mir::normal_callable_semantic_package::BorrowedTextEntryReadRequirementV1;

#[derive(Debug, Clone)]
pub(crate) struct FiniteEntryReadV1 {
    requirement: BorrowedTextEntryReadRequirementV1,
    sum: Rc<SourceValueSumV1>,
    flow: std::sync::Arc<VerifiedSourceValueFlowV1>,
    value: SourceValueRefV1,
}
impl PartialEq for FiniteEntryReadV1 {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.sum, &other.sum) && std::sync::Arc::ptr_eq(&self.flow, &other.flow)
            && self.value == other.value && self.requirement == other.requirement
            && Rc::ptr_eq(self.requirement.source_plan(), other.requirement.source_plan())
    }
}
impl Eq for FiniteEntryReadV1 {}
impl FiniteEntryReadV1 {
    pub(crate) fn requirement(&self) -> &BorrowedTextEntryReadRequirementV1 { &self.requirement }
    pub(crate) fn signed_only(&self) -> bool { self.sum.signed_only() }
    pub(in crate::mir) fn corroborates_finite_domain_v1(&self, domain: &Rc<SourceValueSumV1>) -> bool {
        Rc::ptr_eq(&self.sum, domain)
            && self.flow.read(self.requirement.read(), self.requirement.binding()) == Some(self.value)
    }
    /// Finite original alternatives for this source read, not a runtime
    /// carrier. Null removal requires the SAME covered branch/entry proof.
    pub(crate) fn leaf_class(&self, exclusion: Option<&crate::mir::resolved_semantics::home_new_prefix::EntryNullExclusionV1>)
        -> Option<crate::mir::resolved_value_profile::expression_projection::ExpressionLeafClassV1> {
        use crate::mir::resolved_value_profile::{expression_class::ExpressionValueClassV1 as Class,
            expression_projection::ExpressionLeafClassV1 as Leaf};
        let mut classes = Vec::new();
        for alternative in self.sum.alternatives() {
            match alternative {
                SourceValueAlternativeV1::Text => classes.push(Class::String),
                SourceValueAlternativeV1::SignedI64 => classes.push(Class::I64),
                SourceValueAlternativeV1::Null if exclusion.is_some_and(|proof|
                    proof.same_entry(&self.flow, self.requirement.formal().1)) => {},
                _ => return None,
            }
        }
        Leaf::from_classes(classes)
    }
}

pub(in crate::mir::normal_callable_semantic_package) fn lend_finite_entry_read_v1(
    input: ResolvedFunctionLoweringInputV1<'_>, prepared: &PreparedBorrowedFormalIngressV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    site: &OwnedExprSiteV1, plan: &Rc<ExpressionSourcePlanV1>,
    read: &SourceExprSiteV1, binding: BindingRefV1, root: BindingRefV1,
) -> Result<Option<FiniteEntryReadV1>, String> {
    let reject = || freeze("borrowed-text-entry/initializer-source-identity");
    let original_root = match plan.root() {
        ExpressionPlanRootV1::LocalInitializer { initializer, .. } =>
            initializer.initializer_site() == Some(site.site())
                && input.function().expression_source().initializers().filter(|row| *row == initializer).count() == 1,
        ExpressionPlanRootV1::Statement { statement, role: crate::mir::resolved_value_profile::expression_root::StatementExpressionRoleV1::ReturnValue, value } => {
            let ledger = input.forest().callable_source_ledger(input.owner()).map_err(|_| reject())?;
            let original = crate::mir::resolved_value_profile::expression_plan::statement_expression_source_plan_v1(
                &ledger, input.body_shape().ok_or_else(reject)?, statement).map_err(|_| reject())?;
            value == site.site() && original.as_deref() == Some(plan.as_ref())
        }
        ExpressionPlanRootV1::IfCondition { statement, condition, .. } => {
            let original = crate::mir::resolved_value_profile::expression_plan::condition_expression_source_plan_v1(
                input, statement).map_err(|_| reject())?;
            condition == site.site() && original.as_ref() == plan.as_ref()
        }
        ExpressionPlanRootV1::LoopCondition { statement, condition, .. } => {
            let original = crate::mir::resolved_value_profile::expression_plan::loop_condition_expression_source_plan_v1(
                input, statement).map_err(|_| reject())?;
            condition == site.site() && original.as_ref() == plan.as_ref()
        }
        _ => false,
    };
    if input.owner() != plan.owner() || site.owner() != input.owner()
        || !original_root || root.owner() != input.owner()
        || binding.owner() != input.owner()
        || input.function().variable_ref(read) != Some(ResolvedLexicalRefV1::Local(binding)) {
        return Err(reject());
    }
    let Some(flow) = input.function().expression_source().value_flow() else { return Ok(None) };
    let Some(value) = flow.read(read, binding) else { return Ok(None) };
    if flow.alias_entry_origin(value, |site| input.function().variable_ref(site)) != Some(root) { return Ok(None); }
    let Some(sum) = prepared.lend_source_value_sum_v1(root) else { return Ok(None) };
    // Keep every original finite alternative. The later initializer joins
    // real current/terminating-branch evidence; this source loan removes none.
    if sum.alternatives().is_empty() || sum.alternatives().iter().any(|alternative|
        !matches!(alternative, SourceValueAlternativeV1::Text | SourceValueAlternativeV1::SignedI64 | SourceValueAlternativeV1::Null)) {
        return Ok(None);
    }
    let requirement = super::super::super::borrowed_formal_entry::issue_text_entry_read_requirement_v1(
        contracts, plan, read, binding, root,
        prepared.original_formal_read_v1(input.owner(), read, binding, root)?)?;
    Ok(Some(FiniteEntryReadV1 { requirement, sum, flow: std::sync::Arc::clone(flow), value }))
}

impl PreparedBorrowedFormalIngressV1 {
    /// Select the existing Home walk for an original Text/Core initializer.
    /// This is a source demand only; the qualifier still proves the whole root,
    /// current state and acquisition. Call-owned inputs are not parent leaves.
    pub(in crate::mir::normal_callable_semantic_package) fn requires_initializer_expression_walk_v1(
        &self, input: ResolvedFunctionLoweringInputV1<'_>,
    ) -> bool {
        use crate::mir::resolved_value_profile::expression_plan::{local_expression_source_plan_v1, ExpressionNodeSourceV1 as Node};
        let Ok(ledger) = input.forest().callable_source_ledger(input.owner()) else { return false };
        input.function().expression_source().initializers().any(|initializer| {
            if initializer.initializer_site().is_some_and(|site|
                matches!(input.function().expression_source().literal(site), Some(crate::mir::resolved_semantics::ResolvedLiteralSourceV1::String(_)))) { return true; }
            let Ok(plan) = local_expression_source_plan_v1(&ledger, input.body_shape(), initializer) else { return false };
            if let Some(Node::Reference(ResolvedLexicalRefV1::Local(binding))) = plan.nodes().first().map(|node| node.source()) {
                if let Some(flow) = input.function().expression_source().value_flow() {
                    if let Some(root) = flow.read(plan.nodes()[0].site(), *binding)
                        .and_then(|value| flow.alias_entry_origin(value, |site| input.function().variable_ref(site))) {
                        if self.source_value_sum_v1(root).is_some_and(|sum| sum.signed_only()) { return true; }
                    }
                }
            }
            self.requires_text_expression_plan_v1(input, &plan)
        })
    }

    /// Original ReturnValue demand selects the same Home walk as Text locals.
    /// Selection issues no completion, result class or execution permission.
    pub(in crate::mir::normal_callable_semantic_package) fn requires_text_return_walk_v1(
        &self, input: ResolvedFunctionLoweringInputV1<'_>,
    ) -> bool {
        // A new lifecycle Return needs an original consumer. Passive generated
        // methods and self-recursive-only unused helpers keep their prior lane.
        if !self.source_incoming.source_rows_v1().any(|row|
            row.callee == input.owner() && row.call.owner() != input.owner()) { return false; }
        let Some(shape) = input.body_shape() else { return false };
        let Ok(ledger) = input.forest().callable_source_ledger(input.owner()) else { return false };
        shape.statements().iter().any(|statement| {
            if !matches!(statement, crate::mir::resolved_semantics::BodyStatementShapeV1::Return { value: Some(_), .. }) { return false; }
            let Ok(Some(plan)) = crate::mir::resolved_value_profile::expression_plan::statement_expression_source_plan_v1(
                &ledger, shape, statement.site()) else { return false };
            matches!(plan.nodes().first().map(|node| node.source()),
                Some(crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::Literal(
                    crate::mir::resolved_semantics::ResolvedLiteralSourceV1::String(_))))
                || self.requires_text_expression_plan_v1(input, &plan)
        })
    }

    fn requires_text_expression_plan_v1(
        &self, input: ResolvedFunctionLoweringInputV1<'_>, plan: &ExpressionSourcePlanV1,
    ) -> bool {
        use crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1 as Node;
        use crate::mir::resolved_semantics::ResolvedBinaryOperatorV1;
        if plan.nodes().iter().any(|node| matches!(node.source(), Node::MethodCall)
            && (self.ordinary_core_source_v1(&OwnedExprSiteV1::new(input.owner(), node.site().clone())).is_some()
                || input.function().method_call_source_v1(node.site()).is_some_and(|method|
                super::ordinary_text_argument_source_v1(self,
                    &OwnedExprSiteV1::new(input.owner(), node.site().clone()), method)
                    .map_or(true, |source| source.is_some())))) { return true; }
        if !plan.nodes().first().and_then(|node| input.function().expression_source().binary(node.site()))
            .is_some_and(|binary| binary.operator() == ResolvedBinaryOperatorV1::Add) { return false; }
        let required = plan.nodes_outside_calls().any(|(_, leaf)| {
            match leaf.source() {
                Node::Literal(crate::mir::resolved_semantics::ResolvedLiteralSourceV1::String(_)) => true,
                Node::Reference(ResolvedLexicalRefV1::Local(binding)) => {
                let Some(flow) = input.function().expression_source().value_flow() else { return false };
                let Some(value) = flow.read(leaf.site(), *binding) else { return false };
                let Some(root) = flow.alias_entry_origin(value, |site| input.function().variable_ref(site)) else { return false };
                self.source_value_sum_v1(root).is_some_and(|sum| sum.alternatives().len() == 1
                    && sum.alternatives().contains(&SourceValueAlternativeV1::Text))
                }
                _ => false,
            }
        });
        required
    }

}
