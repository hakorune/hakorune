//! Block statement execution module
//!
//! ## Purpose
//! Sequential statement execution with JoinIR suffix router integration
//!
//! ## Responsibilities
//! - Block/statement execution coordination
//! - Phase 142 JoinIR suffix router integration (NormalizedShadowSuffixRouterBox)
//! - Termination checking
//! - Expression delegation
//!
//! ## Architecture
//! - Phase 142 suffix router is the JoinIR integration point
//! - Block execution coordinates statement → expression → block recursion
//! - Termination checking prevents duplicate terminators
//!
//! ## Integration Points
//! - Called by: control_flow::cf_block, expression building code
//! - Calls: build_statement, build_expression, suffix router
//! - Critical: Phase 142 JoinIR suffix router integration must be preserved

use super::block_driver::{drive_legacy_block_v1, LegacyBlockDescentPortV1};
use crate::ast::ASTNode;
use crate::mir::builder::MirBuilder;
use crate::mir::builder::ValueId;

/// Build a block by sequentially processing statements
///
/// This is a critical integration point for Phase 142 JoinIR suffix router.
/// The suffix router can consume multiple statements and return the count,
/// allowing the loop to skip ahead.
///
/// # Phase 142 Integration
/// - Uses NormalizedShadowSuffixRouterBox for JoinIR route-shape detection
/// - Suffix router can consume statements and return consumed count
/// - Loop continues processing subsequent statements after suffix match
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
    let mut port = OwnedLegacyBlockPortV1 { statements };
    drive_legacy_block_v1(builder, &mut port)
}

struct OwnedLegacyBlockPortV1 {
    statements: Vec<ASTNode>,
}

impl LegacyBlockDescentPortV1 for OwnedLegacyBlockPortV1 {
    type SuffixInput<'a>
        = &'a [ASTNode]
    where
        Self: 'a;

    fn len(&self) -> usize {
        self.statements.len()
    }

    fn suffix_route_input(&self, index: usize) -> Result<Option<Self::SuffixInput<'_>>, String> {
        Ok(Some(&self.statements[index..]))
    }

    fn lower_statement(
        &mut self,
        builder: &mut MirBuilder,
        index: usize,
    ) -> Result<ValueId, String> {
        build_statement(builder, self.statements[index].clone())
    }
}

/// Build a single statement node
///
/// # Phase 212.5: If statement support
/// - Statement-level If (side effects only) is explicitly handled
/// - Expression-level If (value used) goes through build_expression
///
/// # Note
/// - While/LoopRange will be delegated to Loop lowering in the future
/// - Currently delegates to build_expression like other specialized builders
pub(in crate::mir::builder) fn build_statement(
    builder: &mut MirBuilder,
    node: ASTNode,
) -> Result<ValueId, String> {
    // Align current_span to this statement node before lowering expressions under it.
    builder.metadata_ctx.set_current_span(node.span());
    match node {
        // Phase 212.5: Statement としての If 処理
        ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            // Statement としての If - 既存 If lowering を呼ぶ
            let lowering = builder.build_if_statement(*condition, then_body, else_body);
            super::if_statement_descent::complete_if_statement_v1(builder, lowering)
        }
        ASTNode::StaticConstTable { .. } => {
            // Metadata-only declaration; execution observes no runtime statement.
            Ok(crate::mir::builder::emission::constant::emit_void(builder)?)
        }
        ASTNode::FastMemRegion {
            contract,
            body,
            span,
        } => super::super::fastmem::build_fastmem_region(builder, contract, body, span),
        // 将来ここに While / LoopRange / Match / Using など statement 専用分岐を追加する。
        other => builder.build_expression(other),
    }
}
