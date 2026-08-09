//! Resolver-owned source carrier for one direct instance-method function.
//!
//! The parser-private syntax lease is the only place where AST syntax is
//! borrowed. This issuer runs the existing owner-forest resolver and retains
//! only AST-free source identity, the owner-bearing forest, and exact root
//! coverage. It does not bind a body source row, issue body facts, or create a
//! second FunctionOwner issuer.

use std::collections::BTreeSet;

use crate::parser::{
    ParserBoxInstanceMethodSyntaxLeaseV1, ResolverBoxMethodSourceSiteV1,
    ResolverSourceInvocationProvenanceV1,
};

use super::source_site::SourcePathSegmentV1;
use super::{
    FunctionOriginV1, FunctionOwnerIdV1, FunctionSemanticResolverSessionV1, ReceiverPolicyV1,
    ResolveOwnerForestErrorV1, ResolvedScopeRegionPairV1, ResolverCatalogBrandV1,
    ResolverNominalBoxTypeIdV1, SemanticOwnerRootProfileV1, SemanticOwnerSourceKindV1,
    VerifiedInstanceMethodDeclarationCatalogV1, VerifiedResolvedBodyShapeInventoryV1,
    VerifiedResolvedFunctionV1, VerifiedSemanticOwnerForestV1,
};

#[derive(Debug)]
pub(in crate::mir) enum InstanceMethodFunctionCarrierIssueV1 {
    EmptySyntaxLease,
    ParserProvenanceMismatch,
    DeclarationCardinalityMismatch {
        declarations: usize,
        syntax_rows: usize,
    },
    DuplicateSourceSite(ResolverBoxMethodSourceSiteV1),
    DeclarationMissing(ResolverBoxMethodSourceSiteV1),
    DeclarationNameMismatch {
        expected: Box<str>,
        actual: Box<str>,
    },
    OwnerForest(ResolveOwnerForestErrorV1),
    RootCardinality {
        actual: usize,
    },
    RootFunctionMissing(FunctionOwnerIdV1),
    RootProfileMismatch {
        actual_kind: SemanticOwnerSourceKindV1,
        actual_policy: ReceiverPolicyV1,
    },
    BodyShapeMissing(FunctionOwnerIdV1),
    BodyShapeMismatch(FunctionOwnerIdV1),
    BodyItemOrdinalOverflow {
        count: usize,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) struct VerifiedMethodBodyCoverageV1 {
    item_ordinals: Box<[u32]>,
}

impl VerifiedMethodBodyCoverageV1 {
    fn issue(count: usize) -> Result<Self, InstanceMethodFunctionCarrierIssueV1> {
        let item_ordinals = (0..count)
            .map(u32::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| InstanceMethodFunctionCarrierIssueV1::BodyItemOrdinalOverflow { count })?;
        Ok(Self {
            item_ordinals: item_ordinals.into_boxed_slice(),
        })
    }

    pub(in crate::mir) fn item_ordinals(&self) -> &[u32] {
        &self.item_ordinals
    }
}

#[derive(Debug)]
pub(in crate::mir) struct VerifiedInstanceMethodFunctionCarrierRowV1 {
    source_site: ResolverBoxMethodSourceSiteV1,
    name: Box<str>,
    nominal_box_type: ResolverNominalBoxTypeIdV1,
    function_origin: FunctionOriginV1,
    forest: VerifiedSemanticOwnerForestV1,
    body_root: SourcePathSegmentV1,
    body_pair: ResolvedScopeRegionPairV1,
    body_coverage: VerifiedMethodBodyCoverageV1,
    body_shape: VerifiedResolvedBodyShapeInventoryV1,
}

impl VerifiedInstanceMethodFunctionCarrierRowV1 {
    pub(crate) const fn source_site(&self) -> ResolverBoxMethodSourceSiteV1 {
        self.source_site
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) const fn nominal_box_type(&self) -> ResolverNominalBoxTypeIdV1 {
        self.nominal_box_type
    }

    pub(crate) const fn function_origin(&self) -> FunctionOriginV1 {
        self.function_origin
    }

    pub(crate) fn root_function(&self) -> &VerifiedResolvedFunctionV1 {
        let [root] = self.forest.roots() else {
            unreachable!("verified carrier forest always has one root")
        };
        self.forest
            .owner(*root)
            .expect("verified carrier forest root must be present")
    }

    pub(crate) const fn body_root(&self) -> &SourcePathSegmentV1 {
        &self.body_root
    }

    pub(crate) const fn body_pair(&self) -> ResolvedScopeRegionPairV1 {
        self.body_pair
    }

    pub(crate) fn body_coverage(&self) -> &VerifiedMethodBodyCoverageV1 {
        &self.body_coverage
    }

    pub(crate) fn body_shape(&self) -> &VerifiedResolvedBodyShapeInventoryV1 {
        &self.body_shape
    }

