//! Compatibility facade for Box factory policy vocabulary.
//!
//! The passive owner is `hakorune-box-core`. Keep this module so historical
//! `crate::box_factory::{FactoryPolicy, FactoryType}` imports remain stable.

pub use hakorune_box_core::{FactoryPolicy, FactoryType};
