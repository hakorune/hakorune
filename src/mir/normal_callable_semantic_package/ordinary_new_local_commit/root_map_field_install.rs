//! Original Map field set: passive key, exact acquired child, Normal transfer.
use super::map_key_read::FieldMapKeyReadEmissionV1;
use super::*;
use crate::mir::builder::ExactLexicalReadV1;
use crate::mir::normal_callable_semantic_package::{
    EvaluatedExpressionValueV1, ExpressionReferenceReadV1,
};
use crate::mir::resolved_semantics::home_new_prefix::{MapFieldInstallV1, MapHomeObservation};
use std::{collections::BTreeMap, rc::Rc};
type Binding = (BasicBlockId, MirInstruction);
type Finished = super::super::physical_boundary::FinishedBindings;

/// Prepared original operands, before the sole writer emits the checked set.
#[derive(Debug)]
pub(crate) struct PreparedMapFieldInstallV1 {
    frame: Rc<MapFieldInstallV1>,
    key: FieldMapKeyReadEmissionV1,
    candidate: Option<ExactLexicalReadV1>,
}
impl PreparedMapFieldInstallV1 {
    pub(in crate::mir) fn issue(
        frame: Rc<MapFieldInstallV1>,
        receiver: ExactLexicalReadV1,
        candidate: Option<ExactLexicalReadV1>,
        values: Vec<Option<EvaluatedExpressionValueV1>>,
        reads: BTreeMap<usize, ExpressionReferenceReadV1>,
        entry: Binding,
        terminal: Binding,
        continuation: Binding,
        original: Vec<Binding>,
        ledger: &OrdinaryNewClaimLedgerV1,
        function: &MirFunction,
    ) -> Result<Self, String> {
        let key = FieldMapKeyReadEmissionV1::issue(
            Rc::clone(frame.key()),
            receiver,
            values,
            reads,
            entry,
            terminal,
            continuation,
            original,
            ledger,
        )?;
        let prepared = Self {
            frame,
            key,
            candidate,
        };
        prepared.operation(CallValidationContextV1::Lowering(ledger), function)?;
        Ok(prepared)
    }
    pub(in crate::mir) fn operation(
        &self,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
    ) -> Result<InvokeOperation, String> {
        let ledger = context.ledger();
        let original = ledger
            .map_field_install_source_v1(self.frame.site())?
            .ok_or_else(|| freeze("map-get-source-missing"))?;
        if !Rc::ptr_eq(&original, &self.frame) || !Rc::ptr_eq(self.frame.key(), &self.key.key) {
            return Err(freeze("map-get-shape"));
        }
        let source = self.frame.source();
        let installed = source.installed_local_v1(self.frame.site());
        let set = match installed {
            Some((binding, construction, set))
                if self.frame.binding() == Some(binding)
                    && self.frame.acquisition().map(|site| site.site()) == Some(construction.site())
                    && set.arguments().len() == 2 => set,
            None if self.frame.binding().is_none() => source
                .deleted_v1(self.frame.site())
                .filter(|call| call.arguments().len() == 1)
                .ok_or_else(|| freeze("map-get-shape"))?,
            _ => return Err(freeze("map-get-shape")),
        };
        if self.candidate.is_some() != installed.is_some()
            || self.key.values.len() != 1
            || self.key.key.source().nodes().len() != 1
            || self.key.key.source().owner() != set.owner()
            || self.key.key.source().nodes()[0].site() != set.arguments()[0].site()
            || !matches!(self.key.key.source().root(), crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument { call, ordinal: 0, .. }
                if std::sync::Arc::ptr_eq(call, set))
        {
            return Err(freeze("map-get-shape"));
        }
        let base = source.receiver_v1().require_receiver_at_v1(
            context,
            function,
            &self.key.receiver,
            self.frame.receiver_read(),
        )?;
        let key = self.key.values[0]
            .as_ref()
            .map(|value| value.as_operand())
            .transpose()?
            .cloned()
            .filter(|key| key.is_text())
            .ok_or_else(|| freeze("map-get-key-drift"))?;
        let field = source.receiver_v1().field();
        let object = source.payload_object_v1();
        let (Some(candidate), Some(binding), Some(acquisition)) =
            (&self.candidate, self.frame.binding(), self.frame.acquisition())
        else {
            return Ok(InvokeOperation::Map(
                crate::mir::instruction::MapInvokeOperation::FieldDeleteIndexed {
                    field, base, key, object,
                },
            ));
        };
        let rows = ledger.local_commits.borrow();
        let Some(LocalCommitV1::Ordinary(row)) = rows.get(acquisition) else {
            return Err(freeze("map-get-source-missing"));
        };
        if row.binding != binding || row.object != source.payload_object_v1() {
            return Err(freeze("map-get-shape"));
        }
        let local = row
            .emission
            .local()
            .ok_or_else(|| freeze("map-get-source-missing"))?;
        candidate.loan_forwarded_copies(binding, local)?;
        let value = candidate
            .value_for(set.owner(), set.arguments()[1].site().node(), binding)
            .map_err(|error| error.to_string())?;
Ok(InvokeOperation::Map(
            crate::mir::instruction::MapInvokeOperation::FieldInstallIndexed {
                field,
                base,
                key,
                object,
                value,
            },
        ))
    }
}

