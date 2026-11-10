//! Source field requirements and their original evaluation-time Home basis.
use super::*;

impl HomeExpressionPlanV1 {
    /// Whole initializer preflight supplies value-class proof separately. This
    /// frame retains only original read requirements and evaluation-time Homes.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn initializer_reads(
        source: Rc<ExpressionSourcePlanV1>,
        homes: &[LiveHomeV1],
        prepared: Option<Self>,
        requests: &[LocalFieldReadRequestV1],
    ) -> Option<Self> {
        if source.has_opaque_obligations() || source.root().initializer().is_none() {
            return None;
        }
        let mut plan = prepared.unwrap_or_else(|| Self {
            source: Rc::clone(&source),
            scalar_operations: Vec::new(),
            source_fields: Vec::new(),
            print_frame: None,
            argument_frame: None,
            initializer_frame: None,
            rebind_frame: None,
            condition_frame: None,
            loop_recipe: None,
            field_write: None,
            source_field_homes: None,
        });
        if !Rc::ptr_eq(&plan.source, &source) || plan.source_field_homes.is_some() {
            return None;
        }
        plan.source_field_homes = Some(Rc::from(homes));
        plan.with_source_field_requirements(requests)
    }
    /// Keep only the original field requirement and evaluation-time Homes.
    /// Source observation grants no executable or nonnull object operand.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn borrowed_argument_reads_v1(
        source: Rc<ExpressionSourcePlanV1>,
        homes: &[LiveHomeV1],
        requests: &[LocalFieldReadRequestV1],
    ) -> Option<Self> {
        if source.has_opaque_obligations()
            || !matches!(source.root(), ExpressionPlanRootV1::CallArgument { .. })
            || requests.len() != 1 || requests[0].node_in_plan(&source) != Some(0)
            || !matches!(requests[0].received,
                Some(crate::mir::resolved_semantics::home_new_prefix::ReceivedFieldReadRequestV1::StoredMapGet { .. }))
        { return None; }
        Self::source_reads_v1(source, homes, requests)
    }
    /// Refine an already qualified whole root after its strict field batch.
    /// Existing declaration-only roots keep an empty requirement list. The
    /// caller cannot qualify a subset or attach an unrelated receiver node.
    pub(crate) fn with_source_field_requirements(
        self,
        requests: &[LocalFieldReadRequestV1],
    ) -> Option<Self> {
        if !self.source_fields.is_empty() {
            return None;
        }
        let prior_homes: Rc<[_]> = self.source_field_homes.clone().unwrap_or_else(|| {
            Rc::from(
                self.print_frame()
                    .map(|frame| frame.prior_homes())
                    .or_else(|| self.argument_frame().map(|frame| frame.prior_homes()))
                    .or_else(|| self.initializer_frame().map(|frame| frame.prior_homes()))
                    .or_else(|| {
                        self.rebind_frame()
                            .map(|frame| frame.evaluation().prior_homes())
                    })
                    .or_else(|| self.condition_frame().map(|frame| frame.prior_homes()))
                    .or_else(|| self.field_write_frame().map(|frame| frame.prior_homes()))
                    .unwrap_or(&[]),
            )
        });
        let transferred = self
            .evaluation()
            .map(|evaluation| {
                evaluation
                    .ordered_nodes()
                    .iter()
                    .copied()
                    .filter(|node| {
                        evaluation.transfers_node(*node)
                            && evaluation.direct_array_receiver_v1(*node).is_none()
                    })
                    .collect()
            })
            .unwrap_or_default();
        self.with_source_field_basis_v1(requests, prior_homes, &transferred)
    }
    /// Return owns its evaluation in TerminalReturnedSource. This existing plan
    /// retains only the SAME original field requirements and their Home basis.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn terminal_fields_v1(
        evaluation: &CheckedExpressionEvaluationV1,
        requests: &[LocalFieldReadRequestV1],
    ) -> Option<Self> {
        if !matches!(evaluation.source().root(), crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::Statement {
            role: crate::mir::resolved_value_profile::expression_root::StatementExpressionRoleV1::ReturnValue, .. })
            || !evaluation.reference_coverage_is_exact() { return None; }
        let plan = Self {
            source: Rc::clone(evaluation.source()),
            scalar_operations: Vec::new(),
            source_fields: Vec::new(),
            print_frame: None,
            argument_frame: None,
            initializer_frame: None,
            rebind_frame: None,
            condition_frame: None,
            loop_recipe: None,
            field_write: None,
            source_field_homes: None,
        };
        let transferred = evaluation
            .ordered_nodes()
            .iter()
            .copied()
            .filter(|node| {
                evaluation.transfers_node(*node)
                    && evaluation.direct_array_receiver_v1(*node).is_none()
            })
            .collect();
        plan.with_source_field_basis_v1(requests, Rc::from(evaluation.prior_homes()), &transferred)
    }
    fn with_source_field_basis_v1(
        mut self,
        requests: &[LocalFieldReadRequestV1],
        prior_homes: Rc<[LiveHomeV1]>,
        transferred: &std::collections::BTreeSet<usize>,
    ) -> Option<Self> {
        let mut seen = std::collections::BTreeSet::new();
        let mut fields = Vec::with_capacity(requests.len());
        for request in requests {
            let node = request.node_in_plan(&self.source)?;
            let field = ExpressionFieldRequirementV1 {
                plan: Rc::clone(&self.source),
                node,
                request: request.clone(),
                prior_homes: Rc::clone(&prior_homes),
            };
            if !field.is_consistent() || !seen.insert(node) {
                return None;
            }
            fields.push(field);
        }
        let original: std::collections::BTreeSet<_> = self
            .source
            .nodes()
            .iter()
            .enumerate()
            .filter(|(index, _)| !transferred.contains(index))
            .filter_map(|(index, node)| {
                matches!(
                    node.source(),
                    ExpressionNodeSourceV1::Shape(
                        crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { .. }
                    )
                )
                .then_some(index)
            })
            .collect();
        if seen != original {
            return None;
        }
        self.source_fields = fields;
        self.is_consistent().then_some(self)
    }
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn borrowed_return_reads_v1(
        source: Rc<ExpressionSourcePlanV1>,
        homes: &[LiveHomeV1],
        request: &LocalFieldReadRequestV1,
    ) -> Option<Self> {
        if source.has_opaque_obligations()
            || !matches!(source.root(), ExpressionPlanRootV1::Statement {
            role: crate::mir::resolved_value_profile::expression_root::StatementExpressionRoleV1::ReturnValue,
            value, .. } if value == request.site.site())
            || request.node_in_plan(&source) != Some(0)
        {
            return None;
        }
        Self::source_reads_v1(source, homes, std::slice::from_ref(request))
    }
    pub(super) fn has_borrowed_return_field_basis_v1(&self) -> bool {
        let [field] = self.source_fields.as_slice() else {
            return false;
        };
        field.node == 0
            && field.request.receiver == field.request.home
            && field.request.path.is_none()
            && field.request.received.is_none()
            && matches!(self.source.root(), ExpressionPlanRootV1::Statement {
                role: crate::mir::resolved_value_profile::expression_root::StatementExpressionRoleV1::ReturnValue,
                value, .. } if value == field.request.site.site())
    }
    fn source_reads_v1(
        source: Rc<ExpressionSourcePlanV1>,
        homes: &[LiveHomeV1],
        requests: &[LocalFieldReadRequestV1],
    ) -> Option<Self> {
        Self {
            source,
            scalar_operations: Vec::new(),
            source_fields: Vec::new(),
            print_frame: None,
            argument_frame: None,
            initializer_frame: None,
            rebind_frame: None,
            condition_frame: None,
            loop_recipe: None,
            field_write: None,
            source_field_homes: Some(Rc::from(homes)),
        }
        .with_source_field_requirements(requests)
    }
}
