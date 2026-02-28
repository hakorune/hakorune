---
Status: Active
Decision: accepted
Date: 2026-02-28
Scope: HM2 closeout 後の「残り Rust plugin」3件を棚卸しし、mainline 対象/monitor-only 対象/retire 対象を SSOT で固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-29cc/README.md
  - docs/development/current/main/phases/phase-29cc/29cc-212-plg-hm2-min3-route-policy-matrix-lock-ssot.md
---

# 29cc-213 Plugin Residue Classification Lock

## Purpose
HM2 完了後の plugin lane を monitor-only で維持しながら、残り Rust plugin の扱いを固定する。

## Current State

1. plugin lane は `active next: none`（monitor-only）。
2. HM2-min1/min2/min3 の lock は完了済み。
3. reopen は failure-driven のみ。

## Residue Classification (fixed)

1. `plugins/nyash-fixture-plugin`
   - 判定: `test-only keep`
   - 理由: `tools/smokes/v2/profiles/plugins/*` と `tools/smokes/v2/lib/plugin_manager.sh` の loader/auto-load 検証で使用。mainline provider ではない。
2. `plugins/nyash-integer-plugin`
   - 判定: `mainline keep`
   - 理由: `IntCellBox` 提供元として `nyash.toml`/`hako.toml`/plugin smoke で現用。`IntegerBox` core 予約契約（29cc-100）とも整合。
3. `plugins/nyash-math`（legacy）
   - 判定: `retire`
   - 理由: 現用は `plugins/nyash-math-plugin`。legacy 側は mainline/CI 参照がなく重複のみを生むため削除。

## Execution (this lock)

1. `plugins/nyash-math/Cargo.toml` を削除。
2. `plugins/nyash-math/src/lib.rs` を削除。
3. legacy binary artifact `plugins/nyash-math/libnyash_math.so` と空ディレクトリを削除。

## Acceptance

1. `rg -n "nyash-math" tools plugins nyash.toml hako.toml` で現用参照が `nyash-math-plugin` 側のみであること。
2. `tools/checks/dev_gate.sh plugin-module-core8` が緑（plugin lane 既存契約維持）。
3. `CURRENT_TASK.md` と `phase-29cc/README.md` が本分類と同期していること。
