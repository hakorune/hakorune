//! Parser-owned body-source transaction for the bounded instance-method row.
//!
//! This module is intentionally AST-free after `into_parts`. It is the only
//! parser path that may pair the rich parse product with the declaration
//! handoff for body-source work. It does not issue semantic facts, owners,
//! targets, Recipe data, or MIR.

use std::collections::BTreeSet;

use crate::ast::ASTNode;

use super::source_resolver_handoff::{
    build_resolver_source_handoff, ParserBoxResolverSourceHandoffV1, ResolverBoxMethodSourceSiteV1,
    ResolverSourceHandoffErrorV1, ResolverSourceInvocationProvenanceV1,
};
use super::source_seal::{ParsedProgramWithSourceV1, ParserBoxSourceSealV1};
use super::{NyashParser, ParseError, ParserBuildConfig};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ParserBoxMethodBodySourceRowV1 {
    source_site: super::source_resolver_handoff::ResolverBoxMethodSourceSiteV1,
    name: Box<str>,
    body_item_ordinals: Box<[u32]>,
}

impl ParserBoxMethodBodySourceRowV1 {
    pub(crate) fn source_site(
        &self,
    ) -> super::source_resolver_handoff::ResolverBoxMethodSourceSiteV1 {
        self.source_site
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn body_item_ordinals(&self) -> &[u32] {
        &self.body_item_ordinals
    }
}

/// AST-free body source envelope consumed by exactly one resolver observer.
#[derive(Debug)]
pub(crate) struct ParserBoxBodySourceEnvelopeV1 {
    parser_provenance: ResolverSourceInvocationProvenanceV1,
    rows: Box<[ParserBoxMethodBodySourceRowV1]>,
}

impl ParserBoxBodySourceEnvelopeV1 {
    pub(crate) fn consume_with<R>(
        self,
        f: impl FnOnce(&ResolverSourceInvocationProvenanceV1, &[ParserBoxMethodBodySourceRowV1]) -> R,
    ) -> R {
        f(&self.parser_provenance, &self.rows)
    }
}

/// Parser-private borrowed syntax lease for the resolver owner issuer.
///
/// The lease is created only while the rich parser transaction still owns the
/// AST. It is deliberately not an AST-free semantic product: the callback
/// must consume it before returning an owned resolver carrier.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ParserBoxInstanceMethodSyntaxRowV1<'ast> {
    source_site: ResolverBoxMethodSourceSiteV1,
    name: &'ast str,
    params: &'ast [String],
    body: &'ast [ASTNode],
}

impl<'ast> ParserBoxInstanceMethodSyntaxRowV1<'ast> {
    pub(crate) const fn source_site(self) -> ResolverBoxMethodSourceSiteV1 {
        self.source_site
    }

    pub(crate) const fn name(self) -> &'ast str {
        self.name
    }

    pub(crate) const fn params(self) -> &'ast [String] {
        self.params
    }

    pub(crate) const fn body(self) -> &'ast [ASTNode] {
        self.body
    }
}

/// One-shot syntax lease paired with the AST-free parser handoff and body
/// envelope. This type is parser-private and must never be stored downstream.
#[derive(Debug)]
pub(crate) struct ParserBoxInstanceMethodSyntaxLeaseV1<'ast> {
    parser_provenance: ResolverSourceInvocationProvenanceV1,
    rows: Box<[ParserBoxInstanceMethodSyntaxRowV1<'ast>]>,
}

impl<'ast> ParserBoxInstanceMethodSyntaxLeaseV1<'ast> {
    pub(crate) fn parser_provenance(&self) -> &ResolverSourceInvocationProvenanceV1 {
        &self.parser_provenance
    }

    pub(crate) fn rows(&self) -> &[ParserBoxInstanceMethodSyntaxRowV1<'ast>] {
        &self.rows
    }
}

/// One-shot parser transaction. The rich AST and source seals are consumed
/// once; only the declaration handoff and normalized body envelope escape.
#[derive(Debug)]
pub(crate) struct ParserResolverBodyTransactionV1 {
    product: ParsedProgramWithSourceV1,
}

impl ParserResolverBodyTransactionV1 {
    pub(super) fn new(product: ParsedProgramWithSourceV1) -> Self {
        Self { product }
    }

