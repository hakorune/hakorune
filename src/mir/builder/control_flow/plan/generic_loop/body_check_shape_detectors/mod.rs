//! Shape detection functions for generic loop analysis
//!
//! This module provides pattern matching functions to detect specific loop body shapes.
//! Each function matches a particular pattern in the loop body AST.

mod accum;
mod basic;
mod complex_parsers;
mod inner_loop;
mod nested_loop_program2;
mod utils;

pub(super) use accum::*;
pub(super) use basic::*;
pub(super) use complex_parsers::*;
pub(super) use nested_loop_program2::*;
pub(super) use utils::*;
