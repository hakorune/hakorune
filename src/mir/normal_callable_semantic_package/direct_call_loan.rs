//! Package-owned, move-only direct-call disposition for one exact owner.
//!
//! The loan carries only products already issued by the resolver session.  It
//! never resolves a source name and it never reconstructs a target from a raw
//! symbol.  A raw consumer may take one owned row for one owner/site and must
//! leave no row behind before the package closes.

use std::collections::BTreeMap;

use crate::mir::canonical_direct_call::VerifiedCanonicalDirectCallEmissionV1;
use crate::mir::instruction::InvokeCallResultKind;
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, OwnedExprSiteV1, SourceExprSiteV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DirectCallLoanErrorV1 {
    OwnerMismatch,
    SiteMissing,
    SiteAlreadyTaken,
    ResidualRows,
    DuplicateSite,
    DuplicateOwner,
    LifecycleSourceMismatch,
    LifecycleConsumerMissing,
}

#[derive(Debug)]
pub(crate) struct DirectCallDispositionRowV1 {
    argument_sites: Box<[SourceExprSiteV1]>,
    emission: VerifiedCanonicalDirectCallEmissionV1,
    execution: DirectCallExecutionV1,
    result: InvokeCallResultKind,
}

#[derive(Debug)]
enum DirectCallExecutionV1 {
    Scalar,
    Lifecycle,
}

#[path = "direct_call_lifecycle.rs"]
pub(in crate::mir::normal_callable_semantic_package) mod lifecycle;

impl DirectCallDispositionRowV1 {
    pub(crate) fn physical_emission(&self) -> &VerifiedCanonicalDirectCallEmissionV1 {
        &self.emission
    }

    pub(crate) fn lifecycle_emission(
        &self,
    ) -> Result<&VerifiedCanonicalDirectCallEmissionV1, DirectCallLoanErrorV1> {
        match self.execution {
            DirectCallExecutionV1::Lifecycle => Ok(&self.emission),
            DirectCallExecutionV1::Scalar => Err(DirectCallLoanErrorV1::LifecycleSourceMismatch),
        }
    }
    pub(crate) fn new(
        argument_sites: Box<[SourceExprSiteV1]>,
        emission: VerifiedCanonicalDirectCallEmissionV1,
    ) -> Self {
        Self {
            argument_sites,
            emission,
            execution: DirectCallExecutionV1::Scalar,
            result: InvokeCallResultKind::I64,
        }
    }

    /// The callee's source-issued result class, sealed at lifecycle co-seal
    /// from the callee's own terminal relation. `I64` remains the default
    /// for every scalar-admitted row; a Map-source callee yields `Map`.
    pub(crate) const fn result(&self) -> InvokeCallResultKind {
        self.result
    }

    pub(crate) fn argument_sites(&self) -> &[SourceExprSiteV1] {
        &self.argument_sites
    }

    pub(crate) fn into_scalar_emission(
        self,
    ) -> Result<VerifiedCanonicalDirectCallEmissionV1, DirectCallLoanErrorV1> {
        match self.execution {
            DirectCallExecutionV1::Scalar => Ok(self.emission),
            DirectCallExecutionV1::Lifecycle => {
                Err(DirectCallLoanErrorV1::LifecycleConsumerMissing)
            }
        }
    }
}

#[derive(Debug)]
enum DirectCallDispositionSlotV1 {
    Ready(DirectCallDispositionRowV1),
    Taken,
}

/// A private affine inventory for one exact callable owner.
#[must_use]
#[derive(Debug)]
pub(crate) struct DirectCallDispositionLoanV1 {
    owner: FunctionOwnerIdV1,
    rows: BTreeMap<OwnedExprSiteV1, DirectCallDispositionSlotV1>,
}

impl DirectCallDispositionLoanV1 {
    /// Take only the exact Call attached to this ledger's original Completion.
    pub(crate) fn take_terminal_lifecycle(
        &mut self,
        ledger: &super::OrdinaryNewClaimLedgerV1,
        owner: FunctionOwnerIdV1,
        return_site: &crate::mir::resolved_semantics::SourceNodeSiteV1,
    ) -> Result<Option<DirectCallDispositionRowV1>, DirectCallLoanErrorV1> {
        let Some(row) = self.take_terminal(ledger, owner, return_site)? else {
            return Ok(None);
        };
        row.lifecycle_emission()?;
        Ok(Some(row))
    }

