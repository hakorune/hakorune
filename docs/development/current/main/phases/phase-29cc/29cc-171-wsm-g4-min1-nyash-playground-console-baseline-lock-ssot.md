---
Status: Done
Decision: accepted
Date: 2026-02-27
Scope: WSM-G4-min1（projects/nyash-wasm）として nyash_playground run loop の Console baseline + 1 fixture parity を docs-first で固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-130-wsm02d-min2-demo-min-fixture-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-134-wsm-g2-min1-bridge-run-loop-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-135-wsm-g2-min2-headless-run-lock-ssot.md
  - projects/nyash-wasm/nyash_playground.html
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g4_min1_playground_console_baseline_vm.sh
  - tools/checks/dev_gate.sh
---

# 29cc-171 WSM-G4-min1 Nyash Playground Console Baseline Lock

## Purpose
`projects/nyash-wasm` の移植開始点として、`nyash_playground.html` の run loop を最小境界で固定する。  
このミニタスクは Console loop only（`wsm02d_demo_min_*` marker）に限定し、canvas/advanced/public variant は対象外にする。

## Decision
1. 対象HTMLは `projects/nyash-wasm/nyash_playground.html` のみ（`*_public` は非対象）。
2. run loop 契約は `apps/tests/phase29cc_wsm02d_demo_min.hako` の marker 5件と一致させる。
   - `wsm02d_demo_min_log`
   - `wsm02d_demo_min_warn`
   - `wsm02d_demo_min_error`
   - `wsm02d_demo_min_info`
   - `wsm02d_demo_min_debug`
3. acceptance は次の3本を必須とする（Light+Boundary）:
   - `phase29cc_wsm_g4_min1_playground_console_baseline_vm.sh`
   - `tools/checks/dev_gate.sh wasm-demo-g2`
   - `tools/checks/dev_gate.sh wasm-boundary-lite`
4. `WSM-G4-min1` では新語彙を増やさない。Canvas/advanced 追加は `WSM-G4-min2` 以降へ分離する。

## Implemented
1. `phase29cc_wsm_g4_min1_playground_console_baseline_vm.sh` を追加し、次を同時固定:
   - bridge build baseline（`phase29cc_wsm_g2_min1_bridge_build_vm.sh`）
   - browser autorun marker parity（`phase29cc_wsm_g2_browser_run_vm.sh`）
   - fixture compile boundary parity（`phase29cc_wsm02d_demo_min_boundary_vm.sh`）
2. `tools/checks/dev_gate.sh wasm-demo-g2` に G4-min1 smoke を追加し、日常実行で回帰を止める。

## Acceptance
- `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g4_min1_playground_console_baseline_vm.sh`
- `tools/checks/dev_gate.sh wasm-demo-g2`
- `tools/checks/dev_gate.sh wasm-boundary-lite`

## Next
- `WSM-G4-min2`: `projects/nyash-wasm` canvas primer（最小1語彙）を docs-first で固定する。
