/*!
 * MIR Basic Block - Control Flow Graph Building Block
 *
 * SSA-form basic blocks with phi functions and terminator instructions
 */

use super::spanned_instruction::{SpannedInstRef, SpannedInstruction};
use super::{EffectMask, MirInstruction, ValueId};
use crate::ast::Span;
use crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
use crate::runtime::get_global_ring0;
pub use hakorune_mir_core::{BasicBlockId, BasicBlockIdGenerator};
use std::collections::BTreeSet; // Phase 69-3: HashSet → BTreeSet for determinism
use std::fmt;

/// Edge arguments for CFG edges (Phase 260 P0)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EdgeArgs {
    pub layout: JumpArgsLayout,
    pub values: Vec<ValueId>,
}

/// Outgoing edge from a basic block
#[derive(Debug, Clone)]
pub struct OutEdge {
    pub target: BasicBlockId,
    pub args: Option<EdgeArgs>,
}

/// A basic block in SSA form
#[derive(Debug, Clone)]
pub struct BasicBlock {
    /// Unique identifier for this block
    pub id: BasicBlockId,

    /// Instructions in this block (excluding terminator)
    pub instructions: Vec<MirInstruction>,

    /// Span per instruction (aligned with `instructions`)
    pub instruction_spans: Vec<Span>,

    /// Terminator instruction (branch, jump, or return)
    pub terminator: Option<MirInstruction>,

    /// Span for the terminator instruction
    pub terminator_span: Option<Span>,

    /// Predecessors in the control flow graph
    /// Phase 69-3: BTreeSet for deterministic iteration order
    pub predecessors: BTreeSet<BasicBlockId>,

    /// Successors in the control flow graph
    /// Phase 69-3: BTreeSet for deterministic iteration order
    pub successors: BTreeSet<BasicBlockId>,

    /// Combined effect mask for all instructions in this block
    pub effects: EffectMask,

    /// Whether this block is reachable from the entry block
    pub reachable: bool,

    /// Is this block sealed? (all predecessors are known)
    pub sealed: bool,

    /// Phase 260 P2: Return environment metadata
    /// Return has no edge-args operand, so we keep metadata for continuation.
    /// When a JoinIR Jump is converted to MIR Return, this field preserves
    /// all the Jump args (not just the first one) so that exit PHI can correctly
    /// merge carrier values from multiple exit paths.
    pub return_env: Option<Vec<ValueId>>,
    /// Phase 260 P2: Layout for return environment
    pub return_env_layout: Option<JumpArgsLayout>,
}

impl BasicBlock {
    /// Create a new basic block
    pub fn new(id: BasicBlockId) -> Self {
        Self {
            id,
            instructions: Vec::new(),
            instruction_spans: Vec::new(),
            terminator: None,
            terminator_span: None,
            predecessors: BTreeSet::new(), // Phase 69-3: BTreeSet for determinism
            successors: BTreeSet::new(),   // Phase 69-3: BTreeSet for determinism
            effects: EffectMask::PURE,
            reachable: false,
            sealed: false,
            return_env: None,        // Phase 260 P2: No return env by default
            return_env_layout: None, // Phase 260 P2: Unknown by default
        }
    }

    /// Add a spanned instruction to this block
    pub fn add_spanned_instruction(&mut self, sp: SpannedInstruction) {
        self.add_instruction_with_span(sp.inst, sp.span);
    }

    /// Add an instruction to this block
    pub fn add_instruction(&mut self, instruction: MirInstruction) {
        self.add_instruction_with_span(instruction, Span::unknown());
    }

    /// Add an instruction with explicit span to this block
    pub fn add_instruction_with_span(&mut self, instruction: MirInstruction, span: Span) {
        // Update effect mask
        self.effects = self.effects | instruction.effects();

        // Check if this is a terminator instruction
        if self.is_terminator(&instruction) {
            if self.terminator.is_some() {
                panic!("Basic block {} already has a terminator", self.id);
            }
            self.terminator = Some(instruction);
            self.terminator_span = Some(span);

            // Update successors based on terminator
            self.update_successors_from_terminator();
        } else {
            self.instructions.push(instruction);
            self.instruction_spans.push(span);
        }
    }

    /// Add instruction before terminator (for edge-copy in PHI-off mode)
    /// If no terminator exists, behaves like add_instruction()
    pub fn add_instruction_before_terminator(&mut self, instruction: MirInstruction) {
        // Update effect mask
        self.effects = self.effects | instruction.effects();

        // Non-terminator instructions always go into instructions vec
        if !self.is_terminator(&instruction) {
            self.instructions.push(instruction);
            self.instruction_spans.push(Span::unknown());
        } else {
            panic!("Cannot add terminator via add_instruction_before_terminator");
        }
    }