    pub(crate) fn take_terminal(
        &mut self,
        ledger: &super::OrdinaryNewClaimLedgerV1,
        owner: FunctionOwnerIdV1,
        return_site: &crate::mir::resolved_semantics::SourceNodeSiteV1,
    ) -> Result<Option<DirectCallDispositionRowV1>, DirectCallLoanErrorV1> {
        let Some((completion, terminal)) = ledger.call_source_completion_for_owner(owner) else {
            return Ok(None);
        };
        if owner != self.owner
            || completion.owner() != owner
            || terminal.owner() != owner
            || terminal.return_site().node() != return_site
            || completion.explicit_site() != Some(terminal.return_site())
        {
            return Err(DirectCallLoanErrorV1::LifecycleSourceMismatch);
        }
        self.take_once(owner, terminal.call_site().clone())
            .map(Some)
    }
    pub(crate) fn from_rows(
        owner: FunctionOwnerIdV1,
        rows: impl IntoIterator<Item = (SourceExprSiteV1, DirectCallDispositionRowV1)>,
    ) -> Result<Self, DirectCallLoanErrorV1> {
        let mut slots = BTreeMap::new();
        for (site, row) in rows {
            if slots
                .insert(
                    OwnedExprSiteV1::new(owner, site),
                    DirectCallDispositionSlotV1::Ready(row),
                )
                .is_some()
            {
                return Err(DirectCallLoanErrorV1::DuplicateSite);
            }
        }
        if slots.is_empty() {
            return Err(DirectCallLoanErrorV1::ResidualRows);
        }
        Ok(Self { owner, rows: slots })
    }

    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) fn take_once(
        &mut self,
        owner: FunctionOwnerIdV1,
        site: SourceExprSiteV1,
    ) -> Result<DirectCallDispositionRowV1, DirectCallLoanErrorV1> {
        if owner != self.owner {
            return Err(DirectCallLoanErrorV1::OwnerMismatch);
        }
        let key = OwnedExprSiteV1::new(owner, site);
        let slot = self
            .rows
            .get_mut(&key)
            .ok_or(DirectCallLoanErrorV1::SiteMissing)?;
        match std::mem::replace(slot, DirectCallDispositionSlotV1::Taken) {
            DirectCallDispositionSlotV1::Ready(row) => Ok(row),
            DirectCallDispositionSlotV1::Taken => Err(DirectCallLoanErrorV1::SiteAlreadyTaken),
        }
    }

    pub(crate) fn finish_empty(self) -> Result<(), DirectCallLoanErrorV1> {
        if self
            .rows
            .values()
            .any(|slot| matches!(slot, DirectCallDispositionSlotV1::Ready(_)))
        {
            return Err(DirectCallLoanErrorV1::ResidualRows);
        }
        Ok(())
    }
}

/// The per-owner direct-call inventory carried by one installed package.
///
/// Every loan keeps its own exact owner; the collection only routes a raw
/// consumer to the loan that owner already holds and never resolves targets.
/// An owner whose call sites are lowered through a non-raw lane (for example
/// a canonical callable program) never enters direct-call scope, so a fully
/// untouched loan drains without residual; a partially consumed loan is
/// always a violation.
#[must_use]
#[derive(Debug)]
pub(crate) struct DirectCallDispositionLoansV1 {
    loans: BTreeMap<FunctionOwnerIdV1, DirectCallDispositionLoanV1>,
}

impl DirectCallDispositionLoansV1 {
    pub(crate) fn issue(
        loans: impl IntoIterator<Item = DirectCallDispositionLoanV1>,
    ) -> Result<Self, DirectCallLoanErrorV1> {
        let mut issued = BTreeMap::new();
        for loan in loans {
            if issued.insert(loan.owner, loan).is_some() {
                return Err(DirectCallLoanErrorV1::DuplicateOwner);
            }
        }
        Ok(Self { loans: issued })
    }

    pub(crate) fn get(&self, owner: FunctionOwnerIdV1) -> Option<&DirectCallDispositionLoanV1> {
        self.loans.get(&owner)
    }

