//! Explicit comparison builder. The default compiler does not call this module.
//! Declaration/Resolve own source meaning; later stages only read that contract.
mod correspondence;
mod declaration_table;
mod lower;
mod resolve;
mod seal;

pub use declaration_table::{DeclarationTable, Refusal, ResolvedModule};

/// Resolve the entire module before allocating any physical MIR identities.
pub fn resolve(ast: crate::ast::ASTNode) -> Result<ResolvedModule, Refusal> {
    resolve::module(ast)
}

/// Build privately, verify every function, and publish all or nothing.
pub fn compile(ast: crate::ast::ASTNode) -> Result<crate::mir::MirModule, String> {
    let source = resolve(ast).map_err(|error| error.to_string())?;
    let mut drafts = lower::module(&source)?;
    seal::close(&mut drafts)?;
    let mut candidate = crate::mir::MirModule::new("blueprint".into());
    let mut records = Vec::new();
    for draft in drafts {
        let declaration = &source.declarations.entries[draft.declaration.0];
        candidate
            .add_cataloged_box_method(declaration.key.clone(), draft.function)
            .map_err(|error| format!("blueprint/internal/publication: {error}"))?;
        records.push(draft.correspondence);
    }
    crate::mir::MirVerifier::new_strict()
        .verify_document_module(&candidate)
        .map_err(|errors| format!("blueprint/internal/structure: {errors:?}"))?;
    correspondence::verify(&source, &candidate, &records)?;
    Ok(candidate)
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
