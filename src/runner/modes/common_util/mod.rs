/*!
 * Shared helpers for runner/modes/common.rs
 *
 * Minimal extraction to reduce duplication and prepare for full split.
 */

pub mod core_bridge;
pub mod diag;
pub mod emit_direct;
pub mod entry_selection;
pub mod exec;
pub mod hako;
pub mod io;
pub mod legacy;
pub mod plugin_guard;
pub mod provider_registry;
pub mod resolve;
pub mod safety_gate;
pub mod selfhost;
pub mod selfhost_exe;
pub mod source_hint;
pub mod vm_source_prepare;
pub mod vm_user_factory;
pub mod user_box_factory;
pub mod verifier_gate;
