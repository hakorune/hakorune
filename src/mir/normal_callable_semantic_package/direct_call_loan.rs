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
    taken_site: Option<OwnedExprSiteV1>,
    observed_arguments: Option<Box<[(u32, SourceExprSiteV1, crate::mir::ValueId)]>>,
    argument_projections: Option<Box<[super::LexicalCallArgumentProjectionV1]>>,
    emission: DirectCallEmissionV1,
    execution: DirectCallExecutionV1,
}

#[derive(Debug)]
enum DirectCallEmissionV1 {
    ResolverDirect(VerifiedCanonicalDirectCallEmissionV1),
    Static(static_execution::StaticExecutionEmissionV1),
}
/// One materialization facade; each source keeps its own original authority.
pub(crate) struct LifecycleCallEmissionRefV1<'a>(&'a DirectCallEmissionV1);
impl LifecycleCallEmissionRefV1<'_> {
    pub(in crate::mir) fn materialize_call(&self, dst: Option<crate::mir::ValueId>, args: Vec<crate::mir::ValueId>)
        -> Result<crate::mir::definitions::MirCall, String> {
        match self.0 {
            DirectCallEmissionV1::ResolverDirect(emission) => emission.materialize_call(dst, args).map_err(|error| format!("{error:?}")),
            DirectCallEmissionV1::Static(emission) => emission.materialize_call(dst, args),
        }
    }
}
#[path = "direct_call_static_execution.rs"]
mod static_execution;

#[derive(Debug)]
enum DirectCallExecutionV1 {
    Scalar,
    Lifecycle(InvokeCallResultKind),
}

#[path = "direct_call_lifecycle.rs"]
pub(in crate::mir::normal_callable_semantic_package) mod lifecycle;

impl DirectCallDispositionRowV1 {
    pub(in crate::mir) fn static_source_v1(&self) -> Option<&std::rc::Rc<super::StaticIncomingSourceV1>> {
        match &self.emission { DirectCallEmissionV1::Static(emission) => Some(emission.source()), DirectCallEmissionV1::ResolverDirect(_) => None }
    }
    pub(in crate::mir) fn retain_static_publication_v1(&mut self, publication: crate::mir::callable_result_representation::StaticCallResultPublicationTakeV1) -> Result<(), String> {
        match &mut self.emission {
            DirectCallEmissionV1::Static(emission) => emission.retain_publication(publication),
            DirectCallEmissionV1::ResolverDirect(_) => Err("[freeze:contract][static-execution/foreign-publication-owner]".into()),
        }
    }
    pub(in crate::mir) fn validate_static_source_v1(&self, ledger: &super::OrdinaryNewClaimLedgerV1) -> Result<(), String> {
        let Some(source) = self.static_source_v1() else { return Ok(()) };
        ledger.verify_original_static_source_v1(source)?;
        if self.taken_site() != Some(source.call_site()) || self.argument_sites() != source.argument_sites()
            || self.lifecycle_emission().is_err() {
            return Err("[freeze:contract][static-execution/source-drift]".into());
        }
        if let DirectCallEmissionV1::Static(emission) = &self.emission {
            emission.validate()?;
            emission.validate_publication()?;
            ledger.verify_original_static_execution_signature_v1(source, emission.signatures())?;
        }
        Ok(())
    }
    pub(in crate::mir) fn taken_site(&self) -> Option<&OwnedExprSiteV1> { self.taken_site.as_ref() }
    pub(in crate::mir) fn observed_arguments(&self) -> Option<&[(u32, SourceExprSiteV1, crate::mir::ValueId)]> {
        self.observed_arguments.as_deref()
    }
    pub(in crate::mir) fn argument_projections_v1(&self) -> Option<&[super::LexicalCallArgumentProjectionV1]> {
        self.argument_projections.as_deref()
    }
    pub(in crate::mir) fn retain_argument_projections_v1(&mut self, projections: Vec<super::LexicalCallArgumentProjectionV1>) -> Result<(), String> {
        if self.argument_projections.is_some() || self.observed_arguments.is_none() || projections.len() != self.argument_sites.len() {
            return Err("[freeze:contract][direct-call/argument-projection-cardinality]".into());
        }
        self.argument_projections = Some(projections.into_boxed_slice());
        Ok(())
    }
    pub(in crate::mir) fn retain_argument_descent(
        &mut self, observations: Vec<(u32, SourceExprSiteV1, crate::mir::ValueId)>,
    ) -> Result<(), String> {
        if self.observed_arguments.is_some() || self.taken_site.is_none()
            || observations.len() != self.argument_sites.len()
            || observations.iter().enumerate().any(|(index, (ordinal, site, _))|
                *ordinal as usize != index || self.argument_sites.get(index) != Some(site)) {
            return Err("[freeze:contract][direct-call/argument-descent-drift]".into());
        }
        self.observed_arguments = Some(observations.into_boxed_slice());
        Ok(())
    }

