//! Parser-private exact declaration loan for callable parameter resolution.
//!
//! This is unpublished transaction staging, not a semantic product. It uses
//! the placement already sealed beside each source row and never searches by
//! Box or method name.

use crate::ast::{ASTNode, BoxMethodSourceSelectionV1};

use super::catalog::ParserCallableParameterSourceCatalogV1;
use super::model::{ParserCallableDeclarationKindV1, ParserCallableParameterDeclarationSourceV1};

#[derive(Debug)]
pub(crate) struct ParserCallableDeclarationSyntaxRefV1<'ast> {
    source_row_index: u32,
    declaration: &'ast ASTNode,
}

impl<'ast> ParserCallableDeclarationSyntaxRefV1<'ast> {
    pub(crate) const fn source_row_index(&self) -> u32 {
        self.source_row_index
    }

    pub(crate) const fn declaration(&self) -> &'ast ASTNode {
        self.declaration
    }
}

#[derive(Debug)]
pub(crate) struct ParserCallableDeclarationSyntaxLoanV1<'ast> {
    declarations: Box<[ParserCallableDeclarationSyntaxRefV1<'ast>]>,
}

impl<'ast> ParserCallableDeclarationSyntaxLoanV1<'ast> {
    pub(crate) fn declarations(&self) -> &[ParserCallableDeclarationSyntaxRefV1<'ast>] {
        &self.declarations
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ParserCallableSyntaxLoanErrorV1 {
    ProgramNotAvailable,
    ParameterSourceUnavailable,
    CompositeSourceReadyCannotBeDiscarded,
    BoxDeclarationMissing { statement: u32 },
    BoxKindMismatch { statement: u32 },
    InventoryOrdinalOverflow { statement: u32, member: u32 },
    MethodPlacementMissing { statement: u32, inventory: u32 },
    MethodPlacementMismatch { statement: u32, inventory: u32 },
    NonDirectMethod { statement: u32, inventory: u32 },
    MethodIdentityMismatch { statement: u32, inventory: u32 },
    FunctionDeclarationMissing { statement: u32, inventory: u32 },
    ParameterCoverageMismatch { statement: u32, inventory: u32 },
    SourceRowOrdinalOverflow,
}

pub(in crate::parser) fn borrow_callable_declaration_syntax_v1<'ast>(
    ast: &'ast ASTNode,
    catalog: &ParserCallableParameterSourceCatalogV1,
) -> Result<ParserCallableDeclarationSyntaxLoanV1<'ast>, ParserCallableSyntaxLoanErrorV1> {
    let ASTNode::Program { statements, .. } = ast else {
        return Err(ParserCallableSyntaxLoanErrorV1::ProgramNotAvailable);
    };
    let mut rows = Vec::with_capacity(catalog.declarations().len());
    for (source_row_index, source) in catalog.declarations().iter().enumerate() {
        let statement = source.box_statement_ordinal();
        let Some(ASTNode::BoxDeclaration {
            methods, is_static, ..
        }) = statements.get(statement as usize)
        else {
            return Err(ParserCallableSyntaxLoanErrorV1::BoxDeclarationMissing { statement });
        };
        if *is_static
            != matches!(
                source.kind(),
                ParserCallableDeclarationKindV1::StaticBoxMethod
            )
        {
            return Err(ParserCallableSyntaxLoanErrorV1::BoxKindMismatch { statement });
        }
        let inventory = source.inventory_ordinal().inventory_ordinal();
        let placement = usize::try_from(inventory).map_err(|_| {
            ParserCallableSyntaxLoanErrorV1::InventoryOrdinalOverflow {
                statement,
                member: source.source_member_ordinal(),
            }
        })?;
        let Some(entry) = methods.iter_selected_declaration_order().nth(placement) else {
            return Err(ParserCallableSyntaxLoanErrorV1::MethodPlacementMissing {
                statement,
                inventory,
            });
        };
        if entry.site() != source.inventory_ordinal() {
            return Err(ParserCallableSyntaxLoanErrorV1::MethodPlacementMismatch {
                statement,
                inventory,
            });
        }
        if !matches!(
            entry.provenance().explicit_source_selection(),
            Some(BoxMethodSourceSelectionV1::Direct)
        ) {
            return Err(ParserCallableSyntaxLoanErrorV1::NonDirectMethod {
                statement,
                inventory,
            });
        }
        let ASTNode::FunctionDeclaration {
            name,
            params,
            param_decls,
            is_static: method_is_static,
            ..
        } = entry.declaration()
        else {
            return Err(
                ParserCallableSyntaxLoanErrorV1::FunctionDeclarationMissing {
                    statement,
                    inventory,
                },
            );
        };
        if entry.name() != source.diagnostic_name()
            || name != source.diagnostic_name()
            || *method_is_static != *is_static
        {
            return Err(ParserCallableSyntaxLoanErrorV1::MethodIdentityMismatch {
                statement,
                inventory,
            });
        }
        if !parameters_match(source, params, param_decls) {
            return Err(ParserCallableSyntaxLoanErrorV1::ParameterCoverageMismatch {
                statement,
                inventory,
            });
        }
        let source_row_index = u32::try_from(source_row_index)
            .map_err(|_| ParserCallableSyntaxLoanErrorV1::SourceRowOrdinalOverflow)?;
        rows.push(ParserCallableDeclarationSyntaxRefV1 {
            source_row_index,
            declaration: entry.declaration(),
        });
    }
    Ok(ParserCallableDeclarationSyntaxLoanV1 {
        declarations: rows.into_boxed_slice(),
    })
}

