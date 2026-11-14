//! Inline source-interval receipts of the existing expression body.
use super::*;

// Inline intervals begin before committed read copies. Only the existing
// function owner can later insert the shared frame at the entry/Phi prefix.
pub(in crate::mir::builder::ordinary_new_admission::selected) fn begin_inline_scope_v1(
    builder: &MirBuilder,
    state: &CallableSemanticLoweringState,
) -> Result<(BasicBlockId, usize, bool), String> {
    let block = builder
        .function_state
        .current_block
        .ok_or_else(|| freeze("no-block"))?;
    let function = builder
        .function_state
        .current_function
        .as_ref()
        .ok_or_else(|| freeze("no-function"))?;
    let offset = function
        .blocks
        .get(&block)
        .ok_or_else(|| freeze("no-block"))?
        .instructions
        .len();
    let had_entry_frame =
        block == function.entry_block && state.observed_fault_frame_binding_v2(function)?.is_some();
    Ok((block, offset, had_entry_frame))
}

pub(in crate::mir::builder::ordinary_new_admission::selected) fn finish_inline_scope_v1(
    builder: &MirBuilder,
    start: Option<(BasicBlockId, usize, bool)>,
) -> Result<Option<(BasicBlockId, usize, usize, bool)>, String> {
    start
        .map(|(block, start, had_entry_frame)| {
            if builder.function_state.current_block != Some(block) {
                return Err(freeze("expression/nonfault-control"));
            }
            let end = builder
                .function_state
                .current_function
                .as_ref()
                .and_then(|function| function.blocks.get(&block))
                .ok_or_else(|| freeze("no-block"))?
                .instructions
                .len();
            Ok((block, start, end, had_entry_frame))
        })
        .transpose()
}

impl EmittedExpressionBodyV1 {
    pub(in crate::mir::builder::ordinary_new_admission::selected) fn complete_inline_range_v1(
        &mut self,
        builder: &MirBuilder,
    ) -> Result<(), String> {
        self.inline_range = finish_inline_scope_v1(
            builder,
            self.inline_range
                .map(|(block, start, _, had_frame)| (block, start, had_frame)),
        )?;
        Ok(())
    }
    pub(in crate::mir::builder) fn required_fault_frame_v1(&self) -> Result<ValueId, String> {
        self.fault_frame
            .ok_or_else(|| freeze("expression/fault-frame-required"))
    }
}
