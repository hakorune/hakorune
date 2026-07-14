//! Function-owned, path-sensitive Ownership SSA verification.

// SSA-RC-V0 lands the sealed verifier before A1b/SSA-I1 consumers. Remove
// these scoped allowances when the first production consumer is connected.
#![allow(dead_code, unused_imports)]

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
