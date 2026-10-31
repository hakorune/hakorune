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
        fault_frame: ValueId, normal: crate::mir::BasicBlockId, fault: crate::mir::BasicBlockId,
    ) -> Result<(ValueId, [(crate::mir::BasicBlockId, crate::mir::MirInstruction); 2]), String> {
        self.append_checked_field_read_with_destination_v1(base, source, fault_frame, normal, fault, None)
    }
    fn append_checked_field_read_with_destination_v1(
        &mut self, base: ValueId,
        source: &crate::mir::normal_callable_semantic_package::LocalFieldReadSourceV1,
        fault_frame: ValueId, normal: crate::mir::BasicBlockId, fault: crate::mir::BasicBlockId,
        destination: Option<ValueId>,
    ) -> Result<(ValueId, [(crate::mir::BasicBlockId, crate::mir::MirInstruction); 2]), String> {
        use crate::mir::{MirInstruction, MirType};
        if !source.requires_checked_field_read_v1() || normal == fault {
            return Err("[freeze:contract][ordinary-field-read/checked-source-missing]".into());
        }
        let block = self.function_state.current_block.ok_or("[freeze:contract][ordinary-field-read/no-block]")?;
        use crate::mir::instruction::InvokeOperation;
        use crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1;
        let (operation, ty) = match source.result() {
            LocalFieldReadResultV1::BorrowedObject(child) =>
                (InvokeOperation::ObjectFieldBorrow { field: source.field(), base, child: child.child(), nullable: true },
                    MirType::Box(child.child_class().into())),
            LocalFieldReadResultV1::Text(text) if text.field() == source.field() =>
                (InvokeOperation::String(crate::mir::instruction::StringInvokeOperation::FieldMaterialize { field: source.field(), base }), MirType::String),
            LocalFieldReadResultV1::Scalar => (InvokeOperation::FieldGet { field: source.field(), base }, MirType::Integer),
            _ => return Err("[freeze:contract][ordinary-field-read/checked-source-missing]".into()),
        };
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
        use crate::mir::builder::ordinary_new_admission::selected::{append_block, cleanup_chain};
        let required = ledger.source_field_requirement_v1(site)?
            .ok_or("[freeze:contract][ordinary-field-read/checked-source-context-missing]")?;
        if state.owner() != site.owner() || source.field() != field
            || read.value_for(site.owner(), required.request().receiver_site.node(), required.request().receiver)
                .map_err(|issue| issue.to_string())? != base {
            return Err("[freeze:contract][ordinary-field-read/checked-receiver-context]".into());
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
        let outward = self.next_block_id();
        append_block(self, outward, MirInstruction::ReturnFault { fault_frame }, &mut fault)?;
        let landing = cleanup_chain(self, fault_frame, operations, outward, &mut fault)?;
        let base = if let Some(support) = source.shared_support_v1() {
            let (descriptor, handle, _) = ledger.prepare_shared_clone_handle_v1(required.request().receiver, &support.0, required.prior_homes())?;
            if handle != base || descriptor.payload != field.object() {
                return Err("[freeze:contract][shared-field/support-drift]".into());
            }
            let block = self.function_state.current_block.ok_or("[freeze:contract][shared-field/no-block]")?;
            let normal = self.next_block_id();
            let invoke = MirInstruction::Invoke {
                operation: crate::mir::instruction::InvokeOperation::Shared(
                    crate::mir::instruction::SharedInvokeOperation::Borrow { descriptor, handle }),
                fault_frame, normal_landing: normal, fault_landing: landing,
            };
            self.emit_instruction(invoke.clone())?;
            self.start_new_block(normal)?;
            let value = self.next_value_id();
            let projection = MirInstruction::InvokeNormalResult { invoke_block: block, dst: value };
            self.emit_instruction(projection.clone())?;
            normal_prefix.extend([(block, invoke), (normal, projection)]);
            value
        } else { base };
        let normal = self.next_block_id();
        let (value, pair) = self.append_checked_field_read_with_destination_v1(base, source, fault_frame, normal, landing, destination)?;
        ledger.record_source_checked_local_field_read_v1(site, source, read, pair[0].clone(), pair[1].clone(), frame,
            normal_prefix, fault)?;
        Ok((value, pair[1].clone()))
    }
}
