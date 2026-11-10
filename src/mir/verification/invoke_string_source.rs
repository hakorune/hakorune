//! Ephemeral source/finished correspondence for Text carrier lifetimes.
//! No source authority is recovered from MIR types or descriptor tags.
use super::*;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Default)]
pub(super) struct TextSourceImage {
    pub(super) slots: BTreeMap<ValueId, ValueId>,
    pub(super) edges: BTreeMap<(BasicBlockId, BasicBlockId), Vec<(ValueId, ValueId)>>,
    copies: BTreeMap<ValueId, ValueId>,
    literals: BTreeSet<ValueId>,
    aliases: Vec<(BasicBlockId, MirInstruction)>,
    operations: Vec<(BasicBlockId, MirInstruction)>,
    normal_retained: BTreeMap<BasicBlockId, (MirInstruction, Vec<ValueId>)>,
}
#[derive(Clone, Copy)]
pub(super) enum Origin {
    Static,
    Slot(ValueId),
    Raw(ValueId),
}
impl TextSourceImage {
    pub(super) fn is_alias(&self, value: ValueId) -> bool {
        self.slots.contains_key(&value)
            || self.copies.contains_key(&value)
            || self.literals.contains(&value)
    }
    pub(super) fn origin(&self, mut value: ValueId) -> Option<Origin> {
        let mut seen = BTreeSet::new();
        loop {
            if !seen.insert(value) {
                return None;
            }
            if let Some(slot) = self.slots.get(&value) {
                return Some(Origin::Slot(*slot));
            }
            if self.literals.contains(&value) {
                return Some(Origin::Static);
            }
            match self.copies.get(&value) {
                Some(source) => value = *source,
                None => return Some(Origin::Raw(value)),
            }
        }
    }
    pub(super) fn permits_alias(&self, block: BasicBlockId, row: &MirInstruction) -> bool {
        self.aliases
            .iter()
            .any(|(owner, original)| *owner == block && original == row)
    }
    pub(super) fn permits_operation(&self, block: BasicBlockId, row: &MirInstruction) -> bool {
        self.operations
            .iter()
            .any(|(owner, original)| *owner == block && original == row)
    }
    pub(super) fn retained_for(&self, block: BasicBlockId, row: &MirInstruction) -> &[ValueId] {
        self.normal_retained
            .get(&block)
            .filter(|(original, _)| original == row)
            .map_or(&[], |(_, values)| values.as_slice())
    }
    pub(super) fn has_slot(&self, slot: ValueId) -> bool {
        self.slots.values().any(|value| *value == slot)
    }
}

