//! Exact field-node loans from the SAME sealed scalar Return expression.
//! These neither infer class from MIR nor turn nested receivers into bindings.
use super::*;
use crate::mir::resolved_semantics::{SourceStmtSiteV1, SourcePathSegmentV1, ResolvedBinaryOperatorV1};
use crate::mir::normal_callable_semantic_package::CallableScalarResultSourceLoanV1;
use crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1;

#[derive(Debug, Clone)]
pub(crate) struct ResultPathFieldSourceLoanV1 {
    facts: Rc<BorrowedResultSourceFactsV1>,
    completion: Rc<crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1>,
    expression: Rc<ResultExpressionSourceV1>,
    path: crate::mir::resolved_value_profile::expression_plan::OriginalExpressionFieldPathV1,
}
impl PartialEq for ResultPathFieldSourceLoanV1 {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.facts, &other.facts) && Rc::ptr_eq(&self.completion, &other.completion)
            && Rc::ptr_eq(&self.expression, &other.expression) && self.path == other.path
    }
}
impl Eq for ResultPathFieldSourceLoanV1 {}
impl CallableScalarResultSourceLoanV1 {
    pub(crate) fn nested_result_field_sources_v1(&self) -> Result<Vec<ResultPathFieldSourceLoanV1>, String> {
        let mut loans = Vec::new();
        for expression in &self.facts.expressions {
            let nested = expression.plan.nodes().iter().any(|node|
                expression.plan.field_path_v1(node.site()).is_some_and(|(_, path)| path.len() > 1));
            if !nested { continue; }
            let component = self.facts.field_component.as_ref()
                .ok_or_else(|| freeze("result-field-path/component-missing"))?;
            if !component.permits(expression) { return Err(freeze("result-field-path/component-drift")); }
            for index in expression.plan.ordered_postorder_v1().map_err(|_| freeze("result-field-path/order"))? {
                if !matches!(expression.leaves[index], LeafV1::ObjectField { .. } | LeafV1::Field { .. }) { continue; }
                let loan = ResultPathFieldSourceLoanV1 { facts: Rc::clone(&self.facts), completion: Rc::clone(&self.completion),
                    expression: Rc::clone(expression), path: crate::mir::resolved_value_profile::expression_plan::OriginalExpressionFieldPathV1::issue(
                        Rc::clone(expression.plan()), expression.plan.nodes()[index].site()).ok_or_else(|| freeze("result-field-path/original-path"))? };
                loan.descriptor()?;
                loans.push(loan);
            }
        }
        Ok(loans)
    }
}
impl ResultPathFieldSourceLoanV1 {
    pub(crate) fn site(&self) -> OwnedExprSiteV1 { self.path.site() }
    pub(crate) fn receiver_site(&self) -> &SourceExprSiteV1 { self.path.receiver_site() }
    pub(crate) fn root(&self) -> BindingRefV1 { self.path.root() }
    pub(crate) fn parent(&self) -> Option<OwnedExprSiteV1> { self.path.parent() }
    pub(crate) fn descriptor(&self) -> Result<(hakorune_mir_defs::CanonicalFieldRefV1, LocalFieldReadResultV1, Option<&str>), String> {
        self.expression.field_path_descriptor_v1(self.path.node(), self.completion.owner())
    }
    pub(crate) fn recheck(&self, canonical: &CallableScalarResultSourceLoanV1) -> Result<(), String> {
        if !Rc::ptr_eq(self.path.plan(), self.expression.plan())
            || !Rc::ptr_eq(&self.facts, &canonical.facts) || !Rc::ptr_eq(&self.completion, &canonical.completion)
            || !self.facts.expressions.iter().any(|expression| Rc::ptr_eq(expression, &self.expression))
            || !self.facts.field_component.as_ref().is_some_and(|component| component.permits(&self.expression)) {
            return Err(freeze("result-field-path/source-drift"));
        }
        self.descriptor().map(|_| ())
    }
}

