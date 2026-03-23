#![allow(unused_mut, unused_variables)]

mod by_id;
mod by_name;
mod instance_fields;
mod tagged;

pub use by_id::*;
pub use by_name::*;
pub use tagged::*;

#[cfg(test)]
mod tests;
