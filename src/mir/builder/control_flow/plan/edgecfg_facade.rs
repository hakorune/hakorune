//! EdgeCFG facade for plan layer (SSOT boundary).
//!
//! Policy:
//! - Plan must access edgecfg only through this facade.
//! - Keep the surface minimal and explicit.

pub(in crate::mir::builder) use crate::mir::builder::control_flow::edgecfg::api::{
    BlockParams, BranchStub, EdgeStub, ExitKind, Frag, FragEmitSession,
};
