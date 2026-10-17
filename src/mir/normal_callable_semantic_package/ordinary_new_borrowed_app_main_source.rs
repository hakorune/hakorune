//! Borrow the original parser/catalog Main co-seal at the cohort boundary.
use super::*;
use std::rc::Rc;
use crate::mir::builder::{AppMainCatalogCoSealV1, CanonicalSameModuleCallableKeyV1};
use crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1;
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::OrdinaryNewCoSealIssueV1;

pub(in crate::mir::normal_callable_semantic_package) struct BorrowedAppMainSourceLoanV1<'a> {
    source: &'a AppMainCatalogCoSealV1,
    owner: FunctionOwnerIdV1,
    batch_slot: u32,
}

/// Immutable parameter policy issued only from the original Main co-seal.
/// Runtime payloads and physical coordinates belong to emission, never here.
#[derive(Debug)]
pub(in crate::mir) struct MainArgumentsSourceV1 {
    identity: crate::parser::CallableDeclarationIdentityV1,
    key: CanonicalSameModuleCallableKeyV1,
    brand: crate::mir::builder::SameModuleCallableCatalogBrandV1,
    owner: FunctionOwnerIdV1,
    batch_slot: u32,
    parameters: Box<[MainArgumentParameterV1]>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum MainArgumentKindV1 { ScriptArguments, DefaultVoid }
#[derive(Debug)]
pub(in crate::mir) struct MainArgumentParameterV1 {
    ordinal: u32,
    binding: BindingRefV1,
    name: String,
    kind: MainArgumentKindV1,
}
impl MainArgumentParameterV1 {
    pub(in crate::mir) fn binding(&self) -> BindingRefV1 { self.binding }
    pub(in crate::mir) fn name(&self) -> &str { &self.name }
    pub(in crate::mir) fn kind(&self) -> MainArgumentKindV1 { self.kind }
}
impl MainArgumentsSourceV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn matches_catalog_v1(
        &self, source: &AppMainCatalogCoSealV1,
    ) -> bool {
        self.identity.same_as(source.parser_identity()) && &self.key == source.catalog_key()
            && self.brand.is_same(source.catalog_brand())
    }
    pub(in crate::mir::normal_callable_semantic_package) fn require_original_v1(
        &self, batch: &VerifiedResolvedCallableSemanticBatchV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
    ) -> Result<(), &'static str> {
        AppMainSourceRefV1 { identity: &self.identity, key: &self.key }
            .checked_caller_key(batch, contracts, self.owner, self.batch_slot)?;
        if !batch.with_lowering_input(self.batch_slot, |input| self.corroborates_input(input, &self.identity))
            .map_err(|_| "main-arguments/source-input")? { return Err("main-arguments/source-parameters"); }
        Ok(())
    }
    pub(in crate::mir) fn parameters(&self) -> &[MainArgumentParameterV1] { &self.parameters }
    pub(in crate::mir) fn requires_lifecycle(&self) -> bool {
        self.parameters.iter().any(|row| row.kind == MainArgumentKindV1::ScriptArguments)
    }
    pub(in crate::mir) fn resource_ordinal(&self) -> Option<usize> {
        self.parameters.iter().position(|row| row.kind == MainArgumentKindV1::ScriptArguments)
    }
    pub(in crate::mir) fn corroborates_input(
        &self, input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
        identity: &crate::parser::CallableDeclarationIdentityV1,
    ) -> bool {
        let function = input.function();
        self.identity.same_as(identity) && self.owner == input.owner()
            && function.declaration_sites().filter(|site| matches!(site,
                crate::mir::resolved_semantics::SourceBindingSiteV1::Parameter { .. })).count() == self.parameters.len()
            && self.parameters.iter().enumerate().all(|(ordinal, row)| {
                row.ordinal as usize == ordinal && row.binding.owner() == self.owner
                    && function.declaration_binding(&crate::mir::resolved_semantics::SourceBindingSiteV1::Parameter {
                        index: row.ordinal }) == Some(row.binding)
                    && function.binding(row.binding).is_some_and(|binding| binding.diagnostic_name() == row.name)
            })
    }
    pub(in crate::mir::normal_callable_semantic_package) fn corroborates_anchor(
        &self, owner: FunctionOwnerIdV1, identity: &crate::parser::CallableDeclarationIdentityV1,
        key: &CanonicalSameModuleCallableKeyV1,
    ) -> bool {
        self.owner == owner && self.identity.same_as(identity) && &self.key == key
    }
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
    pub(in crate::mir::normal_callable_semantic_package) fn arguments_source_v1(
        &self, batch: &VerifiedResolvedCallableSemanticBatchV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
    ) -> Result<Rc<MainArgumentsSourceV1>, &'static str> {
        self.caller_source_ref_v1().checked_caller_key(batch, contracts, self.owner, self.batch_slot)?;
        let contract = contracts.iter().find(|row| row.owner == self.owner)
            .ok_or("main-arguments/source-contract")?;
        if !self.matches_contract(contract) { return Err("main-arguments/source-contract"); }
        batch.with_lowering_input(self.batch_slot, |input| {
            let parameters = contract.parameters.iter().map(|formal| {
                let binding = input.function().binding(formal.binding)
                    .ok_or("main-arguments/source-binding")?;
                let name = binding.diagnostic_name().to_owned();
                let kind = if name == "args" { MainArgumentKindV1::ScriptArguments }
                    else { MainArgumentKindV1::DefaultVoid };
                Ok(MainArgumentParameterV1 { ordinal: formal.ordinal, binding: formal.binding, name, kind })
            }).collect::<Result<Vec<_>, &'static str>>()?;
            if parameters.iter().filter(|row| row.kind == MainArgumentKindV1::ScriptArguments).count() > 1 {
                return Err("main-arguments/duplicate-args");
            }
            let source = Rc::new(MainArgumentsSourceV1 { identity: self.source.parser_identity().clone(),
                key: self.catalog_key().clone(), brand: self.source.catalog_brand().clone(),
                owner: self.owner, batch_slot: self.batch_slot,
                parameters: parameters.into_boxed_slice() });
            source.require_original_v1(batch, contracts)?;
            if !source.matches_catalog_v1(self.source) { return Err("main-arguments/source-catalog"); }
            if !source.corroborates_input(input, self.source.parser_identity()) {
                return Err("main-arguments/source-parameters");
            }
            Ok(source)
        }).map_err(|_| "main-arguments/source-input")?
    }

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
