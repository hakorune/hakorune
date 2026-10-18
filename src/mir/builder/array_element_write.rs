use super::{MirBuilder, ValueId};
use crate::mir::{
    ArrayElementWriteKind, ArrayReadSiteId, ArrayWriteProducerKind, ArrayWriteSiteId,
    MirInstruction, MirType,
};

/// Physical write observation from the sole Array write emitter.
/// This carries no source, receiver class or borrowed-value permission.
#[derive(Debug)]
pub(in crate::mir) struct CompletedArrayElementWriteV1 {
    site: ArrayWriteSiteId,
    original: (crate::mir::BasicBlockId, MirInstruction),
}
impl CompletedArrayElementWriteV1 {
    pub(in crate::mir) fn site(&self) -> ArrayWriteSiteId { self.site }
    pub(in crate::mir) fn original(&self) -> &(crate::mir::BasicBlockId, MirInstruction) { &self.original }
}

impl MirBuilder {
    /// Raw and Plan calls join the same three original children to the sole
    /// Array writer observation. No physical value grants source permission.
    pub(in crate::mir::builder) fn complete_borrowed_array_set_v1(
        &mut self, ledger: &crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1,
        source: crate::mir::normal_callable_semantic_package::BorrowedArraySetSourceLoanV1,
        children: [ValueId; 3], write: CompletedArrayElementWriteV1,
    ) -> Result<(), String> {
        let MirInstruction::ArrayElementWrite { receiver, index: Some(index), value, .. } = write.original().1
            else { return Err("[freeze:contract][borrowed-set/write-shape]".into()) };
        let function = self.function_state.current_function.as_ref()
            .ok_or("[freeze:contract][borrowed-set/function-missing]")?;
        let correspondence = &mut self.function_state.local_ssa_correspondence;
        let mut relations = Vec::new();
        for (child, operand) in children.into_iter().zip([receiver, index, value]) {
            let relation = correspondence.loan(function, child, operand)
                .ok_or("[freeze:contract][borrowed-set/operand-relation]")?;
            correspondence.require_member(&relation)?;
            relations.push(relation);
        }
        ledger.record_borrowed_array_set_v1(source, write, relations.try_into()
                .map_err(|_| "[freeze:contract][borrowed-set/operand-cardinality]")?, function)
    }

    pub(super) fn emit_array_element_write(
        &mut self,
        dst: Option<ValueId>,
        kind: ArrayElementWriteKind,
        producer: ArrayWriteProducerKind,
        receiver: ValueId,
        index: Option<ValueId>,
        value: ValueId,
    ) -> Result<ArrayWriteSiteId, String> {
        self.emit_array_element_write_recorded_v1(dst, kind, producer, receiver, index, value)
            .map(|completed| completed.site())
    }

    pub(in crate::mir::builder) fn emit_array_element_write_recorded_v1(
        &mut self, dst: Option<ValueId>, kind: ArrayElementWriteKind,
        producer: ArrayWriteProducerKind, receiver: ValueId, index: Option<ValueId>, value: ValueId,
    ) -> Result<CompletedArrayElementWriteV1, String> {
        let site_id = self.next_array_write_site_id();
        let instruction = crate::mir::array_element_write::instruction(
            site_id, dst, kind, producer, receiver, index, value,
        )?;
        let block = self.function_state.current_block.ok_or("No current basic block")?;
        let original = (block, instruction.clone());
        self.emit_instruction(instruction)?;
        if let Some(dst) = dst {
            self.function_state
                .type_ctx
                .value_types
                .insert(dst, MirType::Void);
            if let Some(function) = self.function_state.current_function.as_mut() {
                function.metadata.value_types.insert(dst, MirType::Void);
            }
        }
        Ok(CompletedArrayElementWriteV1 { site: site_id, original })
    }

    pub(super) fn try_emit_known_array_method_write(
        &mut self,
        dst: Option<ValueId>,
        receiver: ValueId,
        method: &str,
        args: &[ValueId],
    ) -> Result<bool, String> {
        self.try_emit_known_array_method_write_recorded_v1(dst, receiver, method, args)
            .map(|completed| completed.is_some())
    }

