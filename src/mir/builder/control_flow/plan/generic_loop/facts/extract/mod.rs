mod collection;
#[cfg(test)]
mod tests;
mod v0;
mod v1;

pub(in crate::mir::builder) use v0::try_extract_generic_loop_v0_facts;
pub(in crate::mir::builder) use v1::{
    has_generic_loop_v1_recipe_hint, try_extract_generic_loop_v1_facts,
};
