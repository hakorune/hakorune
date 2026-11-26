/*!
 * Declaration validation
 *
 */

use crate::ast::ASTNode;
use crate::parser::{NyashParser, ParseError};

impl NyashParser {
    /// Override メソッドの検証
    pub(super) fn validate_override_methods(
        &self,
        child_name: &str,
        parent_name: &str,
        methods: &crate::ast::BoxMethodInventoryV1,
    ) -> Result<(), ParseError> {
        // 現時点では簡単な検証のみ
        // TODO: 親クラスのメソッドシグネチャとの比較
        for entry in methods.iter_selected_declaration_order() {
            let method_name = entry.name();
            let method_ast = entry.declaration();
            if let ASTNode::FunctionDeclaration { is_override, .. } = method_ast {
                if *is_override {
                    // 将来的にここで親クラスのメソッドが存在するかチェック
                    if crate::parser::env::cli_verbose_enabled() {
                        crate::parser::log::debug(&format!(
                            "🔍 Validating override method '{}' in '{}' from '{}'",
                            method_name, child_name, parent_name
                        ));
                    }
                }
            }
        }
        Ok(())
    }
}
