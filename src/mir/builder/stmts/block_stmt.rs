//! Block statement execution module
//!
//! ## Purpose
//! Sequential statement execution through one statement-descent port
//!
//! ## Responsibilities
//! - Block/statement execution coordination
//! - Termination checking
//! - Expression delegation
//!
//! ## Architecture
//! - Block execution coordinates statement → expression → block recursion
//! - Termination checking prevents duplicate terminators
//!
//! ## Integration Points
//! - Called by: control_flow::cf_block, expression building code
//! - Calls: build_statement and build_expression

use super::block_driver::{drive_legacy_block_v1, LegacyBlockDescentPortV1};
use crate::ast::ASTNode;
use crate::mir::builder::raw_expression_dispatch::{
    RawBodyInputViewV1, RawLegacyBodyInputV1, RawLegacyStatementInputV1, RawStatementInputViewV1,
};
use crate::mir::builder::recursive_child_lowering::{
    drive_legacy_expression_v1, drive_legacy_statement_v1, RawLegacyChildLoweringPortV1,
    RecursiveChildLoweringPortV1,
};
use crate::mir::builder::MirBuilder;
use crate::mir::builder::ValueId;

/// Build a block by sequentially processing statements
///
/// # Termination Checking
/// - Checks if block was terminated after each statement
/// - Prevents duplicate terminators (Return, Branch, etc.)
///
/// # Returns
/// - Last statement value, or Void if no statements
pub(in crate::mir::builder) fn build_block(
    builder: &mut MirBuilder,
    statements: Vec<ASTNode>,
) -> Result<ValueId, String> {
    let mut child = RawLegacyChildLoweringPortV1;
    build_block_input_view_with_port_v1(builder, &mut child, RawLegacyBodyInputV1::new(statements))
}

/// Thin body-input facade over the existing sequential block driver.
///
/// The legacy body is one input-view implementation. Located raw body inputs
/// will enter at this boundary only after their structural child descent is
/// implemented; this facade owns no source-site reconstruction.
pub(in crate::mir::builder) fn build_block_input_view_with_port_v1<Port, Input>(
    builder: &mut MirBuilder,
    child: &mut Port,
    input: Input,
) -> Result<ValueId, String>
where
    Port: RecursiveChildLoweringPortV1<StatementInput = ASTNode>,
    Input: RawBodyInputViewV1,
{
    build_block_with_port_v1(builder, child, input.into_legacy_body())
}

/// Run the existing sequential block driver while retaining one child port.
///
/// The driver still owns scope lifetime, suffix routing, termination, and
/// last-value selection. This thin adapter owns only the raw statement list
/// and reuses the caller's child descent for each element.  RAWPORT0 later
/// supplies `RawInvocationChildPortV1` here; the legacy facade above remains
/// the production route through M0.
pub(in crate::mir::builder) fn build_block_with_port_v1<Port>(
    builder: &mut MirBuilder,
    child: &mut Port,
    statements: Vec<ASTNode>,
) -> Result<ValueId, String>
where
    Port: RecursiveChildLoweringPortV1<StatementInput = ASTNode>,
{
    let mut port = OwnedLegacyBlockPortV1 { statements, child };
    drive_legacy_block_v1(builder, &mut port)
}

struct OwnedLegacyBlockPortV1<'port, Port> {
    statements: Vec<ASTNode>,
    child: &'port mut Port,
}

impl<Port> LegacyBlockDescentPortV1 for OwnedLegacyBlockPortV1<'_, Port>
where
    Port: RecursiveChildLoweringPortV1<StatementInput = ASTNode>,
{
    fn len(&self) -> usize {
        self.statements.len()
    }

    fn lower_statement(
        &mut self,
        builder: &mut MirBuilder,
        index: usize,
    ) -> Result<ValueId, String> {
        drive_legacy_statement_v1(builder, self.child, self.statements[index].clone())
    }
}

/// Build a single statement node
///
/// # Phase 212.5: If statement support
/// - Statement-level If (side effects only) is explicitly handled
/// - Expression-level If (value used) goes through the raw expression dispatcher
///
/// # Note
/// - While/LoopRange will be delegated to Loop lowering in the future
/// - Other shapes delegate through the raw statement/expression descent ports
pub(in crate::mir::builder) fn build_statement(
    builder: &mut MirBuilder,
    node: ASTNode,
) -> Result<ValueId, String> {
    let mut child = RawLegacyChildLoweringPortV1;
    build_statement_input_view_with_port_v1(
        builder,
        &mut child,
        RawLegacyStatementInputV1::new(node),
    )
}

