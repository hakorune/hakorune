---
Status: Done
Decision: accepted-but-blocked
Date: 2026-02-27
Scope: WSM-P9-min3 として canvas_advanced（nested loop + extern群）経路の bridge 残存を固定し、段階移植境界を明示する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-191-wsm-p9-min2-loop-canvas-primer-bridge-lock-ssot.md
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p9_min3_canvas_advanced_bridge_lock_vm.sh
---

# 29cc-192 WSM-P9-min3 Canvas Advanced Bridge Lock

## Purpose
`canvas_advanced` fixture は nested loop + 条件分岐 + 複数 extern 呼び出しを含み、
現状は `BridgeRustBackend`（route: `bridge-rust-backend`）維持が必要であることを lock する。

## Blocker
1. native lane に loop/branch/call を直接下ろす binary writer v2 が未実装。
2. 現在の正しい挙動は bridge fallback である。

## Acceptance
1. `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p9_min3_canvas_advanced_bridge_lock_vm.sh`
2. `tools/checks/dev_gate.sh portability`
