//! Compatibility surface for the facts-owned value expression checks.
//!
//! Owner moved to `facts/expr_value.rs`.

#[allow(unused_imports)]
pub(in crate::mir::builder) use crate::mir::builder::control_flow::facts::expr_value::{
    is_supported_value_expr, value_expr_requires_canon,
};