/// Thin statement-input facade over the existing statement dispatcher.
///
/// It deliberately keeps the recursive child port unchanged. The selected
/// located pre-loop lineage will receive its own structural descent adapter
/// rather than widening every raw recursive port at once.
pub(in crate::mir::builder) fn build_statement_input_view_with_port_v1<Port, Input>(
    builder: &mut MirBuilder,
    child: &mut Port,
    input: Input,
) -> Result<ValueId, String>
where
    Port: RecursiveChildLoweringPortV1<
        BodyInput = Vec<ASTNode>,
        StatementInput = ASTNode,
        ExpressionInput = ASTNode,
    >,
    Input: RawStatementInputViewV1,
{
    build_statement_with_port_v1(builder, child, input.into_legacy_statement())
}

/// Run the existing statement dispatcher while retaining one raw child port.
///
/// Direct helper branches remain behavior-identical in M0. The expression
/// default expression path and statement-position If reuse `child`, so nested raw descent no
/// longer recreates a legacy port at this dispatcher boundary.
pub(in crate::mir::builder) fn build_statement_with_port_v1<Port>(
    builder: &mut MirBuilder,
    child: &mut Port,
    node: ASTNode,
) -> Result<ValueId, String>
where
    Port: RecursiveChildLoweringPortV1<
        BodyInput = Vec<ASTNode>,
        StatementInput = ASTNode,
        ExpressionInput = ASTNode,
    >,
{
    // Align current_span to this statement node before lowering expressions under it.
    builder.metadata_ctx.set_current_span(node.span());
    match node {
        // Phase 212.5: Statement としての If 処理
        node @ ASTNode::If { .. } => build_if_statement_with_port_v1(builder, child, node),
        ASTNode::StaticConstTable { .. } => {
            // Metadata-only declaration; execution observes no runtime statement.
            Ok(crate::mir::builder::emission::constant::emit_void(builder)?)
        }
        node @ ASTNode::FastMemRegion { .. } => {
            use crate::mir::resolved_semantics::BodyChildRoleV1;
            let source = child.prepare_body_child_source_v1(&node, BodyChildRoleV1::FastMemBody)?;
            let ASTNode::FastMemRegion {
                contract,
                body,
                span,
            } = node
            else {
                unreachable!()
            };
            let mut scoped = crate::mir::builder::raw_structured_child_scope::
                RawStructuredChildScopePortV1::for_body(child, source);
            super::super::fastmem::build_fastmem_region_with_port_v1(
                builder,
                &mut scoped,
                contract,
                body,
                span,
            )
        }
        // 将来ここに While / LoopRange / Match / Using など statement 専用分岐を追加する。
        other => drive_legacy_expression_v1(builder, child, other),
    }
}

pub(in crate::mir::builder) fn build_if_statement_with_port_v1<Port>(
    builder: &mut MirBuilder,
    child: &mut Port,
    node: ASTNode,
) -> Result<ValueId, String>
where
    Port: RecursiveChildLoweringPortV1<
        BodyInput = Vec<ASTNode>,
        StatementInput = ASTNode,
        ExpressionInput = ASTNode,
    >,
{
    use crate::mir::resolved_semantics::{BodyChildRoleV1, ExprChildRoleV1};
    let condition =
        child.prepare_expression_child_source_v1(&node, ExprChildRoleV1::IfCondition)?;
    let then_body = child.prepare_body_child_source_v1(&node, BodyChildRoleV1::IfThen)?;
    let else_body = match &node {
        ASTNode::If {
            else_body: Some(_), ..
        } => Some(child.prepare_body_child_source_v1(&node, BodyChildRoleV1::IfElse)?),
        _ => None,
    };
    let ASTNode::If {
        condition: condition_node,
        then_body: then_nodes,
        else_body: else_nodes,
        ..
    } = node
    else {
        return Err("[freeze:contract][raw-structured/expected-if]".to_owned());
    };
    let mut scoped =
        crate::mir::builder::raw_structured_child_scope::RawStructuredChildScopePortV1::new(
            child,
            vec![condition],
            [Some(then_body), else_body].into_iter().flatten().collect(),
        );
    let lowering = super::if_statement_descent::drive_raw_if_statement_with_port_v1(
        builder,
        &mut scoped,
        *condition_node,
        then_nodes,
        else_nodes,
    );
    super::if_statement_descent::complete_if_statement_v1(builder, lowering)
}
