//! Original Received destination joins its existing producer and exact Return.
use super::super::physical_boundary::FinishedBindings;
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::ObjectReturnAcquisitionV1;

impl OrdinaryNewClaimLedgerV1 {
    /// Issue the hold from the original Return relation and installed packet.
    /// The builder emits this same event before entering the cleanup graph.
    pub(crate) fn bind_root_text_return_hold_v1(
        &self,
        owner: FunctionOwnerIdV1,
        exit: &SourceStmtSiteV1,
        block: BasicBlockId,
        value: ValueId,
    ) -> Result<Option<MirInstruction>, String> {
        let Some(TerminalRelationV1::Value(terminal)) =
            self.terminal_relation_for_owner_at(owner, exit)
        else {
            return Ok(None);
        };
        match terminal.returned() {
            TerminalReturnedSourceV1::TextResidence { acquisition, .. } => {
                let packet = CallValidationContextV1::Lowering(self)
                    .text_home_for_acquisition_v1(acquisition)?;
                if !packet.acquisition().same_source(acquisition)
                    || packet.value() != value
                    || packet.local_value() != value
                {
                    return Err(freeze("expression/residence-original-pair"));
                }
            }
            TerminalReturnedSourceV1::TextExpression(source) => {
                let rows = self.root_exits.borrow();
                let Some(RootHomeExitProgress::Emitting(order)) =
                    rows.get(&(owner, exit.clone().into()))
                else {
                    return Err(freeze("root-exit-record-without-emission"));
                };
                if !order.return_expression.as_ref().is_some_and(|row| {
                    std::rc::Rc::ptr_eq(&row.contract.source, source) && row.contract.value == value
                }) {
                    return Err(freeze("expression/residence-original-pair"));
                }
            }
            _ => return Ok(None),
        }
        if terminal.owner() != owner || terminal.return_site() != exit {
            return Err(freeze("expression/residence-original-pair"));
        }
        let mut exits = self.root_exits.borrow_mut();
        let Some(RootHomeExitProgress::Emitting(order)) =
            exits.get_mut(&(owner, exit.clone().into()))
        else {
            return Err(freeze("root-exit-record-without-emission"));
        };
        if order.text_return_hold.is_some() {
            return Err(freeze("expression/residence-original-pair"));
        }
        let hold = MirInstruction::StringResidenceInstall {
            value,
            replaces: None,
            returning: true,
        };
        order.text_return_hold = Some((block, hold.clone()));
        Ok(Some(hold))
    }

