# Phase 61 Summary: IF-SUM + BREAK (dev-only, structural)

## Goal

Phase 61 は「Break(P2) に P3 固有ロジックを混ぜない」を達成しつつ、selfhost の `if-sum + break` 形状を **by-name なし**で JoinIR Frontend に載せる。

## Problem (Phase 61 前)

- `break_pattern.rs`（Break/P2 箱）の param-order 決定で、`selfhost_if_sum_p3*` を名指しして P3 helper を優先する案は、
  - 責務混線（Break 箱が P3(if-sum) の意味論まで背負う）
  - dev-only でも撤去困難な by-name 分岐になりやすい
  という理由で採用しない。

## Solution (Phase 61)

### 1) Break(P2) を構造ベースに戻す

- `src/mir/join_ir/frontend/ast_lowerer/loop_patterns/break_pattern.rs`
  - Break(P2) の ownership 経路は `plan_to_p2_inputs_with_relay` のみを使用し、P3 専用分岐を排除する。

### 2) 新箱 `if_sum_break_pattern` を追加（dev-only）

- `src/mir/join_ir/frontend/ast_lowerer/loop_patterns/if_sum_break_pattern.rs`
  - 対象: `break-if + update-if + counter-update + return Var+Var`
  - 検出は構造のみ（by-name なし）。
  - lowering は Select ベースで `sum`/`count` を更新し、exit では `sum + count` を 1 値にして `k_exit` に渡す。

### 3) Ownership を SSOT に使う（param order / carriers）

- `OwnershipAnalyzer` → `plan_to_p3_inputs_with_relay` で carriers/captures を得る。
- Fail-Fast: carriers が Return の 2 変数と一致しない場合は拒否（canonical P3 群との混線防止）。
- relay は単一hopのみ許可（multi-hop は Fail-Fast 維持）。

## Fail-Fast Contract

- Return が `Var + Var` 以外なら対象外。
- loop body が `[break-if, update-if, counter-update]` の最小形状から外れるなら対象外。
- counter update が `i = i + 1` 形で検出できないなら Err。
- relay_path.len() > 1 は Err。
- carriers != return vars は Err。

## Tests

- `tests/normalized_joinir_min.rs`
  - selfhost P3 fixture を Program(JSON v0) として解析し、relay_writes→carriers 変換が成立することを固定。
  - selfhost P3（NormalizedDev 経路）については stdout/exit + 期待値（意味論）で固定し、比較テスト依存を避ける。

## Notes (Reproducibility)

- `docs/private` は submodule のため、fixture JSON を参照する場合は **submodule 側で追跡されていること**を前提とする。
  - 未追跡のままローカルだけで存在すると、clean checkout で `include_str!` が壊れる。

## Next Candidates

- Phase 62: P3(if-sum) の「本番 MIR→JoinIR ルート」へ OwnershipPlan を渡す設計（AST-based ownership 解析の接続点を設計 → dev-only で段階接続）。
- Phase 63: ASTNode → OwnershipPlan の analyzer を追加（analysis-only, dev-only）。
- Phase 63+: multi-hop / merge relay の意味論設計（Fail-Fast を解除する前に SSOT と不変条件を明文化）。
