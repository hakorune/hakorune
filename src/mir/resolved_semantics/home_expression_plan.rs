//! Whole-expression Home qualification and stable scalar loans of one source plan.
//! Source structure remains owned by resolved_value_profile; physical rows live later.
use super::TextResidenceAcquisitionV1;
use crate::mir::dynamic_operator_contract::VerifiedDynamicOperatorExecutionEnvelopeV1;
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use crate::mir::resolved_semantics::{
    home_new_prefix::LocalFieldReadRequestV1, FunctionOwnerIdV1, ResolvedBinaryExpressionSourceV1,
    ResolvedBinaryOperatorV1, ResolvedInitializerRelationV1, ResolvedLiteralSourceV1,
    SourceStmtSiteV1,
};
use crate::mir::resolved_value_profile::expression_plan::{
    ExpressionNodeSourceV1, ExpressionPlanRootV1, ExpressionSourcePlanV1,
};
use std::rc::Rc;
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CheckedExpressionOperationLoanV1 {
    plan: Rc<ExpressionSourcePlanV1>,
    node: usize,
    proof: CheckedExpressionOperationProofV1,
}
/// Raw nullable identity is a separate closed law, never a tagged operator.
#[derive(Debug, Clone, PartialEq, Eq)]
enum CheckedExpressionOperationProofV1 {
    Dynamic(Box<[&'static VerifiedDynamicOperatorExecutionEnvelopeV1]>),
    Scalar(Rc<ExpressionScalarOperationLoanV1>),
    NullableFieldNull { reference: usize, null: usize, request: LocalFieldReadRequestV1 },
    NullableNull {
        reference: usize,
        null: usize,
        frame: Rc<super::MapFieldGetLocalV1>,
    },
}
impl CheckedExpressionOperationLoanV1 {
    pub(crate) fn node(&self) -> usize {
        self.node
    }
    pub(crate) fn source(&self) -> &Rc<ExpressionSourcePlanV1> {
        &self.plan
    }
    pub(crate) fn envelopes(&self) -> &[&'static VerifiedDynamicOperatorExecutionEnvelopeV1] {
        match &self.proof {
            CheckedExpressionOperationProofV1::Dynamic(rows) => rows,
            _ => &[],
        }
    }
    pub(crate) fn single_envelope(
        &self,
    ) -> Option<&'static VerifiedDynamicOperatorExecutionEnvelopeV1> {
        match self.envelopes() {
            [envelope] => Some(*envelope),
            _ => None,
        }
    }
    pub(crate) fn normal_result_v1(
        &self,
    ) -> Option<crate::mir::dynamic_operator_contract::DynamicOperatorNormalResultV1> {
        use crate::mir::dynamic_operator_contract::DynamicOperatorNormalResultV1 as ResultKind;
        match &self.proof {
            CheckedExpressionOperationProofV1::Scalar(_) => self
                .integer_operator_v1()
                .map(|_| ResultKind::NormalInteger),
            CheckedExpressionOperationProofV1::Dynamic(envelopes) => {
                let result = envelopes.first()?.normal_result();
                envelopes
                    .iter()
                    .all(|envelope| envelope.normal_result() == result)
                    .then_some(result)
            }
            CheckedExpressionOperationProofV1::NullableNull { .. }
            | CheckedExpressionOperationProofV1::NullableFieldNull { .. } => None,
        }
    }
    pub(crate) fn nullable_field_null_v1(&self) -> Option<(usize, usize, &LocalFieldReadRequestV1)> {
        let CheckedExpressionOperationProofV1::NullableFieldNull { reference, null, request } = &self.proof else { return None; };
        let binary = self.plan.nodes().get(self.node)?.binary()?;
        let read = self.plan.nodes().get(*reference)?;
        let empty = self.plan.nodes().get(*null)?;
        (request.node_in_plan(&self.plan) == Some(*reference)
            && request.is_nullable_object_comparison_v1(&self.plan)
            && binary.operator() == ResolvedBinaryOperatorV1::Equal
            && matches!(empty.source(), ExpressionNodeSourceV1::Literal(ResolvedLiteralSourceV1::Null))
            && ((read.site() == binary.lhs() && empty.site() == binary.rhs())
                || (read.site() == binary.rhs() && empty.site() == binary.lhs())))
            .then_some((*reference, *null, request))
    }
    pub(crate) fn nullable_null_v1(
        &self,
    ) -> Option<(usize, usize, &Rc<super::MapFieldGetLocalV1>)> {
        let CheckedExpressionOperationProofV1::NullableNull {
            reference,
            null,
            frame,
        } = &self.proof
        else {
            return None;
        };
        let binary = self.plan.nodes().get(self.node)?.binary()?;
        let read = self.plan.nodes().get(*reference)?;
        let empty = self.plan.nodes().get(*null)?;
        if binary.operator() != ResolvedBinaryOperatorV1::Equal
            || frame.site().owner() != self.plan.owner()
            || !matches!(read.source(), ExpressionNodeSourceV1::Reference(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding)) if *binding == frame.binding())
            || !matches!(
                empty.source(),
                ExpressionNodeSourceV1::Literal(ResolvedLiteralSourceV1::Null)
            )
            || !((read.site() == binary.lhs() && empty.site() == binary.rhs())
                || (read.site() == binary.rhs() && empty.site() == binary.lhs()))
        {
            return None;
        }
        Some((*reference, *null, frame))
    }
    pub(crate) fn matches_nullable_null_reference_v1(
        &self,
        references: &std::collections::BTreeMap<usize, CheckedExpressionReferenceV1>,
    ) -> bool {
        let Some((reference, _, frame)) = self.nullable_null_v1() else {
            return false;
        };
        matches!(references.get(&reference), Some(CheckedExpressionReferenceV1::StoredMapGet(original))
            if Rc::ptr_eq(original, frame) && references[&reference].matches_node(&self.plan, reference))
    }
    /// One canonical numeric operation, tied to its original source node.
    /// The writer and independent physical matcher consume this same loan.
    pub(crate) fn integer_operator_v1(&self) -> Option<crate::mir::BinaryOp> {
        use crate::mir::dynamic_operator_contract::{
            DynamicOperatorFamilyV1 as Family, DynamicOperatorNormalResultV1 as ResultKind,
            DynamicOperatorValueClassV1 as Class,
        };
        if let CheckedExpressionOperationProofV1::Scalar(source) = &self.proof {
            if !Rc::ptr_eq(source.plan(), &self.plan)
                || self
                    .plan
                    .nodes()
                    .get(self.node)
                    .and_then(|node| node.binary())
                    != Some(source.binary())
            {
                return None;
            }
            return source.integer_operator_v1();
        }
        let envelope = self.single_envelope()?;
        let domain = envelope.domain();
        if domain.left() != Class::NormalInteger
            || domain.right() != Class::NormalInteger
            || envelope.normal_result() != ResultKind::NormalInteger
        {
            return None;
        }
        match (
            self.plan.nodes().get(self.node)?.binary()?.operator(),
            domain.family(),
        ) {
            (ResolvedBinaryOperatorV1::Add, Family::Add) => Some(crate::mir::BinaryOp::Add),
            (ResolvedBinaryOperatorV1::Multiply, Family::Mul) => Some(crate::mir::BinaryOp::Mul),
            _ => None,
        }
    }
}

