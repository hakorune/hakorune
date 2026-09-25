//! LoopCond source-facts extension for the selected parser loop.
//!
//! This module keeps the existing planner outcome as the Facts/Recipe source
//! and only co-seals the resolver forest, source item rows, and canonical
//! target relation. Physical LoopCond lowering is a later task.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::loop_cond_break_continue::LoopCondBreakContinueFacts;
use crate::mir::builder::control_flow::plan::LoopPlanExpressionPortV1;
use crate::mir::builder::control_flow::plan::PlanBuildOutcome;
use crate::mir::builder::control_flow::recipes::loop_cond_break_continue::LoopCondBreakContinueItem;
use crate::mir::builder::normal_callable_loop_handoff::CallableLoopReadyBodyOnlyProductV1;
use crate::mir::builder::normal_callable_loop_handoff::CallableSemanticLoopHandoffPreEffectReceiptV1;
use crate::mir::builder::normal_callable_loop_source_facts::CallableGenericLoopSourceFactsRouteErrorV1;
use crate::mir::builder::normal_callable_loop_source_port::CallableLoopSourceExpressionPortV1;
use crate::mir::builder::normal_callable_loop_source_route::{
    CallableLoopRouteMatchV1, CallableLoopSoleFamilyV1, CallableLoopSourceItemBindingV1,
    CallableLoopSourceRouteRejectV1, CallableLoopSourceRouteTokenV1,
    CallableLoopSourceTargetProbeV1, CallableLoopSourceTargetRelationV1,
};
use crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
use crate::mir::builder::raw_invocation_source_transport::RawInvocationSourceContextV1;
use crate::mir::loop_structural_facts::VerifiedLoopCondBreakContinueSourceForestProjectionV1;
use crate::mir::resolved_semantics::{
    FunctionOriginV1, FunctionOwnerIdV1, SemanticOwnerSourceKindV1, SourceNodeSiteV1,
    SourceStmtSiteV1,
};
use std::cell::RefCell;
use std::rc::Rc;

/// One move-only source/physical transport for the selected LoopCond route.
///
/// The planner outcome, source forest, resolver item rows, and target relation
/// are transferred together.  No field is reconstructed from AST shape or a
/// source line after this product is issued.
#[derive(Debug)]
pub(in crate::mir::builder) struct SourceLoopCondPhysicalInputV1<'source, 'ledger> {
    owner: FunctionOwnerIdV1,
    parent_site: SourceNodeSiteV1,
    parent_source: &'source RawInvocationSourceContextV1,
    condition_source: RawInvocationSourceContextV1,
    body_source: RawInvocationSourceContextV1,
    condition: ASTNode,
    body: Vec<ASTNode>,
    pre_effect: CallableSemanticLoopHandoffPreEffectReceiptV1,
    outcome: PlanBuildOutcome,
    selection: CallableLoopRouteMatchV1,
    projection: VerifiedLoopCondBreakContinueSourceForestProjectionV1,
    source_items: Box<[CallableLoopSourceItemBindingV1]>,
    source_target: CallableLoopSourceTargetRelationV1,
    source_port: CallableLoopSourceExpressionPortV1<'ledger>,
}

