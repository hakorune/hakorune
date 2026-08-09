//! Neutral lifecycle vocabulary for opaque Dynamic carriers.
//!
//! This module owns no call/operator semantics, Home classification, runtime
//! tag, cleanup placement, or physical end mechanism.

mod model;

pub(crate) use model::DynamicCarrierLifecycleObligationV1;
