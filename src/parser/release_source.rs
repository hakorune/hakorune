//! Parser-owned source carrier for contextual `release root` statements.
//!
//! This module records syntax identity only. It does not resolve the root,
//! prove Home ownership or availability, or issue an executable operation.

use crate::ast::ASTNode;

use super::body_source::ParserBoxInstanceMethodSyntaxLeaseV1;
use super::{ResolverBoxMethodSourceSiteV1, ResolverSourceInvocationProvenanceV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ParserReleaseStatementSourceV1 {
    method_source_site: ResolverBoxMethodSourceSiteV1,
    body_statement_ordinal: u32,
    root: Box<str>,
}

impl ParserReleaseStatementSourceV1 {
    pub(crate) const fn method_source_site(&self) -> ResolverBoxMethodSourceSiteV1 {
        self.method_source_site
    }

    pub(crate) const fn body_statement_ordinal(&self) -> u32 {
        self.body_statement_ordinal
    }

    pub(crate) fn root(&self) -> &str {
        &self.root
    }
}

/// Non-Clone authority emitted once from the rich parser transaction.
#[derive(Debug)]
pub(crate) struct ParserReleaseStatementSourceCatalogV1 {
    parser_provenance: ResolverSourceInvocationProvenanceV1,
    rows: Box<[ParserReleaseStatementSourceV1]>,
}

impl ParserReleaseStatementSourceCatalogV1 {
    pub(crate) fn parser_provenance(&self) -> &ResolverSourceInvocationProvenanceV1 {
        &self.parser_provenance
    }

    pub(crate) fn rows(&self) -> &[ParserReleaseStatementSourceV1] {
        &self.rows
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ReleaseSourceIssueV1 {
    BodyStatementOrdinalOverflow {
        method_source_site: ResolverBoxMethodSourceSiteV1,
    },
    NestedReleaseOutsideI0 {
        method_source_site: ResolverBoxMethodSourceSiteV1,
        body_statement_ordinal: u32,
    },
}

pub(super) fn collect_release_sources(
    syntax: &ParserBoxInstanceMethodSyntaxLeaseV1<'_>,
) -> Result<ParserReleaseStatementSourceCatalogV1, super::body_source::BodySourceTransactionErrorV1>
{
    let mut rows = Vec::new();
    for method in syntax.rows() {
        for (index, statement) in method.body().iter().enumerate() {
            let body_statement_ordinal = u32::try_from(index).map_err(|_| {
                super::body_source::BodySourceTransactionErrorV1::ReleaseSource(
                    ReleaseSourceIssueV1::BodyStatementOrdinalOverflow {
                        method_source_site: method.source_site(),
                    },
                )
            })?;
            match statement {
                ASTNode::Release { root, .. } => rows.push(ParserReleaseStatementSourceV1 {
                    method_source_site: method.source_site(),
                    body_statement_ordinal,
                    root: root.clone().into_boxed_str(),
                }),
                other if contains_release(other) => {
                    return Err(
                        super::body_source::BodySourceTransactionErrorV1::ReleaseSource(
                            ReleaseSourceIssueV1::NestedReleaseOutsideI0 {
                                method_source_site: method.source_site(),
                                body_statement_ordinal,
                            },
                        ),
                    )
                }
                _ => {}
            }
        }
    }
    Ok(ParserReleaseStatementSourceCatalogV1 {
        parser_provenance: syntax.parser_provenance().clone(),
        rows: rows.into_boxed_slice(),
    })
}

fn contains_release(node: &ASTNode) -> bool {
    matches!(node, ASTNode::Release { .. }) || node.any_child(contains_release)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{NyashParser, ParserBuildConfig};

    fn release_catalog(source: &str) -> ParserReleaseStatementSourceCatalogV1 {
        NyashParser::parse_from_string_with_resolver_body_source(
            source,
            ParserBuildConfig::default(),
        )
        .expect("source should parse")
        .with_direct_method_syntax(|_, _, _, releases| releases)
        .expect("release catalog should issue")
    }

    #[test]
    fn direct_release_rows_keep_exact_method_and_body_identity() {
        let catalog =
            release_catalog("box Owner { finish(root: Node) { release root\nrelease other } }");
        assert_eq!(catalog.rows().len(), 2);
        assert_eq!(catalog.rows()[0].method_source_site().member_ordinal(), 0);
        assert_eq!(catalog.rows()[0].body_statement_ordinal(), 0);
        assert_eq!(catalog.rows()[0].root(), "root");
        assert_eq!(catalog.rows()[1].body_statement_ordinal(), 1);
        assert_eq!(catalog.rows()[1].root(), "other");
    }

    #[test]
    fn parenthesized_release_stays_an_ordinary_call() {
        let catalog = release_catalog("box Owner { finish(root: Node) { release(root) } }");
        assert!(catalog.rows().is_empty());
    }

    #[test]
    fn ordinary_release_binding_and_method_call_stay_neutral() {
        let catalog = release_catalog(
            "box Owner { finish(root: Node) { local release = root\nroot.release() } }",
        );
        assert!(catalog.rows().is_empty());
    }

    #[test]
    fn nested_release_is_not_silently_omitted() {
        let transaction = NyashParser::parse_from_string_with_resolver_body_source(
            "box Owner { finish(root: Node) { if true { release root } } }",
            ParserBuildConfig::default(),
        )
        .expect("source should parse");
        let error = transaction
            .with_direct_method_syntax(|_, _, _, _| ())
            .expect_err("nested release is outside the I0 source cohort");
        assert!(matches!(
            error,
            super::super::body_source::BodySourceTransactionErrorV1::ReleaseSource(
                ReleaseSourceIssueV1::NestedReleaseOutsideI0 { .. }
            )
        ));
    }

    #[test]
    fn projected_release_root_is_rejected_by_the_exact_parser_boundary() {
        let error = NyashParser::parse_from_string_with_resolver_body_source(
            "box Owner { finish(root: Node) { release root.field } }",
            ParserBuildConfig::default(),
        )
        .expect_err("projected release must fail at syntax commitment");
        assert!(error
            .to_string()
            .contains("parser/release_exact_root_required"));
    }

    #[test]
    fn receiver_keyword_is_rejected_by_the_exact_parser_boundary() {
        let error = NyashParser::parse_from_string_with_resolver_body_source(
            "box Owner { finish(root: Node) { release me } }",
            ParserBuildConfig::default(),
        )
        .expect_err("receiver keyword must not become a release root");
        assert!(error
            .to_string()
            .contains("parser/release_exact_root_required"));
    }

    #[test]
    fn newline_breaks_contextual_release_recognition() {
        let catalog = release_catalog("box Owner { finish(root: Node) { release\nroot } }");
        assert!(catalog.rows().is_empty());
    }
}
