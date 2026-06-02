/*!
 * Metadata-only helper effect summaries.
 *
 * This owner inventories receiver/foreign field traffic for helper methods.
 * It does not authorize inlining, direct calls, or publication lowering. Later
 * plans such as ReceiverSnapshotPublicationPlanV0 may consume this vocabulary
 * after their own verifier is added.
 */

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectSummary {
    pub method: String,
    pub receiver_value: Option<ValueId>,
    pub receiver_reads: usize,
    pub receiver_writes: usize,
    pub foreign_reads: usize,
    pub foreign_writes: usize,
    pub handle_publications: usize,
    pub nested_call_count: usize,
    pub allocation_count: usize,
    pub safepoint_count: usize,
    pub branch_count: usize,
    pub loop_like_count: usize,
    pub foreign_base_count: usize,
    pub candidate_kind: &'static str,
    pub summary: &'static str,
    pub failure_reason: Option<&'static str>,
}

pub fn refresh_function_effect_summaries(function: &mut MirFunction) {
    function.metadata.effect_summaries.clear();

    let receiver_value = inferred_receiver_value(function);
    let mut classifier = EffectSummaryClassifier::new(receiver_value);

    for (block_id, block) in function.blocks.iter() {
        classifier.observe_block(*block_id, block);
    }

    if !classifier.has_reportable_effect_surface() {
        return;
    }

    let failure_reason = classifier.first_failure_reason();
    function.metadata.effect_summaries.push(EffectSummary {
        method: function.signature.name.clone(),
        receiver_value,
        receiver_reads: classifier.receiver_reads,
        receiver_writes: classifier.receiver_writes,
        foreign_reads: classifier.foreign_reads,
        foreign_writes: classifier.foreign_writes,
        handle_publications: classifier.handle_publications,
        nested_call_count: classifier.nested_call_count,
        allocation_count: classifier.allocation_count,
        safepoint_count: classifier.safepoint_count,
        branch_count: classifier.branch_count,
        loop_like_count: classifier.loop_like_count,
        foreign_base_count: classifier.foreign_bases.len(),
        candidate_kind: classifier.candidate_kind(),
        summary: if failure_reason.is_none() {
            "ok"
        } else {
            "rejected"
        },
        failure_reason,
    });
}

fn inferred_receiver_value(function: &MirFunction) -> Option<ValueId> {
    if !function.signature.name.contains('.') {
        return None;
    }
    function.params.first().copied().or(Some(ValueId::new(0)))
}

struct EffectSummaryClassifier {
    receiver_value: Option<ValueId>,
    receiver_reads: usize,
    receiver_writes: usize,
    foreign_reads: usize,
    foreign_writes: usize,
    handle_publications: usize,
    nested_call_count: usize,
    allocation_count: usize,
    safepoint_count: usize,
    branch_count: usize,
    loop_like_count: usize,
    foreign_bases: BTreeSet<ValueId>,
    copy_aliases: BTreeMap<ValueId, ValueId>,
    field_read_values: BTreeMap<ValueId, FieldReadOrigin>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FieldReadOrigin {
    base: ValueId,
    is_foreign: bool,
}

impl EffectSummaryClassifier {
    fn new(receiver_value: Option<ValueId>) -> Self {
        Self {
            receiver_value,
            receiver_reads: 0,
            receiver_writes: 0,
            foreign_reads: 0,
            foreign_writes: 0,
            handle_publications: 0,
            nested_call_count: 0,
            allocation_count: 0,
            safepoint_count: 0,
            branch_count: 0,
            loop_like_count: 0,
            foreign_bases: BTreeSet::new(),
            copy_aliases: BTreeMap::new(),
            field_read_values: BTreeMap::new(),
        }
    }

    fn observe_block(&mut self, _block_id: BasicBlockId, block: &crate::mir::BasicBlock) {
        for instruction in block.instructions.iter() {
            self.observe_instruction(instruction);
        }
        if let Some(terminator) = block.terminator.as_ref() {
            self.observe_terminator(terminator);
        }
    }

