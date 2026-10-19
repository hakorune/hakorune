//! Borrow the original static target row together with its complete input contract.
//! Result-required ordinals never classify a parameter or erase its carrier tag.
use super::*;
use crate::mir::callable_result_representation::{
    CallableResultEvidenceV1, OwnedCallableResultCallSiteV1,
};
use crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1;
use crate::mir::resolved_semantics::{
    FunctionOwnerIdV1, OwnedExprSiteV1, ResolvedMethodCallReceiverSourceV1,
    VerifiedResolvedMethodCallSourceV1,
};
use std::rc::Rc;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
enum StaticIncomingRouteV1 {
    Qualified {
        route: crate::mir::source_call_target::VerifiedQualifiedStaticCallTargetV1,
        result: VerifiedCallableResultDispositionV1,
    },
    CurrentOwner(super::CurrentOwnerStaticCallSourceV1),
}

impl StaticIncomingRouteV1 {
    fn result(&self) -> &VerifiedCallableResultDispositionV1 {
        match self {
            Self::Qualified { result, .. } => result,
            Self::CurrentOwner(row) => row.result(),
        }
    }
    fn required_i64_arguments(&self) -> &[u32] {
        self.result().required_i64_arguments().unwrap_or(&[])
    }
    fn receiver(&self) -> ResolvedMethodCallReceiverSourceV1 {
        match self {
            Self::Qualified { .. } => ResolvedMethodCallReceiverSourceV1::QualifiedUnbound,
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
    method_source: Arc<VerifiedResolvedMethodCallSourceV1>,
    call_result: Option<Arc<OwnedCallableResultCallSiteV1>>,
}

impl StaticIncomingSourceV1 {
    pub(in crate::mir) fn same_catalog_as(&self, other: &Self) -> bool {
        self.catalog_brand.is_same(&other.catalog_brand) && self.caller == other.caller
    }

    pub(in crate::mir::normal_callable_semantic_package) fn catalog_brand(
        &self,
    ) -> &crate::mir::builder::SameModuleCallableCatalogBrandV1 {
        &self.catalog_brand
    }

    /// The original qualified I64 lane, not general route membership.
    pub(in crate::mir) fn is_qualified_i64_source_v1(&self) -> bool {
        matches!(self.route, StaticIncomingRouteV1::Qualified { .. })
            && matches!(
                self.route.result(),
                VerifiedCallableResultDispositionV1::ExactI64 { .. }
            )
    }
    /// Exact source-only zero-input I64 law; no entry or packet permission.
    pub(in crate::mir::normal_callable_semantic_package) fn is_zeroarg_i64_v1(&self) -> bool {
        self.target.arity() == 0
            && self.argument_sites.is_empty()
            && self.parameters.is_empty()
            && matches!(self.route.result(),
                VerifiedCallableResultDispositionV1::ExactI64 { required_i64_arguments }
                    if required_i64_arguments.is_empty())
    }
    pub(in crate::mir::normal_callable_semantic_package) fn source_result(
        &self,
    ) -> &VerifiedCallableResultDispositionV1 {
        self.route.result()
    }
    pub(in crate::mir) fn method_source(
        &self,
    ) -> &Arc<VerifiedResolvedMethodCallSourceV1> {
        &self.method_source
    }
    pub(in crate::mir::normal_callable_semantic_package) fn call_result_evidence(
        &self,
    ) -> Option<&OwnedCallableResultCallSiteV1> {
        self.call_result.as_deref()
    }

    /// Original CurrentOwner result and full input tuple, for source-only
    /// actual observation. This grants neither executable entry nor ABI.
    pub(in crate::mir::normal_callable_semantic_package) fn is_current_owner_i64_source_v1(
        &self,
    ) -> bool {
        matches!(
            self.current_owner_source().map(|row| row.result()),
            Some(VerifiedCallableResultDispositionV1::ExactI64 { .. })
        ) && self.argument_sites.len() == self.parameters.len()
    }

    /// Original Bool result and input tuple. This is source identity only;
    /// it grants neither an entry carrier nor an executable Bool call.
    pub(in crate::mir::normal_callable_semantic_package) fn is_current_owner_bool_source_v1(
        &self,
    ) -> bool {
        matches!(
            self.current_owner_source().map(|row| row.result()),
            Some(VerifiedCallableResultDispositionV1::ExactBool)
        ) && self.argument_sites.len() == self.parameters.len()
    }

    pub(in crate::mir) fn require_qualified_i64_source_v1(&self) -> Result<(), String> {
        if self.is_qualified_i64_source_v1() {
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
            StaticIncomingRouteV1::Qualified { .. } => None,
        }
    }

    pub(in crate::mir) fn caller(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.caller
    }

    /// Same catalog/caller/site/result handoff; this grants no entry or physical ABI.
    pub(in crate::mir::normal_callable_semantic_package) fn corroborates_publication_handoff(
        &self,
        handoff: &crate::mir::callable_result_representation::VerifiedStaticCallResultPublicationHandoffV1,
    ) -> bool {
        (self.is_qualified_i64_source_v1() || self.is_zeroarg_i64_v1())
            && self.matches_publication_handoff_identity(handoff)
    }

    /// The selected Loop packet supplies the separate CurrentOwner source,
    /// actual and result proof before this source-only result corroboration.
    pub(in crate::mir::normal_callable_semantic_package) fn corroborates_selected_loop_publication_handoff(
        &self,
        handoff: &crate::mir::callable_result_representation::VerifiedStaticCallResultPublicationHandoffV1,
    ) -> bool {
        self.is_current_owner_i64_source_v1()
            && !self.is_qualified_i64_source_v1()
            && self.argument_sites.len() == 1
            && self.matches_publication_handoff_identity(handoff)
    }

    pub(in crate::mir::normal_callable_semantic_package) fn matches_publication_handoff_identity(
        &self,
        handoff: &crate::mir::callable_result_representation::VerifiedStaticCallResultPublicationHandoffV1,
    ) -> bool {
        handoff.catalog_identity() == self.catalog_brand.identity()
            && handoff.caller() == &self.caller
            && handoff.site() == self.call.site()
            && handoff.target() == &self.target
            && handoff.representation() == &crate::mir::callable_result_representation::VerifiedCallableResultRepresentationV1::ExactI64
            && handoff.required_callee_i64_arguments() == self.required_i64_arguments()
    }
    pub(in crate::mir) fn call_site(&self) -> &OwnedExprSiteV1 {
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
    pub(in crate::mir) fn argument_sites(
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
    call_result: Option<&'a Arc<OwnedCallableResultCallSiteV1>>,
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
            && std::ptr::eq(retained.method_source.as_ref(), self.source)
            && match (&retained.call_result, self.call_result) {
                (None, None) => true,
                (Some(retained), Some(original)) => Arc::ptr_eq(retained, original),
                _ => false,
            }
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
        input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    ) -> Result<Rc<StaticIncomingSourceV1>, String> {
        let method_source = input
            .function()
            .method_call_source_v1(self.call.site())
            .ok_or("[freeze:contract][borrowed-static/original-method-source]")?;
        if input.owner() != self.call.owner() || !std::ptr::eq(method_source.as_ref(), self.source)
        {
            return Err("[freeze:contract][borrowed-static/original-method-source]".into());
        }
        Ok(Rc::new(StaticIncomingSourceV1 {
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
            method_source: Arc::clone(method_source),
            call_result: self.call_result.cloned(),
        }))
    }
}

/// Borrowed caller provenance for the same static source lender.
/// Declaration keys retain their existing contract check; Birth needs its row.
pub(in crate::mir::normal_callable_semantic_package) enum StaticIncomingCallerSourceV1<'a> {
    Declaration(&'a CanonicalSameModuleCallableKeyV1),
    Constructor(super::super::original_call_source::OriginalCallSourceCallerV1<'a>),
}
impl<'a> From<&'a CanonicalSameModuleCallableKeyV1> for StaticIncomingCallerSourceV1<'a> {
    fn from(key: &'a CanonicalSameModuleCallableKeyV1) -> Self {
        Self::Declaration(key)
    }
}
impl<'a> From<super::super::original_call_source::OriginalCallSourceCallerV1<'a>>
    for StaticIncomingCallerSourceV1<'a>
{
    fn from(caller: super::super::original_call_source::OriginalCallSourceCallerV1<'a>) -> Self {
        Self::Constructor(caller)
    }
}

impl QualifiedStaticCallClaimIndexV1 {
    /// The caller supplies the original resolved call from the existing source
    /// visit. This lends the same target row; it neither scans nor resolves it.
    pub(in crate::mir::normal_callable_semantic_package) fn incoming_source<'a, 'caller>(
        &'a self,
        caller_source: impl Into<StaticIncomingCallerSourceV1<'caller>>,
        site: &'a OwnedExprSiteV1,
        source: &'a VerifiedResolvedMethodCallSourceV1,
        selected: &super::super::selected_mapping::VerifiedSelectedCallableBatchMapV1,
        contracts: &'a [OwnedCallableParameterContractDeclarationV1],
        app_main: Option<&crate::mir::normal_callable_semantic_package::ordinary_new_coseal::BorrowedAppMainSourceLoanV1<'_>>,
    ) -> Result<Option<StaticIncomingSourceLoanV1<'a>>, String> {
        self.incoming_source_in_census(
            caller_source,
            site,
            source,
            selected,
            contracts,
            &std::collections::BTreeMap::new(),
            app_main,
        )
    }

    pub(in crate::mir::normal_callable_semantic_package) fn incoming_source_in_census<
        'a,
        'caller,
    >(
        &'a self,
        caller_source: impl Into<StaticIncomingCallerSourceV1<'caller>>,
        site: &'a OwnedExprSiteV1,
        source: &'a VerifiedResolvedMethodCallSourceV1,
        selected: &super::super::selected_mapping::VerifiedSelectedCallableBatchMapV1,
        contracts: &'a [OwnedCallableParameterContractDeclarationV1],
        omitted: &std::collections::BTreeMap<FunctionOwnerIdV1, CanonicalSameModuleCallableKeyV1>,
        app_main: Option<&crate::mir::normal_callable_semantic_package::ordinary_new_coseal::BorrowedAppMainSourceLoanV1<'_>>,
    ) -> Result<Option<StaticIncomingSourceLoanV1<'a>>, String> {
        if !self.catalog_brand.is_same(selected.catalog_brand()) {
            return Err("[freeze:contract][borrowed-static/incoming-source-cohort]".into());
        }
        let caller_source = caller_source.into();
        let caller = match caller_source {
            StaticIncomingCallerSourceV1::Declaration(key) => key.clone(),
            StaticIncomingCallerSourceV1::Constructor(original) => original
                .constructor_key_for_call(site, source)?
                .ok_or("[freeze:contract][borrowed-static/constructor-caller-required]")?
                .clone(),
        };
        let Some(original_target) = self.source_target(&caller, site.site()) else {
            return Ok(None);
        };
        let target = original_target.target();
        let route = match original_target {
            VerifiedSourceStaticCallTargetV1::QualifiedStatic(route) => {
                StaticIncomingRouteV1::Qualified {
                    route: route.clone(),
                    result: self
                        .result_for_key(target)
                        .ok_or("[freeze:contract][borrowed-static/source-result-missing]")?
                        .clone(),
                }
            }
            VerifiedSourceStaticCallTargetV1::CurrentOwnerStatic(_) => {
                let row = self
                    .current_owner_source(&caller, site.site())
                    .ok_or("[freeze:contract][borrowed-static/current-owner-source-missing]")?;
                StaticIncomingRouteV1::CurrentOwner(row.clone())
            }
        };
        let call_result = self
            .call_result_rows
            .get(&(caller.clone(), site.site().clone()));
        if call_result.is_some_and(|row| !matches!(row.evidence(),
            CallableResultEvidenceV1::SameModuleStatic { source_target, .. } if source_target == original_target)) {
            return Err("[freeze:contract][borrowed-static/call-result-source-drift]".into());
        }
        let reject = || "[freeze:contract][borrowed-static/incoming-source-identity]".to_owned();
        let caller_contract =
            if matches!(caller_source, StaticIncomingCallerSourceV1::Declaration(_)) {
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
                    .or_else(|| omitted.get(&site.owner()).cloned())
                    .as_ref()
                        != Some(&caller)
                {
                    return Err(reject());
                }
                Some(caller_contract)
            } else {
                None
            };
        if let StaticIncomingRouteV1::CurrentOwner(row) = &route {
            if caller.namespace() != hakorune_mir_defs::SameModuleCallableNamespaceV1::StaticBoxMethod
                || caller_contract.is_none_or(|contract| contract.mode != crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::StaticBoxMethod)
                || caller.owner() != target.owner()
                || row.route().receiver() != crate::mir::source_call_target::CurrentOwnerStaticReceiverV1::CanonicalMe
            { return Err(reject()); }
        }
        if let StaticIncomingRouteV1::Qualified { route, .. } = &route {
            let expected = match route.receiver() {
                crate::mir::source_call_target::QualifiedStaticReceiverV1::ImportedAlias { source_alias, .. } => source_alias.as_ref(),
                crate::mir::source_call_target::QualifiedStaticReceiverV1::UnshadowedCanonicalOwner { canonical_owner } => canonical_owner.as_ref(),
            };
            if source
                .qualified_receiver_identity()
                .map(|identity| identity.source_name())
                != Some(expected)
            {
                return Err(reject());
            }
        }
        let slot =
            match selected.batch_slot(&SelectedNormalCallableKeyV1::Cataloged(target.clone())) {
                Some(slot) => slot,
                None => {
                    let mut owners = omitted.iter().filter(|(_, key)| *key == target);
                    let (owner, _) = owners.next().ok_or_else(reject)?;
                    let mut rows = contracts.iter().filter(|row| row.owner == *owner);
                    let row = rows.next().ok_or_else(reject)?;
                    if owners.next().is_some() || rows.next().is_some() {
                        return Err(reject());
                    }
                    row.batch_slot
                }
            };
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
            call_result,
        }))
    }
}
