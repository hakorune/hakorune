---
Status: Done
Decision: accepted
Date: 2026-02-27
Scope: WSM-P7-min3 として `projects/nyash-wasm` 由来の 2 デモを `.hako` default route で lock する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-185-wsm-p7-min2-default-hako-only-guard-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-175-wsm-g4-min5-headless-two-example-parity-lock-ssot.md
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p7_min3_two_demo_lock_vm.sh
  - projects/nyash-wasm/nyash_playground.html
---

# 29cc-186 WSM-P7-min3 Two Demo Lock

## Purpose
`.hako` default route の実利用境界として、`projects/nyash-wasm` の代表 2 ケースを固定する。

## Decision
1. 固定対象は `webcanvas` / `canvas_advanced` の 2 ケース。
2. 2 ケースの headless parity は既存 `WSM-G4-min5` 契約を正本として再利用する。
3. P7-min3 では新しい fallback を追加しない。未対応は fail-fast のまま扱う。

## Implemented
1. lock smoke 追加:
   - `tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p7_min3_two_demo_lock_vm.sh`
2. smoke は次を実行する:
   - `phase29cc_wsm_g4_min5_headless_two_examples_vm.sh`
3. lock doc は `projects/nyash-wasm` と 2 例の対応を明記する。

## Acceptance
1. `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p7_min3_two_demo_lock_vm.sh`
2. `tools/checks/dev_gate.sh wasm-demo-g3-full`

## Next
1. `WSM-P7-min4` compat retention lock（期限付き保持 + rollback 契約）へ進む。