    fn resolver_emission(&self) -> Option<&VerifiedCanonicalDirectCallEmissionV1> {
        match &self.emission { DirectCallEmissionV1::ResolverDirect(emission) => Some(emission), DirectCallEmissionV1::Static(_) => None }
    }
    pub(crate) fn physical_emission(&self) -> Result<&VerifiedCanonicalDirectCallEmissionV1, DirectCallLoanErrorV1> {
        self.resolver_emission().ok_or(DirectCallLoanErrorV1::LifecycleSourceMismatch)
    }
    pub(in crate::mir) fn callee_owner_v1(&self) -> FunctionOwnerIdV1 {
        match &self.emission {
            DirectCallEmissionV1::ResolverDirect(emission) => emission.target().callable().owner(),
            DirectCallEmissionV1::Static(emission) => emission.source().callee_owner(),
        }
    }
    pub(in crate::mir) fn published_key_v1(&self) -> Option<&hakorune_mir_defs::CanonicalSameModuleCallableKeyV1> {
        match &self.emission {
            DirectCallEmissionV1::ResolverDirect(emission) => emission.target().published_key(),
            DirectCallEmissionV1::Static(emission) => Some(emission.source().target()),
        }
    }
    pub(crate) fn lifecycle_emission(&self) -> Result<LifecycleCallEmissionRefV1<'_>, DirectCallLoanErrorV1> {
        match self.execution {
            DirectCallExecutionV1::Lifecycle(_) => Ok(LifecycleCallEmissionRefV1(&self.emission)),
            DirectCallExecutionV1::Scalar => Err(DirectCallLoanErrorV1::LifecycleSourceMismatch),
        }
    }
    pub(crate) fn new(
        argument_sites: Box<[SourceExprSiteV1]>,
        emission: VerifiedCanonicalDirectCallEmissionV1,
    ) -> Self {
        Self {
            argument_sites,
            taken_site: None,
            observed_arguments: None,
            argument_projections: None,
            emission: DirectCallEmissionV1::ResolverDirect(emission),
            execution: DirectCallExecutionV1::Scalar,
        }
    }

    pub(crate) fn result(&self) -> Option<InvokeCallResultKind> {
        match self.execution {
            DirectCallExecutionV1::Lifecycle(result) => Some(result),
            DirectCallExecutionV1::Scalar =>
                (self.resolver_emission()?.target().signature().result()
                    == Some(crate::mir::exact_trivial_scalar_abi::ExactTrivialScalarAbiV1::I64))
                    .then_some(InvokeCallResultKind::I64),
        }
    }

    pub(crate) fn argument_sites(&self) -> &[SourceExprSiteV1] {
        &self.argument_sites
    }

    pub(crate) fn scalar_emission(&self) -> Result<&VerifiedCanonicalDirectCallEmissionV1, DirectCallLoanErrorV1> {
        match self.execution {
            DirectCallExecutionV1::Scalar => self.physical_emission(),
            DirectCallExecutionV1::Lifecycle(_) => Err(DirectCallLoanErrorV1::LifecycleConsumerMissing),
        }
    }


}

type DirectCallDispositionSlotV1 =
    crate::mir::normal_callable_semantic_package::disposition_slot::DispositionSlotV1<
        DirectCallDispositionRowV1,
    >;

/// A private affine inventory for one exact callable owner.
#[must_use]
#[derive(Debug)]
pub(crate) struct DirectCallDispositionLoanV1 {
    owner: FunctionOwnerIdV1,
    rows: BTreeMap<OwnedExprSiteV1, DirectCallDispositionSlotV1>,
}

