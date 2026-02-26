---
Status: Done
Decision: accepted
Date: 2026-02-26
Scope: plugin lane `PLG-06-min4` として wave-3 rollout（Egui plugin）を docs+fixture+smoke で固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-95-plugin-lane-bootstrap-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-115-plg06-pyparser-wave3-min3-ssot.md
  - docs/development/current/main/phases/phase-29cc/README.md
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - apps/tests/phase29cc_plg06_egui_pilot_min.hako
  - tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg06_egui_pilot_vm.sh
  - plugins/nyash-egui-plugin/src/lib.rs
  - tools/vm_plugin_smoke.sh
---

# 29cc-116 PLG-06 Egui Wave-3 Min4 SSOT

## 0. Goal

`PLG-06`（wave-3）の rollout を `nyash-egui-plugin` 1件（min4）で固定する。

- 1 blocker = 1 plugin = 1 commit
- docs-first（契約）+ fixture + smoke を同時に固定
- 既存 wave-1 / wave-2 / wave-3-min1..min3 を壊さない

## 1. Boundary (fixed)

In scope:
1. Egui plugin（`EguiBox`）を plugin config 経由で VM から生成できること
2. `open` / `uiLabel` / `pollEvent` / `run` の最小動作契約（`egui_ev=` と `egui_run=void`）を fixture/smoke で固定すること
3. `nyash-egui-plugin` の TypeBox ABI 接続不整合（`abi_tag` と `invoke_id` 引数順）を修正して loader 受理を固定すること
4. `tools/vm_plugin_smoke.sh` で wave-1 pilots + wave-2 pilots + wave-3 pilots を連続検証すること

Out of scope:
1. with-egui feature 有効時の実GUI動作保証
2. plugin ABI（PLG-01）や gate pack（PLG-02）の再定義
3. `.hako` plugin 本実装（全面移植）

## 2. Contract Lock

1. fixture: `apps/tests/phase29cc_plg06_egui_pilot_min.hako`
2. smoke: `tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg06_egui_pilot_vm.sh`
3. pass条件:
   - VM実行が `rc=0`
   - 出力に `egui_ev=` を含む
   - 出力に `egui_run=void` を含む
   - `Unknown Box type: EguiBox` を含まない

## 2.1 Known Debt (explicit)

- debt tag: `[plg06/egui:ui-runtime]`
- 現状: wave-3 は headless VM での route 契約固定まで。実GUI（window/event loop）保証は with-egui 側の別管理。
- 昇格条件（次wave以降）:
  1. with-egui feature の cross-platform smoke を別レーンで固定
  2. UI event payload 契約（`pollEvent` の非空イベント）を固定

## 3. Evidence (2026-02-26)

1. `bash tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg06_egui_pilot_vm.sh` -> PASS
2. `bash tools/vm_plugin_smoke.sh` -> PASS（CounterBox + wave-1 pilots + wave-2 pilots + wave-3 pilots）
3. `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` -> PASS
4. `bash tools/smokes/v2/profiles/integration/apps/archive/phase134_plugin_best_effort_init.sh` -> PASS
5. `cargo check --bin hakorune` -> PASS

## 4. Decision

Decision: accepted

- `PLG-06-min4`（wave-3 rollout, Egui plugin）は完了。
- wave-3 plugin rollout は完了。
- plugin lane active next は `none`（monitor-only）。
