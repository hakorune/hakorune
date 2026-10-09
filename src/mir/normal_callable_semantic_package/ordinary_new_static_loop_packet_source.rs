//! One selected CurrentOwner Static packet source, before physical actuals.
//! The original incoming Rc remains the only call/target identity.

use std::collections::BTreeMap;
use std::rc::Rc;

use crate::mir::builder::SelectedNormalCallableKeyV1;
use crate::mir::callable_parameter_contract::{
    CallableParameterContractKindV1, CallableParameterDeclarationModeV1,
};
use crate::mir::callable_result_representation::VerifiedCallableResultDispositionV1;
use crate::mir::resolved_semantics::{
    BindingRefV1, ResolvedMethodCallReceiverSourceV1, SourceBindingSiteV1, SourceExprSiteV1,
    SourceStmtSiteV1,
};

use super::super::model::OwnedCallableParameterContractDeclarationV1;
use super::super::qualified_static_call_claim::{
    incoming_source::StaticIncomingSourceV1, QualifiedStaticCallClaimIndexV1,
};
use super::super::result_contract::VerifiedCallableResultContractCohortV1;
use super::super::selected_mapping::VerifiedSelectedCallableBatchMapV1;
use super::loop_static_source_loan::LoopEntryStaticI64SourceLoanV1;
use super::static_loop_tagged_entry::VerifiedStaticLoopTaggedEntrySourceV1;
use super::{OrdinaryNewClaimLedgerV1, OrdinaryNewCoSealIssueV1};
use crate::mir::definitions::MirCall;
use crate::mir::ValueId;

#[derive(Debug)]
pub(in crate::mir) struct VerifiedStaticLoopPacketSourceV1 {
    loop_site: SourceStmtSiteV1,
    declaration: SourceBindingSiteV1,
    original: Rc<StaticIncomingSourceV1>,
    caller_formal: BindingRefV1,
    argument: SourceExprSiteV1,
}

impl VerifiedStaticLoopPacketSourceV1 {
    pub(in crate::mir) fn argument_site(&self) -> &SourceExprSiteV1 {
        &self.argument
    }

    pub(in crate::mir) fn materialize_unpublished_call(
        &self,
        formal: BindingRefV1,
        actual: ValueId,
    ) -> Result<MirCall, String> {
        if formal != self.caller_formal {
            return Err("[freeze:contract][callable-loop/static-packet-formal-drift]".into());
        }
        let target = self
            .original
            .target()
            .canonical_global_target_v1()
            .map_err(|error| {
                format!("[freeze:contract][callable-loop/static-packet-target] {error}")
            })?;
        Ok(MirCall::global(None, target, vec![actual]))
    }

    pub(in crate::mir) fn corroborates(
        &self,
        entry: &LoopEntryStaticI64SourceLoanV1,
        formal: BindingRefV1,
    ) -> bool {
        self.loop_site == *entry.loop_site()
            && self.declaration == *entry.declaration()
            && Rc::ptr_eq(&self.original, entry.original())
            && self.caller_formal == formal
            && entry.original().argument_sites().get(0) == Some(&self.argument)
    }
}

fn issue_packet_source(
    entry: &LoopEntryStaticI64SourceLoanV1,
    tagged: &VerifiedStaticLoopTaggedEntrySourceV1,
    incoming: &super::lexical_instance_call::PreparedBorrowedFormalIngressV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    results: &VerifiedCallableResultContractCohortV1,
    claims: &QualifiedStaticCallClaimIndexV1,
) -> Result<VerifiedStaticLoopPacketSourceV1, String> {
    let reject = || "[freeze:contract][callable-loop/static-packet-source-unavailable]".to_owned();
    let original = entry.original();
    let [argument] = original.argument_sites() else {
        return Err(reject());
    };
    let [source_formal] = original.parameters() else {
        return Err(reject());
    };
    let mut matching = contracts
        .iter()
        .filter(|row| row.owner == original.callee_owner());
    let contract = matching.next().ok_or_else(reject)?;
    let result = results
        .row(original.target_batch_slot())
        .ok_or_else(reject)?;
    let result_ref = result.borrow();
    if matching.next().is_some()
        || !original.is_current_owner_i64_source_v1()
        || original.is_qualified()
        || !tagged.corroborates(entry)
        || original.call_site().owner() != tagged.formal().owner()
        || !entry.claim().corroborates_source(
            original.call_site(),
            ResolvedMethodCallReceiverSourceV1::CurrentOwner,
            1,
        )
        || !matches!(
            selected.key_for_batch_slot(original.target_batch_slot()),
            Some(SelectedNormalCallableKeyV1::Cataloged(key)) if key == original.target()
        )
        || contract.batch_slot != original.target_batch_slot()
        || contract.mode != CallableParameterDeclarationModeV1::StaticBoxMethod
        || contract.parameters.len() != 1
        || contract.parameters[0].ordinal != 0
        || contract.parameters[0].binding != source_formal.binding
        || contract.parameters[0].kind != CallableParameterContractKindV1::OpaqueHandle
        || source_formal.kind != CallableParameterContractKindV1::OpaqueHandle
        || !incoming.checked_static_input(source_formal.binding)
        || result.owner() != original.callee_owner()
        || result_ref.completion().owner() != original.callee_owner()
        || !result_ref.completion().returns_value()
        || !result_ref.declared_result_agrees_with_i64_source()
        || !matches!(
            claims.result_for_key(original.target()),
            Some(VerifiedCallableResultDispositionV1::ExactI64 { .. })
        )
    {
        return Err(reject());
    }
    Ok(VerifiedStaticLoopPacketSourceV1 {
        loop_site: entry.loop_site().clone(),
        declaration: entry.declaration().clone(),
        original: Rc::clone(original),
        caller_formal: tagged.formal(),
        argument: argument.clone(),
    })
}

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn retain_static_loop_packet_sources_v1(
        &mut self,
        selected: &VerifiedSelectedCallableBatchMapV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        results: &VerifiedCallableResultContractCohortV1,
        claims: &QualifiedStaticCallClaimIndexV1,
    ) -> Result<(), OrdinaryNewCoSealIssueV1> {
        let Some(Ok(incoming)) = self.borrowed_formal_source.as_ref() else {
            return Ok(());
        };
        let entries = self.loop_entry_static_i64_source_loans.borrow();
        let tagged = self.loop_static_tagged_entry.borrow();
        let mut rows = BTreeMap::new();
        for (key, tagged_row) in tagged.iter() {
            let row = match (entries.get(key), tagged_row) {
                (Some(Ok(entry)), Ok(tagged)) => issue_packet_source(
                    entry, tagged, incoming, selected, contracts, results, claims,
                ),
                (Some(Err(error)), _) | (_, Err(error)) => Err(error.clone()),
                (None, _) => {
                    Err("[freeze:contract][callable-loop/static-packet-entry-missing]".to_owned())
                }
            };
            if rows.insert(key.clone(), row).is_some() {
                return Err(OrdinaryNewCoSealIssueV1::BatchLoan);
            }
        }
        *self.loop_static_packet_sources.get_mut() = rows;
        Ok(())
    }

    pub(in crate::mir) fn take_static_loop_packet_source_v1(
        &self,
        loop_site: &SourceStmtSiteV1,
        declaration: &SourceBindingSiteV1,
    ) -> Option<Result<VerifiedStaticLoopPacketSourceV1, String>> {
        self.loop_static_packet_sources
            .borrow_mut()
            .remove(&(loop_site.clone(), declaration.clone()))
    }
}
