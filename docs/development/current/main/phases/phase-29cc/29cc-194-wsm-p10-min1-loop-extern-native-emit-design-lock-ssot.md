---
Status: Done
Decision: accepted-but-blocked
Date: 2026-02-27
Scope: WSM-P10-min1 として loop/extern call native emit の設計境界を固定し、P9 bridge blocker からの遷移条件を明文化する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-191-wsm-p9-min2-loop-canvas-primer-bridge-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-192-wsm-p9-min3-canvas-advanced-bridge-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-193-wsm-p9-min4-bridge-retire-refresh-lock-ssot.md
  - tools/smokes/v2/profiles/integration/phase29cc_wsm/p10/phase29cc_wsm_p10_min1_loop_extern_native_emit_design_lock_vm.sh
  - tools/checks/phase29cc_wsm_p10_loop_extern_native_emit_design_guard.sh
---

# 29cc-194 WSM-P10-min1 Loop/Extern Native Emit Design Lock

## Purpose
`WSM-P9` で固定した bridge blocker（loop/canvas extern）を次フェーズへ進めるため、
native lane の設計契約を先に固定する。

## Design Contract
1. `default` route は fail-fast を維持し、未対応 shape は引き続き `bridge-rust-backend` とする。
2. P10 の shape id 命名規約は `wsm.p10.*` に固定し、`wsm.p10.main_loop_extern_call.v0` を最小語彙とする。
3. native emit 候補は「loop + extern call + integer local update」の単一路形から開始し、1 shape ずつ追加する。
4. `shape_table` 判定と binary writer 実装は同コミットで増やさず、まず min2 で判定形のみ固定する。

## Blocker
1. 既存 native writer は `main_return_i32_const*` 族しか直接出力できない。
2. loop/branch/call を含む最小 wasm section 合成（label/local/call）の writer API が未固定。
3. route-trace は現時点で `shape_id=-`（bridge）を契約値として維持する必要がある。

## Next
1. `WSM-P10-min2`: loop+extern の最小 shape matcher 追加（still bridge, analysis-only）。
2. `WSM-P10-min3`: writer API の section contract 固定（label/local/call）。
3. `WSM-P10-min4`: 1 fixture を native emit へ昇格。

## Acceptance
1. `bash tools/smokes/v2/profiles/integration/phase29cc_wsm/p10/phase29cc_wsm_p10_min1_loop_extern_native_emit_design_lock_vm.sh`
2. `bash tools/checks/phase29cc_wsm_p10_loop_extern_native_emit_design_guard.sh`
3. `tools/checks/dev_gate.sh portability`
