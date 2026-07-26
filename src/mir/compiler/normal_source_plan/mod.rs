//! Source-owned normal compilation family classification.
//!
//! This module observes one owned parsed source exactly once. It does not
//! select a profile, lower MIR, choose a backend, or execute a program.

mod classifier;
mod inventory;
mod product;
mod rejection;

pub(crate) use classifier::NormalSourcePlanClassifierV1;
#[allow(unused_imports)]
pub(crate) use product::{
    PreparedNormalSourcePlanInputV1, SealedNormalCallableModuleSourceV1, SealedNormalScalarRootV1,
    SealedNormalSourcePlanV1,
};
pub(crate) use rejection::{
    NormalSourcePlanErrorV1, NormalSourcePlanStageV1, RejectedNormalSourcePlanV1,
};

#[cfg(test)]
mod tests;
