//! Shape detection functions for generic loop analysis
//!
//! This module provides pattern matching functions to detect specific loop body shapes.
//! Each function matches a particular pattern in the loop body AST.

mod utils;
mod basic;
mod nested_loop_program2;
mod inner_loop;
mod accum;
mod complex_parsers;

pub(super) use utils::*;
pub(super) use basic::*;
pub(super) use nested_loop_program2::*;
pub(super) use accum::*;
pub(super) use complex_parsers::*;
