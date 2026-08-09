//! AST-free one-to-one owner link for selected instance-method bodies.
//!
//! This module consumes only the selected Query body projection and the
//! resolver-issued function carrier. It issues no FunctionOwnerId and does not
//! re-resolve, reselect Query, or infer body behavior.

use std::collections::BTreeSet;

use crate::parser::ResolverSourceInvocationProvenanceV1;

use super::{
    DeclaredInstanceMethodContractRefV1, ResolverCatalogBrandV1, ResolverNominalBoxTypeIdV1,
    VerifiedDeclaredQueryBodySourceCatalogV1, VerifiedInstanceMethodBodySourceRowV1,
    VerifiedInstanceMethodFunctionCarrierCatalogV1, VerifiedInstanceMethodFunctionCarrierRowV1,
    VerifiedResolvedFunctionV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum InstanceMethodBodyOwnerBindingIssueV1 {
    ParserProvenanceMismatch,
    ResolverBrandMismatch,
    DuplicateSelectedBody {
        box_statement_ordinal: u32,
        member_ordinal: u32,
    },
    MissingCarrier {
        nominal_box_type: ResolverNominalBoxTypeIdV1,
        box_statement_ordinal: u32,
        member_ordinal: u32,
    },
    DuplicateCarrier {
        nominal_box_type: ResolverNominalBoxTypeIdV1,
        box_statement_ordinal: u32,
        member_ordinal: u32,
    },
    NameMismatch {
        box_statement_ordinal: u32,
        member_ordinal: u32,
    },
    BodyCoverageMismatch {
        box_statement_ordinal: u32,
        member_ordinal: u32,
        body: Box<[u32]>,
        carrier: Box<[u32]>,
    },
}

/// One selected body relation. The root function/owner is borrowed from the
/// carrier row; this row never stores or issues a second owner token.
#[derive(Debug)]
pub(in crate::mir) struct VerifiedInstanceMethodBodyOwnerRowV1<'body, 'contract, 'carrier> {
    body: &'body VerifiedInstanceMethodBodySourceRowV1,
    contract: DeclaredInstanceMethodContractRefV1<'contract>,
    carrier: &'carrier VerifiedInstanceMethodFunctionCarrierRowV1,
}

impl<'body, 'contract, 'carrier> VerifiedInstanceMethodBodyOwnerRowV1<'body, 'contract, 'carrier> {
    pub(crate) fn body(&self) -> &'body VerifiedInstanceMethodBodySourceRowV1 {
        self.body
    }

    pub(crate) fn contract(&self) -> DeclaredInstanceMethodContractRefV1<'contract> {
        self.contract
    }

    pub(crate) fn carrier(&self) -> &'carrier VerifiedInstanceMethodFunctionCarrierRowV1 {
        self.carrier
    }

    pub(crate) fn root_function(&self) -> &'carrier VerifiedResolvedFunctionV1 {
        self.carrier.root_function()
    }
}

/// Catalog-level one-to-one owner relation for the selected Query subset.
#[derive(Debug)]
pub(in crate::mir) struct VerifiedInstanceMethodBodyOwnerCatalogV1<'body, 'contract, 'carrier> {
    resolver_brand: ResolverCatalogBrandV1,
    parser_provenance: &'body ResolverSourceInvocationProvenanceV1,
    rows: Box<[VerifiedInstanceMethodBodyOwnerRowV1<'body, 'contract, 'carrier>]>,
}

impl<'body, 'contract, 'carrier>
    VerifiedInstanceMethodBodyOwnerCatalogV1<'body, 'contract, 'carrier>
{
    pub(crate) const fn resolver_brand(&self) -> ResolverCatalogBrandV1 {
        self.resolver_brand
    }

    pub(crate) fn parser_provenance(&self) -> &'body ResolverSourceInvocationProvenanceV1 {
        self.parser_provenance
    }

    pub(crate) fn rows(
        &self,
    ) -> &[VerifiedInstanceMethodBodyOwnerRowV1<'body, 'contract, 'carrier>] {
        &self.rows
    }
}

pub(in crate::mir) struct InstanceMethodBodyOwnerBindingIssuerV1;

impl InstanceMethodBodyOwnerBindingIssuerV1 {
    pub(crate) fn issue<'body, 'contract, 'carrier>(
        body: &VerifiedDeclaredQueryBodySourceCatalogV1<'body, 'contract>,
        carrier: &'carrier VerifiedInstanceMethodFunctionCarrierCatalogV1,
    ) -> Result<
        VerifiedInstanceMethodBodyOwnerCatalogV1<'body, 'contract, 'carrier>,
        InstanceMethodBodyOwnerBindingIssueV1,
    > {
        if !body
            .parser_provenance()
            .same_as(carrier.parser_provenance())
        {
            return Err(InstanceMethodBodyOwnerBindingIssueV1::ParserProvenanceMismatch);
        }
        if body.resolver_brand() != carrier.resolver_brand() {
            return Err(InstanceMethodBodyOwnerBindingIssueV1::ResolverBrandMismatch);
        }

        let mut seen_body = BTreeSet::new();
        let mut rows = Vec::with_capacity(body.rows().len());
        for body_ref in body.rows() {
            let body_row = body_ref.body();
            let key = (
                body_row.nominal_box_type(),
                body_row.box_statement_ordinal(),
                body_row.method_member_ordinal(),
            );
            if !seen_body.insert(key) {
                return Err(
                    InstanceMethodBodyOwnerBindingIssueV1::DuplicateSelectedBody {
                        box_statement_ordinal: key.1,
                        member_ordinal: key.2,
                    },
                );
            }

            let mut matches = carrier.rows().iter().filter(|candidate| {
                candidate.nominal_box_type() == key.0
                    && candidate.source_site().box_statement_ordinal() == key.1
                    && candidate.source_site().member_ordinal() == key.2
            });
            let Some(carrier_row) = matches.next() else {
                return Err(InstanceMethodBodyOwnerBindingIssueV1::MissingCarrier {
                    nominal_box_type: key.0,
                    box_statement_ordinal: key.1,
                    member_ordinal: key.2,
                });
            };
            if matches.next().is_some() {
                return Err(InstanceMethodBodyOwnerBindingIssueV1::DuplicateCarrier {
                    nominal_box_type: key.0,
                    box_statement_ordinal: key.1,
                    member_ordinal: key.2,
                });
            }
            if body_row.name() != carrier_row.name() {
                return Err(InstanceMethodBodyOwnerBindingIssueV1::NameMismatch {
                    box_statement_ordinal: key.1,
                    member_ordinal: key.2,
                });
            }

            let body_coverage = body_row.body_item_ordinals();
            let carrier_coverage = carrier_row.body_coverage().item_ordinals();
            if body_coverage != carrier_coverage {
                return Err(
                    InstanceMethodBodyOwnerBindingIssueV1::BodyCoverageMismatch {
                        box_statement_ordinal: key.1,
                        member_ordinal: key.2,
                        body: body_coverage.to_vec().into_boxed_slice(),
                        carrier: carrier_coverage.to_vec().into_boxed_slice(),
                    },
                );
            }

            rows.push(VerifiedInstanceMethodBodyOwnerRowV1 {
                body: body_row,
                contract: body_ref.contract(),
                carrier: carrier_row,
            });
        }

        Ok(VerifiedInstanceMethodBodyOwnerCatalogV1 {
            resolver_brand: body.resolver_brand(),
            parser_provenance: body.parser_provenance(),
            rows: rows.into_boxed_slice(),
        })
    }
}
