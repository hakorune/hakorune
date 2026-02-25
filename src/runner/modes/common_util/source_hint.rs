use crate::ast::ASTNode;
use crate::mir::{MirCompileResult, MirCompiler};

/// Compile AST with a source filename hint, reducing call-site duplication.
/// Falls back to regular compile when filename is None or empty.
pub fn compile_with_source_hint(
    compiler: &mut MirCompiler,
    ast: ASTNode,
    filename: Option<&str>,
) -> Result<MirCompileResult, String> {
    if let Some(f) = filename {
        if !f.is_empty() {
            return compiler.compile_with_source(ast, Some(f));
        }
    }
    compiler.compile_with_source(ast, None)
}
