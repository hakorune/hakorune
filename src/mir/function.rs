/*!
 * MIR Function and Module - High-level MIR organization
 *
 * Functions contain basic blocks and SSA values, modules contain functions.
 * This parent module keeps the public surface stable while thinner child
 * modules own data definitions, behavior, display helpers, and tests.
 */

mod display;
mod function_impl;
mod module_impl;
mod types;

#[cfg(test)]
mod tests;

pub use types::{
    ClosureBodyId, FunctionMetadata, FunctionSignature, FunctionStats, MirEnumDecl,
    MirEnumVariantDecl, MirFunction, MirModule, ModuleMetadata, ModuleStats, UserBoxFieldDecl,
};
