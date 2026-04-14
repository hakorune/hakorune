use crate::ast::ASTNode;
use crate::mir::join_ir::lowering::complex_addend_normalizer::{
    ComplexAddendNormalizer, NormalizationResult,
};

pub(super) fn normalize_loop_break_body_complex_addends(body: &[ASTNode]) -> Option<Vec<ASTNode>> {
    let mut normalized_body = Vec::new();
    let mut has_normalization = false;

    for node in body {
        match ComplexAddendNormalizer::normalize_assign(node) {
            NormalizationResult::Normalized {
                temp_def,
                new_assign,
                ..
            } => {
                normalized_body.push(temp_def);
                normalized_body.push(new_assign);
                has_normalization = true;
            }
            NormalizationResult::Unchanged => normalized_body.push(node.clone()),
        }
    }

    has_normalization.then_some(normalized_body)
}
