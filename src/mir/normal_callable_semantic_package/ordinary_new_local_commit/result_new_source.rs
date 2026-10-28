//! Borrow exact original New and nullable acquisition source across affine take.
//! This lends retained source fields, never physical completion permission.
use super::*;

pub(in crate::mir::normal_callable_semantic_package) struct ResultNewSourceRefV1<'a> {
    pub(in crate::mir::normal_callable_semantic_package) site: &'a OwnedExprSiteV1,
    pub(in crate::mir::normal_callable_semantic_package) class: &'a str,
    pub(in crate::mir::normal_callable_semantic_package) arity: usize,
    pub(in crate::mir::normal_callable_semantic_package) object: CanonicalObjectIdV1,
    pub(in crate::mir::normal_callable_semantic_package) destruction:
        super::super::ObjectDestructionDispositionV1,
    pub(in crate::mir::normal_callable_semantic_package) construction:
        &'a super::super::ConstructionEligibilityV1,
    pub(in crate::mir::normal_callable_semantic_package) prefix:
        &'a Result<ResultNewHomePrefixV1, HomePrefixUnavailableV1>,
    pub(in crate::mir::normal_callable_semantic_package) arguments: &'a Result<
        Box<[super::super::OrdinaryNewTrivialArgumentV1]>,
        crate::mir::resolved_semantics::home_new_prefix::SelectedNewArgumentUnavailableV1,
    >,
    pub(in crate::mir::normal_callable_semantic_package) children:
        Option<&'a [super::super::OwnedFieldChildV1]>,
}

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn with_result_new_source_v1<T>(
        &self,
        site: &OwnedExprSiteV1,
        read: impl FnOnce(ResultNewSourceRefV1<'_>) -> Result<T, String>,
    ) -> Result<Option<T>, String> {
        let claims = self.result_claims.borrow();
        let commits = self.local_commits.borrow();
        let source = match (claims.get(site), commits.get(site)) {
            (Some(_), Some(_)) => return Err(freeze("result-source/duplicate-store")),
            (Some(claim), None) => {
                if claim.site() != site || claim.class() != claim.box_source().name() {
                    return Err(freeze("result-source/claim-identity"));
                }
                ResultNewSourceRefV1 {
                    site: claim.site(),
                    class: claim.class(),
                    arity: claim.arity(),
                    object: claim.object(),
                    destruction: claim.core.destruction(),
                    construction: claim.construction(),
                    prefix: &claim.home_prefix,
                    arguments: &claim.core.argument_rows,
                    children: claim.core.children(),
                }
            }
            (None, Some(LocalCommitV1::Result(row))) => {
                if &row.site != site {
                    return Err(freeze("result-source/commit-identity"));
                }
                ResultNewSourceRefV1 {
                    site: &row.site,
                    class: row.box_source.name(),
                    arity: row.arity,
                    object: row.object,
                    destruction: row.destruction,
                    construction: &row.construction,
                    prefix: &row.home_prefix,
                    arguments: &row.argument_rows,
                    children: row.children.as_deref(),
                }
            }
            (None, Some(_)) => return Err(freeze("result-source/commit-kind")),
            (None, None) => return Ok(None),
        };
        read(source).map(Some)
    }
}

