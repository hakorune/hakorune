//! Root exit progress for the readable-Map terminal.
//!
//! The MapGet entry retains the already-emitted checked read exactly like a
//! Call ingress: the source relation owns the key and receiver class; this
//! file only records and re-verifies physical coordinates.
use super::*;
use crate::mir::normal_callable_semantic_package::EvaluatedExpressionValueV1;

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
        stored: Option<Box<StoredMapGetReturnEmissionV1>>,
        function: &MirFunction,
    ) -> Result<(), String> {
        let local_bindings = self.select_local_call_binding_groups(owner, site)?;
        // A surrounding Bool arm is still being emitted here. Validate this
        // producer now; the existing exit/final consumer owns whole-group seals.
        if let Some(stored) = &stored {
            let kind = stored.validate(
                CallValidationContextV1::Lowering(self),
                function,
                None,
                &invoke,
                &projection,
            )?;
            super::super::root_cleanup_graph::call::ingress(
                function,
                &bindings,
                &invoke,
                &projection,
                kind,
            )?;
        }
        self.record_root_home_exit_with_entry(
            owner,
            site,
            origins,
            bindings,
            RootHomeExitEntry::MapGet {
                stored,
                local_bindings,
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
        stored: Option<&StoredMapGetReturnEmissionV1>,
        context: CallValidationContextV1<'_>,
    ) -> Result<(), String> {
        let expected = if let Some(stored) = stored {
            if stored.source.terminal().owner() != owner
                || stored.source.terminal().return_site() != exit
            {
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
        let MirInstruction::Invoke { fault_frame, .. } = &invoke.1 else {
            return Err(freeze("map-get-shape"));
        };
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
        self.check_local_call_binding_groups_in_context_v1(
            context,
            owner,
            exit,
            function,
            finishing,
            local_bindings,
            &[],
        )?;
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
    pub(crate) fn issue(
        source: crate::mir::normal_callable_semantic_package::StoredMapGetReturnSourceLoanV1,
        read: StoredMapGetReadEmissionV1,
        ledger: &OrdinaryNewClaimLedgerV1,
        function: &MirFunction,
    ) -> Result<Self, String> {
        let packet = Self { source, read };
        packet.operation(CallValidationContextV1::Lowering(ledger), function)?;
        Ok(packet)
    }
    pub(super) fn bindings(&self) -> &[(BasicBlockId, MirInstruction)] {
        &self.read.key_read.original
    }
    pub(super) fn copy_dependencies(&self) -> Result<Vec<(BasicBlockId, MirInstruction)>, String> {
        self.read.copy_dependencies()
    }
    pub(crate) fn operation(
        &self,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
    ) -> Result<InvokeOperation, String> {
        let terminal = self.source.terminal();
        let retained = context
            .ledger()
            .stored_map_get_return_source_v1(terminal.owner(), terminal.return_site())?
            .ok_or_else(|| freeze("map-get-source-missing"))?;
        if retained.terminal() != terminal
            || retained.get_source() != &self.read.source
            || !std::rc::Rc::ptr_eq(retained.parts().1, &self.read.key_read.key)
        {
            return Err(freeze("map-get-shape"));
        }
        self.read.operation(context, function)
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate(
        &self,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        finishing: Option<&super::super::physical_boundary::FinishedBindings>,
        invoke: &(BasicBlockId, MirInstruction),
        projection: &(BasicBlockId, MirInstruction),
    ) -> Result<crate::mir::instruction::InvokeCallResultKind, String> {
        self.operation(context, function)?;
        self.read
            .validate(context, function, finishing, invoke, projection)?;
        Ok(
            crate::mir::instruction::InvokeCallResultKind::BorrowedNullableHandle {
                support: self.source.parts().0.result_support_v1(),
            },
        )
    }
}

/// Exact get policy over the common passive key/receiver observations.
#[derive(Debug)]
pub(crate) struct StoredMapGetReadEmissionV1 {
    source: crate::mir::normal_callable_semantic_package::StoredMapGetSourceLoanV1,
    key_read: super::map_key_read::FieldMapKeyReadEmissionV1,
}
impl StoredMapGetReadEmissionV1 {
    pub(in crate::mir) fn issue(
        source: crate::mir::normal_callable_semantic_package::StoredMapGetSourceLoanV1,
        key: std::rc::Rc<
            crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1,
        >,
        receiver: crate::mir::builder::ExactLexicalReadV1,
        values: Vec<Option<EvaluatedExpressionValueV1>>,
        reads: std::collections::BTreeMap<usize, super::ExpressionReferenceReadV1>,
        entry: (BasicBlockId, MirInstruction),
        terminal: (BasicBlockId, MirInstruction),
        continuation: (BasicBlockId, MirInstruction),
        original: Vec<(BasicBlockId, MirInstruction)>,
        ledger: &OrdinaryNewClaimLedgerV1,
        function: &MirFunction,
    ) -> Result<Self, String> {
        let key_read = super::map_key_read::FieldMapKeyReadEmissionV1::issue(key, receiver, values, reads,
            entry, terminal, continuation, original, ledger)?;
        let read = Self { source, key_read };
        read.operation(CallValidationContextV1::Lowering(ledger), function)?;
        Ok(read)
    }
    fn copy_dependencies(&self) -> Result<Vec<(BasicBlockId, MirInstruction)>, String> {
        self.key_read.copy_dependencies()
    }
    fn expression(&self) -> super::ExpressionPhysicalProjectionRefV1<'_> {
        self.key_read.expression()
    }
    pub(in crate::mir) fn operation(
        &self,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
    ) -> Result<InvokeOperation, String> {
        self.source.recheck_v1(context.ledger())?;
        let get = self.source.get();
        if self.key_read.values.len() != 1
            || self.key_read.key.source().nodes().len() != 1
            || get.arguments().len() != 1
            || self.key_read.key.source().owner() != get.owner()
            || self.key_read.key.source().nodes()[0].site() != get.arguments()[0].site()
            || !matches!(self.key_read.key.source().root(), crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument { call, ordinal: 0, .. }
                if std::sync::Arc::ptr_eq(call, get))
        {
            return Err(freeze("map-get-shape"));
        }
        let base = self.source.receiver().require_receiver_at_v1(
            context,
            function,
            &self.key_read.receiver,
            self.source.receiver().receiver_site(),
        )?;
        let key = self.key_read.values[0]
            .as_ref()
            .map(|key| key.as_operand())
            .transpose()?
            .cloned()
            .filter(|key| key.is_text())
            .ok_or_else(|| freeze("map-get-key-drift"))?;
        Ok(InvokeOperation::Map(
            crate::mir::instruction::MapInvokeOperation::FieldGetIndexed {
                field: self.source.receiver().field(),
                base,
                key,
                object: self.source.storage().payload_object_v1(),
            },
        ))
    }
    fn validate(&self, context: CallValidationContextV1<'_>, function: &MirFunction,
        finishing: Option<&super::super::physical_boundary::FinishedBindings>,
        invoke: &(BasicBlockId, MirInstruction), projection: &(BasicBlockId, MirInstruction),
    ) -> Result<ValueId, String> {
        self.key_read.validate(context, function, finishing, &self.operation(context, function)?, invoke, projection)
    }
}

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn visit_stored_map_get_groups_v1(
        &self,
        owner: FunctionOwnerIdV1,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        finishing: &super::super::physical_boundary::FinishedBindings,
        module: &crate::mir::MirModule,
        mut visit: impl FnMut((BasicBlockId, MirInstruction)) -> Result<(), String>,
    ) -> Result<(), String> {
        context.with_local_call_group_tree_v2(owner, |groups| {
            for group in groups {
                let Some(packet) = group.stored_map_get() else {
                    continue;
                };
                group.validate_recorded_in_context(context, function, Some(finishing))?;
                if let CallValidationContextV1::Finalized(source) = context {
                    packet
                        .frame
                        .source()
                        .receiver()
                        .require_finished_birth_v1(source, module)?;
                }
                visit(
                    finishing
                        .binding(packet.invoke.0, &packet.invoke.1)?
                        .ok_or_else(|| freeze("map-get-binding-missing"))?,
                )?;
            }
            Ok(())
        })?;
        let exits = self.root_exits.borrow();
        for ((actual, exit), row) in exits.iter().filter(|((actual, _), _)| *actual == owner) {
            let RootHomeExitProgress::Emitted {
                entry:
                    RootHomeExitEntry::MapGet {
                        stored: Some(stored),
                        local_bindings,
                        invoke,
                        projection,
                        frame,
                    },
                bindings,
                ..
            } = row
            else {
                continue;
            };
            let exit = exit
                .explicit_site()
                .ok_or_else(|| freeze("map-get-source-missing"))?;
            if let CallValidationContextV1::Finalized(source) = context {
                stored
                    .source
                    .get_source()
                    .receiver()
                    .require_finished_birth_v1(source, module)?;
            }
            self.validate_map_get_entry(
                *actual,
                exit,
                function,
                Some(finishing),
                local_bindings,
                invoke,
                projection,
                frame,
                bindings,
                Some(stored),
                context,
            )?;
            visit(
                finishing
                    .binding(invoke.0, &invoke.1)?
                    .ok_or_else(|| freeze("map-get-binding-missing"))?,
            )?;
        }
        Ok(())
    }
}

/// A get becomes a Local group only after the sole Local writer's real receipt.
/// No payload Home/End, callable support or replacement source index is issued.
#[derive(Debug)]
pub(crate) struct StoredMapGetLocalEmissionV1 {
    frame: std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::MapFieldGetLocalV1>,
    read: StoredMapGetReadEmissionV1,
    value: ValueId,
    local: ValueId,
    copy: (BasicBlockId, MirInstruction),
    invoke: (BasicBlockId, MirInstruction),
    normal: (BasicBlockId, MirInstruction),
    faults: Vec<(BasicBlockId, MirInstruction)>,
    original: Vec<(BasicBlockId, MirInstruction)>,
}
impl StoredMapGetLocalEmissionV1 {
    pub(crate) fn issue(
        frame: std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::MapFieldGetLocalV1>,
        read: StoredMapGetReadEmissionV1,
        value: ValueId,
        local: ValueId,
        copy: (BasicBlockId, MirInstruction),
        invoke: (BasicBlockId, MirInstruction),
        normal: (BasicBlockId, MirInstruction),
        faults: Vec<(BasicBlockId, MirInstruction)>,
        ledger: &OrdinaryNewClaimLedgerV1,
        function: &MirFunction,
    ) -> Result<Self, String> {
        let mut original = read.copy_dependencies()?;
        for row in read
            .key_read.original
            .iter()
            .chain([&invoke, &normal, &copy])
            .chain(faults.iter())
        {
            if !original.contains(row) {
                original.push(row.clone());
            }
        }
        let packet = Self {
            frame,
            read,
            value,
            local,
            copy,
            invoke,
            normal,
            faults,
            original,
        };
        packet.validate_paths(CallValidationContextV1::Lowering(ledger), function, None)?;
        Ok(packet)
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn frame(
        &self,
    ) -> &std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::MapFieldGetLocalV1> {
        &self.frame
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn bindings(
        &self,
    ) -> &[(BasicBlockId, MirInstruction)] {
        &self.original
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn copy_receipt(
        &self,
    ) -> (ValueId, ValueId, &(BasicBlockId, MirInstruction)) {
        (self.local, self.value, &self.copy)
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_recorded(
        &self,
        bindings: &[(BasicBlockId, MirInstruction)],
        coordinates: Option<&super::super::physical_boundary::FinishedBindings>,
    ) -> Result<(), String> {
        self.read
            .expression()
            .validate_recorded(bindings, coordinates)?;
        for row in [&self.invoke, &self.normal]
            .into_iter()
            .chain(self.faults.iter())
        {
            let actual = match coordinates {
                Some(p) => p.binding(row.0, &row.1)?,
                None => Some(row.clone()),
            };
            if actual
                .as_ref()
                .is_some_and(|row| bindings.iter().filter(|b| *b == row).count() != 1)
            {
                return Err(freeze("map-get-binding-drift"));
            }
        }
        if coordinates.is_none() && bindings.iter().filter(|row| **row == self.copy).count() != 1 {
            return Err(freeze("initializer-expression/local-copy-membership"));
        }
        Ok(())
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_paths(
        &self,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        finishing: Option<&super::super::physical_boundary::FinishedBindings>,
    ) -> Result<(), String> {
        let ledger = context.ledger();
        let retained = ledger
            .stored_map_get_local_source_v1(
                self.frame.site().owner(),
                self.frame.statement().node(),
            )?
            .ok_or_else(|| freeze("map-get-source-missing"))?;
        if !std::rc::Rc::ptr_eq(&retained, &self.frame)
            || self.frame.source() != &self.read.source
            || !std::rc::Rc::ptr_eq(self.frame.key(), &self.read.key_read.key)
            || !matches!(self.copy.1, MirInstruction::Copy { dst, src } if dst == self.local && src == self.value)
            || self
                .read
                .validate(context, function, finishing, &self.invoke, &self.normal)?
                != self.value
        {
            return Err(freeze("map-get-shape"));
        }
        let copy = ledger.validate_initializer_local_copy_v1(
            context,
            function,
            finishing,
            self.value,
            self.local,
            Some(&self.copy),
        )?;
        let map = |row: &(BasicBlockId, MirInstruction)| match finishing {
            Some(p) => p
                .binding(row.0, &row.1)?
                .ok_or_else(|| freeze("map-get-binding-missing")),
            None => Ok(row.clone()),
        };
        if let Some(copy) = copy {
            super::ExpressionPhysicalProjectionRefV1::validate_normal_order_v1(
                function,
                &[map(&self.normal)?, copy],
            )?;
        }
        let faults = match finishing {
            Some(p) => p.bindings(&self.faults)?,
            None => self.faults.clone(),
        };
        ledger.validate_source_field_fault_v1(
            self.frame.site().owner(),
            function,
            &map(&self.invoke)?,
            &faults,
            self.frame.prior_homes(),
        )
    }
}
impl CallValidationContextV1<'_> {
    /// Borrow the original get Normal through the actual Local receipt and
    /// its complete read Copy chain. This grants no scalar/tagged operand.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn stored_map_get_read_v1(
        self,
        frame: &std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::MapFieldGetLocalV1>,
        read: &crate::mir::builder::ExactLexicalReadV1,
        function: &MirFunction,
        finishing: Option<&super::super::physical_boundary::FinishedBindings>,
    ) -> Result<(ValueId, (BasicBlockId, MirInstruction)), String> {
        self.with_local_call_group_tree_v2(frame.site().owner(), |groups| {
            let mut rows = groups
                .iter()
                .filter_map(|group| group.stored_map_get().map(|packet| (group, packet)))
                .filter(|(_, packet)| std::rc::Rc::ptr_eq(&packet.frame, frame));
            let (group, packet) = rows
                .next()
                .ok_or_else(|| freeze("map-get-source-missing"))?;
            if rows.next().is_some() {
                return Err(freeze("map-get-shape"));
            }
            group.validate_recorded_in_context(self, function, finishing)?;
            let (value, copies) = read.loan_local_initializer_v1(frame.binding())?;
            if value != packet.value
                || copies.first() != Some(&packet.copy)
                || copies.iter().filter(|row| **row == packet.copy).count() != 1
            {
                return Err(freeze("initializer-expression/local-copy-correspondence"));
            }
            let value = read.source_value().2;
            Ok((value, packet.normal.clone()))
        })
    }
}
