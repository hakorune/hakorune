//! Function-owned, path-sensitive Ownership SSA verification.

mod classify;
mod error;
mod model;
mod published_homes;
mod verify;

#[cfg(test)]
mod tests;

pub(crate) use error::OwnershipSsaErrorV1;
pub(crate) use published_homes::verify_published_object_homes_v1;
pub(crate) use model::{
    FunctionResultOwnershipV1, MirOwnershipKindV1, OwnershipFunctionAbiV1,
    OwnershipFunctionOwnerV1, OwnershipOperationKindV1,
    VerifiedOwnershipSsaV1,
};

use crate::mir::MirFunction;

pub(crate) fn verify_ownership_ssa_v1(
    function: &MirFunction,
    abi: &OwnershipFunctionAbiV1,
) -> Result<VerifiedOwnershipSsaV1, OwnershipSsaErrorV1> {
    verify::verify(function, abi)
}
