//! Conditional source contract for a named Array receiver, never provider proof.
//! The package co-seals the declaration inventories with the resolver ledger.
use super::{
    BindingRefV1, CallableSemanticSourceLedgerView, CoreMethodHomeResultRelationV1,
    FunctionOwnerIdV1, ResolvedAssignmentTargetV1, ResolvedLexicalRefV1, ResolvedLiteralSourceV1,
    ResolvedMethodCallReceiverSourceV1, SourceExprSiteV1, SourceStmtSiteV1,
    VerifiedResolvedMethodCallSourceV1, VerifiedResolverCoreMethodCallableContractV1,
};
use crate::analysis::brand_program_declaration_catalog::VerifiedBrandProgramDeclarationCatalogV1;
use crate::parser::ParserOrdinaryBoxSourceCoverageV1;

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

/// Must survive lowering until the actual backend discharges this requirement.
/// Owning this row alone does not authorize a physical Array representation.
#[derive(Debug)]
pub(crate) struct NamedArrayConstructionRequirementV1 {
    owner: FunctionOwnerIdV1,
    construction: SourceExprSiteV1,
    binding: BindingRefV1,
    call: SourceExprSiteV1,
    argument: SourceExprSiteV1,
}

impl NamedArrayConstructionRequirementV1 {
    pub(crate) fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }
    pub(crate) fn construction(&self) -> &SourceExprSiteV1 {
        &self.construction
    }
    pub(crate) fn binding(&self) -> BindingRefV1 {
        self.binding
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
        if binding.owner() != ledger.owner()
            || ledger.variable_ref(call.receiver_site())
                != Some(ResolvedLexicalRefV1::Local(binding))
        {
            return Err(E::UnsupportedReceiver);
        }
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
            owner: ledger.owner(),
            construction: construction_site.clone(),
            binding,
            call: call.site().clone(),
            argument: argument.site().clone(),
        })
    }
}