#[path = "home_expression_leaves.rs"]
mod leaves;
pub(crate) use leaves::ExpressionCallSourceV1;
pub(super) use leaves::{
    prepare_expression_calls_v1, ExpressionLeafProfileV1, PreparedExpressionCallsV1,
};
#[path = "home_field_write_expression.rs"]
mod field_write_expression;
pub(super) use field_write_expression::qualify_field_write_expression_v1;
pub(crate) use field_write_expression::QualifiedFieldWriteRhsV1;
#[path = "home_bool_condition.rs"]
mod condition;
pub(crate) use condition::{
    qualify_bool_condition_v1, qualify_condition_expression_v1, CheckedBoolConditionFrameV1,
};
#[path = "home_argument_expression.rs"]
mod argument_expression;
#[path = "home_expression_operations.rs"]
mod operations;
pub(crate) use argument_expression::{
    qualify_argument_expression_v1, CheckedArgumentExpressionFrameV1,
};
#[path = "home_initializer_expression.rs"]
mod initializer_expression;
pub(crate) use initializer_expression::{
    qualify_initializer_expression_v1, CheckedInitializerExpressionFrameV1,
};
#[path = "home_binding_rebind_expression.rs"]
mod binding_rebind_expression;
pub(crate) use binding_rebind_expression::{
    qualify_binding_rebind_expression_v1, qualify_binding_rebind_expression_with_loop_dynamic_v1,
    BindingRebindDestinationV1, CheckedBindingRebindFrameV1,
};
#[path = "home_expression_evaluation.rs"]
mod evaluation;
pub(super) use evaluation::{prepare_value_evaluation_v1, qualify_text_evaluation_v1};
pub(crate) use evaluation::{
    CheckedExpressionEvaluationV1, CheckedExpressionReferenceV1, CheckedScalarReferenceV1,
};
#[path = "home_print_expression.rs"]
mod print_expression;
pub(super) use print_expression::qualify_statement_evaluation_v1;
pub(crate) use print_expression::{qualify_print_expression_v1, CheckedPrintExpressionFrameV1};

#[path = "home_borrowed_field_write.rs"]
mod field_write;
pub(crate) use field_write::{CheckedFieldWriteFrameV1, FieldWritePlanV1};

#[path = "home_expression_scalar_operation.rs"]
mod scalar_operation;
pub(crate) use scalar_operation::ExpressionScalarOperationLoanV1;

/// A mandatory source read of this SAME complete expression, independent of
/// whether its physical row or optional leaf loan later survives. No layout,
/// field value or Home permission is inferred from the request itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExpressionFieldRequirementV1 {
    plan: Rc<ExpressionSourcePlanV1>,
    node: usize,
    request: LocalFieldReadRequestV1,
    prior_homes: Rc<[LiveHomeV1]>,
}
impl ExpressionFieldRequirementV1 {
    pub(crate) fn plan(&self) -> &Rc<ExpressionSourcePlanV1> {
        &self.plan
    }
    pub(crate) fn request(&self) -> &LocalFieldReadRequestV1 {
        &self.request
    }
    pub(crate) fn prior_homes(&self) -> &[LiveHomeV1] {
        &self.prior_homes
    }
    fn is_consistent(&self) -> bool {
        self.request.site.owner() == self.plan.owner()
            && self.request.home.owner() == self.plan.owner()
            && self.request.node_in_plan(&self.plan) == Some(self.node)
    }
}

/// An initializer keeps either its existing evaluated frame or an exact
/// ownership transfer of an already acquired Home.
#[derive(Debug, Clone, PartialEq, Eq)]
enum InitializerFrameV1 {
    Evaluation(Rc<CheckedInitializerExpressionFrameV1>),
    Move {
        source: Rc<ExpressionSourcePlanV1>,
        unary: crate::mir::resolved_semantics::ResolvedUnaryExpressionSourceV1,
        from: crate::mir::resolved_semantics::BindingRefV1,
        acquisition: crate::mir::resolved_semantics::OwnedExprSiteV1,
    },
}

