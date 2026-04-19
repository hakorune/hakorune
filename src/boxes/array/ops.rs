//! ArrayBox operations entry point.
//! Responsibility-specific impls live under `ops/`.

use super::*;

mod access;
mod capacity;
mod mutation;
mod sequence;
mod shared;
mod text;

use storage::ArrayStorage;
