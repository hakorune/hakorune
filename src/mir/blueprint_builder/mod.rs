//! Explicit comparison builder. The default compiler does not call this module.
//! Declaration/Resolve own source meaning; later stages only read that contract.
mod declaration_table;
mod lower;
mod resolve;
mod seal;

pub use declaration_table::{DeclarationTable, Refusal, ResolvedModule};

/// Resolve the entire module before allocating any physical MIR identities.
pub fn resolve(ast: crate::ast::ASTNode) -> Result<ResolvedModule, Refusal> {
    resolve::module(ast)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::NyashParser;

    #[test]
    fn annotated_scalar_source_resolves_before_any_mir_exists() {
        let ast =
            NyashParser::parse_from_string("static box Main { main(): i64 { return 1 + 2 } }")
                .unwrap();
        let resolved = resolve(ast).unwrap();
        assert_eq!(resolved.declarations.entries.len(), 1);
        assert_eq!(resolved.functions.len(), 1);
        let mut drafts = lower::module(&resolved).unwrap();
        seal::close(&mut drafts).unwrap();
        assert!(drafts[0]
            .function
            .blocks
            .values()
            .all(|block| block.terminator.is_some()));
        let ast =
            NyashParser::parse_from_string("static box Main { main() { return 1 } }").unwrap();
        assert_eq!(resolve(ast).unwrap_err().code, "MissingTypeAnnotation");
    }
}
