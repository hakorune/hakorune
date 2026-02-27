---
Status: Done
Decision: accepted-but-blocked
Date: 2026-02-27
Scope: WSM-P9-min2 として loop/canvas primer 経路が現状 BridgeRustBackend であることを固定し、native移植のブロッカーを明示する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-190-wsm-p9-min1-const-binop-native-shape-lock-ssot.md
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p9_min2_loop_canvas_primer_bridge_lock_vm.sh
---

# 29cc-191 WSM-P9-min2 Loop + Canvas Primer Bridge Lock

## Purpose
`webcanvas` 相当の loop + extern 呼び出し経路は、現状の native binary writer が未対応であり、
`default` では `BridgeRustBackend` に落ちることを fail-fast 契約として固定する。

## Blocker
1. native lane は `const-return` 系 shape 拡張（P9-min1）までは吸収済み。
2. loop + extern call（WebCanvas/Console 呼び出し）を持つ shape は native emit が未実装。
3. route-trace は `plan=bridge-rust-backend` が期待値。

## Acceptance
1. `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p9_min2_loop_canvas_primer_bridge_lock_vm.sh`
2. `tools/checks/dev_gate.sh portability`
