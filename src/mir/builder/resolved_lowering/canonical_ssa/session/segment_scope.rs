//! Session-local identity for the common V2 segment allocation scope.

use super::CanonicalSsaFunctionSessionV2;
use crate::mir::builder::resolved_lowering::common_v2_segment_block_allocation::{
    PreparedSegmentBlockReceiptV1, SegmentBlockAllocationBrandV1,
};

impl CanonicalSsaFunctionSessionV2<'_> {
    pub(in crate::mir::builder::resolved_lowering) fn segment_blocks_issued(&self) -> bool {
        self.segment_blocks_issued
    }

    pub(in crate::mir::builder::resolved_lowering) fn mark_segment_blocks_issued(&mut self) {
        self.segment_blocks_issued = true;
    }

    pub(in crate::mir::builder::resolved_lowering) fn segment_block_brand(
        &self,
    ) -> SegmentBlockAllocationBrandV1 {
        self.segment_block_brand.clone()
    }

    pub(in crate::mir::builder::resolved_lowering) fn owns_segment_receipt(
        &self,
        receipt: &PreparedSegmentBlockReceiptV1,
    ) -> bool {
        receipt.belongs_to(&self.segment_block_brand)
    }
}
