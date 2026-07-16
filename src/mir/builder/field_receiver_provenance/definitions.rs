use super::SameRootReceiverProofErrorV1;
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};
use std::collections::BTreeMap;

pub(super) enum DefinitionKindV1 {
    Parameter(u32),
    Instruction {
        instruction_index: usize,
        is_terminator: bool,
    },
}

pub(super) struct DefinitionSiteV1 {
    pub(super) block: BasicBlockId,
    pub(super) order: Option<usize>,
    pub(super) kind: DefinitionKindV1,
}

impl DefinitionSiteV1 {
    pub(super) fn instruction<'a>(
        &self,
        function: &'a MirFunction,
    ) -> Result<&'a MirInstruction, SameRootReceiverProofErrorV1> {
        let block = function
            .blocks
            .get(&self.block)
            .ok_or(SameRootReceiverProofErrorV1::MissingCfgBlock)?;
        let DefinitionKindV1::Instruction {
            instruction_index,
            is_terminator,
        } = self.kind
        else {
            return Err(SameRootReceiverProofErrorV1::MissingDefinition);
        };
        if is_terminator {
            block
                .terminator
                .as_ref()
                .ok_or(SameRootReceiverProofErrorV1::MissingDefinition)
        } else {
            block
                .instructions
                .get(instruction_index)
                .ok_or(SameRootReceiverProofErrorV1::MissingDefinition)
        }
    }
}

pub(super) struct ExactDefinitionIndexV1 {
    by_value: BTreeMap<ValueId, DefinitionSiteV1>,
    phi_input_count: usize,
}

impl ExactDefinitionIndexV1 {
    pub(super) fn build(function: &MirFunction) -> Result<Self, SameRootReceiverProofErrorV1> {
        let mut by_value = BTreeMap::new();
        for (index, value) in function.params.iter().copied().enumerate() {
            insert_exact(
                &mut by_value,
                value,
                DefinitionSiteV1 {
                    block: function.entry_block,
                    order: None,
                    kind: DefinitionKindV1::Parameter(index as u32),
                },
            )?;
        }

        let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
        block_ids.sort();
        let mut phi_input_count = 0usize;
        for block_id in block_ids {
            let block = function
                .blocks
                .get(&block_id)
                .ok_or(SameRootReceiverProofErrorV1::MissingDefinition)?;
            for (order, instruction) in block.instructions.iter().enumerate() {
                if let MirInstruction::Phi { inputs, .. } = instruction {
                    phi_input_count = phi_input_count.saturating_add(inputs.len());
                }
                if let Some(dst) = instruction.dst_value() {
                    insert_exact(
                        &mut by_value,
                        dst,
                        DefinitionSiteV1 {
                            block: block_id,
                            order: Some(order),
                            kind: DefinitionKindV1::Instruction {
                                instruction_index: order,
                                is_terminator: false,
                            },
                        },
                    )?;
                }
            }
            if let Some(instruction) = block.terminator.as_ref() {
                if let Some(dst) = instruction.dst_value() {
                    insert_exact(
                        &mut by_value,
                        dst,
                        DefinitionSiteV1 {
                            block: block_id,
                            order: Some(block.instructions.len()),
                            kind: DefinitionKindV1::Instruction {
                                instruction_index: block.instructions.len(),
                                is_terminator: true,
                            },
                        },
                    )?;
                }
            }
        }

        Ok(Self {
            by_value,
            phi_input_count,
        })
    }

    pub(super) fn get(
        &self,
        value: ValueId,
    ) -> Result<&DefinitionSiteV1, SameRootReceiverProofErrorV1> {
        self.by_value
            .get(&value)
            .ok_or(SameRootReceiverProofErrorV1::MissingDefinition)
    }

    pub(super) fn traversal_budget(&self) -> usize {
        self.by_value
            .len()
            .saturating_mul(4)
            .saturating_add(self.phi_input_count.saturating_mul(2))
            .saturating_add(1)
    }
}

fn insert_exact(
    index: &mut BTreeMap<ValueId, DefinitionSiteV1>,
    value: ValueId,
    site: DefinitionSiteV1,
) -> Result<(), SameRootReceiverProofErrorV1> {
    if index.insert(value, site).is_some() {
        return Err(SameRootReceiverProofErrorV1::MultipleDefinition);
    }
    Ok(())
}
