//! JoinIR Instruction Handlers
//!
//! Phase 260 P0.2: Modularized handlers extracted from joinir_block_converter.rs
//!
//! Each handler corresponds to a specific JoinIR instruction type and is
//! responsible for converting it to MIR instructions.

pub(super) mod call;
pub(super) mod conditional_method_call;
pub(super) mod field_access;
pub(super) mod if_merge;
pub(super) mod jump;
pub(super) mod method_call;
pub(super) mod nested_if_merge;
pub(super) mod new_box;
pub(super) mod ret;
pub(super) mod select;
