//! Producer-backed Script semantic source for the first Complete closure.
//!
//! This product is intentionally narrow: the selected runtime window must be
//! empty or literal-only before a Script owner is issued.  It borrows the
//! already-owned Program while sealing one shared forest and projection; no
//! raw source carrier can manufacture the Complete loan.

use crate::ast::ASTNode;
use crate::mir::compiler::source_projection::VerifiedSourceProjectionV1;
use crate::mir::resolved_semantics::{
    FunctionOwnerIdV1, ResolvedScriptLiteralDraftV1, SemanticOwnerForestDraftV1,
    SemanticOwnerRootProfileV1, VerifiedSemanticOwnerForestV1, VerifiedSemanticOwnerProductV1,
};

use super::normal_default_root_catalog_lifecycle::PreparedNormalDefaultProgramRootV1;

#[derive(Debug)]
pub(super) struct VerifiedScriptSemanticSourceV1<'source> {
    source: &'source PreparedNormalDefaultProgramRootV1,
    forest: VerifiedSemanticOwnerForestV1,
    projection: VerifiedSourceProjectionV1,
    literal_source_indices: Box<[usize]>,
}

impl<'source> VerifiedScriptSemanticSourceV1<'source> {
    pub(super) fn seal(
        source: &'source PreparedNormalDefaultProgramRootV1,
        owner: FunctionOwnerIdV1,
        literal_source_indices: Box<[usize]>,
    ) -> Result<Self, String> {
        let ASTNode::Program { statements, .. } = source.source_ast() else {
            return Err("[mir/script-semantic/source-root] expected Program".to_owned());
        };
        for &index in &literal_source_indices {
            if !matches!(statements.get(index), Some(ASTNode::Literal { .. })) {
                return Err(format!(
                    "[mir/script-semantic/literal-coverage] source_statement_index={index}"
                ));
            }
        }
        let product = ResolvedScriptLiteralDraftV1::new(owner)
            .seal()
            .map_err(|error| format!("[mir/script-semantic/seal] {error:?}"))?;
        let mut draft = SemanticOwnerForestDraftV1::new();
        draft
            .insert_product(owner, VerifiedSemanticOwnerProductV1::Script(product))
            .map_err(|error| format!("[mir/script-semantic/forest] {error:?}"))?;
        let forest = draft
            .seal()
            .map_err(|error| format!("[mir/script-semantic/forest] {error:?}"))?;
        let projection = VerifiedSourceProjectionV1::seal_with_root_profile(
            source.source_ast(),
            &forest,
            SemanticOwnerRootProfileV1::Script,
        )
        .map_err(|error| format!("[mir/script-semantic/projection] {error}"))?;
        Ok(Self {
            source,
            forest,
            projection,
            literal_source_indices,
        })
    }

    pub(super) fn source(&self) -> &PreparedNormalDefaultProgramRootV1 {
        &self.source
    }

    pub(super) fn forest(&self) -> &VerifiedSemanticOwnerForestV1 {
        &self.forest
    }

    pub(super) fn projection(&self) -> &VerifiedSourceProjectionV1 {
        &self.projection
    }

    pub(super) fn literal_source_indices(&self) -> &[usize] {
        &self.literal_source_indices
    }
}

#[cfg(test)]
mod tests {
    use super::VerifiedScriptSemanticSourceV1;
    use crate::mir::builder::PreparedNormalDefaultProgramRootV1;
    use crate::mir::resolved_semantics::{FunctionOwnerIssuerV1, SemanticOwnerRootProfileV1};
    use crate::parser::NyashParser;

    fn owner() -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
        FunctionOwnerIssuerV1::new_for_compilation()
            .expect("owner issuer")
            .issue()
            .expect("root owner")
    }

    #[test]
    fn literal_program_seals_one_script_owner_and_program_projection() {
        let ast = NyashParser::parse_from_string("0").expect("literal source");
        let source = PreparedNormalDefaultProgramRootV1::seal(ast).expect("Program source");
        let product = VerifiedScriptSemanticSourceV1::seal(&source, owner(), [0].into())
            .expect("literal Script product");

        assert_eq!(product.forest().owner_count(), 1);
        assert_eq!(product.forest().roots().len(), 1);
        assert_eq!(
            product
                .forest()
                .semantic_owner(product.forest().roots()[0])
                .expect("Script owner")
                .root_profile(),
            SemanticOwnerRootProfileV1::Script
        );
        assert_eq!(product.literal_source_indices(), &[0]);
        assert!(product
            .projection()
            .owner_root(source.source_ast(), product.forest().roots()[0])
            .is_ok());
    }

    #[test]
    fn literal_product_rejects_a_non_literal_source_ordinal() {
        let ast = NyashParser::parse_from_string("0").expect("literal source");
        let source = PreparedNormalDefaultProgramRootV1::seal(ast).expect("Program source");
        let error = VerifiedScriptSemanticSourceV1::seal(&source, owner(), [1].into())
            .expect_err("out-of-range literal coverage must reject");

        assert!(error.contains("literal-coverage"));
    }
}