    /// A Text Return transfers exactly its original current residence after
    /// Normal cleanup. Its Fault source still owns that SAME acquisition.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_return_expression_producer_v1(
        &self,
        context: CallValidationContextV1<'_>,
        owner: FunctionOwnerIdV1,
        exit: &SourceStmtSiteV1,
        order: &RootHomeCleanupOrderV1,
        entry: &RootHomeExitEntry,
        bindings: &[(BasicBlockId, MirInstruction)],
        producer_projection: Option<&FinishedBindings>,
        return_projection: Option<&FinishedBindings>,
        function: Option<&MirFunction>,
    ) -> Result<Option<Vec<(BasicBlockId, MirInstruction)>>, String> {
        if let Some(bindings) = self.validate_plain_scalar_return_read_v1(
            context,
            owner,
            exit,
            order,
            entry,
            bindings,
            producer_projection,
            return_projection,
            function,
        )? {
            return Ok(Some(bindings));
        }
        if let Some(TerminalRelationV1::Scalar(terminal)) =
            self.terminal_relation_for_owner_at(owner, exit)
        {
            if let Some(source) = terminal.evaluation() {
                let row = order
                    .return_expression
                    .as_ref()
                    .ok_or_else(|| freeze("expression/residence-original-pair"))?;
                if !std::rc::Rc::ptr_eq(source, &row.contract.source)
                    || source.source().owner() != owner
                    || source.source().root().statement() != exit
                    || terminal.owner() != owner
                    || terminal.return_site() != exit
                    || row.contract.residence.is_some()
                    || order.text_return_hold.is_some()
                    || !matches!(entry, RootHomeExitEntry::Plain { .. })
                {
                    return Err(freeze("expression/residence-original-pair"));
                }
                row.expression().validate_source(context)?;
                if let Some(function) = function {
                    row.validate(context, function, producer_projection)?;
                    if function.blocks.values().flat_map(|block| block.all_instructions())
                        .filter(|instruction| matches!(instruction, MirInstruction::Return { value: Some(value) } if *value == row.contract.value))
                        .count() != 1 {
                        return Err(freeze("expression/residence-original-pair"));
                    }
                }
                let returned = match return_projection {
                    Some(projection) => projection.bindings(bindings)?,
                    None => bindings.to_vec(),
                };
                if returned
                    .iter()
                    .filter_map(|(_, instruction)| match instruction {
                        MirInstruction::Return { value } => Some(*value),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    != vec![Some(row.contract.value)]
                {
                    return Err(freeze("expression/residence-original-pair"));
                }
                if row.root_fault.is_some() {
                    let continuation = row
                        .continuation
                        .as_ref()
                        .ok_or_else(|| freeze("expression/residence-original-pair"))?;
                    let normal = bindings
                        .last()
                        .and_then(|(_, instruction)| match instruction {
                            MirInstruction::Jump {
                                target,
                                edge_args: None,
                            } => Some(*target),
                            _ => None,
                        })
                        .ok_or_else(|| freeze("expression/residence-original-pair"))?;
                    let coordinate = |block| match return_projection {
                        Some(projection) => projection
                            .destination(block)
                            .ok_or_else(|| freeze("expression/residence-original-pair")),
                        None => Ok(block),
                    };
                    if coordinate(normal)?
                        != producer_projection.map_or(Ok(continuation.0), |projection| {
                            projection
                                .destination(continuation.0)
                                .ok_or_else(|| freeze("expression/residence-original-pair"))
                        })?
                    {
                        return Err(freeze("expression/residence-original-pair"));
                    }
                }
                let producer = row.expression().require_unique(|instruction| {
                    instruction.dst_value() == Some(row.contract.value)
                })?;
                let mandatory = match producer_projection {
                    Some(projection) => projection
                        .binding(producer.0, &producer.1)?
                        .ok_or_else(|| freeze("root-exit-binding-drift"))?,
                    None => producer.clone(),
                };
                return Ok(Some(vec![mandatory]));
            }
        }
        let Some(TerminalRelationV1::Value(value)) =
            self.terminal_relation_for_owner_at(owner, exit)
        else {
            return if order.text_return_hold.is_none() && order.return_expression.is_none() {
                Ok(None)
            } else {
                Err(freeze("expression/residence-original-pair"))
            };
        };
        if let TerminalReturnedSourceV1::TextExpression(source) = value.returned() {
            let row = order
                .return_expression
                .as_ref()
                .ok_or_else(|| freeze("expression/residence-original-pair"))?;
            if !std::rc::Rc::ptr_eq(source, &row.contract.source)
                || source.source().owner() != owner
                || source.source().root().statement() != exit
                || !matches!(entry, RootHomeExitEntry::Plain { .. })
            {
                return Err(freeze("expression/residence-original-pair"));
            }
            let residence = row
                .contract
                .residence
                .as_ref()
                .ok_or_else(|| freeze("expression/residence-original-pair"))?;
            row.expression().validate_source(context)?;
            if let Some(function) = function {
                row.validate(context, function, producer_projection)?;
            }
            let hold = order
                .text_return_hold
                .as_ref()
                .ok_or_else(|| freeze("expression/residence-original-pair"))?;
            if !matches!(hold.1, MirInstruction::StringResidenceInstall { value, replaces: None, returning: true }
                if value == row.contract.value)
                || !matches!(bindings.last(), Some((block, MirInstruction::Jump { .. })) if *block == hold.0)
            {
                return Err(freeze("expression/residence-original-pair"));
            }
            let returned = match return_projection {
                Some(projection) => projection.bindings(bindings)?,
                None => bindings.to_vec(),
            };
            if returned
                .iter()
                .filter_map(|(_, instruction)| match instruction {
                    MirInstruction::Return { value } => Some(*value),
                    _ => None,
                })
                .collect::<Vec<_>>()
                != vec![Some(row.contract.value)]
            {
                return Err(freeze("expression/residence-original-pair"));
            }
            let acquisition = row.expression().require_unique(|instruction| {
                matches!(instruction,
                MirInstruction::InvokeNormalResult { dst, .. } if *dst == row.contract.value)
            })?;
            let mut mandatory = Vec::new();
            for (original, projection) in [
                (acquisition, producer_projection),
                (&residence.install, producer_projection),
                (hold, return_projection),
            ] {
                mandatory.push(match projection {
                    Some(projection) => projection
                        .binding(original.0, &original.1)?
                        .ok_or_else(|| freeze("root-exit-binding-drift"))?,
                    None => original.clone(),
                });
            }
            return Ok(Some(mandatory));
        }
        if order.return_expression.is_some() {
            return Err(freeze("expression/residence-original-pair"));
        }
        let TerminalReturnedSourceV1::TextResidence {
            binding,
            acquisition,
        } = value.returned()
        else {
            return if order.text_return_hold.is_none() {
                Ok(None)
            } else {
                Err(freeze("expression/residence-original-pair"))
            };
        };
        let normal = self
            .normal_exit_projection_v1(owner, exit)?
            .ok_or_else(|| freeze("projection-proof-identity"))?;
        let home =
            crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1::Text(acquisition.clone());
        if binding.owner() != owner
            || value.owner() != owner
            || value.return_site() != exit
            || !matches!(entry, RootHomeExitEntry::Plain { .. })
            || normal
                .fault_homes()
                .iter()
                .filter(|candidate| **candidate == home)
                .count()
                != 1
            || normal.homes().contains(&home)
        {
            return Err(freeze("projection-proof-identity"));
        }
        // The source return may read an alias; its sealed acquisition, rather
        // than spelling/binding equality, selects the one installed packet.
        let packet = context.text_home_for_acquisition_v1(acquisition)?;
        if !packet.acquisition().same_source(acquisition) || packet.value() != packet.local_value()
        {
            return Err(freeze("expression/residence-original-pair"));
        }
        let mut mandatory = Vec::new();
        for original in [
            packet.root_acquisition_binding_v1()?,
            packet
                .install_binding()
                .ok_or_else(|| freeze("expression/residence-original-pair"))?,
        ] {
            mandatory.push(match producer_projection {
                Some(projection) => projection
                    .binding(original.0, &original.1)?
                    .ok_or_else(|| freeze("root-exit-binding-drift"))?,
                None => original.clone(),
            });
        }
        if !matches!(mandatory[0].1, MirInstruction::InvokeNormalResult { dst, .. } if dst == packet.value())
            || !matches!(mandatory[1].1, MirInstruction::StringResidenceInstall { value, returning: false, .. } if value == packet.value())
        {
            return Err(freeze("expression/residence-original-pair"));
        }
        let original = order
            .text_return_hold
            .as_ref()
            .ok_or_else(|| freeze("expression/residence-original-pair"))?;
        if !matches!(original.1, MirInstruction::StringResidenceInstall {
            value, replaces: None, returning: true } if value == packet.value())
            || !matches!(bindings.last(), Some((block, MirInstruction::Jump { .. })) if *block == original.0)
        {
            return Err(freeze("expression/residence-original-pair"));
        }
        mandatory.push(match return_projection {
            Some(projection) => projection
                .binding(original.0, &original.1)?
                .ok_or_else(|| freeze("root-exit-binding-drift"))?,
            None => original.clone(),
        });
        let returned = match return_projection {
            Some(projection) => projection.bindings(bindings)?,
            None => bindings.to_vec(),
        };
        let mut returns = returned.iter().filter_map(|(_, row)| match row {
            MirInstruction::Return { value } => Some(value),
            _ => None,
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

#[path = "return_expression_emission.rs"]
mod expression;
pub(crate) use expression::ReturnExpressionContractV1;

impl FinalizedRootSourceHandoffV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn finished_return_expression_call_binding_v1(
        &self,
        owner: FunctionOwnerIdV1,
        exit: &SourceStmtSiteV1,
        site: &OwnedExprSiteV1,
        original: &(BasicBlockId, MirInstruction),
        function: &MirFunction,
        copy: bool,
    ) -> Result<Option<(BasicBlockId, usize)>, String> {
        let exits = self.ledger.root_exits.borrow();
        let order = match exits.get(&(owner, exit.clone().into())) {
            Some(RootHomeExitProgress::Emitted { order, .. })
            | Some(RootHomeExitProgress::Finalized(order)) => order,
            _ => return Err(freeze("finished-terminal-call/exit-missing")),
        };
        let Some(expression) = &order.return_expression else {
            return Ok(None);
        };
        expression.require_source_v1(owner, exit)?;
        if site.owner() != owner
            || !expression.has_call_binding_v1(
                CallValidationContextV1::Finalized(self),
                site,
                original,
                copy,
            )?
        {
            return Err(freeze("finished-terminal-call/original-producer"));
        }
        let (symbol, finished) = self.ledger.finished_binding_for_owner(owner, original)?;
        if copy {
            super::super::finalized_source_projection::find_finished_copy(
                &symbol, &finished, function,
            )
            .map(Some)
        } else {
            super::super::finalized_source_projection::find_finished_producer(
                &symbol, &finished, function,
            )
            .map(Some)
        }
    }
}
