# 29cc-206 WSM-G4-min9 WebCanvas WasmBox Re-promotion Lock

Status: Accepted
Scope: WSM-G4-min9 として `webcanvas` fixture/playground source を marker-only ConsoleBox から `WasmCanvasBox` 直呼び出しへ再昇格し、WAT compile parity を lock する。

## 1. Goal

`webcanvas` を static route で維持しつつ、source 側の責務を「canvas marker 出力」ではなく `env.canvas.*` 呼び出し語彙へ戻す。

## 2. Invariants

1. `wsm_g4_min3_webcanvas_source_lock` / marker (`_marker_1`, `_marker_2`) は維持する。
2. fixture `apps/tests/phase29cc_wsm_g4_min3_webcanvas_fixture_min.hako` は `WasmCanvasBox` を内包し、次の extern を使う:
   - `env.canvas.clear`
   - `env.canvas.fillRect`
   - `env.canvas.fillText`
3. `projects/nyash-wasm/nyash_playground.html` の `webcanvas` source も同形に同期する。
4. compile parity テスト `wasm_demo_g4_min3_webcanvas_fixture_compile_to_wat_contract` は上記 import 名を検証する。

Marker lock:
- `wsm_g4_min3_webcanvas_marker_1`
- `wsm_g4_min3_webcanvas_marker_2`

## 3. Gate

- `bash tools/smokes/v2/profiles/integration/phase29cc_wsm/g4/phase29cc_wsm_g4_min9_webcanvas_wasmbox_repromotion_vm.sh`

## 4. Next

- `WSM-G4-min10`: `canvas_advanced` も同方式で `WasmCanvasBox` 再昇格を lock する。