    pub(crate) fn get_mut(
        &mut self,
        owner: FunctionOwnerIdV1,
    ) -> Option<&mut DirectCallDispositionLoanV1> {
        self.loans.get_mut(&owner)
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &DirectCallDispositionLoanV1> {
        self.loans.values()
    }

    pub(crate) fn iter_mut(&mut self) -> impl Iterator<Item = &mut DirectCallDispositionLoanV1> {
        self.loans.values_mut()
    }

    pub(crate) fn finish_empty(self) -> Result<(), DirectCallLoanErrorV1> {
        for loan in self.loans.into_values() {
            let mut taken = false;
            let mut ready = false;
            for slot in loan.rows.values() {
                match slot {
                    DirectCallDispositionSlotV1::Ready(_) => ready = true,
                    DirectCallDispositionSlotV1::Taken => taken = true,
                }
            }
            if taken && ready {
                return Err(DirectCallLoanErrorV1::ResidualRows);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, DeclarationAttrs, ParamDecl, Span};
    use crate::mir::canonical_direct_call::VerifiedCanonicalDirectCallEmissionV1;
    use crate::mir::compiler::VerifiedResolvedCallableProgramV1;
    use crate::mir::resolved_semantics::FunctionOwnerIssuerV1;

    fn fixture() -> (
        FunctionOwnerIdV1,
        SourceExprSiteV1,
        VerifiedCanonicalDirectCallEmissionV1,
    ) {
        let function = ASTNode::FunctionDeclaration {
            name: "helper".to_owned(),
            params: vec!["value".to_owned()],
            param_decls: vec![ParamDecl {
                name: "value".to_owned(),
                declared_type_name: Some("i64".to_owned()),
            }],
            return_type_name: Some("i64".to_owned()),
            body: Vec::new(),
            uses: Vec::new(),
            contracts: Vec::new(),
            is_static: true,
            is_override: false,
            attrs: DeclarationAttrs::default(),
            span: Span::unknown(),
        };
        let source = VerifiedResolvedCallableProgramV1::resolve(ASTNode::Program {
            statements: vec![function],
            span: Span::unknown(),
        })
        .expect("callable fixture");
        let header = source
            .module()
            .source()
            .catalog()
            .index()
            .resolve_free_static_source_call("helper", 1)
            .expect("exact helper header");
        let owner = header.callable().owner();
        let site = crate::mir::resolved_semantics::SourcePathV1::function_body().expr();
        let emission = VerifiedCanonicalDirectCallEmissionV1::conservative_from_header(header);
        (owner, site, emission)
    }

    fn foreign_owner() -> FunctionOwnerIdV1 {
        let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().expect("owner issuer");
        issuer.issue().expect("foreign owner")
    }

    #[test]
    fn take_once_rejects_second_take_and_finishes_empty() {
        let (owner, site, emission) = fixture();
        let mut loan = DirectCallDispositionLoanV1::from_rows(
            owner,
            [(
                site.clone(),
                DirectCallDispositionRowV1::new(Box::new([]), emission),
            )],
        )
        .expect("one-row loan");
        assert!(loan.take_once(owner, site.clone()).is_ok());
        assert_eq!(
            loan.take_once(owner, site).err(),
            Some(DirectCallLoanErrorV1::SiteAlreadyTaken)
        );
        loan.finish_empty().expect("no residual rows");
    }

    #[test]
    fn take_once_rejects_foreign_owner_without_consuming_row() {
        let (owner, site, emission) = fixture();
        let foreign = foreign_owner();
        let mut loan = DirectCallDispositionLoanV1::from_rows(
            owner,
            [(
                site.clone(),
                DirectCallDispositionRowV1::new(Box::new([]), emission),
            )],
        )
        .expect("one-row loan");
        assert_eq!(
            loan.take_once(foreign, site.clone()).err(),
            Some(DirectCallLoanErrorV1::OwnerMismatch)
        );
        assert!(loan.take_once(owner, site).is_ok());
    }

    #[test]
    fn finish_empty_rejects_residual_rows() {
        let (owner, site, emission) = fixture();
        let loan = DirectCallDispositionLoanV1::from_rows(
            owner,
            [(
                site,
                DirectCallDispositionRowV1::new(Box::new([]), emission),
            )],
        )
        .expect("one-row loan");
        assert_eq!(
            loan.finish_empty(),
            Err(DirectCallLoanErrorV1::ResidualRows)
        );
    }

    #[test]
    fn from_rows_rejects_duplicate_sites() {
        let (owner, site, emission) = fixture();
        let second = VerifiedCanonicalDirectCallEmissionV1::clone(&emission);
        assert_eq!(
            DirectCallDispositionLoanV1::from_rows(
                owner,
                [
                    (
                        site.clone(),
                        DirectCallDispositionRowV1::new(Box::new([]), emission),
                    ),
                    (site, DirectCallDispositionRowV1::new(Box::new([]), second),),
                ],
            )
            .err(),
            Some(DirectCallLoanErrorV1::DuplicateSite)
        );
    }
}
