//! Function-relative source transport for the live raw invocation port.
//!
//! The selected invocation route owns one shrinking dual-state carrier.  A
//! located row keeps the already-issued callable root receipt plus the exact
//! `SourcePathV1` node.  An unlocated row names one finite migration portal;
//! it is an execute-once compatibility state, never a retry route.

use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::ASTNode;
use crate::mir::builder::MirBuilder;
use crate::mir::resolved_semantics::{
    BodyChildRoleV1, ExprChildRoleV1, ExprChildSyntaxV1, SourceBodyKindV1, SourceNodeSiteV1,
    SourcePathSegmentV1, SourcePathV1,
};
use crate::mir::ValueId;

use super::callable_declaration_catalog::SelectedTopLevelFunctionKeyV1;
use super::normal_instance_constructor_admission::NormalInstanceConstructorSourceKeyV1;
use super::normal_script_semantic_lowering_state::ScriptSemanticLoweringState;
use super::normal_script_semantic_source::VerifiedScriptSemanticSourceV1;
use super::raw_invocation_source_item_site::body_item_site;
use super::raw_invocation_source_statement_classification::{
    is_bare_function_call_statement, is_located_control_or_diagnostic_terminal,
    is_located_lambda_statement, is_located_scalar_statement,
    is_located_zero_child_runtime_completion, reason_for_non_box_statement,
};
use super::raw_structured_child_scope::PreparedRawChildSourceV1;
use super::recursive_child_lowering::{
    lower_raw_expression_with_recursion_guard_v1, RawInvocationChildPortV1,
    RecursiveChildLoweringPortV1,
};
use super::{CanonicalSameModuleCallableKeyV1, RawSourceLocatorV1};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum RawInvocationRootLineageV1 {
    ScriptRoot,
    Main(RawSourceLocatorV1),
    Cataloged(CanonicalSameModuleCallableKeyV1),
    TopLevel(SelectedTopLevelFunctionKeyV1),
    InstanceConstructor(NormalInstanceConstructorSourceKeyV1),
    NestedBoxMethod {
        parent_site: SourceNodeSiteV1,
        method_key: Box<str>,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum RawUnlocatedPortalV1 {
    CallObject,
}

impl RawInvocationRootLineageV1 {
    fn allows_bare_function_call_location(&self) -> bool {
        matches!(
            self,
            Self::Cataloged(_) | Self::TopLevel(_) | Self::InstanceConstructor(_)
        )
    }

    pub(in crate::mir::builder) fn nested_box_method(
        parent_site: SourceNodeSiteV1,
        method_key: String,
    ) -> Self {
        Self::NestedBoxMethod {
            parent_site,
            method_key: method_key.into_boxed_str(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::mir::builder) struct LocatedRawNodeV1<T> {
    node: T,
    root: RawInvocationRootLineageV1,
    site: SourceNodeSiteV1,
    body_kind: SourceBodyKindV1,
}

impl<T> LocatedRawNodeV1<T> {
    fn new(
        node: T,
        root: RawInvocationRootLineageV1,
        site: SourceNodeSiteV1,
        body_kind: SourceBodyKindV1,
    ) -> Self {
        Self {
            node,
            root,
            site,
            body_kind,
        }
    }

    pub(in crate::mir::builder) fn into_parts(
        self,
    ) -> (
        T,
        RawInvocationRootLineageV1,
        SourceNodeSiteV1,
        SourceBodyKindV1,
    ) {
        (self.node, self.root, self.site, self.body_kind)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum RawInvocationSourceTransportV1<T> {
    Located(LocatedRawNodeV1<T>),
    UnlocatedCompatibility {
        node: T,
        reason: RawUnlocatedPortalV1,
    },
}

impl<T> RawInvocationSourceTransportV1<T> {
    pub(in crate::mir::builder) fn root(node: T, root: RawInvocationRootLineageV1) -> Self {
        Self::Located(LocatedRawNodeV1::new(
            node,
            root,
            SourcePathV1::function_body().node(),
            SourceBodyKindV1::Function,
        ))
    }

    pub(in crate::mir::builder) fn script_root(node: T) -> Self {
        Self::Located(LocatedRawNodeV1::new(
            node,
            RawInvocationRootLineageV1::ScriptRoot,
            SourcePathV1::program_body().node(),
            SourceBodyKindV1::Program,
        ))
    }

    pub(in crate::mir::builder) fn script_semantic_root(node: T) -> Self {
        Self::Located(LocatedRawNodeV1::new(
            node,
            RawInvocationRootLineageV1::ScriptRoot,
            SourcePathV1::program_body().node(),
            SourceBodyKindV1::Program,
        ))
    }

    pub(in crate::mir::builder) fn unlocated(node: T, reason: RawUnlocatedPortalV1) -> Self {
        Self::UnlocatedCompatibility { node, reason }
    }

    pub(in crate::mir::builder) fn into_parts(
        self,
    ) -> (
        T,
        Option<(
            RawInvocationRootLineageV1,
            SourceNodeSiteV1,
            SourceBodyKindV1,
        )>,
        Option<RawUnlocatedPortalV1>,
    ) {
        match self {
            Self::Located(located) => {
                let (node, root, site, body_kind) = located.into_parts();
                (node, Some((root, site, body_kind)), None)
            }
            Self::UnlocatedCompatibility { node, reason } => (node, None, Some(reason)),
        }
    }
}

impl RawInvocationChildPortV1<'_, '_> {
    fn lower_script_binding_rebind_v1(
        &mut self,
        builder: &mut MirBuilder,
        input: ASTNode,
    ) -> Result<ValueId, String> {
        let role = match &input {
            ASTNode::Assignment { target, .. }
                if matches!(target.as_ref(), ASTNode::Variable { .. }) =>
            {
                ExprChildRoleV1::AssignmentTarget
            }
            ASTNode::CompoundAssignment { target, .. }
                if matches!(target.as_ref(), ASTNode::Variable { .. }) =>
            {
                ExprChildRoleV1::CompoundAssignmentTarget
            }
            ASTNode::GroupedAssignmentExpr { .. } => ExprChildRoleV1::GroupedAssignmentTarget,
            _ => return Err("[freeze:contract][script-lexical/rebind-source-drift]".to_owned()),
        };
        let ledger = self
            .semantic_ledger
            .clone()
            .expect("script rebind lowering requires semantic ledger");
        let target_site = self
            .current_source_context_v1()
            .ok_or_else(|| "[freeze:contract][script-lexical/rebind-parent-site]".to_owned())?
            .child_expression(&input, role)?
            .site()
            .cloned()
            .ok_or_else(|| "[freeze:contract][script-lexical/rebind-target-site]".to_owned())?;
        let binding = ledger
            .borrow()
            .assignment_binding(&target_site)
            .ok_or_else(|| "[freeze:contract][script-lexical/rebind-binding]".to_owned())?;
        let value = lower_raw_expression_with_recursion_guard_v1(builder, self, input)?;
        ledger.borrow_mut().rebind(binding, value)?;
        Ok(value)
    }

    pub(in crate::mir::builder) fn with_script_semantic_source_v1<R>(
        &mut self,
        source: VerifiedScriptSemanticSourceV1<'_>,
        execute: impl FnOnce(&mut Self) -> R,
    ) -> Result<R, String> {
        let [root] = source.forest().roots() else {
            return Err("[freeze:contract][mir/script-semantic/root-cardinality]".to_owned());
        };
        if source
            .projection()
            .owner_root(source.source(), *root)
            .is_err()
            || source.runtime_source_indices().iter().any(|index| {
                !matches!(
                    source.source(),
                    ASTNode::Program { statements, .. } if statements.get(*index).is_some()
                )
            })
        {
            return Err("[freeze:contract][mir/script-semantic/source-proof]".to_owned());
        }
        let state = Rc::new(RefCell::new(ScriptSemanticLoweringState::new(
            source.into_lowering_input(),
        )?));
        let parent = std::mem::replace(&mut self.semantic_ledger, Some(state));
        let result = self.with_source_transport_v1(
            RawInvocationSourceTransportV1::script_semantic_root(()),
            |port, ()| execute(port),
        );
        self.semantic_ledger = parent;
        Ok(result)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum RawInvocationSourceContextV1 {
    Located {
        root: RawInvocationRootLineageV1,
        site: SourceNodeSiteV1,
        body_kind: Option<SourceBodyKindV1>,
    },
    UnlocatedCompatibility(RawUnlocatedPortalV1),
}

impl RawInvocationSourceContextV1 {
    pub(in crate::mir::builder) fn from_transport<T>(
        transport: RawInvocationSourceTransportV1<T>,
    ) -> (T, Self) {
        let (node, located, reason) = transport.into_parts();
        let context = match (located, reason) {
            (Some((root, site, body_kind)), None) => Self::Located {
                root,
                site,
                body_kind: Some(body_kind),
            },
            (None, Some(reason)) => Self::UnlocatedCompatibility(reason),
            _ => unreachable!("[freeze:contract][raw-invocation/source-transport-state]"),
        };
        (node, context)
    }

    pub(in crate::mir::builder) fn body_statement(
        &self,
        statement: ASTNode,
        index: usize,
    ) -> RawInvocationSourceTransportV1<ASTNode> {
        match self {
            Self::Located {
                root,
                site,
                body_kind,
            } => {
                if !matches!(&statement, ASTNode::BoxDeclaration { .. })
                    && !is_located_control_or_diagnostic_terminal(&statement)
                    && !is_located_scalar_statement(&statement)
                    && !is_located_zero_child_runtime_completion(&statement)
                    && !is_located_lambda_statement(&statement)
                    && !(root.allows_bare_function_call_location()
                        && is_bare_function_call_statement(&statement))
                {
                    let reason = reason_for_non_box_statement(&statement);
                    return RawInvocationSourceTransportV1::unlocated(statement, reason);
                }
                let kind = body_kind.expect("located body transport must retain its body kind");
                let child = body_item_site(kind, site, index);
                RawInvocationSourceTransportV1::Located(LocatedRawNodeV1::new(
                    statement,
                    root.clone(),
                    child,
                    kind,
                ))
            }
            Self::UnlocatedCompatibility(reason) => {
                RawInvocationSourceTransportV1::unlocated(statement, *reason)
            }
        }
    }

    pub(in crate::mir::builder) fn site(&self) -> Option<&SourceNodeSiteV1> {
        match self {
            Self::Located { site, .. } => Some(site),
            Self::UnlocatedCompatibility(_) => None,
        }
    }

    fn child_call_argument(&self, index: usize) -> Self {
        match self {
            Self::Located { root, site, .. } => Self::Located {
                root: root.clone(),
                site: SourcePathV1::from_node(site)
                    .child(SourcePathSegmentV1::Argument(index as u32))
                    .node(),
                body_kind: None,
            },
            Self::UnlocatedCompatibility(reason) => Self::UnlocatedCompatibility(*reason),
        }
    }

    pub(in crate::mir::builder) fn child_expression(
        &self,
        parent: &ASTNode,
        role: ExprChildRoleV1,
    ) -> Result<Self, String> {
        let Self::Located { root, site, .. } = self else {
            return Ok(self.clone());
        };
        let resolved = role.resolve(parent).ok_or_else(|| {
            format!(
                "[freeze:contract][raw-invocation/expr-child-role] parent={} role={role:?}",
                parent.node_type()
            )
        })?;
        if !matches!(resolved.syntax(), ExprChildSyntaxV1::Node(_)) {
            return Err(format!(
                "[freeze:contract][raw-invocation/expr-child-missing] parent={} role={role:?}",
                parent.node_type()
            ));
        }
        Ok(Self::Located {
            root: root.clone(),
            site: SourcePathV1::from_node(site)
                .child(resolved.segment())
                .node(),
            body_kind: None,
        })
    }

    pub(in crate::mir::builder) fn child_body(
        &self,
        parent: &ASTNode,
        role: BodyChildRoleV1,
    ) -> Result<Self, String> {
        let Self::Located { root, site, .. } = self else {
            return Ok(self.clone());
        };
        let resolved = role.resolve(parent).ok_or_else(|| {
            format!(
                "[freeze:contract][raw-invocation/body-child-role] parent={} role={role:?}",
                parent.node_type()
            )
        })?;
        if resolved.statements().is_none() {
            return Err(format!(
                "[freeze:contract][raw-invocation/body-child-missing] parent={} role={role:?}",
                parent.node_type()
            ));
        }
        let kind = resolved.kind();
        let path = kind.append_root_path(SourcePathV1::from_node(site));
        Ok(Self::Located {
            root: root.clone(),
            site: path.node(),
            body_kind: Some(kind),
        })
    }

    pub(in crate::mir::builder) fn structured_body_statement(
        &self,
        statement: ASTNode,
        index: usize,
    ) -> Result<RawInvocationSourceTransportV1<ASTNode>, String> {
        Ok(self.body_statement(statement, index))
    }

    pub(in crate::mir::builder) fn child_statement(
        &self,
        statement: &ASTNode,
        index: usize,
    ) -> Result<Self, String> {
        let Self::Located {
            root,
            site,
            body_kind,
        } = self
        else {
            return Ok(self.clone());
        };
        let kind = body_kind.ok_or_else(|| {
            "[freeze:contract][raw-invocation/missing-parent-body-kind]".to_owned()
        })?;
        if kind != SourceBodyKindV1::Program
            && !is_located_control_or_diagnostic_terminal(statement)
            && !is_located_scalar_statement(statement)
            && !is_located_zero_child_runtime_completion(statement)
            && !is_located_lambda_statement(statement)
            && !(root.allows_bare_function_call_location()
                && is_bare_function_call_statement(statement))
        {
            return Err(format!(
                "[freeze:contract][raw-invocation/statement-source-role] kind={}",
                statement.node_type()
            ));
        }
        let child = body_item_site(kind, site, index);
        Ok(Self::Located {
            root: root.clone(),
            site: child,
            body_kind: None,
        })
    }
}

/// Temporal source scope used only by the selected invocation port.
///
/// The callback executes exactly once.  Restoring the parent after it returns
/// is structural recursion bookkeeping, not a retry or route reselection.
pub(in crate::mir::builder) trait RawSourceTransportPortV1 {
    fn with_source_transport_v1<T, R>(
        &mut self,
        transport: RawInvocationSourceTransportV1<T>,
        execute: impl FnOnce(&mut Self, T) -> R,
    ) -> R;

    fn current_source_context_v1(&self) -> Option<RawInvocationSourceContextV1>;
}

impl RawSourceTransportPortV1 for RawInvocationChildPortV1<'_, '_> {
    fn with_source_transport_v1<T, R>(
        &mut self,
        transport: RawInvocationSourceTransportV1<T>,
        execute: impl FnOnce(&mut Self, T) -> R,
    ) -> R {
        let (node, source) = RawInvocationSourceContextV1::from_transport(transport);
        let parent = self.active_source.replace(source);
        let result = execute(self, node);
        self.active_source = parent;
        result
    }

    fn current_source_context_v1(&self) -> Option<RawInvocationSourceContextV1> {
        self.active_source.clone()
    }
}

impl RecursiveChildLoweringPortV1 for RawInvocationChildPortV1<'_, '_> {
    type BodyInput = Vec<ASTNode>;
    type StatementInput = ASTNode;
    type ExpressionInput = ASTNode;

    fn try_emit_source_bound_static_call_result_v1(
        &mut self,
        builder: &mut MirBuilder,
        owner: &str,
        method: &str,
        checked_source_arity: u32,
        arguments: &[ValueId],
    ) -> Result<Option<ValueId>, String> {
        super::raw_static_result_publication::try_emit_source_bound_static_call_result_v1(
            self,
            builder,
            owner,
            method,
            checked_source_arity,
            arguments,
        )
    }

    fn cleanup_exit_policy_v1(&self) -> super::control_flow::cleanup::CleanupExitPolicyV1 {
        self.cleanup_exit_policy
    }

    fn lower_body(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::BodyInput,
    ) -> Result<ValueId, String> {
        match self.current_source_context_v1() {
            Some(context) => super::raw_invocation_body::drive_located_invocation_body_v1(
                builder, self, input, context,
            ),
            None => Err("[freeze:contract][raw-invocation/missing-root-body-receipt]".to_owned()),
        }
    }

    fn lower_statement(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::StatementInput,
    ) -> Result<ValueId, String> {
        if self.active_source.is_none() {
            return Err(
                "[freeze:contract][raw-invocation/missing-statement-source-receipt]".to_owned(),
            );
        }
        if self.callable_ledger.is_some() && matches!(input, ASTNode::Local { .. }) {
            return self.lower_callable_local_v1(builder, input);
        }
        if self.semantic_ledger.is_some() {
            return match input {
                local @ ASTNode::Local { .. } => self.lower_script_local_v1(builder, local),
                nowait @ ASTNode::Nowait { .. } => self.lower_script_nowait_v1(builder, nowait),
                outbox @ ASTNode::Outbox { .. } => self.lower_script_outbox_v1(builder, outbox),
                other => {
                    super::stmts::block_stmt::build_statement_with_port_v1(builder, self, other)
                }
            };
        }
        super::stmts::block_stmt::build_statement_with_port_v1(builder, self, input)
    }

    fn lower_expression(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::ExpressionInput,
    ) -> Result<ValueId, String> {
        if self.active_source.is_none() {
            return Err(
                "[freeze:contract][raw-invocation/missing-expression-source-receipt]".to_owned(),
            );
        }
        if self.semantic_ledger.is_some() && matches!(input, ASTNode::Local { .. }) {
            return self.lower_script_local_v1(builder, input);
        }
        if self.callable_ledger.is_some() && matches!(input, ASTNode::Local { .. }) {
            return self.lower_callable_local_v1(builder, input);
        }
        if self.semantic_ledger.is_some() && matches!(input, ASTNode::Nowait { .. }) {
            return self.lower_script_nowait_v1(builder, input);
        }
        if self.semantic_ledger.is_some() && matches!(input, ASTNode::Outbox { .. }) {
            return self.lower_script_outbox_v1(builder, input);
        }
        if self.semantic_ledger.is_some()
            && (matches!(
                &input,
                ASTNode::Assignment { target, .. } | ASTNode::CompoundAssignment { target, .. }
                    if matches!(target.as_ref(), ASTNode::Variable { .. })
            ) || matches!(&input, ASTNode::GroupedAssignmentExpr { .. }))
        {
            return self.lower_script_binding_rebind_v1(builder, input);
        }
        if self.callable_ledger.is_some()
            && (matches!(
                &input,
                ASTNode::Assignment { target, .. } | ASTNode::CompoundAssignment { target, .. }
                    if matches!(target.as_ref(), ASTNode::Variable { .. })
            ) || matches!(&input, ASTNode::GroupedAssignmentExpr { .. }))
        {
            return self.lower_callable_binding_rebind_v1(builder, input);
        }
        if self.callable_ledger.is_some() && matches!(input, ASTNode::Variable { .. }) {
            return self.read_callable_variable_v1();
        }
        if let (Some(ledger), ASTNode::Variable { .. }) = (&self.semantic_ledger, &input) {
            let site = self
                .current_source_context_v1()
                .and_then(|context| context.site().cloned())
                .ok_or_else(|| "[freeze:contract][script-lexical/variable-site]".to_owned())?;
            let binding = ledger
                .borrow()
                .variable_binding(&site)
                .ok_or_else(|| "[freeze:contract][script-lexical/variable-binding]".to_owned())?;
            return ledger
                .borrow()
                .value(binding)
                .ok_or_else(|| "[freeze:contract][script-lexical/variable-value]".to_owned());
        }
        lower_raw_expression_with_recursion_guard_v1(builder, self, input)
    }

    fn prepare_expression_child_source_v1(
        &self,
        parent: &ASTNode,
        role: ExprChildRoleV1,
    ) -> Result<PreparedRawChildSourceV1, String> {
        let context = self
            .current_source_context_v1()
            .ok_or_else(|| {
                "[freeze:contract][raw-invocation/missing-parent-expression-receipt]".to_owned()
            })?
            .child_expression(parent, role)?;
        Ok(PreparedRawChildSourceV1::Exact(context))
    }

    fn prepare_body_child_source_v1(
        &self,
        parent: &ASTNode,
        role: BodyChildRoleV1,
    ) -> Result<PreparedRawChildSourceV1, String> {
        let context = self
            .current_source_context_v1()
            .ok_or_else(|| {
                "[freeze:contract][raw-invocation/missing-parent-body-receipt]".to_owned()
            })?
            .child_body(parent, role)?;
        Ok(PreparedRawChildSourceV1::Exact(context))
    }

    fn prepare_body_statement_source_v1(
        &self,
        statement: &ASTNode,
        index: usize,
    ) -> Result<PreparedRawChildSourceV1, String> {
        let context = self
            .current_source_context_v1()
            .ok_or_else(|| {
                "[freeze:contract][raw-invocation/missing-parent-statement-receipt]".to_owned()
            })?
            .child_statement(statement, index)?;
        Ok(PreparedRawChildSourceV1::Exact(context))
    }

    fn with_prepared_child_source_v1<R>(
        &mut self,
        source: PreparedRawChildSourceV1,
        execute: impl FnOnce(&mut Self) -> R,
    ) -> R {
        match source {
            PreparedRawChildSourceV1::Preserve => execute(self),
            PreparedRawChildSourceV1::Exact(source) => {
                let parent = self.active_source.replace(source);
                let result = execute(self);
                self.active_source = parent;
                result
            }
        }
    }

    fn with_call_argument_source_v1<R>(
        &mut self,
        index: usize,
        execute: impl FnOnce(&mut Self) -> R,
    ) -> R {
        let source = self
            .active_source
            .as_ref()
            .map(|source| source.child_call_argument(index));
        let parent = source.and_then(|source| self.active_source.replace(source));
        let result = execute(self);
        self.active_source = parent;
        result
    }
}

#[cfg(test)]
#[path = "raw_invocation_source_transport_tests.rs"]
mod tests;