    fn observe_instruction(&mut self, instruction: &MirInstruction) {
        match instruction {
            MirInstruction::Copy { dst, src } => {
                self.copy_aliases.insert(*dst, self.resolve_value(*src));
            }
            MirInstruction::FieldGet { dst, base, .. } => {
                let base = self.resolve_value(*base);
                let is_foreign = self.is_foreign_base(base);
                if is_foreign {
                    self.foreign_reads += 1;
                    self.foreign_bases.insert(base);
                } else {
                    self.receiver_reads += 1;
                }
                self.field_read_values
                    .insert(*dst, FieldReadOrigin { base, is_foreign });
            }
            MirInstruction::FieldSet { base, value, .. } => {
                let base = self.resolve_value(*base);
                let value = self.resolve_value(*value);
                if self.is_foreign_base(base) {
                    self.foreign_writes += 1;
                    self.foreign_bases.insert(base);
                    return;
                }
                self.receiver_writes += 1;
                if self.is_foreign_handle_publication_value(value) {
                    self.handle_publications += 1;
                }
            }
            MirInstruction::Call { .. } => {
                self.nested_call_count += 1;
            }
            MirInstruction::NewBox { .. }
            | MirInstruction::NewClosure { .. }
            | MirInstruction::FutureNew { .. } => {
                self.allocation_count += 1;
            }
            MirInstruction::Safepoint | MirInstruction::Await { .. } => {
                self.safepoint_count += 1;
            }
            _ => {}
        }
    }

    fn observe_terminator(&mut self, instruction: &MirInstruction) {
        match instruction {
            MirInstruction::Branch { .. } => {
                self.branch_count += 1;
            }
            MirInstruction::Jump { .. } => {
                self.loop_like_count += 1;
            }
            MirInstruction::Call { .. } => {
                self.nested_call_count += 1;
            }
            MirInstruction::Safepoint | MirInstruction::Await { .. } => {
                self.safepoint_count += 1;
            }
            _ => {}
        }
    }

    fn is_foreign_base(&self, value: ValueId) -> bool {
        self.receiver_value
            .is_some_and(|receiver| receiver != value)
    }

    fn resolve_value(&self, value: ValueId) -> ValueId {
        let mut current = value;
        for _ in 0..8 {
            let Some(next) = self.copy_aliases.get(&current).copied() else {
                return current;
            };
            if next == current {
                return current;
            }
            current = next;
        }
        current
    }

    fn is_foreign_handle_publication_value(&self, value: ValueId) -> bool {
        self.foreign_bases.contains(&value)
    }

    fn has_reportable_effect_surface(&self) -> bool {
        self.receiver_reads > 0
            || self.receiver_writes > 0
            || self.foreign_reads > 0
            || self.foreign_writes > 0
            || self.handle_publications > 0
            || self.nested_call_count > 0
            || self.allocation_count > 0
            || self.safepoint_count > 0
    }

    fn candidate_kind(&self) -> &'static str {
        if self.foreign_writes > 0 || self.foreign_bases.len() > 1 || self.nested_call_count > 0 {
            return "rejected_effect_shape";
        }
        if self.foreign_reads > 0 && self.handle_publications > 0 && self.receiver_writes > 0 {
            return "mixed_base_publication_candidate";
        }
        if self.foreign_reads > 0 && self.receiver_writes > 0 {
            return "mixed_base_scalar_snapshot_candidate";
        }
        if self.receiver_reads > 0 || self.receiver_writes > 0 {
            return "receiver_local_leaf_candidate";
        }
        "generic_effect_summary"
    }

