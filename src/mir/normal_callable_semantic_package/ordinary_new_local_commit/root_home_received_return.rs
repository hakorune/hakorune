//! Original Received destination joins its existing producer and exact Return.
use super::super::physical_boundary::FinishedBindings;
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::ObjectReturnAcquisitionV1;

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_received_return_producer_v1(
        &self,
        owner: FunctionOwnerIdV1,
        exit: &SourceStmtSiteV1,
        entry: &RootHomeExitEntry,
        bindings: &[(BasicBlockId, MirInstruction)],
        producer_projection: Option<&FinishedBindings>,
        return_projection: Option<&FinishedBindings>,
    ) -> Result<Option<(BasicBlockId, MirInstruction)>, String> {
        let Some(TerminalRelationV1::Value(value)) =
            self.terminal_relation_for_owner_at(owner, exit)
        else {
            return Ok(None);
        };
        let TerminalReturnedSourceV1::OwnedCall(obligation) = value.returned() else {
            return Ok(None);
        };
        let ObjectReturnAcquisitionV1::Received(original) = obligation.acquisition() else {
            return Ok(None);
        };
        if self.normal_exit_projection_v1(owner, exit)?.is_none()
            || !matches!(entry, RootHomeExitEntry::Plain { .. })
        {
            return Err(freeze("received-return/source-or-entry"));
        }
        let (declaration, destination) = original
            .local_binding()
            .ok_or_else(|| freeze("received-return/destination-missing"))?;
        let rows = self.local_commits.borrow();
        let Some(LocalCommitV1::CallReceived(row)) = rows.get(original.site()) else {
            return Err(freeze("received-return/commit-missing"));
        };
        if original.owner() != owner
            || original.site().owner() != owner
            || row.owner != owner
            || row.binding != destination
            || &row.declaration != declaration
        {
            return Err(freeze("received-return/destination-identity"));
        }
        let local = row
            .local()
            .ok_or_else(|| freeze("received-return/local-not-installed"))?;
        if producer_projection.is_some() {
            row.checked_bindings()?;
        }
        let CallReceivedProgress::Emitted {
            bindings: produced,
            packet,
            ..
        } = &row.progress
        else {
            return Err(freeze("received-return/producer-missing"));
        };
        let mut groups = entry
            .local_call_groups()
            .iter()
            .filter(|group| group.site() == original.site());
        let group = groups
            .next()
            .ok_or_else(|| freeze("received-return/group-missing"))?;
        if groups.next().is_some() {
            return Err(freeze("received-return/group-duplicate"));
        }
        let retained_packet = match (packet, group.lexical()) {
            (Some(packet), Some(shared)) if std::ptr::eq(packet.as_ref(), shared) => {
                if packet.call_site() != original.site() {
                    return Err(freeze("received-return/packet-site"));
                }
                packet.validate_recorded(produced)?;
                Some(shared)
            }
            (None, Some(shared)) => {
                // ClaimLocal keeps its packet in the binding group, whereas
                // Self receiver commits must retain that same packet too.
                if shared.call_site() != original.site()
                    || shared.original_row()?.source_target().is_self_receiver()
                {
                    return Err(freeze("received-return/packet-identity"));
                }
                shared.validate_recorded(produced)?;
                Some(shared)
            }
            (None, None) => None,
            _ => return Err(freeze("received-return/packet-identity")),
        };
        let mut producers = produced.iter().filter(|(_, instruction)| {
            matches!(instruction, MirInstruction::InvokeNormalResult { dst, .. } if *dst == local)
        });
        let producer = producers
            .next()
            .ok_or_else(|| freeze("received-return/producer-missing"))?;
        if producers.next().is_some() || !group.bindings().contains(producer) {
            return Err(freeze("received-return/producer-identity"));
        }
        if let Some(packet) = retained_packet {
            if packet.outer_bindings().1 != producer {
                return Err(freeze("received-return/packet-producer"));
            }
        }
        let producer = match producer_projection {
            Some(projection) => projection
                .binding(producer.0, &producer.1)?
                .ok_or_else(|| freeze("received-return/producer-removed"))?,
            None => producer.clone(),
        };
        let MirInstruction::InvokeNormalResult { dst, .. } = &producer.1 else {
            return Err(freeze("received-return/producer-kind"));
        };
        let returned = match return_projection {
            Some(projection) => projection.bindings(bindings)?,
            None => bindings.to_vec(),
        };
        let mut returns = returned
            .iter()
            .filter_map(|(_, instruction)| match instruction {
                MirInstruction::Return { value } => Some(value),
                _ => None,
            });
        if returns.next() != Some(&Some(*dst)) || returns.next().is_some() {
            return Err(freeze("received-return/return-value"));
        }
        Ok(Some(producer))
    }
}
