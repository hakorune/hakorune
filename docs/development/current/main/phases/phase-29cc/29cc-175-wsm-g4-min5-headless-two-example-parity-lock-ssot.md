---
Status: Accepted
Phase: 29cc
Task: WSM-G4-min5
Title: Nyash Playground Headless Two-Example Parity Lock
Depends:
  - docs/development/current/main/phases/phase-29cc/29cc-174-wsm-g4-min4-canvas-advanced-fixture-parity-lock-ssot.md
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g4_min5_headless_two_examples_vm.sh
  - projects/nyash-wasm/nyash_playground.html
---

# 29cc-175 WSM-G4-min5 Headless Two-Example Parity Lock

## Goal

`nyash_playground.html` の headless run で `webcanvas` / `canvas_advanced` を指定実行し、
2-example parity を固定する。

## Scope

1. autorun に `example` query を追加し、`[autorun-example] <name>` を出力する。
2. headless run で次 marker を固定する:
   - webcanvas: `wsm_g4_min3_webcanvas_marker_1`, `wsm_g4_min3_webcanvas_marker_2`
   - canvas_advanced: `wsm_g4_min4_canvas_advanced_marker_1`, `wsm_g4_min4_canvas_advanced_marker_2`
3. `WSM-G4-min5` では wasm runtime の語彙追加はしない（source/run parity lock のみ）。

## Implementation Contract

1. `phase29cc_wsm_g4_min5_headless_two_examples_vm.sh` を追加し、次を固定:
   - lock doc keyword contract
   - `projects/nyash-wasm/build.sh` 成功
   - `?autorun=1&example=webcanvas` と `?autorun=1&example=canvas_advanced` の headless DOM marker parity
   - `phase29cc_wsm_g4_min4_canvas_advanced_fixture_parity_vm.sh` の回帰確認
2. `wasm-demo-g2` へ min5 smoke を追加する。

## Acceptance

- `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g4_min5_headless_two_examples_vm.sh`
- `tools/checks/dev_gate.sh wasm-demo-g2`
- `tools/checks/dev_gate.sh wasm-boundary-lite`

## Next

- `WSM-G4-min6`: G4 closeout（gate promotion / pointer sync）を固定する。
