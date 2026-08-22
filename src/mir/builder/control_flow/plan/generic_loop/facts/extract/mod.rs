mod collection;
#[cfg(test)]
mod progression_role_baseline_tests;
#[cfg(test)]
mod successful_extraction_tests;
#[cfg(test)]
pub(in crate::mir::builder) mod test_support;
#[cfg(test)]
mod tests;
mod v0;
mod v1;

pub(in crate::mir::builder) use collection::observe_generic_loop_carrier_observation;
pub(in crate::mir::builder) use v0::try_extract_generic_loop_v0_facts;
#[allow(unused_imports)] // Passive O0-S0 primary; first sibling consumer lands in O0-R0.
pub(in crate::mir::builder) use v1::{
    has_generic_loop_v1_recipe_hint, has_generic_loop_v1_recipe_hint_with_policy,
    try_extract_generic_loop_v1, try_extract_generic_loop_v1_facts,
    try_extract_generic_loop_v1_facts_with_policy, try_extract_generic_loop_v1_with_policy,
};
