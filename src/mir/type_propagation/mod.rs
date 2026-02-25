//! Phase 279 P0: Type propagation SSOT module
//!
//! # 責務
//!
//! 型伝播パイプラインの SSOT 入口を提供する。
//! lifecycle.rs と joinir_function_converter.rs の両方がこのモジュールを使用する。
//!
//! # 公開 API
//!
//! - `TypePropagationPipeline::run()` - SSOT 型伝播パイプライン入口
//!
//! # 設計原則
//!
//! - **入口一本化**: 全てのルートが TypePropagationPipeline::run() を呼ぶ
//! - **Private step**: PHI 推論を private にして、直接呼び出しを防止
//! - **順序固定**: Copy → BinOp → Copy → PHI（順序ドリフト防止）

mod pipeline;

pub use pipeline::TypePropagationPipeline;