    /// Check if an instruction is a terminator
    fn is_terminator(&self, instruction: &MirInstruction) -> bool {
        matches!(
            instruction,
            MirInstruction::Branch { .. }
                | MirInstruction::Jump { .. }
                | MirInstruction::Return { .. }
                | MirInstruction::Throw { .. }
        )
    }

    /// Update successors based on the terminator instruction
    fn update_successors_from_terminator(&mut self) {
        self.successors = self.successors_from_terminator();
    }

    /// Compute successors from the terminator (SSOT for CFG verification)
    pub fn successors_from_terminator(&self) -> BTreeSet<BasicBlockId> {
        let mut successors = BTreeSet::new();

        if let Some(ref terminator) = self.terminator {
            match terminator {
                MirInstruction::Branch {
                    then_bb, else_bb, ..
                } => {
                    successors.insert(*then_bb);
                    successors.insert(*else_bb);
                }
                MirInstruction::Jump { target, .. } => {
                    successors.insert(*target);
                }
                MirInstruction::Return { .. } => {
                    // No successors for return
                }
                MirInstruction::Throw { .. } => {
                    // No normal successors for throw - control goes to exception handlers
                    // Exception edges are handled separately from normal control flow
                }
                _ => unreachable!("Non-terminator instruction in terminator position"),
            }
        }

        successors
    }

    /// Enumerate all outgoing CFG edges
    pub fn out_edges(&self) -> Vec<OutEdge> {
        match self.terminator {
            Some(MirInstruction::Branch {
                then_bb,
                else_bb,
                ref then_edge_args,
                ref else_edge_args,
                ..
            }) => vec![
                OutEdge {
                    target: then_bb,
                    args: then_edge_args.clone(),
                },
                OutEdge {
                    target: else_bb,
                    args: else_edge_args.clone(),
                },
            ],
            Some(MirInstruction::Jump {
                target,
                ref edge_args,
                ..
            }) => vec![OutEdge {
                target,
                args: edge_args.clone(), // Phase 260 P2: No fallback, terminator SSOT
            }],
            _ => Vec::new(),
        }
    }

    /// Get edge args for a specific target (if present)
    pub fn edge_args_to(&self, target: BasicBlockId) -> Option<EdgeArgs> {
        self.out_edges()
            .into_iter()
            .find(|edge| edge.target == target)
            .and_then(|edge| edge.args)
    }

    /// Set Return environment metadata (Return-specific)
    ///
    /// Return has no edge-args operand, so we keep this metadata for continuation.
    pub fn set_return_env(&mut self, env: EdgeArgs) {
        if !matches!(self.terminator, Some(MirInstruction::Return { .. })) {
            panic!("set_return_env requires Return terminator");
        }
        self.return_env = Some(env.values);
        self.return_env_layout = Some(env.layout);
    }

    /// Get Return environment metadata
    pub fn return_env(&self) -> Option<EdgeArgs> {
        if matches!(self.terminator, Some(MirInstruction::Return { .. })) {
            match (self.return_env.as_ref(), self.return_env_layout) {
                (Some(values), Some(layout)) => Some(EdgeArgs {
                    layout,
                    values: values.clone(),
                }),
                _ => None,
            }
        } else {
            None
        }
    }

    /// Set jump terminator with edge args (SSOT write helper)
    pub fn set_jump_with_edge_args(&mut self, target: BasicBlockId, edge_args: Option<EdgeArgs>) {
        let terminator = MirInstruction::Jump {
            target,
            edge_args: edge_args.clone(),
        };
        if !self.is_terminator(&terminator) {
            panic!("Instruction is not a valid terminator: {:?}", terminator);
        }

        self.effects = self.effects | terminator.effects();
        self.terminator = Some(terminator);
        self.terminator_span = Some(self.fallback_terminator_span());
        self.update_successors_from_terminator();
    }

    /// Set branch terminator with per-edge args
    pub fn set_branch_with_edge_args(
        &mut self,
        condition: ValueId,
        then_bb: BasicBlockId,
        then_edge_args: Option<EdgeArgs>,
        else_bb: BasicBlockId,
        else_edge_args: Option<EdgeArgs>,
    ) {
        let terminator = MirInstruction::Branch {
            condition,
            then_bb,
            else_bb,
            then_edge_args,
            else_edge_args,
        };
        if !self.is_terminator(&terminator) {
            panic!("Instruction is not a valid terminator: {:?}", terminator);
        }

        self.effects = self.effects | terminator.effects();
        self.terminator = Some(terminator);
        self.terminator_span = Some(self.fallback_terminator_span());
        self.update_successors_from_terminator();
    }