/// Complete physical packet; source prefix membership and final closeout share it.
#[derive(Debug)]
pub(crate) struct MapFieldInstallEmissionV1 {
    prepared: PreparedMapFieldInstallV1,
    invoke: Binding,
    normal: Binding,
    end: Binding,
    continuation: Binding,
    prior_faults: Vec<Binding>,
    normal_faults: Vec<Binding>,
    original: Vec<Binding>,
}
impl MapFieldInstallEmissionV1 {
    pub(crate) fn issue(
        prepared: PreparedMapFieldInstallV1,
        invoke: Binding,
        normal: Binding,
        end: Binding,
        continuation: Binding,
        prior_faults: Vec<Binding>,
        normal_faults: Vec<Binding>,
        ledger: &OrdinaryNewClaimLedgerV1,
        function: &MirFunction,
    ) -> Result<Self, String> {
        let mut original = prepared.key.copy_dependencies()?;
        if let Some(candidate) = &prepared.candidate {
            original.extend_from_slice(candidate.original_copy_dependencies_v1()?);
        }
        for row in prepared
            .key
            .original
            .iter()
            .chain([&invoke, &normal, &end, &continuation])
            .chain(prior_faults.iter())
            .chain(normal_faults.iter())
        {
            if !original.contains(row) {
                original.push(row.clone());
            }
        }
        let packet = Self {
            prepared,
            invoke,
            normal,
            end,
            continuation,
            prior_faults,
            normal_faults,
            original,
        };
        packet.validate_paths(CallValidationContextV1::Lowering(ledger), function, None)?;
        Ok(packet)
    }
    pub(crate) fn frame(&self) -> &Rc<MapFieldInstallV1> {
        &self.prepared.frame
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn candidate_value(
        &self,
    ) -> Result<ValueId, String> {
        match self.invoke.1 {
            MirInstruction::Invoke {
                operation:
                    InvokeOperation::Map(
                        crate::mir::instruction::MapInvokeOperation::FieldInstallIndexed {
                            value,
                            ..
                        },
                    ),
                ..
            } => Ok(value),
            _ => Err(freeze("map-get-shape")),
        }
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn normal(
        &self,
    ) -> &Binding {
        &self.normal
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn end(
        &self,
    ) -> &Binding {
        &self.end
    }
    pub(crate) fn bindings(&self) -> &[Binding] {
        &self.original
    }
    pub(crate) fn invoke(&self) -> &Binding {
        &self.invoke
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_recorded(
        &self,
        rows: &[Binding],
        finishing: Option<&Finished>,
    ) -> Result<(), String> {
        for row in &self.original {
            let actual = match finishing {
                Some(p) => p.binding(row.0, &row.1)?,
                None => Some(row.clone()),
            };
            if actual
                .as_ref()
                .is_some_and(|row| rows.iter().filter(|actual| *actual == row).count() != 1)
            {
                return Err(freeze("map-get-binding-drift"));
            }
        }
        Ok(())
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_paths(
        &self,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        finishing: Option<&Finished>,
    ) -> Result<(), String> {
        let operation = self.prepared.operation(context, function)?;
        let outcome = self.prepared.key.validate(
            context,
            function,
            finishing,
            &operation,
            &self.invoke,
            &self.normal,
        )?;
        let map = |row: &Binding| match finishing {
            Some(p) => p
                .binding(row.0, &row.1)?
                .ok_or_else(|| freeze("map-get-binding-missing")),
            None => Ok(row.clone()),
        };
        let invoke = map(&self.invoke)?;
        let end = map(&self.end)?;
        let continuation = finishing
            .map(|p| p.binding(self.continuation.0, &self.continuation.1))
            .transpose()?
            .unwrap_or_else(|| Some(self.continuation.clone()));
        let MirInstruction::Invoke { fault_frame, .. } = invoke.1 else {
            return Err(freeze("map-get-shape"));
        };
        let MirInstruction::Invoke {
            operation:
                InvokeOperation::Map(crate::mir::instruction::MapInvokeOperation::EndOutcome {
                    outcome: actual,
                }),
            fault_frame: end_frame,
            normal_landing,
            fault_landing,
            ..
        } = end.1
        else {
            return Err(freeze("map-get-shape"));
        };
        if actual != outcome
            || fault_frame != end_frame
            || normal_landing == fault_landing
            || !matches!(
                self.continuation.1,
                MirInstruction::Jump {
                    edge_args: None,
                    ..
                }
            )
            || finishing.map_or(Some(self.continuation.0), |p| {
                p.destination(self.continuation.0)
            }) != Some(normal_landing)
        {
            return Err(freeze("map-get-binding-drift"));
        }
        let mut ordered = vec![map(&self.normal)?, end.clone()];
        if let Some(continuation) = continuation {
            ordered.push(continuation);
        }
        super::ExpressionPhysicalProjectionRefV1::validate_normal_order_v1(function, &ordered)?;
        for row in &self.original {
            if !super::super::physical_boundary::check_binding(function, finishing, row.0, &row.1)?
            {
                return Err(freeze("map-get-binding-drift"));
            }
        }
        let faults = |rows: &[Binding]| {
            finishing
                .map(|p| p.bindings(rows))
                .unwrap_or_else(|| Ok(rows.to_vec()))
        };
        let ledger = context.ledger();
        ledger.validate_source_field_fault_v1(
            self.frame().site().owner(),
            function,
            &invoke,
            &faults(&self.prior_faults)?,
            self.frame().prior_homes(),
        )?;
        ledger.validate_source_field_fault_v1(
            self.frame().site().owner(),
            function,
            &end,
            &faults(&self.normal_faults)?,
            self.frame().normal_homes(),
        )
    }
}

impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn map_field_install_source_v1(
        &self,
        site: &OwnedExprSiteV1,
    ) -> Result<Option<Rc<MapFieldInstallV1>>, String> {
        let Some(flow) = self
            .completion_for_owner(site.owner())
            .and_then(|c| c.cleanup().root_flow())
        else {
            return Ok(None);
        };
        let mut matching = flow.maps().iter().filter_map(|row| match row {
            MapHomeObservation::FieldInstall(frame) if frame.site() == site => Some(frame),
            _ => None,
        });
        let frame = matching.next().cloned();
        if matching.next().is_some() {
            return Err(freeze("map-source-duplicate"));
        }
        Ok(frame)
    }
}

/// One original installed-local Return, retaining an owner-neutral SSA alias.
/// The earlier candidate still has New/Birth/Fault uses and is never a borrowed
/// result root. The alias exists only after the checked install/old-End Normal.
#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) struct InstalledMapReturnEmissionV1
{
    terminal: crate::mir::resolved_semantics::home_new_prefix::TerminalValueReturnV1,
    install: Rc<MapFieldInstallEmissionV1>,
    alias: Binding,
}
impl InstalledMapReturnEmissionV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn install(
        &self,
    ) -> &MapFieldInstallEmissionV1 {
        &self.install
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn alias(
        &self,
    ) -> &Binding {
        &self.alias
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_return(
        &self,
        entry: &RootHomeExitEntry,
        bindings: &[Binding],
        projection: Option<&Finished>,
    ) -> Result<(), String> {
        if !matches!(entry, RootHomeExitEntry::Plain { .. }) {
            return Err(freeze("borrowed-return/original-exit"));
        }
        let alias = projection
            .map(|p| p.binding(self.alias.0, &self.alias.1))
            .transpose()?
            .unwrap_or_else(|| Some(self.alias.clone()))
            .ok_or_else(|| freeze("borrowed-return/return-missing"))?;
        let value = alias
            .1
            .dst_value()
            .ok_or_else(|| freeze("borrowed-return/return-missing"))?;
        let rows = projection.map(|p| p.bindings(bindings)).transpose()?;
        let mut returns =
            rows.as_deref()
                .unwrap_or(bindings)
                .iter()
                .filter_map(|(_, row)| match row {
                    MirInstruction::Return { value } => Some(value),
                    _ => None,
                });
        if returns.next() != Some(&Some(value)) || returns.next().is_some() {
            return Err(freeze("borrowed-return/return-missing"));
        }
        Ok(())
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate(
        &self,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        finishing: Option<&Finished>,
        entry: &RootHomeExitEntry,
    ) -> Result<(), String> {
        let source = context
            .ledger()
            .terminal_relation_for_owner_at(self.terminal.owner(), self.terminal.return_site());
        if !matches!(source, Some(TerminalRelationV1::Value(original)) if original == &self.terminal)
            || !matches!(self.terminal.returned(), TerminalReturnedSourceV1::StoredMapInstalled(frame)
                if Rc::ptr_eq(frame, self.install.frame()))
            || entry
                .local_call_groups()
                .iter()
                .filter_map(|group| group.map_field_install())
                .filter(|packet| std::ptr::eq(*packet, self.install.as_ref()))
                .count()
                != 1
        {
            return Err(freeze("borrowed-return/original-contract"));
        }
        self.install.validate_paths(context, function, finishing)?;
        let candidate = self.install.candidate_value()?;
        if !matches!(self.alias.1, MirInstruction::Copy { src, dst } if src == candidate && src != dst)
        {
            return Err(freeze("borrowed-return/original-contract"));
        }
        let map = |row: &Binding| match finishing {
            Some(p) => p
                .binding(row.0, &row.1)?
                .ok_or_else(|| freeze("map-get-binding-missing")),
            None => Ok(row.clone()),
        };
        let alias = match finishing {
            Some(p) => p
                .relation_binding(function, &self.alias)?
                .ok_or_else(|| freeze("borrowed-return/return-missing"))?,
            None => self.alias.clone(),
        };
        if !super::super::physical_boundary::check_binding(function, None, alias.0, &alias.1)? {
            return Err(freeze("map-get-binding-drift"));
        }
        let end = map(self.install.end())?;
        let MirInstruction::Invoke { normal_landing, .. } = end.1 else {
            return Err(freeze("map-get-shape"));
        };
        if !crate::mir::verification::utils::compute_dominators(function)
            .dominates(normal_landing, alias.0)
        {
            return Err(freeze("borrowed-return/original-contract"));
        }
        Ok(())
    }
}
impl RootHomeCleanupOrderV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn installed_map_return(
        &self,
    ) -> Option<&InstalledMapReturnEmissionV1> {
        self.installed_map_return.as_ref()
    }
}
impl OrdinaryNewClaimLedgerV1 {
    /// Lend an existing checked install only for this original Return source.
    /// The evaluated raw value is corroboration, never an eligibility issuer.
    pub(crate) fn installed_map_return_candidate_v1(
        &self,
        owner: FunctionOwnerIdV1,
        exit: &SourceStmtSiteV1,
        value: ValueId,
        function: &MirFunction,
    ) -> Result<Option<Rc<MapFieldInstallEmissionV1>>, String> {
        let Some(TerminalRelationV1::Value(terminal)) =
            self.terminal_relation_for_owner_at(owner, exit)
        else {
            return Ok(None);
        };
        let TerminalReturnedSourceV1::StoredMapInstalled(frame) = terminal.returned() else {
            return Ok(None);
        };
        let context = CallValidationContextV1::Lowering(self);
        let packet = context.with_local_call_group_tree_v2(owner, |groups| {
            let mut packets = groups
                .iter()
                .filter_map(|group| group.map_field_install_rc())
                .filter(|packet| Rc::ptr_eq(packet.frame(), frame));
            let packet = packets
                .next()
                .ok_or_else(|| freeze("map-get-source-missing"))?;
            if packets.next().is_some() {
                return Err(freeze("map-get-shape"));
            }
            Ok(packet)
        })?;
        packet.validate_paths(context, function, None)?;
        if terminal.owner() != owner
            || terminal.return_site() != exit
            || packet.candidate_value()? != value
        {
            return Err(freeze("borrowed-return/original-contract"));
        }
        Ok(Some(packet))
    }
    pub(crate) fn record_installed_map_return_alias_v1(
        &self,
        owner: FunctionOwnerIdV1,
        exit: &SourceStmtSiteV1,
        install: Rc<MapFieldInstallEmissionV1>,
        alias: Binding,
    ) -> Result<(), String> {
        let Some(TerminalRelationV1::Value(terminal)) =
            self.terminal_relation_for_owner_at(owner, exit)
        else {
            return Err(freeze("borrowed-return/original-contract"));
        };
        if !matches!(terminal.returned(), TerminalReturnedSourceV1::StoredMapInstalled(frame) if Rc::ptr_eq(frame, install.frame()))
            || !matches!(alias.1, MirInstruction::Copy { src, dst } if src == install.candidate_value()? && src != dst)
        {
            return Err(freeze("borrowed-return/original-contract"));
        }
        let packet = InstalledMapReturnEmissionV1 {
            terminal: terminal.clone(),
            install,
            alias,
        };
        let mut exits = self.root_exits.borrow_mut();
        let Some(RootHomeExitProgress::Prepared(order)) =
            exits.get_mut(&(owner, exit.clone().into()))
        else {
            return Err(freeze("root-exit-record-without-prepare"));
        };
        if order.installed_map_return.is_some() {
            return Err(freeze("borrowed-return/original-exit"));
        }
        order.installed_map_return = Some(packet);
        Ok(())
    }
}
