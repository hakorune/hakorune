//! Compatibility re-export while trim policy lives under `cleanup::policies`.

#[allow(unused_imports)]
pub use crate::mir::builder::control_flow::cleanup::policies::trim_policy::{
    classify_trim_like_loop, TrimPolicyResult,
};
