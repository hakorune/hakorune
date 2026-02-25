//! MirBuilder utility functions - orchestrator module
//!
//! This module organizes builder helper functions into focused submodules.
//!
//! ## Module Organization (8 modules)
//!
//! ### Core Infrastructure
//! - **debug**: Builder debug logging (NYASH_BUILDER_DEBUG)
//! - **id_alloc**: Value/Block ID allocation with PHI reservation (Phase 136, 201-A)
//!
//! ### SSA & Type Operations
//! - **local_ssa**: LocalSSA convenience wrappers (recv, arg, cond, etc.)
//! - **type_ops**: Type checking and casting (dead_code, future use)
//!
//! ### Memory & Concurrency
//! - **weak_ref**: WeakRef and Barrier operations (Phase 285A1)
//! - **pinning**: Value pinning/slotification for PHI participation (Phase 25.1b)
//!
//! ### Block & Call Management
//! - **block_mgmt**: Basic block creation and switching (~32 lines legacy cleanup)
//! - **boxcall_emit**: BoxCall emission with routing logic (Phase 87, 84-4-B)
//!
//! ## Design Principles
//! - Single responsibility: Each module handles ONE concern
//! - Phase preservation: All Phase comments preserved in extracted modules
//! - Zero circular dependencies: Clean module hierarchy
//! - Legacy cleanup: ~32 lines of disabled code removed (saved to git history)

mod block_mgmt;
mod boxcall_emit;
mod debug;
mod id_alloc;
mod local_ssa;
mod pinning;
mod type_ops;
mod weak_ref;

// Re-export debug utilities (used across builder modules)
pub(super) use debug::{builder_debug_enabled, builder_debug_log};
