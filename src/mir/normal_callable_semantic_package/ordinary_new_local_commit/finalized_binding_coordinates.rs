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

impl OrdinaryNewClaimLedgerV1 {
    /// Project the retained LoopBody Invoke through the existing finished
    /// binding authority, without inventing a Home local-call row.
    pub(in crate::mir) fn finished_loop_body_call_producer_v1<'module>(
        &self,
        site: &OwnedExprSiteV1,
        packet: &super::super::super::PreparedLoopStaticBodyDetachedPacketV1,
        module: &'module crate::mir::MirModule,
    ) -> Result<(&'module MirFunction, (BasicBlockId, usize)), String> {
        let owner = site.owner();
        if packet.source_site() != site || !packet.corroborates_retained_scalar() {
            return Err(freeze("finished-loop-body/source-site-drift"));
        }
        let (symbol, finished) =
            self.with_finished_child_projection(owner, |symbol, projection| {
                project_recorded(symbol, projection, packet.original_invoke_binding())
            })?;
        let function = module
            .functions
            .get(&symbol)
            .ok_or_else(|| freeze("finished-loop-body/function-missing"))?;
        let coordinate = find_finished_producer(&symbol, &finished, function)?;
        if function
            .blocks
            .values()
            .flat_map(|block| block.all_instructions())
            .filter(|instruction| *instruction == &finished.1)
            .count()
            != 1
        {
            return Err(freeze("finished-loop-body/producer-duplicate"));
        }
        Ok((function, coordinate))
    }
}
