//! Function-relative source transport for the live raw invocation port.
//!
//! The selected invocation route owns one shrinking dual-state carrier.  A
//! located row keeps the already-issued callable root receipt plus the exact
//! `SourcePathV1` node.  An unlocated row names one finite migration portal;
//! it is an execute-once compatibility state, never a retry route.

use crate::ast::ASTNode;
use crate::mir::builder::stmts::block_driver::{
    drive_legacy_block_v1, LegacyBlockDescentPortV1,
};
use crate::mir::builder::MirBuilder;
use crate::mir::resolved_semantics::{
    BodyChildRoleV1, ExprChildRoleV1, ExprChildSyntaxV1, SourceBodyKindV1,
    SourceNodeSiteV1, SourcePathSegmentV1, SourcePathV1,
};
use crate::mir::ValueId;

use super::normal_instance_constructor_admission::NormalInstanceConstructorSourceKeyV1;
use super::normal_top_level_function_admission::NormalTopLevelFunctionSourceKeyV1;
use super::recursive_child_lowering::{
    lower_raw_expression_with_recursion_guard_v1, RawInvocationChildPortV1,
    RecursiveChildLoweringPortV1,
};
use super::raw_structured_child_scope::PreparedRawChildSourceV1;
use super::{CanonicalSameModuleCallableKeyV1, RawSourceLocatorV1};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum RawInvocationRootLineageV1 {
    ScriptRoot,
    Main(RawSourceLocatorV1),
    Cataloged(CanonicalSameModuleCallableKeyV1),
    TopLevel(NormalTopLevelFunctionSourceKeyV1),
    InstanceConstructor(NormalInstanceConstructorSourceKeyV1),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum RawUnlocatedPortalV1 {
    ControlBody,
    ScalarBinding,
    CallObject,
    NestedBoxAdmission,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::mir::builder) struct LocatedRawNodeV1<T> {
    node: T,
    root: RawInvocationRootLineageV1,
    site: SourceNodeSiteV1,
}

impl<T> LocatedRawNodeV1<T> {
    fn new(node: T, root: RawInvocationRootLineageV1, site: SourceNodeSiteV1) -> Self {
        Self { node, root, site }
    }

    pub(in crate::mir::builder) fn into_parts(
        self,
    ) -> (T, RawInvocationRootLineageV1, SourceNodeSiteV1) {
        (self.node, self.root, self.site)
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
    pub(in crate::mir::builder) fn root(
        node: T,
        root: RawInvocationRootLineageV1,
    ) -> Self {
        Self::Located(LocatedRawNodeV1::new(
            node,
            root,
            SourcePathV1::function_body().node(),
        ))
    }

    pub(in crate::mir::builder) fn unlocated(
        node: T,
        reason: RawUnlocatedPortalV1,
    ) -> Self {
        Self::UnlocatedCompatibility { node, reason }
    }

    pub(in crate::mir::builder) fn into_parts(
        self,
    ) -> (
        T,
        Option<(RawInvocationRootLineageV1, SourceNodeSiteV1)>,
        Option<RawUnlocatedPortalV1>,
    ) {
        match self {
            Self::Located(located) => {
                let (node, root, site) = located.into_parts();
                (node, Some((root, site)), None)
            }
            Self::UnlocatedCompatibility { node, reason } => (node, None, Some(reason)),
        }
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
            (Some((root, site)), None) => Self::Located {
                root,
                site,
                body_kind: Some(SourceBodyKindV1::Function),
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
                {
                    let reason = reason_for_non_box_statement(&statement);
                    return RawInvocationSourceTransportV1::unlocated(
                        statement,
                        reason,
                    );
                }
                let kind =
                    body_kind.expect("located body transport must retain its body kind");
                let child = if kind == SourceBodyKindV1::Function
                    && site.segments() == [SourcePathSegmentV1::FunctionBody]
                {
                    SourcePathV1::root_body(index).node()
                } else {
                    SourcePathV1::from_node(site)
                        .child(kind.item_segment(index as u32))
                        .node()
                };
                RawInvocationSourceTransportV1::Located(LocatedRawNodeV1::new(
                    statement,
                    root.clone(),
                    child,
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
        let mut path = SourcePathV1::from_node(site);
        if let Some(segment) = kind.root_segment() {
            path = path.child(segment);
        }
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
        let kind =
            body_kind.ok_or_else(|| {
                "[freeze:contract][raw-invocation/missing-parent-body-kind]".to_owned()
            })?;
        if !is_located_control_or_diagnostic_terminal(statement) {
            return Err(format!(
                "[freeze:contract][raw-invocation/statement-source-role] kind={}",
                statement.node_type()
            ));
        }
        let child = if kind == SourceBodyKindV1::Function
            && site.segments() == [SourcePathSegmentV1::FunctionBody]
        {
            SourcePathV1::root_body(index).node()
        } else {
            SourcePathV1::from_node(site)
                .child(kind.item_segment(index as u32))
                .node()
        };
        Ok(Self::Located {
            root: root.clone(),
            site: child,
            body_kind: None,
        })
    }
}

fn reason_for_non_box_statement(statement: &ASTNode) -> RawUnlocatedPortalV1 {
    match statement {
        ASTNode::Program { .. }
        | ASTNode::Loop { .. }
        | ASTNode::Lambda { .. }
        | ASTNode::TryCatch { .. } => RawUnlocatedPortalV1::ControlBody,

        ASTNode::Assignment { .. }
        | ASTNode::CompoundAssignment { .. }
        | ASTNode::Print { .. }
        | ASTNode::Return { .. }
        | ASTNode::GroupedAssignmentExpr { .. }
        | ASTNode::Local { .. } => RawUnlocatedPortalV1::ScalarBinding,

        ASTNode::Break { .. }
        | ASTNode::Continue { .. }
        | ASTNode::UsingStatement { .. }
        | ASTNode::ImportStatement { .. }
        | ASTNode::BuildGate { .. }
        | ASTNode::Nowait { .. }
        | ASTNode::AwaitExpression { .. }
        | ASTNode::QMarkPropagate { .. }
        | ASTNode::ArrayLiteral { .. }
        | ASTNode::MapLiteral { .. }
        | ASTNode::RecordLiteral { .. }
        | ASTNode::RecordUpdate { .. }
        | ASTNode::Arrow { .. }
        | ASTNode::Throw { .. }
        | ASTNode::FunctionDeclaration { .. }
        | ASTNode::EnumDeclaration { .. }
        | ASTNode::BrandDeclaration { .. }
        | ASTNode::TypeAliasDeclaration { .. }
        | ASTNode::GlobalVar { .. }
        | ASTNode::StaticConstTable { .. }
        | ASTNode::Literal { .. }
        | ASTNode::Variable { .. }
        | ASTNode::UnaryOp { .. }
        | ASTNode::BinaryOp { .. }
        | ASTNode::CheckExpr { .. }
        | ASTNode::MethodCall { .. }
        | ASTNode::FieldAccess { .. }
        | ASTNode::Index { .. }
        | ASTNode::New { .. }
        | ASTNode::This { .. }
        | ASTNode::Me { .. }
        | ASTNode::FromCall { .. }
        | ASTNode::ThisField { .. }
        | ASTNode::MeField { .. }
        | ASTNode::Outbox { .. }
        | ASTNode::FunctionCall { .. }
        | ASTNode::Call { .. } => RawUnlocatedPortalV1::CallObject,

        ASTNode::BoxDeclaration { .. }
        | ASTNode::If { .. }
        | ASTNode::TaskScope { .. }
        | ASTNode::FastMemRegion { .. }
        | ASTNode::BlockExpr { .. }
        | ASTNode::ScopeBox { .. }
        | ASTNode::LoopRange { .. }
        | ASTNode::ContextScope { .. }
        | ASTNode::MatchExpr { .. }
        | ASTNode::EnumMatchExpr { .. } => {
            unreachable!("[freeze:contract][raw-invocation/direct-box-classifier]")
        }
    }
}

fn is_located_control_or_diagnostic_terminal(statement: &ASTNode) -> bool {
    matches!(
        statement,
        ASTNode::If { .. }
            | ASTNode::TaskScope { .. }
            | ASTNode::FastMemRegion { .. }
            | ASTNode::ScopeBox { .. }
            | ASTNode::BlockExpr { .. }
            | ASTNode::LoopRange { .. }
            | ASTNode::ContextScope { .. }
            | ASTNode::MatchExpr { .. }
            | ASTNode::EnumMatchExpr { .. }
    )
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

    fn lower_body(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::BodyInput,
    ) -> Result<ValueId, String> {
        match self.current_source_context_v1() {
            Some(context) => drive_located_invocation_body_v1(builder, self, input, context),
            None => {
                Err("[freeze:contract][raw-invocation/missing-root-body-receipt]".to_owned())
            }
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
                "[freeze:contract][raw-invocation/missing-parent-expression-receipt]"
                    .to_owned()
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
                "[freeze:contract][raw-invocation/missing-parent-statement-receipt]"
                    .to_owned()
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
}

pub(in crate::mir::builder) fn drive_located_invocation_body_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    statements: Vec<ASTNode>,
    context: RawInvocationSourceContextV1,
) -> Result<ValueId, String>
where
    Port: RawSourceTransportPortV1 + super::raw_expression_dispatch::RawExpressionDispatchPortV1,
{
    let mut body = LocatedInvocationBlockPortV1 {
        statements: statements.into_iter(),
        source: context,
        child: port,
    };
    drive_legacy_block_v1(builder, &mut body)
}

struct LocatedInvocationBlockPortV1<'port, Port> {
    statements: std::vec::IntoIter<ASTNode>,
    source: RawInvocationSourceContextV1,
    child: &'port mut Port,
}

impl<Port> LegacyBlockDescentPortV1 for LocatedInvocationBlockPortV1<'_, Port>
where
    Port: RawSourceTransportPortV1 + super::raw_expression_dispatch::RawExpressionDispatchPortV1,
{
    type SuffixInput<'a>
        = &'a [ASTNode]
    where
        Self: 'a;

    fn len(&self) -> usize {
        self.statements.len()
    }

    fn suffix_route_input(&self, _index: usize) -> Result<Option<Self::SuffixInput<'_>>, String> {
        Ok(Some(self.statements.as_slice()))
    }

    fn consume_suffix_prefix(&mut self, count: usize) {
        for _ in 0..count {
            let _ = self.statements.next();
        }
    }

    fn lower_statement(
        &mut self,
        builder: &mut MirBuilder,
        index: usize,
    ) -> Result<ValueId, String> {
        let statement = self
            .statements
            .next()
            .expect("block driver index stays within the owned source iterator");
        let transport = self.source.body_statement(statement, index);
        self.child.with_source_transport_v1(transport, |child, statement| {
            super::stmts::block_stmt::build_statement_with_port_v1(builder, child, statement)
        })
    }
}

#[cfg(test)]
#[path = "raw_invocation_source_transport_tests.rs"]
mod tests;
