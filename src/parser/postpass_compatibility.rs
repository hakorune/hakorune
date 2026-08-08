//! Explicit compatibility postpass arm for the public AST parser family.
//!
//! This module is deliberately not a fallback from the ordinary source-seal
//! arm. The total S0 coordinator selects it from a typed cohort before any
//! delegate lowering starts. It returns AST only; it never issues a source
//! authority product.

use crate::ast::ASTNode;

use super::ParseError;

pub(super) fn lower(ast: ASTNode) -> Result<ASTNode, ParseError> {
    super::delegate_lowering::lower_delegate_exposes(ast)
}
