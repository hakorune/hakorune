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
