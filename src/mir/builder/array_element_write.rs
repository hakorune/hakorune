use super::{MirBuilder, ValueId};
use crate::mir::{
    ArrayElementWriteKind, ArrayWriteProducerKind, ArrayWriteSiteId, MirInstruction, MirType,
};

impl MirBuilder {
    pub(super) fn emit_array_element_write(
        &mut self,
        dst: Option<ValueId>,
        kind: ArrayElementWriteKind,
        producer: ArrayWriteProducerKind,
        receiver: ValueId,
        index: Option<ValueId>,
        value: ValueId,
    ) -> Result<(), String> {
        let site_id = self.next_array_write_site_id();
        let instruction = crate::mir::array_element_write::instruction(
            site_id, dst, kind, producer, receiver, index, value,
        )?;
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
        Ok(())
    }

    pub(super) fn try_emit_known_array_method_write(
        &mut self,
        dst: Option<ValueId>,
        receiver: ValueId,
        method: &str,
        args: &[ValueId],
    ) -> Result<bool, String> {
        let Some(method_id) =
            crate::boxes::array::ArrayMethodId::from_name_and_arity(method, args.len())
        else {
            return Ok(false);
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
            _ => return Ok(false),
        };
        self.emit_array_element_write(
            dst,
            kind,
            ArrayWriteProducerKind::MethodCall,
            receiver,
            index,
            value,
        )?;
        Ok(true)
    }

    fn next_array_write_site_id(&self) -> ArrayWriteSiteId {
        let next = self
            .function_state
            .current_function
            .as_ref()
            .into_iter()
            .flat_map(|function| function.blocks.values())
            .flat_map(|block| block.instructions.iter())
            .filter_map(|instruction| match instruction {
                MirInstruction::ArrayElementWrite { site_id, .. } => Some(site_id.0),
                _ => None,
            })
            .max()
            .map_or(0, |site| site.saturating_add(1));
        ArrayWriteSiteId::new(next)
    }
}
