---
Status: Active
Scope: code（仕様不変、Freeze taxonomy のSSOT整合）
Related:
- docs/development/current/main/phases/phase-29an/README.md
- docs/development/current/main/design/planfrag-freeze-taxonomy.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29an P3: Freeze tag `unstructured` をコード側へ追加（SSOT整合）

Date: 2025-12-29  
Status: Ready for execution  
Scope: `Freeze` の語彙を taxonomy SSOT に合わせる（未使用のまま、仕様不変）

## Objective

- `docs/development/current/main/design/planfrag-freeze-taxonomy.md` にある `unstructured`（irreducible / multi-entry 等）を、コード側 `Freeze` にも追加する
- 将来の Skeleton 一意化（P4+）で `Freeze::unstructured(...)` を使える土台を作る

## Non-goals

- 既存の挙動変更（この P3 では `unstructured` を発火させない）
- 新 env var / 恒常ログ追加
- error message の変更（`Display` の既存フォーマットは維持）

## Implementation

### Step 1: `Freeze::unstructured()` を追加

Update:
- `src/mir/builder/control_flow/plan/planner/freeze.rs`

Add:
- `pub(in crate::mir::builder) fn unstructured(message: impl Into<String>) -> Self`
  - `tag: "unstructured"`
  - `message: ...`

### Step 2: unit test を追加

同ファイル内の `#[cfg(test)]` でOK（新規モジュール不要）。

Test:
- `Freeze::unstructured("x").to_string()` が `"[plan/freeze:unstructured] x"` を含む

## Verification（required）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Commit

- `git add -A && git commit -m "phase29an(p3): add freeze unstructured tag"`

