pub struct MetadataContext<SpanT, RegionIdT> {
    pub current_span: SpanT,
    pub current_region: RegionIdT,
}

impl<SpanT: Copy, RegionIdT: Copy> MetadataContext<SpanT, RegionIdT> {
    pub fn current_span(&self) -> SpanT {
        self.current_span
    }
}
