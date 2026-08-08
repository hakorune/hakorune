//! Parser-private C-S1 delegate target lookup.
//!
//! This module borrows the open postpass product. It does not mutate the AST,
//! inventory, source seals, or generated delegate rows. Names are query
//! selectors only; exact parser brand, Box path, and explicit source relation
//! remain the authority.

use crate::ast::{ASTNode, BoxMethodInventoryV1, FieldDecl};

use super::source_authority::{
    DelegateSourceDeclarationV1, ExplicitMethodSourceRelationV1, MethodSourceRelationV1,
};
use super::source_path::SourceBoxDeclarationPathV1;
use super::source_seal::{OpenParserPostpassProductV1, PreparedBoxSourceSealV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DelegateTargetIndexErrorV1 {
    SourceAlignmentUnavailable,
    ForeignBrand,
    DuplicateBoxPath,
    DuplicateBoxName,
    SealPathMismatch,
    MethodRelationMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum DelegateTargetResolutionV1<'product> {
    Candidate(TargetMethodRefV1<'product>),
    Declined,
    Unresolved,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct TargetMethodRefV1<'product> {
    target_box_path: SourceBoxDeclarationPathV1,
    method: &'product ExplicitMethodSourceRelationV1,
}

impl<'product> TargetMethodRefV1<'product> {
    pub(super) fn target_box_path(&self) -> &SourceBoxDeclarationPathV1 {
        &self.target_box_path
    }

    pub(super) fn method_name(&self) -> &str {
        self.method.name()
    }

    pub(super) fn method_source_relation(&self) -> &ExplicitMethodSourceRelationV1 {
        self.method
    }
}

#[derive(Debug)]
pub(super) struct DelegateTargetIndexV1<'product> {
    entries: Box<[TargetBoxEntryV1<'product>]>,
}

#[derive(Debug)]
struct TargetBoxEntryV1<'product> {
    path: SourceBoxDeclarationPathV1,
    box_name: &'product str,
    field_decls: &'product [FieldDecl],
    inventory: &'product BoxMethodInventoryV1,
    method_relations: &'product [MethodSourceRelationV1],
}

impl<'product> DelegateTargetIndexV1<'product> {
    pub(super) fn issue(
        ast: &'product ASTNode,
        prepared_source_seals: &'product [PreparedBoxSourceSealV1],
        final_box_paths: &'product [SourceBoxDeclarationPathV1],
    ) -> Result<Self, DelegateTargetIndexErrorV1> {
        let ASTNode::Program { statements, .. } = ast else {
            return Err(DelegateTargetIndexErrorV1::SourceAlignmentUnavailable);
        };
        let boxes = statements
            .iter()
            .filter_map(|statement| match statement {
                ASTNode::BoxDeclaration {
                    name, field_decls, ..
                } => Some((name.as_str(), field_decls.as_slice())),
                _ => None,
            })
            .collect::<Vec<_>>();
        if boxes.len() != final_box_paths.len() || boxes.len() != prepared_source_seals.len() {
            return Err(DelegateTargetIndexErrorV1::SourceAlignmentUnavailable);
        }

        let brand = final_box_paths
            .first()
            .map(|path| path.brand())
            .or_else(|| {
                prepared_source_seals
                    .first()
                    .map(|seal| seal.box_site.path().brand())
            })
            .ok_or(DelegateTargetIndexErrorV1::SourceAlignmentUnavailable)?;
        let mut entries = Vec::with_capacity(boxes.len());
        for (index, ((box_name, field_decls), (path, seal))) in boxes
            .iter()
            .zip(final_box_paths.iter().zip(prepared_source_seals.iter()))
            .enumerate()
        {
            if path.brand() != brand || seal.brand != *brand {
                return Err(DelegateTargetIndexErrorV1::ForeignBrand);
            }
            if final_box_paths[..index]
                .iter()
                .any(|previous| previous == path)
            {
                return Err(DelegateTargetIndexErrorV1::DuplicateBoxPath);
            }
            if entries
                .iter()
                .any(|entry: &TargetBoxEntryV1<'product>| entry.box_name == *box_name)
            {
                return Err(DelegateTargetIndexErrorV1::DuplicateBoxName);
            }
            if seal.box_site.path() != path {
                return Err(DelegateTargetIndexErrorV1::SealPathMismatch);
            }
            validate_method_relations(seal)?;
            entries.push(TargetBoxEntryV1 {
                path: path.clone(),
                box_name,
                field_decls,
                inventory: &seal.inventory,
                method_relations: &seal.method_relations,
            });
        }
        Ok(Self {
            entries: entries.into_boxed_slice(),
        })
    }

    pub(super) fn resolve(
        &self,
        delegate: &DelegateSourceDeclarationV1,
    ) -> DelegateTargetResolutionV1<'product> {
        let host_path = delegate.source_site().box_site().path();
        let Some(host) = self.entries.iter().find(|entry| &entry.path == host_path) else {
            return DelegateTargetResolutionV1::Rejected;
        };
        let mut fields = host
            .field_decls
            .iter()
            .filter(|field| field.name == delegate.delegate_field_name());
        let Some(field) = fields.next() else {
            return DelegateTargetResolutionV1::Unresolved;
        };
        if fields.next().is_some() {
            return DelegateTargetResolutionV1::Rejected;
        }
        let Some(target_name) = field.declared_type_name.as_deref() else {
            return DelegateTargetResolutionV1::Unresolved;
        };
        let mut targets = self
            .entries
            .iter()
            .filter(|entry| entry.box_name == target_name);
        let Some(target) = targets.next() else {
            return DelegateTargetResolutionV1::Declined;
        };
        if targets.next().is_some() {
            return DelegateTargetResolutionV1::Rejected;
        }
        let mut relations = target
            .method_relations
            .iter()
            .filter_map(|relation| match relation {
                MethodSourceRelationV1::Explicit(explicit)
                    if explicit.name() == delegate.source_method_name() =>
                {
                    Some(explicit)
                }
                _ => None,
            });
        let Some(method) = relations.next() else {
            return DelegateTargetResolutionV1::Rejected;
        };
        if relations.next().is_some() {
            return DelegateTargetResolutionV1::Rejected;
        }
        if method.source_site().box_site().path() != &target.path
            || target
                .inventory
                .get(method.name())
                .map(|entry| entry.site())
                != Some(method.inventory_ordinal())
        {
            return DelegateTargetResolutionV1::Rejected;
        }
        DelegateTargetResolutionV1::Candidate(TargetMethodRefV1 {
            target_box_path: target.path.clone(),
            method,
        })
    }

    pub(super) fn method_declaration(
        &self,
        target: &TargetMethodRefV1<'product>,
    ) -> Option<&'product ASTNode> {
        self.entries
            .iter()
            .find(|entry| entry.path == target.target_box_path)
            .and_then(|entry| entry.inventory.get(target.method_name()))
            .map(|entry| entry.declaration())
    }
}

