//! Normal source policy and compatibility origin.

mod compatibility_origin;
mod parser_bound_policy;
mod rejection;

pub(crate) use compatibility_origin::{
    NormalCallableCompatibilityOriginErrorV1, NormalCallableCompatibilityOriginV1,
};
pub(in crate::mir) use parser_bound_policy::require_static_main_v1;
pub(crate) use rejection::NormalSourcePlanErrorV1;
