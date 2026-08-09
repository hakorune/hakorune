//! One-shot parser-to-resolver source handoff for the bounded Box cohort.
//!
//! This module consumes the parser's final non-Clone source seals and emits
//! only AST-free source syntax. It does not resolve names/types, issue Home
//! ABI, or construct a target/Recipe product.

use std::collections::HashSet;

use crate::ast::{ASTNode, BoxMethodInventoryOrdinalV1};

use super::callable_contract_syntax::CallableContractSyntaxV1;
use super::callable_parameter_source::{
    project_neutral_parameter_syntax_v1, ResolverMethodParameterSyntaxV1,
};
use super::source_authority::ParserInvocationBrandV1;
use super::source_seal::{ParsedProgramWithSourceV1, ParserBoxSourceSealV1};
use super::{NyashParser, ParseError, ParserBuildConfig};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ResolverMethodSignatureSyntaxV1 {
    parameters: Box<[ResolverMethodParameterSyntaxV1]>,
    return_type_name: Option<Box<str>>,
    is_static: bool,
}

impl ResolverMethodSignatureSyntaxV1 {
    pub(crate) fn parameters(&self) -> &[ResolverMethodParameterSyntaxV1] {
        &self.parameters
    }

    pub(crate) fn return_type_name(&self) -> Option<&str> {
        self.return_type_name.as_deref()
    }

