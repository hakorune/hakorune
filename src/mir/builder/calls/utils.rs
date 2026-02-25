//! 🎯 箱理論: Call処理のユーティリティ関数群
//!
//! 責務:
//! - 型名パース（parse_type_name_to_mir）
//! - 文字列リテラル抽出（extract_string_literal）
//! - Call結果アノテーション（annotate_call_result_from_func_name）
//! - Call target解決（resolve_call_target）

use crate::ast::ASTNode;
use crate::mir::builder::{MirBuilder, MirType, ValueId};

/// Map a user-facing type name to MIR type
pub fn parse_type_name_to_mir(name: &str) -> MirType {
    super::special_handlers::parse_type_name_to_mir(name)
}

/// Extract string literal from AST node if possible
pub fn extract_string_literal(node: &ASTNode) -> Option<String> {
    super::special_handlers::extract_string_literal(node)
}

impl MirBuilder {
    /// Annotate a call result `dst` with the return type and origin if the callee
    /// is a known user/static function in the current module.
    pub(in crate::mir::builder) fn annotate_call_result_from_func_name<S: AsRef<str>>(
        &mut self,
        dst: ValueId,
        func_name: S,
    ) {
        super::annotation::annotate_call_result_from_func_name(self, dst, func_name)
    }

    /// Resolve function call target to type-safe Callee
    /// Implements the core logic of compile-time function resolution
    pub(in crate::mir::builder) fn resolve_call_target(
        &self,
        name: &str,
    ) -> Result<crate::mir::definitions::call_unified::Callee, String> {
        super::method_resolution::resolve_call_target(
            name,
            &self.comp_ctx.current_static_box,
            &self.variable_ctx.variable_map,
        )
    }
}
