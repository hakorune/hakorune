# Phase 63 Summary: Ownership AST Analyzer (dev-only)

## Goal

本番ルート（MIR builder）の入力である `crate::ast::ASTNode` から、Ownership-Relay の解析結果（`OwnershipPlan`）を生成できるようにする。

- JSON v0 専用の「Local=rebind」扱いは使わない
- `normalized_dev` feature 下でのみ有効（analysis-only）

## Changes

- 追加: `src/mir/join_ir/ownership/ast_analyzer.rs`
  - `AstOwnershipAnalyzer` を実装（AST → `Vec<OwnershipPlan>`）
  - ScopeKind: Function / Loop / If / Block
  - 不変条件: `owned_vars / relay_writes / captures / condition_captures` を `OwnershipPlan` として出力
- 導線: `src/mir/join_ir/ownership/mod.rs`
  - `#[cfg(feature = "normalized_dev")] pub use ast_analyzer::*;` を追加

## Tests

- `src/mir/join_ir/ownership/ast_analyzer.rs` 内にユニットテストを追加:
  - loop-local owned + write（carrier 相当）
  - condition capture（condition_captures ⊆ captures）
  - relay write（外側 owned への更新）

## Notes

- Phase 64 で、本番 P3(if-sum) ルート（`pattern3_with_if_phi.rs`）へ dev-only で接続し、carrier order / boundary inputs の SSOT 化を開始する。
