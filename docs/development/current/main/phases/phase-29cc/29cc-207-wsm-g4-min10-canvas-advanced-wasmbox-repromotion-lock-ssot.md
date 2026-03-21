# 29cc-207 WSM-G4-min10 Canvas Advanced WasmBox Re-promotion Lock

Status: Accepted
Scope: WSM-G4-min10 として `canvas_advanced` fixture/playground source を marker-only ConsoleBox から `WasmCanvasBox` 直呼び出しへ再昇格し、WAT compile parity を lock する。

Depends on:
- `docs/development/current/main/phases/phase-29cc/29cc-206-wsm-g4-min9-webcanvas-wasmbox-repromotion-lock-ssot.md`

## 1. Goal

`canvas_advanced` の表示語彙を `WasmCanvasBox`（clear/fillRect/fillText）で固定し、`webcanvas` と同じ source discipline へ揃える。

## 2. Invariants

1. `wsm_g4_min4_canvas_advanced_source_lock` / marker (`_marker_1`, `_marker_2`) は維持する。
2. fixture `apps/tests/phase29cc_wsm_g4_min4_canvas_advanced_fixture_min.hako` は `WasmCanvasBox` を内包し、次の extern を使う:
   - `env.canvas.clear`
   - `env.canvas.fillRect`
   - `env.canvas.fillText`
3. `projects/nyash-wasm/nyash_playground.html` の `canvas_advanced` source も同形に同期する。
4. compile parity テスト `wasm_demo_g4_min4_canvas_advanced_fixture_compile_to_wat_contract` は上記 import 名を検証する。

Marker lock:
- `wsm_g4_min4_canvas_advanced_marker_1`
- `wsm_g4_min4_canvas_advanced_marker_2`

## 3. Gate

- `bash tools/smokes/v2/profiles/integration/phase29cc_wsm/g4/phase29cc_wsm_g4_min10_canvas_advanced_wasmbox_repromotion_vm.sh`

## 4. Note

この lock は source/compile parity 固定であり、default route の plan 種別（shape/native or bridge）の変更は含まない。
