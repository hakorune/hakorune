/*!
 * Loop Builder Facade (Minimal API)
 *
 * Goal: Decouple loop construction from concrete MirBuilder types by exposing
 * a small trait that both legacy (mir::builder::MirBuilder) and modularized
 * builders can implement. This enables shared helpers and gradual migration.
 *
 * Note: Only legacy MirBuilder is wired for now to keep WIP modularized code
 * out of the main build. The modularized builder can implement this trait
 * later without changing callers.
 */

use super::{BasicBlockId, MirInstruction, ValueId};

/// Minimal API for constructing loops and emitting instructions
pub trait LoopBuilderApi {
    /// Allocate a new basic block id
    fn new_block(&mut self) -> BasicBlockId;
    /// Get current block id
    fn current_block(&self) -> Result<BasicBlockId, String>;
    /// Switch current block, creating it if needed
    fn start_new_block(&mut self, block: BasicBlockId) -> Result<(), String>;
    /// Emit an instruction to the current block
    fn emit(&mut self, inst: MirInstruction) -> Result<(), String>;
    /// Allocate a new SSA value id
    fn new_value(&mut self) -> ValueId;

    /// Add predecessor edge to a block (CFG maintenance)
    fn add_predecessor(&mut self, _block: BasicBlockId, _pred: BasicBlockId) -> Result<(), String> {
        Err("add_predecessor not implemented".into())
    }
    /// Seal a block when all predecessors are known
    fn seal_block(&mut self, _block: BasicBlockId) -> Result<(), String> {
        Err("seal_block not implemented".into())
    }
    /// Insert a phi at block start
    fn insert_phi_at_block_start(
        &mut self,
        _block: BasicBlockId,
        _dst: ValueId,
        _inputs: Vec<(BasicBlockId, ValueId)>,
    ) -> Result<(), String> {
        Err("insert_phi_at_block_start not implemented".into())
    }
}

/// Helper: simplified loop lowering usable by any LoopBuilderApi implementor
pub fn build_simple_loop<L: LoopBuilderApi>(
    lb: &mut L,
    condition: ValueId,
    build_body: &mut dyn FnMut(&mut L) -> Result<(), String>,
) -> Result<ValueId, String> {
    let header = lb.new_block();
    let body = lb.new_block();
    let after = lb.new_block();

    // Jump to header
    lb.emit(MirInstruction::Jump {
        target: header,
        edge_args: None,
    })?;

    // Header: branch on provided condition
    lb.start_new_block(header)?;
    lb.emit(MirInstruction::Branch {
        condition,
        then_bb: body,
        else_bb: after,
        then_edge_args: None,
        else_edge_args: None,
    })?;

    // Body
    lb.start_new_block(body)?;
    build_body(lb)?;
    lb.emit(MirInstruction::Jump {
        target: header,
        edge_args: None,
    })?;

    // After: return void value
    lb.start_new_block(after)?;
    let void_id = lb.new_value();
    lb.emit(MirInstruction::Const {
        dst: void_id,
        value: crate::mir::ConstValue::Void,
    })?;
    Ok(void_id)
}

// Legacy wiring for `MirBuilder` lives in `src/mir/builder/loop_api_impl.rs`
// so that emit/CFG mutations stay inside `src/mir/builder/**`.
