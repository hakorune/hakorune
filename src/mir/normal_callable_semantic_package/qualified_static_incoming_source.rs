//! Borrow the original static target row together with its complete input contract.
//! Result-required ordinals never classify a parameter or erase its carrier tag.
use super::*;
use crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1;
use crate::mir::resolved_semantics::{
    FunctionOwnerIdV1, OwnedExprSiteV1, ResolvedMethodCallReceiverSourceV1,
    VerifiedResolvedMethodCallSourceV1,
};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
enum StaticIncomingRouteV1 {
    Qualified(crate::mir::resolved_semantics::home_new_prefix::QualifiedStaticCallClaimV1),
    CurrentOwner(super::CurrentOwnerStaticCallSourceV1),
}

impl StaticIncomingRouteV1 {
    fn required_i64_arguments(&self) -> &[u32] {
        match self {
            Self::Qualified(claim) => claim.required_i64_arguments(),
            Self::CurrentOwner(row) => match row.result() {
                VerifiedCallableResultDispositionV1::ExactI64 {
                    required_i64_arguments,
                } => required_i64_arguments,
                _ => &[],
            },
        }
    }
    fn receiver(&self) -> ResolvedMethodCallReceiverSourceV1 {
        match self {
            Self::Qualified(_) => ResolvedMethodCallReceiverSourceV1::QualifiedUnbound,
            Self::CurrentOwner(_) => ResolvedMethodCallReceiverSourceV1::CurrentOwner,
        }
    }
}

/// Owned source identity retained once per original call. No physical values,
/// receiver lease, executable actuals or finished-coordinate claim live here.
#[derive(Debug)]
pub(in crate::mir) struct StaticIncomingSourceV1 {
    catalog_brand: crate::mir::builder::SameModuleCallableCatalogBrandV1,
    caller: CanonicalSameModuleCallableKeyV1,
    call: OwnedExprSiteV1,
    target: CanonicalSameModuleCallableKeyV1,
    target_batch_slot: u32,
    callee_owner: FunctionOwnerIdV1,
    argument_sites: Box<[SourceExprSiteV1]>,
    parameters: Box<[super::super::model::OwnedCallableParameterContractV1]>,
    route: StaticIncomingRouteV1,
}

impl StaticIncomingSourceV1 {
    pub(in crate::mir) fn is_qualified(&self) -> bool {
        matches!(self.route, StaticIncomingRouteV1::Qualified(_))
    }
    /// Exact source-only zero-input I64 law; no entry or packet permission.
    pub(in crate::mir::normal_callable_semantic_package) fn is_zeroarg_i64_v1(&self) -> bool {
        self.target.arity() == 0
            && self.argument_sites.is_empty()
            && self.parameters.is_empty()
            && match &self.route {
                StaticIncomingRouteV1::Qualified(claim) => {
                    claim.required_i64_arguments().is_empty()
                }
                StaticIncomingRouteV1::CurrentOwner(row) => matches!(row.result(),
                    VerifiedCallableResultDispositionV1::ExactI64 { required_i64_arguments }
                        if required_i64_arguments.is_empty()),
            }
    }

