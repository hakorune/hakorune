//! Sole checked field append and source-owned read Fault completion.
use super::*;
use crate::mir::{BasicBlockId, MirInstruction};
use crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
use crate::mir::normal_callable_semantic_package::{LocalFieldReadSourceV1, OrdinaryNewClaimLedgerV1};
use crate::mir::resolved_semantics::OwnedExprSiteV1;
use std::rc::Rc;

impl crate::mir::MirBuilder {
    /// Sole checked received-field append. The caller supplies the enclosing
    /// source-owned Fault cleanup; this writer never infers receiver authority.
    pub(in crate::mir::builder) fn append_checked_received_field_read_v1(
        &mut self, base: ValueId,
        source: &crate::mir::normal_callable_semantic_package::LocalFieldReadSourceV1,
        site: &OwnedExprSiteV1, ledger: &OrdinaryNewClaimLedgerV1,
        fault_frame: ValueId, normal: crate::mir::BasicBlockId, fault: crate::mir::BasicBlockId,
    ) -> Result<(ValueId, [(BasicBlockId, MirInstruction); 2], Vec<(BasicBlockId, MirInstruction)>), String> {
        let (base, prefix) = self.append_checked_field_receiver_v1(base, source, site, ledger, fault_frame, fault)?;
        let (value, pair) = self.append_checked_field_read_with_destination_v1(base, source, fault_frame, normal, fault, None)?;
        Ok((value, pair, prefix))
    }

    /// Source and enclosing-expression reads use one Shared Borrow transaction.
    fn append_checked_field_receiver_v1(
        &mut self, base: ValueId, source: &LocalFieldReadSourceV1,
        site: &OwnedExprSiteV1, ledger: &OrdinaryNewClaimLedgerV1,
        fault_frame: ValueId, fault: BasicBlockId,
    ) -> Result<(ValueId, Vec<(BasicBlockId, MirInstruction)>), String> {
        let Some(support) = source.shared_support_v1() else { return Ok((base, Vec::new())); };
        let required = ledger.source_field_requirement_v1(site)?
            .ok_or("[freeze:contract][ordinary-field-read/checked-source-context-missing]")?;
        let (descriptor, handle, _) = ledger.prepare_shared_clone_handle_v1(required.request().receiver,
            &support.0, required.prior_homes())?;
        if handle != base || descriptor.payload != source.field().object()
            || required.request().shared_support.as_ref() != Some(support) {
            return Err("[freeze:contract][shared-field/support-drift]".into());
        }
        let block = self.function_state.current_block.ok_or("[freeze:contract][shared-field/no-block]")?;
        let normal = self.next_block_id();
        let invoke = MirInstruction::Invoke {
            operation: crate::mir::instruction::InvokeOperation::Shared(
                crate::mir::instruction::SharedInvokeOperation::Borrow { descriptor, handle }),
            fault_frame, normal_landing: normal, fault_landing: fault,
        };
        self.emit_instruction(invoke.clone())?;
        self.start_new_block(normal)?;
        let value = self.next_value_id();
        let projection = MirInstruction::InvokeNormalResult { invoke_block: block, dst: value };
        self.emit_instruction(projection.clone())?;
        Ok((value, vec![(block, invoke), (normal, projection)]))
    }
    fn append_checked_field_read_with_destination_v1(
        &mut self, base: ValueId,
        source: &crate::mir::normal_callable_semantic_package::LocalFieldReadSourceV1,
        fault_frame: ValueId, normal: crate::mir::BasicBlockId, fault: crate::mir::BasicBlockId,
        destination: Option<ValueId>,
    ) -> Result<(ValueId, [(crate::mir::BasicBlockId, crate::mir::MirInstruction); 2]), String> {
        use crate::mir::MirInstruction;
        if !source.requires_checked_field_read_v1() || normal == fault {
            return Err("[freeze:contract][ordinary-field-read/checked-source-missing]".into());
        }
        let block = self.function_state.current_block.ok_or("[freeze:contract][ordinary-field-read/no-block]")?;
        let (operation, ty) = source.checked_operation_v1(base)
            .ok_or("[freeze:contract][ordinary-field-read/checked-source-missing]")?;
        let invoke = MirInstruction::Invoke {
            operation,
            fault_frame, normal_landing: normal, fault_landing: fault,
        };
        self.emit_instruction(invoke.clone())?;
        self.start_new_block(normal)?;
        let dst = destination.unwrap_or_else(|| self.next_value_id());
        let projection = MirInstruction::InvokeNormalResult { invoke_block: block, dst };
        self.emit_instruction(projection.clone())?;
        self.function_state.type_ctx.value_types.insert(dst, ty.clone());
        if let Some(function) = self.function_state.current_function.as_mut() {
            function.metadata.value_types.insert(dst, ty);
        }
        Ok((dst, [(block, invoke), (normal, projection)]))
    }

