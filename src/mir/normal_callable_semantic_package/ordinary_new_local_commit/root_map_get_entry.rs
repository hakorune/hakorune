//! Root exit progress for the readable-Map terminal.
//!
//! The MapGet entry retains the already-emitted checked read exactly like a
//! Call ingress: the source relation owns the key and receiver class; this
//! file only records and re-verifies physical coordinates.
use super::*;

impl OrdinaryNewClaimLedgerV1 {
    /// Claim this exit's lifecycle binding groups into the MapGet entry and
    /// seal the emitted read, mirroring `record_root_call_exit`.
    pub(crate) fn record_root_map_get_exit(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceStmtSiteV1,
        invoke: (BasicBlockId, MirInstruction),
        projection: (BasicBlockId, MirInstruction),
        frame: (BasicBlockId, MirInstruction),
        origins: Vec<(RootHomeReleaseOriginV1, BasicBlockId, MirInstruction)>,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
        stored: Option<Box<StoredMapGetReturnEmissionV1>>, function: &MirFunction,
    ) -> Result<(), String> {
        let local_bindings = self.select_local_call_binding_groups(owner, site)?;
        // A surrounding Bool arm is still being emitted here. Validate this
        // producer now; the existing exit/final consumer owns whole-group seals.
        if let Some(stored) = &stored {
            let kind = stored.validate(CallValidationContextV1::Lowering(self), function, None, &invoke, &projection)?;
            super::super::root_cleanup_graph::call::ingress(function, &bindings, &invoke, &projection, kind)?;
        }
        self.record_root_home_exit_with_entry(
            owner,
            site,
            origins,
            bindings,
            RootHomeExitEntry::MapGet {
                stored, local_bindings,
                invoke,
                projection,
                frame,
            },
        )
    }

    /// Artifact seal evidence: this exact exit site holds an emitted MapGet
    /// entry. Child completeness is separately enforced by finalized child
    /// validation; this flag only answers the seal-time root question.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn terminal_map_get_return_emitted_at(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceStmtSiteV1,
    ) -> bool {
        matches!(
            self.root_exits.borrow().get(&(owner, site.clone().into())),
            Some(RootHomeExitProgress::Emitted {
                entry: RootHomeExitEntry::MapGet { .. },
                ..
            })
        )
    }

    /// Re-verify one emitted MapGet entry against its source relation and
    /// the finished function. Mirrors `validate_call_entry`'s Call arm:
    /// relation presence, operation shape, frame identity, binding coverage,
    /// local-call groups, and the shared cleanup ingress graph.
    pub(super) fn validate_map_get_entry(
        &self,
        owner: FunctionOwnerIdV1,
        exit: &SourceStmtSiteV1,
        function: &MirFunction,
        finishing: Option<&super::super::physical_boundary::FinishedBindings>,
        local_bindings: &[super::super::RootLifecyclePrefixBindingGroupV1],
        invoke: &(BasicBlockId, MirInstruction),
        projection: &(BasicBlockId, MirInstruction),
        frame: &(BasicBlockId, MirInstruction),
        cleanup: &[(BasicBlockId, MirInstruction)],
        stored: Option<&StoredMapGetReturnEmissionV1>, context: CallValidationContextV1<'_>,
    ) -> Result<(), String> {
        let expected = if let Some(stored) = stored {
            if stored.source.terminal().owner() != owner || stored.source.terminal().return_site() != exit {
                return Err(freeze("map-get-source-missing"));
            }
            stored.validate(context, function, finishing, invoke, projection)?
        } else {
        let relation = self
            .terminal_map_get_return_for_owner_at(owner, exit)
            .ok_or_else(|| freeze("map-get-source-missing"))?;
        let MirInstruction::Invoke {
            operation:
                InvokeOperation::Map(crate::mir::instruction::MapInvokeOperation::CheckedGetI64 {
                    utf8,
                    ..
                }),
            ..
        } = &invoke.1
        else {
            return Err(freeze("map-get-shape"));
        };
        if utf8.as_str() != relation.key() {
            return Err(freeze("map-get-key-drift"));
        }
        crate::mir::instruction::InvokeCallResultKind::I64
        };
        let MirInstruction::Invoke { fault_frame, .. } = &invoke.1 else { return Err(freeze("map-get-shape")) };
        // The recorded instruction pins the mode — `RootOwned` for the root,
        // `Borrowed` for a cataloged child like the readable-Map callee —
        // matching `map.rs`'s call row; `check_binding` below re-verifies it.
        if !matches!(&frame.1,
            MirInstruction::FaultFrameEnter { dst, .. } if dst == fault_frame)
        {
            return Err(freeze("map-get-frame-drift"));
        }
        for (id, instruction) in [invoke, projection, frame] {
            if !super::super::physical_boundary::check_binding(
                function,
                finishing,
                *id,
                instruction,
            )? {
                return Err(freeze("map-get-binding-drift"));
            }
        }
        self.check_local_call_binding_groups_in_context_v1(context, owner, exit, function, finishing, local_bindings, &[])?;
        let mapped = |binding: &(BasicBlockId, MirInstruction)| match finishing {
            Some(p) => p
                .binding(binding.0, &binding.1)?
                .ok_or_else(|| freeze("map-get-binding-missing")),
            None => Ok(binding.clone()),
        };
        let cleanup = match finishing {
            Some(p) => p.bindings(cleanup)?,
            None => cleanup.to_vec(),
        };
        super::super::root_cleanup_graph::call::ingress(
            function,
            &cleanup,
            &mapped(invoke)?,
            &mapped(projection)?,
            expected,
        )?;
        Ok(())
    }
}

