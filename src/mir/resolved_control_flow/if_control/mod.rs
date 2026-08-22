//! Disconnected, effect-free statement-`If` control products.
//!
//! This facade preserves the logical `resolved_control_flow::if_control`
//! module while keeping the source analyzer, co-sealed product, and one-shot
//! use ledgers in responsibility-specific children. No child introduces a
//! second If-control authority.

#![allow(unused_imports)]

mod analyzer;
mod product;
mod use_ledger;

pub(super) use product::VerifiedLocatedIfControlV1;
pub(crate) use product::{
    ResolvedIfControlMaterializationV1, ResolvedIfElsePortV1, ResolvedIfFallthroughPortV1,
    VerifiedResolvedFunctionIfControlV1,
};

pub(crate) use use_ledger::{
    FunctionIfControlUseErrorV1, FunctionIfControlUseLedgerV1, IfControlCoverageUseErrorV1,
    IfControlCoverageUseV1,
};

pub(super) use analyzer::{analyze_resolved_if_control_v1, ResolvedIfControlErrorV1};
pub(crate) use analyzer::{
    verify_resolved_function_if_control_v1,
    verify_resolved_function_if_control_with_direct_call_v1,
    ResolvedFunctionIfControlContractErrorV1,
};
