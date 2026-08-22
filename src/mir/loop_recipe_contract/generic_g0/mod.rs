//! Caller-zero Generic G0 portable Recipe producer.
//!
//! This subtree owns the one Generic source-to-Recipe mapping used by the
//! caller-zero source-parent seam.  Common Recipe verification, JoinSig
//! elaboration, and source-bound Core remain shared owners; physical demand
//! and production selection stay outside this module.

mod after;
mod operation_effect;
mod producer;
mod recipe;
mod relations;

pub(crate) use after::VerifiedGenericG0TailCapabilityV1;

#[cfg(test)]
#[path = "producer_tests.rs"]
mod producer_tests;

#[cfg(test)]
pub(crate) use producer_tests::{
    generic_operation_demand_parts_for_test, generic_operation_effect_for_test,
};

pub(crate) use producer::{
    produce_generic_g0_recipe_v1, GenericG0RecipeProducerRejectV1, VerifiedGenericRecipeProductG0,
};
