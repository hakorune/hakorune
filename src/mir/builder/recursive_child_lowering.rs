//! Behavior-neutral recursive child-lowering port.
//!
//! This module owns the typed body, statement, and expression entry boundary.
//! It owns no source navigation, callable-result plan, location, ledger,
//! MethodCall route, or result-publication policy.

use crate::ast::ASTNode;
use crate::mir::{MirBuilder, ValueId};

const MAX_RAW_EXPRESSION_RECURSION_DEPTH: usize = 200;

pub(in crate::mir::builder) trait RecursiveChildLoweringPortV1 {
    type BodyInput;
    type StatementInput;
    type ExpressionInput;

    fn lower_body(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::BodyInput,
    ) -> Result<ValueId, String>;

    fn lower_statement(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::StatementInput,
    ) -> Result<ValueId, String>;

    fn lower_expression(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::ExpressionInput,
    ) -> Result<ValueId, String>;
}

pub(in crate::mir::builder) fn drive_legacy_body_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    input: Port::BodyInput,
) -> Result<ValueId, String>
where
    Port: RecursiveChildLoweringPortV1,
{
    port.lower_body(builder, input)
}

pub(in crate::mir::builder) fn drive_legacy_statement_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    input: Port::StatementInput,
) -> Result<ValueId, String>
where
    Port: RecursiveChildLoweringPortV1,
{
    port.lower_statement(builder, input)
}

pub(in crate::mir::builder) fn drive_legacy_expression_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    input: Port::ExpressionInput,
) -> Result<ValueId, String>
where
    Port: RecursiveChildLoweringPortV1,
{
    port.lower_expression(builder, input)
}

pub(in crate::mir::builder) struct RawLegacyChildLoweringPortV1;

impl RecursiveChildLoweringPortV1 for RawLegacyChildLoweringPortV1 {
    type BodyInput = Vec<ASTNode>;
    type StatementInput = ASTNode;
    type ExpressionInput = ASTNode;

    fn lower_body(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::BodyInput,
    ) -> Result<ValueId, String> {
        super::stmts::block_stmt::build_block(builder, input)
    }

    fn lower_statement(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::StatementInput,
    ) -> Result<ValueId, String> {
        super::stmts::block_stmt::build_statement(builder, input)
    }

    fn lower_expression(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::ExpressionInput,
    ) -> Result<ValueId, String> {
        lower_raw_expression_with_recursion_guard_v1(builder, input)
    }
}

fn lower_raw_expression_with_recursion_guard_v1(
    builder: &mut MirBuilder,
    input: ASTNode,
) -> Result<ValueId, String> {
    builder.recursion_depth += 1;
    let current_depth = builder.recursion_depth;
    if current_depth > MAX_RAW_EXPRESSION_RECURSION_DEPTH {
        let ring0 = crate::runtime::get_global_ring0();
        ring0
            .log
            .error("\n[FATAL] ============================================");
        ring0.log.error(&format!(
            "[FATAL] Recursion depth exceeded {} in build_expression",
            MAX_RAW_EXPRESSION_RECURSION_DEPTH
        ));
        ring0
            .log
            .error(&format!("[FATAL] Current depth: {current_depth}"));
        ring0.log.error(&format!(
            "[FATAL] AST node type: {:?}",
            std::mem::discriminant(&input)
        ));
        ring0
            .log
            .error("[FATAL] ============================================\n");
        builder.recursion_depth -= 1;
        return Err(format!(
            "Recursion depth exceeded: {current_depth} (possible infinite loop)"
        ));
    }

    let result = builder.build_expression_impl(input);
    builder.recursion_depth -= 1;
    result
}

pub(in crate::mir::builder) fn drive_raw_legacy_body_v1(
    builder: &mut MirBuilder,
    input: Vec<ASTNode>,
) -> Result<ValueId, String> {
    let mut port = RawLegacyChildLoweringPortV1;
    drive_legacy_body_v1(builder, &mut port, input)
}

pub(in crate::mir::builder) fn drive_raw_legacy_statement_v1(
    builder: &mut MirBuilder,
    input: ASTNode,
) -> Result<ValueId, String> {
    let mut port = RawLegacyChildLoweringPortV1;
    drive_legacy_statement_v1(builder, &mut port, input)
}

pub(in crate::mir::builder) fn drive_raw_legacy_expression_v1(
    builder: &mut MirBuilder,
    input: ASTNode,
) -> Result<ValueId, String> {
    let mut port = RawLegacyChildLoweringPortV1;
    drive_legacy_expression_v1(builder, &mut port, input)
}
