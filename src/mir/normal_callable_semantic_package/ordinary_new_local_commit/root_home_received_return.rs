//! Original Received destination joins its existing producer and exact Return.
use crate::mir::normal_callable_semantic_package::EvaluatedExpressionValueV1;
use super::super::physical_boundary::FinishedBindings;
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::ObjectReturnAcquisitionV1;

impl OrdinaryNewClaimLedgerV1 {
    /// Issue the hold from the original Return relation and installed packet.
    /// The builder emits this same event before entering the cleanup graph.
    pub(crate) fn bind_root_text_return_hold_v1(
        &self, owner: FunctionOwnerIdV1, exit: &SourceStmtSiteV1,
        block: BasicBlockId, value: ValueId,
    ) -> Result<Option<MirInstruction>, String> {
        let Some(TerminalRelationV1::Value(terminal)) = self.terminal_relation_for_owner_at(owner, exit)
            else { return Ok(None) };
        match terminal.returned() {
            TerminalReturnedSourceV1::TextResidence { acquisition, .. } => {
                let packet = CallValidationContextV1::Lowering(self).text_home_for_acquisition_v1(acquisition)?;
                if !packet.acquisition().same_source(acquisition) || packet.value() != value || packet.local_value() != value {
                    return Err(freeze("expression/residence-original-pair"));
                }
            }
            TerminalReturnedSourceV1::TextExpression(source) => {
                let rows = self.root_exits.borrow();
                let Some(RootHomeExitProgress::Emitting(order)) = rows.get(&(owner, exit.clone().into()))
                    else { return Err(freeze("root-exit-record-without-emission")) };
                if !order.return_expression.as_ref().is_some_and(|row|
                    std::rc::Rc::ptr_eq(&row.source, source) && row.value == value) {
                    return Err(freeze("expression/residence-original-pair"));
                }
            }
            _ => return Ok(None),
        }
        if terminal.owner() != owner || terminal.return_site() != exit {
            return Err(freeze("expression/residence-original-pair"));
        }
        let mut exits = self.root_exits.borrow_mut();
        let Some(RootHomeExitProgress::Emitting(order)) = exits.get_mut(&(owner, exit.clone().into()))
            else { return Err(freeze("root-exit-record-without-emission")) };
        if order.text_return_hold.is_some() { return Err(freeze("expression/residence-original-pair")); }
        let hold = MirInstruction::StringResidenceInstall { value, replaces: None, returning: true };
        order.text_return_hold = Some((block, hold.clone()));
        Ok(Some(hold))
    }

