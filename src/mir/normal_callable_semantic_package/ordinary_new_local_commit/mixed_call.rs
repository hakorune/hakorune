//! The same received-call commit retains a conditional Fresh obligation.
//! Its original sealed packet, never non-nullness, supplies the outcome role.
use super::*;
use crate::mir::instruction::InvokeCallResultKind;
use std::rc::Rc;

impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn begin_mixed_call_emission_v1(
        &self, source: &super::super::lexical_instance_call::LexicalInstanceCallDispositionRowV1,
    ) -> Result<(), String> {
        let descriptor = self.mixed_call_teardown_v1(source)?;
        let site = source.call_site();
        let loan = source.source_target().mixed_local_source().ok_or_else(|| freeze("mixed-call/source-missing"))?;
        let observation = self.mixed_call_source_v1(site).ok_or_else(|| freeze("mixed-call/observation-missing"))?;
        if observation.local_binding() != Some((loan.declaration(), loan.destination()))
            || descriptor.object() != loan.object()
            || !matches!(loan.declaration(), SourceBindingSiteV1::Local { .. })
            || loan.destination().owner() != site.owner()
        { return Err(freeze("mixed-call/commit-source-drift")); }
        let mut rows = self.local_commits.borrow_mut();
        if rows.contains_key(site) { return Err(freeze("mixed-call/duplicate-emission")); }
        rows.insert(site.clone(), LocalCommitV1::CallReceived(CallReceivedCommitV1 {
            owner: site.owner(), binding: loan.destination(), declaration: loan.declaration().clone(),
            object: loan.object(), release: CallReceivedReleaseV1::Mixed { formal_ordinal: loan.callee_ordinal() },
            end_children: descriptor.owned_children().map(|children| Some(children.into())),
            field_support_parents: None,
            progress: CallReceivedProgress::Emitting,
        }));
        Ok(())
    }

    pub(super) fn checked_mixed_received_packet_v1<'a>(
        &'a self, site: &OwnedExprSiteV1, commit: &CallReceivedCommitV1,
        packet: &EmittedLexicalCallProjectionV1,
    ) -> Result<&'a [crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1], String> {
        self.checked_mixed_received_packet_in_context_v1(site, commit, packet, CallValidationContextV1::Lowering(self))
    }

    pub(super) fn checked_mixed_received_packet_in_context_v1<'a>(
        &'a self, site: &OwnedExprSiteV1, commit: &CallReceivedCommitV1,
        packet: &EmittedLexicalCallProjectionV1, context: CallValidationContextV1<'_>,
    ) -> Result<&'a [crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1], String> {
        context.require_ledger(self)?;
        let source = packet.original_row()?;
        let descriptor = self.mixed_call_teardown_using_v1(source, context)?;
        let loan = source.source_target().mixed_local_source().ok_or_else(|| freeze("mixed-call/source-missing"))?;
        let observation = self.mixed_call_source_v1(site).ok_or_else(|| freeze("mixed-call/observation-missing"))?;
        let expected = InvokeCallResultKind::MixedHandle { formal_ordinal: loan.callee_ordinal() };
        let children = descriptor.owned_children().map(|children| Some(Box::<[_]>::from(children)));
        if packet.call_site() != site || commit.owner != site.owner()
            || commit.declaration != *loan.declaration() || commit.binding != loan.destination()
            || commit.object != descriptor.object() || commit.end_children != children
            || source.result() != Some(expected)
            || commit.release != (CallReceivedReleaseV1::Mixed { formal_ordinal: loan.callee_ordinal() })
            || observation.mixed() != Some(loan)
        { return Err(freeze("mixed-call/record-source-drift")); }
        Ok(observation.arguments())
    }

    pub(crate) fn record_mixed_call_emission_v1(
        &self, site: &OwnedExprSiteV1, result: ValueId,
        bindings: Vec<(BasicBlockId, MirInstruction)>, packet: Rc<EmittedLexicalCallProjectionV1>,
    ) -> Result<(), String> {
        {
            let rows = self.local_commits.borrow();
            let Some(LocalCommitV1::CallReceived(row)) = rows.get(site) else { return Err(freeze("mixed-call/record-without-begin")); };
            if !matches!(row.progress, CallReceivedProgress::Emitting) || bindings.is_empty() {
                return Err(freeze("mixed-call/emission-state-drift"));
            }
            let arguments = self.checked_mixed_received_packet_v1(site, row, &packet)?;
            if packet.value_with_ledger(site.owner(), arguments, self)? != result { return Err(freeze("mixed-call/record-result-drift")); }
            packet.validate_recorded(&bindings)?;
        }
        {
            let groups = self.root_local_call_bindings.borrow();
            let original = groups.get(&site.owner()).into_iter().flatten()
                .find(|group| group.site() == site).and_then(|group| group.lexical());
            if original.is_none_or(|original| !std::ptr::eq(original, packet.as_ref())) {
                return Err(freeze("mixed-call/record-packet-drift"));
            }
        }
        let mut rows = self.local_commits.borrow_mut();
        let Some(LocalCommitV1::CallReceived(row)) = rows.get_mut(site) else { return Err(freeze("mixed-call/record-without-begin")); };
        row.progress = CallReceivedProgress::Emitted { result, bindings, packet: Some(packet), phase: CallReceivedPhase::ExpressionCompleted };
        Ok(())
    }
}