fn parameters_match(
    source: &ParserCallableParameterDeclarationSourceV1,
    names: &[String],
    declarations: &[crate::ast::ParamDecl],
) -> bool {
    source.parameters().len() == names.len()
        && names.len() == declarations.len()
        && source.parameters().iter().enumerate().all(|(index, row)| {
            row.ordinal() == u32::try_from(index).unwrap_or(u32::MAX)
                && row.name() == names[index]
                && row.name() == declarations[index].name
                && row.declared_type().as_deref()
                    == declarations[index].declared_type_name.as_deref()
                && row.transfer().is_ordinary()
        })
}

#[cfg(test)]
mod tests {
    use crate::ast::{ASTNode, ParamDecl};
    use crate::parser::source_authority::{
        ParserInvocationBrandV1, SourceBoxDeclarationSiteV1, SourceBoxMemberSiteV1,
        SourceBoxMethodSiteV1,
    };
    use crate::parser::source_path::SourceBoxDeclarationPathV1;
    use crate::parser::NyashParser;
    use crate::parser::callable_source_anchor::CallableDeclarationAnchorV1;

    use super::*;
    use crate::parser::callable_parameter_source::model::ParserCallableParameterSourceRowV1;

    fn fixture_catalog(
        kind: ParserCallableDeclarationKindV1,
        diagnostic_name: &str,
        declared_type_name: Option<&str>,
    ) -> (ASTNode, ParserCallableParameterSourceCatalogV1) {
        let ast =
            NyashParser::parse_from_string("box Sample { field run(value) { return value } }")
                .unwrap();
        let ASTNode::Program { statements, .. } = &ast else {
            unreachable!("fixture parses as Program")
        };
        let ASTNode::BoxDeclaration { methods, .. } = &statements[0] else {
            unreachable!("fixture contains one Box")
        };
        let inventory_ordinal = methods
            .iter_selected_declaration_order()
            .next()
            .expect("fixture method")
            .site();
        let brand = ParserInvocationBrandV1::issue();
        let source_site = SourceBoxMethodSiteV1::Direct {
            member: SourceBoxMemberSiteV1::new(
                SourceBoxDeclarationSiteV1::from_path(SourceBoxDeclarationPathV1::root(
                    brand.clone(),
                    0,
                )),
                1,
            ),
        };
        let parameter = ParamDecl {
            name: "value".to_owned(),
            declared_type_name: declared_type_name.map(str::to_owned),
        };
        let declaration = ParserCallableParameterDeclarationSourceV1::new(
            source_site,
            inventory_ordinal,
            CallableDeclarationAnchorV1::issue().identity(),
            kind,
            diagnostic_name.to_owned(),
            vec![ParserCallableParameterSourceRowV1::ordinary(0, &parameter)].into_boxed_slice(),
        );
        (
            ast,
            ParserCallableParameterSourceCatalogV1::new(
                brand,
                vec![declaration].into_boxed_slice(),
            ),
        )
    }

    #[test]
    fn rejects_static_instance_cross_wiring() {
        let (ast, catalog) = fixture_catalog(
            ParserCallableDeclarationKindV1::StaticBoxMethod,
            "run",
            None,
        );
        assert_eq!(
            borrow_callable_declaration_syntax_v1(&ast, &catalog).unwrap_err(),
            ParserCallableSyntaxLoanErrorV1::BoxKindMismatch { statement: 0 }
        );
    }

    #[test]
    fn rejects_diagnostic_name_repair() {
        let (ast, catalog) = fixture_catalog(
            ParserCallableDeclarationKindV1::InstanceBoxMethod,
            "renamed",
            None,
        );
        assert_eq!(
            borrow_callable_declaration_syntax_v1(&ast, &catalog).unwrap_err(),
            ParserCallableSyntaxLoanErrorV1::MethodIdentityMismatch {
                statement: 0,
                inventory: 0,
            }
        );
    }

    #[test]
    fn rejects_parameter_type_reconstruction() {
        let (ast, catalog) = fixture_catalog(
            ParserCallableDeclarationKindV1::InstanceBoxMethod,
            "run",
            Some("i64"),
        );
        assert_eq!(
            borrow_callable_declaration_syntax_v1(&ast, &catalog).unwrap_err(),
            ParserCallableSyntaxLoanErrorV1::ParameterCoverageMismatch {
                statement: 0,
                inventory: 0,
            }
        );
    }
}
