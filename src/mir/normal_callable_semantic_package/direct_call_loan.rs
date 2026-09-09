//! Package-owned, move-only direct-call disposition for the selected App Main.
//!
//! The loan carries only products already issued by the resolver session.  It
//! never resolves a source name and it never reconstructs a target from a raw
//! symbol.  A raw consumer may take one owned row for one owner/site and must
//! leave no row behind before the package closes.

use std::collections::BTreeMap;

use crate::mir::canonical_direct_call::VerifiedCanonicalDirectCallEmissionV1;
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, OwnedExprSiteV1, SourceExprSiteV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AppMainDirectCallLoanErrorV1 {
    OwnerMismatch,
    SiteMissing,
    SiteAlreadyTaken,
    ResidualRows,
    DuplicateSite,
    LifecycleSourceMismatch,
    LifecycleConsumerMissing,
}

#[derive(Debug)]
pub(crate) struct AppMainDirectCallDispositionRowV1 {
    argument_sites: Box<[SourceExprSiteV1]>,
    emission: VerifiedCanonicalDirectCallEmissionV1,
    execution: AppMainCallExecutionV1,
}

#[derive(Debug)]
enum AppMainCallExecutionV1 {
    Scalar,
    Lifecycle,
}

#[path = "direct_call_lifecycle.rs"]
mod lifecycle;

impl AppMainDirectCallDispositionRowV1 {
    pub(crate) fn lifecycle_emission(
        &self,
    ) -> Result<&VerifiedCanonicalDirectCallEmissionV1, AppMainDirectCallLoanErrorV1> {
        match self.execution {
            AppMainCallExecutionV1::Lifecycle => Ok(&self.emission),
            AppMainCallExecutionV1::Scalar => {
                Err(AppMainDirectCallLoanErrorV1::LifecycleSourceMismatch)
            }
        }
    }
    pub(crate) fn new(
        argument_sites: Box<[SourceExprSiteV1]>,
        emission: VerifiedCanonicalDirectCallEmissionV1,
    ) -> Self {
        Self {
            argument_sites,
            emission,
            execution: AppMainCallExecutionV1::Scalar,
        }
    }

    pub(crate) fn argument_sites(&self) -> &[SourceExprSiteV1] {
        &self.argument_sites
    }

    pub(crate) fn into_scalar_emission(
        self,
    ) -> Result<VerifiedCanonicalDirectCallEmissionV1, AppMainDirectCallLoanErrorV1> {
        match self.execution {
            AppMainCallExecutionV1::Scalar => Ok(self.emission),
            AppMainCallExecutionV1::Lifecycle => {
                Err(AppMainDirectCallLoanErrorV1::LifecycleConsumerMissing)
            }
        }
    }
}

#[derive(Debug)]
enum AppMainDirectCallDispositionSlotV1 {
    Ready(AppMainDirectCallDispositionRowV1),
    Taken,
}

/// A private affine inventory for one exact App Main owner.
#[must_use]
#[derive(Debug)]
pub(crate) struct AppMainDirectCallDispositionLoanV1 {
    owner: FunctionOwnerIdV1,
    rows: BTreeMap<OwnedExprSiteV1, AppMainDirectCallDispositionSlotV1>,
}