/// Borrowed source view: allocation semantics and execution eligibility are distinct.
pub(in crate::mir::normal_callable_semantic_package) struct LocalNewSourceRefV1<'a> {
    pub(in crate::mir::normal_callable_semantic_package) destruction: super::super::ObjectDestructionDispositionV1,
    pub(in crate::mir::normal_callable_semantic_package) prefix: &'a Result<CallerNewHomePrefixV1, HomePrefixUnavailableV1>,
    pub(in crate::mir::normal_callable_semantic_package) arguments: &'a Result<Box<[super::super::OrdinaryNewTrivialArgumentV1]>, crate::mir::resolved_semantics::home_new_prefix::SelectedNewArgumentUnavailableV1>,
    pub(in crate::mir::normal_callable_semantic_package) children: Option<&'a [super::super::OwnedFieldChildV1]>,
    pub(in crate::mir::normal_callable_semantic_package) box_source: &'a crate::parser::ParserOrdinaryBoxSourceRowV1,
    pub(in crate::mir::normal_callable_semantic_package) object: CanonicalObjectIdV1,
    pub(in crate::mir::normal_callable_semantic_package) construction: &'a super::super::ConstructionEligibilityV1,
    pub(in crate::mir::normal_callable_semantic_package) construction_source: &'a super::super::ConstructionSourceEligibilityV1,
    pub(in crate::mir::normal_callable_semantic_package) allocation_execution: Option<&'a super::super::AllocationOnlyNewExecutionV1>,
    pub(in crate::mir::normal_callable_semantic_package) declaration: &'a SourceBindingSiteV1,
}