impl DirectCallDispositionLoanV1 {

    pub(crate) fn take_terminal(
        &mut self,
        ledger: &super::OrdinaryNewClaimLedgerV1,
        owner: FunctionOwnerIdV1,
        return_site: &crate::mir::resolved_semantics::SourceNodeSiteV1,
    ) -> Result<Option<DirectCallDispositionRowV1>, DirectCallLoanErrorV1> {
        let stmt_site =
            crate::mir::resolved_semantics::SourceStmtSiteV1::from_node(return_site.clone());
        let Some((completion, terminal)) =
            ledger.call_source_completion_for_owner_at(owner, &stmt_site)
        else {
            return Ok(None);
        };
        if owner != self.owner
            || completion.owner() != owner
            || terminal.owner() != owner
            || terminal.return_site().node() != return_site
            || !completion.explicit_sites().contains(terminal.return_site())
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
        Ok(Self {
            owner,
            rows: slots,
        })
    }

    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir) fn take_optional_static_execution_v1(
        &mut self, owner: FunctionOwnerIdV1, site: SourceExprSiteV1,
    ) -> Result<Option<DirectCallDispositionRowV1>, DirectCallLoanErrorV1> {
        if owner != self.owner { return Err(DirectCallLoanErrorV1::OwnerMismatch); }
        let key = OwnedExprSiteV1::new(owner, site.clone());
        match self.rows.get(&key) {
            Some(DirectCallDispositionSlotV1::Ready(row)) if row.static_source_v1().is_some() => {
                row.lifecycle_emission()?;
                self.take_once(owner, site).map(Some)
            }
            Some(DirectCallDispositionSlotV1::Taken) => Err(DirectCallLoanErrorV1::SiteAlreadyTaken),
            _ => Ok(None),
        }
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
            DirectCallDispositionSlotV1::Ready(mut row) => {
                row.taken_site = Some(key);
                Ok(row)
            },
            DirectCallDispositionSlotV1::Taken => Err(DirectCallLoanErrorV1::SiteAlreadyTaken),
        }
    }

    /// Every lifecycle-admitted row's borrowed-Map formal ordinals with the
    /// callee owner the signature belongs to — the sealed call edge half the
    /// install-side co-seal matches against described `ArgumentHandoff`
    /// obligations. Nothing here re-reads the source; the signature's own
    /// sealed ABI is the evidence.
    pub(in crate::mir::normal_callable_semantic_package) fn map_argument_edges(&self) -> Vec<(OwnedExprSiteV1, u32, FunctionOwnerIdV1)> {
        self.map_edges_v1(true)
    }
    pub(in crate::mir::normal_callable_semantic_package) fn map_formal_edges(&self) -> Vec<(OwnedExprSiteV1, u32, FunctionOwnerIdV1)> {
        self.map_edges_v1(false)
    }
    fn map_edges_v1(&self, lifecycle_only: bool) -> Vec<(OwnedExprSiteV1, u32, FunctionOwnerIdV1)> {
        self.rows.iter().filter_map(|(site, slot)| match slot {
            DirectCallDispositionSlotV1::Ready(row) if !lifecycle_only || matches!(row.execution, DirectCallExecutionV1::Lifecycle(_)) =>
                row.resolver_emission().map(|emission| (site, emission)),
            _ => None,
        }).flat_map(|(site, emission)| emission.target().signature().params().iter().enumerate()
            .filter(|(_, kind)| **kind == crate::mir::resolved_semantics::ExactCallableParamAbiV1::Map)
            .map(move |(ordinal, _)| (site.clone(), ordinal as u32, emission.target().callable().owner())))
            .collect()
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

    pub(in crate::mir) fn has_static_execution_for_owner_v1(&self, owner: FunctionOwnerIdV1) -> bool {
        self.loans.get(&owner).is_some_and(|loan| loan.rows.values().any(|slot| matches!(slot,
            DirectCallDispositionSlotV1::Ready(row) if row.static_source_v1().is_some() && row.lifecycle_emission().is_ok())))
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
        self.loans
            .into_values()
            .try_for_each(DirectCallDispositionLoanV1::finish_empty)
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
