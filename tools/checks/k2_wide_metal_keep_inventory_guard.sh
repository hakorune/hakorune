#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

INVENTORY_DOC="docs/development/current/main/design/atomic-tls-gc-truthful-native-seam-inventory.md"
ALLOC_DOC="docs/development/current/main/design/hako-alloc-policy-state-contract-ssot.md"
PHASE_PLAN_DOC="docs/development/current/main/design/kernel-implementation-phase-plan-ssot.md"
FINAL_METAL_DOC="docs/development/current/main/design/final-metal-split-ssot.md"
STAGE2_ALLOC_DOC="docs/development/current/main/design/stage2-selfhost-and-hako-alloc-ssot.md"
AXIS_DOC="docs/development/current/main/design/execution-lanes-and-axis-separation-ssot.md"

echo "[k2-wide-metal-keep-inventory] running truthful seam inventory / boundary-shrink pack"
echo "[k2-wide-metal-keep-inventory] --- runtime v0 abi slice lock ---"
bash tools/checks/phase29cc_runtime_v0_abi_slice_guard.sh

echo "[k2-wide-metal-keep-inventory] --- doc/inventory lock ---"
rg -F -q 'hako.atomic.fence_i64' "$INVENTORY_DOC"
rg -F -q 'hako.tls.last_error_text_h' "$INVENTORY_DOC"
rg -F -q 'hako.gc.write_barrier_i64' "$INVENTORY_DOC"
rg -F -q 'hako.osvm.reserve_bytes_i64' "$INVENTORY_DOC"
rg -F -q 'current live implementation row is `GC trigger threshold policy`' "$ALLOC_DOC"
rg -F -q 'There is no third live allocator row yet' "$ALLOC_DOC"
rg -F -q 'metal keep review as truthful seam inventory + boundary-shrink planning' "$PHASE_PLAN_DOC"
rg -F -q '`hako.osvm` is the capability facade only; raw OS VM syscall glue remains native keep' "$FINAL_METAL_DOC"
rg -F -q '`hako.osvm` names the `.hako` capability surface; the reserve/commit/decommit rows are already landed, and final OS VM syscall glue and platform-specific body stay native keep until a later dedicated retirement wave says otherwise' "$STAGE2_ALLOC_DOC"
rg -F -q 'final OS VM syscall glue, TLS/atomic platform glue, and other platform-specific leaf bodies remain native keep' "$AXIS_DOC"

echo "[k2-wide-metal-keep-inventory] ok"