/// Return policy is separate from the shared get read. Only this wrapper
/// lends a callable result support (including the producer's key ordinal).
#[derive(Debug)]
pub(crate) struct StoredMapGetReturnEmissionV1 {
    source: crate::mir::normal_callable_semantic_package::StoredMapGetReturnSourceLoanV1,
    read: StoredMapGetReadEmissionV1,
}
impl StoredMapGetReturnEmissionV1 {
    pub(crate) fn issue(source: crate::mir::normal_callable_semantic_package::StoredMapGetReturnSourceLoanV1,
        receiver: crate::mir::builder::ExactLexicalReadV1,
        values: Vec<Option<crate::mir::instruction::StringInvokeOperand>>,
        reads: std::collections::BTreeMap<usize, super::ExpressionReferenceReadV1>,
        entry: (BasicBlockId, MirInstruction), terminal: (BasicBlockId, MirInstruction),
        continuation: (BasicBlockId, MirInstruction), original: Vec<(BasicBlockId, MirInstruction)>,
        ledger: &OrdinaryNewClaimLedgerV1, function: &MirFunction,
    ) -> Result<Self, String> {
        let read = StoredMapGetReadEmissionV1 {
            source: source.get_source().clone(), key: std::rc::Rc::clone(source.parts().1),
            receiver, values, reads, entry, terminal, continuation, original,
        };
        let packet = Self { source, read };
        packet.operation(CallValidationContextV1::Lowering(ledger), function)?;
        packet.read.expression().validate_source(CallValidationContextV1::Lowering(ledger))?;
        Ok(packet)
    }
    pub(super) fn bindings(&self) -> &[(BasicBlockId, MirInstruction)] { &self.read.original }
    pub(super) fn copy_dependencies(&self) -> Result<Vec<(BasicBlockId, MirInstruction)>, String> {
        self.read.copy_dependencies()
    }
    pub(crate) fn operation(&self, context: CallValidationContextV1<'_>, function: &MirFunction) -> Result<InvokeOperation, String> {
        let terminal = self.source.terminal();
        let retained = context.ledger().stored_map_get_return_source_v1(terminal.owner(), terminal.return_site())?
            .ok_or_else(|| freeze("map-get-source-missing"))?;
        if retained.terminal() != terminal || retained.get_source() != &self.read.source
            || !std::rc::Rc::ptr_eq(retained.parts().1, &self.read.key) {
            return Err(freeze("map-get-shape"));
        }
        self.read.operation(context, function)
    }
    fn validate(&self, context: CallValidationContextV1<'_>, function: &MirFunction,
        finishing: Option<&super::super::physical_boundary::FinishedBindings>,
        invoke: &(BasicBlockId, MirInstruction), projection: &(BasicBlockId, MirInstruction),
    ) -> Result<crate::mir::instruction::InvokeCallResultKind, String> {
        self.operation(context, function)?;
        self.read.validate(context, function, finishing, invoke, projection)?;
        Ok(crate::mir::instruction::InvokeCallResultKind::BorrowedNullableHandle {
            support: self.source.parts().0.result_support_v1(),
        })
    }
}