    fn first_failure_reason(&self) -> Option<&'static str> {
        if self.foreign_bases.len() > 1 {
            return Some("multiple_foreign_bases");
        }
        if self.foreign_writes > 0 {
            return Some("foreign_write_present");
        }
        if self.nested_call_count > 0 {
            return Some("nested_call_present");
        }
        if self.allocation_count > 0 {
            return Some("allocation_present");
        }
        if self.safepoint_count > 0 {
            return Some("safepoint_present");
        }
        if self.branch_count > 0 {
            return Some("branch_present");
        }
        if self.loop_like_count > 0 {
            return Some("loop_or_jump_present");
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{
        BasicBlockId, BinaryOp, ConstValue, EffectMask, FunctionSignature, MirInstruction, MirType,
    };

    fn publish_selection_function() -> MirFunction {
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "ProofQueue.publishSelection/3".to_string(),
                params: vec![
                    MirType::Box("ProofQueue".to_string()),
                    MirType::Integer,
                    MirType::Box("ProofPage".to_string()),
                    MirType::Integer,
                ],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        function.params = vec![
            ValueId::new(0),
            ValueId::new(1),
            ValueId::new(2),
            ValueId::new(3),
        ];
        let entry = function.get_block_mut(BasicBlockId::new(0)).unwrap();
        entry.add_instruction(MirInstruction::FieldSet {
            base: ValueId::new(0),
            field: "last_selected_index".to_string(),
            value: ValueId::new(1),
            declared_type: Some(MirType::Integer),
        });
        entry.add_instruction(MirInstruction::FieldGet {
            dst: ValueId::new(4),
            base: ValueId::new(2),
            field: "page_id".to_string(),
            declared_type: Some(MirType::Integer),
        });
        entry.add_instruction(MirInstruction::FieldSet {
            base: ValueId::new(0),
            field: "last_selected_page_id".to_string(),
            value: ValueId::new(4),
            declared_type: Some(MirType::Integer),
        });
        entry.add_instruction(MirInstruction::FieldSet {
            base: ValueId::new(0),
            field: "last_selected_kind".to_string(),
            value: ValueId::new(3),
            declared_type: Some(MirType::Integer),
        });
        entry.add_instruction(MirInstruction::FieldSet {
            base: ValueId::new(0),
            field: "last_selected_page".to_string(),
            value: ValueId::new(2),
            declared_type: Some(MirType::Box("ProofPage".to_string())),
        });
        entry.add_instruction(MirInstruction::FieldGet {
            dst: ValueId::new(5),
            base: ValueId::new(0),
            field: "select_count".to_string(),
            declared_type: Some(MirType::Integer),
        });
        entry.add_instruction(MirInstruction::Const {
            dst: ValueId::new(6),
            value: ConstValue::Integer(1),
        });
        entry.add_instruction(MirInstruction::BinOp {
            dst: ValueId::new(7),
            op: BinaryOp::Add,
            lhs: ValueId::new(5),
            rhs: ValueId::new(6),
        });
        entry.add_instruction(MirInstruction::FieldSet {
            base: ValueId::new(0),
            field: "select_count".to_string(),
            value: ValueId::new(7),
            declared_type: Some(MirType::Integer),
        });
        entry.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(6)),
        });
        function
    }

    #[test]
    fn summarizes_mixed_base_publication_candidate_without_authorizing_inline() {
        let mut function = publish_selection_function();

        refresh_function_effect_summaries(&mut function);

        let summary = &function.metadata.effect_summaries[0];
        assert_eq!(summary.method, "ProofQueue.publishSelection/3");
        assert_eq!(summary.receiver_value, Some(ValueId::new(0)));
        assert_eq!(summary.receiver_reads, 1);
        assert_eq!(summary.receiver_writes, 5);
        assert_eq!(summary.foreign_reads, 1);
        assert_eq!(summary.foreign_writes, 0);
        assert_eq!(summary.handle_publications, 1);
        assert_eq!(summary.foreign_base_count, 1);
        assert_eq!(summary.candidate_kind, "mixed_base_publication_candidate");
        assert_eq!(summary.summary, "ok");
    }
}