impl AppMainDirectCallDispositionLoanV1 {
    /// Take only the exact Call attached to this ledger's original Completion.
    pub(crate) fn take_terminal_lifecycle(
        &mut self,
        ledger: &super::OrdinaryNewClaimLedgerV1,
        owner: FunctionOwnerIdV1,
        return_site: &crate::mir::resolved_semantics::SourceNodeSiteV1,
    ) -> Result<Option<AppMainDirectCallDispositionRowV1>, AppMainDirectCallLoanErrorV1> {
        let Some((completion, terminal)) = ledger.call_source_completion() else {
            return Ok(None);
        };
        if owner != self.owner
            || completion.owner() != owner
            || terminal.owner() != owner
            || terminal.return_site().node() != return_site
            || completion.explicit_site() != Some(terminal.return_site())
        {
            return Err(AppMainDirectCallLoanErrorV1::LifecycleSourceMismatch);
        }
        let row = self.take_once(owner, terminal.call_site().clone())?;
        row.lifecycle_emission()?;
        Ok(Some(row))
    }
    pub(crate) fn from_rows(
        owner: FunctionOwnerIdV1,
        rows: impl IntoIterator<Item = (SourceExprSiteV1, AppMainDirectCallDispositionRowV1)>,
    ) -> Result<Self, AppMainDirectCallLoanErrorV1> {
        let mut slots = BTreeMap::new();
        for (site, row) in rows {
            if slots
                .insert(
                    OwnedExprSiteV1::new(owner, site),
                    AppMainDirectCallDispositionSlotV1::Ready(row),
                )
                .is_some()
            {
                return Err(AppMainDirectCallLoanErrorV1::DuplicateSite);
            }
        }
        if slots.is_empty() {
            return Err(AppMainDirectCallLoanErrorV1::ResidualRows);
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
    ) -> Result<AppMainDirectCallDispositionRowV1, AppMainDirectCallLoanErrorV1> {
        if owner != self.owner {
            return Err(AppMainDirectCallLoanErrorV1::OwnerMismatch);
        }
        let key = OwnedExprSiteV1::new(owner, site);
        let slot = self
            .rows
            .get_mut(&key)
            .ok_or(AppMainDirectCallLoanErrorV1::SiteMissing)?;
        match std::mem::replace(slot, AppMainDirectCallDispositionSlotV1::Taken) {
            AppMainDirectCallDispositionSlotV1::Ready(row) => Ok(row),
            AppMainDirectCallDispositionSlotV1::Taken => {
                Err(AppMainDirectCallLoanErrorV1::SiteAlreadyTaken)
            }
        }
    }

    pub(crate) fn finish_empty(self) -> Result<(), AppMainDirectCallLoanErrorV1> {
        if self
            .rows
            .values()
            .any(|slot| matches!(slot, AppMainDirectCallDispositionSlotV1::Ready(_)))
        {
            return Err(AppMainDirectCallLoanErrorV1::ResidualRows);
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
        let mut loan = AppMainDirectCallDispositionLoanV1::from_rows(
            owner,
            [(
                site.clone(),
                AppMainDirectCallDispositionRowV1::new(Box::new([]), emission),
            )],
        )
        .expect("one-row loan");
        assert!(loan.take_once(owner, site.clone()).is_ok());
        assert_eq!(
            loan.take_once(owner, site).err(),
            Some(AppMainDirectCallLoanErrorV1::SiteAlreadyTaken)
        );
        loan.finish_empty().expect("no residual rows");
    }

    #[test]
    fn take_once_rejects_foreign_owner_without_consuming_row() {
        let (owner, site, emission) = fixture();
        let foreign = foreign_owner();
        let mut loan = AppMainDirectCallDispositionLoanV1::from_rows(
            owner,
            [(
                site.clone(),
                AppMainDirectCallDispositionRowV1::new(Box::new([]), emission),
            )],
        )
        .expect("one-row loan");
        assert_eq!(
            loan.take_once(foreign, site.clone()).err(),
            Some(AppMainDirectCallLoanErrorV1::OwnerMismatch)
        );
        assert!(loan.take_once(owner, site).is_ok());
    }

    #[test]
    fn finish_empty_rejects_residual_rows() {
        let (owner, site, emission) = fixture();
        let loan = AppMainDirectCallDispositionLoanV1::from_rows(
            owner,
            [(
                site,
                AppMainDirectCallDispositionRowV1::new(Box::new([]), emission),
            )],
        )
        .expect("one-row loan");
        assert_eq!(
            loan.finish_empty(),
            Err(AppMainDirectCallLoanErrorV1::ResidualRows)
        );
    }

    #[test]
    fn from_rows_rejects_duplicate_sites() {
        let (owner, site, emission) = fixture();
        let second = VerifiedCanonicalDirectCallEmissionV1::clone(&emission);
        assert_eq!(
            AppMainDirectCallDispositionLoanV1::from_rows(
                owner,
                [
                    (
                        site.clone(),
                        AppMainDirectCallDispositionRowV1::new(Box::new([]), emission),
                    ),
                    (
                        site,
                        AppMainDirectCallDispositionRowV1::new(Box::new([]), second),
                    ),
                ],
            )
            .err(),
            Some(AppMainDirectCallLoanErrorV1::DuplicateSite)
        );
    }
}