/// Physical observations of the SAME original get, passive key and receiver.
/// This read owns no Return policy, callable ordinal or payload Home.
#[derive(Debug)]
struct StoredMapGetReadEmissionV1 {
    source: crate::mir::normal_callable_semantic_package::StoredMapGetSourceLoanV1,
    key: std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1>,
    receiver: crate::mir::builder::ExactLexicalReadV1,
    values: Vec<Option<crate::mir::instruction::StringInvokeOperand>>,
    reads: std::collections::BTreeMap<usize, super::ExpressionReferenceReadV1>,
    entry: (BasicBlockId, MirInstruction),
    terminal: (BasicBlockId, MirInstruction), continuation: (BasicBlockId, MirInstruction),
    original: Vec<(BasicBlockId, MirInstruction)>,
}
impl StoredMapGetReadEmissionV1 {
    fn copy_dependencies(&self) -> Result<Vec<(BasicBlockId, MirInstruction)>, String> {
        let mut rows = self.receiver.original_copy_dependencies_v1()?.to_vec();
        for read in self.reads.values() { rows.extend_from_slice(read.original_copy_dependencies_v1()?); }
        Ok(rows)
    }
    fn expression(&self) -> super::ExpressionPhysicalProjectionRefV1<'_> {
        super::ExpressionPhysicalProjectionRefV1::new(&self.key, &self.values, &[], &[], &self.reads, &self.original)
    }
    fn operation(&self, context: CallValidationContextV1<'_>, function: &MirFunction) -> Result<InvokeOperation, String> {
        self.source.recheck_v1(context.ledger())?;
        let get = self.source.get();
        if self.values.len() != 1 || self.key.source().nodes().len() != 1
            || get.arguments().len() != 1 || self.key.source().owner() != get.owner()
            || self.key.source().nodes()[0].site() != get.arguments()[0].site()
            || !matches!(self.key.source().root(), crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument { call, ordinal: 0, .. }
                if std::sync::Arc::ptr_eq(call, get)) {
            return Err(freeze("map-get-shape"));
        }
        let base = self.source.receiver().require_receiver_at_v1(context, function, &self.receiver,
            self.source.receiver().receiver_site())?;
        let key = self.values[0].clone().filter(|key| key.is_text()).ok_or_else(|| freeze("map-get-key-drift"))?;
        Ok(InvokeOperation::Map(crate::mir::instruction::MapInvokeOperation::FieldGetIndexed {
            field: self.source.receiver().field(), base, key, object: self.source.storage().payload_object_v1(),
        }))
    }
    fn validate(&self, context: CallValidationContextV1<'_>, function: &MirFunction,
        finishing: Option<&super::super::physical_boundary::FinishedBindings>,
        invoke: &(BasicBlockId, MirInstruction), projection: &(BasicBlockId, MirInstruction),
    ) -> Result<crate::mir::ValueId, String> {
        let ledger = context.ledger();
        if !matches!(&invoke.1, MirInstruction::Invoke { operation, .. } if *operation == self.operation(context, function)?) {
            return Err(freeze("map-get-shape"));
        }
        // Both coordinates belong to this exact Invoke. A Local consumer may
        // continue after Normal, so root-exit graph checks cannot supply this.
        let MirInstruction::Invoke { normal_landing, fault_landing, .. } = invoke.1
            else { return Err(freeze("map-get-shape")) };
        let MirInstruction::InvokeNormalResult { invoke_block, dst: value } = projection.1
            else { return Err(freeze("map-get-binding-drift")) };
        if invoke_block != invoke.0 || projection.0 != normal_landing
            || normal_landing == fault_landing || normal_landing == invoke.0 {
            return Err(freeze("map-get-binding-drift"));
        }
        for (block, row) in [invoke, projection] {
            if !super::super::physical_boundary::check_binding(function, finishing, *block, row)? {
                return Err(freeze("map-get-binding-drift"));
            }
        }
        let expression = self.expression();
        let MirInstruction::Invoke { fault_frame, .. } = invoke.1 else { return Err(freeze("map-get-frame-drift")) };
        let key_frame = expression.require_unique(|row| matches!(row, MirInstruction::FaultFrameEnter { dst, .. } if *dst == fault_frame))?;
        let actual_frame = match finishing { Some(p) => p.binding(key_frame.0, &key_frame.1)?
            .ok_or_else(|| freeze("map-get-frame-drift"))?, None => key_frame.clone() };
        if !super::super::physical_boundary::check_binding(function, None, actual_frame.0, &actual_frame.1)? {
            return Err(freeze("map-get-frame-drift"));
        }
        let actual = finishing.map(|p| p.bindings(&self.original)).transpose()?;
        expression.validate_recorded(actual.as_deref().unwrap_or(&self.original), finishing)?;
        let covered = expression.validate_local_continuation_v1(context, function, finishing,
            &self.entry, &self.terminal, &self.continuation, None, None)?;
        let actual_invoke = match finishing { Some(p) => p.binding(invoke.0, &invoke.1)?
            .ok_or_else(|| freeze("map-get-binding-missing"))?, None => invoke.clone() };
        let MirInstruction::Jump { target, edge_args: None } = self.continuation.1
            else { return Err(freeze("map-get-binding-drift")) };
        let target = match finishing { Some(p) => p.destination(target)
            .ok_or_else(|| freeze("map-get-binding-missing"))?, None => target };
        if target != actual_invoke.0 { return Err(freeze("map-get-binding-drift")); }
        for original in self.receiver.original_copy_dependencies_v1()? {
            let actual = match finishing { Some(p) => p.relation_binding(function, original)?, None => Some(original.clone()) };
            if let Some((block, row)) = actual {
                if !super::super::physical_boundary::check_binding(function, None, block, &row)? {
                    return Err(freeze("map-get-binding-drift"));
                }
            }
        }
        if finishing.is_none() {
            let mut ordered = expression.ordered_bindings_in_context(context)?;
            ordered.push(self.terminal.clone());
            expression.validate_original_scope(ledger, function, &covered, &self.entry, &self.terminal, &self.continuation, &ordered)?;
        }
        Ok(value)
    }
}

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn visit_stored_map_get_groups_v1(
        &self, owner: FunctionOwnerIdV1, context: CallValidationContextV1<'_>,
        function: &MirFunction, finishing: &super::super::physical_boundary::FinishedBindings,
        module: &crate::mir::MirModule,
        mut visit: impl FnMut((BasicBlockId, MirInstruction)) -> Result<(), String>,
    ) -> Result<(), String> {
        let exits = self.root_exits.borrow();
        for ((actual, exit), row) in exits.iter().filter(|((actual, _), _)| *actual == owner) {
            let RootHomeExitProgress::Emitted { entry: RootHomeExitEntry::MapGet {
                stored: Some(stored), local_bindings, invoke, projection, frame,
            }, bindings, .. } = row else { continue };
            let exit = exit.explicit_site().ok_or_else(|| freeze("map-get-source-missing"))?;
            if let CallValidationContextV1::Finalized(source) = context {
                stored.source.get_source().receiver().require_finished_birth_v1(source, module)?;
            }
            self.validate_map_get_entry(*actual, exit, function, Some(finishing), local_bindings,
                invoke, projection, frame, bindings, Some(stored), context)?;
            visit(finishing.binding(invoke.0, &invoke.1)?.ok_or_else(|| freeze("map-get-binding-missing"))?)?;
        }
        Ok(())
    }
}
