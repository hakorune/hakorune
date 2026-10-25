//! Checked emission on the existing local-field row and original Call packet.
use super::*;
use crate::mir::builder::ExactLexicalReadV1;
use crate::mir::instruction::InvokeOperation;
use crate::mir::normal_callable_semantic_package::CallValidationContextV1;
use crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1;
type Binding = (BasicBlockId, MirInstruction);

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir) fn local_field_read_requires_checked_v1(&self, site: &OwnedExprSiteV1) -> bool {
        self.local_field_reads.borrow().get(site).is_some_and(|row| row.source.requires_checked_field_read_v1())
    }

    /// Lend the emitted value for the exact already-issued scalar local field
    /// read. This is a source dependency lookup; it cannot create or consume
    /// another field read.
    pub(in crate::mir) fn local_scalar_field_read_emission_v1(
        &self,
        site: &OwnedExprSiteV1,
    ) -> Result<Option<Binding>, String> {
        let rows = self.local_field_reads.borrow();
        let Some(row) = rows.get(site) else {
            return Ok(None);
        };
        if row.source.result()
            != &crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1::Scalar
        {
            return Ok(None);
        }
        self.recheck_local_field_source_v1(site, row, CallValidationContextV1::Lowering(self))?;
        let LocalReadProgress::Emitted {
            block,
            instruction,
            source,
        } = &row.progress
        else {
            return Err(fault("scalar/emission-missing"));
        };
        if !Rc::ptr_eq(source, &row.source) || !source.corroborates_instruction(instruction) {
            return Err(fault("scalar/emission-source-drift"));
        }
        Ok(Some((*block, instruction.clone())))
    }

    pub(super) fn record_checked_local_field_read_v1(
        &self, site: &OwnedExprSiteV1, source: &Rc<LocalFieldReadSourceV1>,
        read: Option<Rc<ExactLexicalReadV1>>, invoke: Binding, normal: Binding,
        cleanup: CheckedFieldReadCleanupV1,
    ) -> Result<(), String> {
        let mut rows = self.local_field_reads.borrow_mut();
        let row = rows.get_mut(site).ok_or_else(|| fault("received/read-missing"))?;
        if !source.requires_checked_field_read_v1() || !Rc::ptr_eq(source, &row.source)
            || !matches!(&row.progress, LocalReadProgress::Taken { source: taken, base }
                if Rc::ptr_eq(taken, source) && read.as_ref().is_none_or(|read| read.source_value().2 == *base)
                    && matches!(&invoke.1, MirInstruction::Invoke { operation: InvokeOperation::FieldGet { base: actual, .. }, .. } if actual == base)) {
            return Err(fault("received/emission-source"));
        }
        self.recheck_local_field_source_v1(site, row, CallValidationContextV1::Lowering(self))?;
        row.progress = LocalReadProgress::CheckedEmitted { source: Rc::clone(source), invoke, normal, read, cleanup };
        drop(rows);
        self.checked_field_pair_v1(site, CallValidationContextV1::Lowering(self))?
            .ok_or_else(|| fault("received/emission-missing"))?;
        Ok(())
    }

    pub(in crate::mir) fn checked_field_pair_v1(
        &self, site: &OwnedExprSiteV1, context: CallValidationContextV1<'_>,
    ) -> Result<Option<[Binding; 2]>, String> {
        context.require_ledger(self)?;
        let rows = self.local_field_reads.borrow();
        let Some(row) = rows.get(site) else { return Ok(None); };
        if !row.source.requires_checked_field_read_v1() { return Ok(None); }
        self.recheck_local_field_source_v1(site, row, context)?;
        let LocalReadProgress::CheckedEmitted { source, invoke, normal, read, .. } = &row.progress
            else { return Err(fault("received/unconsumed-read")); };
        if !Rc::ptr_eq(source, &row.source) { return Err(fault("received/descriptor-drift")); }
        let base = if let Some(path) = source.statement_path().filter(|path| path.requires_checked_scalar_v1()) {
            if read.is_some() { return Err(fault("received/nested-lexical-read")); }
            let parent = path.checked_parent_v1().ok_or_else(|| fault("received/nested-parent"))?;
            let plan = self.source_field_requirement_v1(site)?.ok_or_else(|| fault("received/expression-source-missing"))?.plan();
            let bindings = self.expression_field_read_bindings_v1(&parent, plan, context)?;
            match bindings.last().map(|row| &row.1) {
                Some(MirInstruction::ObjectFieldGet { dst, .. } | MirInstruction::InvokeNormalResult { dst, .. }) => *dst,
                _ => return Err(fault("received/nested-parent-result")),
            }
        } else {
            read.as_ref().ok_or_else(|| fault("received/exact-read-missing"))?
                .value_for(site.owner(), row.receiver_site.node(), row.receiver)
                .map_err(|issue| format!("{}: {issue}", fault("received/exact-read")))?
        };
        if !matches!((&invoke.1, &normal.1),
            (MirInstruction::Invoke { operation: InvokeOperation::FieldGet { field, base: actual }, normal_landing, fault_landing, .. },
             MirInstruction::InvokeNormalResult { invoke_block, dst })
                if *field == source.field() && *actual == base && *invoke_block == invoke.0
                    && *normal_landing == normal.0 && normal_landing != fault_landing && *dst != base)
        { return Err(fault("received/invoke-normal-drift")); }
        if let Some(received) = &row.source.received { context.with_taken_packet_v1(received.target().call_site(), |packet| {
            let original = packet.original_row()?;
            if original.source_target() != received.target() { return Err(fault("received/packet-source")); }
            let value = if received.mixed().is_some() {
                self.corroborate_mixed_call_using_v1(original, context)?;
                let observation = self.mixed_call_source_v1(original.call_site())
                    .ok_or_else(|| fault("received/mixed-observation"))?;
                if original.source_target().mixed_local_source() != received.mixed() { return Err(fault("received/mixed-packet")); }
                packet.value_in_context_v1(site.owner(), observation.arguments(), context)?
            } else {
                let original = self.borrowed_object_call_packet_v1(original)?;
                if Some(original.result()) != received.object_result_kind_v1() { return Err(fault("received/object-packet")); }
                packet.value_in_context_v1(site.owner(), original.arguments(), context)?
            };
            read.as_ref().ok_or_else(|| fault("received/exact-read-missing"))?.loan_forwarded_copies(row.receiver, value)?;
            Ok(())
        })?; }
        Ok(Some([invoke.clone(), normal.clone()]))
    }

    pub(in crate::mir) fn expression_field_read_bindings_v1(
        &self, site: &OwnedExprSiteV1, plan: &Rc<ExpressionSourcePlanV1>, context: CallValidationContextV1<'_>,
    ) -> Result<Vec<Binding>, String> {
        let required = self.source_field_requirement_v1(site)?
            .ok_or_else(|| fault("expression-source-requirement-missing"))?;
        if !Rc::ptr_eq(required.plan(), plan) { return Err(fault("expression-source-plan")); }
        match self.checked_field_pair_v1(site, context)? {
            Some(pair) => {
                let mut rows = self.checked_field_normal_prefix_v1(site)?;
                rows.extend(pair);
                Ok(rows)
            },
            None => self.expression_field_read_emission_v1(site, plan, context).map(|binding| vec![binding]),
        }
    }

    pub(in crate::mir::normal_callable_semantic_package) fn checked_field_copy_dependencies_v1(
        &self, site: &OwnedExprSiteV1, context: CallValidationContextV1<'_>,
    ) -> Result<Vec<Binding>, String> {
        if self.checked_field_pair_v1(site, context)?.is_none() { return Ok(Vec::new()); }
        let rows = self.local_field_reads.borrow();
        let row = rows.get(site).ok_or_else(|| fault("received/read-missing"))?;
        let LocalReadProgress::CheckedEmitted { read, .. } = &row.progress else { return Err(fault("received/emission-missing")); };
        if row.source.statement_path().is_some() { return Ok(Vec::new()); }
        let Some(received) = row.source.received.as_ref() else {
            // checked_field_pair replays the same complete numeric/source loan
            // and exact lexical receiver. A direct Self read has no Call result.
            if row.numeric_source.is_none() { return Err(fault("received/source-missing")); }
            return read.as_ref().ok_or_else(|| fault("received/exact-read-missing"))?
                .original_copy_dependencies_v1().map(<[_]>::to_vec);
        };
        context.with_taken_packet_v1(received.target().call_site(), |packet| {
            let normal = packet.outer_bindings()?.1;
            let MirInstruction::InvokeNormalResult { dst, .. } = normal.1 else { return Err(fault("received/packet-normal")); };
            read.as_ref().ok_or_else(|| fault("received/exact-read-missing"))?.loan_forwarded_copies(row.receiver, dst).map(<[_]>::to_vec)
        })
    }
    /// Existing received field demand selects full Call validation; this does
    /// not manufacture a borrowed-formal incoming row or executable arguments.
    pub(in crate::mir::normal_callable_semantic_package) fn has_received_field_packet_source_v1(
        &self, target: &super::super::lexical_instance_call::LexicalInstanceCallSourceTargetV1,
    ) -> Result<bool, String> {
        let rows = self.local_field_reads.borrow();
        let mut demanded = false;
        for (site, row) in rows.iter() {
            let Some(received) = row.source.received_root_source_v1() else { continue };
            if received.mixed().is_some() || received.target().call_site() != target.call_site() { continue; }
            let requirement = self.source_field_requirement_v1(site)?
                .ok_or_else(|| fault("received/expression-source-missing"))?;
            let ingress = self.borrowed_formal_source.as_ref().ok_or_else(|| fault("received/ingress-missing"))?
                .as_ref().map_err(Clone::clone)?;
            received.recheck_issuing_target_v1(&self.callable_result_classes, ingress, target, requirement.request())?;
            if row.receiver != requirement.request().receiver || row.home != requirement.request().home
                || row.receiver_site != requirement.request().receiver_site { return Err(fault("received/packet-read-drift")); }
            demanded = true;
        }
        Ok(demanded)
    }

    pub(in crate::mir::normal_callable_semantic_package) fn has_checked_field_reads_v1(&self, owner: FunctionOwnerIdV1) -> bool {
        self.local_field_reads.borrow().iter().any(|(site, row)| site.owner() == owner && row.source.requires_checked_field_read_v1())
            || self.field_reads.borrow().iter().any(|(site, row)| site.owner() == owner
                && matches!(row.progress, Progress::CheckedEmitted { .. }))
    }

    pub(in crate::mir) fn checked_field_read_owners_v1(&self) -> BTreeSet<FunctionOwnerIdV1> {
        self.local_field_reads.borrow().iter()
            .filter(|(_, row)| row.source.requires_checked_field_read_v1())
            .map(|(site, _)| site.owner())
            .chain(self.field_reads.borrow().iter().filter_map(|(site, row)|
                matches!(row.progress, Progress::CheckedEmitted { .. }).then_some(site.owner())))
            .collect()
    }

    /// Every checked read and Normal result must be the same original row after
    /// finishing. Instruction shape alone never creates source membership.
    pub(in crate::mir::normal_callable_semantic_package) fn validate_checked_field_census_v1(
        &self, owner: FunctionOwnerIdV1, function: &MirFunction, context: CallValidationContextV1<'_>,
        mut project: impl FnMut(&Binding) -> Result<Option<Binding>, String>,
    ) -> Result<Vec<Binding>, String> {
        let sites: Vec<_> = self.local_field_reads.borrow().iter()
            .filter(|(site, row)| site.owner() == owner && row.source.requires_checked_field_read_v1())
            .map(|(site, _)| site.clone()).collect();
        let mut expected = Vec::new();
        for site in sites {
            self.validate_checked_field_cleanup_v1(&site, function, context, &mut project)?;
            let pair = self.checked_field_pair_v1(&site, context)?.ok_or_else(|| fault("received/pair-missing"))?;
            for binding in pair {
                let mapped = project(&binding)?.ok_or_else(|| fault("received/finished-binding-missing"))?;
                if expected.contains(&mapped) { return Err(fault("received/duplicate-binding")); }
                expected.push(mapped);
            }
        }
        for (site, row) in self.field_reads.borrow().iter().filter(|(site, row)| {
            site.owner() == owner && matches!(row.progress, Progress::CheckedEmitted { .. })
        }) {
            self.recheck_terminal_field_value_v1(site, row, context)?;
            let linked = self.terminal_relations_for_owner(owner).into_iter().any(|relation| {
                matches!(relation, TerminalRelationV1::I64Field(field) if field.field_read_site() == site)
            });
            if !linked { return Err(fault("received/terminal-source-missing")); }
            let Progress::CheckedEmitted { invoke, projection } = &row.progress else { unreachable!() };
            for binding in [invoke, projection] {
                let mapped = project(binding)?.ok_or_else(|| fault("received/finished-binding-missing"))?;
                if expected.contains(&mapped) { return Err(fault("received/duplicate-binding")); }
                expected.push(mapped);
            }
        }
        let published = expected.clone();
        for (block, body) in &function.blocks {
            for row in body.all_instructions() {
                let checked = matches!(row, MirInstruction::Invoke { operation: InvokeOperation::FieldGet { .. }, .. })
                    || matches!(row, MirInstruction::InvokeNormalResult { invoke_block, .. }
                        if function.blocks.get(invoke_block).and_then(|body| body.terminator.as_ref()).is_some_and(|row|
                            matches!(row, MirInstruction::Invoke { operation: InvokeOperation::FieldGet { .. }, .. })));
                if !checked { continue; }
                let index = expected.iter().position(|binding| binding == &(*block, row.clone()))
                    .ok_or_else(|| fault("received/unowned-or-drifted-read"))?;
                expected.swap_remove(index);
            }
        }
        if !expected.is_empty() { return Err(fault("received/finished-read-missing")); }
        Ok(published)
    }

}

