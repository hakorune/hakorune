//! Shared source expression navigation and value classification.

pub(crate) mod expression_children;
pub(crate) mod expression_class;
pub(crate) mod expression_plan;
pub(crate) mod expression_position;
pub(crate) mod expression_projection;
pub(crate) mod expression_root;
pub(crate) mod expression_source;

#[cfg(test)]
mod expression_argument_source_tests;