    /// Get edge-args from the current terminator
    ///
    /// Jump uses its edge-args operand. Return uses return_env metadata.
    pub fn edge_args_from_terminator(&self) -> Option<EdgeArgs> {
        match self.terminator {
            Some(MirInstruction::Jump { ref edge_args, .. }) => edge_args.clone(),
            Some(MirInstruction::Return { .. }) => self.return_env(),
            _ => None,
        }
    }

    /// Add a predecessor
    pub fn add_predecessor(&mut self, pred: BasicBlockId) {
        self.predecessors.insert(pred);
    }

    /// Remove a predecessor
    pub fn remove_predecessor(&mut self, pred: BasicBlockId) {
        self.predecessors.remove(&pred);
    }

    /// Get all instructions including terminator
    pub fn all_instructions(&self) -> impl Iterator<Item = &MirInstruction> {
        self.instructions.iter().chain(self.terminator.iter())
    }

    /// Get all values defined in this block
    pub fn defined_values(&self) -> Vec<ValueId> {
        self.all_instructions()
            .filter_map(|inst| inst.dst_value())
            .collect()
    }

    /// Get all values used in this block
    pub fn used_values(&self) -> Vec<ValueId> {
        self.all_instructions()
            .flat_map(|inst| inst.used_values())
            .collect()
    }

    /// Check if this block is empty (no instructions)
    pub fn is_empty(&self) -> bool {
        self.instructions.is_empty() && self.terminator.is_none()
    }

    /// Check if this block has a terminator
    pub fn is_terminated(&self) -> bool {
        self.terminator.is_some()
    }

    /// Check if this block ends with a return
    pub fn ends_with_return(&self) -> bool {
        matches!(self.terminator, Some(MirInstruction::Return { .. }))
    }

    /// Get the phi instructions at the beginning of this block
    pub fn phi_instructions(&self) -> impl Iterator<Item = &MirInstruction> {
        self.instructions
            .iter()
            .take_while(|inst| matches!(inst, MirInstruction::Phi { .. }))
    }

    /// Get non-phi instructions
    pub fn non_phi_instructions(&self) -> impl Iterator<Item = &MirInstruction> {
        self.instructions
            .iter()
            .skip_while(|inst| matches!(inst, MirInstruction::Phi { .. }))
    }

    /// Get span for instruction index (phi/non-phi)
    pub fn instruction_span(&self, idx: usize) -> Option<Span> {
        self.instruction_spans.get(idx).copied()
    }

