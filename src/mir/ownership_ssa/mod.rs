//! Function-owned, path-sensitive Ownership SSA verification.

mod classify;
mod error;
mod model;
mod verify;

#[cfg(test)]
mod tests;

pub(crate) use error::OwnershipSsaErrorV1;
pub(crate) use model::{
    FunctionResultOwnershipV1, MirOwnershipKindV1, OwnershipDispositionV1, OwnershipFunctionAbiV1,
    OwnershipFunctionOwnerV1, OwnershipOperationKindV1, OwnershipOperationV1,
    VerifiedOwnershipSsaV1,
};

use crate::mir::MirFunction;

pub(crate) fn verify_ownership_ssa_v1(
    function: &MirFunction,
    abi: &OwnershipFunctionAbiV1,
) -> Result<VerifiedOwnershipSsaV1, OwnershipSsaErrorV1> {
    verify::verify(function, abi)
}
