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
    let (candidate, records) = lower_and_seal(&source)?;
    crate::mir::MirVerifier::new_strict()
        .verify_document_module(&candidate)
        .map_err(|errors| format!("blueprint/internal/structure: {errors:?}"))?;
    correspondence::verify(&source, &candidate, &records)?;
    Ok(candidate)
}

fn lower_and_seal(
    source: &ResolvedModule,
) -> Result<(crate::mir::MirModule, Vec<lower::Correspondence>), String> {
    let mut drafts = lower::module(source)?;
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
    Ok((candidate, records))
}

#[cfg(test)]
mod tests;
