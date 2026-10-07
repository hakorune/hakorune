//! Compose original opaque argument uses with the sole qualified-static index.
//! This product proves source identity only, never integer view or transport.
use super::borrowed_formal_uses::{BorrowedFormalUseDraftKindV1, BorrowedFormalUsesDraftV1};
use super::*;
use crate::mir::builder::SelectedNormalCallableKeyV1;
use crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1;
use crate::mir::normal_callable_semantic_package::qualified_static_call_claim::{
    caller_key_for_function, incoming_source::QualifiedStaticIncomingSourceV1,
    QualifiedStaticCallClaimIndexV1,
};
use std::collections::BTreeMap;

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package) struct QualifiedStaticArgumentSourceV1 {
    source: std::rc::Rc<QualifiedStaticIncomingSourceV1>,
    ordinal: u32,
    use_site: OwnedExprSiteV1,
    binding: BindingRefV1,
    formal: BindingRefV1,
}

impl QualifiedStaticArgumentSourceV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn call(&self) -> &OwnedExprSiteV1 {
        self.source.call_site()
    }
    pub(in crate::mir::normal_callable_semantic_package) fn call_source(
        &self,
    ) -> &QualifiedStaticIncomingSourceV1 {
        &self.source
    }
    pub(in crate::mir::normal_callable_semantic_package) fn retained_call_source(
        &self,
    ) -> &std::rc::Rc<QualifiedStaticIncomingSourceV1> {
        &self.source
    }
    pub(in crate::mir::normal_callable_semantic_package) fn ordinal(&self) -> u32 {
        self.ordinal
    }
    pub(in crate::mir::normal_callable_semantic_package) fn use_site(&self) -> &OwnedExprSiteV1 {
        &self.use_site
    }
    pub(in crate::mir::normal_callable_semantic_package) fn binding(&self) -> BindingRefV1 {
        self.binding
    }
    pub(in crate::mir::normal_callable_semantic_package) fn formal(&self) -> BindingRefV1 {
        self.formal
    }
    pub(in crate::mir::normal_callable_semantic_package) fn target(
        &self,
    ) -> &CanonicalSameModuleCallableKeyV1 {
        self.source.target()
    }
    pub(in crate::mir::normal_callable_semantic_package) fn target_formal(&self) -> BindingRefV1 {
        self.source.parameters()[self.ordinal as usize].binding
    }
    pub(in crate::mir::normal_callable_semantic_package) fn required_i64_arguments(
        &self,
    ) -> &[u32] {
        self.source.required_i64_arguments()
    }
}

pub(super) fn collect_static_argument_sources_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    definitions: &BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1>,
    claims: Option<&QualifiedStaticCallClaimIndexV1>,
    app_main: Option<&super::BorrowedAppMainSourceLoanV1<'_>>,
) -> Result<BTreeMap<(OwnedExprSiteV1, u32), QualifiedStaticArgumentSourceV1>, String> {
    let mut result = BTreeMap::new();
    let Some(claims) = claims else {
        return Ok(result);
    };
    let mut call_sources = BTreeMap::new();
    for (owner, draft) in definitions {
        let mut caller_contracts = contracts.iter().filter(|row| row.owner == *owner);
        let contract = caller_contracts
            .next()
            .ok_or_else(|| freeze("borrowed-static/caller-contract"))?;
        if caller_contracts.next().is_some() {
            return Err(freeze("borrowed-static/duplicate-caller-contract"));
        }
        let caller_key = caller_key_for_function(selected, contract.batch_slot, false, None)
            .ok_or_else(|| freeze("borrowed-static/caller-key"))?;
        batch
            .with_lowering_input(contract.batch_slot, |input| {
                if input.owner() != *owner {
                    return Err(freeze("borrowed-static/caller-owner"));
                }
                let mut source_calls = BTreeMap::new();
                for (site, source) in input.function().method_calls() {
                    if source_calls.insert(site.clone(), source).is_some() {
                        return Err(freeze("borrowed-static/duplicate-call-source"));
                    }
                }
                for row in &draft.uses {
                    let BorrowedFormalUseDraftKindV1::UnresolvedArgument { call, ordinal } =
                        &row.kind
                    else {
                        continue;
                    };
                    let source = source_calls
                        .get(call.site())
                        .copied()
                        .ok_or_else(|| freeze("borrowed-static/call-source"))?;
                    let shared_source = if let Some(shared) = call_sources.get(call) {
                        std::rc::Rc::clone(shared)
                    } else {
                        let Some(incoming) = claims.incoming_source(
                            &caller_key,
                            call,
                            source,
                            selected,
                            contracts,
                            app_main,
                        )?
                        else {
                            continue;
                        };
                        let shared = incoming.retain();
                        call_sources.insert(call.clone(), std::rc::Rc::clone(&shared));
                        shared
                    };
                    let argument = source
                        .arguments()
                        .get(*ordinal as usize)
                        .ok_or_else(|| freeze("borrowed-static/argument-ordinal"))?;
                    if call.owner() != *owner
                        || source.owner() != *owner
                        || argument.ordinal() != *ordinal
                        || argument.site() != row.site.site()
                        || row.site.owner() != *owner
                        || row.binding.owner() != *owner
                        || input.function().variable_ref(argument.site())
                            != Some(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(
                                row.binding,
                            ))
                        || draft.origins.get(&row.binding) != Some(&row.formal)
                        || !contract.parameters.iter().any(|formal| {
                            formal.binding == row.formal
                                && formal.kind.is_ordinary_borrowed_handle()
                        })
                    {
                        return Err(freeze("borrowed-static/source-identity"));
                    }
                    let fact = QualifiedStaticArgumentSourceV1 {
                        source: shared_source,
                        ordinal: *ordinal,
                        use_site: row.site.clone(),
                        binding: row.binding,
                        formal: row.formal,
                    };
                    if result.insert((call.clone(), *ordinal), fact).is_some() {
                        return Err(freeze("borrowed-static/duplicate-argument"));
                    }
                }
                Ok(())
            })
            .map_err(|_| freeze("borrowed-static/source-loan"))??;
    }
    Ok(result)
}

#[cfg(test)]
#[path = "ordinary_new_borrowed_static_retention_tests.rs"]
mod retention_tests;
