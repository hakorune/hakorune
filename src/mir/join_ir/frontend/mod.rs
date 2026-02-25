//! JoinIR Frontend: AST/CFG → JoinIR
//!
//! このモジュールは AST/CFG から JoinIR を直接生成する「フロントエンド経路」を提供する。
//!
//! ## 責務
//!
//! - **AST/CFG→JoinIR のみを扱う**: MIR/PHI には触らない
//! - **PHI 生成前に JoinIR を挿入**: JoinIR を PHI 生成の SSOT とする
//! - **既存 MIR Builder 経路は保持**: デフォルトでは OFF、実験モードでのみ有効
//!
//! ## Phase 34 での位置付け
//!
//! - **Phase 34-1**: 設計フェーズ（skeleton のみ）✅
//! - **Phase 34-2**: 実装開始（`IfSelectTest.*` simple pattern）⏳
//!   - 入力: Program(JSON v0) を AST 代わりに使用
//!   - 対象: `if cond { return 1 } else { return 2 }` のみ
//!   - 出力: JoinModule（Select + Ret）
//! - **Phase 34-3 以降**: Stage-1/Stage-B への段階的拡大
//!
//! ## Phase 34-2 の使用方法
//!
//! ```rust,ignore
//! use crate::mir::join_ir::frontend::ast_lowerer::AstToJoinIrLowerer;
//!
//! let program_json = serde_json::from_str(/* JSON v0 */)?;
//! let mut lowerer = AstToJoinIrLowerer::new();
//! let join_module = lowerer.lower_program_json(&program_json);
//!
//! // JoinIR Runner で実行
//! let result = run_joinir_function(vm, &join_module, entry, &args)?;
//! ```
//!
//! ## 関連ドキュメント
//!
//! - `docs/private/roadmap2/phases/phase-34-joinir-frontend/README.md`
//! - `docs/development/architecture/join-ir.md`

pub mod ast_lowerer;
pub mod func_meta;

// Re-export for convenience
pub use ast_lowerer::AstToJoinIrLowerer;
pub use func_meta::{JoinFuncMeta, JoinFuncMetaMap};

// Phase 34-1: skeleton 完了 ✅
// Phase 34-2: IfSelectTest.* simple pattern 実装中 ⏳
// Phase 34-3: Stage-1/Stage-B への段階的拡大（予定）
