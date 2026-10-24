//! Root exit progress for a source-issued checked Dynamic field read.
//! The retained terminal relation and FieldRead row own source identity;
//! this entry records only the physical Invoke, projection and cleanup graph.
use super::*;

impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn record_root_field_get_exit(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceStmtSiteV1,
        invoke: (BasicBlockId, MirInstruction),
        projection: (BasicBlockId, MirInstruction),
        frame: (BasicBlockId, MirInstruction),
        origins: Vec<(RootHomeReleaseOriginV1, BasicBlockId, MirInstruction)>,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
        function: &MirFunction,
    ) -> Result<(), String> {
        let relation = self.terminal_relation_for_owner_at(owner, site)
            .and_then(|row| match row { TerminalRelationV1::I64Field(row) => Some(row), _ => None })
            .ok_or_else(|| freeze("field-get-source-missing"))?;
        let reads = self.field_reads.borrow();
        let read = reads.get(relation.field_read_site())
            .ok_or_else(|| freeze("field-get-read-missing"))?;
        let source_field = read.field;
        if !matches!(&read.progress, crate::mir::normal_callable_semantic_package::ordinary_new_coseal::field_reads::Progress::CheckedEmitted { invoke: expected, projection: result }
            if *expected == invoke && *result == projection)
        {
            return Err(freeze("field-get-read-drift"));
        }
        drop(reads);
        let MirInstruction::Invoke {
            operation: InvokeOperation::FieldGet { field, .. },
            fault_frame,
            ..
        } = &invoke.1 else { return Err(freeze("field-get-shape")); };
        if *field != source_field || !matches!(&projection.1,
            MirInstruction::InvokeNormalResult { invoke_block, .. } if *invoke_block == invoke.0)
            || !matches!(&frame.1, MirInstruction::FaultFrameEnter { dst, .. } if dst == fault_frame)
        {
            return Err(freeze("field-get-source-drift"));
        }
        super::super::root_cleanup_graph::call::ingress(
            function,
            &bindings,
            &invoke,
            &projection,
            crate::mir::instruction::InvokeCallResultKind::I64,
        )?;
        let local_bindings = self.select_local_call_binding_groups(owner, site)?;
        self.record_root_home_exit_with_entry(owner, site, origins, bindings,
            RootHomeExitEntry::FieldGet { local_bindings, invoke, projection, frame })
    }

    pub(super) fn validate_field_get_entry(
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
        context: CallValidationContextV1<'_>,
    ) -> Result<(), String> {
        let relation = self.terminal_relation_for_owner_at(owner, exit)
            .and_then(|row| match row { TerminalRelationV1::I64Field(row) => Some(row), _ => None })
            .ok_or_else(|| freeze("field-get-source-missing"))?;
        let reads = self.field_reads.borrow();
        let read = reads.get(relation.field_read_site())
            .ok_or_else(|| freeze("field-get-read-missing"))?;
        let field = read.field;
        if !matches!(&read.progress, crate::mir::normal_callable_semantic_package::ordinary_new_coseal::field_reads::Progress::CheckedEmitted { invoke: expected, projection: result }
            if expected == invoke && result == projection)
        {
            return Err(freeze("field-get-read-drift"));
        }
        self.recheck_terminal_field_value_v1(relation.field_read_site(), read, context)?;
        drop(reads);
        let MirInstruction::Invoke {
            operation: InvokeOperation::FieldGet { field: actual, .. },
            fault_frame,
            ..
        } = &invoke.1 else { return Err(freeze("field-get-shape")); };
        if *actual != field || !matches!(&projection.1,
            MirInstruction::InvokeNormalResult { invoke_block, .. } if *invoke_block == invoke.0)
            || !matches!(&frame.1, MirInstruction::FaultFrameEnter { dst, .. } if dst == fault_frame)
        {
            return Err(freeze("field-get-source-drift"));
        }
        for (block, instruction) in [invoke, projection, frame] {
            if !super::super::physical_boundary::check_binding(function, finishing, *block, instruction)? {
                return Err(freeze("field-get-binding-drift"));
            }
        }
        self.check_local_call_binding_groups_in_context_v1(
            context, owner, exit, function, finishing, local_bindings, &[],
        )?;
        let mapped = |binding: &(BasicBlockId, MirInstruction)| match finishing {
            Some(projected) => projected.binding(binding.0, &binding.1)?
                .ok_or_else(|| freeze("field-get-binding-missing")),
            None => Ok(binding.clone()),
        };
        let cleanup = match finishing {
            Some(projected) => projected.bindings(cleanup)?,
            None => cleanup.to_vec(),
        };
        super::super::root_cleanup_graph::call::ingress(
            function, &cleanup, &mapped(invoke)?, &mapped(projection)?,
            crate::mir::instruction::InvokeCallResultKind::I64,
        ).map(|_| ())
    }
}
