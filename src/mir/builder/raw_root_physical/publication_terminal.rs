//! PUBLICATION0 opaque Raw module transition.

#[derive(Debug)]
pub(in crate::mir) struct RawPublishedModuleV1 {
    _seal: RawPublishedModuleSealV1,
}

#[derive(Debug)]
struct RawPublishedModuleSealV1;

impl RawPublishedModuleV1 {
    pub(in crate::mir::builder) fn installed() -> Self {
        Self {
            _seal: RawPublishedModuleSealV1,
        }
    }
}