    pub(crate) fn into_parts(
        self,
    ) -> Result<
        (
            ParserBoxResolverSourceHandoffV1,
            ParserBoxBodySourceEnvelopeV1,
        ),
        BodySourceTransactionErrorV1,
    > {
        let (program, seals, _, _) = self.product.into_postpass_parts();
        let ast = program.ast();
        let handoff = build_resolver_source_handoff(ast, &seals)
            .map_err(BodySourceTransactionErrorV1::ResolverHandoff)?;
        let rows = collect_body_rows(ast, &seals, &handoff)?;
        let envelope = ParserBoxBodySourceEnvelopeV1 {
            parser_provenance: handoff.parser_provenance(),
            rows: rows.into_boxed_slice(),
        };
        Ok((handoff, envelope))
    }

    /// Run one resolver callback while the rich parser AST remains borrowed.
    ///
    /// The higher-ranked callback return prevents an AST/syntax borrow from
    /// escaping. The callback may return only owned AST-free resolver data.
    pub(crate) fn with_direct_method_syntax<R>(
        self,
        callback: impl for<'ast> FnOnce(
            ParserBoxResolverSourceHandoffV1,
            ParserBoxBodySourceEnvelopeV1,
            ParserBoxInstanceMethodSyntaxLeaseV1<'ast>,
            super::release_source::ParserReleaseStatementSourceCatalogV1,
        ) -> R,
    ) -> Result<R, BodySourceTransactionErrorV1> {
        let (program, seals, _, _) = self.product.into_postpass_parts();
        let ast = program.ast();
        let handoff = build_resolver_source_handoff(ast, &seals)
            .map_err(BodySourceTransactionErrorV1::ResolverHandoff)?;
        let rows = collect_body_rows(ast, &seals, &handoff)?;
        let syntax_lease = collect_syntax_lease(ast, &seals, &handoff)?;
        let release_sources = super::release_source::collect_release_sources(&syntax_lease)?;
        let envelope = ParserBoxBodySourceEnvelopeV1 {
            parser_provenance: handoff.parser_provenance(),
            rows: rows.into_boxed_slice(),
        };
        Ok(callback(handoff, envelope, syntax_lease, release_sources))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BodySourceTransactionErrorV1 {
    ResolverHandoff(ResolverSourceHandoffErrorV1),
    ProgramNotAvailable,
    BoxSealMissing {
        statement_ordinal: u32,
    },
    MethodInventoryMissing {
        name: Box<str>,
    },
    MethodDeclarationUnsupported {
        name: Box<str>,
    },
    StaticMethodUnsupported {
        name: Box<str>,
    },
    BodyItemOrdinalOverflow {
        name: Box<str>,
    },
    DuplicateSourceSite {
        statement_ordinal: u32,
        member_ordinal: u32,
    },
    ReleaseSource(super::release_source::ReleaseSourceIssueV1),
}

impl NyashParser {
    pub(crate) fn parse_from_string_with_resolver_body_source(
        input: impl Into<String>,
        build_config: ParserBuildConfig,
    ) -> Result<ParserResolverBodyTransactionV1, ParseError> {
        let product = Self::parse_from_string_with_source_seal(input, build_config)?;
        Ok(ParserResolverBodyTransactionV1::new(product))
    }
}

fn collect_body_rows(
    ast: &ASTNode,
    seals: &[ParserBoxSourceSealV1],
    handoff: &ParserBoxResolverSourceHandoffV1,
) -> Result<Vec<ParserBoxMethodBodySourceRowV1>, BodySourceTransactionErrorV1> {
    let statements = match ast {
        ASTNode::Program { statements, .. } => statements,
        _ => return Err(BodySourceTransactionErrorV1::ProgramNotAvailable),
    };
    let mut seen_sites = BTreeSet::new();
    let mut rows = Vec::new();
    for box_row in handoff.boxes() {
        let statement_ordinal = box_row.statement_ordinal();
        let ASTNode::BoxDeclaration { .. } = statements
            .get(statement_ordinal as usize)
            .ok_or(BodySourceTransactionErrorV1::ProgramNotAvailable)?
        else {
            return Err(BodySourceTransactionErrorV1::ProgramNotAvailable);
        };
        let seal = seals
            .iter()
            .find(|seal| seal.box_site().statement_ordinal() == statement_ordinal)
            .ok_or(BodySourceTransactionErrorV1::BoxSealMissing { statement_ordinal })?;
        for method in box_row.methods() {
            let source_site = method.source_site();
            if !seen_sites.insert((
                source_site.box_statement_ordinal(),
                source_site.member_ordinal(),
            )) {
                return Err(BodySourceTransactionErrorV1::DuplicateSourceSite {
                    statement_ordinal: source_site.box_statement_ordinal(),
                    member_ordinal: source_site.member_ordinal(),
                });
            }
            let entry = seal
                .inventory()
                .iter_selected_declaration_order()
                .find(|entry| entry.site() == method.inventory_ordinal())
                .ok_or_else(|| BodySourceTransactionErrorV1::MethodInventoryMissing {
                    name: method.name().to_owned().into_boxed_str(),
                })?;
            let ASTNode::FunctionDeclaration {
                body, is_static, ..
            } = entry.declaration()
            else {
                return Err(BodySourceTransactionErrorV1::MethodDeclarationUnsupported {
                    name: method.name().to_owned().into_boxed_str(),
                });
            };
            if *is_static {
                return Err(BodySourceTransactionErrorV1::StaticMethodUnsupported {
                    name: method.name().to_owned().into_boxed_str(),
                });
            }
            let body_item_ordinals = (0..body.len())
                .map(u32::try_from)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| BodySourceTransactionErrorV1::BodyItemOrdinalOverflow {
                    name: method.name().to_owned().into_boxed_str(),
                })?;
            rows.push(ParserBoxMethodBodySourceRowV1 {
                source_site,
                name: method.name().to_owned().into_boxed_str(),
                body_item_ordinals: body_item_ordinals.into_boxed_slice(),
            });
        }
    }
    Ok(rows)
}

