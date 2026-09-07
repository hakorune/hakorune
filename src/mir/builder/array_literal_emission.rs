//! Physical outputs of the intrinsic literal emitter, never source authority.
use crate::mir::{ArrayWriteSiteId, BasicBlockId};
use crate::mir::{MirBuilder, MirInstruction, ValueId};

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct ArrayLiteralEmission {
    pub recipe: super::ArrayLocalRecipeV1,
    pub entry: BasicBlockId,
    pub allocation: ValueId,
    pub allocation_site: (BasicBlockId, usize),
    pub claim: String,
    pub claim_site: (BasicBlockId, usize),
    pub elements: Vec<ArrayElementEmission>,
}

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct ArrayElementEmission {
    pub value: ValueId,
    pub definition: MirInstruction,
    pub definition_site: (BasicBlockId, usize),
    pub write: ArrayWriteSiteId,
    pub write_site: (BasicBlockId, usize),
}

pub(in crate::mir::builder) fn last_instruction(
    builder: &MirBuilder,
) -> Result<((BasicBlockId, usize), &MirInstruction), String> {
    let function = builder
        .function_state
        .current_function
        .as_ref()
        .ok_or_else(|| fault("function-missing"))?;
    let block_id = builder
        .function_state
        .current_block
        .ok_or_else(|| fault("block-missing"))?;
    let block = function
        .blocks
        .get(&block_id)
        .ok_or_else(|| fault("block-missing"))?;
    let index = block
        .instructions
        .len()
        .checked_sub(1)
        .ok_or_else(|| fault("instruction-missing"))?;
    Ok(((block_id, index), &block.instructions[index]))
}

impl ArrayLiteralEmission {
    pub(in crate::mir::builder) fn begin(
        builder: &MirBuilder,
        allocation: ValueId,
        allocation_site: (BasicBlockId, usize),
        claim: String,
        recipe: super::ArrayLocalRecipeV1,
    ) -> Result<Self, String> {
        let (claim_site, instruction) = last_instruction(builder)?;
        if !matches!(instruction, MirInstruction::ArrayStateContractClaim { contract_id, array }
            if *array == allocation && *contract_id == claim)
        {
            return Err(fault("claim-emission-drift"));
        }
        Ok(Self {
            recipe,
            entry: builder
                .function_state
                .current_function
                .as_ref()
                .unwrap()
                .entry_block,
            allocation,
            allocation_site,
            claim,
            claim_site,
            elements: Vec::new(),
        })
    }

    pub(in crate::mir::builder) fn record_element(
        &mut self,
        builder: &MirBuilder,
        value: ValueId,
        definition_site: (BasicBlockId, usize),
        definition: MirInstruction,
        write: ArrayWriteSiteId,
    ) -> Result<(), String> {
        let (write_site, instruction) = last_instruction(builder)?;
        if !matches!(instruction, MirInstruction::ArrayElementWrite { site_id, receiver, value: actual, .. }
            if *site_id == write && *receiver == self.allocation && *actual == value)
        {
            return Err(fault("write-emission-drift"));
        }
        self.elements.push(ArrayElementEmission {
            value,
            definition,
            definition_site,
            write,
            write_site,
        });
        Ok(())
    }
}

fn fault(reason: &str) -> String {
    format!("[freeze:contract][array-emission/{reason}]")
}
