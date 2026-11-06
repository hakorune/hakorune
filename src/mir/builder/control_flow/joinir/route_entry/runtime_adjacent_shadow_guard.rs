//! Runtime-adjacent ProgramJSON shadow guard boundary.
//!
//! This module is intentionally read-only. It records the boundary immediately
//! after Rust `try_build_outcome(ctx)` and before route registry candidate
//! selection. ProgramJSON shadow evidence is verified by lifecycle gates; this
//! Rust seam must not write shadow data into route authority.

