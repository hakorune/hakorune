//! Compatibility facade for the legacy Builder build methods.
//!
//! The lowering implementations live in responsibility-specific sibling
//! modules.  This module intentionally keeps the historical `builder_build`
//! path stable for existing crate-local consumers and test fixtures.

pub(in crate::mir::builder) use super::new_expression::PreparedRawNewExpressionV1;
