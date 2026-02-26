---
Status: Active
Decision: accepted
Date: 2026-02-26
Scope: PLG-04 wave-1 complete 後の de-rust/selfhost mainline 実行順を固定する（plugin wave-2 主線 + WASM 修正並行）。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-29cc/README.md
  - docs/development/current/main/phases/phase-29cc/29cc-95-plugin-lane-bootstrap-ssot.md
  - docs/development/current/main/design/de-rust-master-task-map-ssot.md
  - docs/development/current/main/design/de-rust-lane-map-ssot.md
  - docs/development/current/main/design/de-rust-post-g1-runtime-plan-ssot.md
  - docs/development/current/main/design/de-rust-runtime-meaning-decision-red-inventory-ssot.md
  - docs/guides/wasm-guide/planning/unsupported_features.md
---

# 29cc-105 Post-Wave1 Route Lock SSOT

## 0. Goal

PLG-04 wave-1 完了後に、脱Rustセルフホスティングの主線を迷走なく固定する。

1. 主線: plugin wave-2 を docs-first で順次実装（1 blocker = 1 plugin = 1 commit）
2. 並行: WASM `executor.rs` 周辺修正は non-blocking lane として進める
3. gate FAIL 時は pointer を PROMOTE せず、failure-driven で lane を再起動する

## 1. Fixed Order (post-wave1)

1. `PLG-05-min1` (active): wave-2 entry lock（最初の1件を docs+fixture+smoke で固定）
2. `PLG-05-minN`: wave-2 plugins を 1件ずつ固定（`29cc-95` inventory 順）
3. `PLG-06-min1`: wave-3 entry lock（wave-2 完了後）

WASM parallel lane（non-blocking）:
1. `WSM-01`: wasm unsupported inventory の最小修正（BoxCall/ExternCall 起点）
2. `WSM-02+`: runtime parity 影響を `phase-29y` gates で検証

## 2. Route Decision

Decision: accepted

- Mainline は **Wave-2-first**（plugin lane 継続）を採用する。
- WASM 修正は並行で進めるが、mainline blocker にはしない。
- runtime parity を壊した場合のみ lane C blocker を再起動する。

## 3. Acceptance Gates

Daily quick (plugin mainline):
1. `cargo check --bin hakorune`
2. `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`
3. `bash tools/smokes/v2/profiles/integration/apps/archive/phase134_plugin_best_effort_init.sh`

Milestone:
1. `bash tools/plugin_v2_smoke.sh`
2. `bash tools/smoke_plugins.sh`
3. `bash tools/checks/windows_wsl_cmd_smoke.sh --build --cmd-smoke`

When WASM/runtime touched:
1. `bash tools/smokes/v2/profiles/integration/apps/phase29y_lane_gate_quick_vm.sh`
2. `bash tools/smokes/v2/profiles/integration/apps/phase29y_lane_gate_vm.sh`

## 4. Active Next

- plugin lane active next: `PLG-05-min3`（wave-2 rollout）
- wasm lane active next: `WSM-01`（non-blocking parallel）
