/*!
 Nyash Programming Language — Rust library crate.
 Provides parser, MIR, backends, runner, and supporting runtime.
*/

// Allow referring to this crate as `nyash_rust` from within the crate, matching external paths.
extern crate self as nyash_rust;

// WebAssembly support
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[path = "ast/mod.rs"]
pub mod ast; // using historical ast.rs
pub mod box_arithmetic {
    pub use crate::boxes::arithmetic::{
        AddBox, CompareBox, DivideBox, ModuloBox, MultiplyBox, SubtractBox,
    };
}
pub mod box_factory; // unified Box Factory
pub mod box_operators {
    pub use crate::boxes::operators::*;
}
#[path = "boxes/box_trait.rs"]
pub mod box_trait;
pub mod boxes;
#[path = "core/channel_box.rs"]
pub mod channel_box;
pub mod core; // core models shared by backends
#[path = "core/environment.rs"]
pub mod environment;
#[path = "core/exception_box.rs"]
pub mod exception_box;
#[path = "core/finalization.rs"]
pub mod finalization;
#[path = "core/instance_v2.rs"]
pub mod instance_v2; // simplified InstanceBox implementation
#[path = "core/method_box.rs"]
pub mod method_box;
#[path = "boxes/operator_traits.rs"]
pub mod operator_traits; // trait-based operator overloading
pub mod parser; // using historical parser.rs
#[path = "core/scope_tracker.rs"]
pub mod scope_tracker; // Box lifecycle tracking for VM
pub mod stdlib;
pub mod tokenizer;
#[path = "core/type_box.rs"]
pub mod type_box; // TypeBox system (arithmetic moved from box_trait.rs)

#[path = "core/value.rs"]
pub mod value;

pub mod messaging;
pub mod transport;

// MIR (Mid-level Intermediate Representation)
pub mod mir;
#[cfg(feature = "aot-plan-import")]
pub mod mir_aot_plan_import {
    pub use crate::mir::aot_plan_import::*;
}

// Backends
pub mod backend;
// JIT functionality archived to archive/jit-cranelift/
pub mod semantics; // Unified semantics trait for MIR evaluation/lowering

#[path = "benchmarks/mod.rs"]
pub mod benchmarks;

// BID-FFI / Plugin system (prototype)
pub mod bid;

// Configuration system
pub mod config;

// CLI system
pub mod cli;

// Runtime system (plugins, registry, etc.)
pub mod debug;
pub mod runtime;
// Unified Grammar scaffolding
pub mod grammar;
pub mod syntax; // syntax sugar config and helpers
                // Execution runner (CLI coordinator)
pub mod runner;
pub mod runner_hv1_inline_guard {}
pub mod stage1;
pub mod using; // using resolver scaffolding (Phase 15)

pub mod runner_plugin_init {
    pub use crate::runner::plugin_init::*;
}

// Host providers (extern bridge for Hako boxes)
pub mod host_providers;
// Core providers (ring1) — expose providers tree for in-crate re-exports
pub mod providers;

// C‑ABI PoC shim (20.36/20.37)
pub mod abi {
    pub mod nyrt_shim;
}

// Expose the macro engine module under a raw identifier; the source lives under `src/macro/`.
#[path = "macro/mod.rs"]
pub mod r#macro;

#[cfg(target_arch = "wasm32")]
#[path = "wasm_test/mod.rs"]
pub mod wasm_test;

#[cfg(test)]
pub mod tests;

// Re-export main types for easy access
pub use ast::{ASTNode, BinaryOperator, LiteralValue};
pub use box_arithmetic::{AddBox, CompareBox, DivideBox, ModuloBox, MultiplyBox, SubtractBox};
pub use box_factory::RuntimeError;
pub use box_trait::{BoolBox, IntegerBox, NyashBox, StringBox, VoidBox};
pub use boxes::console_box::ConsoleBox;
pub use boxes::debug_box::DebugBox;
pub use boxes::map_box::MapBox;
pub use boxes::math_box::{FloatBox, MathBox, RangeBox};
pub use boxes::null_box::{null, NullBox};
pub use boxes::random_box::RandomBox;
pub use boxes::sound_box::SoundBox;
pub use boxes::time_box::{DateTimeBox, TimeBox, TimerBox};
pub use channel_box::{ChannelBox, MessageBox};
pub use environment::{Environment, PythonCompatEnvironment};
pub use instance_v2::InstanceBox; // 🎯 新実装テスト（nyash_rustパス使用）
pub use method_box::{BoxType, EphemeralInstance, FunctionDefinition, MethodBox};
pub use parser::{NyashParser, ParseError};
pub use tokenizer::{NyashTokenizer, Token, TokenType};
pub use type_box::{MethodSignature, TypeBox, TypeRegistry}; // 🌟 TypeBox exports

pub use value::NyashValue;

// WASM support to be reimplemented with VM/LLVM backends
