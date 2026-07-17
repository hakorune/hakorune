//! Behavior-neutral recursive child-lowering port.
//!
//! This module owns the typed body, statement, and expression entry boundary.
//! It owns no source navigation, callable-result plan, location, ledger,
//! MethodCall route, or result-publication policy.

use crate::ast::ASTNode;
use crate::mir::{MirBuilder, ValueId};

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

struct RawLegacyChildLoweringPortV1;

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
        builder.build_expression_impl(input)
    }
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
