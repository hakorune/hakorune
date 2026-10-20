//! Original Received destination joins its existing producer and exact Return.
use super::super::physical_boundary::FinishedBindings;
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::ObjectReturnAcquisitionV1;

impl OrdinaryNewClaimLedgerV1 {
    /// A Text Return transfers exactly its original current residence after
    /// Normal cleanup. Its Fault source still owns that SAME acquisition.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_text_return_producer_v1(
        &self, context: CallValidationContextV1<'_>, owner: FunctionOwnerIdV1,
        exit: &SourceStmtSiteV1, entry: &RootHomeExitEntry,
        bindings: &[(BasicBlockId, MirInstruction)],
        producer_projection: Option<&FinishedBindings>, return_projection: Option<&FinishedBindings>,
    ) -> Result<Option<Vec<(BasicBlockId, MirInstruction)>>, String> {
        let Some(TerminalRelationV1::Value(value)) = self.terminal_relation_for_owner_at(owner, exit)
            else { return Ok(None) };
        let TerminalReturnedSourceV1::TextResidence { binding, acquisition } = value.returned()
            else { return Ok(None) };
        let normal = self.normal_exit_projection_v1(owner, exit)?
            .ok_or_else(|| freeze("projection-proof-identity"))?;
        let home = crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1::Text(acquisition.clone());
        if binding.owner() != owner || value.owner() != owner || value.return_site() != exit
            || !matches!(entry, RootHomeExitEntry::Plain { .. })
            || normal.fault_homes().iter().filter(|candidate| **candidate == home).count() != 1
            || normal.homes().contains(&home) {
            return Err(freeze("projection-proof-identity"));
        }
        // The source return may read an alias; its sealed acquisition, rather
        // than spelling/binding equality, selects the one installed packet.
        let packet = context.text_home_for_acquisition_v1(acquisition)?;
        if !packet.acquisition().same_source(acquisition) || packet.value() != packet.local_value() {
            return Err(freeze("expression/residence-original-pair"));
        }
        let mut mandatory = Vec::new();
        for original in [packet.root_acquisition_binding_v1()?, packet.install_binding()
            .ok_or_else(|| freeze("expression/residence-original-pair"))?] {
            mandatory.push(match producer_projection {
                Some(projection) => projection.binding(original.0, &original.1)?
                    .ok_or_else(|| freeze("root-exit-binding-drift"))?,
                None => original.clone(),
            });
        }
        if !matches!(mandatory[0].1, MirInstruction::InvokeNormalResult { dst, .. } if dst == packet.value())
            || !matches!(mandatory[1].1, MirInstruction::StringResidenceInstall { value, .. } if value == packet.value()) {
            return Err(freeze("expression/residence-original-pair"));
        }
        let returned = match return_projection { Some(projection) => projection.bindings(bindings)?,
            None => bindings.to_vec() };
        let mut returns = returned.iter().filter_map(|(_, row)| match row {
            MirInstruction::Return { value } => Some(value), _ => None,
        });
        if returns.next() != Some(&Some(packet.value())) || returns.next().is_some() {
            return Err(freeze("expression/residence-original-pair"));
        }
        Ok(Some(mandatory))
    }

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
            if packet.outer_bindings()?.1 != producer {
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
