//! Exact physical coordinates after source membership has been established.
use super::*;

/// Shared independent Copy shape/unique-definition check after source membership.
pub(in super::super) fn find_finished_copy(
    symbol: &str,
    finished: &Binding,
    function: &MirFunction,
) -> Result<(BasicBlockId, usize), String> {
    if !matches!(finished.1, MirInstruction::Copy { .. }) {
        return Err(freeze("finished-copy/instruction"));
    }
    let coordinate = find_finished_producer(symbol, finished, function)?;
    let dst = finished.1.dst_value().expect("checked Copy");
    if function.params.contains(&dst) {
        return Err(freeze("finished-copy/parameter-collision"));
    }
    if function
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .filter(|instruction| instruction.dst_value() == Some(dst))
        .count()
        != 1
    {
        return Err(freeze("finished-copy/destination-duplicate"));
    }
    Ok(coordinate)
}

pub(in super::super) fn find_finished_producer(
    symbol: &str,
    finished: &Binding,
    function: &MirFunction,
) -> Result<(BasicBlockId, usize), String> {
    if function.signature.name != symbol {
        return Err(freeze("finished-local-call/function"));
    }
    let block = function
        .blocks
        .get(&finished.0)
        .ok_or_else(|| freeze("finished-local-call/block-missing"))?;
    let index = crate::mir::verification::utils::unique_instruction_position(
        block.all_instructions(),
        &finished.1,
        || freeze("finished-local-call/producer-missing"),
        || freeze("finished-local-call/producer-duplicate"),
    )?;
    Ok((finished.0, index))
}
