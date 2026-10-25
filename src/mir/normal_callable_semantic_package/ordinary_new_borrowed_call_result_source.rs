//! Original ordinary result sources, shared by expression and Call owners.
//! Source qualification grants no executable input, result ABI or Home.
use super::super::borrowed_formal_uses::BorrowedIncomingSourceV1;
use super::static_source::{
    ordinary_text_argument_source_v1, validate_qualified_argument_sources_v1,
    validate_static_source_actuals_v1,
};
use super::*;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::normal_callable_semantic_package::result_contract::CompletedTextResultSourceV1;
use crate::mir::normal_callable_semantic_package::StaticIncomingSourceV1;
use crate::mir::resolved_semantics::home_new_prefix::{
    BorrowedCallArgumentsV1, LocalCallResultClassV1, LocalCallResultSourceV1,
};
use std::rc::Rc;

/// Original ordinary call syntax only. Instance result meaning is supplied
/// later by completed_instance_text_source_v1, never by an opaque destination.
#[derive(Debug, Clone)]
pub(super) struct OrdinaryArgumentCallSourceV1 {
    source: BorrowedIncomingSourceV1,
    method: std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1>,
}
impl PartialEq for OrdinaryArgumentCallSourceV1 {
    fn eq(&self, other: &Self) -> bool {
        self.source.same_source(&other.source) && std::sync::Arc::ptr_eq(&self.method, &other.method)
    }
}
impl Eq for OrdinaryArgumentCallSourceV1 {}
impl OrdinaryArgumentCallSourceV1 {
    pub(super) fn call_site(&self) -> &OwnedExprSiteV1 { self.source.call_site() }
    pub(super) fn is_instance(&self) -> bool { matches!(self.source, BorrowedIncomingSourceV1::Instance(_)) }
    pub(super) fn method_source(&self) -> &std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1> { &self.method }
    pub(super) fn validate_input_v1(
        &self, input: ResolvedFunctionLoweringInputV1<'_>,
        prepared: &PreparedBorrowedFormalIngressV1,
        contracts: Option<&[OwnedCallableParameterContractDeclarationV1]>,
    ) -> Result<(), String> {
        let reject = || freeze("borrowed-actual/text-call-source-identity");
        if input.owner() != self.call_site().owner() || !input.function()
            .method_call_source_v1(self.call_site().site()).is_some_and(|method| std::sync::Arc::ptr_eq(method, &self.method)) {
            return Err(reject());
        }
        if let BorrowedIncomingSourceV1::Instance(target) = &self.source {
            let original = crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1::original_instance_input_call_v1(
                prepared, contracts.ok_or_else(reject)?, self.call_site())?.ok_or_else(reject)?;
            if !original.source.same_source(&self.source) || !target.matches_receiver(input, &self.method) {
                return Err(reject());
            }
        }
        Ok(())
    }
}
/// Preserve the Static Text contract while lending a strict Instance source
/// without classifying its result. Stored targets keep their separate owner.
pub(super) fn original_argument_call_source_v1(
    prepared: &PreparedBorrowedFormalIngressV1, site: &OwnedExprSiteV1,
) -> Result<Option<OrdinaryArgumentCallSourceV1>, String> {
    if let Some(source) = original_static_source_v1(prepared, site)? {
        return Ok(ordinary_text_argument_source_v1(prepared, site, source.method_source())?
            .map(|source| OrdinaryArgumentCallSourceV1 {
                method: std::sync::Arc::clone(source.method_source()),
                source: BorrowedIncomingSourceV1::Static(source),
            }));
    }
    let reject = || freeze("borrowed-actual/text-call-source-identity");
    let mut rows = prepared.source_incoming.source_rows_with_method_v1().filter(|(row, _)| &row.call == site);
    let Some((row, method)) = rows.next() else { return Ok(None) };
    if rows.next().is_some() { return Err(reject()); }
    let BorrowedIncomingSourceV1::Instance(target) = &row.source else { return Ok(None) };
    let method = method.ok_or_else(reject)?;
    if method.owner() != site.owner() || method.site() != site.site()
        || target.call_site() != site || row.callee != target.callee_owner()
        || target.receiver_site() != method.receiver_site()
        || target.target().namespace() != hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod
        || target.target().name() != method.selector()
        || target.target().arity() != method.arity()
        || target.argument_sites().len() != method.arity() as usize
        || target.argument_sites().iter().ne(method.arguments().iter().map(|arg| arg.site())) {
        return Err(reject());
    }
    Ok(Some(OrdinaryArgumentCallSourceV1 { source: row.source.clone(), method: std::sync::Arc::clone(method) }))
}
pub(super) fn ordinary_argument_call_source_v1(
    prepared: &PreparedBorrowedFormalIngressV1, site: &OwnedExprSiteV1,
    method: &std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1>,
) -> Result<Option<OrdinaryArgumentCallSourceV1>, String> {
    let source = original_argument_call_source_v1(prepared, site)?;
    if source.as_ref().is_some_and(|source| !std::sync::Arc::ptr_eq(source.method_source(), method)) {
        return Err(freeze("borrowed-actual/text-call-source-identity"));
    }
    Ok(source)
}

