//! Pre-body validation for SelectedNormal top-level physical projections.
//!
//! This validator observes the already-issued source inventory and the
//! existing draft admission projection.  It does not issue a target, touch
//! the collector, or create a semantic receipt.

use std::collections::BTreeMap;

use crate::ast::ASTNode;

use crate::mir::builder::callable_declaration_catalog::{
    SelectedNormalCallableKeyV1, SelectedNormalCallableSourceSiteV1,
    VerifiedSelectedNormalCallableSourceInventoryV1,
};
use crate::mir::builder::normal_top_level_function_admission::NormalTopLevelFunctionDraftAdmissionV1;

pub(super) fn validate_selected_normal_top_level_projections(
    statements: &[ASTNode],
    sources: &VerifiedSelectedNormalCallableSourceInventoryV1,
) -> Result<(), String> {
    let mut symbols = BTreeMap::<String, usize>::new();

    for (statement_index, statement) in statements.iter().enumerate() {
        let ASTNode::FunctionDeclaration { name, params, .. } = statement else {
            continue;
        };
        let source_key = sources.top_level_function(statement_index).ok_or_else(|| {
            format!(
                "[freeze:contract][mir/selected-normal/source-missing] statement={statement_index}"
            )
        })?;
        if source_key.declared_name() != name || source_key.declared_arity() != params.len() {
            return Err(format!(
                "[freeze:contract][mir/selected-normal/source-projection-mismatch] statement={statement_index}"
            ));
        }
        let admission =
            NormalTopLevelFunctionDraftAdmissionV1::from_catalog_key(source_key.clone());
        let symbol = admission.physical_symbol().to_owned();
        if let Some(first_statement_index) = symbols.insert(symbol.clone(), statement_index) {
            return Err(format!(
                "[freeze:contract][mir/selected-normal/duplicate-physical-projection] symbol={symbol} first_statement={first_statement_index} second_statement={statement_index}"
            ));
        }
    }

    for (key, site) in sources.entries() {
        let SelectedNormalCallableKeyV1::TopLevel(key) = key else {
            continue;
        };
        let SelectedNormalCallableSourceSiteV1::ProgramFunction { statement_index } = site else {
            return Err(
                "[freeze:contract][mir/selected-normal/top-level-site-kind-mismatch]".to_owned(),
            );
        };
        let Some(ASTNode::FunctionDeclaration { name, params, .. }) =
            statements.get(*statement_index)
        else {
            return Err(format!(
                "[freeze:contract][mir/selected-normal/source-statement-missing] statement={statement_index}"
            ));
        };
        if key.declared_name() != name || key.declared_arity() != params.len() {
            return Err(format!(
                "[freeze:contract][mir/selected-normal/source-row-mismatch] statement={statement_index}"
            ));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::validate_selected_normal_top_level_projections;
    use crate::ast::{ASTNode, DeclarationAttrs, Span};
    use crate::mir::builder::callable_declaration_catalog::VerifiedSameModuleCallableDeclarationCatalogV1;

    fn function(name: &str, arity: usize) -> ASTNode {
        ASTNode::FunctionDeclaration {
            name: name.to_owned(),
            params: (0..arity).map(|index| format!("arg{index}")).collect(),
            param_decls: Vec::new(),
            return_type_name: None,
            body: Vec::new(),
            uses: Vec::new(),
            contracts: Vec::new(),
            is_static: true,
            is_override: false,
            attrs: DeclarationAttrs::default(),
            span: Span::unknown(),
        }
    }

    #[test]
    fn accepts_unique_selected_normal_physical_projections() {
        let root = ASTNode::Program {
            statements: vec![function("first", 0), function("second", 1)],
            span: Span::unknown(),
        };
        let catalog = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&root)
            .expect("selected callable catalog");
        let ASTNode::Program { statements, .. } = root else {
            unreachable!()
        };
        validate_selected_normal_top_level_projections(
            &statements,
            catalog.selected_source_inventory(),
        )
        .expect("unique selected projections");
    }

    #[test]
    fn rejects_same_name_and_arity_selected_normal_physical_projection() {
        let root = ASTNode::Program {
            statements: vec![function("same", 0), function("same", 0)],
            span: Span::unknown(),
        };
        let catalog = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&root)
            .expect("selected callable catalog");
        let ASTNode::Program { statements, .. } = root else {
            unreachable!()
        };
        let error = validate_selected_normal_top_level_projections(
            &statements,
            catalog.selected_source_inventory(),
        )
        .expect_err("duplicate selected projection");
        assert!(error.contains("duplicate-physical-projection"));
        assert!(error.contains("symbol=same/0"));
        assert!(error.contains("first_statement=0"));
        assert!(error.contains("second_statement=1"));
    }
}
