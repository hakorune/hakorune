//! Borrow the original parser/catalog Main co-seal at the cohort boundary.
use super::*;
use crate::mir::builder::{AppMainCatalogCoSealV1, CanonicalSameModuleCallableKeyV1};
use crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1;
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::OrdinaryNewCoSealIssueV1;

pub(in crate::mir::normal_callable_semantic_package) struct BorrowedAppMainSourceLoanV1<'a> {
    source: &'a AppMainCatalogCoSealV1,
    owner: FunctionOwnerIdV1,
    batch_slot: u32,
}

/// Ephemeral view of the already-issued Main relation. Neither borrower
/// stores another source registry or synthesizes a callable key.
#[derive(Clone, Copy)]
pub(in crate::mir::normal_callable_semantic_package) struct AppMainSourceRefV1<'a> {
    identity: &'a crate::parser::CallableDeclarationIdentityV1,
    key: &'a CanonicalSameModuleCallableKeyV1,
}
impl<'a> AppMainSourceRefV1<'a> {
    pub(in crate::mir::normal_callable_semantic_package) fn checked_caller_key(
        self, batch: &VerifiedResolvedCallableSemanticBatchV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        owner: FunctionOwnerIdV1, slot: u32,
    ) -> Result<&'a CanonicalSameModuleCallableKeyV1, &'static str> {
        let mut declarations = batch.declarations().filter(|row| row.identity().same_as(self.identity));
        let declaration = declarations.next().ok_or("original-caller/app-main-identity")?;
        if declarations.next().is_some() || declaration.owner() != owner || declaration.batch_slot() != slot
            || self.key.namespace() != hakorune_mir_defs::SameModuleCallableNamespaceV1::StaticBoxMethod
        { return Err("original-caller/app-main-identity"); }
        let mut rows = contracts.iter().filter(|row| row.owner == owner);
        let row = rows.next().ok_or("original-caller/app-main-contract")?;
        if rows.next().is_some() || row.batch_slot != slot
            || row.mode != crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::StaticBoxMethod
            || row.parameters.len() != self.key.arity() as usize
            || row.parameters.iter().enumerate().any(|(index, formal)| formal.ordinal as usize != index || formal.binding.owner() != owner)
        { return Err("original-caller/app-main-contract"); }
        let exact = batch.with_lowering_input(slot, |input| input.owner() == owner
            && row.parameters.iter().all(|formal| input.function().declaration_binding(
                &crate::mir::resolved_semantics::SourceBindingSiteV1::Parameter { index: formal.ordinal }) == Some(formal.binding)))
            .map_err(|_| "original-caller/app-main-input")?;
        if !exact { return Err("original-caller/app-main-contract"); }
        Ok(self.key)
    }
}

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn app_main_caller_source_ref_v1(&self) -> Option<AppMainSourceRefV1<'_>> {
        Some(AppMainSourceRefV1 { identity: self.app_main_identity.as_ref()?, key: self.app_main_catalog_key.as_ref()? })
    }
}

impl BorrowedAppMainSourceLoanV1<'_> {
    pub(in crate::mir::normal_callable_semantic_package) fn caller_source_ref_v1(&self) -> AppMainSourceRefV1<'_> {
        AppMainSourceRefV1 { identity: self.source.parser_identity(), key: self.source.catalog_key() }
    }

    pub(in crate::mir::normal_callable_semantic_package) fn batch_slot(&self) -> u32 {
        self.batch_slot
    }
    pub(in crate::mir::normal_callable_semantic_package) fn catalog_key(
        &self,
    ) -> &CanonicalSameModuleCallableKeyV1 {
        self.source.catalog_key()
    }
    pub(in crate::mir::normal_callable_semantic_package) fn matches_function(
        &self,
        owner: FunctionOwnerIdV1,
        batch_slot: u32,
        key: &CanonicalSameModuleCallableKeyV1,
    ) -> bool {
        self.owner == owner && self.batch_slot == batch_slot && self.catalog_key() == key
    }
    pub(in crate::mir::normal_callable_semantic_package) fn matches_contract(
        &self,
        contract: &OwnedCallableParameterContractDeclarationV1,
    ) -> bool {
        contract.owner == self.owner && contract.batch_slot == self.batch_slot
            && contract.mode == crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::StaticBoxMethod
            && contract.parameters.len() == self.source.catalog_key().arity() as usize
            && contract.parameters.iter().enumerate().all(|(ordinal, formal)| formal.ordinal as usize == ordinal && formal.binding.owner() == self.owner)
    }
}

/// This is the existing exact declaration-identity query, now retaining its
/// catalog relation and owner. No name lookup or selected Main row is issued.
pub(in crate::mir::normal_callable_semantic_package) fn borrow_app_main_source_v1<'a>(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    source: Option<&'a AppMainCatalogCoSealV1>,
) -> Result<Option<BorrowedAppMainSourceLoanV1<'a>>, OrdinaryNewCoSealIssueV1> {
    source
        .map(|source| {
            let mut matches = batch
                .declarations()
                .filter(|row| row.identity().same_as(source.parser_identity()));
            let declaration = matches
                .next()
                .ok_or(OrdinaryNewCoSealIssueV1::AppMainIdentityMissing)?;
            if matches.next().is_some() {
                return Err(OrdinaryNewCoSealIssueV1::AppMainIdentityDuplicate);
            }
            Ok(BorrowedAppMainSourceLoanV1 {
                source,
                owner: declaration.owner(),
                batch_slot: declaration.batch_slot(),
            })
        })
        .transpose()
}

#[cfg(test)]
#[path = "qualified_static_input_source_tests.rs"]
mod input_source_tests;
