//! Tests for loop canonicalizer
//!
//! Separated from canonicalizer.rs for better maintainability.

use super::canonicalizer::canonicalize_loop_expr;
use super::skeleton_types::{CarrierRole, SkeletonStep, UpdateKind};
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::loop_pattern_detection::LoopPatternKind;

mod basic;
mod skip_whitespace;
mod trim_leading;
mod continue_pattern;
mod trim_trailing;
mod parse_string;
mod parse_array;
mod parse_object;
mod parse_number;
mod escape_skip;
