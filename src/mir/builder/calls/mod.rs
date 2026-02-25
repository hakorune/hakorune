//! 🎯 箱理論: Call系処理のモジュール分離
//!
//! 責務別に明確に分離された「箱」の集合：
//! - lowering: 関数lowering（static/instance method → MIR function）
//! - utils: ユーティリティ（resolve/parse/extract）
//! - emit: Call命令発行（統一Call/Legacy Call） ✅ Phase 2完了
//! - build: Call構築（function call/method call） ✅ Phase 2完了
//! - guard: 構造ガード（静的Box/ランタイムBox混線防止） ✅ Phase 25.1d完了

// Existing modules (already implemented elsewhere)
pub mod annotation;
pub mod call_target;
pub mod call_unified;
pub mod extern_calls;
pub mod function_lowering;
pub mod method_resolution;
pub mod special_handlers;

// New refactored modules (Box Theory Phase 1 & 2 & 25.1d & Phase 3 & Step 4 & Step 5)
pub mod build; // Phase 2: Call building
pub mod context_lifecycle; // Lowering context lifecycle management (prepare/restore)
pub mod debug_method_routing; // Debug/REPL/MIR method routing (extracted from build.rs)
pub mod effects_analyzer; // Phase 3-B: Effects analyzer (エフェクト解析専用箱)
pub mod emit; // Phase 2: Call emission
pub mod guard; // Phase 25.1d: Structural guard (static/runtime box separation)
pub mod lowering;
pub mod materializer;
pub mod parameter_setup; // Step 3: Parameter setup and binding (static/instance methods)
pub mod receiver_binding; // Step 4: Receiver ('me'/'this') normalization and binding
pub mod resolver; // Phase 25.1d: Callee resolution (CallTarget → Callee)
pub mod skeleton_builder; // Step 5: Function/method skeleton creation
pub mod special_method_handlers; // Special method handlers (TypeOp, math, str normalization)
pub mod static_resolution; // Step 3: Static method resolution and fallback logic
pub mod unified_emitter; // Phase 3-A: Unified call emitter (統一Call発行専用箱)
pub mod utils; // Phase 3-C: Call materializer (Call前処理・準備専用箱)

// Re-export public interfaces
#[allow(unused_imports)]
pub use build::*;
#[allow(unused_imports)]
pub use call_target::CallTarget;
#[allow(unused_imports)]
pub use emit::*;
#[allow(unused_imports)]
pub use lowering::*;
#[allow(unused_imports)]
pub use utils::*;