// Raw and checked reads share the same source-to-physical census owner.
impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn validate_field_reads(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
    ) -> Result<(), String> {
        self.validate_field_reads_with_projection_v1(owner, function, CallValidationContextV1::Lowering(self), |binding| Ok(Some(binding.clone())))
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn validate_field_reads_with_projection_v1(
        &self, owner: FunctionOwnerIdV1, function: &MirFunction, context: CallValidationContextV1<'_>,
        mut project: impl FnMut(&(BasicBlockId, MirInstruction)) -> Result<Option<(BasicBlockId, MirInstruction)>, String>,
    ) -> Result<(), String> {
        let reads = self.field_reads.borrow();
        let mut expected = Vec::new();
        let stored_receivers = self.stored_terminal_receiver_reads_v1(owner)?;
        expected.extend(
            stored_receivers
                .iter()
                .map(|(block, instruction)| (*block, instruction)),
        );
        for (site, row) in reads.iter() {
            if site.owner() != owner {
                continue;
            }
            self.recheck_terminal_field_value_v1(site, row, context)?;
            match &row.progress {
                Progress::Emitted(block, instruction) => expected.push((*block, instruction)),
                // The root FieldGet entry owns and independently validates
                // the checked Invoke/result pair. This census owns only
                // direct field-read instructions.
                Progress::CheckedEmitted { .. } => {}
                _ => return Err(fault("unconsumed-read")),
            }
        }
        let argument_reads = self.argument_field_reads.borrow();
        for (site, row) in argument_reads.iter() {
            if site.owner() != owner || matches!(row.progress, Progress::Released) {
                continue;
            }
            let Progress::Emitted(block, instruction) = &row.progress else {
                return Err(fault("unconsumed-read"));
            };
            expected.push((*block, instruction));
        }
        let local_reads = self.local_field_reads.borrow();
        // One checked source loan lives only for this immutable validation.
        // Each row still rechecks its own path/descriptor and physical relation.
        let result = self.validate_result_path_fields_present_v1(owner, &local_reads)?;
        if let Some(flow) = self.completion_for_owner(owner).and_then(|completion| completion.cleanup().root_flow()) {
            for field in flow.source_expression_fields() {
                let site = &field.request().site;
                if site.owner() != owner { return Err(fault("foreign-source-requirement")); }
                let row = local_reads.get(site).ok_or_else(|| fault("required-source-read-missing"))?;
                self.recheck_local_field_source_using_v1(site, row, context, ||
                    self.recheck_result_path_field_using_result_v1(site, row, context, result.as_ref()))?;
            }
        }
        for (site, row) in local_reads.iter() {
            if site.owner() != owner { continue; }
            self.recheck_local_field_source_using_v1(site, row, context, ||
                self.recheck_result_path_field_using_result_v1(site, row, context, result.as_ref()))?;
            if matches!(row.progress, LocalReadProgress::CheckedEmitted { .. }) { continue; }
            let LocalReadProgress::Emitted { block, instruction, source } = &row.progress else {
                return Err(fault("unconsumed-read"));
            };
            if !Rc::ptr_eq(&row.source, source)
                || !source.corroborates_instruction(instruction)
            {
                return Err(fault("source-descriptor-drift"));
            }
            if let Some(local) = &row.local { local.validate_function(function, row)?; }
            expected.push((*block, instruction));
        }
        let array_destinations: BTreeSet<_> = expected.iter().filter_map(|(_, instruction)| match instruction {
            MirInstruction::FieldGet { dst, .. } => Some(*dst), _ => None,
        }).collect();
        for block in function.blocks.values() {
            for actual in block
                .all_instructions()
                .filter(|i| matches!(i, MirInstruction::ObjectFieldGet { .. } | MirInstruction::FieldGet { .. }))
            {
                // Finishing may contract the read's block into a surviving
                // neighbour; dst/base/field identity is the provenance, the
                // boundary's sequence check owns positional drift.
                if matches!(actual, MirInstruction::FieldGet { dst, .. } if !array_destinations.contains(dst)) { continue; }
                let index = expected
                    .iter()
                    .position(|(_, inst)| *inst == actual)
                    .ok_or_else(|| fault("unowned-or-drifted-read"))?;
                expected.swap_remove(index);
            }
        }
        if !expected.is_empty() {
            return Err(fault("missing-emission"));
        }
        self.validate_owned_ready_reads_v1(owner, function, context, &mut project)?;
        self.validate_checked_field_census_v1(owner, function,
            context, project)
            .map(|_| ())
    }
}
