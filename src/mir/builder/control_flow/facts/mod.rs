//! Top-level descriptive owner surface for control-flow facts.
//!
//! During folderization, implementations still live under `plan/`.
//! This module is the compatibility owner that non-`plan/` consumers should
//! depend on first.

pub(in crate::mir::builder) mod canon;
pub(crate) mod ast_feature_extractor;
pub(in crate::mir::builder) mod extractors;
pub(in crate::mir::builder) mod route_shape_recognizers;

#[allow(unused_imports)]
pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::facts::*;
