---
Status: Accepted
Phase: 29cc
Task: WSM-G4-min4
Title: Nyash Playground Canvas Advanced Fixture Parity Lock
Depends:
  - docs/development/current/main/phases/phase-29cc/29cc-173-wsm-g4-min3-webcanvas-fixture-parity-lock-ssot.md
  - projects/nyash-wasm/nyash_playground.html
  - apps/tests/phase29cc_wsm_g4_min4_canvas_advanced_fixture_min.hako
  - tools/smokes/v2/profiles/integration/phase29cc_wsm/g4/phase29cc_wsm_g4_min4_canvas_advanced_fixture_parity_vm.sh
---

# 29cc-174 WSM-G4-min4 Canvas Advanced Fixture Parity Lock

## Goal

`nyash_playground.html` の `canvas_advanced` ルートについて、source marker と
fixture compile parity（loop + draw primitives）を同時に固定する。

## Scope

1. `canvas_advanced` source marker を固定する。
   - `wsm_g4_min4_canvas_advanced_source_lock`
   - `wsm_g4_min4_canvas_advanced_marker_1`
   - `wsm_g4_min4_canvas_advanced_marker_2`
2. fixture `phase29cc_wsm_g4_min4_canvas_advanced_fixture_min.hako` の compile parity を固定する。
   - fixture は marker のみではなく、loop を含む描画語彙を含む。
3. 新しい wasm extern/boxcall 語彙は追加しない。

## Implementation Contract

1. `phase29cc_wsm_g4_min4_canvas_advanced_fixture_parity_vm.sh` を追加し、次を固定:
   - lock doc keyword contract
   - `nyash_playground.html` source marker contract
   - cargo test: `wasm_demo_g4_min4_canvas_advanced_fixture_compile_to_wat_contract`
   - `phase29cc_wsm_g4_min3_webcanvas_fixture_parity_vm.sh` の回帰確認
2. `wasm-demo-g2` へ min4 smoke を追加する。

## Acceptance

- `bash tools/smokes/v2/profiles/integration/phase29cc_wsm/g4/phase29cc_wsm_g4_min4_canvas_advanced_fixture_parity_vm.sh`
- `tools/checks/dev_gate.sh wasm-demo-g2`
- `tools/checks/dev_gate.sh wasm-boundary-lite`

## Next

- `WSM-G4-min5`: headless browser で `webcanvas`/`canvas_advanced` の 2-example parity を固定する。
