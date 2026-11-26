//! Conditional source contract for a named Array receiver, never provider proof.
//! The package co-seals the declaration inventories with the resolver ledger.
use super::{
    BindingRefV1, CallableSemanticSourceLedgerView, CoreMethodHomeResultRelationV1,
    FunctionOwnerIdV1, ResolvedAssignmentTargetV1, ResolvedLexicalRefV1, ResolvedLiteralSourceV1,
    ResolvedMethodCallReceiverSourceV1, SourceExprSiteV1, SourceStmtSiteV1,
    VerifiedResolvedMethodCallSourceV1, VerifiedResolverCoreMethodCallableContractV1,
    NamedArrayAppendOnlySourceV1, NamedArrayEmptyTransferSourceV1, OwnedExprSiteV1,
    ResolvedAssignmentSourceV1,
};
use crate::analysis::brand_program_declaration_catalog::VerifiedBrandProgramDeclarationCatalogV1;
use crate::parser::ParserOrdinaryBoxSourceCoverageV1;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum NamedArrayRequirementIssueV1 {
    ForeignCall,
    UnsupportedReceiver,
    InitializerMissing,
    AmbiguousInitializer,
    ConstructionMissing,
    NotNamedArray,
    DeclarationCollision,
    DuplicateDeclaration,
    ConstructorShape,
    ReassignedReceiver,
    ValueDemand,
    ArgumentShape,
    TextSourceMissing,
    ForeignTextSource,
}

/// Exact original named construction identity, not a provider capability.
/// Both write and read source consumers borrow this same issuer; operation,
/// element ownership and physical representation remain separate requirements.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NamedArrayConstructionSourceV1 {
    owner: FunctionOwnerIdV1,
    construction: SourceExprSiteV1,
    binding: BindingRefV1,
}

impl NamedArrayConstructionSourceV1 {
    pub(crate) fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }
    pub(crate) fn construction(&self) -> &SourceExprSiteV1 {
        &self.construction
    }
    pub(crate) fn binding(&self) -> BindingRefV1 {
        self.binding
    }

    pub(crate) fn issue(
        ledger: &CallableSemanticSourceLedgerView<'_>,
        call: &VerifiedResolvedMethodCallSourceV1,
        ordinary: &ParserOrdinaryBoxSourceCoverageV1,
        brands: &VerifiedBrandProgramDeclarationCatalogV1,
    ) -> Result<Self, NamedArrayRequirementIssueV1> {
        use NamedArrayRequirementIssueV1 as E;
        if call.owner() != ledger.owner()
            || !ledger
                .method_calls()
                .any(|(site, row)| site == call.site() && std::ptr::eq(row, call))
        {
            return Err(E::ForeignCall);
        }
        let ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(binding)) =
            call.receiver()
        else {
            return Err(E::UnsupportedReceiver);
        };
        if ledger.variable_ref(call.receiver_site())
            != Some(ResolvedLexicalRefV1::Local(binding))
        {
            return Err(E::UnsupportedReceiver);
        }
        Self::issue_for_binding(ledger, binding, ordinary, brands)
    }

    /// The same original construction identity, independent of a get/push call.
    /// This supplies no provider, element, ownership-transfer or End capability.
    pub(crate) fn issue_for_binding(
        ledger: &CallableSemanticSourceLedgerView<'_>,
        binding: BindingRefV1,
        ordinary: &ParserOrdinaryBoxSourceCoverageV1,
        brands: &VerifiedBrandProgramDeclarationCatalogV1,
    ) -> Result<Self, NamedArrayRequirementIssueV1> {
        use NamedArrayRequirementIssueV1 as E;
        if binding.owner() != ledger.owner() { return Err(E::UnsupportedReceiver); }
        let mut initializers = ledger
            .initializer_relations()
            .filter(|row| row.binding() == binding);
        let initializer = initializers.next().ok_or(E::InitializerMissing)?;
        if initializers.next().is_some() {
            return Err(E::AmbiguousInitializer);
        }
        let construction_site = initializer
            .initializer_site()
            .ok_or(E::ConstructionMissing)?;
        let construction = ledger
            .construction_source(construction_site)
            .ok_or(E::ConstructionMissing)?;
        if crate::runtime::CoreBoxId::from_name(construction.class())
            != Some(crate::runtime::CoreBoxId::Array)
        {
            return Err(E::NotNamedArray);
        }
        if ordinary
            .row_for(construction.class())
            .map_err(|_| E::DuplicateDeclaration)?
            .is_some()
            || brands.contains_name(construction.class())
        {
            return Err(E::DeclarationCollision);
        }
        if !construction.arguments().is_empty() || !construction.field_initializers().is_empty() {
            return Err(E::ConstructorShape);
        }
        if ledger.assignment_targets().any(|(_, target)| matches!(target, ResolvedAssignmentTargetV1::BindingRebind(actual) if *actual == binding)) {
            return Err(E::ReassignedReceiver);
        }
        Ok(Self {
            owner: ledger.owner(),
            construction: construction_site.clone(),
            binding,
        })
    }
}

