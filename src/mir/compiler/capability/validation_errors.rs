use crate::ast::ASTNode;

use super::super::lowering_input::CanonicalLoweringErrorV1;

pub(super) fn source_navigation(error: impl ToString) -> CanonicalLoweringErrorV1 {
    CanonicalLoweringErrorV1::SourceNavigation {
        detail: error.to_string(),
    }
}

pub(super) fn unsupported<T>(
    site: impl Into<String>,
    node: &ASTNode,
    reason: &'static str,
) -> Result<T, CanonicalLoweringErrorV1> {
    Err(CanonicalLoweringErrorV1::UnsupportedFirstFamilyShape {
        site: site.into(),
        actual: node.node_type(),
        reason,
    })
}