    pub(in crate::mir) fn require_qualified(&self) -> Result<(), String> {
        if self.is_qualified() {
            Ok(())
        } else {
            Err("[freeze:contract][borrowed-static/qualified-source-required]".into())
        }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn current_owner_source(
        &self,
    ) -> Option<&super::CurrentOwnerStaticCallSourceV1> {
        match &self.route {
            StaticIncomingRouteV1::CurrentOwner(row) => Some(row),
            StaticIncomingRouteV1::Qualified(_) => None,
        }
    }

    pub(in crate::mir::normal_callable_semantic_package) fn caller(
        &self,
    ) -> &CanonicalSameModuleCallableKeyV1 {
        &self.caller
    }

    /// Same catalog/caller/site/result handoff; this grants no entry or physical ABI.
    pub(in crate::mir::normal_callable_semantic_package) fn corroborates_publication_handoff(
        &self,
        handoff: &crate::mir::callable_result_representation::VerifiedStaticCallResultPublicationHandoffV1,
    ) -> bool {
        self.is_qualified() && handoff.catalog_identity() == self.catalog_brand.identity()
            && handoff.caller() == &self.caller
            && handoff.site() == self.call.site()
            && handoff.target() == &self.target
            && handoff.representation() == &crate::mir::callable_result_representation::VerifiedCallableResultRepresentationV1::ExactI64
            && handoff.required_callee_i64_arguments() == self.required_i64_arguments()
    }
    pub(in crate::mir::normal_callable_semantic_package) fn call_site(&self) -> &OwnedExprSiteV1 {
        &self.call
    }
    pub(in crate::mir) fn target(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.target
    }
    pub(in crate::mir::normal_callable_semantic_package) fn target_batch_slot(&self) -> u32 {
        self.target_batch_slot
    }
    pub(in crate::mir::normal_callable_semantic_package) fn callee_owner(
        &self,
    ) -> FunctionOwnerIdV1 {
        self.callee_owner
    }
    pub(in crate::mir::normal_callable_semantic_package) fn argument_sites(
        &self,
    ) -> &[SourceExprSiteV1] {
        &self.argument_sites
    }
    pub(in crate::mir::normal_callable_semantic_package) fn parameters(
        &self,
    ) -> &[super::super::model::OwnedCallableParameterContractV1] {
        &self.parameters
    }
    pub(in crate::mir::normal_callable_semantic_package) fn required_i64_arguments(
        &self,
    ) -> &[u32] {
        self.route.required_i64_arguments()
    }
}

pub(in crate::mir::normal_callable_semantic_package) struct StaticIncomingSourceLoanV1<'a> {
    catalog_brand: crate::mir::builder::SameModuleCallableCatalogBrandV1,
    caller: CanonicalSameModuleCallableKeyV1,
    contract: &'a OwnedCallableParameterContractDeclarationV1,
    call: &'a OwnedExprSiteV1,
    source: &'a VerifiedResolvedMethodCallSourceV1,
    route: StaticIncomingRouteV1,
    target: &'a CanonicalSameModuleCallableKeyV1,
}

impl<'a> StaticIncomingSourceLoanV1<'a> {
    pub(in crate::mir::normal_callable_semantic_package) fn contract(
        &self,
    ) -> &'a OwnedCallableParameterContractDeclarationV1 {
        self.contract
    }

    /// Corroborate a previously retained call against this same original loan.
    /// This never issues a second source record for an opaque argument sibling.
    pub(in crate::mir::normal_callable_semantic_package) fn corroborates_retained(
        &self,
        retained: &StaticIncomingSourceV1,
    ) -> bool {
        retained.catalog_brand.is_same(&self.catalog_brand)
            && retained.caller == self.caller
            && retained.call_site() == self.call
            && retained.target() == self.target
            && retained.target_batch_slot() == self.contract.batch_slot
            && retained.callee_owner() == self.contract.owner
            && retained.argument_sites().len() == self.source.arguments().len()
            && retained
                .argument_sites()
                .iter()
                .zip(self.source.arguments())
                .all(|(site, argument)| site == argument.site())
            && retained.parameters().len() == self.contract.parameters.len()
            && retained
                .parameters()
                .iter()
                .zip(&self.contract.parameters)
                .all(|(left, right)| {
                    left.ordinal == right.ordinal
                        && left.binding == right.binding
                        && left.kind == right.kind
                })
            && retained.route == self.route
    }

    pub(in crate::mir::normal_callable_semantic_package) fn retain(
        &self,
    ) -> Rc<StaticIncomingSourceV1> {
        Rc::new(StaticIncomingSourceV1 {
            catalog_brand: self.catalog_brand.clone(),
            caller: self.caller.clone(),
            call: self.call.clone(),
            target: self.target.clone(),
            target_batch_slot: self.contract.batch_slot,
            callee_owner: self.contract.owner,
            argument_sites: self
                .source
                .arguments()
                .iter()
                .map(|argument| argument.site().clone())
                .collect(),
            parameters: self
                .contract
                .parameters
                .iter()
                .map(
                    |formal| super::super::model::OwnedCallableParameterContractV1 {
                        ordinal: formal.ordinal,
                        binding: formal.binding,
                        kind: formal.kind.clone(),
                    },
                )
                .collect(),
            route: self.route.clone(),
        })
    }
}

