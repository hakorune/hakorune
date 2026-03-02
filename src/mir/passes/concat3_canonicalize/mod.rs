//! String concat chain canonicalization for MIR.
//!
//! Rewrites one-level string `BinOp::Add` chains:
//! - `(a + b) + c`
//! - `a + (b + c)`
//!
//! into:
//! `Call Extern("nyash.string.concat3_hhh", [a, b, c])`
//!
//! Contract:
//! - Only rewrites when the inner `Add` value is single-use by the outer `Add`.
//! - Removes the folded inner instruction in the same pass (span-aligned).

mod analysis;
mod planner;
mod rewrite;

use crate::mir::{MirFunction, MirModule};

use self::analysis::{build_def_map, build_use_counts, infer_stringish_values};
use self::planner::collect_plans;
use self::rewrite::apply_plans;

pub(super) const CONCAT3_EXTERN: &str = "nyash.string.concat3_hhh";
pub(super) const CONCAT_HH_EXTERN: &str = "nyash.string.concat_hh";

/// Canonicalize string concat chains to `concat3_hhh`.
///
/// Returns number of outer instructions rewritten.
pub fn canonicalize_string_concat3(module: &mut MirModule) -> usize {
    let mut rewritten = 0usize;
    for (_name, func) in &mut module.functions {
        rewritten += canonicalize_in_function(func);
    }
    rewritten
}

fn canonicalize_in_function(function: &mut MirFunction) -> usize {
    let stringish = infer_stringish_values(function);
    let def_map = build_def_map(function);
    let use_counts = build_use_counts(function);
    let plans_by_block = collect_plans(function, &stringish, &def_map, &use_counts);
    apply_plans(function, plans_by_block)
}

#[cfg(test)]
mod tests;
