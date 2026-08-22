//! Canonical Script A capability and immediate C transport.

mod consumer;
mod issuer;
mod model;
mod required_argument_source;

pub(in crate::mir::builder) use consumer::{
    CanonicalScriptCBoundSourceV1, CanonicalScriptCPreparedLoweringSourceV1,
};
pub(in crate::mir::builder) use issuer::issue_into_c_transport;

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