impl SourceLoopCondPhysicalInputV1<'_, '_> {
    pub(in crate::mir::builder) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir::builder) fn parent_site(&self) -> &SourceNodeSiteV1 {
        &self.parent_site
    }

    pub(in crate::mir::builder) fn parent_source(&self) -> &RawInvocationSourceContextV1 {
        self.parent_source
    }

    pub(in crate::mir::builder) fn condition_source(&self) -> &RawInvocationSourceContextV1 {
        &self.condition_source
    }

    pub(in crate::mir::builder) fn body_source(&self) -> &RawInvocationSourceContextV1 {
        &self.body_source
    }

    pub(in crate::mir::builder) fn condition(&self) -> &ASTNode {
        &self.condition
    }

    pub(in crate::mir::builder) fn body(&self) -> &[ASTNode] {
        &self.body
    }

    pub(in crate::mir::builder) fn pre_effect(
        &self,
    ) -> &CallableSemanticLoopHandoffPreEffectReceiptV1 {
        &self.pre_effect
    }

    pub(in crate::mir::builder) fn outcome(&self) -> &PlanBuildOutcome {
        &self.outcome
    }

    pub(in crate::mir::builder) fn selection(&self) -> &CallableLoopRouteMatchV1 {
        &self.selection
    }

    pub(in crate::mir::builder) fn projection(
        &self,
    ) -> &VerifiedLoopCondBreakContinueSourceForestProjectionV1 {
        &self.projection
    }

    pub(in crate::mir::builder) fn source_items(&self) -> &[CallableLoopSourceItemBindingV1] {
        &self.source_items
    }

    pub(in crate::mir::builder) fn source_target(&self) -> &CallableLoopSourceTargetRelationV1 {
        &self.source_target
    }

    pub(in crate::mir::builder) const fn source_port(
        &self,
    ) -> &CallableLoopSourceExpressionPortV1<'_> {
        &self.source_port
    }

    pub(in crate::mir::builder) fn loop_cond_facts(
        &self,
    ) -> Result<&LoopCondBreakContinueFacts, String> {
        self.outcome
            .facts
            .as_ref()
            .and_then(|facts| facts.facts.loop_cond_break_continue())
            .ok_or_else(|| "[freeze:contract][callable-loop/loop-cond/facts-missing]".to_owned())
    }

    /// Validate the co-sealed input before any Builder allocation or physical
    /// route execution.  Every relation checked here was issued by the same
    /// source owner; this method never derives a binding from a name or AST
    /// shape.
    pub(in crate::mir::builder) fn validate_for_source_port(&self) -> Result<(), String> {
        if self.pre_effect.owner() != self.owner || self.pre_effect.loop_site() != &self.parent_site
        {
            return Err(
                "[freeze:contract][callable-loop/loop-cond/pre-effect-owner-site-mismatch]"
                    .to_owned(),
            );
        }
        if self.projection.owner() != self.owner {
            return Err(
                "[freeze:contract][callable-loop/loop-cond/projection-owner-mismatch]".to_owned(),
            );
        }
        let parent_stmt_site = SourceStmtSiteV1::from_node(self.parent_site.clone());
        if self.projection.member_sites().first() != Some(&parent_stmt_site) {
            return Err(
                "[freeze:contract][callable-loop/loop-cond/projection-root-site-mismatch]"
                    .to_owned(),
            );
        }
        let condition_site = self.condition_source.site().ok_or_else(|| {
            "[freeze:contract][callable-loop/cond/source-site-missing]".to_owned()
        })?;
        let body_site = self.body_source.site().ok_or_else(|| {
            "[freeze:contract][callable-loop/body/source-site-missing]".to_owned()
        })?;
        if self.pre_effect.loop_site() != &self.parent_site
            || !self
                .parent_source
                .shares_root_lineage(&self.condition_source)
            || !self.parent_source.shares_root_lineage(&self.body_source)
            || !condition_site
                .segments()
                .starts_with(self.parent_site.segments())
            || !body_site
                .segments()
                .starts_with(self.parent_site.segments())
        {
            return Err(
                "[freeze:contract][callable-loop/loop-cond/source-site-parent-mismatch]".to_owned(),
            );
        }
        if self.source_target.core_method_items().is_empty() {
            let target_site = self.source_target.call_site();
            let target_matches = self
                .source_items
                .iter()
                .filter(|item| item.call_site() == target_site)
                .count();
            if target_matches == 0 {
                return Err(
                    "[freeze:contract][callable-loop/loop-cond/source-target-site-missing]"
                        .to_owned(),
                );
            }
            if target_matches > 1 {
                return Err(
                    "[freeze:contract][callable-loop/loop-cond/source-target-site-multiple]"
                        .to_owned(),
                );
            }
        } else {
            let core_sites = self
                .source_target
                .core_method_items()
                .iter()
                .map(|item| item.call_site())
                .collect::<std::collections::BTreeSet<_>>();
            if core_sites.len() != self.source_target.core_method_items().len()
                || core_sites.len() != self.source_items.len()
                || self
                    .source_items
                    .iter()
                    .any(|item| !core_sites.contains(item.call_site()))
            {
                return Err(
                    "[freeze:contract][callable-loop/loop-cond/core-method-item-coverage]"
                        .to_owned(),
                );
            }
        }
        if self.source_items.iter().any(|item| {
            !item
                .call_site()
                .node()
                .segments()
                .starts_with(self.parent_site.segments())
        }) {
            return Err(
                "[freeze:contract][callable-loop/loop-cond/source-item-parent-mismatch]".to_owned(),
            );
        }
        if self.source_target.core_method_items().is_empty()
            && !self.source_target.has_exact_i64_result()
        {
            return Err(
                "[freeze:contract][callable-loop/loop-cond/result-requirement-mismatch]".to_owned(),
            );
        }
        self.source_port
            .expr(&self.condition, &self.condition_source)
            .map_err(|error| {
                format!("[freeze:contract][callable-loop/loop-cond/source-port] {error}")
            })?;
        self.source_port
            .body(&self.body, &self.body_source)
            .map_err(|error| {
                format!("[freeze:contract][callable-loop/loop-cond/source-port] {error}")
            })?;
        let facts = self.loop_cond_facts()?;
        if facts.condition != self.condition || facts.recipe.body.body != self.body {
            return Err(
                "[freeze:contract][callable-loop/loop-cond/recipe-source-mismatch]".to_owned(),
            );
        }
        if self.selection.sole_family() != Some(CallableLoopSoleFamilyV1::LoopCondBreakContinue)
        {
            return Err(
                "[freeze:contract][callable-loop/loop-cond/route-selection-mismatch]".to_owned(),
            );
        }
        Ok(())
    }

    /// Consume the source port through the already-issued loop topology before
    /// a physical Builder session is opened.  This is deliberately a
    /// preflight: it does not lower or allocate MIR, but it proves that the
    /// condition, body statements, nested control-flow bodies, and method-call
    /// sites all remain reachable through the same located carrier.
    pub(in crate::mir::builder) fn preflight_source_port(&self) -> Result<(), String> {
        preflight_source_port_inputs(
            &self.source_port,
            &self.condition,
            &self.condition_source,
            &self.body,
            &self.body_source,
        )?;
        let facts = self.loop_cond_facts()?;
        let body = self
            .source_port
            .body(&self.body, &self.body_source)
            .map_err(|error| format!("[freeze:contract][callable-loop/source-port] {error}"))?;
        preflight_source_exit_if_items(&self.source_port, &body, &facts.recipe.items)
    }
}

