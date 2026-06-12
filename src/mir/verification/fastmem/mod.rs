use crate::mir::function::MirFunction;
use crate::mir::instruction::{FastMemRegionId, MemOpKind};
use crate::mir::verification_types::VerificationError;
use crate::mir::{BasicBlockId, ValueId};

mod escape;
mod region;

#[cfg(test)]
mod tests;

pub(super) fn check_fastmem_regions(function: &MirFunction) -> Result<(), Vec<VerificationError>> {
    let mut errors = Vec::new();
    let regions = region::collect_regions(function, &mut errors);
    let memop_sites = region::collect_memop_sites(function, &regions, &mut errors);

    region::check_region_counts(function, &regions, &memop_sites, &mut errors);
    escape::check_memop_escape(function, &memop_sites, &mut errors);

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) struct MemOpSite {
    pub region: FastMemRegionId,
    pub kind: MemOpKind,
    pub dst: Option<ValueId>,
}

pub(super) fn push_region_error(
    function: &MirFunction,
    block: Option<BasicBlockId>,
    instruction_index: Option<usize>,
    region: Option<u32>,
    contract: Option<String>,
    reason: impl Into<String>,
    errors: &mut Vec<VerificationError>,
) {
    errors.push(VerificationError::FastMemContractViolation {
        function: function.signature.name.clone(),
        block,
        instruction_index,
        region,
        contract,
        reason: format!("[freeze:contract][fastmem/{}]", reason.into()),
    });
}
