//! One selected CurrentOwner Static packet source, before physical actuals.
//! The original incoming Rc remains the only call/target identity.

use std::collections::BTreeMap;
use std::rc::Rc;

use crate::mir::builder::SelectedNormalCallableKeyV1;
use crate::mir::callable_parameter_contract::{
    CallableParameterContractKindV1, CallableParameterDeclarationModeV1,
};
use crate::mir::callable_result_representation::VerifiedCallableResultDispositionV1;
use crate::mir::callable_result_representation::VerifiedStaticCallResultPublicationHandoffV1;
use crate::mir::resolved_semantics::{
    home_new_prefix::{LocalCallArgumentV1, LocalCallObservationV1, LocalCallResultClassV1},
    BindingRefV1, OwnedExprSiteV1, ResolvedMethodCallReceiverSourceV1, SourceBindingSiteV1,
    SourceExprSiteV1, SourceStmtSiteV1,
};

use super::super::model::OwnedCallableParameterContractDeclarationV1;
use super::super::qualified_static_call_claim::{
    incoming_source::StaticIncomingSourceV1, QualifiedStaticCallClaimIndexV1,
};
use super::super::result_contract::VerifiedCallableResultContractCohortV1;
use super::super::selected_mapping::VerifiedSelectedCallableBatchMapV1;
use super::lexical_instance_call::{
    issue_original_static_forwarded_actual_v1, PendingBorrowedFormalActualsV1,
    PreparedBorrowedFormalActualV1, VerifiedStaticForwardedActualV1,
};
use super::loop_static_source_loan::LoopEntryStaticI64SourceLoanV1;
use super::static_loop_tagged_entry::VerifiedStaticLoopTaggedEntrySourceV1;
use super::{OrdinaryNewClaimLedgerV1, OrdinaryNewCoSealIssueV1};
use crate::mir::definitions::MirCall;
use crate::mir::function::MirFunction;
use crate::mir::instruction::{InvokeCallResultKind, InvokeOperation};
use crate::mir::{BasicBlockId, MirInstruction, ValueId};

#[derive(Debug)]
pub(in crate::mir) struct VerifiedStaticLoopPacketSourceV1 {
    loop_site: SourceStmtSiteV1,
    declaration: SourceBindingSiteV1,
    original: Rc<StaticIncomingSourceV1>,
    caller_formal: BindingRefV1,
    argument: SourceExprSiteV1,
    complete_incoming: Box<[Rc<StaticIncomingSourceV1>]>,
    forwarded_actuals: Box<[VerifiedStaticForwardedActualV1]>,
}

impl VerifiedStaticLoopPacketSourceV1 {
    pub(in crate::mir) fn local_call_site(&self) -> &OwnedExprSiteV1 {
        self.original.call_site()
    }

    pub(in crate::mir) fn original_source(&self) -> &Rc<StaticIncomingSourceV1> {
        &self.original
    }

    pub(in crate::mir) fn corroborates_local_call(
        &self,
        observation: &LocalCallObservationV1,
    ) -> bool {
        observation.site() == self.original.call_site()
            && observation.result() == LocalCallResultClassV1::I64
            && matches!(observation.local_binding(), Some((declaration, _))
                if declaration == &self.declaration)
            && matches!(observation.arguments(),
                [LocalCallArgumentV1::BorrowedActual { ordinal: 0, site }]
                    if site == &self.argument)
    }

    pub(in crate::mir) fn publication_source(
        &self,
    ) -> (
        &crate::mir::builder::CanonicalSameModuleCallableKeyV1,
        &SourceExprSiteV1,
    ) {
        (self.original.caller(), self.original.call_site().site())
    }

    pub(in crate::mir) fn corroborates_publication_handoff(
        &self,
        handoff: &VerifiedStaticCallResultPublicationHandoffV1,
    ) -> bool {
        self.original
            .corroborates_selected_loop_publication_handoff(handoff)
            && self
                .forwarded_actuals
                .iter()
                .zip(self.complete_incoming.iter())
                .filter(|(_, source)| Rc::ptr_eq(source, &self.original))
                .filter(|(actual, source)| actual.corroborates(source, self.caller_formal))
                .count()
                == 1
    }

    pub(in crate::mir) fn argument_site(&self) -> &SourceExprSiteV1 {
        &self.argument
    }

