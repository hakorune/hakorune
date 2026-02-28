---
Status: Accepted
Phase: 29cc
Task: WSM-G4-min7
Title: Nyash Playground Webdisplay Fixture Parity Lock
Depends:
  - projects/nyash-wasm/nyash_playground.html
  - apps/tests/phase29cc_wsm_g4_min7_webdisplay_fixture_min.hako
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g4_min7_webdisplay_fixture_parity_vm.sh
---

# 29cc-204 WSM-G4-min7 Webdisplay Fixture Parity Lock

## Goal

`nyash_playground.html` の `webdisplay` ルートを static prebuilt wasm で固定し、
headless 実行時の marker parity を契約化する。

## Scope

1. `webdisplay` example は `PREBUILT_WASM_MAP` から wasm を読み込む。
2. fixture marker を固定:
   - `wsm_g4_min7_webdisplay_marker_1`
   - `wsm_g4_min7_webdisplay_marker_2`
3. headless autorun marker を固定:
   - `[autorun-example] webdisplay`
   - `[autorun] done`

## Acceptance

- `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g4_min7_webdisplay_fixture_parity_vm.sh`

## Next

- `WasmDisplayBox` は current backend が user-defined BoxCall を native emit できるようになってから再昇格する。
