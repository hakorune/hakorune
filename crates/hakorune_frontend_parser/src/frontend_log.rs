//! Passive frontend logging boundary marker.
//!
//! Active logging remains host-provided by the main crate until file movement.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrontendLogBoundary;

impl FrontendLogBoundary {
    pub const fn name(self) -> &'static str {
        "frontend_log"
    }
}