    /// Get instruction with span by index.
    pub fn instruction_with_span(&self, idx: usize) -> Option<SpannedInstRef<'_>> {
        self.instructions
            .get(idx)
            .zip(self.instruction_spans.get(idx))
            .map(|(inst, span)| SpannedInstRef { inst, span: *span })
    }

    /// Get span for terminator instruction
    pub fn terminator_span(&self) -> Option<Span> {
        self.terminator_span
    }

    /// Get terminator together with its span.
    pub fn terminator_spanned(&self) -> Option<SpannedInstRef<'_>> {
        self.terminator.as_ref().map(|inst| SpannedInstRef {
            inst,
            span: self.terminator_span.unwrap_or_else(Span::unknown),
        })
    }

    /// Iterate instructions with their spans.
    pub fn iter_spanned(&self) -> impl Iterator<Item = SpannedInstRef<'_>> {
        self.instructions
            .iter()
            .zip(self.instruction_spans.iter())
            .map(|(inst, span)| SpannedInstRef { inst, span: *span })
    }

    /// Iterate instructions with index and span.
    pub fn iter_spanned_enumerated(&self) -> impl Iterator<Item = (usize, SpannedInstRef<'_>)> {
        self.iter_spanned().enumerate().map(|(idx, sp)| (idx, sp))
    }

    /// Iterate all instructions (including terminator) with spans.
    pub fn all_spanned_instructions(&self) -> impl Iterator<Item = SpannedInstRef<'_>> {
        self.iter_spanned()
            .chain(self.terminator_spanned().into_iter())
    }

    /// Iterate all instructions (including terminator) with index and span.
    /// Non-phi + phi + terminator share the same indexing as `all_instructions()`.
    pub fn all_spanned_instructions_enumerated(
        &self,
    ) -> impl Iterator<Item = (usize, SpannedInstRef<'_>)> {
        self.all_spanned_instructions().enumerate()
    }

    /// Insert instruction at the beginning (after phi instructions)
    pub fn insert_instruction_after_phis(&mut self, instruction: MirInstruction) {
        let phi_count = self.phi_instructions().count();
        if std::env::var("NYASH_SCHEDULE_TRACE").ok().as_deref() == Some("1") {
            if let MirInstruction::Copy { dst, src } = &instruction {
                get_global_ring0().log.debug(&format!(
                    "[insert-after-phis] bb={:?} phi_count={} inserting Copy dst=%{} src=%{} total_inst={}",
                    self.id,
                    phi_count,
                    dst.0,
                    src.0,
                    self.instructions.len()
                ));
            }
        }
        self.effects = self.effects | instruction.effects();
        self.instructions.insert(phi_count, instruction);
        self.instruction_spans.insert(phi_count, Span::unknown());
    }

    /// Insert spanned instruction at the beginning (after phi instructions)
    pub fn insert_spanned_after_phis(&mut self, sp: SpannedInstruction) {
        let phi_count = self.phi_instructions().count();
        if std::env::var("NYASH_SCHEDULE_TRACE").ok().as_deref() == Some("1") {
            if let MirInstruction::Copy { dst, src } = &sp.inst {
                get_global_ring0().log.debug(&format!(
                    "[insert-after-phis] bb={:?} phi_count={} inserting Copy dst=%{} src=%{} total_inst={}",
                    self.id, phi_count, dst.0, src.0, self.instructions.len()));
            }
        }
        self.effects = self.effects | sp.inst.effects();
        self.instructions.insert(phi_count, sp.inst);
        self.instruction_spans.insert(phi_count, sp.span);
    }

    /// Update PHI instruction input by destination ValueId
    /// Used for loop back-edge PHI updates
    pub fn update_phi_input(
        &mut self,
        phi_dst: ValueId,
        incoming: (BasicBlockId, ValueId),
    ) -> Result<(), String> {
        for inst in &mut self.instructions {
            if let MirInstruction::Phi { dst, inputs, .. } = inst {
                if *dst == phi_dst {
                    inputs.push(incoming);
                    return Ok(());
                }
            }
        }
        Err(format!(
            "PHI instruction with dst {:?} not found in block {:?}",
            phi_dst, self.id
        ))
    }

    /// Replace terminator instruction
    pub fn set_terminator(&mut self, terminator: MirInstruction) {
        if !self.is_terminator(&terminator) {
            panic!("Instruction is not a valid terminator: {:?}", terminator);
        }

        self.effects = self.effects | terminator.effects();
        self.terminator = Some(terminator);
        self.terminator_span = Some(self.fallback_terminator_span());
        self.update_successors_from_terminator();
    }

    /// Replace terminator with explicit span
    pub fn set_terminator_with_span(&mut self, terminator: MirInstruction, span: Span) {
        if !self.is_terminator(&terminator) {
            panic!("Instruction is not a valid terminator: {:?}", terminator);
        }
        self.effects = self.effects | terminator.effects();
        self.terminator = Some(terminator);
        self.terminator_span = Some(span);
        self.update_successors_from_terminator();
    }

    fn fallback_terminator_span(&self) -> Span {
        self.instruction_spans
            .last()
            .copied()
            .unwrap_or_else(Span::unknown)
    }

    /// Drain instructions into spanned form (keeps block empty and aligned).
    pub fn drain_spanned_instructions(&mut self) -> Vec<SpannedInstruction> {
        let insts = std::mem::take(&mut self.instructions);
        let spans = std::mem::take(&mut self.instruction_spans);
        insts
            .into_iter()
            .zip(spans.into_iter())
            .map(|(inst, span)| SpannedInstruction { inst, span })
            .collect()
    }

    /// Mark this block as reachable
    pub fn mark_reachable(&mut self) {
        self.reachable = true;
    }

    /// Seal this block (all predecessors are known)
    pub fn seal(&mut self) {
        self.sealed = true;
    }

    /// Check if this block is sealed
    pub fn is_sealed(&self) -> bool {
        self.sealed
    }

    /// Check if this block dominates another block (simplified check)
    /// Phase 69-3: Changed to BTreeSet for determinism
    pub fn dominates(&self, other: BasicBlockId, dominators: &[BTreeSet<BasicBlockId>]) -> bool {
        if let Some(dom_set) = dominators.get(other.to_usize()) {
            dom_set.contains(&self.id)
        } else {
            false
        }
    }
}

