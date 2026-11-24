//! LoopCond source-facts extension for the selected parser loop.
//!
//! This module keeps the existing planner outcome as the Facts/Recipe source
//! and only co-seals the resolver forest, source item rows, and canonical
//! target relation. Physical LoopCond lowering is a later task.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::LoopPlanExpressionPortV1;
use crate::mir::builder::normal_callable_loop_handoff::CallableLoopReadyBodyOnlyProductV1;
use crate::mir::builder::normal_callable_loop_handoff::CallableSemanticLoopHandoffPreEffectReceiptV1;
use crate::mir::builder::normal_callable_loop_source_facts::CallableGenericLoopSourceFactsRouteErrorV1;
use crate::mir::builder::normal_callable_loop_source_port::CallableLoopSourceExpressionPortV1;
use crate::mir::builder::normal_callable_loop_source_route::{
    CallableLoopSourceCallCoverageV1, CallableLoopSourceItemBindingV1, CallableLoopSourceRouteRejectV1,
    CallableLoopSourceRouteTokenV1, CallableLoopSourceTargetProbeV1,
    VerifiedCallableLoopCallFreeCoverageV1,
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
#[path = "loop_cond_phi_session.rs"]
mod phi_session;
pub(in crate::mir::builder) use phi_session::{SourceLoopPhiSessionV1, SourceLoopPointCapabilityV2};
pub(in crate::mir) use phi_session::{CompletedSourceLoopPhiPointsV2, PreparedSourceLoopPhiCompletionV2};

/// One move-only source/physical transport for the selected LoopCond route.
///
/// The planner outcome, source forest, resolver item rows, and target relation
/// are transferred together.  No field is reconstructed from AST shape or a
/// source line after this product is issued.
#[derive(Debug)]
pub(in crate::mir) struct SourceLoopCondPhysicalInputV1<'source, 'ledger> {
    owner: FunctionOwnerIdV1,
    parent_site: SourceNodeSiteV1,
    parent_source: &'source RawInvocationSourceContextV1,
    condition_source: RawInvocationSourceContextV1,
    body_source: RawInvocationSourceContextV1,
    condition: ASTNode,
    body: Vec<ASTNode>,
    pre_effect: CallableSemanticLoopHandoffPreEffectReceiptV1,
    projection: VerifiedLoopCondBreakContinueSourceForestProjectionV1,
    source_items: Box<[CallableLoopSourceItemBindingV1]>,
    source_port: CallableLoopSourceExpressionPortV1<'ledger>,
    retained_source: Option<Rc<super::generic::RetainedCallableLoopSourceV1>>,
    phi_session: SourceLoopPhiSessionV1,
    source_recipe: Option<Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>>,
}

impl SourceLoopCondPhysicalInputV1<'_, '_> {
    pub(in crate::mir::builder) fn checked_source_recipe_v2(&self)
        -> Option<&Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>> { self.source_recipe.as_ref() }
    pub(in crate::mir::builder) fn phi_session_v1(&self) -> &SourceLoopPhiSessionV1 { &self.phi_session }
    pub(in crate::mir::builder) fn prepare_checked_phis_v2(&self, builder: &mut crate::mir::builder::MirBuilder,
        header: crate::mir::BasicBlockId, after: crate::mir::BasicBlockId,
    ) -> Result<(), String> {
        self.validate_for_source_port()?;
        let product = self.source_recipe.as_ref().ok_or("[freeze:contract][source-loop/source-product-missing]")?;
        self.source_port.prepare_checked_phis_v2(&self.phi_session, builder, Rc::clone(product), header, after)
    }
    pub(in crate::mir::builder) fn checked_header_point_v2(&self, binding: crate::mir::resolved_semantics::BindingRefV1)
        -> Result<SourceLoopPointCapabilityV2<'_>, String> {
        let product = self.source_recipe.as_ref().ok_or("[freeze:contract][source-loop/source-product-missing]")?;
        self.phi_session.header_point(product, binding)
    }
    pub(in crate::mir::builder) fn open_source_lifecycle_prefix_v2(&self,
        claims: Option<&crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1>,
    ) -> Result<(), String> {
        if std::env::var_os("NYASH_DEBUG_LOCAL_CALLS").is_some() {
            eprintln!("[local-call-bindings] loop-lifecycle-request owner={:?} product={} claims={}",
                self.owner, self.source_recipe.is_some(), claims.is_some());
        }
        if let Some(product) = &self.source_recipe {
            claims.ok_or("[freeze:contract][source-loop/claim-ledger-missing]")?
                .open_source_loop_prefix_v2(Rc::clone(product))?;
        }
        Ok(())
    }
    pub(in crate::mir::builder) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir::builder) fn parent_source(&self) -> &RawInvocationSourceContextV1 {
        self.parent_source
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
        if self.projection.members().first().map(|member| member.source().site()) != Some(&parent_stmt_site) {
            return Err(
                "[freeze:contract][callable-loop/loop-cond/projection-root-site-mismatch]"
                    .to_owned(),
            );
        }
        let condition_site = self.condition_source.site().ok_or_else(|| {
            "[freeze:contract][callable-loop/cond/source-site-missing]".to_owned()
        })?;
        if let Some(source) = &self.retained_source {
            use crate::mir::builder::normal_callable_loop_handoff::CallableLoopBindingRoleV1;
            let reads = self.pre_effect.rows().iter().flat_map(|row| row.receipts())
                .chain(self.pre_effect.body_only_rows().iter().flat_map(|row| row.receipts()))
                .filter(|receipt| receipt.role() == CallableLoopBindingRoleV1::ConditionRead)
                .map(|receipt| (receipt.site().clone(), receipt.binding()));
            if source.owner() != self.owner || source.site().node() != &self.parent_site
                || !Rc::ptr_eq(source.bindings(), self.pre_effect.source())
                || !source.matches_condition_reads_v1(condition_site, reads) {
                return Err("[freeze:contract][callable-loop/loop-cond/retained-condition-drift]".into());
            }
        }
        if let Some(product) = &self.source_recipe {
            if self.retained_source.as_ref().is_none_or(|source| !Rc::ptr_eq(source, &product.source.source))
                || product.source.condition.source().nodes().first().is_none_or(|node| node.site().node() != condition_site) {
                return Err("[freeze:contract][source-loop/physical-input-product-drift]".into());
            }
        }
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
        if let Some(product) = &self.source_recipe {
            CallableLoopSourceCallCoverageV1::validate_checked_recipe_v2(product, self.owner, &self.parent_site, &self.source_items)
                .map_err(|error| format!("[freeze:contract][source-loop/call-coverage] {error:?}"))?;
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
        )
    }
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
    source_recipe: Option<Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>>,
}