    pub(in crate::mir::builder) fn try_emit_known_array_method_write_recorded_v1(
        &mut self, dst: Option<ValueId>, receiver: ValueId, method: &str, args: &[ValueId],
    ) -> Result<Option<CompletedArrayElementWriteV1>, String> {
        let Some(method_id) =
            crate::boxes::array::ArrayMethodId::from_name_and_arity(method, args.len())
        else {
            return Ok(None);
        };
        let (kind, index, value) = match method_id {
            crate::boxes::array::ArrayMethodId::Push => {
                (ArrayElementWriteKind::Push, None, args[0])
            }
            crate::boxes::array::ArrayMethodId::Set => {
                (ArrayElementWriteKind::Set, Some(args[0]), args[1])
            }
            crate::boxes::array::ArrayMethodId::Insert => {
                (ArrayElementWriteKind::Insert, Some(args[0]), args[1])
            }
            _ => return Ok(None),
        };
        let completed = self.emit_array_element_write_recorded_v1(
            dst,
            kind,
            ArrayWriteProducerKind::MethodCall,
            receiver,
            index,
            value,
        )?;
        Ok(Some(completed))
    }

    pub(in crate::mir::builder) fn emit_checked_field_array_read_v1(&mut self, recipe: &crate::mir::normal_callable_semantic_package::FieldArrayReadRecipeV1, receiver: ValueId, index: ValueId, dst: ValueId) -> Result<ArrayReadSiteId, String> {
        if self.function_state.type_ctx.value_types.get(&index) != Some(&MirType::Integer) { return Err(crate::mir::named_array_obligation::fault("read-index-type")); }
        let read = self.emit_array_element_read(Some(dst), receiver, index)?;
        self.function_state.type_ctx.value_types.insert(dst, MirType::Integer);
        let function = self.function_state.current_function.as_mut().ok_or_else(|| crate::mir::named_array_obligation::fault("read-function-missing"))?;
        function.metadata.value_types.insert(dst, MirType::Integer);
        function.metadata.named_array_read_obligations.push(crate::mir::named_array_obligation::NamedArrayReadMarkerV1 { owner: recipe.owner(), read, receiver, index, dst });
        Ok(read)
    }

    pub(super) fn emit_array_element_read(
        &mut self,
        dst: Option<ValueId>,
        receiver: ValueId,
        index: ValueId,
    ) -> Result<ArrayReadSiteId, String> {
        let site_id = self.next_array_read_site_id();
        self.emit_instruction(MirInstruction::ArrayRead {
            site_id,
            dst,
            receiver,
            operation: crate::mir::instruction::ArrayReadOperationV1::Element { index },
        })?;
        if let Some(dst) = dst {
            self.function_state
                .type_ctx
                .value_types
                .insert(dst, MirType::Unknown);
            if let Some(function) = self.function_state.current_function.as_mut() {
                function.metadata.value_types.insert(dst, MirType::Unknown);
            }
        }
        Ok(site_id)
    }

    /// `ArrayBox.get/1` on a receiver whose `Callee::Method` already proved
    /// `ArrayBox` — the caller gates the box name; this arm only checks the
    /// method identity, the same contract the write arm keeps.
    pub(super) fn try_emit_known_array_method_read(
        &mut self,
        dst: Option<ValueId>,
        receiver: ValueId,
        method: &str,
        args: &[ValueId],
    ) -> Result<bool, String> {
        if crate::boxes::array::ArrayMethodId::from_name_and_arity(method, args.len())
            != Some(crate::boxes::array::ArrayMethodId::Get)
        {
            return Ok(false);
        }
        self.emit_array_element_read(dst, receiver, args[0])?;
        Ok(true)
    }

    pub(super) fn next_array_read_site_id(&self) -> ArrayReadSiteId {
        let next = self
            .function_state
            .current_function
            .as_ref()
            .into_iter()
            .flat_map(|function| function.blocks.values())
            .flat_map(|block| block.all_instructions())
            .filter_map(|instruction| match instruction {
                MirInstruction::ArrayRead { site_id, .. } => Some(site_id.0),
                _ => None,
            })
            .max()
            .map_or(0, |site| site.saturating_add(1));
        ArrayReadSiteId::new(next)
    }

    pub(super) fn next_array_write_site_id(&self) -> ArrayWriteSiteId {
        let next = self
            .function_state
            .current_function
            .as_ref()
            .into_iter()
            .flat_map(|function| function.blocks.values())
            .flat_map(|block| block.all_instructions())
            .filter_map(|instruction| match instruction {
                MirInstruction::ArrayElementWrite { site_id, .. }
                | MirInstruction::Invoke {
                    operation:
                        crate::mir::instruction::InvokeOperation::ArrayElementWrite { site_id, .. },
                    ..
                } => Some(site_id.0),
                _ => None,
            })
            .max()
            .map_or(0, |site| site.saturating_add(1));
        ArrayWriteSiteId::new(next)
    }
}

#[cfg(test)]
#[path = "array_element_write_tests.rs"]
mod tests;
