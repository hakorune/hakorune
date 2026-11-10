//! Function-owned, path-sensitive Ownership SSA verification.

mod classify;
mod error;
mod model;
mod published_homes;
mod verify;

#[cfg(test)]
mod tests;

pub(crate) use error::OwnershipSsaErrorV1;
pub(crate) use model::{
    FunctionResultOwnershipV1, MirOwnershipKindV1, OwnershipFunctionAbiV1,
    OwnershipFunctionOwnerV1, OwnershipOperationKindV1, VerifiedOwnershipSsaV1,
};
pub(crate) use published_homes::verify_published_object_homes_v1;

use crate::mir::MirFunction;

pub(crate) fn verify_ownership_ssa_v1(
    function: &MirFunction,
    abi: &OwnershipFunctionAbiV1,
) -> Result<VerifiedOwnershipSsaV1, OwnershipSsaErrorV1> {
    verify::verify(function, abi, None)
}

/// Public module definitions enable a checked local child view; the path-sensitive
/// verifier and all of its existing ownership checks remain the same owner.
pub(crate) fn verify_ownership_ssa_with_module_v1(
    function: &MirFunction,
    abi: &OwnershipFunctionAbiV1,
    module: &crate::mir::MirModule,
) -> Result<(), OwnershipSsaErrorV1> {
    verify::verify(function, abi, Some(module)).map(|_| ())
}