    pub(crate) fn forest(&self) -> &VerifiedSemanticOwnerForestV1 {
        &self.forest
    }
}

#[derive(Debug)]
pub(in crate::mir) struct VerifiedInstanceMethodFunctionCarrierCatalogV1 {
    resolver_brand: ResolverCatalogBrandV1,
    parser_provenance: ResolverSourceInvocationProvenanceV1,
    rows: Box<[VerifiedInstanceMethodFunctionCarrierRowV1]>,
}

impl VerifiedInstanceMethodFunctionCarrierCatalogV1 {
    pub(crate) const fn resolver_brand(&self) -> ResolverCatalogBrandV1 {
        self.resolver_brand
    }

    pub(crate) fn parser_provenance(&self) -> &ResolverSourceInvocationProvenanceV1 {
        &self.parser_provenance
    }

    pub(crate) fn rows(&self) -> &[VerifiedInstanceMethodFunctionCarrierRowV1] {
        &self.rows
    }
}

pub(in crate::mir) struct InstanceMethodFunctionCarrierIssuerV1;

impl InstanceMethodFunctionCarrierIssuerV1 {
    pub(crate) fn issue<'ast>(
        lease: ParserBoxInstanceMethodSyntaxLeaseV1<'ast>,
        declarations: &VerifiedInstanceMethodDeclarationCatalogV1,
        resolver: &mut FunctionSemanticResolverSessionV1,
    ) -> Result<VerifiedInstanceMethodFunctionCarrierCatalogV1, InstanceMethodFunctionCarrierIssueV1>
    {
        let parser_provenance = lease.parser_provenance().clone();
        if !parser_provenance.same_as(declarations.parser_provenance()) {
            return Err(InstanceMethodFunctionCarrierIssueV1::ParserProvenanceMismatch);
        }
        let syntax_rows = lease.rows();
        if syntax_rows.is_empty() {
            return Err(InstanceMethodFunctionCarrierIssueV1::EmptySyntaxLease);
        }
        if syntax_rows.len() != declarations.declarations().len() {
            return Err(
                InstanceMethodFunctionCarrierIssueV1::DeclarationCardinalityMismatch {
                    declarations: declarations.declarations().len(),
                    syntax_rows: syntax_rows.len(),
                },
            );
        }

        let mut seen_sites = BTreeSet::new();
        let mut rows = Vec::with_capacity(syntax_rows.len());
        for syntax in syntax_rows.iter().copied() {
            let source_site = syntax.source_site();
            if !seen_sites.insert((
                source_site.box_statement_ordinal(),
                source_site.member_ordinal(),
            )) {
                return Err(InstanceMethodFunctionCarrierIssueV1::DuplicateSourceSite(
                    source_site,
                ));
            }
            let declaration = declarations.declaration_at_source_site(source_site).ok_or(
                InstanceMethodFunctionCarrierIssueV1::DeclarationMissing(source_site),
            )?;
            if declaration.name() != syntax.name() {
                return Err(
                    InstanceMethodFunctionCarrierIssueV1::DeclarationNameMismatch {
                        expected: declaration.name().to_owned().into_boxed_str(),
                        actual: syntax.name().to_owned().into_boxed_str(),
                    },
                );
            }

            let function_view = super::FunctionSyntaxViewV1::from_borrowed_function_parts(
                syntax.params(),
                syntax.body(),
                ReceiverPolicyV1::DeclaredInstance,
            );
            let (forest, mut body_shapes) = resolver
                .resolve_forest_with_body_shapes(function_view)
                .map_err(InstanceMethodFunctionCarrierIssueV1::OwnerForest)?;
            let [root_owner] = forest.roots() else {
                return Err(InstanceMethodFunctionCarrierIssueV1::RootCardinality {
                    actual: forest.roots().len(),
                });
            };
            let root = forest.owner(*root_owner).ok_or(
                InstanceMethodFunctionCarrierIssueV1::RootFunctionMissing(*root_owner),
            )?;
            let profile = root.root_profile();
            if profile.source_kind() != SemanticOwnerSourceKindV1::DeclaredFunction
                || profile.receiver_policy() != ReceiverPolicyV1::DeclaredInstance
            {
                return Err(InstanceMethodFunctionCarrierIssueV1::RootProfileMismatch {
                    actual_kind: profile.source_kind(),
                    actual_policy: profile.receiver_policy(),
                });
            }
            let body_shape = body_shapes.remove(root_owner).ok_or(
                InstanceMethodFunctionCarrierIssueV1::BodyShapeMissing(*root_owner),
            )?;
            if body_shape.owner() != *root_owner || *body_shape.body_root() != profile.body_root() {
                return Err(InstanceMethodFunctionCarrierIssueV1::BodyShapeMismatch(
                    *root_owner,
                ));
            }
            rows.push(VerifiedInstanceMethodFunctionCarrierRowV1 {
                source_site,
                name: syntax.name().to_owned().into_boxed_str(),
                nominal_box_type: declaration.nominal_box_type(),
                function_origin: root.function_origin(),
                body_root: profile.body_root(),
                body_pair: root.lowering_roots().body_pair(),
                body_coverage: VerifiedMethodBodyCoverageV1::issue(syntax.body().len())?,
                body_shape,
                forest,
            });
        }

        Ok(VerifiedInstanceMethodFunctionCarrierCatalogV1 {
            resolver_brand: declarations.resolver_brand(),
            parser_provenance,
            rows: rows.into_boxed_slice(),
        })
    }
}
