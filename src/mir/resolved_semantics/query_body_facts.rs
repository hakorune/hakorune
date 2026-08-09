//! Private bounded Query body-facts projection.
//!
//! This module consumes the already-selected owner link and reads only the
//! carrier's neutral body-shape inventory. It never reselects Query, walks AST
//! syntax, or issues a second semantic axis.

use super::body_shape::{BodyExpressionShapeV1, BodyShapeRelationV1, BodyStatementShapeV1};
use super::ids::BindingRefV1;
use super::instance_method_body_owner::{
    VerifiedInstanceMethodBodyOwnerCatalogV1, VerifiedInstanceMethodBodyOwnerRowV1,
};
use super::records::BindingKindV1;
use super::source_site::{SourceExprSiteV1, SourceStmtSiteV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum QueryBodyFactsDeclineV1 {
    EmptyBody,
    MissingReturnValue,
    ReturnCount,
    ExtraStatement,
    ExtraExpression,
    ExtraRelation,
    NotReceiverMe,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum QueryBodyFactsUnresolvedV1 {
    OpaqueShape,
    IncompleteReturnRelation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum QueryBodyFactsRejectV1 {
    OwnerMismatch,
    BodyRootMismatch,
    ForeignReceiverBinding,
    WrongReceiverBindingKind,
    MissingReturnValueRelation,
    DuplicateReturnValueRelation,
    MixedOwnerCatalog,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum QueryBodyFactsIssueV1 {
    Declined(QueryBodyFactsDeclineV1),
    Unresolved(QueryBodyFactsUnresolvedV1),
    Rejected(QueryBodyFactsRejectV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) struct ReceiverReadFactV1 {
    expression: SourceExprSiteV1,
    receiver: BindingRefV1,
}

impl ReceiverReadFactV1 {
    pub(crate) const fn expression(&self) -> &SourceExprSiteV1 {
        &self.expression
    }

    pub(crate) const fn receiver(&self) -> BindingRefV1 {
        self.receiver
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) struct OrdinaryReturnFactV1 {
    statement: SourceStmtSiteV1,
    value: SourceExprSiteV1,
}

impl OrdinaryReturnFactV1 {
    pub(crate) const fn statement(&self) -> &SourceStmtSiteV1 {
        &self.statement
    }

    pub(crate) const fn value(&self) -> &SourceExprSiteV1 {
        &self.value
    }
}

#[derive(Debug)]
pub(in crate::mir) struct VerifiedCallableQueryBodyFactsRowV1<'row, 'body, 'contract, 'carrier> {
    owner: &'row VerifiedInstanceMethodBodyOwnerRowV1<'body, 'contract, 'carrier>,
    receiver_read: ReceiverReadFactV1,
    ordinary_return: OrdinaryReturnFactV1,
}

impl<'row, 'body, 'contract, 'carrier>
    VerifiedCallableQueryBodyFactsRowV1<'row, 'body, 'contract, 'carrier>
{
    pub(crate) fn owner(
        &self,
    ) -> &'row VerifiedInstanceMethodBodyOwnerRowV1<'body, 'contract, 'carrier> {
        self.owner
    }

    pub(crate) fn receiver_read(&self) -> &ReceiverReadFactV1 {
        &self.receiver_read
    }

    pub(crate) fn ordinary_return(&self) -> &OrdinaryReturnFactV1 {
        &self.ordinary_return
    }
}

#[derive(Debug)]
pub(in crate::mir) struct VerifiedCallableQueryBodyFactsCatalogV1<'row, 'body, 'contract, 'carrier>
{
    rows: Box<[VerifiedCallableQueryBodyFactsRowV1<'row, 'body, 'contract, 'carrier>]>,
}

impl<'row, 'body, 'contract, 'carrier>
    VerifiedCallableQueryBodyFactsCatalogV1<'row, 'body, 'contract, 'carrier>
{
    pub(crate) fn rows(
        &self,
    ) -> &[VerifiedCallableQueryBodyFactsRowV1<'row, 'body, 'contract, 'carrier>] {
        &self.rows
    }
}

pub(in crate::mir) struct QueryBodyFactsIssuerV1;

impl QueryBodyFactsIssuerV1 {
    pub(crate) fn issue<'row, 'body, 'contract, 'carrier>(
        owner: &'row VerifiedInstanceMethodBodyOwnerCatalogV1<'body, 'contract, 'carrier>,
    ) -> Result<
        VerifiedCallableQueryBodyFactsCatalogV1<'row, 'body, 'contract, 'carrier>,
        QueryBodyFactsIssueV1,
    > {
        let mut rows = Vec::with_capacity(owner.rows().len());
        for owner_row in owner.rows() {
            rows.push(Self::observe_row(owner_row)?);
        }
        Ok(VerifiedCallableQueryBodyFactsCatalogV1 {
            rows: rows.into_boxed_slice(),
        })
    }

    fn observe_row<'row, 'body, 'contract, 'carrier>(
        owner: &'row VerifiedInstanceMethodBodyOwnerRowV1<'body, 'contract, 'carrier>,
    ) -> Result<
        VerifiedCallableQueryBodyFactsRowV1<'row, 'body, 'contract, 'carrier>,
        QueryBodyFactsIssueV1,
    > {
        let root = owner.root_function();
        let carrier = owner.carrier();
        let shape = carrier.body_shape();
        if shape.owner() != root.owner() {
            return Err(QueryBodyFactsIssueV1::Rejected(
                QueryBodyFactsRejectV1::OwnerMismatch,
            ));
        }
        if shape.body_root() != carrier.body_root() {
            return Err(QueryBodyFactsIssueV1::Rejected(
                QueryBodyFactsRejectV1::BodyRootMismatch,
            ));
        }

        let (return_site, value_site) = match shape.statements() {
            [BodyStatementShapeV1::Return {
                site: return_site,
                value: Some(value_site),
            }] => (return_site, value_site),
            [] => {
                return Err(QueryBodyFactsIssueV1::Declined(
                    QueryBodyFactsDeclineV1::EmptyBody,
                ));
            }
            _ => {
                return Err(QueryBodyFactsIssueV1::Declined(
                    QueryBodyFactsDeclineV1::ReturnCount,
                ));
            }
        };

        let (me_site, receiver) = match shape.expressions() {
            [BodyExpressionShapeV1::Me { site, receiver }] => (site, receiver),
            [] => {
                return Err(QueryBodyFactsIssueV1::Declined(
                    QueryBodyFactsDeclineV1::MissingReturnValue,
                ));
            }
            _ => {
                return Err(QueryBodyFactsIssueV1::Declined(
                    QueryBodyFactsDeclineV1::NotReceiverMe,
                ));
            }
        };

        if receiver.owner() != root.owner() {
            return Err(QueryBodyFactsIssueV1::Rejected(
                QueryBodyFactsRejectV1::ForeignReceiverBinding,
            ));
        }
        let Some(binding) = root.binding(*receiver) else {
            return Err(QueryBodyFactsIssueV1::Rejected(
                QueryBodyFactsRejectV1::ForeignReceiverBinding,
            ));
        };
        if binding.kind() != BindingKindV1::Receiver {
            return Err(QueryBodyFactsIssueV1::Rejected(
                QueryBodyFactsRejectV1::WrongReceiverBindingKind,
            ));
        }
        if value_site != me_site {
            return Err(QueryBodyFactsIssueV1::Rejected(
                QueryBodyFactsRejectV1::MissingReturnValueRelation,
            ));
        }

        let matching_relations = shape
            .relations()
            .iter()
            .filter(|relation| return_value_relation(relation, return_site, value_site))
            .count();
        if matching_relations == 0 {
            return Err(QueryBodyFactsIssueV1::Rejected(
                QueryBodyFactsRejectV1::MissingReturnValueRelation,
            ));
        }
        if matching_relations > 1 {
            return Err(QueryBodyFactsIssueV1::Rejected(
                QueryBodyFactsRejectV1::DuplicateReturnValueRelation,
            ));
        }
        if shape.relations().len() != 1 {
            return Err(QueryBodyFactsIssueV1::Declined(
                QueryBodyFactsDeclineV1::ExtraRelation,
            ));
        }
        if !shape.effects().is_empty() {
            return Err(QueryBodyFactsIssueV1::Declined(
                QueryBodyFactsDeclineV1::ExtraExpression,
            ));
        }

        Ok(VerifiedCallableQueryBodyFactsRowV1 {
            owner,
            receiver_read: ReceiverReadFactV1 {
                expression: me_site.clone(),
                receiver: *receiver,
            },
            ordinary_return: OrdinaryReturnFactV1 {
                statement: return_site.clone(),
                value: value_site.clone(),
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
        && relation.role == super::source_site::SourcePathSegmentV1::Value
        && relation.child == *value_site
}

#[cfg(test)]
#[path = "query_body_facts_tests.rs"]
mod tests;
