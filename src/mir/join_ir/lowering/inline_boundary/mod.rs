//! Phase 188-Impl-3: JoinInlineBoundary - Boundary information for JoinIR inlining
//!
//! This module defines the boundary between JoinIR fragments and the host MIR function.
//! It enables clean separation of concerns:
//!
//! - **Box A**: JoinIR Frontend (doesn't know about host ValueIds)
//! - **Box B**: Join→MIR Bridge (converts to MIR using local ValueIds)
//! - **Box C**: JoinInlineBoundary (stores boundary info)
//! - **Box D**: JoinMirInlineMerger (injects Copy instructions at boundary)
//!
//! ## Design Philosophy
//!
//! The JoinIR lowerer should work with **JoinIR-side ValueIds** allocated via
//! `JoinValueSpace` (Param: 100-999, Local: 1000+) without knowing anything about
//! the host function's ValueId space. This ensures:
//!
//! 1. **Modularity**: JoinIR lowerers are pure transformers
//! 2. **Reusability**: Same lowerer can be used in different contexts
//! 3. **Testability**: JoinIR can be tested independently
//! 4. **Correctness**: SSA properties are maintained via explicit Copy instructions

mod constructors;
mod types;

#[cfg(test)]
mod tests;

pub use types::{JoinInlineBoundary, JumpArgsLayout, LoopExitBinding};
