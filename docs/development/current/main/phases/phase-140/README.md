# Phase 140: NormalizedExprLowererBox (pure expressions)

Status: DONE ✅  
Scope: Normalized shadow の expression lowering を AST walker で一般化し、パターン爆発を避ける（pure のみ）。  
Related:
- `docs/development/current/main/design/normalized-expr-lowering.md`
- `docs/development/current/main/30-Backlog.md`
- `docs/development/current/main/phases/phase-139/README.md`
- `src/mir/control_tree/normalized_shadow/common/expr_lowerer_box.rs`
- `src/mir/control_tree/normalized_shadow/common/return_value_lowerer_box.rs`

---

## Goal

- `NormalizedExprLowererBox` を導入し、pure expression を JoinIR（Normalized dialect）へ lowering できるようにする。
- return lowering は `ReturnValueLowererBox` → `ExprLowererBox` 委譲へ寄せ、return の形追加を “総当たり” にしない。

## Scope (Phase 140 P0)

### ✅ In scope: pure expression

- `Variable`
- `Literal`（Integer/Bool から開始）
- `UnaryOp`（not, -）
- `BinaryOp`（+ - * /）
- `Compare`（==, <, <=, >, >=）

### ❌ Out of scope

- Call/MethodCall（Phase 141+）
- Short-circuit（&&/||）の制御フローを伴う lowering
- 例外/throw/try/catch
- NormalizationPlan の粒度変更（suffix→statement などの再設計は Phase 141+ の検討に回す）

## Contract

`NormalizedExprLowererBox::lower_expr(...) -> Result<Option<ValueId>, String>`

- `Ok(Some(vid))`: lowering 成功
- `Ok(None)`: out-of-scope（既存経路へフォールバック、既定挙動不変）
- `Err(_)`: 内部不整合のみ（strict では fail-fast）

## Implementation Notes

- 既存の `ReturnValueLowererBox` は “return 構文” の薄い箱に縮退し、expr lowering の実体は `ExprLowererBox` に置く。
- 型は過剰に推論しない。Const/Compare/BinOp の最小ヒントのみ（必要なら段階投入）。

## Tests

- Unit tests:
  - `cargo test -p nyash-rust --lib mir::control_tree::normalized_shadow::common`
- Smokes (regressions):
  - Phase 139（VM/LLVM EXE）
  - Phase 136/137/138（return 系）
  - Phase 97（フォールバック）

## Implementation (DONE)

- Added pure expression SSOT: `src/mir/control_tree/normalized_shadow/common/expr_lowerer_box.rs`
- `ReturnValueLowererBox` shrunk to return-syntax only and delegates to `NormalizedExprLowererBox`
- Explicitly keeps out-of-scope = `Ok(None)` (fallback) and avoids fail-fast expansion