fn preflight_source_exit_if_items<'input, P>(
    port: &P,
    body: &P::BodyInput<'input>,
    items: &[LoopCondBreakContinueItem],
) -> Result<(), String>
where
    P: LoopPlanExpressionPortV1 + 'input,
{
    for item in items {
        let LoopCondBreakContinueItem::ExitIf {
            if_stmt,
            block: None,
        } = item
        else {
            continue;
        };
        let statement = port
            .body_stmt(body, if_stmt.index())
            .map_err(|error| error.render())?;
        if !matches!(
            port.stmt_syntax(&statement),
            ASTNode::If {
                else_body: None,
                ..
            }
        ) {
            return Err(
                "[freeze:contract][callable-loop/loop-cond/source-exit-if-shape]".to_owned(),
            );
        }
        let then_body = port
            .child_body_from_stmt(
                &statement,
                crate::mir::resolved_semantics::BodyChildRoleV1::IfThen,
            )
            .map_err(|error| error.render())?;
        crate::mir::builder::control_flow::plan::validate_return_exit_branch_input(
            port,
            &then_body,
            "[freeze:contract][callable-loop/loop-cond/source-exit-if-then]",
        )?;
    }
    Ok(())
}

pub(super) fn preflight_source_port_inputs(
    source_port: &CallableLoopSourceExpressionPortV1<'_>,
    condition: &ASTNode,
    condition_source: &RawInvocationSourceContextV1,
    body: &[ASTNode],
    body_source: &RawInvocationSourceContextV1,
) -> Result<(), String> {
    let condition = source_port
        .expr(condition, condition_source)
        .map_err(|error| format!("[freeze:contract][callable-loop/source-port] {error}"))?;
    preflight_source_expr(source_port, condition)?;

    let body = source_port
        .body(body, body_source)
        .map_err(|error| format!("[freeze:contract][callable-loop/source-port] {error}"))?;
    preflight_source_body(source_port, &body)
}

fn preflight_source_body<'input, P>(port: &P, body: &P::BodyInput<'input>) -> Result<(), String>
where
    P: LoopPlanExpressionPortV1 + 'input,
{
    for index in 0..port.body_statements(body).len() {
        let statement = port
            .body_stmt(body, index)
            .map_err(|error| error.render())?;
        preflight_source_stmt(port, &statement)?;
    }
    Ok(())
}

