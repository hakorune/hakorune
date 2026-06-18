//! Passive frontend environment boundary marker.
//!
//! Active environment reads remain in the main crate until parser/tokenizer
//! files move behind this crate root.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrontendEnvBoundary;

impl FrontendEnvBoundary {
    pub const fn name(self) -> &'static str {
        "frontend_env"
    }
}