    /// Lend one already-checked original source actual. This does not make
    /// the pending SourceStatic phase generally executable.
    pub(in crate::mir) fn selected_forwarded_actual_for_call(
        &self,
        caller: &crate::mir::builder::CanonicalSameModuleCallableKeyV1,
        site: &SourceExprSiteV1,
    ) -> Result<&PreparedBorrowedFormalActualV1, String> {
        let reject = || "[freeze:contract][callable-loop/static-forwarded-actual-loan]".to_owned();
        if self.complete_incoming.len() != self.forwarded_actuals.len() {
            return Err(reject());
        }
        let mut matching = self
            .complete_incoming
            .iter()
            .zip(self.forwarded_actuals.iter())
            .filter(|(source, _)| source.caller() == caller && source.call_site().site() == site);
        let (source, witness) = matching.next().ok_or_else(reject)?;
        if matching.next().is_some()
            || source.target() != self.original.target()
            || !witness.corroborates(source, witness.caller_formal())
        {
            return Err(reject());
        }
        Ok(witness.actual())
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

    /// Observe the disposable canonical Invoke without publishing an affine
    /// packet or mutating the shared ledger. Final coordinates remain owed.
    pub(in crate::mir) fn corroborate_unpublished_physical_call(
        &self,
        handoff: &VerifiedStaticCallResultPublicationHandoffV1,
        formal: BindingRefV1,
        actual: ValueId,
        function: &MirFunction,
        entry: BasicBlockId,
        normal: BasicBlockId,
        result: ValueId,
    ) -> Result<(), String> {
        let reject = || "[freeze:contract][callable-loop/static-physical-packet-drift]".to_owned();
        if !self.corroborates_publication_handoff(handoff) || formal != self.caller_formal {
            return Err(reject());
        }
        let expected = self.materialize_unpublished_call(formal, actual)?;
        corroborate_unpublished_physical_shape(&expected, actual, function, entry, normal, result)
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
            && self.forwarded_actuals.len() == self.complete_incoming.len()
            && self
                .forwarded_actuals
                .iter()
                .zip(self.complete_incoming.iter())
                .all(|(actual, source)| {
                    let caller_formal = if Rc::ptr_eq(source, &self.original) {
                        formal
                    } else {
                        actual.caller_formal()
                    };
                    actual.corroborates(source, caller_formal)
                })
            && self
                .complete_incoming
                .iter()
                .filter(|row| Rc::ptr_eq(row, &self.original))
                .count()
                == 1
    }
}

fn corroborate_unpublished_physical_shape(
    expected: &MirCall,
    actual: ValueId,
    function: &MirFunction,
    entry: BasicBlockId,
    normal: BasicBlockId,
    result: ValueId,
) -> Result<(), String> {
    let reject = || "[freeze:contract][callable-loop/static-physical-packet-drift]".to_owned();
    if entry == normal || actual == result {
        return Err(reject());
    }
    let source = function.blocks.get(&entry).ok_or_else(reject)?;
    let Some(MirInstruction::Invoke {
        operation: InvokeOperation::Call { call, result: kind },
        normal_landing,
        fault_landing,
        ..
    }) = source.terminator.as_ref()
    else {
        return Err(reject());
    };
    let landing = function.blocks.get(&normal).ok_or_else(reject)?;
    let fault = function.blocks.get(fault_landing).ok_or_else(reject)?;
    if call != expected
        || *kind != InvokeCallResultKind::I64
        || *normal_landing != normal
        || *fault_landing == entry
        || *fault_landing == normal
        || !landing.predecessors.contains(&entry)
        || !fault.predecessors.contains(&entry)
        || !matches!(landing.instructions.first(), Some(MirInstruction::InvokeNormalResult {
                invoke_block,
                dst,
            }) if *invoke_block == entry && *dst == result)
        || function
            .blocks
            .values()
            .flat_map(|block| block.all_instructions())
            .filter(|instruction| instruction.dst_value() == Some(result))
            .count()
            != 1
    {
        return Err(reject());
    }
    Ok(())
}

fn issue_packet_source(
    entry: &LoopEntryStaticI64SourceLoanV1,
    tagged: &VerifiedStaticLoopTaggedEntrySourceV1,
    incoming: &super::lexical_instance_call::PreparedBorrowedFormalIngressV1,
    pending: &PendingBorrowedFormalActualsV1,
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
    let complete_incoming = incoming.static_incoming_cohort_v1(original)?;
    let forwarded_actuals = complete_incoming
        .iter()
        .map(|source| {
            let caller_formal = incoming
                .static_arguments
                .get(&(source.call_site().clone(), 0))
                .ok_or_else(reject)?
                .formal();
            if Rc::ptr_eq(source, original) && caller_formal != tagged.formal() {
                return Err(reject());
            }
            issue_original_static_forwarded_actual_v1(incoming, pending, source, caller_formal)
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(VerifiedStaticLoopPacketSourceV1 {
        loop_site: entry.loop_site().clone(),
        declaration: entry.declaration().clone(),
        original: Rc::clone(original),
        caller_formal: tagged.formal(),
        argument: argument.clone(),
        complete_incoming,
        forwarded_actuals: forwarded_actuals.into_boxed_slice(),
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
                    entry,
                    tagged,
                    incoming,
                    &self.borrowed_formal_actuals,
                    selected,
                    contracts,
                    results,
                    claims,
                )
                .and_then(|packet| {
                    let site = packet.local_call_site();
                    let observation = self
                        .local_call_for_owner(site.owner(), site.site())
                        .ok_or_else(|| {
                            "[freeze:contract][callable-loop/static-packet-local-source-missing]"
                                .to_owned()
                        })?;
                    if !packet.corroborates_local_call(observation) {
                        return Err(
                            "[freeze:contract][callable-loop/static-packet-local-source-drift]"
                                .to_owned(),
                        );
                    }
                    Ok(packet)
                }),
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

    pub(in crate::mir) fn require_static_loop_local_route_v1(
        &self,
        packet: &VerifiedStaticLoopPacketSourceV1,
    ) -> Result<(), String> {
        let site = packet.local_call_site();
        if !self
            .lifecycle_local_call_sites
            .borrow()
            .get(&site.owner())
            .is_some_and(|sites| sites.contains(site))
        {
            return Err("[freeze:contract][callable-loop/static-packet-route-missing]".into());
        }
        Ok(())
    }

    #[cfg(test)]
    pub(in crate::mir) fn remove_static_loop_local_route_for_test(
        &self,
        caller: &crate::mir::builder::CanonicalSameModuleCallableKeyV1,
    ) {
        let sources = self.loop_static_packet_sources.borrow();
        let site = sources
            .values()
            .filter_map(|row| row.as_ref().ok())
            .find(|row| row.publication_source().0 == caller)
            .expect("selected packet")
            .local_call_site()
            .clone();
        self.lifecycle_local_call_sites
            .borrow_mut()
            .get_mut(&site.owner())
            .expect("selected route")
            .retain(|row| row != &site);
    }
}

#[cfg(test)]
mod physical_shape_tests {
    use super::*;
    use crate::mir::{BasicBlock, EffectMask, FunctionSignature, MirType};

    #[test]
    fn selected_static_observation_refuses_wrong_result_and_landing() {
        let entry = BasicBlockId(0);
        let normal = BasicBlockId(1);
        let fault = BasicBlockId(2);
        let actual = ValueId(0);
        let result = ValueId(4);
        let call = MirCall::global(
            None,
            hakorune_mir_defs::CanonicalGlobalTargetV1::new_static_box_method(
                "SizeClassBox".into(),
                "normalize_size".into(),
                1,
            )
            .unwrap(),
            vec![actual],
        );
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "selected".into(),
                params: vec![MirType::Integer],
                return_type: MirType::Integer,
                effects: EffectMask::ALL,
            },
            entry,
        );
        function
            .blocks
            .get_mut(&entry)
            .unwrap()
            .add_instruction(MirInstruction::Invoke {
                operation: InvokeOperation::Call {
                    call: call.clone(),
                    result: InvokeCallResultKind::I64,
                },
                fault_frame: ValueId(3),
                normal_landing: normal,
                fault_landing: fault,
            });
        let mut normal_block = BasicBlock::new(normal);
        normal_block.predecessors.insert(entry);
        normal_block.add_instruction(MirInstruction::InvokeNormalResult {
            invoke_block: entry,
            dst: result,
        });
        function.blocks.insert(normal, normal_block);
        let mut fault_block = BasicBlock::new(fault);
        fault_block.predecessors.insert(entry);
        function.blocks.insert(fault, fault_block);
        assert!(corroborate_unpublished_physical_shape(
            &call, actual, &function, entry, normal, result
        )
        .is_ok());
        {
            let source = function.blocks.get_mut(&entry).unwrap();
            let Some(MirInstruction::Invoke {
                operation: InvokeOperation::Call { result: kind, .. },
                ..
            }) = source.terminator.as_mut()
            else {
                panic!("Invoke");
            };
            *kind = InvokeCallResultKind::Unit;
        }
        assert!(corroborate_unpublished_physical_shape(
            &call, actual, &function, entry, normal, result
        )
        .unwrap_err()
        .contains("static-physical-packet-drift"));
        {
            let source = function.blocks.get_mut(&entry).unwrap();
            let Some(MirInstruction::Invoke {
                operation: InvokeOperation::Call { result: kind, .. },
                normal_landing,
                ..
            }) = source.terminator.as_mut()
            else {
                panic!("Invoke");
            };
            *kind = InvokeCallResultKind::I64;
            *normal_landing = fault;
        }
        assert!(corroborate_unpublished_physical_shape(
            &call, actual, &function, entry, normal, result
        )
        .unwrap_err()
        .contains("static-physical-packet-drift"));
    }
}
