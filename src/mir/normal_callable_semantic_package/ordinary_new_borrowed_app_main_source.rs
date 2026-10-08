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

impl BorrowedAppMainSourceLoanV1<'_> {
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