// Like the previous flow payload, equality compares immutable original facts.
// It never replaces the physical consumer's explicit Rc membership checks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct HomeExpressionPlanV1 {
    source: Rc<ExpressionSourcePlanV1>,
    scalar_operations: Vec<Rc<ExpressionScalarOperationLoanV1>>,
    source_fields: Vec<ExpressionFieldRequirementV1>,
    print_frame: Option<Rc<CheckedPrintExpressionFrameV1>>,
    argument_frame: Option<Rc<CheckedArgumentExpressionFrameV1>>,
    initializer_frame: Option<InitializerFrameV1>,
    rebind_frame: Option<Rc<CheckedBindingRebindFrameV1>>,
    condition_frame: Option<Rc<CheckedBoolConditionFrameV1>>,
    loop_recipe: Option<Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>>,
    field_write: Option<FieldWritePlanV1>,
    source_field_homes: Option<Rc<[LiveHomeV1]>>,
}
impl HomeExpressionPlanV1 {
    pub(super) fn move_initializer(
        source: Rc<ExpressionSourcePlanV1>,
        unary: &crate::mir::resolved_semantics::ResolvedUnaryExpressionSourceV1,
        from: crate::mir::resolved_semantics::BindingRefV1,
        acquisition: crate::mir::resolved_semantics::OwnedExprSiteV1,
    ) -> Option<Self> {
        let initializer = source.root().initializer()?;
        if initializer.binding().owner() != source.owner()
            || from.owner() != source.owner()
            || acquisition.owner() != source.owner()
            || source.nodes().len() != 2
            || !matches!(source.nodes()[0].source(), ExpressionNodeSourceV1::Unary)
            || unary.operator() != crate::mir::resolved_semantics::ResolvedUnaryOperatorV1::Move
            || unary.site() != source.nodes()[0].site()
            || unary.operand() != source.nodes()[1].site()
            || !matches!(source.nodes()[1].source(), ExpressionNodeSourceV1::Reference(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding)) if *binding == from)
        {
            return None;
        }
        let plan = Self {
            source: Rc::clone(&source),
            scalar_operations: Vec::new(),
            source_fields: Vec::new(),
            print_frame: None,
            argument_frame: None,
            initializer_frame: Some(InitializerFrameV1::Move { source, unary: unary.clone(), from, acquisition }),
            rebind_frame: None,
            condition_frame: None,
            loop_recipe: None,
            field_write: None,
            source_field_homes: None,
        };
        plan.is_consistent().then_some(plan)
    }
    /// Called only after the original scalar preflight proves both Integer
    /// children. The parent still must close its complete field/call batch
    /// before installing this wrapper in the persistent Home flow.
    pub(super) fn scalar_root(
        source: Rc<ExpressionSourcePlanV1>,
        binary: &ResolvedBinaryExpressionSourceV1,
        positive_divisor: Option<i64>,
    ) -> Option<Self> {
        let loan = ExpressionScalarOperationLoanV1::issue_root(
            Rc::clone(&source),
            binary,
            positive_divisor,
        )?;
        Some(Self {
            source,
            scalar_operations: vec![loan],
            source_fields: Vec::new(),
            print_frame: None,
            argument_frame: None,
            initializer_frame: None,
            rebind_frame: None,
            condition_frame: None,
            loop_recipe: None,
            field_write: None,
            source_field_homes: None,
        })
    }
    /// Whole initializer preflight supplies value-class proof separately. This
    /// frame retains only original read requirements and evaluation-time Homes.
    pub(super) fn initializer_reads(
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
    pub(super) fn borrowed_argument_reads_v1(
        source: Rc<ExpressionSourcePlanV1>, homes: &[LiveHomeV1],
        requests: &[LocalFieldReadRequestV1],
    ) -> Option<Self> {
        if source.has_opaque_obligations()
            || !matches!(source.root(), ExpressionPlanRootV1::CallArgument { .. })
            || requests.len() != 1 || requests[0].node_in_plan(&source) != Some(0)
            || !matches!(requests[0].received,
                Some(crate::mir::resolved_semantics::home_new_prefix::ReceivedFieldReadRequestV1::StoredMapGet { .. }))
        { return None; }
        Self {
            source, scalar_operations: Vec::new(), source_fields: Vec::new(),
            print_frame: None, argument_frame: None, initializer_frame: None,
            rebind_frame: None, condition_frame: None, loop_recipe: None,
            field_write: None, source_field_homes: Some(Rc::from(homes)),
        }.with_source_field_requirements(requests)
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
                    .filter(|node| evaluation.transfers_node(*node) && evaluation.direct_array_receiver_v1(*node).is_none())
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
            .filter(|node| evaluation.transfers_node(*node) && evaluation.direct_array_receiver_v1(*node).is_none())
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
    /// The same completed evaluation, independent of its existing purpose.
    /// Scalar field RHS stays source-only; Text RHS borrows this same evaluator.
    pub(crate) fn evaluation(&self) -> Option<&CheckedExpressionEvaluationV1> {
        self.argument_frame()
            .map(|frame| frame.evaluation())
            .or_else(|| self.initializer_frame().map(|frame| frame.evaluation()))
            .or_else(|| self.rebind_frame().map(|frame| frame.evaluation()))
            .or_else(|| self.condition_frame().map(|frame| frame.evaluation()))
            .or_else(|| self.print_frame().map(|frame| frame.evaluation()))
            .or_else(|| self.field_write_rhs().and_then(|rhs| rhs.evaluation()))
    }
    pub(crate) fn field_write_plan(&self) -> Option<&FieldWritePlanV1> {
        self.field_write.as_ref()
    }
    pub(crate) fn field_write_frame(&self) -> Option<&Rc<CheckedFieldWriteFrameV1>> {
        self.field_write.as_ref().map(|plan| &plan.frame)
    }
    pub(crate) fn field_write_rhs(&self) -> Option<&Rc<QualifiedFieldWriteRhsV1>> {
        self.field_write.as_ref().and_then(|plan| plan.rhs.as_ref())
    }
    /// Existing formal consumers cannot borrow a LOCAL permission.
    pub(crate) fn borrowed_field_write_frame(&self) -> Option<&Rc<CheckedFieldWriteFrameV1>> {
        self.field_write_frame().filter(|frame| frame.is_borrowed())
    }
    pub(crate) fn initializer_frame(&self) -> Option<&Rc<CheckedInitializerExpressionFrameV1>> {
        match self.initializer_frame.as_ref()? {
            InitializerFrameV1::Evaluation(frame) => Some(frame),
            InitializerFrameV1::Move { .. } => None,
        }
    }
    pub(crate) fn move_initializer_v1(&self) -> Option<(&Rc<ExpressionSourcePlanV1>, crate::mir::resolved_semantics::BindingRefV1, &crate::mir::resolved_semantics::OwnedExprSiteV1)> {
        match self.initializer_frame.as_ref()? {
            InitializerFrameV1::Move { source, from, acquisition, .. } => Some((source, *from, acquisition)),
            InitializerFrameV1::Evaluation(_) => None,
        }
    }
    pub(crate) fn rebind_frame(&self) -> Option<&Rc<CheckedBindingRebindFrameV1>> {
        self.rebind_frame.as_ref()
    }
    pub(crate) fn argument_frame(&self) -> Option<&Rc<CheckedArgumentExpressionFrameV1>> {
        self.argument_frame.as_ref()
    }
    pub(crate) fn condition_frame(&self) -> Option<&Rc<CheckedBoolConditionFrameV1>> {
        self.condition_frame.as_ref()
    }
    pub(crate) fn loop_recipe_v2(
        &self,
    ) -> Option<&Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>> {
        self.loop_recipe.as_ref()
    }
    /// Refine only the original installed Loop condition; no second slot or
    /// equal-frame reconstruction is allowed. This is no Home After grant.
    pub(super) fn attach_loop_recipe_v2(
        &mut self,
        product: Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>,
    ) -> bool {
        if self.loop_recipe.is_some() || !self.matches_loop_recipe_v2(&product) {
            return false;
        }
        self.loop_recipe = Some(product);
        true
    }
    fn matches_loop_recipe_v2(
        &self,
        product: &crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2,
    ) -> bool {
        matches!(self.source.root(), crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::LoopCondition { .. })
            && Rc::ptr_eq(&self.source, product.source.condition.source())
            && self.condition_frame.as_ref().is_some_and(|frame| Rc::ptr_eq(frame, &product.source.condition))
            && product.source.source.site() == self.source.root().statement()
            && product.source.source.owner() == self.source.owner()
    }
    pub(crate) fn print_frame(&self) -> Option<&Rc<CheckedPrintExpressionFrameV1>> {
        self.print_frame.as_ref()
    }
    pub(crate) fn source_fields(&self) -> &[ExpressionFieldRequirementV1] {
        &self.source_fields
    }
    pub(crate) fn source(&self) -> &Rc<ExpressionSourcePlanV1> {
        &self.source
    }
    pub(crate) fn scalar_operations(
        &self,
    ) -> impl Iterator<Item = &Rc<ExpressionScalarOperationLoanV1>> {
        self.scalar_operations.iter()
    }
    pub(super) fn is_consistent(&self) -> bool {
        let mut seen = std::collections::BTreeSet::new();
        [
            self.initializer_frame.is_some(),
            self.rebind_frame.is_some(),
            self.argument_frame.is_some(),
            self.print_frame.is_some(),
            self.condition_frame.is_some(),
            self.field_write.is_some(),
            self.source_field_homes.is_some(),
        ]
        .into_iter()
        .filter(|present| *present)
        .count()
            <= 1
            && self.source_field_homes.as_ref().is_none_or(|homes| {
                (self.source.root().initializer().is_some()
                    || matches!(self.source.root(), ExpressionPlanRootV1::CallArgument { .. }))
                    && homes.iter().all(|home| home.owner() == self.source.owner())
                    && homes
                        .iter()
                        .map(LiveHomeV1::binding)
                        .collect::<std::collections::BTreeSet<_>>()
                        .len()
                        == homes.len()
            })
            && self.field_write.as_ref().is_none_or(|plan| {
                Rc::ptr_eq(plan.frame.source(), &self.source)
                    && match (&plan.rhs, plan.frame.is_borrowed()) {
                        (None, true) => true,
                        (Some(rhs), false) => rhs.is_consistent(&self.source),
                        _ => false,
                    }
            })
            && self
                .loop_recipe
                .as_ref()
                .is_none_or(|product| self.matches_loop_recipe_v2(product))
            && self
                .condition_frame
                .as_ref()
                .is_none_or(|frame| Rc::ptr_eq(frame.source(), &self.source))
            && self.argument_frame.as_ref().is_none_or(|frame| {
                Rc::ptr_eq(frame.source(), &self.source)
                    && Rc::ptr_eq(frame.source_loan().plan(), &self.source)
            })
            && self
                .print_frame
                .as_ref()
                .is_none_or(|frame| Rc::ptr_eq(frame.source(), &self.source))
            && self.initializer_frame.as_ref().is_none_or(|frame| match frame {
                InitializerFrameV1::Evaluation(frame) => Rc::ptr_eq(frame.source(), &self.source),
                InitializerFrameV1::Move { source, unary, from, acquisition } => {
                    Rc::ptr_eq(source, &self.source)
                        && from.owner() == source.owner()
                        && acquisition.owner() == source.owner()
                        && source.nodes().len() == 2
                        && unary.operator() == crate::mir::resolved_semantics::ResolvedUnaryOperatorV1::Move
                        && unary.site() == source.nodes()[0].site()
                        && unary.operand() == source.nodes()[1].site()
                }
            })
            && self
                .rebind_frame
                .as_ref()
                .is_none_or(|frame| Rc::ptr_eq(frame.source(), &self.source))
            && self
                .source_fields
                .iter()
                .all(|field| Rc::ptr_eq(&self.source, field.plan()) && field.is_consistent())
            && self.scalar_operations.iter().all(|loan| {
                Rc::ptr_eq(&self.source, loan.plan())
                    && self
                        .source
                        .nodes()
                        .get(loan.node)
                        .is_some_and(|node| node.binary().is_some())
                    && seen.insert(loan.node)
            })
    }
}

// Root-flow expression access and refinement stay with the expression owner.
impl super::RootHomeFlow {
    pub(crate) fn source_expression_fields(
        &self,
    ) -> impl Iterator<Item = &ExpressionFieldRequirementV1> {
        self.expression_plans
            .iter()
            .flat_map(HomeExpressionPlanV1::source_fields)
    }
    /// Refine an existing whole expression, preserving the original stable
    /// operation loans. No equal-source reconstruction can replace its Rc.
    pub(crate) fn refine_expression_plan(&mut self, refined: HomeExpressionPlanV1) -> bool {
        let Some(original) = self
            .expression_plans
            .iter_mut()
            .find(|old| std::rc::Rc::ptr_eq(old.source(), refined.source()))
        else {
            return false;
        };
        if !refined.is_consistent()
            || !original.source_fields().is_empty()
            || match (&original.source_field_homes, &refined.source_field_homes) {
                (None, None) => false,
                (Some(a), Some(b)) => !Rc::ptr_eq(a, b),
                _ => true,
            }
            || match (original.print_frame(), refined.print_frame()) {
                (None, None) => false,
                (Some(a), Some(b)) => !std::rc::Rc::ptr_eq(a, b),
                _ => true,
            }
            || match (original.initializer_frame(), refined.initializer_frame()) {
                (None, None) => false,
                (Some(a), Some(b)) => !Rc::ptr_eq(a, b),
                _ => true,
            }
            || match (original.move_initializer_v1(), refined.move_initializer_v1()) {
                (None, None) => false,
                (Some((a, from_a, acquisition_a)), Some((b, from_b, acquisition_b))) =>
                    !Rc::ptr_eq(a, b) || from_a != from_b || acquisition_a != acquisition_b,
                _ => true,
            }
            || match (original.rebind_frame(), refined.rebind_frame()) {
                (None, None) => false,
                (Some(a), Some(b)) => !Rc::ptr_eq(a, b),
                _ => true,
            }
            || match (original.argument_frame(), refined.argument_frame()) {
                (None, None) => false,
                (Some(a), Some(b)) => !Rc::ptr_eq(a, b),
                _ => true,
            }
            || match (original.condition_frame(), refined.condition_frame()) {
                (None, None) => false,
                (Some(a), Some(b)) => !std::rc::Rc::ptr_eq(a, b),
                _ => true,
            }
            || match (original.loop_recipe_v2(), refined.loop_recipe_v2()) {
                (None, None) => false,
                (Some(a), Some(b)) => !Rc::ptr_eq(a, b),
                _ => true,
            }
            || match (original.field_write_frame(), refined.field_write_frame()) {
                (None, None) => false,
                (Some(a), Some(b)) => !std::rc::Rc::ptr_eq(a, b),
                _ => true,
            }
            || match (original.field_write_rhs(), refined.field_write_rhs()) {
                (None, None) => false,
                (Some(a), Some(b)) => !std::rc::Rc::ptr_eq(a, b),
                _ => true,
            }
            || original.scalar_operations().count() != refined.scalar_operations().count()
            || !original
                .scalar_operations()
                .zip(refined.scalar_operations())
                .all(|(a, b)| std::rc::Rc::ptr_eq(a, b))
        {
            return false;
        }
        *original = refined;
        true
    }
    pub(crate) fn expression_plans(&self) -> &[HomeExpressionPlanV1] {
        &self.expression_plans
    }
    pub(crate) fn scalar_operations(
        &self,
    ) -> impl Iterator<Item = &std::rc::Rc<ExpressionScalarOperationLoanV1>> {
        self.expression_plans
            .iter()
            .flat_map(HomeExpressionPlanV1::scalar_operations)
    }
}

/// A scoped transaction over the existing shared Home expression inventory.
/// Approval stays private until the complete field batch succeeds; holding the
/// one mutable inventory borrow prevents a sibling walk from changing its slot.
/// Commit consumes the approved SAME source/frame without a second decision.
pub(super) struct ExpressionPlanInstallationV1<'a, 'source> {
    locals: &'a super::local_flow::PrefixLocalFlow<'source>,
    owner: FunctionOwnerIdV1,
    plans: std::cell::RefMut<'a, Vec<HomeExpressionPlanV1>>,
    approved: Option<HomeExpressionPlanV1>,
}
impl<'a, 'source> ExpressionPlanInstallationV1<'a, 'source> {
    pub(super) fn new(
        owner: FunctionOwnerIdV1,
        locals: &'a super::local_flow::PrefixLocalFlow<'source>,
        plans: std::cell::RefMut<'a, Vec<HomeExpressionPlanV1>>,
    ) -> Self {
        Self {
            owner,
            locals,
            plans,
            approved: None,
        }
    }
    pub(super) fn approve(&mut self, plan: &HomeExpressionPlanV1) -> bool {
        if self.approved.is_some()
            || plan.source().owner() != self.owner
            || !plan.is_consistent()
            || plan.evaluation().is_some_and(|evaluation| evaluation.cores().values()
                .filter_map(|core| core.field_array_access_v1()).any(|count| !self.locals.has_field_array_access_source_v1(count)))
            || self.plans.iter().any(|old| {
                old.source().root().statement() == plan.source().root().statement()
                    && old.source().nodes()[0].site() == plan.source().nodes()[0].site()
            })
        {
            return false;
        }
        self.approved = Some(plan.clone());
        true
    }
    /// Reuse the Call slot's canonical equal frame without reopening approval
    /// or deciding field/Home semantics after the final field batch.
    pub(super) fn retain_approved_argument(
        &mut self,
        frame: Rc<CheckedArgumentExpressionFrameV1>,
    ) -> bool {
        let Some(plan) = self.approved.as_mut() else {
            return false;
        };
        if !Rc::ptr_eq(plan.source(), frame.source())
            || plan
                .argument_frame()
                .is_none_or(|original| original.as_ref() != frame.as_ref())
        {
            return false;
        }
        plan.argument_frame = Some(frame);
        true
    }
    pub(super) fn commit(mut self) {
        self.plans.push(
            self.approved
                .take()
                .expect("only approved expression installation commits"),
        );
    }
}

#[cfg(test)]
mod installation_tests {
    use super::*;
    use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
    use crate::mir::resolved_semantics::home_new_prefix::local_flow::PrefixLocalFlow;
    #[test]
    fn complete_expression_installation_publishes_only_after_the_final_field_batch() {
        let parsed = crate::parser::NyashParser::parse_from_string(
            "function probe() { print(\"猫\") return 0 }",
        )
        .unwrap();
        let crate::ast::ASTNode::Program { statements, .. } = parsed else {
            panic!("Program")
        };
        let function = statements
            .into_iter()
            .find(|node| matches!(node, crate::ast::ASTNode::FunctionDeclaration { .. }))
            .unwrap();
        let unit = VerifiedResolvedSourceUnitV1::resolve_function(function).unwrap();
        let input = unit.root_function_input().unwrap();
        let shape = input.body_shape().unwrap();
        let statement = shape
            .statements()
            .iter()
            .find(|statement| {
                matches!(
                    statement,
                    crate::mir::resolved_semantics::BodyStatementShapeV1::Print { .. }
                )
            })
            .unwrap();
        let ledger = input
            .forest()
            .callable_source_ledger(input.owner())
            .unwrap();
        let source = crate::mir::resolved_value_profile::expression_plan::statement_expression_source_plan_v1(
            &ledger, shape, statement.site()).unwrap().unwrap();
        let flow = PrefixLocalFlow::new(input);
        let sibling = flow.clone();
        for batch_succeeds in [false, true] {
            let mut installation = flow.expression_installation();
            let mut batches = 0;
            let plan = qualify_print_expression_v1::<()>(
                Rc::clone(&source),
                &[],
                &mut |_| None,
                &mut |_| None,
                &mut |_| None,
                &mut |_| None,
                &mut |_| None,
                &mut |_| Ok(None),
                &mut |plan| installation.approve(plan),
                &mut |fields, _| {
                    assert!(fields.is_empty());
                    batches += 1;
                    Ok(batch_succeeds)
                },
            )
            .unwrap();
            assert_eq!(batches, 1);
            assert_eq!(plan.is_some(), batch_succeeds);
            if let Some(plan) = plan {
                let original_frame = Rc::clone(plan.print_frame().unwrap());
                installation.commit();
                let installed = sibling.expression_plans();
                assert_eq!(installed.len(), 1);
                assert!(Rc::ptr_eq(installed[0].source(), &source));
                assert!(Rc::ptr_eq(
                    installed[0].print_frame().unwrap(),
                    &original_frame
                ));
                assert!(installed[0]
                    .print_frame()
                    .unwrap()
                    .owned_result_nodes()
                    .is_empty());
                // Membership comes from the installed source frame even when
                // no call, field, temporary or physical observation exists.
                use crate::mir::resolved_semantics::home_new_prefix::{
                    RootHomeExitV1, RootHomeFlow,
                };
                let root = RootHomeFlow {
                    named_arrays: Vec::new(),
                    exits: Default::default(),
                    maps: Vec::new(),
                    local_calls: Vec::new(),
                    expression_plans: installed,
                };
                let expected = root.lifecycle_prefixes(&[]).unwrap();
                assert_eq!(expected.len(), 1);
                let covered = RootHomeExitV1::issue(
                    Box::new([]),
                    Box::new([]),
                    vec![source.root().statement().clone()].into_boxed_slice(),
                );
                let sibling = RootHomeExitV1::issue(Box::new([]), Box::new([]), Box::new([]));
                assert!(expected[0].covered_by(&covered));
                assert!(!expected[0].covered_by(&sibling));
            } else {
                drop(installation);
                assert!(
                    sibling.expression_plans().is_empty(),
                    "failed field batch publishes no frame"
                );
            }
        }
        let mut installation = sibling.expression_installation();
        let mut batches = 0;
        assert!(qualify_print_expression_v1::<()>(
            source,
            &[],
            &mut |_| None,
            &mut |_| None,
            &mut |_| None,
            &mut |_| None,
            &mut |_| None,
            &mut |_| Ok(None),
            &mut |plan| installation.approve(plan),
            &mut |_, _| {
                batches += 1;
                Ok(true)
            }
        )
        .unwrap()
        .is_none());
        assert_eq!(
            batches, 0,
            "duplicate source slot rejects before a field batch"
        );
        // LOCAL i64 demand validates the whole RHS even without Sub/Div.
        // This is expression preparation, not LOCAL store/publication permission.
        for (rhs, integer_reference, eligible) in [
            ("7", false, true),
            ("2 + 3", false, true),
            ("n", true, true),
            ("n", false, false),
            ("true", false, false),
            ("\"7\"", false, false),
            ("\"text\" + 1", false, false),
            ("null", false, false),
            ("1 - 2", false, true),
            ("6 / 2", false, true),
            ("6 / 0", false, false),
            ("2 * 3", false, true),
            ("6 % 2", false, true),
            ("6 % 0", false, true),
            ("3 & 1", false, true),
            ("2 << 1", false, true),
        ] {
            for batch_succeeds in [false, true] {
                let parsed = crate::parser::NyashParser::parse_from_string(&format!(
                    "function probe(p, n) {{ p.value = {rhs} return p }}"
                ))
                .unwrap();
                let crate::ast::ASTNode::Program { mut statements, .. } = parsed else {
                    panic!("Program")
                };
                let unit =
                    VerifiedResolvedSourceUnitV1::resolve_function(statements.remove(0)).unwrap();
                let input = unit.root_function_input().unwrap();
                let shape = input.body_shape().unwrap();
                let ledger = input
                    .forest()
                    .callable_source_ledger(input.owner())
                    .unwrap();
                let source = crate::mir::resolved_value_profile::expression_plan::assignment_expression_source_plan_v1(
                    &ledger, shape, &shape.assignment_sources()[0]).unwrap();
                let flow = PrefixLocalFlow::new(input);
                let mut installation = flow.expression_installation();
                let mut batches = 0;
                let plan = qualify_field_write_expression_v1::<()>(Rc::clone(&source), true, None,
                    &mut |_| integer_reference.then_some(crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind::Integer),
                    &mut |_| None, &mut |_| None, &mut |_| None, &mut |plan| installation.approve(plan),
                    &mut |requests, _| { assert!(requests.is_empty()); batches += 1; Ok(batch_succeeds) }).unwrap();
                assert_eq!(plan.is_some(), eligible && batch_succeeds, "RHS {rhs}");
                assert_eq!(
                    batches,
                    usize::from(eligible),
                    "reject wrong RHS before field staging"
                );
                if plan.is_some() {
                    installation.commit();
                } else {
                    drop(installation);
                }
                let installed = flow.expression_plans();
                assert_eq!(installed.len(), usize::from(eligible && batch_succeeds));
                if let Some(plan) = installed.first() {
                    assert!(Rc::ptr_eq(plan.source(), &source));
                }
            }
        }
        // A borrowed write retains its original assignment and allocation-order
        // snapshot. This frame alone never issues a physical store capability.
        {
            let parsed = crate::parser::NyashParser::parse_from_string(
                "function probe(p, n) { p.value = n return p }",
            )
            .unwrap();
            let crate::ast::ASTNode::Program { mut statements, .. } = parsed else {
                panic!("Program")
            };
            let unit =
                VerifiedResolvedSourceUnitV1::resolve_function(statements.remove(0)).unwrap();
            let input = unit.root_function_input().unwrap();
            let shape = input.body_shape().unwrap();
            let assignment = &shape.assignment_sources()[0];
            let crate::mir::resolved_semantics::ResolvedAssignmentTargetV1::FieldWrite { receiver } =
                input
                    .function()
                    .assignment_target(assignment.target_site())
                    .unwrap()
            else {
                panic!("field write")
            };
            let binding = |site| match input.function().variable_ref(site).unwrap() {
                crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding) => binding,
                _ => panic!("local source"),
            };
            let homes = [
                LiveHomeV1::Stable(binding(receiver)),
                LiveHomeV1::Stable(binding(assignment.value_site())),
            ];
            let ledger = input
                .forest()
                .callable_source_ledger(input.owner())
                .unwrap();
            let source = crate::mir::resolved_value_profile::expression_plan::assignment_expression_source_plan_v1(
                &ledger, shape, assignment).unwrap();
            assert!(HomeExpressionPlanV1::borrowed_field_write(
                Rc::clone(&source),
                &[homes[0].clone(), homes[0].clone()]
            )
            .is_none());
            let plan =
                HomeExpressionPlanV1::borrowed_field_write(Rc::clone(&source), &homes).unwrap();
            let frame = Rc::clone(plan.borrowed_field_write_frame().unwrap());
            assert!(Rc::ptr_eq(frame.source(), &source));
            assert_eq!(frame.prior_homes(), &homes);
            assert!(plan.print_frame().is_none() && plan.condition_frame().is_none());
            let flow = PrefixLocalFlow::new(input);
            let mut installation = flow.expression_installation();
            assert!(installation.approve(&plan));
            drop(installation);
            assert!(flow.expression_plans().is_empty());
            let mut installation = flow.expression_installation();
            assert!(installation.approve(&plan));
            installation.commit();
            let installed = flow.expression_plans();
            assert!(Rc::ptr_eq(
                installed[0].borrowed_field_write_frame().unwrap(),
                &frame
            ));
            use crate::mir::resolved_semantics::home_new_prefix::{
                LifecyclePrefixSourceV1, RootHomeExitV1, RootHomeFlow,
            };
            let root = RootHomeFlow {
                named_arrays: Vec::new(),
                exits: Default::default(),
                maps: Vec::new(),
                local_calls: Vec::new(),
                expression_plans: installed,
            };
            let prefixes = root.lifecycle_prefixes(&[]).unwrap();
            assert_eq!(
                prefixes.len(),
                1,
                "an original write requires a packet without any calls"
            );
            assert!(
                matches!(&prefixes[0], LifecyclePrefixSourceV1::BorrowedFieldWrite { frame: original, .. }
                if Rc::ptr_eq(original, &frame))
            );
            assert!(prefixes[0].covered_by(&RootHomeExitV1::issue(
                Box::new([]),
                Box::new([]),
                vec![assignment.statement_site().clone()].into_boxed_slice()
            )));
            assert!(!prefixes[0].covered_by(&RootHomeExitV1::issue(
                Box::new([]),
                Box::new([]),
                Box::new([])
            )));
            let mut installation = flow.expression_installation();
            assert!(!installation.approve(&plan));
        }
        // Bool roots share the transaction and source inventory, while their
        // conditional reach and primitive result laws stay a separate frame.
        for expression in [
            "true && false",
            "false || true",
            "(1 == 1) && (2 < 3)",
            "(1 != 2) || (3 >= 4)",
            "(\"a\" == \"a\") && true",
            "(\"a\" != \"b\") || false",
            "(\"a\" < \"b\") && true",
            "(\"a\" <= \"b\") && true",
            "(\"b\" > \"a\") || false",
            "(\"b\" >= \"a\") && true",
        ] {
            let parsed = crate::parser::NyashParser::parse_from_string(&format!(
                "function probe() {{ if {expression} {{ return 1 }} return 0 }}"
            ))
            .unwrap();
            let crate::ast::ASTNode::Program { mut statements, .. } = parsed else {
                panic!("Program")
            };
            let unit =
                VerifiedResolvedSourceUnitV1::resolve_function(statements.remove(0)).unwrap();
            let input = unit.root_function_input().unwrap();
            let statement = input.function().if_region_sites().next().unwrap();
            let source = crate::mir::resolved_value_profile::expression_plan::condition_expression_source_plan_v1(input, statement).unwrap();
            let flow = PrefixLocalFlow::new(input);
            let sibling = flow.clone();
            for succeeds in [false, true] {
                let mut installation = flow.expression_installation();
                let mut batches = 0;
                let plan = qualify_bool_condition_v1::<()>(
                    Rc::clone(&source),
                    &[],
                    &mut |_| None,
                    &mut |_| None,
                    &mut |_| None,
                    &mut |_| None,
                    &mut |_| Ok(None),
                    &mut |plan| installation.approve(plan),
                    &mut |fields, _| {
                        assert!(fields.is_empty());
                        batches += 1;
                        Ok(succeeds)
                    },
                )
                .unwrap();
                assert_eq!(batches, 1, "{expression}");
                assert_eq!(plan.is_some(), succeeds);
                if let Some(plan) = plan {
                    assert!(plan.print_frame().is_none());
                    let original = Rc::clone(plan.condition_frame().unwrap());
                    assert!(Rc::ptr_eq(original.source(), &source));
                    assert_eq!(original.logical_nodes().len(), 1);
                    assert!(original.calls().is_empty());
                    let portable = crate::mir::loop_recipe_contract::project_source_loop_bool_v1(
                        Rc::clone(&original),
                    )
                    .unwrap();
                    assert!(portable.matches_frame(&original));
                    assert!(portable.imports().is_empty());
                    assert_eq!(portable.region().may_fault(), expression.contains('"'));
                    installation.commit();
                    let installed = sibling.expression_plans();
                    assert_eq!(installed.len(), 1);
                    assert!(Rc::ptr_eq(
                        installed[0].condition_frame().unwrap(),
                        &original
                    ));
                    use crate::mir::resolved_semantics::home_new_prefix::{
                        LifecyclePrefixSourceV1, RootHomeExitV1, RootHomeFlow,
                    };
                    let root = RootHomeFlow {
                        named_arrays: Vec::new(),
                        exits: Default::default(),
                        maps: Vec::new(),
                        local_calls: Vec::new(),
                        expression_plans: installed,
                    };
                    let prefixes = root.lifecycle_prefixes(&[]).unwrap();
                    assert_eq!(
                        prefixes.len(),
                        1,
                        "even a call-free Bool condition is mandatory"
                    );
                    assert!(
                        matches!(&prefixes[0], LifecyclePrefixSourceV1::BoolCondition { frame, .. }
                        if Rc::ptr_eq(frame, &original))
                    );
                    assert!(prefixes[0].covered_by(&RootHomeExitV1::issue(
                        Box::new([]),
                        Box::new([]),
                        vec![statement.clone()].into_boxed_slice()
                    )));
                    assert!(!prefixes[0].covered_by(&RootHomeExitV1::issue(
                        Box::new([]),
                        Box::new([]),
                        Box::new([])
                    )));
                } else {
                    drop(installation);
                    assert!(sibling.expression_plans().is_empty());
                }
            }
            let mut installation = sibling.expression_installation();
            let mut batches = 0;
            assert!(qualify_bool_condition_v1::<()>(
                source,
                &[],
                &mut |_| None,
                &mut |_| None,
                &mut |_| None,
                &mut |_| None,
                &mut |_| Ok(None),
                &mut |plan| installation.approve(plan),
                &mut |_, _| {
                    batches += 1;
                    Ok(true)
                }
            )
            .unwrap()
            .is_none());
            assert_eq!(batches, 0);
        }
        for (expression, selected, accepted) in [
            ("true && false", true, true),
            ("\"a\" < \"b\"", true, true),
            ("(\"a\" < \"b\") || (\"z\" > \"x\")", true, true),
            ("(1 == 1) && (2 < 3)", true, true),
            ("false && obj.missing()", true, false),
            ("1 && true", true, false),
            ("true", false, false),
            ("obj != null && true", false, false),
            ("(3 - 1 == 2) && true", false, false),
        ] {
            let parsed = crate::parser::NyashParser::parse_from_string(&format!(
                "function probe(obj) {{ if {expression} {{ return 1 }} return 0 }}"
            ))
            .unwrap();
            let crate::ast::ASTNode::Program { mut statements, .. } = parsed else {
                panic!("Program")
            };
            let unit =
                VerifiedResolvedSourceUnitV1::resolve_function(statements.remove(0)).unwrap();
            let input = unit.root_function_input().unwrap();
            let statement = input.function().if_region_sites().next().unwrap();
            for field_ok in [false, true] {
                let flow = PrefixLocalFlow::new(input);
                let mut calls = Vec::new();
                let mut path_calls = Default::default();
                let mut batches = 0;
                let mut demands = 0;
                use crate::mir::resolved_semantics::home_new_prefix::condition::{
                    observe, ConditionInstallationV1,
                };
                let result = observe::<()>(
                    input,
                    statement,
                    &flow,
                    &[],
                    true,
                    &mut calls,
                    &mut path_calls,
                    &mut |requests, _| {
                        assert!(requests.is_empty());
                        batches += 1;
                        Ok(field_ok.then(Vec::new))
                    },
                    &mut |_, _| {
                        demands += 1;
                        Ok(None)
                    },
                )
                .unwrap();
                assert_eq!(
                    matches!(result, ConditionInstallationV1::Unselected),
                    !selected,
                    "{expression}"
                );
                assert_eq!(
                    matches!(result, ConditionInstallationV1::Installed),
                    accepted && field_ok,
                    "{expression}"
                );
                assert_eq!(batches, usize::from(accepted), "{expression}");
                assert_eq!(demands, usize::from(expression == "false && obj.missing()"));
                assert_eq!(
                    flow.expression_plans().len(),
                    usize::from(accepted && field_ok)
                );
                assert!(calls.is_empty() && path_calls.is_empty());
            }
        }
        for expression in [
            "1 && true",
            "false && obj.missing()",
            "\"x\" == 1",
            "\"x\" < 1",
            "\"x\" == null",
            "(\"a\" + \"b\") == \"ab\"",
        ] {
            let parsed = crate::parser::NyashParser::parse_from_string(&format!(
                "function probe(obj) {{ if {expression} {{ return 1 }} return 0 }}"
            ))
            .unwrap();
            let crate::ast::ASTNode::Program { mut statements, .. } = parsed else {
                panic!("Program")
            };
            let unit =
                VerifiedResolvedSourceUnitV1::resolve_function(statements.remove(0)).unwrap();
            let input = unit.root_function_input().unwrap();
            let source = crate::mir::resolved_value_profile::expression_plan::condition_expression_source_plan_v1(
                input, input.function().if_region_sites().next().unwrap()).unwrap();
            let mut batches = 0;
            assert!(qualify_bool_condition_v1::<()>(
                source,
                &[],
                &mut |_| None,
                &mut |_| None,
                &mut |_| None,
                &mut |_| None,
                &mut |_| Ok(None),
                &mut |_| { panic!("unproved condition cannot install") },
                &mut |_, _| {
                    batches += 1;
                    Ok(true)
                }
            )
            .unwrap()
            .is_none());
            assert_eq!(batches, 0);
        }
    }

    #[test]
    fn initializer_scope_keeps_its_original_profile_and_home_snapshot() {
        let parsed = crate::parser::NyashParser::parse_from_string(
            "function probe() { local prior = \"p\" + \"q\" local out = \"a\" + \"b\" return 0 }",
        )
        .unwrap();
        let crate::ast::ASTNode::Program { mut statements, .. } = parsed else {
            panic!("Program")
        };
        let unit = VerifiedResolvedSourceUnitV1::resolve_function(statements.remove(0)).unwrap();
        let input = unit.root_function_input().unwrap();
        let ledger = input
            .forest()
            .callable_source_ledger(input.owner())
            .unwrap();
        let initializers: Vec<_> = input
            .function()
            .expression_source()
            .initializers()
            .collect();
        let source =
            crate::mir::resolved_value_profile::expression_plan::local_expression_source_plan_v1(
                &ledger,
                input.body_shape(),
                initializers[1],
            )
            .unwrap();
        for profile in [
            leaves::ExpressionLeafProfileV1::Print,
            leaves::ExpressionLeafProfileV1::LocalInitializer,
        ] {
            let homes = [LiveHomeV1::Stable(initializers[0].binding())];
            let prepared = leaves::prepare_expression_calls_v1::<()>(
                &source,
                profile,
                &Default::default(),
                &homes,
                &mut |_| panic!("literal initializer has no Call"),
            )
            .unwrap()
            .unwrap();
            let mut batches = 0;
            let frame = qualify_initializer_expression_v1::<()>(
                prepared,
                &[],
                &[],
                &mut |_| None,
                None,
                false,
                &mut |_| None,
                &mut |_, _, _| None,
                &mut |_| None,
                &mut |_| None,
                &mut |_| None,
                &mut |_| None,
                &mut |_| true,
                &mut |_, _| {
                    batches += 1;
                    Ok(true)
                },
            )
            .unwrap();
            assert_eq!(
                frame.is_some(),
                profile == leaves::ExpressionLeafProfileV1::LocalInitializer
            );
            assert_eq!(batches, usize::from(frame.is_some()));
            if let Some(plan) = frame {
                assert_eq!(plan.initializer_frame().unwrap().prior_homes(), homes);
            }
        }
    }

    #[test]
    fn original_loop_condition_keeps_frame_and_conditional_reach_without_if_permission() {
        use crate::mir::resolved_value_profile::{
            expression_children::{ExpressionChildReachV1, ExpressionChildrenV1},
            expression_plan::{
                condition_expression_source_plan_v1, loop_condition_expression_source_plan_v1,
                ExpressionPlanRootV1,
            },
        };
        for (operator, truth) in [("&&", true), ("||", false)] {
            let parsed = crate::parser::NyashParser::parse_from_string(&format!(
                "function probe(flag) {{ loop(flag {operator} true) {{ break }} if true {{ return 1 }} return 0 }}")).unwrap();
            let crate::ast::ASTNode::Program { mut statements, .. } = parsed else {
                panic!("Program")
            };
            let unit =
                VerifiedResolvedSourceUnitV1::resolve_function(statements.remove(0)).unwrap();
            let input = unit.root_function_input().unwrap();
            let site = input.function().only_loop_site().unwrap();
            let source = loop_condition_expression_source_plan_v1(input, &site).unwrap();
            let ledger = input
                .forest()
                .callable_source_ledger(input.owner())
                .unwrap();
            let membership = ledger.resolved_loop_source(&site).unwrap();
            assert!(
                matches!(source.root(), ExpressionPlanRootV1::LoopCondition { statement, condition, bundle, frame }
                if statement == &site && condition == source.nodes()[0].site()
                    && Some(bundle) == input.function().loop_region_bundle(&site).ok()
                    && frame.matches(membership.frame()))
            );
            assert_eq!(source.root().statement(), &site);
            assert!(source.root().initializer().is_none());
            assert_eq!(source.ordered_postorder_v1().unwrap(), vec![1, 2, 0]);
            let ExpressionChildrenV1::Known(children) = source.nodes()[0].children() else {
                panic!("original children")
            };
            assert_eq!(children[0].reach, ExpressionChildReachV1::Always);
            assert_eq!(
                children[1].reach,
                ExpressionChildReachV1::When {
                    condition: children[0].site.clone(),
                    value: truth
                }
            );
            assert!(condition_expression_source_plan_v1(input, &site).is_err());
            let if_site = input.function().if_region_sites().next().unwrap();
            assert!(loop_condition_expression_source_plan_v1(input, if_site).is_err());
            let mut batches = 0;
            assert!(
                qualify_bool_condition_v1::<()>(
                    source,
                    &[],
                    &mut |_| None,
                    &mut |_| None,
                    &mut |_| None,
                    &mut |_| None,
                    &mut |_| Ok(None),
                    &mut |_| true,
                    &mut |_, _| {
                        batches += 1;
                        Ok(true)
                    }
                )
                .unwrap()
                .is_none(),
                "source purpose cannot borrow If-only Home permission"
            );
            assert_eq!(batches, 0);
        }
    }
}
