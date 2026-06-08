use crate::mir::instruction::FastMemRegionId;

impl super::MirBuilder {
    #[inline]
    pub(crate) fn current_fastmem_region(&self) -> Option<FastMemRegionId> {
        self.scope_ctx.current_fastmem_region()
    }

    #[inline]
    pub(crate) fn push_fastmem_region(&mut self, region: FastMemRegionId) {
        self.scope_ctx.push_fastmem_region(region);
    }

    #[inline]
    pub(crate) fn pop_fastmem_region(&mut self) -> Option<FastMemRegionId> {
        self.scope_ctx.pop_fastmem_region()
    }
}
