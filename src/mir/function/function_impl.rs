use super::{FunctionMetadata, FunctionSignature, FunctionStats, MirFunction};
use crate::mir::{BasicBlock, BasicBlockId, MirInstruction, ValueId};
use std::collections::HashMap;

impl MirFunction {
    /// Create a new MIR function
    pub fn new(signature: FunctionSignature, entry_block: BasicBlockId) -> Self {
        let mut blocks = HashMap::new();
        blocks.insert(entry_block, BasicBlock::new(entry_block));

        // Reserve ValueIds exactly for `signature.params` (%0..%N-1).
        // `signature.params` is the SSOT for callable arity and already includes
        // `me` for instance methods lowered by MirBuilder.
        let param_count = signature.params.len() as u32;
        let total_value_ids = param_count;
        let initial_counter = total_value_ids.max(1); // At least 1 to reserve ValueId(0) as sentinel

        // Hotfix 5: Pre-populate params vector with reserved ValueIds.
        // Without this, setup_function_params() will allocate NEW ValueIds starting from
        // the already-incremented counter, causing signature/body mismatch.
        let mut pre_params = Vec::new();
        for i in 0..total_value_ids {
            pre_params.push(ValueId::new(i));
        }

        if std::env::var("NYASH_LOOPFORM_DEBUG").is_ok() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[MirFunction::new] fn='{}' params={}, receiver={}, total={}, initial_counter={}, pre_params={:?}",
                signature.name,
                param_count,
                0,
                total_value_ids,
                initial_counter,
                pre_params
            ));
        }

        Self {
            signature,
            blocks,
            entry_block,
            locals: Vec::new(),
            params: pre_params,
            next_value_id: initial_counter,
            metadata: FunctionMetadata::default(),
        }
    }

    /// Get the next available ValueId
    pub fn next_value_id(&mut self) -> ValueId {
        let id = ValueId::new(self.next_value_id);
        self.next_value_id += 1;
        id
    }

    /// Reserve ValueIds for function parameters (Hotfix 1: Parameter ValueId Reservation)
    ///
    /// Call this after setting params to ensure next_value_id doesn't overlap
    /// with parameter ValueIds. This prevents SSA violations where local variables
    /// overwrite parameter values.
    ///
    /// # Box-First理論
    /// - 「境界をはっきりさせる」: パラメータ予約を明示的に
    /// - 「いつでも戻せる」: 呼び出しタイミングを制御可能
    pub fn reserve_parameter_value_ids(&mut self) {
        let param_count = self.signature.params.len();
        if self.next_value_id < param_count as u32 {
            if std::env::var("NYASH_LOOPFORM_DEBUG").is_ok() {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!(
                    "[MirFunction::reserve_parameter_value_ids] fn='{}' reserving {} params, adjusting counter {} -> {}",
                    self.signature.name,
                    param_count,
                    self.next_value_id,
                    param_count
                ));
            }
            self.next_value_id = param_count as u32;
        }
    }

    /// Add a new basic block
    pub fn add_block(&mut self, block: BasicBlock) -> BasicBlockId {
        let id = block.id;
        if self.blocks.contains_key(&id)
            && std::env::var("NYASH_LOCAL_SSA_TRACE").ok().as_deref() == Some("1")
        {
            let ring0 = crate::runtime::get_global_ring0();
            ring0
                .log
                .debug(&format!("[mir-function] replacing existing block {:?}", id));
        }
        self.blocks.insert(id, block);
        id
    }

    /// Get a basic block by ID
    pub fn get_block(&self, id: BasicBlockId) -> Option<&BasicBlock> {
        self.blocks.get(&id)
    }

    /// Get a mutable basic block by ID
    pub fn get_block_mut(&mut self, id: BasicBlockId) -> Option<&mut BasicBlock> {
        self.blocks.get_mut(&id)
    }

    /// Get the entry block
    pub fn entry_block(&self) -> &BasicBlock {
        self.blocks
            .get(&self.entry_block)
            .expect("Entry block must exist")
    }

    /// Get all basic block IDs in insertion order
    pub fn block_ids(&self) -> Vec<BasicBlockId> {
        let mut ids: Vec<_> = self.blocks.keys().copied().collect();
        ids.sort();
        ids
    }

    /// Get all values defined in this function
    pub fn defined_values(&self) -> Vec<ValueId> {
        let mut values = Vec::new();
        values.extend(&self.params);

        for block in self.blocks.values() {
            values.extend(block.defined_values());
        }

        values
    }

    /// Verify function integrity (basic checks)
    pub fn verify(&self) -> Result<(), String> {
        // Check entry block exists
        if !self.blocks.contains_key(&self.entry_block) {
            return Err("Entry block does not exist".to_string());
        }

        // Check all blocks are reachable from entry
        let reachable = self.compute_reachable_blocks();
        for (id, _block) in &self.blocks {
            if !reachable.contains(id) {
                let ring0 = crate::runtime::get_global_ring0();
                ring0
                    .log
                    .debug(&format!("Warning: Block {} is unreachable", id));
            }
        }

        // Check terminator consistency
        for block in self.blocks.values() {
            if !block.is_terminated() && !block.is_empty() {
                return Err(format!("Block {} is not properly terminated", block.id));
            }

            // Check successor/predecessor consistency
            for successor_id in &block.successors {
                if let Some(successor) = self.blocks.get(successor_id) {
                    if !successor.predecessors.contains(&block.id) {
                        return Err(format!(
                            "Inconsistent CFG: {} -> {} but {} doesn't have {} as predecessor",
                            block.id, successor_id, successor_id, block.id
                        ));
                    }
                } else {
                    return Err(format!(
                        "Block {} references non-existent successor {}",
                        block.id, successor_id
                    ));
                }
            }
        }

        Ok(())
    }

    /// Compute reachable blocks from entry
    fn compute_reachable_blocks(&self) -> std::collections::HashSet<BasicBlockId> {
        crate::mir::verification::utils::compute_reachable_blocks(self)
    }

    /// Update predecessor/successor relationships
    pub fn update_cfg(&mut self) {
        // Clear all predecessors
        for block in self.blocks.values_mut() {
            block.predecessors.clear();
        }

        // Rebuild predecessors from successors
        let edges: Vec<(BasicBlockId, BasicBlockId)> = self
            .blocks
            .values()
            .flat_map(|block| block.successors.iter().map(move |&succ| (block.id, succ)))
            .collect();

        for (pred, succ) in edges {
            if let Some(successor_block) = self.blocks.get_mut(&succ) {
                successor_block.add_predecessor(pred);
            }
        }
    }

    /// Mark reachable blocks
    pub fn mark_reachable_blocks(&mut self) {
        let reachable = self.compute_reachable_blocks();
        for (id, block) in &mut self.blocks {
            if reachable.contains(id) {
                block.mark_reachable();
            }
        }
    }

    /// Prune blocks that are unreachable from the entry block.
    ///
    /// This is a structural CFG cleanup helper. Callers remain responsible for
    /// any module-level semantic refresh after block removal.
    pub fn prune_unreachable_blocks(&mut self) -> usize {
        let reachable = self.compute_reachable_blocks();
        let before = self.blocks.len();

        self.blocks.retain(|id, block| {
            let keep = reachable.contains(id);
            block.reachable = keep;
            keep
        });

        if self.blocks.len() != before {
            self.update_cfg();
        }

        before - self.blocks.len()
    }

    /// Get function statistics
    pub fn stats(&self) -> FunctionStats {
        let instruction_count = self
            .blocks
            .values()
            .map(|block| block.instructions.len() + if block.terminator.is_some() { 1 } else { 0 })
            .sum();

        let phi_count = self
            .blocks
            .values()
            .map(|block| block.phi_instructions().count())
            .sum();

        FunctionStats {
            block_count: self.blocks.len(),
            instruction_count,
            phi_count,
            value_count: self.next_value_id as usize,
            is_pure: self.signature.effects.is_pure(),
        }
    }

    /// Set jump terminator if block is not already terminated.
    /// Helper for JSON v0 Bridge and loop lowering.
    pub fn set_jump_terminator(
        &mut self,
        bb_id: BasicBlockId,
        target: BasicBlockId,
    ) -> Result<(), String> {
        if let Some(bb) = self.get_block_mut(bb_id) {
            if !bb.is_terminated() {
                bb.set_terminator(MirInstruction::Jump {
                    target,
                    edge_args: None,
                });
            }
            Ok(())
        } else {
            Err(format!("Block {:?} not found", bb_id))
        }
    }

    /// Set branch terminator.
    /// Helper for JSON v0 Bridge if/else lowering.
    pub fn set_branch_terminator(
        &mut self,
        bb_id: BasicBlockId,
        condition: ValueId,
        then_bb: BasicBlockId,
        else_bb: BasicBlockId,
    ) -> Result<(), String> {
        if let Some(bb) = self.get_block_mut(bb_id) {
            bb.set_terminator(MirInstruction::Branch {
                condition,
                then_bb,
                else_bb,
                then_edge_args: None,
                else_edge_args: None,
            });
            Ok(())
        } else {
            Err(format!("Block {:?} not found", bb_id))
        }
    }
}
