//! Caller-zero Generic G0 portable Recipe producer.
//!
//! This subtree is test-only until the production selector/cutover rows open.
//! It owns the one source-to-Recipe mapping, while common Recipe verification,
//! JoinSig elaboration, and source-bound Core remain shared owners.

mod after;
mod operation_effect;
mod producer;
mod recipe;
mod relations;

#[cfg(test)]
#[path = "producer_tests.rs"]
mod producer_tests;

pub(crate) use producer::{
    produce_generic_g0_recipe_v1, GenericG0RecipeProducerRejectV1, VerifiedGenericRecipeProductG0,
};