impl OrdinaryNewClaimLedgerV1 {
    /// Borrow the SAME original local New row across its affine move.
    /// Construction eligibility and physical/Home availability remain separate.
    pub(in crate::mir::normal_callable_semantic_package) fn with_local_new_source_v1<T>(
        &self,
        acquisition: &OwnedExprSiteV1,
        root: BindingRefV1,
        read: impl FnOnce(LocalNewSourceRefV1<'_>) -> Result<T, String>,
    ) -> Result<Option<T>, String> {
        if root.owner() != acquisition.owner() {
            return Err(freeze("home-source/owner"));
        }
        let claims = self.claims.borrow();
        let commits = self.local_commits.borrow();
        let (binding, box_source, object, construction, construction_source, allocation_execution, declaration,
            destruction, prefix, arguments, children) =
            match (claims.get(acquisition), commits.get(acquisition)) {
                (Some(_), Some(_)) => return Err(freeze("home-source/duplicate-store")),
                (Some(claim), None) => {
                    if &claim.core.site != acquisition
                        || claim.core.class.as_ref() != claim.core.box_source.name()
                    {
                        return Err(freeze("home-source/claim-identity"));
                    }
                    (
                        claim.destination,
                        &claim.core.box_source,
                        claim.core.object,
                        &claim.core.construction,
                        &claim.core.construction_source,
                        claim.core.allocation_execution.as_ref(),
                        &claim.declaration,
                        claim.core.destruction(), &claim.home_prefix, &claim.core.argument_rows, claim.core.children(),
                    )
                }
                (None, Some(LocalCommitV1::Ordinary(row))) => (
                    row.binding,
                    &row.box_source,
                    row.object,
                    &row.construction,
                    &row.construction_source,
                    row.allocation_execution.as_ref(),
                    &row.declaration,
                    row.destruction, &row.home_prefix, &row.argument_rows, row.children.as_deref(),
                ),
                (None, Some(_)) => return Err(freeze("home-source/commit-kind")),
                (None, None) => return Ok(None),
            };
        if binding != root {
            return Err(freeze("home-source/domain"));
        }
        read(LocalNewSourceRefV1 { box_source, object, construction, construction_source, allocation_execution, declaration,
            destruction, prefix, arguments, children }).map(Some)
    }

    /// Executable actual classification preserves its stronger construction gate.
    pub(in crate::mir::normal_callable_semantic_package) fn borrowed_local_new_actual_class_v1(
        &self, acquisition: &OwnedExprSiteV1, root: BindingRefV1,
    ) -> Result<Option<Box<str>>, String> {
        self.with_local_new_source_v1(acquisition, root, |source| {
            if !matches!(source.construction, Ok(plan) if plan.object() == source.object) { return Err(freeze("home-source/domain")); }
            Ok(source.box_source.name().into())
        })
    }

    /// Nullable class comes from this exact original local acquisition, not
    /// another same-class construction or an emitted object's reverse lookup.
    pub(in crate::mir::normal_callable_semantic_package) fn borrowed_nullable_actual_class_v1(
        &self,
        binding: BindingRefV1,
    ) -> Result<Option<Box<str>>, String> {
        let Some(call) = self.received_actual_observation_v1(binding, LocalCallResultClassV1::Nullable)?
        else { return Ok(None); };
        let Some(ingress) = self.borrowed_formal_source.as_ref() else {
            return Ok(None);
        };
        let ingress = ingress.as_ref().map_err(Clone::clone)?;
        let Some(target) = ingress.object_source_target_at_v1(call.site())? else {
            return Ok(None);
        };
        if target.call_site() != call.site() || target.call_site().owner() != binding.owner() {
            return Err(freeze("nullable-source/target-identity"));
        }
        if target.is_self_receiver() {
            let Some(receiver) = self.receiver_call_observation(call.site()) else {
                return Ok(None);
            };
            if receiver.destination() != binding
                || receiver.callee() != target.target()
                || self.callable_result_classes.get(target.target()) != Some(receiver.class())
            {
                return Err(freeze("nullable-source/receiver-identity"));
            }
        }
        let Some(class) = self.nullable_result_class(target.target()) else {
            return Err(freeze("nullable-source/result-class"));
        };
        Ok(Some(class.into()))
    }

    fn received_actual_observation_v1(
        &self, binding: BindingRefV1, expected: LocalCallResultClassV1,
    ) -> Result<Option<&crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1>, String> {
        // The original AppMain Completion can live only in root_completion.
        // A retained indexed refusal is authoritative: never retry the root.
        let completion = match self.completion_index.get(&binding.owner()) {
            Some(row) => row,
            None => match self.root_completion.as_ref() {
                Some(row @ Ok(root)) if root.owner() == binding.owner() => row,
                Some(Err(issue)) => {
                    return Err(format!(
                        "{}: {issue:?}",
                        freeze("nullable-source/completion")
                    ))
                }
                _ => return Ok(None),
            },
        };
        let completion = completion
            .as_ref()
            .map_err(|issue| format!("{}: {issue:?}", freeze("nullable-source/completion")))?;
        if completion.owner() != binding.owner() {
            return Err(freeze("nullable-source/owner"));
        }
        let Some(flow) = completion.cleanup().root_flow() else {
            return Ok(None);
        };
        let mut calls = flow.local_calls().iter().filter(|call| {
            call.local_binding()
                .is_some_and(|(_, destination)| destination == binding)
        });
        let Some(call) = calls.next() else {
            return Ok(None);
        };
        let additional_calls = calls.count();
        if additional_calls != 0
            || call.owner() != binding.owner()
            || call.result() != expected
            || !matches!(call.local_binding(), Some((SourceBindingSiteV1::Local { .. }, destination)) if destination == binding)
        {
            return Err(format!(
                "{} binding={binding:?} additional_calls={additional_calls} call_owner={:?} site={:?} result={:?} destination={:?}",
                freeze("nullable-source/acquisition"), call.owner(), call.site(),
                call.result(), call.local_binding(),
            ));
        }
        Ok(Some(call))
    }

    pub(in crate::mir::normal_callable_semantic_package) fn borrowed_mixed_actual_class_v1(
        &self, binding: BindingRefV1,
    ) -> Result<Option<Box<str>>, String> {
        let Some(call) = self.received_actual_observation_v1(binding, LocalCallResultClassV1::MixedObject)?
        else { return Ok(None); };
        let Some(source) = self.borrowed_formal_source.as_ref() else { return Ok(None); };
        let source = source.as_ref().map_err(Clone::clone)?;
        let Some(loan) = source.mixed_received_actual_source_v1(binding)? else { return Ok(None); };
        if call.site() != loan.call() || call.mixed() != Some(loan)
            || call.local_binding() != Some((loan.declaration(), loan.destination())) {
            return Err(freeze("mixed-actual/original-observation"));
        }
        self.callable_result_classes.recheck_mixed_local_roots_v1(loan).map_err(freeze)?;
        Ok(Some(loan.fresh_class().into()))
    }

}
