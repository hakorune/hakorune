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
#[path = "home_expression_nullable_result.rs"]
mod nullable_result;
pub(crate) use nullable_result::{
    BorrowedResultReadV1, BranchCallResultSourceV1, DeclaredCallReadSourceV1,
    DeclaredReadReceiverV1, DeclaredReadValueV1, NullableResultReferenceV1, SourceCallResultReadV1,
};
#[path = "home_expression_operator_loan.rs"]
mod operator_loan;
pub(crate) use operator_loan::CheckedExpressionOperationLoanV1;
use operator_loan::CheckedExpressionOperationProofV1;

#[path = "home_expression_leaves.rs"]
mod leaves;
#[path = "home_expression_shared.rs"]
mod shared;
pub(crate) use leaves::ExpressionCallSourceV1;
pub(super) use leaves::{prepare_expression_calls_v1, ExpressionLeafProfileV1};
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
pub(super) use initializer_expression::qualify_alias_null_initializer_v1;
pub(crate) use initializer_expression::{
    qualify_initializer_expression_v1, CheckedInitializerExpressionFrameV1,
};
#[path = "home_binding_rebind_expression.rs"]
mod binding_rebind_expression;
pub(crate) use binding_rebind_expression::{
    qualify_binding_rebind_expression_with_loop_dynamic_v1, BindingRebindDestinationV1,
    CheckedBindingRebindFrameV1,
};
#[path = "home_expression_evaluation.rs"]
mod evaluation;
pub(super) use evaluation::{prepare_value_evaluation_v1, qualify_text_evaluation_v1};
pub(crate) use evaluation::{
    CheckedExpressionEvaluationV1, CheckedExpressionReferenceV1, CheckedScalarReferenceV1,
    ComparisonExecutionKindV1, ComparisonExecutionPlanV1,
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
    /// One explicitly shared handle: fresh, moved plain Box or Shared clone.
    /// Every operation retains its original share; only a moved Box has `moved`.
    Shared {
        source: Rc<ExpressionSourcePlanV1>,
        payload_source: super::SharedPayloadSourceV1,
        shared: crate::mir::resolved_semantics::ResolvedUnaryExpressionSourceV1,
        moved: Option<crate::mir::resolved_semantics::ResolvedUnaryExpressionSourceV1>,
        prior_homes: Rc<[LiveHomeV1]>,
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
#[path = "home_expression_source_fields.rs"]
mod source_fields;

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
            || !matches!(source.nodes()[0].source(), ExpressionNodeSourceV1::Unary(_))
            || !unary.is_move()
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
            initializer_frame: Some(InitializerFrameV1::Move {
                source,
                unary: unary.clone(),
                from,
                acquisition,
            }),
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
            InitializerFrameV1::Move { .. } | InitializerFrameV1::Shared { .. } => None,
        }
    }
    pub(crate) fn move_initializer_v1(
        &self,
    ) -> Option<(
        &Rc<ExpressionSourcePlanV1>,
        crate::mir::resolved_semantics::BindingRefV1,
        &crate::mir::resolved_semantics::OwnedExprSiteV1,
    )> {
        match self.initializer_frame.as_ref()? {
            InitializerFrameV1::Move {
                source,
                from,
                acquisition,
                ..
            } => Some((source, *from, acquisition)),
            InitializerFrameV1::Evaluation(_) | InitializerFrameV1::Shared { .. } => None,
        }
    }
    pub(crate) fn shared_initializer_v1(
        &self,
    ) -> Option<(
        &Rc<ExpressionSourcePlanV1>,
        &super::SharedPayloadSourceV1,
        &[LiveHomeV1],
    )> {
        match self.initializer_frame.as_ref()? {
            InitializerFrameV1::Shared {
                source,
                payload_source,
                prior_homes,
                ..
            } => Some((source, payload_source, prior_homes)),
            _ => None,
        }
    }
    pub(crate) fn shared_initializer_source_v1(&self) -> Option<&Rc<ExpressionSourcePlanV1>> {
        self.shared_initializer_v1().map(|row| row.0)
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
                    || matches!(self.source.root(), ExpressionPlanRootV1::CallArgument { .. })
                    || self.has_borrowed_return_field_basis_v1())
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
                        && unary.is_move()
                        && unary.site() == source.nodes()[0].site()
                        && unary.operand() == source.nodes()[1].site()
                }
                InitializerFrameV1::Shared { source, payload_source, shared, moved, prior_homes } => {
                    use super::SharedPayloadSourceV1 as P;
                    use crate::mir::resolved_semantics::ResolvedUnaryOperatorV1 as Op;
                    let nodes = source.nodes();
                    Rc::ptr_eq(source, &self.source)
                        && source.root().initializer().is_some_and(|row| row.binding().owner() == source.owner())
                        && match (payload_source, moved) {
                        (P::Existing { from, acquisition, is_clone: true }, None) =>
                            Self::shared_clone_source_is_consistent_v1(source, *from, acquisition, shared, prior_homes),
                        (P::Existing { from, acquisition, is_clone: false }, Some(moved)) => {
                            from.owner() == source.owner() && acquisition.owner() == source.owner() && nodes.len() == 3
                                && shared.operator() == Op::Shared && moved.is_move()
                                && shared.site() == nodes[0].site() && shared.operand() == moved.site()
                                && moved.site() == nodes[1].site() && moved.operand() == nodes[2].site()
                                && matches!(nodes[0].source(), ExpressionNodeSourceV1::Unary(_))
                                && matches!(nodes[1].source(), ExpressionNodeSourceV1::Unary(_))
                                && matches!(nodes[2].source(), ExpressionNodeSourceV1::Reference(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding)) if *binding == *from)
                                && prior_homes.iter().filter(|home| home.binding() == *from).count() == 1
                        }
                        (P::Fresh { site: payload }, None) => {
                            payload.owner() == source.owner() && shared.operator() == Op::Shared
                                && nodes.first().is_some_and(|node| node.site() == shared.site())
                                && shared.operand() == payload.site()
                                && nodes.iter().any(|node| node.site() == payload.site())
                        }
                        _ => false,
                    }
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
            || match (
                original.move_initializer_v1(),
                refined.move_initializer_v1(),
            ) {
                (None, None) => false,
                (Some((a, from_a, acquisition_a)), Some((b, from_b, acquisition_b))) => {
                    !Rc::ptr_eq(a, b) || from_a != from_b || acquisition_a != acquisition_b
                }
                _ => true,
            }
            || match (
                original.shared_initializer_v1(),
                refined.shared_initializer_v1(),
            ) {
                (None, None) => false,
                (Some((a, pa, ha)), Some((b, pb, hb))) => !Rc::ptr_eq(a, b) || pa != pb || ha != hb,
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
            || plan.evaluation().is_some_and(|evaluation| {
                evaluation
                    .cores()
                    .values()
                    .filter_map(|core| core.field_array_access_v1())
                    .any(|count| !self.locals.has_field_array_access_source_v1(count))
            })
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
#[path = "home_expression_installation_tests.rs"]
mod installation_tests;
