---
Status: Done
Decision: accepted
Date: 2026-02-25
Scope: plugin lane の gate pack（PLG-02）を固定し、quick/milestone 受け入れを done 判定まで閉じる。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-95-plugin-lane-bootstrap-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-96-plugin-abi-loader-acceptance-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-98-plg03-counterbox-wave1-pilot-ssot.md
  - docs/development/current/main/phases/phase-29cc/README.md
  - tools/plugin_v2_smoke.sh
  - tools/smoke_plugins.sh
  - tools/smokes/v2/profiles/integration/apps/phase134_plugin_best_effort_init.sh
  - tools/checks/windows_wsl_cmd_smoke.sh
  - src/runtime/ring0/mod.rs
---

# 29cc-97 Plugin Gate Pack Lock SSOT

## 0. Goal

`PLG-02` として plugin lane の quick/milestone gate pack を固定し、
受け入れ条件と現ブロッカーを 1 枚に集約する。

## 1. Gate Pack Contract

Quick (daily):
1. `cargo check --bin hakorune`
2. `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`
3. `bash tools/smokes/v2/profiles/integration/apps/phase134_plugin_best_effort_init.sh`
4. `bash tools/checks/phase29y_derust_blocker_sync_guard.sh`

Milestone:
1. `bash tools/plugin_v2_smoke.sh`
2. `bash tools/smoke_plugins.sh`
3. `bash tools/checks/windows_wsl_cmd_smoke.sh --build --cmd-smoke`

## 2. Acceptance (PLG-02 done criteria)

1. quick 4本が連続で PASS。
2. milestone 3本が PASS。
3. 失敗時は blocker を明示し、PASS するまで `PLG-02` を done にしない。

## 3. Evidence (2026-02-25, re-verified)

Quick:
1. `cargo check --bin hakorune` -> PASS
2. `phase29bq_fast_gate_vm.sh --only bq` -> PASS
3. `phase134_plugin_best_effort_init.sh` -> PASS
4. `phase29y_derust_blocker_sync_guard.sh` -> PASS

Milestone:
1. `tools/plugin_v2_smoke.sh` -> PASS
2. `tools/smoke_plugins.sh` -> PASS
3. `windows_wsl_cmd_smoke.sh --build --cmd-smoke` -> PASS

note:
- `PLG-02-BFIX-01`（Ring0 未初期化）を解消した。
- `PLG-02-BFIX-02`（legacy nyash binary exit=2）を解消した。
- `tools/plugin_v2_smoke.sh` / `tools/smoke_plugins.sh` は `hakorune` 実行 + parser-stable fixture へ更新済み。

## 4. Blocker History

1. `PLG-02-BFIX-01`:
   - id: `plg02.ring0_init_for_nyash_plugin_smokes`
   - fix: Ring0 global 初期化を `ensure_global_ring0_initialized()` に集約して早期ログ経路を保護

2. `PLG-02-BFIX-02`:
   - id: `plg02.legacy_nyash_binary_exit2_in_plugin_smokes`
   - fix: milestone scripts を `hakorune` 実行へ統一し、壊れた fixture/assertion を現行契約へ更新

current blocker:
- none

## 5. Next

active next:
- `PLG-04`（wave rollout）

promotion condition:
- satisfied（2026-02-25）
