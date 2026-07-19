use crate::mir::instruction::FastMemRegionId;

impl super::MirBuilder {
    #[inline]
    pub(crate) fn current_fastmem_region(&self) -> Option<FastMemRegionId> {
        self.function_state.scope.current_fastmem_region()
    }

    #[inline]
    pub(crate) fn push_fastmem_region(&mut self, region: FastMemRegionId) {
        self.function_state.scope.push_fastmem_region(region);
    }

    #[inline]
    pub(crate) fn pop_fastmem_region(&mut self) -> Option<FastMemRegionId> {
        self.function_state.scope.pop_fastmem_region()
    }
}
