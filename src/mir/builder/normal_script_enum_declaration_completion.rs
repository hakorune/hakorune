//! Selected-Script completion for source-only enum declarations.
//!
//! Enum facts are collected and installed by the Program declaration-facts
//! owner. This terminal only preserves the statement's existing Void result;
//! it owns neither enum inventory nor constructor or match lowering.

use crate::ast::ASTNode;
use crate::mir::builder::emission::constant::emit_void;
use crate::mir::{MirBuilder, ValueId};

pub(super) fn lower_direct_enum_declaration_runtime_completion_v1(
    builder: &mut MirBuilder,
    statement: &ASTNode,
) -> Result<ValueId, String> {
    if !matches!(statement, ASTNode::EnumDeclaration { .. }) {
        return Err(
            "[freeze:contract][mir/script-runtime/enum-declaration-source-drift]".to_owned(),
        );
    }
    builder.metadata_ctx.set_current_span(statement.span());
    emit_void(builder)
}
