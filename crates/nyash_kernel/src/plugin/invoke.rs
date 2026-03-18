#![allow(unused_mut, unused_variables)]

mod by_id;
mod instance_fields;
mod tagged;

pub use by_id::*;
pub use tagged::*;

#[cfg(test)]
mod tests;