impl fmt::Display for BasicBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}:", self.id)?;

        // Show predecessors
        if !self.predecessors.is_empty() {
            let preds: Vec<String> = self.predecessors.iter().map(|p| format!("{}", p)).collect();
            writeln!(f, "  ; preds: {}", preds.join(", "))?;
        }

        // Show instructions
        for instruction in &self.instructions {
            writeln!(f, "  {}", instruction)?;
        }

        // Show terminator
        if let Some(ref terminator) = self.terminator {
            writeln!(f, "  {}", terminator)?;
        }

        // Show effects if not pure
        if !self.effects.is_pure() {
            writeln!(f, "  ; effects: {}", self.effects)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{BinaryOp, ConstValue};

    #[test]
    fn test_basic_block_creation() {
        let bb_id = BasicBlockId::new(0);
        let bb = BasicBlock::new(bb_id);

        assert_eq!(bb.id, bb_id);
        assert!(bb.is_empty());
        assert!(!bb.is_terminated());
        assert!(bb.effects.is_pure());
    }

    #[test]
    fn test_instruction_addition() {
        let bb_id = BasicBlockId::new(0);
        let mut bb = BasicBlock::new(bb_id);

        let const_inst = MirInstruction::Const {
            dst: ValueId::new(0),
            value: ConstValue::Integer(42),
        };

        bb.add_instruction(const_inst);

        assert_eq!(bb.instructions.len(), 1);
        assert!(!bb.is_empty());
        assert!(bb.effects.is_pure());
    }

    #[test]
    fn test_terminator_addition() {
        let bb_id = BasicBlockId::new(0);
        let mut bb = BasicBlock::new(bb_id);

        let return_inst = MirInstruction::Return {
            value: Some(ValueId::new(0)),
        };

        bb.add_instruction(return_inst);

        assert!(bb.is_terminated());
        assert!(bb.ends_with_return());
        assert_eq!(bb.instructions.len(), 0); // Terminator not in instructions
        assert!(bb.terminator.is_some());
    }

    #[test]
    fn test_branch_successors() {
        let bb_id = BasicBlockId::new(0);
        let mut bb = BasicBlock::new(bb_id);

        let then_bb = BasicBlockId::new(1);
        let else_bb = BasicBlockId::new(2);

        let branch_inst = MirInstruction::Branch {
            condition: ValueId::new(0),
            then_bb,
            else_bb,
            then_edge_args: None,
            else_edge_args: None,
        };

        bb.add_instruction(branch_inst);

        assert_eq!(bb.successors.len(), 2);
        assert!(bb.successors.contains(&then_bb));
        assert!(bb.successors.contains(&else_bb));
    }

    #[test]
    fn test_basic_block_id_generator() {
        let mut gen = BasicBlockIdGenerator::new();

        let bb1 = gen.next();
        let bb2 = gen.next();
        let bb3 = gen.next();

        assert_eq!(bb1, BasicBlockId(0));
        assert_eq!(bb2, BasicBlockId(1));
        assert_eq!(bb3, BasicBlockId(2));

        assert_eq!(gen.peek_next(), BasicBlockId(3));
    }

    #[test]
    fn test_value_tracking() {
        let bb_id = BasicBlockId::new(0);
        let mut bb = BasicBlock::new(bb_id);

        let val1 = ValueId::new(1);
        let val2 = ValueId::new(2);
        let val3 = ValueId::new(3);

        // Add instruction that defines val3 and uses val1, val2
        bb.add_instruction(MirInstruction::BinOp {
            dst: val3,
            op: BinaryOp::Add,
            lhs: val1,
            rhs: val2,
        });

        let defined = bb.defined_values();
        let used = bb.used_values();

        assert_eq!(defined, vec![val3]);
        assert_eq!(used, vec![val1, val2]);
    }

    #[test]
    fn test_phi_instruction_ordering() {
        let bb_id = BasicBlockId::new(0);
        let mut bb = BasicBlock::new(bb_id);

        // Add phi instruction
        let phi_inst = MirInstruction::Phi {
            dst: ValueId::new(0),
            inputs: vec![(BasicBlockId::new(1), ValueId::new(1))],
            type_hint: None, // Phase 63-6: Test code, no type hint
        };
        bb.add_instruction(phi_inst);

        // Add regular instruction
        let const_inst = MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::Integer(42),
        };
        bb.add_instruction(const_inst);

        // Phi instructions should come first
        let phi_count = bb.phi_instructions().count();
        assert_eq!(phi_count, 1);

        let non_phi_count = bb.non_phi_instructions().count();
        assert_eq!(non_phi_count, 1);
    }
}
