---
Status: Accepted
Phase: 29cc
Task: WSM-G4-min2
Title: Nyash Playground Canvas Primer Lock
Depends:
  - docs/development/current/main/phases/phase-29cc/29cc-171-wsm-g4-min1-nyash-playground-console-baseline-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-137-wsm-g3-min1-gap-inventory-lock-ssot.md
  - projects/nyash-wasm/nyash_playground.html
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g4_min2_playground_canvas_primer_vm.sh
---

# 29cc-172 WSM-G4-min2 Nyash Playground Canvas Primer Lock

## Goal

`WSM-G4-min1` の Console baseline を壊さず、`projects/nyash-wasm/nyash_playground.html` における
Canvas primer の最小語彙を docs-first で固定する。

この min2 は `WebCanvasBox` の primer 1語彙（`clear`）を中心に lock し、
advanced 形や追加 API 拡張は次タスクへ分離する。

## Scope

1. 対象は `projects/nyash-wasm/nyash_playground.html` の primer 契約のみ。
2. primer shape は次の marker で固定する（順序は不問）。
   - `wsm_g4_min2_canvas_primer_lock`
   - `wsm_g4_min2_canvas_vocab_clear`
   - `new WebCanvasBox("game-canvas", 400, 250)`
   - `me.canvas.clear()`
3. `WSM-G4-min2` では新しい extern/boxcall を増やさない。
4. acceptance は docs lock + 既存 wasm gate の緑で固定する。

## Implementation Contract

1. `phase29cc_wsm_g4_min2_playground_canvas_primer_vm.sh` を追加し、次を同時固定する:
   - lock doc keyword contract（この文書）
   - `nyash_playground.html` primer marker contract
   - 既存 run-loop baseline parity（`phase29cc_wsm_g4_min1_playground_console_baseline_vm.sh`）
2. `tools/checks/dev_gate.sh wasm-demo-g2` へ min2 smoke を追加する。
3. `wasm-boundary-lite` を regress させない。

## Acceptance

- `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g4_min2_playground_canvas_primer_vm.sh`
- `tools/checks/dev_gate.sh wasm-demo-g2`
- `tools/checks/dev_gate.sh wasm-boundary-lite`

## Next

- `WSM-G4-min3`: canvas primer を fixture parity へ昇格（最小 1 fixture）。
