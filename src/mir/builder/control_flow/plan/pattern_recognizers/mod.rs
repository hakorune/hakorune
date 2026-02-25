//! Pattern Recognizers Module
//!
//! Phase 287 P1: Modularization of AST pattern detection functions.
//!
//! This module contains specialized recognizers for different loop patterns:
//! - continue/break/return detection
//! - infinite loop detection
//! - if-else phi detection
//! - carrier count estimation
//! - parse_number/string/whitespace patterns
//!
//! Each recognizer is responsible for a single "question" about the AST structure.

pub mod continue_break;
pub mod infinite_loop;
pub mod if_else_phi;
pub mod carrier_count;
pub mod parse_number;
pub mod parse_string;
pub mod skip_whitespace;