fn preflight_source_stmt<'input, P>(
    port: &P,
    statement: &P::StmtInput<'input>,
) -> Result<(), String>
where
    P: LoopPlanExpressionPortV1 + 'input,
{
    let expression = port
        .statement_expr(statement)
        .map_err(|error| error.render())?;
    preflight_source_expr(port, expression)
}

fn preflight_source_expr<'input, P>(
    port: &P,
    expression: P::ExprInput<'input>,
) -> Result<(), String>
where
    P: LoopPlanExpressionPortV1 + 'input,
{
    use crate::ast::ASTNode;
    use crate::mir::resolved_semantics::{BodyChildRoleV1, ExprChildRoleV1};

    match port.expr_syntax(&expression) {
        ASTNode::MethodCall { arguments, .. } => {
            port.call_source(&expression)
                .map_err(|error| error.render())?;
            let receiver = port
                .child_expr(&expression, ExprChildRoleV1::Receiver)
                .map_err(|error| error.render())?;
            preflight_source_expr(port, receiver)?;
            for index in 0..arguments.len() {
                let argument = port
                    .child_expr(&expression, ExprChildRoleV1::CallArgument(index as u32))
                    .map_err(|error| error.render())?;
                preflight_source_expr(port, argument)?;
            }
        }
        ASTNode::If { else_body, .. } => {
            let condition = port
                .child_expr(&expression, ExprChildRoleV1::IfCondition)
                .map_err(|error| error.render())?;
            preflight_source_expr(port, condition)?;
            let then_body = port
                .child_body(&expression, BodyChildRoleV1::IfThen)
                .map_err(|error| error.render())?;
            preflight_source_body(port, &then_body)?;
            if else_body.is_some() {
                let else_body = port
                    .child_body(&expression, BodyChildRoleV1::IfElse)
                    .map_err(|error| error.render())?;
                preflight_source_body(port, &else_body)?;
            }
        }
        ASTNode::Loop { .. } => {
            let condition = port
                .child_expr(&expression, ExprChildRoleV1::LoopCondition)
                .map_err(|error| error.render())?;
            preflight_source_expr(port, condition)?;
            let body = port
                .child_body(&expression, BodyChildRoleV1::LoopBody)
                .map_err(|error| error.render())?;
            preflight_source_body(port, &body)?;
        }
        ASTNode::BinaryOp { .. } => {
            let left = port
                .child_expr(&expression, ExprChildRoleV1::BinaryLeft)
                .map_err(|error| error.render())?;
            let right = port
                .child_expr(&expression, ExprChildRoleV1::BinaryRight)
                .map_err(|error| error.render())?;
            preflight_source_expr(port, left)?;
            preflight_source_expr(port, right)?;
        }
        ASTNode::UnaryOp { .. } => {
            let operand = port
                .child_expr(&expression, ExprChildRoleV1::UnaryOperand)
                .map_err(|error| error.render())?;
            preflight_source_expr(port, operand)?;
        }
        _ => {}
    }
    Ok(())
}

#[derive(Debug)]
pub(in crate::mir::builder) struct CallableLoopCondSourceFactsV1<'source> {
    _owner: FunctionOwnerIdV1,
    _parent_source: &'source RawInvocationSourceContextV1,
    _condition_source: RawInvocationSourceContextV1,
    _body_source: RawInvocationSourceContextV1,
    _condition: ASTNode,
    _body: Vec<ASTNode>,
    _binding_product: CallableLoopReadyBodyOnlyProductV1,
    route_token: CallableLoopSourceRouteTokenV1,
}

