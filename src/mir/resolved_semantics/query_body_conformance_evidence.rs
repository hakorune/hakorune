//! Bounded resolver evidence for the first Query body-conformance cohort.
//!
//! This is deliberately narrower than general body conformance.  It proves
//! only the exact `return me` shape, including the declaration's Handle/
//! Trivial Home boundary and the absence of a Home transfer in that shape.
//! It does not extend the neutral body-effect vocabulary or inspect MIR.

use std::ptr;

use super::body_shape::{BodyExpressionShapeV1, BodyShapeRelationV1, BodyStatementShapeV1};
use super::home_relation::{HomeDemandV1, HomeResultRelationV1};
use super::instance_method_body_owner::{
    VerifiedInstanceMethodBodyOwnerCatalogV1, VerifiedInstanceMethodBodyOwnerRowV1,
};
use super::query_body_facts::{
    VerifiedCallableQueryBodyFactsCatalogV1, VerifiedCallableQueryBodyFactsRowV1,
};
use super::source_site::{SourceExprSiteV1, SourcePathSegmentV1, SourceStmtSiteV1};
use super::ResolverCatalogBrandV1;
use crate::parser::ResolverSourceInvocationProvenanceV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum QueryBodyHomeTransferV1 {
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct VerifiedQueryBodyStructuralSafetyV1 {
    statements: u8,
    expressions: u8,
    relations: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) struct VerifiedQueryBodyHomeFlowEvidenceV1 {
    transfer: QueryBodyHomeTransferV1,
}

impl VerifiedQueryBodyHomeFlowEvidenceV1 {
    pub(crate) const fn transfer(&self) -> QueryBodyHomeTransferV1 {
        self.transfer
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum QueryBodyConformanceEvidenceNoSafeSliceV1 {
    BodyCoverageNotBounded,
    UnsupportedNestedOwner,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum QueryBodyConformanceEvidenceDeclineV1 {
    ObservedEffect,
    ShapeOutsideCohort,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum QueryBodyConformanceEvidenceRejectV1 {
    FactsCardinalityMismatch,
    FactsOwnerMismatch,
    OwnerMismatch,
    BodyRootMismatch,
    BodyCoverageMismatch,
    HomeReceiverMismatch,
    HomeParameterMismatch,
    HomeResultMismatch,
    ReturnRelationMismatch,
    FactsShapeMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum QueryBodyConformanceEvidenceIssueV1 {
    NoSafeSlice(QueryBodyConformanceEvidenceNoSafeSliceV1),
    Declined(QueryBodyConformanceEvidenceDeclineV1),
    Rejected(QueryBodyConformanceEvidenceRejectV1),
}

/// One complete, bounded body evidence row.  The owner row is borrowed from
/// the existing owner catalog; this product issues no new owner or contract.
#[derive(Debug)]
pub(in crate::mir) struct VerifiedQueryBodyConformanceEvidenceV1<'owner, 'body, 'contract, 'carrier>
{
    owner: &'owner VerifiedInstanceMethodBodyOwnerRowV1<'body, 'contract, 'carrier>,
    safety: VerifiedQueryBodyStructuralSafetyV1,
    home_flow: VerifiedQueryBodyHomeFlowEvidenceV1,
}

impl<'owner, 'body, 'contract, 'carrier>
    VerifiedQueryBodyConformanceEvidenceV1<'owner, 'body, 'contract, 'carrier>
{
    pub(crate) fn owner(
        &self,
    ) -> &'owner VerifiedInstanceMethodBodyOwnerRowV1<'body, 'contract, 'carrier> {
        self.owner
    }

    pub(crate) const fn home_flow(&self) -> VerifiedQueryBodyHomeFlowEvidenceV1 {
        self.home_flow
    }
}

/// Atomic catalog-level evidence for the selected Query body cohort.
#[derive(Debug)]
pub(in crate::mir) struct VerifiedQueryBodyConformanceEvidenceCatalogV1<
    'owner,
    'body,
    'contract,
    'carrier,
> {
    resolver_brand: ResolverCatalogBrandV1,
    parser_provenance: &'body ResolverSourceInvocationProvenanceV1,
    rows: Box<[VerifiedQueryBodyConformanceEvidenceV1<'owner, 'body, 'contract, 'carrier>]>,
}

impl<'owner, 'body, 'contract, 'carrier>
    VerifiedQueryBodyConformanceEvidenceCatalogV1<'owner, 'body, 'contract, 'carrier>
{
    pub(crate) const fn resolver_brand(&self) -> ResolverCatalogBrandV1 {
        self.resolver_brand
    }

    pub(crate) fn parser_provenance(&self) -> &'body ResolverSourceInvocationProvenanceV1 {
        self.parser_provenance
    }

    pub(crate) fn rows(
        &self,
    ) -> &[VerifiedQueryBodyConformanceEvidenceV1<'owner, 'body, 'contract, 'carrier>] {
        &self.rows
    }
}

pub(in crate::mir) struct QueryBodyConformanceEvidenceIssuerV1;

impl QueryBodyConformanceEvidenceIssuerV1 {
    pub(crate) fn issue<'owner, 'body, 'contract, 'carrier>(
        owner: &'owner VerifiedInstanceMethodBodyOwnerCatalogV1<'body, 'contract, 'carrier>,
        facts: &VerifiedCallableQueryBodyFactsCatalogV1<'owner, 'body, 'contract, 'carrier>,
    ) -> Result<
        VerifiedQueryBodyConformanceEvidenceCatalogV1<'owner, 'body, 'contract, 'carrier>,
        QueryBodyConformanceEvidenceIssueV1,
    > {
        if owner.rows().len() != facts.rows().len() {
            return Err(QueryBodyConformanceEvidenceIssueV1::Rejected(
                QueryBodyConformanceEvidenceRejectV1::FactsCardinalityMismatch,
            ));
        }

        let rows = owner
            .rows()
            .iter()
            .zip(facts.rows())
            .map(|(owner_row, facts_row)| Self::issue_row(owner_row, facts_row))
            .collect::<Result<Vec<_>, _>>()?
            .into_boxed_slice();
        Ok(VerifiedQueryBodyConformanceEvidenceCatalogV1 {
            resolver_brand: owner.resolver_brand(),
            parser_provenance: owner.parser_provenance(),
            rows,
        })
    }

    fn issue_row<'owner, 'body, 'contract, 'carrier>(
        owner: &'owner VerifiedInstanceMethodBodyOwnerRowV1<'body, 'contract, 'carrier>,
        facts: &VerifiedCallableQueryBodyFactsRowV1<'owner, 'body, 'contract, 'carrier>,
    ) -> Result<
        VerifiedQueryBodyConformanceEvidenceV1<'owner, 'body, 'contract, 'carrier>,
        QueryBodyConformanceEvidenceIssueV1,
    > {
        if !ptr::eq(facts.owner(), owner) {
            return Err(QueryBodyConformanceEvidenceIssueV1::Rejected(
                QueryBodyConformanceEvidenceRejectV1::FactsOwnerMismatch,
            ));
        }

        let contract = owner.contract();
        let home = contract.home_abi();
        if home.receiver() != HomeDemandV1::Handle {
            return Err(QueryBodyConformanceEvidenceIssueV1::Rejected(
                QueryBodyConformanceEvidenceRejectV1::HomeReceiverMismatch,
            ));
        }
        if !home.parameters().is_empty() {
            return Err(QueryBodyConformanceEvidenceIssueV1::Rejected(
                QueryBodyConformanceEvidenceRejectV1::HomeParameterMismatch,
            ));
        }
        if home.result() != HomeResultRelationV1::Trivial {
            return Err(QueryBodyConformanceEvidenceIssueV1::Rejected(
                QueryBodyConformanceEvidenceRejectV1::HomeResultMismatch,
            ));
        }

        let shape = owner.carrier().body_shape();
        if shape.owner() != owner.root_function().owner() {
            return Err(QueryBodyConformanceEvidenceIssueV1::Rejected(
                QueryBodyConformanceEvidenceRejectV1::OwnerMismatch,
            ));
        }
        if shape.body_root() != owner.carrier().body_root() {
            return Err(QueryBodyConformanceEvidenceIssueV1::Rejected(
                QueryBodyConformanceEvidenceRejectV1::BodyRootMismatch,
            ));
        }
        if owner.body().body_item_ordinals() != [0] {
            return Err(QueryBodyConformanceEvidenceIssueV1::NoSafeSlice(
                QueryBodyConformanceEvidenceNoSafeSliceV1::BodyCoverageNotBounded,
            ));
        }

        let (return_site, value_site) = match shape.statements() {
            [BodyStatementShapeV1::Return {
                site,
                value: Some(value),
            }] => (site, value),
            _ => {
                return Err(QueryBodyConformanceEvidenceIssueV1::Declined(
                    QueryBodyConformanceEvidenceDeclineV1::ShapeOutsideCohort,
                ));
            }
        };
        let (me_site, receiver) = match shape.expressions() {
            [BodyExpressionShapeV1::Me { site, receiver }] => (site, receiver),
            _ => {
                return Err(QueryBodyConformanceEvidenceIssueV1::Declined(
                    QueryBodyConformanceEvidenceDeclineV1::ShapeOutsideCohort,
                ));
            }
        };
        if shape.effects().len() != 0 {
            return Err(QueryBodyConformanceEvidenceIssueV1::Declined(
                QueryBodyConformanceEvidenceDeclineV1::ObservedEffect,
            ));
        }
        if shape.relations().len() != 1
            || !return_value_relation(&shape.relations()[0], return_site, value_site)
        {
            return Err(QueryBodyConformanceEvidenceIssueV1::Rejected(
                QueryBodyConformanceEvidenceRejectV1::ReturnRelationMismatch,
            ));
        }
        if facts.receiver_read().expression() != me_site
            || facts.receiver_read().receiver() != *receiver
            || facts.ordinary_return().statement() != return_site
            || facts.ordinary_return().value() != value_site
        {
            return Err(QueryBodyConformanceEvidenceIssueV1::Rejected(
                QueryBodyConformanceEvidenceRejectV1::FactsShapeMismatch,
            ));
        }

        Ok(VerifiedQueryBodyConformanceEvidenceV1 {
            owner,
            safety: VerifiedQueryBodyStructuralSafetyV1 {
                statements: 1,
                expressions: 1,
                relations: 1,
            },
            home_flow: VerifiedQueryBodyHomeFlowEvidenceV1 {
                transfer: QueryBodyHomeTransferV1::None,
            },
        })
    }
}

fn return_value_relation(
    relation: &BodyShapeRelationV1,
    return_site: &SourceStmtSiteV1,
    value_site: &SourceExprSiteV1,
) -> bool {
    relation.parent == *return_site.node()
        && relation.role == SourcePathSegmentV1::Value
        && relation.child == *value_site
}