fn validate_method_relations(
    seal: &PreparedBoxSourceSealV1,
) -> Result<(), DelegateTargetIndexErrorV1> {
    for (index, relation) in seal.method_relations.iter().enumerate() {
        if seal.method_relations[..index]
            .iter()
            .any(|previous| previous.name() == relation.name())
        {
            return Err(DelegateTargetIndexErrorV1::MethodRelationMismatch);
        }
        if let MethodSourceRelationV1::Explicit(explicit) = relation {
            let Some(entry) = seal.inventory.get(explicit.name()) else {
                return Err(DelegateTargetIndexErrorV1::MethodRelationMismatch);
            };
            if entry.site() != explicit.inventory_ordinal()
                || explicit.source_site().box_site() != seal.box_site()
            {
                return Err(DelegateTargetIndexErrorV1::MethodRelationMismatch);
            }
        }
    }
    Ok(())
}

impl OpenParserPostpassProductV1 {
    pub(super) fn issue_delegate_target_index(
        &self,
    ) -> Result<DelegateTargetIndexV1<'_>, DelegateTargetIndexErrorV1> {
        DelegateTargetIndexV1::issue(
            &self.ast,
            &self.source_session.prepared_source_seals,
            &self.final_box_paths,
        )
    }

    pub(super) fn delegate_source_declarations(
        &self,
    ) -> impl Iterator<Item = &DelegateSourceDeclarationV1> {
        self.source_session
            .prepared_source_seals
            .iter()
            .flat_map(PreparedBoxSourceSealV1::delegate_source_declarations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{NyashParser, ParserBuildConfig};

    fn open_product(source: &str) -> OpenParserPostpassProductV1 {
        let config = ParserBuildConfig::default();
        let pre = super::super::normalize_logical_ops(source);
        let mut tokenizer =
            crate::tokenizer::NyashTokenizer::with_grammar_profile(pre, config.grammar_profile);
        let tokens = tokenizer.tokenize().expect("test source must tokenize");
        let mut parser = NyashParser::new(tokens);
        parser.build_config = config;
        let ast = parser.parse_program().expect("test source must parse");
        OpenParserPostpassProductV1::new(
            ast,
            std::mem::take(&mut parser.prepared_source_seals),
            parser.take_source_build_gate_records(),
            parser.take_metadata(),
        )
        .prune_build_gates(&parser)
        .expect("test source must produce an open postpass product")
    }

    #[test]
    fn c_s1_positive_target_is_exact_and_reusable() {
        let product = open_product(
            r#"
box Target { run() { return 1 } }
box Host {
    target: Target
    delegate target exposes { run as runAlias }
}
"#,
        );
        let index = product
            .issue_delegate_target_index()
            .expect("ordinary same-brand target index should issue");
        let row = product
            .delegate_source_declarations()
            .next()
            .expect("delegate row should be transported");
        let first = index.resolve(row);
        let second = index.resolve(row);
        let DelegateTargetResolutionV1::Candidate(first) = first else {
            panic!("expected an exact target candidate");
        };
        let DelegateTargetResolutionV1::Candidate(second) = second else {
            panic!("expected the borrowed target to be reusable");
        };
        assert_eq!(first.method_name(), "run");
        assert_eq!(first.target_box_path(), second.target_box_path());
    }

    #[test]
    fn c_s1_missing_field_is_unresolved_without_partial_target() {
        let product = open_product(
            r#"
box Target { run() { return 1 } }
box Host { delegate target exposes { run as runAlias } }
"#,
        );
        let index = product.issue_delegate_target_index().unwrap();
        let row = product.delegate_source_declarations().next().unwrap();
        assert_eq!(index.resolve(row), DelegateTargetResolutionV1::Unresolved);
    }

    #[test]
    fn c_s1_missing_method_is_rejected_not_fallback() {
        let product = open_product(
            r#"
box Target { run() { return 1 } }
box Host {
    target: Target
    delegate target exposes { missing as missingAlias }
}
"#,
        );
        let index = product.issue_delegate_target_index().unwrap();
        let row = product.delegate_source_declarations().next().unwrap();
        assert_eq!(index.resolve(row), DelegateTargetResolutionV1::Rejected);
    }

    #[test]
    fn c_s1_duplicate_target_name_rejects_index() {
        let product = open_product(
            r#"
box Target { run() { return 1 } }
box Target { run() { return 2 } }
"#,
        );
        assert!(matches!(
            product.issue_delegate_target_index(),
            Err(DelegateTargetIndexErrorV1::DuplicateBoxName)
        ));
    }
}