    pub(in crate::mir::builder) fn complete_source_checked_field_read_v1(
        &mut self, base: ValueId, field: hakorune_mir_defs::CanonicalFieldRefV1,
        site: &OwnedExprSiteV1, ledger: &OrdinaryNewClaimLedgerV1,
        source: &Rc<LocalFieldReadSourceV1>, read: Rc<crate::mir::builder::ExactLexicalReadV1>,
        state: &mut CallableSemanticLoweringState, destination: Option<ValueId>,
    ) -> Result<(ValueId, (BasicBlockId, MirInstruction)), String> {
        use crate::mir::builder::ordinary_new_admission::selected::{fault_landing_for, FaultCleanupPlanV1};
        let required = ledger.source_field_requirement_v1(site)?
            .ok_or("[freeze:contract][ordinary-field-read/checked-source-context-missing]")?;
        if state.owner() != site.owner() || source.field() != field
            || read.value_for(site.owner(), required.request().receiver_site.node(), required.request().receiver)
                .map_err(|issue| issue.to_string())? != base {
            return Err("[freeze:contract][ordinary-field-read/checked-receiver-context]".into());
        }
        if !source.requires_checked_field_read_v1() {
            let result = self.complete_exact_field_read_with_destination_v1(base, field, site, ledger,
                super::PreparedExactFieldReadClaimV1::Local(Rc::clone(source)), destination)?;
            ledger.retain_raw_field_receiver_v1(site, source, read)?;
            return Ok(result);
        }
        let operations = ledger.prior_home_unwind_for(required.prior_homes())?;
        let entry = self.function_state.current_block.ok_or("[freeze:contract][ordinary-field-read/no-block]")?;
        let start = self.function_state.current_function.as_ref().and_then(|function| function.blocks.get(&entry))
            .ok_or("[freeze:contract][ordinary-field-read/no-function]")?.instructions.len();
        let fault_frame = state.borrow_fault_frame(self)?;
        let function = self.function_state.current_function.as_ref().ok_or("[freeze:contract][ordinary-field-read/no-function]")?;
        let frame = state.fault_frame_binding(function)?;
        // Capture only rows this writer actually appended on the Normal path.
        let mut normal_prefix: Vec<_> = function.blocks[&entry].instructions[start..].iter()
            .map(|row| (entry, row.clone())).collect();
        if normal_prefix.iter().any(|row| row != &frame) {
            return Err("[freeze:contract][ordinary-field-read/checked-frame-append]".into());
        }
        let mut fault = Vec::new();
        let landing = fault_landing_for(self, fault_frame, &FaultCleanupPlanV1::unwind(operations), &mut fault)?;
        let (base, shared_prefix) = self.append_checked_field_receiver_v1(base, source, site, ledger, fault_frame, landing)?;
        normal_prefix.extend(shared_prefix);
        let normal = self.next_block_id();
        let (value, pair) = self.append_checked_field_read_with_destination_v1(base, source, fault_frame, normal, landing, destination)?;
        ledger.record_source_checked_local_field_read_v1(site, source, read, pair[0].clone(), pair[1].clone(), frame,
            normal_prefix, fault)?;
        Ok((value, pair[1].clone()))
    }
}