/// Must survive lowering until the actual backend discharges this requirement.
/// Owning this row alone does not authorize a physical Array representation.
#[derive(Debug)]
pub(crate) struct NamedArrayConstructionRequirementV1 {
    source: NamedArrayConstructionSourceV1,
    call: SourceExprSiteV1,
    argument: SourceExprSiteV1,
}

impl NamedArrayConstructionRequirementV1 {
    pub(crate) fn owner(&self) -> FunctionOwnerIdV1 {
        self.source.owner()
    }
    pub(crate) fn construction(&self) -> &SourceExprSiteV1 {
        self.source.construction()
    }
    pub(crate) fn binding(&self) -> BindingRefV1 {
        self.source.binding()
    }
    pub(crate) fn call(&self) -> &SourceExprSiteV1 {
        &self.call
    }
    pub(crate) fn argument(&self) -> &SourceExprSiteV1 {
        &self.argument
    }

    pub(crate) fn issue(
        ledger: &CallableSemanticSourceLedgerView<'_>,
        call: &VerifiedResolvedMethodCallSourceV1,
        ordinary: &ParserOrdinaryBoxSourceCoverageV1,
        brands: &VerifiedBrandProgramDeclarationCatalogV1,
        text_source: Option<&VerifiedResolverCoreMethodCallableContractV1>,
    ) -> Result<Self, NamedArrayRequirementIssueV1> {
        use NamedArrayRequirementIssueV1 as E;
        let source = NamedArrayConstructionSourceV1::issue(ledger, call, ordinary, brands)?;
        if !ledger
            .source_site_inventory()
            .contains_statement(&SourceStmtSiteV1::from_node(call.site().node().clone()))
        {
            return Err(E::ValueDemand);
        }
        let [argument] = call.arguments() else {
            return Err(E::ArgumentShape);
        };
        if call.arity() != 1 || argument.ordinal() != 0 {
            return Err(E::ArgumentShape);
        }
        if !matches!(
            ledger.literal_source(argument.site()),
            Some(ResolvedLiteralSourceV1::String(_))
        ) {
            let text = text_source.ok_or(E::TextSourceMissing)?;
            if text.owner() != ledger.owner()
                || text.call_site() != argument.site()
                || text.result_site() != argument.site()
                || text.target().result() != CoreMethodHomeResultRelationV1::TextToCaller
            {
                return Err(E::ForeignTextSource);
            }
        }
        Ok(Self {
            source,
            call: call.site().clone(),
            argument: argument.site().clone(),
        })
    }
}