impl<'source> CallableLoopCondSourceFactsV1<'source> {
    /// Consume the source Facts product at the physical boundary.  The
    /// resulting input feeds the checked source-loop writer through raw child entry.
    pub(in crate::mir::builder) fn into_physical_input<'ledger>(
        self,
        source_ledger: &'ledger Rc<RefCell<CallableSemanticLoweringState>>,
        declared_instance_locator: Option<
            crate::mir::normal_callable_semantic_package::
                DeclaredInstanceCallLocatorScopeV1<'ledger>,
        >,
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
            source_recipe,
        } = self;
        let (owner, parent_site, projection, source_items) =
            route_token.into_physical_parts().map_err(|error| {
                format!("[freeze:contract][callable-loop/loop-cond/input] {error:?}")
            })?;
        if source_ledger.borrow().owner() != owner {
            return Err(
                "[freeze:contract][callable-loop/loop-cond/source-port-owner-mismatch]".to_owned(),
            );
        }
        let source_port = CallableLoopSourceExpressionPortV1::new(
            source_ledger,
            declared_instance_locator,
        );
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
        let retained_source = {
            let state = source_ledger.borrow();
            let source = state.ordinary_new_claim_ledger()
                .map(|ledger| ledger.retained_loop_source_v1(owner, &SourceStmtSiteV1::from_node(parent_site.clone())))
                .transpose()?.flatten().cloned();
            if source.as_ref().is_some_and(|source| !source.matches_identity_v1(
                owner, &parent_site, state.function_origin(), state.source_kind())) {
                return Err("[freeze:contract][callable-loop/loop-cond/retained-source-identity]".into());
            }
            source
        };
        Ok(SourceLoopCondPhysicalInputV1 {
            owner,
            parent_site,
            parent_source,
            condition_source,
            body_source,
            condition,
            body,
            pre_effect,
            projection,
            source_items,
            source_port,
            retained_source,
            phi_session: SourceLoopPhiSessionV1::default(),
            source_recipe,
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
    projection: Option<VerifiedLoopCondBreakContinueSourceForestProjectionV1>,
    source_items: Box<[CallableLoopSourceItemBindingV1]>,
    source_target_probe: CallableLoopSourceTargetProbeV1,
    call_free: Option<VerifiedCallableLoopCallFreeCoverageV1>,
    source_recipe: Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>,
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
    let route_token = CallableLoopSourceRouteTokenV1::issue_checked_recipe_v2(
        owner, parent_site, function_origin, source_kind, projection,
        Rc::clone(&source_recipe), source_items, source_target_probe, call_free,
    ).map_err(CallableGenericLoopSourceFactsRouteErrorV1::LoopCondRouteRejected)?;
    Ok(CallableLoopCondSourceFactsV1 {
        _owner: owner,
        _parent_source: parent_source,
        _condition_source: condition_source,
        _body_source: body_source,
        _condition: condition,
        _body: body,
        _binding_product: binding_product,
        route_token,
        source_recipe: Some(source_recipe),
    })
}
