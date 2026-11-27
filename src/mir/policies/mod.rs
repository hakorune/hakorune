//! MIR policy SSOT (shared by router/canonicalizer).
//!
//! Goal: avoid duplicated policy logic across layers.

pub(crate) mod a_prime_i64_callable_storage_layout;
pub(crate) mod call_name_classification;
pub(crate) mod callee_box_kind;
pub mod return_prelude_policy;
pub(crate) mod source_method_reserved_route;
pub(crate) mod source_method_typeop_route;

#[cfg(test)]
mod source_method_reserved_route_tests;
#[cfg(test)]
mod source_method_typeop_route_tests;
