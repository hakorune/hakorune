//! Stack-depth guard shared by both raw expression ports.

use crate::ast::ASTNode;
use crate::mir::{MirBuilder, ValueId};

const MAX_RAW_EXPRESSION_RECURSION_DEPTH: usize = 200;

pub(in crate::mir::builder) fn with_legacy_expression_recursion_guard_v1<F>(
    builder: &mut MirBuilder,
    node_kind: std::mem::Discriminant<ASTNode>,
    lower: F,
) -> Result<ValueId, String>
where
    F: FnOnce(&mut MirBuilder) -> Result<ValueId, String>,
{
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
        ring0
            .log
            .error(&format!("[FATAL] AST node type: {:?}", node_kind));
        ring0
            .log
            .error("[FATAL] ============================================\n");
        builder.recursion_depth -= 1;
        return Err(format!(
            "Recursion depth exceeded: {current_depth} (possible infinite loop)"
        ));
    }

    let result = lower(builder);
    builder.recursion_depth -= 1;
    result
}
