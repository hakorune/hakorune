//! Declaration Indexer - Pre-indexing symbols before lowering
//!
//! Purpose: Collect non-callable declaration facts before lowering
//!
//! Responsibilities:
//! - Detect static box Main with main() method (app vs script mode)
//! - Index user-defined boxes before AST lowering
//! - Record narrow verified static scalar facts
//!
//! Called by: `lower_root()` in module_lifecycle.rs

use super::declaration_order::sorted_method_entries;
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
/// Callable membership and method bodies belong to the complete immutable
/// callable declaration catalog installed by `lower_root` before this pass.
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
        ASTNode::BrandDeclaration {
            name,
            underlying_type_name,
            ..
        } => {
            builder
                .comp_ctx
                .register_brand_decl(name.clone(), underlying_type_name.clone());
        }
        ASTNode::EnumDeclaration {
            name,
            variants,
            type_parameters,
            ..
        } => {
            builder.comp_ctx.register_enum_decl(
                name.clone(),
                type_parameters.clone(),
                variants.clone(),
            );
        }
        ASTNode::BoxDeclaration {
            name,
            fields, // Phase 285LLVM-1.1: Extract fields
            field_decls,
            methods,
            is_static,
            is_record,
            is_sync,
            init_fields,
            weak_fields,
            type_parameters,
            ..
        } => {
            if *is_sync {
                return;
            }
            if *is_record {
                builder.comp_ctx.register_record_decl(
                    name.clone(),
                    type_parameters.clone(),
                    field_decls,
                );
                return;
            }
            if !*is_static {
                // Phase 285LLVM-1.1: Register instance box with field information
                builder.comp_ctx.register_user_box_declared_fields(
                    name.clone(),
                    fields,
                    field_decls,
                    init_fields,
                    weak_fields,
                );
            } else {
                // Static box: no fields
                builder.comp_ctx.register_user_box(name.clone());
                for (mname, mast) in sorted_method_entries(methods) {
                    if let ASTNode::FunctionDeclaration { params, body, .. } = mast {
                        let func_name =
                            format!("{}.{}{}", name, mname, format!("/{}", params.len()));
                        if name == "HakoAllocObjectLifecycleFacadeReason" {
                            builder
                                .comp_ctx
                                .register_static_scalar_method_fact_if_verified(
                                    &func_name, params, body,
                                );
                        }
                    }
                }
            }
        }
        _ => {}
    }
}
