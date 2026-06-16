//! Object representation planning vocabulary.
//!
//! This module is intentionally passive. It names exact-AOT object storage
//! outcomes, but it does not choose them, does not mutate MIR, and is not wired
//! to lowering. MIRBuilder records object meaning; later analysis can produce
//! these plans for backend consumers.

#[path = "object_storage_plan/alias.rs"]
mod alias;
#[path = "object_storage_plan/fastpath.rs"]
mod fastpath;
#[path = "object_storage_plan/ids.rs"]
mod ids;
#[path = "object_storage_plan/inventory.rs"]
mod inventory;
#[path = "object_storage_plan/publication.rs"]
mod publication;
#[path = "object_storage_plan/report.rs"]
mod report;
#[path = "object_storage_plan/storage.rs"]
mod storage;

pub use alias::*;
pub use fastpath::*;
pub use ids::*;
pub use inventory::*;
pub use publication::*;
pub use report::*;
pub use storage::*;

#[cfg(test)]
#[path = "object_storage_plan/tests.rs"]
mod tests;