    /// A Text Return transfers exactly its original current residence after
    /// Normal cleanup. Its Fault source still owns that SAME acquisition.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_return_expression_producer_v1(
        &self, context: CallValidationContextV1<'_>, owner: FunctionOwnerIdV1,
        exit: &SourceStmtSiteV1, order: &RootHomeCleanupOrderV1, entry: &RootHomeExitEntry,
        bindings: &[(BasicBlockId, MirInstruction)],
        producer_projection: Option<&FinishedBindings>, return_projection: Option<&FinishedBindings>,
        function: Option<&MirFunction>,
    ) -> Result<Option<Vec<(BasicBlockId, MirInstruction)>>, String> {
        if let Some(TerminalRelationV1::I64Scalar(terminal)) = self.terminal_relation_for_owner_at(owner, exit) {
            if let Some(source) = terminal.evaluation() {
                let row = order.return_expression.as_ref().ok_or_else(|| freeze("expression/residence-original-pair"))?;
                if !std::rc::Rc::ptr_eq(source, &row.source) || source.source().owner() != owner
                    || source.source().root().statement() != exit || terminal.owner() != owner
                    || terminal.return_site() != exit || row.residence.is_some() || order.text_return_hold.is_some()
                    || !matches!(entry, RootHomeExitEntry::Plain { .. }) { return Err(freeze("expression/residence-original-pair")); }
                row.expression().validate_source(context)?;
                if let Some(function) = function { row.validate(context, function, producer_projection)?; }
                let returned = match return_projection { Some(projection) => projection.bindings(bindings)?, None => bindings.to_vec() };
                if returned.iter().filter_map(|(_, instruction)| match instruction {
                    MirInstruction::Return { value } => Some(*value), _ => None,
                }).collect::<Vec<_>>() != vec![Some(row.value)] { return Err(freeze("expression/residence-original-pair")); }
                let producer = row.expression().require_unique(|instruction| instruction.dst_value() == Some(row.value))?;
                let mandatory = match producer_projection { Some(projection) => projection.binding(producer.0, &producer.1)?
                    .ok_or_else(|| freeze("root-exit-binding-drift"))?, None => producer.clone() };
                return Ok(Some(vec![mandatory]));
            }
        }
        let Some(TerminalRelationV1::Value(value)) = self.terminal_relation_for_owner_at(owner, exit)
            else {
                return if order.text_return_hold.is_none() && order.return_expression.is_none() { Ok(None) } else { Err(freeze("expression/residence-original-pair")) };
            };
        if let TerminalReturnedSourceV1::TextExpression(source) = value.returned() {
            let row = order.return_expression.as_ref().ok_or_else(|| freeze("expression/residence-original-pair"))?;
            if !std::rc::Rc::ptr_eq(source, &row.source) || source.source().owner() != owner
                || source.source().root().statement() != exit || !matches!(entry, RootHomeExitEntry::Plain { .. }) {
                return Err(freeze("expression/residence-original-pair"));
            }
            let residence = row.residence.as_ref().ok_or_else(|| freeze("expression/residence-original-pair"))?;
            row.expression().validate_source(context)?;
            if let Some(function) = function { row.validate(context, function, producer_projection)?; }
            let hold = order.text_return_hold.as_ref().ok_or_else(|| freeze("expression/residence-original-pair"))?;
            if !matches!(hold.1, MirInstruction::StringResidenceInstall { value, replaces: None, returning: true }
                if value == row.value) || !matches!(bindings.last(), Some((block, MirInstruction::Jump { .. })) if *block == hold.0) {
                return Err(freeze("expression/residence-original-pair"));
            }
            let returned = match return_projection { Some(projection) => projection.bindings(bindings)?, None => bindings.to_vec() };
            if returned.iter().filter_map(|(_, instruction)| match instruction {
                MirInstruction::Return { value } => Some(*value), _ => None,
            }).collect::<Vec<_>>() != vec![Some(row.value)] { return Err(freeze("expression/residence-original-pair")); }
            let acquisition = row.expression().require_unique(|instruction| matches!(instruction,
                MirInstruction::InvokeNormalResult { dst, .. } if *dst == row.value))?;
            let mut mandatory = Vec::new();
            for (original, projection) in [(acquisition, producer_projection), (&residence.install, producer_projection), (hold, return_projection)] {
                mandatory.push(match projection { Some(projection) => projection.binding(original.0, &original.1)?
                    .ok_or_else(|| freeze("root-exit-binding-drift"))?, None => original.clone() });
            }
            return Ok(Some(mandatory));
        }
        if order.return_expression.is_some() { return Err(freeze("expression/residence-original-pair")); }
        let TerminalReturnedSourceV1::TextResidence { binding, acquisition } = value.returned()
            else {
                return if order.text_return_hold.is_none() { Ok(None) } else { Err(freeze("expression/residence-original-pair")) };
            };
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
            || !matches!(mandatory[1].1, MirInstruction::StringResidenceInstall { value, returning: false, .. } if value == packet.value()) {
            return Err(freeze("expression/residence-original-pair"));
        }
        let original = order.text_return_hold.as_ref()
            .ok_or_else(|| freeze("expression/residence-original-pair"))?;
        if !matches!(original.1, MirInstruction::StringResidenceInstall {
            value, replaces: None, returning: true } if value == packet.value())
            || !matches!(bindings.last(), Some((block, MirInstruction::Jump { .. })) if *block == original.0) {
            return Err(freeze("expression/residence-original-pair"));
        }
        mandatory.push(match return_projection {
            Some(projection) => projection.binding(original.0, &original.1)?
                .ok_or_else(|| freeze("root-exit-binding-drift"))?,
            None => original.clone(),
        });
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

/// One Return-owned expression. Residence carries only its additional ownership
/// events; scalar results reuse the same values, reads and independent replay.
#[derive(Debug)]
pub(super) struct ReturnExpressionEmissionV1 {
    pub(super) source: std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1>,
    pub(super) value: ValueId,
    values: Vec<Option<EvaluatedExpressionValueV1>>,
    calls: Vec<std::rc::Rc<crate::mir::normal_callable_semantic_package::EmittedLexicalCallProjectionV1>>,
    path_reads: Vec<std::rc::Rc<crate::mir::builder::ExactLexicalReadV1>>,
    reads: std::collections::BTreeMap<usize, crate::mir::normal_callable_semantic_package::ExpressionReferenceReadV1>,
    entry: (BasicBlockId, MirInstruction), terminal: (BasicBlockId, MirInstruction),
    continuation: (BasicBlockId, MirInstruction),
    residence: Option<ReturnExpressionResidenceV1>,
    original: Vec<(BasicBlockId, MirInstruction)>,
}
#[derive(Debug)]
struct ReturnExpressionResidenceV1 {
    materialize: Option<[(BasicBlockId, MirInstruction); 2]>,
    install: (BasicBlockId, MirInstruction),
}
impl ReturnExpressionEmissionV1 {
    fn expression(&self) -> super::ExpressionPhysicalProjectionRefV1<'_> {
        super::ExpressionPhysicalProjectionRefV1::new(&self.source, &self.values, &self.calls,
            &self.path_reads, &self.reads, &self.original)
    }
    fn validate(&self, context: CallValidationContextV1<'_>, function: &MirFunction,
        projection: Option<&FinishedBindings>) -> Result<(), String> {
        let expression = self.expression();
        expression.validate_source(context)?;
        expression.validate_effective_root_v1(self.value, self.residence.is_some(),
            self.residence.is_some() && self.source.owned_root_node_v1().is_none(),
            self.residence.as_ref().and_then(|row| row.materialize.as_ref()))?;
        let recorded = projection.map(|projection| projection.bindings(&self.original)).transpose()?;
        expression.validate_recorded(recorded.as_deref().unwrap_or(&self.original), projection)?;
        let covered = match &self.residence {
            Some(row) => expression.validate_residence_continuation_v1(context, function, projection,
                &self.entry, &self.terminal, &self.continuation, Some(&row.install), None, row.materialize.as_ref())?,
            None => expression.validate_scalar_continuation_v1(context, function, projection,
                &self.entry, &self.terminal, &self.continuation, None)?,
        };
        if projection.is_none() {
            let mut ordered = expression.ordered_bindings_in_context(context)?;
            if let Some(row) = &self.residence {
                if let Some(pair) = &row.materialize { ordered.extend(pair.iter().cloned()); }
                ordered.push(row.install.clone());
            }
            ordered.push(self.terminal.clone());
            expression.validate_original_scope(context.ledger(), function, &covered, &self.entry,
                &self.terminal, &self.continuation, &ordered)?;
        }
        Ok(())
    }
}
impl RootHomeCleanupOrderV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn return_expression_bindings_v1(&self)
        -> &[(BasicBlockId, MirInstruction)] {
        self.return_expression.as_ref().map_or(&[], |row| row.original.as_slice())
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn text_expression_install_v1(&self)
        -> Option<&(BasicBlockId, MirInstruction)> {
        self.return_expression.as_ref().and_then(|row| row.residence.as_ref()).map(|row| &row.install)
    }
}
impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn retain_return_expression_v1(&self,
        source: std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1>,
        value: ValueId, values: Vec<Option<EvaluatedExpressionValueV1>>,
        calls: Vec<std::rc::Rc<crate::mir::normal_callable_semantic_package::EmittedLexicalCallProjectionV1>>,
        path_reads: Vec<std::rc::Rc<crate::mir::builder::ExactLexicalReadV1>>,
        reads: std::collections::BTreeMap<usize, crate::mir::normal_callable_semantic_package::ExpressionReferenceReadV1>,
        entry: (BasicBlockId, MirInstruction), terminal: (BasicBlockId, MirInstruction),
        continuation: (BasicBlockId, MirInstruction), materialize: Option<[(BasicBlockId, MirInstruction); 2]>,
        original: Vec<(BasicBlockId, MirInstruction)>, function: &MirFunction,
    ) -> Result<(), String> {
        let owner = source.source().owner(); let site = source.source().root().statement();
        if self.return_expression_evaluation_v1(owner, site)?.is_none_or(|original|
            !std::rc::Rc::ptr_eq(&original, &source)) { return Err(freeze("expression/residence-original-pair")); }
        let installs: Vec<_> = original.iter().filter(|(_, row)| matches!(row,
            MirInstruction::StringResidenceInstall { .. })).collect();
        use crate::mir::resolved_value_profile::{expression_projection::ExpressionLeafClassV1 as Leaf,
            expression_class::ExpressionValueClassV1 as Class};
        let residence = match source.root_class() {
            Leaf::Value(Class::String) => {
                if installs.len() != 1 || !matches!(installs[0].1, MirInstruction::StringResidenceInstall {
                    value: installed, replaces: None, returning: false } if installed == value) {
                    return Err(freeze("expression/residence-original-pair"));
                }
                Some(ReturnExpressionResidenceV1 { install: (*installs[0]).clone(), materialize })
            }
            Leaf::Value(Class::I64) if installs.is_empty() && materialize.is_none() => None,
            _ => return Err(freeze("expression/residence-original-pair")),
        };
        let packet = ReturnExpressionEmissionV1 { residence, source: source.clone(), value,
            values, calls, path_reads, reads, entry, terminal, continuation, original };
        packet.validate(CallValidationContextV1::Lowering(self), function, None)?;
        let mut rows = self.root_exits.borrow_mut();
        let Some(RootHomeExitProgress::Prepared(order)) = rows.get_mut(&(owner, site.clone().into()))
            else { return Err(freeze("root-exit-record-without-prepare")) };
        if order.return_expression.is_some() { return Err(freeze("expression/residence-original-pair")); }
        order.return_expression = Some(Box::new(packet));
        Ok(())
    }
}
