use super::MirInstruction;
use crate::ast::Span;

/// MIR instruction bundled with its source span.
#[derive(Debug, Clone)]
pub struct SpannedInstruction {
    pub inst: MirInstruction,
    pub span: Span,
}

/// Reference view of a MIR instruction bundled with its span.
#[derive(Debug, Clone, Copy)]
pub struct SpannedInstRef<'a> {
    pub inst: &'a MirInstruction,
    pub span: Span,
}
