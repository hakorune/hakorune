//! Parser-owned body-source transaction for the bounded instance-method row.
//!
//! This module is intentionally AST-free after `into_parts`. It is the only
//! parser path that may pair the rich parse product with the declaration
//! handoff for body-source work. It does not issue semantic facts, owners,
//! targets, Recipe data, or MIR.

use std::collections::BTreeSet;

use crate::ast::ASTNode;

use super::source_resolver_handoff::{
    build_resolver_source_handoff, ParserBoxResolverSourceHandoffV1,
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
        f: impl FnOnce(
            &ResolverSourceInvocationProvenanceV1,
            &[ParserBoxMethodBodySourceRowV1],
        ) -> R,
    ) -> R {
        f(&self.parser_provenance, &self.rows)
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
        let (ast, seals, _, _) = self.product.into_postpass_parts();
        let handoff = build_resolver_source_handoff(&ast, &seals)
            .map_err(BodySourceTransactionErrorV1::ResolverHandoff)?;
        let rows = collect_body_rows(&ast, &seals, &handoff)?;
        let envelope = ParserBoxBodySourceEnvelopeV1 {
            parser_provenance: handoff.parser_provenance(),
            rows: rows.into_boxed_slice(),
        };
        Ok((handoff, envelope))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BodySourceTransactionErrorV1 {
    ResolverHandoff(ResolverSourceHandoffErrorV1),
    ProgramNotAvailable,
    BoxSealMissing { statement_ordinal: u32 },
    MethodInventoryMissing { name: Box<str> },
    MethodDeclarationUnsupported { name: Box<str> },
    StaticMethodUnsupported { name: Box<str> },
    BodyItemOrdinalOverflow { name: Box<str> },
    DuplicateSourceSite { statement_ordinal: u32, member_ordinal: u32 },
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

#[cfg(test)]
#[path = "body_source_tests.rs"]
mod body_source_tests;
