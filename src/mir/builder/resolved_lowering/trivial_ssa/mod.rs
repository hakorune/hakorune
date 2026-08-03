//! Function-owned Binding SSA lowering for the closed trivial profile.

mod callable_abi;
mod direct_call;
mod direct_call_type;
mod if_recipe_physicalizer;
mod lowerer;
mod operation;
mod operation_type;
mod parameter_entry;

pub(super) use callable_abi::install_trivial_callable_abi_v1;
pub(super) use lowerer::CanonicalTrivialSsaLowererV1;
