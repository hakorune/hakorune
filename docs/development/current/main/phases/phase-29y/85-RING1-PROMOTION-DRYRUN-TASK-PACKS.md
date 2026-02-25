---
Status: Historical (completed; pointer-only)
Decision: accepted
Date: 2026-02-19
Scope: ring1 provisional domain（array/map/path/console）昇格時に使った dry-run task pack の履歴を保持する。現行ステータスは `60-NEXT-TASK-PLAN.md` を正本とする。
Related:
  - docs/development/current/main/design/ring1-core-provider-promotion-template-ssot.md
  - docs/development/current/main/design/ring1-core-provider-scope-ssot.md
  - docs/development/current/main/phases/phase-29y/60-NEXT-TASK-PLAN.md
  - CURRENT_TASK.md
---

# Phase 29y Ring1 Promotion Dry-Run Task Packs

## Purpose

- `RING1-CORE-05` として、provisional 4 domain 全件の dry-run 実施結果を 1 枚に固定する。
- 実装開始前に「どのコミットで何を触るか」を先に決め、`min1/min2/min3` の混線を防いだ履歴を残す。

## Fixed Rules

1. `min1`: provider 実装 + runtime wiring のみ（smoke/guard/lane gate は触らない）
2. `min2`: fixture + smoke + guard のみ（provider 実装ロジックは触らない）
3. `min3`: docs 同期 + lane gate 統合のみ（provider 実装は触らない）

受け入れコマンド（共通）:

```bash
bash tools/checks/ring1_core_scope_guard.sh
bash tools/checks/phase29y_lane_gate_guard.sh
bash tools/smokes/v2/profiles/integration/apps/phase29y_lane_gate_vm.sh
```

## Domain Packs (historical dry-run records)

### `array` (`RING1-CORE-06`, completed)

- `min1` planned scope:
  - `src/providers/ring1/mod.rs`（`pub mod array;`）
  - `src/providers/ring1/array/mod.rs`（最小 provider 骨格）
  - `src/runtime/provider_lock/mod.rs`（array dispatch wiring）
- `min2` planned scope:
  - `apps/tests/ring1_array_provider/array_size_push_min.hako`
  - `tools/smokes/v2/profiles/integration/apps/ring1_array_provider_vm.sh`
  - `tools/checks/ring1_array_provider_guard.sh`
- `min3` planned scope:
  - `docs/development/current/main/design/ring1-core-provider-scope-ssot.md`（array row: `provisional -> accepted`）
  - `src/providers/ring1/array/README.md`（active 更新）
  - `docs/development/current/main/phases/phase-29y/60-NEXT-TASK-PLAN.md`
  - `CURRENT_TASK.md`

### `map` (`RING1-CORE-07`, completed)

- `min1` planned scope:
  - `src/providers/ring1/mod.rs`（`pub mod map;`）
  - `src/providers/ring1/map/mod.rs`（最小 provider 骨格）
  - `src/runtime/provider_lock/mod.rs`（map dispatch wiring）
- `min2` planned scope:
  - `apps/tests/ring1_map_provider/map_get_set_min.hako`
  - `tools/smokes/v2/profiles/integration/apps/ring1_map_provider_vm.sh`
  - `tools/checks/ring1_map_provider_guard.sh`
- `min3` planned scope:
  - `docs/development/current/main/design/ring1-core-provider-scope-ssot.md`（map row 更新）
  - `src/providers/ring1/map/README.md`（active 更新）
  - `docs/development/current/main/phases/phase-29y/60-NEXT-TASK-PLAN.md`
  - `CURRENT_TASK.md`

### `path` (`RING1-CORE-08`, completed)

- `min1` planned scope:
  - `src/providers/ring1/mod.rs`（`pub mod path;`）
  - `src/providers/ring1/path/mod.rs`（最小 provider 骨格）
  - `src/runtime/provider_lock/mod.rs`（path dispatch wiring）
- `min2` planned scope:
  - `apps/tests/ring1_path_provider/path_join_exists_min.hako`
  - `tools/smokes/v2/profiles/integration/apps/ring1_path_provider_vm.sh`
  - `tools/checks/ring1_path_provider_guard.sh`
- `min3` planned scope:
  - `docs/development/current/main/design/ring1-core-provider-scope-ssot.md`（path row 更新）
  - `src/providers/ring1/path/README.md`（active 更新）
  - `docs/development/current/main/phases/phase-29y/60-NEXT-TASK-PLAN.md`
  - `CURRENT_TASK.md`

### `console` (`RING1-CORE-09`, completed)

- `min1` planned scope:
  - `src/providers/ring1/mod.rs`（`pub mod console;`）
  - `src/providers/ring1/console/mod.rs`（最小 provider 骨格）
  - `src/runtime/provider_lock/mod.rs`（console dispatch wiring）
- `min2` planned scope:
  - `apps/tests/ring1_console_provider/console_warn_error_min.hako`
  - `tools/smokes/v2/profiles/integration/apps/ring1_console_provider_vm.sh`
  - `tools/checks/ring1_console_provider_guard.sh`
- `min3` planned scope:
  - `docs/development/current/main/design/ring1-core-provider-scope-ssot.md`（console row 更新）
  - `src/providers/ring1/console/README.md`（active 更新）
  - `docs/development/current/main/phases/phase-29y/60-NEXT-TASK-PLAN.md`
  - `CURRENT_TASK.md`

## Result (RING1-CORE-05)

- dry-run task pack は 4 domain 分すべて確定済み。
- `RING1-CORE-06-min1/min2/min3` で `array` は `accepted` へ昇格済み。
- `RING1-CORE-07-min1/min2/min3` で `map` は `accepted` へ昇格済み。
- `RING1-CORE-08-min1/min2/min3` で `path` は `accepted` へ昇格済み。
- `RING1-CORE-09-min1/min2/min3` で `console` は `accepted` へ昇格済み。
- provisional 4 domain（`array/map/path/console`）の昇格は完了。
- 現在の ring1 状態と運用順序は `docs/development/current/main/phases/phase-29y/60-NEXT-TASK-PLAN.md` を正本とする。
