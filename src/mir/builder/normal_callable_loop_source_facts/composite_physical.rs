//! Source-bound physical input for the composite LoopBreak owner.
//!
//! The composite envelope keeps the resolver projection/Recipe paired with
//! the ordered publication requirements before the existing Parts/LoopV0
//! owner allocates.  It does not consume publication rows itself; expression
//! lowering remains the sole handoff consumer.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::LoopPlanExpressionPortV1;
use crate::mir::builder::normal_callable_loop_handoff::CallableLoopReadyBodyOnlyProductV1;
use crate::mir::builder::normal_callable_loop_source_facts::composite::VerifiedCallableLoopBreakCompositeSourceCandidateV1;
use crate::mir::builder::normal_callable_loop_source_port::CallableLoopSourceExpressionPortV1;
use crate::mir::builder::normal_callable_loop_source_route::{
    CallableLoopSourceItemBindingV1, CallableLoopSourceItemDispositionV1,
    CallableLoopSourceTargetProbeV1,
};
use crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
use crate::mir::builder::raw_invocation_source_transport::RawInvocationSourceContextV1;
use crate::mir::compiler::loop_break_composite_source_projection::VerifiedLoopBreakCompositeSourceProjectionV1;
use crate::mir::resolved_semantics::{
    FunctionOriginV1, FunctionOwnerIdV1, SemanticOwnerSourceKindV1, SourceNodeSiteV1,
};
use std::cell::RefCell;
use std::rc::Rc;

use crate::mir::builder::control_flow::plan::recipe_tree::BuiltRecipeTree;

/// One move-only composite physical envelope.  The direct input keeps its
/// singleton relation and remains a separate contract.
#[derive(Debug)]
pub(in crate::mir::builder) struct SourceLoopBreakCompositePhysicalInputV1<'source, 'ledger> {
    owner: FunctionOwnerIdV1,
    function_origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    parent_site: SourceNodeSiteV1,
    parent_source: &'source RawInvocationSourceContextV1,
    parent_node: ASTNode,
    condition_source: RawInvocationSourceContextV1,
    body_source: RawInvocationSourceContextV1,
    condition: ASTNode,
    body: Vec<ASTNode>,
    projection: VerifiedLoopBreakCompositeSourceProjectionV1,
    recipe: BuiltRecipeTree,
    source_items: Box<[CallableLoopSourceItemBindingV1]>,
    source_dispositions: Box<[CallableLoopSourceItemDispositionV1]>,
    source_port: CallableLoopSourceExpressionPortV1<'ledger>,
}

impl<'source, 'ledger> SourceLoopBreakCompositePhysicalInputV1<'source, 'ledger> {
    #[allow(clippy::too_many_arguments)]
    pub(in crate::mir::builder) fn from_candidate(
        candidate: VerifiedCallableLoopBreakCompositeSourceCandidateV1,
        parent_source: &'source RawInvocationSourceContextV1,
        parent_node: ASTNode,
        condition_source: RawInvocationSourceContextV1,
        body_source: RawInvocationSourceContextV1,
        condition: ASTNode,
        body: Vec<ASTNode>,
        binding_product: CallableLoopReadyBodyOnlyProductV1,
        source_items: Box<[CallableLoopSourceItemBindingV1]>,
        source_target_probe: CallableLoopSourceTargetProbeV1,
        source_ledger: &'ledger Rc<RefCell<CallableSemanticLoweringState>>,
    ) -> Result<Self, String> {
        let (projection, recipe) = candidate.into_parts();
        let parent_site = parent_source.site().cloned().ok_or_else(|| {
            "[freeze:contract][callable-loop/composite/parent-site-missing]".to_owned()
        })?;
        let owner = projection.owner();
        let function_origin = projection.function_origin();
        let source_kind = projection.source_kind();
        if projection.loop_site().node() != &parent_site || source_ledger.borrow().owner() != owner
        {
            return Err(
                "[freeze:contract][callable-loop/composite/candidate-source-mismatch]".to_owned(),
            );
        }
        let source_dispositions = source_target_probe
            .into_item_dispositions(&source_items)
            .map_err(|error| {
                format!("[freeze:contract][callable-loop/composite/source-target] {error:?}")
            })?;
        if source_items.is_empty() || source_dispositions.is_empty() {
            return Err(
                "[freeze:contract][callable-loop/composite/source-target-empty]".to_owned(),
            );
        }
        binding_product
            .consume_pre_effect(
                &parent_site,
                condition_source.site().ok_or_else(|| {
                    "[freeze:contract][callable-loop/composite/condition-site-missing]".to_owned()
                })?,
                body_source.site().ok_or_else(|| {
                    "[freeze:contract][callable-loop/composite/body-site-missing]".to_owned()
                })?,
            )
            .map_err(|error| {
                format!("[freeze:contract][callable-loop/composite/pre-effect] {error}")
            })?;
        Ok(Self {
            owner,
            function_origin,
            source_kind,
            parent_site,
            parent_source,
            parent_node,
            condition_source,
            body_source,
            condition,
            body,
            projection,
            recipe,
            source_items,
            source_dispositions,
            source_port: CallableLoopSourceExpressionPortV1::new(source_ledger),
        })
    }