pub(super) fn images_for_module(
    module: &crate::mir::MirModule,
    root: Option<&crate::mir::finalized_root_handoff::FinalizedRootHandoffV1>,
) -> Result<BTreeMap<String, TextSourceImage>, String> {
    let mut images = BTreeMap::<String, TextSourceImage>::new();
    let Some(source) = root.and_then(|root| root.root_source()) else {
        return Ok(images);
    };
    source.visit_finished_text_loops_v2(module, |function, view| {
        let image = images.entry(function.signature.name.clone()).or_default();
        for (phi, predecessor, incoming) in &view.edges {
            let MirInstruction::Phi { dst, .. } = &phi.1 else {
                return Err("string-owned-escape".into());
            };
            if image
                .slots
                .insert(*dst, view.slot)
                .is_some_and(|old| old != view.slot)
            {
                return Err("string-owned-escape".into());
            }
            let edge = image.edges.entry((*predecessor, phi.0)).or_default();
            if edge.iter().any(|(slot, _)| *slot == view.slot) {
                return Err("string-owned-escape".into());
            }
            edge.push((view.slot, *incoming));
            if !image.aliases.contains(phi) {
                image.aliases.push(phi.clone());
            }
        }
        for binding in &view.dependencies {
            match &binding.1 {
                MirInstruction::Copy { dst, src } => {
                    if image
                        .copies
                        .insert(*dst, *src)
                        .is_some_and(|old| old != *src)
                    {
                        return Err("string-owned-escape".into());
                    }
                    if !image.aliases.contains(binding) {
                        image.aliases.push(binding.clone());
                    }
                }
                MirInstruction::Const {
                    dst,
                    value: crate::mir::ConstValue::String(_),
                } => {
                    image.literals.insert(*dst);
                }
                _ => {}
            }
        }
        for (binding, value) in &view.normal_ends {
            let (original, values) = image
                .normal_retained
                .entry(binding.0)
                .or_insert_with(|| (binding.1.clone(), Vec::new()));
            if original != &binding.1 {
                return Err("string-owned-escape".into());
            }
            if !values.contains(value) {
                values.push(*value);
            }
        }
        for binding in &view.operations {
            if !image.operations.contains(binding) {
                image.operations.push(binding.clone());
            }
        }
        Ok(())
    })?;
    Ok(images)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::instruction::FaultFrameMode;
    use crate::mir::{BasicBlock, ConstValue, EffectMask, FunctionSignature, MirType};

    #[test]
    fn text_read_occurrences_follow_the_supplied_carrier_image_and_actual_end() {
        // Isolate the independent consumer with a supplied image. This does
        // not claim that source lowering admits new loop-carried Text shapes.
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "carrier_read".into(),
                params: vec![],
                return_type: MirType::Integer,
                effects: EffectMask::CONTROL,
            },
            BasicBlockId(0),
        );
        let mut entry = BasicBlock::new(BasicBlockId(0));
        entry.add_instruction(MirInstruction::FaultFrameEnter {
            dst: ValueId(0),
            mode: FaultFrameMode::RootOwned,
        });
        entry.set_terminator(MirInstruction::Invoke {
            operation: InvokeOperation::String(Op::Materialize {
                value: Operand::StaticText { utf8: "abc".into() },
            }),
            fault_frame: ValueId(0),
            normal_landing: BasicBlockId(1),
            fault_landing: BasicBlockId(4),
        });
        function.add_block(entry);
        let mut acquired = BasicBlock::new(BasicBlockId(1));
        acquired.add_instruction(MirInstruction::InvokeNormalResult {
            dst: ValueId(2),
            invoke_block: BasicBlockId(0),
        });
        acquired.add_instruction(MirInstruction::StringResidenceInstall {
            value: ValueId(2),
            replaces: None,
            returning: false,
        });
        acquired.set_terminator(MirInstruction::Jump {
            target: BasicBlockId(2),
            edge_args: None,
        });
        function.add_block(acquired);
        let phi = MirInstruction::Phi {
            dst: ValueId(3),
            inputs: vec![(BasicBlockId(1), ValueId(2))],
            type_hint: None,
        };
        let copy = MirInstruction::Copy {
            dst: ValueId(5),
            src: ValueId(3),
        };
        let read = MirInstruction::StringIndexOf {
            dst: ValueId(6),
            value: Operand::TextOperand(ValueId(5)),
            needle: Operand::TextOperand(ValueId(5)),
        };
        let end = MirInstruction::Invoke {
            operation: InvokeOperation::String(Op::EndIfOwned { value: ValueId(3) }),
            fault_frame: ValueId(0),
            normal_landing: BasicBlockId(3),
            fault_landing: BasicBlockId(4),
        };
        let mut body = BasicBlock::new(BasicBlockId(2));
        for row in [&phi, &copy, &read] {
            body.add_instruction(row.clone());
        }
        body.set_terminator(end.clone());
        function.add_block(body);
        let mut normal = BasicBlock::new(BasicBlockId(3));
        normal.set_terminator(MirInstruction::Return {
            value: Some(ValueId(6)),
        });
        function.add_block(normal);
        let mut fault = BasicBlock::new(BasicBlockId(4));
        fault.set_terminator(MirInstruction::ReturnFault {
            fault_frame: ValueId(0),
        });
        function.add_block(fault);
        let mut image = TextSourceImage::default();
        image.slots.insert(ValueId(3), ValueId(3));
        image.copies.insert(ValueId(5), ValueId(3));
        image.edges.insert(
            (BasicBlockId(1), BasicBlockId(2)),
            vec![(ValueId(3), ValueId(2))],
        );
        image.aliases = vec![(BasicBlockId(2), phi), (BasicBlockId(2), copy)];
        image.operations = vec![(BasicBlockId(2), read.clone()), (BasicBlockId(2), end)];
        assert_eq!(
            read.used_values()
                .iter()
                .filter(|v| **v == ValueId(5))
                .count(),
            2
        );
        assert!(super::super::lifetime::check(&function, Some(&image)).is_ok());
        for side in [0, 1] {
            let mut ended = function.clone();
            let mut image = TextSourceImage {
                slots: image.slots.clone(),
                edges: image.edges.clone(),
                copies: image.copies.clone(),
                literals: image.literals.clone(),
                aliases: image.aliases.clone(),
                operations: image.operations.clone(),
                normal_retained: image.normal_retained.clone(),
            };
            let mut read = read.clone();
            if let MirInstruction::StringIndexOf { value, needle, .. } = &mut read {
                if side == 0 {
                    *needle = Operand::StaticText { utf8: "a".into() };
                } else {
                    *value = Operand::StaticText { utf8: "a".into() };
                }
            }
            ended
                .blocks
                .get_mut(&BasicBlockId(2))
                .unwrap()
                .instructions
                .pop();
            ended
                .blocks
                .get_mut(&BasicBlockId(3))
                .unwrap()
                .instructions
                .push(read.clone());
            image
                .operations
                .retain(|(_, row)| row.text_read_operands_v1().is_none());
            image.operations.push((BasicBlockId(3), read));
            assert!(matches!(
                super::super::lifetime::check(&ended, Some(&image)),
                Err((_, "string-borrow-not-live", _))
            ));
        }
    }
}