impl ResultExpressionSourceV1 {
    fn field_path_descriptor_v1(&self, index: usize, owner: FunctionOwnerIdV1) -> Result<(hakorune_mir_defs::CanonicalFieldRefV1, LocalFieldReadResultV1, Option<&str>), String> {
        let geometry = crate::mir::resolved_value_profile::expression_plan::OriginalExpressionFieldPathV1::issue(
            Rc::clone(self.plan()), self.plan.nodes()[index].site()).ok_or_else(|| freeze("result-field-path/original-path"))?;
        let root = geometry.root();
        let path = geometry.fields();
        let object = geometry.root_site();
        let root_index = self.plan.nodes().iter().position(|node| node.site() == object)
            .ok_or_else(|| freeze("result-field-path/root"))?;
        if root.owner() != owner || self.plan.owner() != owner { return Err(freeze("result-field-path/owner")); }
        if !matches!(self.leaves[root_index], LeafV1::Receiver(binding) if binding == root)
            || path.last() != Some(&index)
            || path[..path.len()-1].iter().any(|index| !matches!(self.leaves[*index], LeafV1::ObjectField { .. })) {
            return Err(freeze("result-field-path/receiver"));
        }
        match &self.leaves[index] {
            LeafV1::ObjectField { source, .. } => Ok((source.field().ok_or_else(|| freeze("result-field-path/object-source"))?,
                LocalFieldReadResultV1::Alias(source.class().into()), Some(source.class()))),
            LeafV1::Field { source, .. } => Ok((source.field(), LocalFieldReadResultV1::Scalar, source.declared_type_name())),
            _ => Err(freeze("result-field-path/source")),
        }
    }
}

impl BorrowedResultCohortV1 {
    /// Same presealed Return facts, before the canonical Home/Completion walk.
    /// Only an exhaustive pure Integer Add/field expression is Home-neutral.
    pub(in crate::mir::normal_callable_semantic_package) fn checked_nested_field_return_v1(
        &self, ingress: &PreparedBorrowedFormalIngressV1,
        input: ResolvedFunctionLoweringInputV1<'_>, site: &OwnedExprSiteV1,
        exit: &SourceStmtSiteV1, receiver: BindingRefV1,
    ) -> Result<bool, String> {
        let Some(Ok(proof)) = self.get(&input.owner()) else { return Ok(false); };
        let mut expressions = proof.expressions.iter().filter(|expression|
            expression.plan.nodes().first().is_some_and(|node| node.site() == site.site()));
        let Some(expression) = expressions.next() else { return Ok(false); };
        if expressions.next().is_some() { return Err(freeze("result-field-path/return-duplicate")); }
        if !expression.plan.nodes().iter().any(|node|
            expression.plan.field_path_v1(node.site()).is_some_and(|(_, path)| path.len() > 1)) { return Ok(false); }
        if proof.class != BorrowedResultClassV1::I64 { return Ok(false); }
        proof.require_source_i64_v1()?;
        let expected = crate::mir::resolved_semantics::SourcePathV1::from_node(exit.node())
            .child(SourcePathSegmentV1::Value).expr();
        if site.owner() != input.owner() || site.site() != &expected || !proof.returns.contains(site)
            || expression.plan.root().statement() != exit || receiver.owner() != input.owner()
            || !input.function().bindings().any(|(binding, row)| binding == receiver
                && matches!(row.kind(), crate::mir::resolved_semantics::BindingKindV1::Receiver)) {
            return Err(freeze("result-field-path/return-identity"));
        }
        let component = proof.field_component.as_ref().ok_or_else(|| freeze("result-field-path/component-missing"))?;
        self.require_field_component_v1(component)?;
        if !component.permits(expression) { return Err(freeze("result-field-path/component-drift")); }
        composition::source_dependencies_v1(proof, ingress)?;
        for (index, node) in expression.plan.nodes().iter().enumerate() {
            match &expression.leaves[index] {
                LeafV1::Receiver(binding) if *binding == receiver => {},
                LeafV1::ObjectField { .. } | LeafV1::Field { .. } => {
                    expression.field_path_descriptor_v1(index, input.owner())?;
                    if expression.plan.field_path_v1(node.site()).map(|path| path.0) != Some(receiver) {
                        return Err(freeze("result-field-path/return-receiver"));
                    }
                },
                LeafV1::Structural if matches!(node.source(), ExpressionNodeSourceV1::Literal(ResolvedLiteralSourceV1::Integer(_))) => {},
                LeafV1::Structural if node.binary().is_some_and(|binary| binary.operator() == ResolvedBinaryOperatorV1::Add) => {},
                _ => return Ok(false), // Calls/New/opaque operations retain their own observations.
            }
        }
        Ok(true)
    }
}