    pub(in crate::mir::builder) fn parent_source(&self) -> &RawInvocationSourceContextV1 {
        self.parent_source
    }

    pub(in crate::mir::builder) fn parent_node(&self) -> &ASTNode {
        &self.parent_node
    }

    pub(in crate::mir::builder) fn recipe(&self) -> &BuiltRecipeTree {
        &self.recipe
    }

    pub(in crate::mir::builder) const fn source_port(
        &self,
    ) -> &CallableLoopSourceExpressionPortV1<'_> {
        &self.source_port
    }

    /// Recheck all co-sealed relations immediately before Parts allocation.
    pub(in crate::mir::builder) fn validate_for_source_port(&self) -> Result<(), String> {
        if self.projection.owner() != self.owner
            || self.projection.function_origin() != self.function_origin
            || self.projection.source_kind() != self.source_kind
            || self.projection.loop_site().node() != &self.parent_site
        {
            return Err(
                "[freeze:contract][callable-loop/composite/source-identity-mismatch]".to_owned(),
            );
        }
        let condition_site = self.condition_source.site().ok_or_else(|| {
            "[freeze:contract][callable-loop/composite/condition-site-missing]".to_owned()
        })?;
        let body_site = self.body_source.site().ok_or_else(|| {
            "[freeze:contract][callable-loop/composite/body-site-missing]".to_owned()
        })?;
        if !self
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
                "[freeze:contract][callable-loop/composite/source-lineage-mismatch]".to_owned(),
            );
        }
        if self.source_items.len() != self.source_dispositions.len()
            || self.source_items.is_empty()
            || self
                .source_items
                .iter()
                .zip(self.source_dispositions.iter())
                .any(|(item, disposition)| {
                    item.call_site() != disposition.call_site()
                        || !item
                            .call_site()
                            .node()
                            .segments()
                            .starts_with(self.parent_site.segments())
                        || match disposition {
                            CallableLoopSourceItemDispositionV1::SelectedStatic(relation) => {
                                !relation.has_exact_callee_i64_requirement(&[1])
                            }
                            CallableLoopSourceItemDispositionV1::CoreMethod(core_method) => {
                                core_method.call_site() != item.call_site()
                            }
                        }
                })
        {
            return Err(
                "[freeze:contract][callable-loop/composite/source-target-relation-mismatch]"
                    .to_owned(),
            );
        }
        self.source_port
            .expr(&self.condition, &self.condition_source)
            .map_err(|error| {
                format!("[freeze:contract][callable-loop/composite/source-port] {error}")
            })?;
        let body = self
            .source_port
            .body(&self.body, &self.body_source)
            .map_err(|error| {
                format!("[freeze:contract][callable-loop/composite/source-port] {error}")
            })?;
        if self.source_port.body_statements(&body).len() != self.body.len() {
            return Err("[freeze:contract][callable-loop/composite/body-cardinality]".to_owned());
        }
        Ok(())
    }
}

impl VerifiedCallableLoopBreakCompositeSourceCandidateV1 {
    #[allow(clippy::too_many_arguments)]
    pub(in crate::mir::builder) fn into_physical_input<'source, 'ledger>(
        self,
        parent_source: &'source RawInvocationSourceContextV1,
        parent_node: ASTNode,
        condition_source: RawInvocationSourceContextV1,
        body_source: RawInvocationSourceContextV1,
        condition: ASTNode,
        body: Vec<ASTNode>,
        binding_product: CallableLoopReadyBodyOnlyProductV1,
        source_items: Box<[CallableLoopSourceItemBindingV1]>,
        source_target_probe: CallableLoopSourceTargetProbeV1,
        source_ledger: &'ledger Rc<RefCell<CallableSemanticLoweringState>>,
    ) -> Result<SourceLoopBreakCompositePhysicalInputV1<'source, 'ledger>, String> {
        SourceLoopBreakCompositePhysicalInputV1::from_candidate(
            self,
            parent_source,
            parent_node,
            condition_source,
            body_source,
            condition,
            body,
            binding_product,
            source_items,
            source_target_probe,
            source_ledger,
        )
    }
}