fn collect_syntax_lease<'ast>(
    ast: &'ast ASTNode,
    seals: &'ast [ParserBoxSourceSealV1],
    handoff: &ParserBoxResolverSourceHandoffV1,
) -> Result<ParserBoxInstanceMethodSyntaxLeaseV1<'ast>, BodySourceTransactionErrorV1> {
    let statements = match ast {
        ASTNode::Program { statements, .. } => statements,
        _ => return Err(BodySourceTransactionErrorV1::ProgramNotAvailable),
    };
    let mut rows = Vec::new();
    for box_row in handoff.boxes() {
        let statement_ordinal = box_row.statement_ordinal();
        let ASTNode::BoxDeclaration { .. } = statements
            .get(statement_ordinal as usize)
            .ok_or(BodySourceTransactionErrorV1::ProgramNotAvailable)?
        else {
            return Err(BodySourceTransactionErrorV1::ProgramNotAvailable);
        };
        let seal = seals
            .iter()
            .find(|seal| seal.box_site().statement_ordinal() == statement_ordinal)
            .ok_or(BodySourceTransactionErrorV1::BoxSealMissing { statement_ordinal })?;
        for method in box_row.methods() {
            let entry = seal
                .inventory()
                .iter_selected_declaration_order()
                .find(|entry| entry.site() == method.inventory_ordinal())
                .ok_or_else(|| BodySourceTransactionErrorV1::MethodInventoryMissing {
                    name: method.name().to_owned().into_boxed_str(),
                })?;
            let ASTNode::FunctionDeclaration {
                params,
                body,
                is_static,
                ..
            } = entry.declaration()
            else {
                return Err(BodySourceTransactionErrorV1::MethodDeclarationUnsupported {
                    name: method.name().to_owned().into_boxed_str(),
                });
            };
            if *is_static {
                return Err(BodySourceTransactionErrorV1::StaticMethodUnsupported {
                    name: method.name().to_owned().into_boxed_str(),
                });
            }
            rows.push(ParserBoxInstanceMethodSyntaxRowV1 {
                source_site: method.source_site(),
                name: entry.name(),
                params,
                body,
            });
        }
    }
    Ok(ParserBoxInstanceMethodSyntaxLeaseV1 {
        parser_provenance: handoff.parser_provenance(),
        rows: rows.into_boxed_slice(),
    })
}

#[cfg(test)]
#[path = "body_source_tests.rs"]
mod body_source_tests;
