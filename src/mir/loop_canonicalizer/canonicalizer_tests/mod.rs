//! Tests for loop canonicalizer
//!
//! Separated from canonicalizer.rs for better maintainability.

use super::canonicalizer::canonicalize_loop_expr;
use super::skeleton_types::{CarrierRole, SkeletonStep, UpdateKind};
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::loop_route_detection::LoopRouteKind;

mod basic;
mod continue_pattern;
mod escape_skip;
mod parse_array;
mod parse_number;
mod parse_object;
mod parse_string;
mod skip_whitespace;
mod trim_leading;
mod trim_trailing;
