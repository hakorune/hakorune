//! Small loop utilities for MirBuilder
use super::{BasicBlockId, MirBuilder};

/// Push loop context (header/exit) onto the MirBuilder stacks.
#[allow(dead_code)]
pub(crate) fn push_loop_context(
    builder: &mut super::MirBuilder,
    header: BasicBlockId,
    exit: BasicBlockId,
) {
    builder.scope_ctx.loop_header_stack.push(header);
    builder.scope_ctx.loop_exit_stack.push(exit);
}

/// Pop loop context (header/exit) from the MirBuilder stacks.
#[allow(dead_code)]
pub(crate) fn pop_loop_context(builder: &mut super::MirBuilder) {
    let _ = builder.scope_ctx.loop_header_stack.pop();
    let _ = builder.scope_ctx.loop_exit_stack.pop();
}

/// Peek current loop header block id
#[allow(dead_code)]
#[allow(dead_code)]
pub(crate) fn current_header(builder: &super::MirBuilder) -> Option<BasicBlockId> {
    builder.scope_ctx.loop_header_stack.last().copied()
}

/// Peek current loop exit block id
#[allow(dead_code)]
pub(crate) fn current_exit(builder: &super::MirBuilder) -> Option<BasicBlockId> {
    builder.scope_ctx.loop_exit_stack.last().copied()
}

/// Returns true if the builder is currently inside at least one loop context.
#[allow(dead_code)]
#[allow(dead_code)]
pub(crate) fn in_loop(builder: &super::MirBuilder) -> bool {
    !builder.scope_ctx.loop_header_stack.is_empty()
}

/// Current loop nesting depth (0 means not in a loop).
#[allow(dead_code)]
#[allow(dead_code)]
pub(crate) fn depth(builder: &super::MirBuilder) -> usize {
    builder.scope_ctx.loop_header_stack.len()
}

/// Add predecessor edge metadata to a basic block.
/// 📦 Hotfix 6: Auto-create block if it doesn't exist yet
/// This ensures add_predecessor() works even before start_new_block() is called.
#[allow(dead_code)]
pub(crate) fn add_predecessor(
    builder: &mut MirBuilder,
    block: BasicBlockId,
    pred: BasicBlockId,
) -> Result<(), String> {
    if let Some(ref mut function) = builder.scope_ctx.current_function {
        // 📦 Hotfix 6: Ensure block exists (same as start_new_block logic)
        // Create block if not present, without changing current_block
        if !function.blocks.contains_key(&block) {
            function.add_block(super::BasicBlock::new(block));
        }

        if let Some(bb) = function.get_block_mut(block) {
            bb.add_predecessor(pred);
            return Ok(());
        }
        return Err(format!(
            "Block {} not found (impossible after auto-create)",
            block
        ));
    }
    Err("No current function".to_string())
}