/// Original LOCAL holder identity for the source Home walk. Element support,
/// when selected, stays anchored to the same structural source.
#[derive(Debug, Clone)]
pub(crate) struct NamedArrayHolderSourceLoanV1 {
    construction: NamedArrayConstructionSourceV1,
    element_contract: Option<Rc<NamedArrayAppendOnlySourceV1>>,
    append_only: Option<Rc<NamedArrayAppendOnlySourceV1>>,
    empty_transfer: Option<NamedArrayEmptyTransferSourceV1>,
}
impl PartialEq for NamedArrayHolderSourceLoanV1 {
    fn eq(&self, other: &Self) -> bool {
        self.construction == other.construction
            && self.empty_transfer == other.empty_transfer
            && match (&self.element_contract, &other.element_contract) {
                (Some(a), Some(b)) => Rc::ptr_eq(a, b),
                (None, None) => true,
                _ => false,
            }
            && match (&self.append_only, &other.append_only) {
                (Some(a), Some(b)) => Rc::ptr_eq(a, b),
                (None, None) => true,
                _ => false,
            }
    }
}
impl Eq for NamedArrayHolderSourceLoanV1 {}
impl NamedArrayHolderSourceLoanV1 {
    pub(in crate::mir) fn from_element_contract(holder: Rc<NamedArrayAppendOnlySourceV1>) -> Self {
        Self {
            construction: holder.construction().clone(),
            element_contract: Some(holder),
            append_only: None,
            empty_transfer: None,
        }
    }
    pub(in crate::mir) fn from_construction(
        construction: NamedArrayConstructionSourceV1,
        empty_transfer: Option<NamedArrayEmptyTransferSourceV1>,
    ) -> Self {
        Self { construction, element_contract: None, append_only: None, empty_transfer }
    }
    pub(in crate::mir) fn from_append_only(
        construction: NamedArrayConstructionSourceV1,
        append_only: Rc<NamedArrayAppendOnlySourceV1>,
    ) -> Self {
        Self { construction, element_contract: None, append_only: Some(append_only), empty_transfer: None }
    }
    pub(crate) fn proves_empty_transfer(
        &self, assignment: &ResolvedAssignmentSourceV1,
        destination: BindingRefV1, field: &str,
    ) -> bool {
        self.element_contract.is_none() && self.append_only.is_none() && self.empty_transfer.as_ref().is_some_and(|source|
            source.matches_transfer(&self.construction, assignment, destination, field))
    }
    pub(crate) fn has_element_contract(&self) -> bool { self.element_contract.is_some() }
    pub(crate) fn is_dynamic_carrier_append_only_v1(&self) -> bool {
        self.element_contract.is_none() && self.append_only.as_ref().is_some_and(|source|
            source.has_appends() && source.reads().is_empty())
    }
    pub(crate) fn is_exact_empty_transfer_v1(&self) -> bool {
        self.element_contract.is_none() && self.append_only.is_none() && self.empty_transfer.is_some()
    }
    pub(crate) fn original_append_sites_v1(&self) -> Vec<OwnedExprSiteV1> {
        let rows = self.append_only.as_ref().map(|source| source.appends())
            .or_else(|| self.element_contract.as_ref().map(|row| row.appends()));
        rows.into_iter().flatten().map(|row|
            OwnedExprSiteV1::new(self.binding().owner(), row.call().clone())).collect()
    }
    pub(crate) fn append_candidate_is_null_v1(&self, call: &OwnedExprSiteV1) -> Option<bool> {
        if call.owner() != self.binding().owner() { return None; }
        self.element_contract.as_ref()
            .and_then(|row| row.appends().iter().find(|append| append.call() == call.site())
                .map(|append| append.null_candidate()))
            .or_else(|| self.append_only.as_ref()?.append_candidate_is_null_v1(call.site()))
    }
    pub(crate) fn binding(&self) -> BindingRefV1 {
        self.construction.binding()
    }
    pub(in crate::mir) fn construction_site(&self) -> &SourceExprSiteV1 {
        self.construction.construction()
    }
    pub(crate) fn construction(&self) -> OwnedExprSiteV1 {
        OwnedExprSiteV1::new(self.construction.owner(), self.construction_site().clone())
    }
    pub(crate) fn append_candidate(&self, call: &OwnedExprSiteV1) -> Option<OwnedExprSiteV1> {
        if call.owner() != self.binding().owner() {
            return None;
        }
        let candidate = self.element_contract.as_ref()
            .and_then(|row| row.appends().iter().find(|append| append.call() == call.site())
                .map(|append| append.candidate()))
            .or_else(|| self.append_only.as_ref()?.append_candidate_v1(call.site()))?;
        Some(OwnedExprSiteV1::new(call.owner(), candidate.clone()))
    }
    pub(crate) fn append_receiver_v1(
        &self,
        call: &OwnedExprSiteV1,
    ) -> Option<SourceExprSiteV1> {
        if call.owner() != self.binding().owner() {
            return None;
        }
        self.element_contract
            .as_ref()
            .and_then(|row| {
                row.appends()
                    .iter()
                    .find(|append| append.call() == call.site())
                    .map(|append| append.receiver().clone())
            })
            .or_else(|| {
                self.append_only
                    .as_ref()?
                    .appends()
                    .iter()
                    .find(|append| append.call() == call.site())
                    .map(|append| append.receiver().clone())
            })
    }
}
