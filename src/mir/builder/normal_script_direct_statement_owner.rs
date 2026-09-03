//! Selected-normal Script statement terminals that already have typed owners.
//!
//! Source classification stays in `normal_script_program_item_admission`.
//! This sibling only hands a preselected statement to its existing production
//! owner through the caller's current invocation port.

use crate::ast::ASTNode;
use crate::mir::builder::emission::constant::emit_void;
use crate::mir::builder::fastmem::build_fastmem_region_with_port_v1;
use crate::mir::builder::module_lifecycle::RootCallableCapturePortV1;
use crate::mir::builder::raw_expression_dispatch::unsupported_raw_ast_node_error_v1;
use crate::mir::builder::raw_structured_child_scope::RawStructuredChildScopePortV1;
use crate::mir::builder::recursive_child_lowering::{
    drive_legacy_expression_v1, with_legacy_expression_recursion_guard_v1,
};
use crate::mir::builder::stmts::if_statement_descent::{
    complete_if_statement_v1, drive_raw_if_statement_with_port_v1,
};
use crate::mir::builder::stmts::print_stmt::lower_raw_print_statement_with_port_v1;
use crate::mir::{MirBuilder, ValueId};

use super::normal_script_program_item_admission::is_direct_selected_unsupported_statement_v1;

pub(super) fn lower_direct_print_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    statement: ASTNode,
) -> Result<ValueId, String>
where
    Port: RootCallableCapturePortV1,
{
    if !matches!(&statement, ASTNode::Print { .. }) {
        return Err("[freeze:contract][mir/script-runtime/print-source-drift]".to_owned());
    }
    lower_raw_print_statement_with_port_v1(builder, port, statement)
}

pub(super) fn lower_direct_port_aware_expression_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    statement: ASTNode,
) -> Result<ValueId, String>
where
    Port: RootCallableCapturePortV1,
{
    drive_legacy_expression_v1(builder, port, statement)
}

pub(super) fn lower_direct_if_statement_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    statement: ASTNode,
) -> Result<ValueId, String>
where
    Port: RootCallableCapturePortV1,
{
    use crate::mir::resolved_semantics::{BodyChildRoleV1, ExprChildRoleV1};
    builder.metadata_ctx.set_current_span(statement.span());
    let condition_source =
        port.prepare_expression_child_source_v1(&statement, ExprChildRoleV1::IfCondition)?;
    let then_source = port.prepare_body_child_source_v1(&statement, BodyChildRoleV1::IfThen)?;
    let else_source = if matches!(
        &statement,
        ASTNode::If {
            else_body: Some(_),
            ..
        }
    ) {
        Some(port.prepare_body_child_source_v1(&statement, BodyChildRoleV1::IfElse)?)
    } else {
        None
    };
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = statement
    else {
        return Err("[freeze:contract][mir/script-runtime/if-source-drift]".to_owned());
    };
    let mut scoped = RawStructuredChildScopePortV1::new(
        port,
        vec![condition_source],
        [Some(then_source), else_source]
            .into_iter()
            .flatten()
            .collect(),
    );
    let lowering =
        drive_raw_if_statement_with_port_v1(builder, &mut scoped, *condition, then_body, else_body);
    complete_if_statement_v1(builder, lowering)
}

pub(super) fn lower_direct_fastmem_region_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    statement: ASTNode,
) -> Result<ValueId, String>
where
    Port: RootCallableCapturePortV1,
{
    builder.metadata_ctx.set_current_span(statement.span());
    let source = port.prepare_body_child_source_v1(
        &statement,
        crate::mir::resolved_semantics::BodyChildRoleV1::FastMemBody,
    )?;
    let ASTNode::FastMemRegion {
        contract,
        body,
        span,
    } = statement
    else {
        return Err("[freeze:contract][mir/script-runtime/fastmem-source-drift]".to_owned());
    };
    let mut scoped = RawStructuredChildScopePortV1::for_body(port, source);
    build_fastmem_region_with_port_v1(builder, &mut scoped, contract, body, span)
}

pub(super) fn lower_direct_selected_unsupported_statement_v1(
    builder: &mut MirBuilder,
    statement: &ASTNode,
) -> Result<ValueId, String> {
    if !is_direct_selected_unsupported_statement_v1(statement) {
        return Err(
            "[freeze:contract][mir/script-runtime/unsupported-statement-source-drift]".to_owned(),
        );
    }
    builder.metadata_ctx.set_current_span(statement.span());
    let node_kind = std::mem::discriminant(statement);
    with_legacy_expression_recursion_guard_v1(builder, node_kind, |_| {
        Err(unsupported_raw_ast_node_error_v1(statement))
    })
}

pub(super) fn lower_direct_static_const_runtime_completion_v1(
    builder: &mut MirBuilder,
    statement: &ASTNode,
) -> Result<ValueId, String> {
    if !matches!(statement, ASTNode::StaticConstTable { .. }) {
        return Err("[freeze:contract][mir/script-runtime/static-const-source-drift]".to_owned());
    }
    builder.metadata_ctx.set_current_span(statement.span());
    emit_void(builder)
}

#[cfg(test)]
mod tests;