impl<'source> CallableLoopCondSourceFactsV1<'source> {
    /// Consume the source Facts product at the physical boundary.  The
    /// resulting input feeds the sole LoopCond physical consumer,
    /// `lower_loop_cond_break_continue_source`, through the raw child-entry
    /// caller — never the raw `LoopRouteContext` route.
    pub(in crate::mir::builder) fn into_physical_input<'ledger>(
        self,
        source_ledger: &'ledger Rc<RefCell<CallableSemanticLoweringState>>,
    ) -> Result<SourceLoopCondPhysicalInputV1<'source, 'ledger>, String> {
        let Self {
            _owner,
            _parent_source,
            _condition_source,
            _body_source,
            _condition,
            _body,
            _binding_product,
            route_token,
        } = self;
        let (owner, parent_site, outcome, selection, projection, source_items, source_target) =
            route_token.into_physical_parts().map_err(|error| {
                format!("[freeze:contract][callable-loop/loop-cond/input] {error:?}")
            })?;
        if source_ledger.borrow().owner() != owner {
            return Err(
                "[freeze:contract][callable-loop/loop-cond/source-port-owner-mismatch]".to_owned(),
            );
        }
        let source_port = CallableLoopSourceExpressionPortV1::new(source_ledger);
        let facts = outcome
            .facts
            .as_ref()
            .and_then(|facts| facts.facts.loop_cond_break_continue())
            .ok_or_else(|| "[freeze:contract][callable-loop/loop-cond/facts-missing]".to_owned())?;
        if facts.condition != _condition || facts.recipe.body.body != _body {
            return Err(
                "[freeze:contract][callable-loop/loop-cond/source-facts-shape-mismatch]".to_owned(),
            );
        }
        let condition = _condition;
        let body = _body;
        let parent_source = _parent_source;
        let condition_source = _condition_source;
        let body_source = _body_source;
        let parent_site_from_source = parent_source.site().ok_or_else(|| {
            "[freeze:contract][callable-loop/loop-cond/parent-site-missing]".to_owned()
        })?;
        if parent_site_from_source != &parent_site {
            return Err(
                "[freeze:contract][callable-loop/loop-cond/parent-site-mismatch]".to_owned(),
            );
        }
        let condition_site = condition_source.site().ok_or_else(|| {
            "[freeze:contract][callable-loop/loop-cond/condition-site-missing]".to_owned()
        })?;
        let body_site = body_source.site().ok_or_else(|| {
            "[freeze:contract][callable-loop/loop-cond/body-site-missing]".to_owned()
        })?;
        let pre_effect = _binding_product
            .consume_pre_effect(&parent_site, condition_site, body_site)
            .map_err(|error| {
                format!("[freeze:contract][callable-loop/loop-cond/pre-effect] {error}")
            })?;
        Ok(SourceLoopCondPhysicalInputV1 {
            owner,
            parent_site,
            parent_source,
            condition_source,
            body_source,
            condition,
            body,
            pre_effect,
            outcome,
            selection,
            projection,
            source_items,
            source_target,
            source_port,
        })
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn issue<'source>(
    owner: FunctionOwnerIdV1,
    parent_source: &'source RawInvocationSourceContextV1,
    condition_source: RawInvocationSourceContextV1,
    body_source: RawInvocationSourceContextV1,
    condition: ASTNode,
    body: Vec<ASTNode>,
    binding_product: CallableLoopReadyBodyOnlyProductV1,
    function_origin: Option<FunctionOriginV1>,
    source_kind: Option<SemanticOwnerSourceKindV1>,
    outcome: PlanBuildOutcome,
    selection: CallableLoopRouteMatchV1,
    projection: Option<VerifiedLoopCondBreakContinueSourceForestProjectionV1>,
    source_items: Box<[CallableLoopSourceItemBindingV1]>,
    source_target_probe: CallableLoopSourceTargetProbeV1,
) -> Result<CallableLoopCondSourceFactsV1<'source>, CallableGenericLoopSourceFactsRouteErrorV1> {
    let function_origin = function_origin.ok_or_else(|| {
        CallableGenericLoopSourceFactsRouteErrorV1::LoopCondRouteRejected(
            CallableLoopSourceRouteRejectV1::SourceIdentityMissing,
        )
    })?;
    let source_kind = source_kind.ok_or_else(|| {
        CallableGenericLoopSourceFactsRouteErrorV1::LoopCondRouteRejected(
            CallableLoopSourceRouteRejectV1::SourceIdentityMissing,
        )
    })?;
    let parent_site = parent_source.site().cloned().ok_or_else(|| {
        CallableGenericLoopSourceFactsRouteErrorV1::LoopCondRouteRejected(
            CallableLoopSourceRouteRejectV1::SourceParentMissing,
        )
    })?;
    let route_token = CallableLoopSourceRouteTokenV1::issue_with_source_relations(
        owner,
        parent_site,
        function_origin,
        source_kind,
        outcome,
        selection,
        projection,
        source_items,
        source_target_probe,
    )
    .map_err(CallableGenericLoopSourceFactsRouteErrorV1::LoopCondRouteRejected)?;
    Ok(CallableLoopCondSourceFactsV1 {
        _owner: owner,
        _parent_source: parent_source,
        _condition_source: condition_source,
        _body_source: body_source,
        _condition: condition,
        _body: body,
        _binding_product: binding_product,
        route_token,
    })
}