impl QualifiedStaticCallClaimIndexV1 {
    /// The caller supplies the original resolved call from the existing source
    /// visit. This lends the same target row; it neither scans nor resolves it.
    pub(in crate::mir::normal_callable_semantic_package) fn incoming_source<'a>(
        &'a self,
        caller: &CanonicalSameModuleCallableKeyV1,
        site: &'a OwnedExprSiteV1,
        source: &'a VerifiedResolvedMethodCallSourceV1,
        selected: &super::super::selected_mapping::VerifiedSelectedCallableBatchMapV1,
        contracts: &'a [OwnedCallableParameterContractDeclarationV1],
        app_main: Option<&crate::mir::normal_callable_semantic_package::ordinary_new_coseal::BorrowedAppMainSourceLoanV1<'_>>,
    ) -> Result<Option<StaticIncomingSourceLoanV1<'a>>, String> {
        if !self.catalog_brand.is_same(selected.catalog_brand()) {
            return Err("[freeze:contract][borrowed-static/incoming-source-cohort]".into());
        }
        let (route, target) = if let Some((claim, target)) = self.claim_target(caller, site.site())
        {
            (StaticIncomingRouteV1::Qualified(claim.clone()), target)
        } else if let Some(row) = self.current_owner_source(caller, site.site()) {
            (
                StaticIncomingRouteV1::CurrentOwner(row.clone()),
                row.route().target(),
            )
        } else {
            return Ok(None);
        };
        let reject = || "[freeze:contract][borrowed-static/incoming-source-identity]".to_owned();
        let mut callers = contracts.iter().filter(|row| row.owner == site.owner());
        let caller_contract = callers.next().ok_or_else(reject)?;
        let main = app_main.filter(|main| main.matches_contract(caller_contract));
        if callers.next().is_some()
            || super::caller_key_for_function(
                selected,
                caller_contract.batch_slot,
                main.is_some(),
                main.map(|main| main.catalog_key()),
            )
            .as_ref()
                != Some(caller)
        {
            return Err(reject());
        }
        if let StaticIncomingRouteV1::CurrentOwner(row) = &route {
            if caller.namespace() != hakorune_mir_defs::SameModuleCallableNamespaceV1::StaticBoxMethod
                || caller_contract.mode != crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::StaticBoxMethod
                || caller.owner() != target.owner()
                || row.route().receiver() != crate::mir::source_call_target::CurrentOwnerStaticReceiverV1::CanonicalMe
            { return Err(reject()); }
        }
        let slot = selected
            .batch_slot(&SelectedNormalCallableKeyV1::Cataloged(target.clone()))
            .ok_or_else(reject)?;
        let mut matches = contracts.iter().filter(|row| row.batch_slot == slot);
        let contract = matches.next().ok_or_else(reject)?;
        if matches.next().is_some()
            || source.owner() != site.owner()
            || source.site() != site.site()
            || source.receiver() != route.receiver()
            || source.selector() != target.name()
            || source.arity() != target.arity()
            || source.arguments().len() != target.arity() as usize
            || target.namespace() != hakorune_mir_defs::SameModuleCallableNamespaceV1::StaticBoxMethod
            || contract.mode != crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::StaticBoxMethod
            || contract.parameters.len() != source.arguments().len()
            || !source.arguments().iter().zip(&contract.parameters).enumerate().all(
                |(ordinal, (argument, formal))| argument.ordinal() as usize == ordinal
                    && formal.ordinal as usize == ordinal
                    && formal.binding.owner() == contract.owner,
            )
            || route.required_i64_arguments().iter().any(|ordinal| *ordinal >= target.arity())
        {
            return Err(reject());
        }
        Ok(Some(StaticIncomingSourceLoanV1 {
            catalog_brand: self.catalog_brand.clone(),
            caller: caller.clone(),
            contract,
            call: site,
            source,
            route,
            target,
        }))
    }
}
