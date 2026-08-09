//! Bounded Query body-conformance catalog.
//!
//! This module consumes only already-sealed declaration meaning and the
//! bounded `return me` evidence catalog. It compares those authorities; it
//! never reissues Query/Home/signature/ABI meaning or observes syntax/MIR.

use std::collections::BTreeSet;

use super::{
    DeclaredInstanceMethodContractRefV1, DeclaredInstanceMethodIdentityV1, DeclaredQueryBehaviorV1,
    ResolverCatalogBrandV1, VerifiedDeclaredInstanceMethodContractCatalogV1,
    VerifiedQueryBodyConformanceEvidenceCatalogV1, VerifiedQueryBodyConformanceEvidenceV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum QueryBodyConformanceIssueV1 {
    EvidenceCardinalityMismatch,
    ParserProvenanceMismatch,
    ResolverBrandMismatch,
    MissingEvidence(DeclaredInstanceMethodIdentityV1),
    DuplicateEvidence(DeclaredInstanceMethodIdentityV1),
    ExtraEvidence(DeclaredInstanceMethodIdentityV1),
    ContractBehaviorMismatch(DeclaredInstanceMethodIdentityV1),
}

/// One bounded conformance relation between an already-declared contract and
/// its exact evidence row. Neither side is reinterpreted here.
#[derive(Debug)]
pub(in crate::mir) struct VerifiedCallableBodyConformanceV1<
    'decl,
    'evidence,
    'owner,
    'body,
    'source_contract,
    'carrier,
> {
    contract: DeclaredInstanceMethodContractRefV1<'decl>,
    evidence: &'evidence VerifiedQueryBodyConformanceEvidenceV1<
        'owner,
        'body,
        'source_contract,
        'carrier,
    >,
}

impl<'decl, 'evidence, 'owner, 'body, 'source_contract, 'carrier>
    VerifiedCallableBodyConformanceV1<'decl, 'evidence, 'owner, 'body, 'source_contract, 'carrier>
{
    pub(crate) fn contract(&self) -> DeclaredInstanceMethodContractRefV1<'decl> {
        self.contract
    }

    pub(crate) fn evidence(
        &self,
    ) -> &'evidence VerifiedQueryBodyConformanceEvidenceV1<'owner, 'body, 'source_contract, 'carrier>
    {
        self.evidence
    }
}

/// Full-coverage catalog for the bounded selected Query cohort.
#[derive(Debug)]
pub(in crate::mir) struct VerifiedCallableBodyConformanceCatalogV1<
    'decl,
    'evidence,
    'owner,
    'body,
    'source_contract,
    'carrier,
> {
    resolver_brand: ResolverCatalogBrandV1,
    rows: Box<
        [VerifiedCallableBodyConformanceV1<
            'decl,
            'evidence,
            'owner,
            'body,
            'source_contract,
            'carrier,
        >],
    >,
}

impl<'decl, 'evidence, 'owner, 'body, 'source_contract, 'carrier>
    VerifiedCallableBodyConformanceCatalogV1<
        'decl,
        'evidence,
        'owner,
        'body,
        'source_contract,
        'carrier,
    >
{
    pub(crate) const fn resolver_brand(&self) -> ResolverCatalogBrandV1 {
        self.resolver_brand
    }

    pub(crate) fn rows(
        &self,
    ) -> &[VerifiedCallableBodyConformanceV1<
        'decl,
        'evidence,
        'owner,
        'body,
        'source_contract,
        'carrier,
    >] {
        &self.rows
    }
}

pub(in crate::mir) struct QueryBodyConformanceIssuerV1;

impl QueryBodyConformanceIssuerV1 {
    pub(crate) fn issue<'decl, 'evidence, 'owner, 'body, 'source_contract, 'carrier>(
        declared: &'decl VerifiedDeclaredInstanceMethodContractCatalogV1,
        evidence: &'evidence VerifiedQueryBodyConformanceEvidenceCatalogV1<
            'owner,
            'body,
            'source_contract,
            'carrier,
        >,
    ) -> Result<
        VerifiedCallableBodyConformanceCatalogV1<
            'decl,
            'evidence,
            'owner,
            'body,
            'source_contract,
            'carrier,
        >,
        QueryBodyConformanceIssueV1,
    > {
        if declared.selected_pair_count() != evidence.rows().len() {
            return Err(QueryBodyConformanceIssueV1::EvidenceCardinalityMismatch);
        }
        if declared.resolver_brand() != evidence.resolver_brand() {
            return Err(QueryBodyConformanceIssueV1::ResolverBrandMismatch);
        }
        if !declared
            .parser_provenance()
            .same_as(evidence.parser_provenance())
        {
            return Err(QueryBodyConformanceIssueV1::ParserProvenanceMismatch);
        }

        let selected = declared.selected_contracts().collect::<Vec<_>>();
        let mut seen = BTreeSet::new();
        for evidence_row in evidence.rows() {
            let identity = evidence_row.owner().contract().identity();
            if !seen.insert(identity) {
                return Err(QueryBodyConformanceIssueV1::DuplicateEvidence(identity));
            }
            if !selected
                .iter()
                .any(|contract| contract.identity() == identity)
            {
                return Err(QueryBodyConformanceIssueV1::ExtraEvidence(identity));
            }
        }

        let mut rows = Vec::with_capacity(selected.len());
        for contract in selected {
            let identity = contract.identity();
            if contract.query().behavior() != DeclaredQueryBehaviorV1::ReceiverDirectReadNoEffects {
                return Err(QueryBodyConformanceIssueV1::ContractBehaviorMismatch(
                    identity,
                ));
            }
            let mut matches = evidence
                .rows()
                .iter()
                .filter(|evidence_row| evidence_row.owner().contract().identity() == identity);
            let Some(evidence_row) = matches.next() else {
                return Err(QueryBodyConformanceIssueV1::MissingEvidence(identity));
            };
            if matches.next().is_some() {
                return Err(QueryBodyConformanceIssueV1::DuplicateEvidence(identity));
            }
            rows.push(VerifiedCallableBodyConformanceV1 {
                contract,
                evidence: evidence_row,
            });
        }

        Ok(VerifiedCallableBodyConformanceCatalogV1 {
            resolver_brand: declared.resolver_brand(),
            rows: rows.into_boxed_slice(),
        })
    }
}
