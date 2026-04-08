//! Declaration Indexer - Pre-indexing symbols before lowering
//!
//! Purpose: Collect user-defined boxes and static methods for fallback resolution
//!
//! Responsibilities:
//! - Detect static box Main with main() method (app vs script mode)
//! - Index user-defined boxes and static methods before AST lowering
//! - Enable safe fallback for bare calls in using-prepended code
//!
//! Called by: `lower_root()` in module_lifecycle.rs

use super::MirBuilder;
use crate::ast::ASTNode;

/// Detect if AST contains static box Main with main() method
///
/// Used to determine:
/// - true  => App mode (Main.main is entry)
/// - false => Script/Test mode (top-level Program runs sequentially)
pub(super) fn has_main_static(ast: &ASTNode) -> bool {
    use crate::ast::ASTNode as N;
    if let N::Program { statements, .. } = ast {
        for st in statements {
            if let N::BoxDeclaration {
                name,
                methods,
                is_static,
                ..
            } = st
            {
                if *is_static && name == "Main" {
                    if let Some(m) = methods.get("main") {
                        if let N::FunctionDeclaration { .. } = m {
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}

/// Unified declaration indexing (Phase A): collect symbols before lowering
///
/// Pre-indexes:
/// - user_defined_boxes: non-static Box names (for NewBox birth() skip)
/// - static_method_index: name -> [(BoxName, arity)] (for bare-call fallback)
///
/// # Arguments
/// - `builder`: MirBuilder with comp_ctx for registration
/// - `node`: AST node to traverse
pub(super) fn index_declarations(builder: &mut MirBuilder, node: &ASTNode) {
    match node {
        ASTNode::Program { statements, .. } => {
            for st in statements {
                index_declarations(builder, st);
            }
        }
        ASTNode::BoxDeclaration {
            name,
            fields, // Phase 285LLVM-1.1: Extract fields
            field_decls,
            methods,
            is_static,
            ..
        } => {
            if !*is_static {
                // Phase 285LLVM-1.1: Register instance box with field information
                if field_decls.is_empty() {
                    builder
                        .comp_ctx
                        .register_user_box_with_fields(name.clone(), fields.clone());
                } else {
                    builder
                        .comp_ctx
                        .register_user_box_with_field_decls(name.clone(), field_decls.clone());
                }
            } else {
                // Static box: no fields
                builder.comp_ctx.register_user_box(name.clone());
                for (mname, mast) in methods {
                    if let ASTNode::FunctionDeclaration { params, .. } = mast {
                        builder
                            .comp_ctx
                            .static_method_index
                            .entry(mname.clone())
                            .or_insert_with(Vec::new)
                            .push((name.clone(), params.len()));
                    }
                }
            }
        }
        _ => {}
    }
}