    pub(crate) fn is_static(&self) -> bool {
        self.is_static
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ResolverBoxMethodSourceSiteV1 {
    box_statement_ordinal: u32,
    member_ordinal: u32,
}

impl ResolverBoxMethodSourceSiteV1 {
    pub(crate) fn box_statement_ordinal(self) -> u32 {
        self.box_statement_ordinal
    }

    pub(crate) fn member_ordinal(self) -> u32 {
        self.member_ordinal
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ResolverBoxMethodSourceRowV1 {
    source_site: ResolverBoxMethodSourceSiteV1,
    inventory_ordinal: BoxMethodInventoryOrdinalV1,
    name: Box<str>,
    signature: ResolverMethodSignatureSyntaxV1,
    callable_contract: Option<CallableContractSyntaxV1>,
}

impl ResolverBoxMethodSourceRowV1 {
    pub(crate) fn source_site(&self) -> ResolverBoxMethodSourceSiteV1 {
        self.source_site
    }

    pub(crate) fn inventory_ordinal(&self) -> BoxMethodInventoryOrdinalV1 {
        self.inventory_ordinal
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn signature(&self) -> &ResolverMethodSignatureSyntaxV1 {
        &self.signature
    }

    pub(crate) fn callable_contract(&self) -> Option<&CallableContractSyntaxV1> {
        self.callable_contract.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ResolverBoxSourceRowV1 {
    statement_ordinal: u32,
    name: Box<str>,
    methods: Box<[ResolverBoxMethodSourceRowV1]>,
}

impl ResolverBoxSourceRowV1 {
    pub(crate) fn statement_ordinal(&self) -> u32 {
        self.statement_ordinal
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn methods(&self) -> &[ResolverBoxMethodSourceRowV1] {
        &self.methods
    }
}

/// One-shot source authority transferred out of the parser.
///
/// The invocation brand is deliberately private. A resolver can consume this
/// product, but cannot forge or independently compare a parser brand.
#[derive(Debug)]
pub(crate) struct ParserBoxResolverSourceHandoffV1 {
    brand: ParserInvocationBrandV1,
    boxes: Box<[ResolverBoxSourceRowV1]>,
}

/// Opaque parser-invocation provenance carried into resolver products.
///
/// The resolver may retain and compare this membership evidence, but it
/// cannot use it as nominal type identity or mint another parser seal.
#[derive(Debug, Clone)]
pub(crate) struct ResolverSourceInvocationProvenanceV1(ParserInvocationBrandV1);

impl ResolverSourceInvocationProvenanceV1 {
    pub(crate) fn same_as(&self, other: &Self) -> bool {
        self.0.same_as(&other.0)
    }
}

impl ParserBoxResolverSourceHandoffV1 {
    pub(crate) fn boxes(&self) -> &[ResolverBoxSourceRowV1] {
        &self.boxes
    }

    /// Consume the handoff without dropping its parser invocation brand.
    ///
    /// The next resolver issuer must move both pieces into its own
    /// source-backed catalog. There is intentionally no `into_boxes` API:
    /// dropping the brand would make a partial re-issuance look authoritative.
    pub(crate) fn into_parts(
        self,
    ) -> (
        ResolverSourceInvocationProvenanceV1,
        Box<[ResolverBoxSourceRowV1]>,
    ) {
        (ResolverSourceInvocationProvenanceV1(self.brand), self.boxes)
    }

    pub(crate) fn same_source_invocation(&self, other: &Self) -> bool {
        self.brand == other.brand
    }

    pub(crate) fn parser_provenance(&self) -> ResolverSourceInvocationProvenanceV1 {
        ResolverSourceInvocationProvenanceV1(self.brand.clone())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ResolverSourceHandoffErrorV1 {
    ProgramNotAvailable,
    BoxSiteMissing { statement_ordinal: u32 },
    BoxNameMissing { statement_ordinal: u32 },
    NonDirectMethodSite { name: Box<str> },
    GeneratedOnlyBox { statement_ordinal: u32 },
    DuplicateMethodName { name: Box<str> },
    MethodRelationMissing { name: Box<str> },
    MethodRelationMismatch { name: Box<str> },
    MethodDeclarationUnsupported { name: Box<str> },
}

impl NyashParser {
    /// Parse and consume the rich parser source seals into the single
    /// AST-free resolver ingress. The AST remains available to the caller;
    /// the source seals do not.
    pub(crate) fn parse_from_string_with_resolver_source_handoff(
        input: impl Into<String>,
        build_config: ParserBuildConfig,
    ) -> Result<(ASTNode, ParserBoxResolverSourceHandoffV1), ParseError> {
        let parsed = Self::parse_from_string_with_source_seal(input, build_config)?;
        parsed
            .into_ast_and_resolver_source_handoff()
            .map_err(|error| ParseError::GrammarContract {
                stable_reject_tag: "resolver/source-handoff",
                detail: format!("resolver source handoff failed: {error:?}"),
                line: 0,
            })
    }
}

impl ParsedProgramWithSourceV1 {
    pub(crate) fn into_ast_and_resolver_source_handoff(
        self,
    ) -> Result<(ASTNode, ParserBoxResolverSourceHandoffV1), ResolverSourceHandoffErrorV1> {
        let (ast, seals, _, _) = self.into_postpass_parts();
        let handoff = build_resolver_source_handoff(&ast, &seals)?;
        Ok((ast, handoff))
    }
}

pub(super) fn build_resolver_source_handoff(
    ast: &ASTNode,
    seals: &[ParserBoxSourceSealV1],
) -> Result<ParserBoxResolverSourceHandoffV1, ResolverSourceHandoffErrorV1> {
    let statements = match ast {
        ASTNode::Program { statements, .. } => statements,
        _ => return Err(ResolverSourceHandoffErrorV1::ProgramNotAvailable),
    };

    let mut brand: Option<ParserInvocationBrandV1> = None;
    let mut boxes = Vec::with_capacity(seals.len());
    for seal in seals {
        let statement_ordinal = seal.box_site().statement_ordinal();
        let box_name = match statements.get(statement_ordinal as usize) {
            Some(ASTNode::BoxDeclaration { name, .. }) => name.clone().into_boxed_str(),
            Some(_) | None => {
                return Err(ResolverSourceHandoffErrorV1::BoxSiteMissing { statement_ordinal })
            }
        };

        let seal_brand = seal.box_site().path().brand().clone();
        if let Some(existing) = &brand {
            if *existing != seal_brand {
                return Err(ResolverSourceHandoffErrorV1::MethodRelationMismatch {
                    name: box_name,
                });
            }
        } else {
            brand = Some(seal_brand);
        }

        let methods = collect_explicit_methods(seal, statement_ordinal)?;
        boxes.push(ResolverBoxSourceRowV1 {
            statement_ordinal,
            name: box_name,
            methods: methods.into_boxed_slice(),
        });
    }

    let Some(brand) = brand else {
        return Err(ResolverSourceHandoffErrorV1::ProgramNotAvailable);
    };
    Ok(ParserBoxResolverSourceHandoffV1 {
        brand,
        boxes: boxes.into_boxed_slice(),
    })
}

fn collect_explicit_methods(
    seal: &ParserBoxSourceSealV1,
    statement_ordinal: u32,
) -> Result<Vec<ResolverBoxMethodSourceRowV1>, ResolverSourceHandoffErrorV1> {
    let mut names = HashSet::new();
    let mut methods = Vec::new();
    for relation in seal.method_relations() {
        let Some(source_site) = relation.source_site() else {
            continue;
        };
        if !source_site.is_direct() {
            return Err(ResolverSourceHandoffErrorV1::NonDirectMethodSite {
                name: relation.name().to_owned().into_boxed_str(),
            });
        }
        let name = relation.name().to_owned().into_boxed_str();
        if !names.insert(name.clone()) {
            return Err(ResolverSourceHandoffErrorV1::DuplicateMethodName { name });
        }
        let Some(entry) = seal
            .inventory()
            .iter_selected_declaration_order()
            .find(|entry| entry.site() == relation.inventory_ordinal())
        else {
            return Err(ResolverSourceHandoffErrorV1::MethodRelationMissing { name });
        };
        if entry.name() != name.as_ref() {
            return Err(ResolverSourceHandoffErrorV1::MethodRelationMismatch { name });
        }
        let signature = signature_from_declaration(entry.declaration(), &name)?;
        methods.push(ResolverBoxMethodSourceRowV1 {
            source_site: ResolverBoxMethodSourceSiteV1 {
                box_statement_ordinal: statement_ordinal,
                member_ordinal: source_site.source_member_ordinal(),
            },
            inventory_ordinal: relation.inventory_ordinal(),
            name,
            signature,
            callable_contract: relation.callable_contract().cloned(),
        });
    }
    if methods.is_empty() {
        return Err(ResolverSourceHandoffErrorV1::GeneratedOnlyBox { statement_ordinal });
    }
    Ok(methods)
}

fn signature_from_declaration(
    declaration: &ASTNode,
    name: &str,
) -> Result<ResolverMethodSignatureSyntaxV1, ResolverSourceHandoffErrorV1> {
    let ASTNode::FunctionDeclaration {
        params,
        param_decls,
        return_type_name,
        is_static,
        ..
    } = declaration
    else {
        return Err(ResolverSourceHandoffErrorV1::MethodDeclarationUnsupported {
            name: name.to_owned().into_boxed_str(),
        });
    };
    let params = project_neutral_parameter_syntax_v1(param_decls, params);
    Ok(ResolverMethodSignatureSyntaxV1 {
        parameters: params,
        return_type_name: return_type_name.clone().map(String::into_boxed_str),
        is_static: *is_static,
    })
}

#[cfg(test)]
#[path = "source_resolver_handoff_tests.rs"]
mod tests;