#[derive(Debug, Clone)]
enum OrdinaryResultOriginV1 {
    Static(Rc<StaticIncomingSourceV1>),
    Instance {
        source: LexicalInstanceCallSourceTargetV1,
        text: CompletedTextResultSourceV1,
    },
}
impl PartialEq for OrdinaryResultOriginV1 {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Static(a), Self::Static(b)) => Rc::ptr_eq(a, b),
            (
                Self::Instance {
                    source: a,
                    text: at,
                },
                Self::Instance {
                    source: b,
                    text: bt,
                },
            ) => a == b && at == bt,
            _ => false,
        }
    }
}
impl Eq for OrdinaryResultOriginV1 {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OrdinaryResultSourceLoanV1 {
    source: OrdinaryResultOriginV1,
    result: LocalCallResultClassV1,
}
impl OrdinaryResultSourceLoanV1 {
    pub(crate) fn result(&self) -> LocalCallResultClassV1 {
        self.result
    }
    pub(crate) fn static_source(&self) -> Option<&Rc<StaticIncomingSourceV1>> {
        match &self.source {
            OrdinaryResultOriginV1::Static(source) => Some(source),
            _ => None,
        }
    }
    pub(crate) fn instance_source(&self) -> Option<&LexicalInstanceCallSourceTargetV1> {
        match &self.source {
            OrdinaryResultOriginV1::Instance { source, .. } => Some(source),
            _ => None,
        }
    }
    pub(crate) fn call_site(&self) -> &OwnedExprSiteV1 {
        match &self.source {
            OrdinaryResultOriginV1::Static(source) => source.call_site(),
            OrdinaryResultOriginV1::Instance { source, .. } => source.call_site(),
        }
    }
    pub(crate) fn receiver_site(&self) -> &SourceExprSiteV1 {
        match &self.source {
            OrdinaryResultOriginV1::Static(source) => source.method_source().receiver_site(),
            OrdinaryResultOriginV1::Instance { source, .. } => source.receiver_site(),
        }
    }
    pub(crate) fn argument_sites(&self) -> &[SourceExprSiteV1] {
        match &self.source {
            OrdinaryResultOriginV1::Static(source) => source.argument_sites(),
            OrdinaryResultOriginV1::Instance { source, .. } => source.argument_sites(),
        }
    }
    pub(crate) fn target(&self) -> &hakorune_mir_defs::CanonicalSameModuleCallableKeyV1 {
        match &self.source {
            OrdinaryResultOriginV1::Static(source) => source.target(),
            OrdinaryResultOriginV1::Instance { source, .. } => source.target(),
        }
    }
    pub(crate) fn corroborates_input_v1(&self, input: ResolvedFunctionLoweringInputV1<'_>) -> bool {
        let Some(call) = input
            .function()
            .method_call_source_v1(self.call_site().site())
        else {
            return false;
        };
        match &self.source {
            OrdinaryResultOriginV1::Static(source) => {
                std::sync::Arc::ptr_eq(call, source.method_source())
            }
            OrdinaryResultOriginV1::Instance { source, .. } => {
                source.call_site().owner() == input.owner()
                    && source.matches_receiver(input, call)
                    && source.target().name() == call.selector()
                    && source.target().arity() == call.arity()
            }
        }
    }
}

/// Select the retained source role once; a selected refusal never tries another writer.
pub(in crate::mir::normal_callable_semantic_package) fn source_ordinary_result_arguments_v1(
    source: &Result<PreparedBorrowedFormalIngressV1, String>,
    pending: &PendingBorrowedFormalActualsV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    site: &OwnedExprSiteV1,
    targets: &super::super::PreparedLexicalInstanceCallSourceTargetsV1,
    selected: &crate::mir::normal_callable_semantic_package::selected_mapping::VerifiedSelectedCallableBatchMapV1,
    completed: Option<&crate::mir::normal_callable_semantic_package::result_contract::VerifiedCallableResultContractBuilderV1>,
    expression: Option<
        &crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1,
    >,
) -> Result<Option<BorrowedCallArgumentsV1>, String> {
    let Ok(prepared) = source else {
        return Ok(None);
    };
    if original_static_source_v1(prepared, site)?.is_some() {
        return source_static_result_arguments_v1(
            source, pending, contracts, site, selected, completed,
        );
    }
    // An unrelated scalar consumer must keep its original scalar demand/refusal.
    // This passive loan needs the SAME original Text-purpose expression.
    let Some(expression) = expression else {
        return Ok(None);
    };
    if expression.owner() != site.owner()
        || !(matches!(expression.root(), crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::Statement {
            role: crate::mir::resolved_value_profile::expression_root::StatementExpressionRoleV1::PrintValue, .. })
            || matches!(expression.root(), crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument { .. })
            || matches!(expression.root(), crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::Assignment { assignment }
                if assignment.form() == crate::mir::resolved_semantics::ResolvedAssignmentFormV1::Plain))
        || !expression.nodes().iter().any(|node| node.site() == site.site()
            && matches!(node.source(), crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::MethodCall))
    { return Ok(None); }
    let Some(original) = crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1::original_instance_input_call_v1(prepared, contracts, site)?
        else { return Ok(None) };
    // Do not project SelectedStored/PendingStored into this ordinary Instance role.
    let BorrowedIncomingSourceV1::Instance(target) = original.source else {
        return Ok(None);
    };
    let Some(text) = completed
        .and_then(|rows| rows.row(target.target_batch_slot()))
        .and_then(|row| row.completed_instance_text_source_v1(selected, &target))
    else {
        return Ok(None);
    };
    let reject = || freeze("borrowed-instance/source-identity");
    let retained =
        super::super::original_lexical_target_at_v1(targets, site)?.ok_or_else(reject)?;
    if retained != &target
        || original.callee != target.callee_owner()
        || target.target().namespace()
            != hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod
    {
        return Err(reject());
    }
    let arguments =
        project_pending_instance_arguments_by_phase_v1(source, pending, contracts, &target, None)?;
    Ok(Some(BorrowedCallArgumentsV1::SourceResult {
        loan: LocalCallResultSourceV1::Ordinary(OrdinaryResultSourceLoanV1 {
            source: OrdinaryResultOriginV1::Instance {
                source: target,
                text,
            },
            result: LocalCallResultClassV1::Text,
        }),
        arguments,
    }))
}

/// Lend the original Static Text/Bool result and input tuple, never an ABI.
/// Once selected, missing/refused actuals remain terminal refusals.
pub(in crate::mir::normal_callable_semantic_package) fn source_static_result_arguments_v1(
    source: &Result<PreparedBorrowedFormalIngressV1, String>,
    pending: &PendingBorrowedFormalActualsV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    site: &OwnedExprSiteV1,
    selected: &crate::mir::normal_callable_semantic_package::selected_mapping::VerifiedSelectedCallableBatchMapV1,
    completed: Option<&crate::mir::normal_callable_semantic_package::result_contract::VerifiedCallableResultContractBuilderV1>,
) -> Result<Option<crate::mir::resolved_semantics::home_new_prefix::BorrowedCallArgumentsV1>, String>
{
    // Preserve scalar's existing unselected behavior when no source inventory
    // exists. Its issuer still reports errors for selected Instance targets.
    let Ok(prepared) = source else {
        return Ok(None);
    };
    let Some(original) = original_static_source_v1(prepared, site)? else {
        return Ok(None);
    };
    let catalog_text =
        ordinary_text_argument_source_v1(prepared, site, original.method_source())?.is_some();
    let completed_text = completed
        .and_then(|rows| rows.row(original.target_batch_slot()))
        .is_some_and(|row| {
            row.borrow()
                .corroborates_static_text_source_v1(selected, original)
        });
    use crate::mir::callable_result_representation::{
        VerifiedCallableResultDispositionV1 as Result,
        VerifiedCallableResultRepresentationV1 as Representation,
    };
    use crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1 as Class;
    let result = if matches!(original.source_result(), Result::ExactBool) {
        // Instance callers need not have a general call-site row. The sealed
        // target result is unconditional, as in project_unconditional_result;
        // any retained call-site row must agree. Input and ABI checks follow.
        if original.call_result_evidence().is_some_and(|row| {
            row.static_target_key() != Some(original.target())
                || row.result_representation() != Representation::ExactBool
        }) {
            return Err(freeze("borrowed-static/core-source-identity"));
        }
        Class::Bool
    } else if catalog_text || completed_text {
        Class::Text
    } else {
        return Ok(None);
    };
    let original = Rc::clone(original);
    let reject = || freeze("borrowed-static/core-source-identity");
    let mut matches = contracts
        .iter()
        .filter(|contract| contract.owner == original.callee_owner());
    let contract = matches.next().ok_or_else(reject)?;
    if matches.next().is_some() || contract.batch_slot != original.target_batch_slot()
        || contract.mode != crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::StaticBoxMethod
        || contract.parameters.len() != original.target().arity() as usize
        || contract.parameters.len() != original.parameters().len()
        || contract.parameters.iter().zip(original.parameters()).enumerate().any(|(index, (actual, expected))|
            actual.ordinal as usize != index || actual.ordinal != expected.ordinal
                || actual.binding.owner() != contract.owner || actual.binding != expected.binding || actual.kind != expected.kind)
    { return Err(reject()); }
    let incoming = prepared
        .source_incoming
        .project_source_v1(&[contract.owner].into_iter().collect())
        .map_err(|issue| format!("{}: {issue:?}", reject()))?;
    let mut matching = incoming.iter().filter(|row| &row.call == site);
    let row = matching.next().ok_or_else(reject)?;
    if matching.next().is_some()
        || row.callee != contract.owner
        || !matches!(&row.source, BorrowedIncomingSourceV1::Static(retained)
            if Rc::ptr_eq(retained, &original))
    {
        return Err(reject());
    }
    let rows = pending
        .get(site)
        .ok_or_else(|| freeze("borrowed-static/source-unobserved"))?
        .as_ref()
        .map_err(Clone::clone)?;
    let rows = rows
        .source_static_snapshot_v1()
        .ok_or_else(|| freeze("borrowed-static/source-phase-required"))?;
    let identity =
        validate_static_source_actuals_v1(prepared, rows, site, original.required_i64_arguments())?;
    if !Rc::ptr_eq(&identity.source, &original) {
        return Err(reject());
    }
    validate_qualified_argument_sources_v1(prepared, pending, site, identity)?;
    Ok(Some(
        crate::mir::resolved_semantics::home_new_prefix::BorrowedCallArgumentsV1::SourceResult {
            loan:
                crate::mir::resolved_semantics::home_new_prefix::LocalCallResultSourceV1::Ordinary(
                    OrdinaryResultSourceLoanV1 {
                        source: OrdinaryResultOriginV1::Static(original),
                        result,
                    },
                ),
            arguments: rows.ordered_arguments.clone(),
        },
    ))
}

impl crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1 {
    /// Reborrow the SAME completed Text source from the original observation.
    /// Execution additionally requires the whole incoming cohort and signature.
    /// The affine packet never owns a second result-proof registry.
    pub(in crate::mir::normal_callable_semantic_package) fn checked_local_instance_text_result_source_v1(
        &self,
        source: &LexicalInstanceCallSourceTargetV1,
    ) -> Result<bool, String> {
        let Some(observation) =
            self.local_call_for_owner(source.call_site().owner(), source.call_site().site())
        else {
            return Ok(false);
        };
        let Some(LocalCallResultSourceV1::Ordinary(loan)) = observation.result_source() else {
            return Ok(false);
        };
        let OrdinaryResultOriginV1::Instance {
            source: original,
            text,
        } = &loan.source
        else {
            return Ok(false);
        };
        let reject = || freeze("borrowed-call/result-not-corroborated");
        if original != source
            || observation.site() != source.call_site()
            || observation.owner() != source.call_site().owner()
            || observation.result() != LocalCallResultClassV1::Text
            || loan.result != LocalCallResultClassV1::Text
        {
            return Err(reject());
        }
        let (selected, signatures) = self
            .original_instance_input_issuance
            .as_ref()
            .ok_or_else(reject)?;
        let catalog = self
            .original_parameter_contracts
            .as_deref()
            .ok_or_else(reject)?;
        let completion = self
            .completion_source_for_owner_v1(source.callee_owner())
            .ok_or_else(reject)?;
        let terminals = self
            .terminal_relation_index
            .get(&source.callee_owner())
            .ok_or_else(reject)?;
        if !text.matches_retained(completion, terminals)
            || !text.corroborates_instance(selected, source)
        {
            return Err(reject());
        }
        let prepared = self
            .borrowed_formal_source
            .as_ref()
            .ok_or_else(reject)?
            .as_ref()
            .map_err(Clone::clone)?;
        let incoming =
            Self::original_instance_input_call_v1(prepared, catalog, source.call_site())?
                .ok_or_else(reject)?;
        if incoming.callee != source.callee_owner()
            || !matches!(&incoming.source, BorrowedIncomingSourceV1::Instance(target) if target == source)
        {
            return Err(reject());
        }
        let signature = signatures
            .row(source.target_batch_slot())
            .ok_or_else(reject)?;
        if signature.mode() != crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::InstanceBoxMethod
            || signature.source_logical_arity() != source.target().arity() || signature.receiver_lane_count() != 1
            || signature.lanes().first().is_none_or(|lane| lane.index() != 0
                || lane.role() != crate::mir::normal_callable_semantic_package::physical_signature::PhysicalCallableLaneRoleV1::InstanceReceiver)
        { return Err(reject()); }
        let arguments = self
            .checked_completed_original_instance_target_arguments_v1(
                source, selected, catalog, signatures,
            )?
            .ok_or_else(|| freeze("borrowed-entry/original-input-not-ready"))?;
        if arguments != observation.arguments() {
            return Err(freeze("borrowed-call/disposition-identity"));
        }
        Ok(true)
    }

    pub(in crate::mir) fn corroborate_local_instance_text_call_v1(
        &self,
        row: &LexicalInstanceCallDispositionRowV1,
    ) -> Result<(), String> {
        if row.result() != Some(crate::mir::instruction::InvokeCallResultKind::OwnedString)
            || !self.checked_local_instance_text_result_source_v1(row.source_target())?
        {
            return Err(freeze("borrowed-call/result-not-corroborated"));
        }
        Ok(())
    }
}
