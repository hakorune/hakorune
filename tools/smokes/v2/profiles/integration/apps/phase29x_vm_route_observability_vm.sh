#!/bin/bash
# Phase 29x X19: VM route observability smoke
#
# Contract pin:
# 1) `NYASH_VM_ROUTE_TRACE=1` emits stable `[vm-route/pre-dispatch]` and `[vm-route/select]` tags.
# 2) `backend=vm` lane=vm は `NYASH_VM_HAKO_PREFER_STRICT_DEV=0` 明示時に観測する。
# 3) `backend=vm` + `NYASH_VM_USE_FALLBACK=1` is `lane=compat-fallback`.
# 4) `backend=vm-hako` is `lane=vm-hako`.
# 5) legacy tag form (`[vm-route] pre-dispatch`) is absent.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/vm_route_pin.sh"
require_env || exit 2

VM_INPUT="$NYASH_ROOT/apps/tests/phase285_leak_report.hako"
VM_HAKO_INPUT="$NYASH_ROOT/apps/tests/phase29z_vm_hako_s0_reject_compare_ne_min.hako"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"

if [ ! -f "$VM_INPUT" ]; then
    test_fail "phase29x_vm_route_observability_vm: fixture missing: $VM_INPUT"
    exit 1
fi
if [ ! -f "$VM_HAKO_INPUT" ]; then
    test_fail "phase29x_vm_route_observability_vm: fixture missing: $VM_HAKO_INPUT"
    exit 1
fi

set +e
OUT_DEFAULT=$(run_with_vm_route_pin env NYASH_VM_ROUTE_TRACE=1 NYASH_DISABLE_PLUGINS=1 timeout "$RUN_TIMEOUT_SECS" "$NYASH_BIN" --backend vm "$VM_INPUT" 2>&1)
RC_DEFAULT=$?
set -e
if [ "$RC_DEFAULT" -eq 124 ]; then
    test_fail "phase29x_vm_route_observability_vm: default vm run timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi
if [ "$RC_DEFAULT" -ne 0 ]; then
    echo "[INFO] default vm output:"
    echo "$OUT_DEFAULT" | head -n 120 || true
    test_fail "phase29x_vm_route_observability_vm: default vm run failed (rc=$RC_DEFAULT)"
    exit 1
fi
if ! echo "$OUT_DEFAULT" | grep -q "^\[vm-route/pre-dispatch\] backend=vm file=.*phase285_leak_report.hako$"; then
    echo "[INFO] default vm output:"
    echo "$OUT_DEFAULT" | head -n 120 || true
    test_fail "phase29x_vm_route_observability_vm: missing vm pre-dispatch tag"
    exit 1
fi
if ! echo "$OUT_DEFAULT" | grep -q "^\[vm-route/select\] backend=vm lane=vm reason=default$"; then
    echo "[INFO] default vm output:"
    echo "$OUT_DEFAULT" | head -n 120 || true
    test_fail "phase29x_vm_route_observability_vm: missing vm default route tag"
    exit 1
fi
if echo "$OUT_DEFAULT" | grep -q "^\[vm-route\] pre-dispatch"; then
    echo "[INFO] default vm output:"
    echo "$OUT_DEFAULT" | head -n 120 || true
    test_fail "phase29x_vm_route_observability_vm: legacy pre-dispatch tag detected (default)"
    exit 1
fi

set +e
OUT_FALLBACK=$(NYASH_VM_ROUTE_TRACE=1 NYASH_VM_USE_FALLBACK=1 NYASH_DISABLE_PLUGINS=1 timeout "$RUN_TIMEOUT_SECS" "$NYASH_BIN" --backend vm "$VM_INPUT" 2>&1)
RC_FALLBACK=$?
set -e
if [ "$RC_FALLBACK" -eq 124 ]; then
    test_fail "phase29x_vm_route_observability_vm: fallback vm run timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi
if [ "$RC_FALLBACK" -ne 0 ]; then
    echo "[INFO] fallback vm output:"
    echo "$OUT_FALLBACK" | head -n 120 || true
    test_fail "phase29x_vm_route_observability_vm: fallback vm run failed (rc=$RC_FALLBACK)"
    exit 1
fi
if ! echo "$OUT_FALLBACK" | grep -q "^\[vm-route/pre-dispatch\] backend=vm file=.*phase285_leak_report.hako$"; then
    echo "[INFO] fallback vm output:"
    echo "$OUT_FALLBACK" | head -n 120 || true
    test_fail "phase29x_vm_route_observability_vm: missing vm pre-dispatch tag (fallback)"
    exit 1
fi
if ! echo "$OUT_FALLBACK" | grep -q "^\[vm-route/select\] backend=vm lane=compat-fallback reason=env:NYASH_VM_USE_FALLBACK=1$"; then
    echo "[INFO] fallback vm output:"
    echo "$OUT_FALLBACK" | head -n 120 || true
    test_fail "phase29x_vm_route_observability_vm: missing vm fallback route tag"
    exit 1
fi
if echo "$OUT_FALLBACK" | grep -q "^\[vm-route\] pre-dispatch"; then
    echo "[INFO] fallback vm output:"
    echo "$OUT_FALLBACK" | head -n 120 || true
    test_fail "phase29x_vm_route_observability_vm: legacy pre-dispatch tag detected (fallback)"
    exit 1
fi

set +e
OUT_VM_HAKO=$(NYASH_VM_ROUTE_TRACE=1 timeout "$RUN_TIMEOUT_SECS" "$NYASH_BIN" --backend vm-hako "$VM_HAKO_INPUT" 2>&1)
RC_VM_HAKO=$?
set -e
if [ "$RC_VM_HAKO" -eq 124 ]; then
    test_fail "phase29x_vm_route_observability_vm: vm-hako run timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi
if [ "$RC_VM_HAKO" -eq 0 ]; then
    echo "[INFO] vm-hako output:"
    echo "$OUT_VM_HAKO" | head -n 120 || true
    test_fail "phase29x_vm_route_observability_vm: expected non-zero vm-hako frame run"
    exit 1
fi
if ! echo "$OUT_VM_HAKO" | grep -q "^\[vm-route/pre-dispatch\] backend=vm-hako file=.*phase29z_vm_hako_s0_reject_compare_ne_min.hako$"; then
    echo "[INFO] vm-hako output:"
    echo "$OUT_VM_HAKO" | head -n 120 || true
    test_fail "phase29x_vm_route_observability_vm: missing vm-hako pre-dispatch tag"
    exit 1
fi
if ! echo "$OUT_VM_HAKO" | grep -q "^\[vm-route/select\] backend=vm-hako lane=vm-hako reason=backend:vm-hako$"; then
    echo "[INFO] vm-hako output:"
    echo "$OUT_VM_HAKO" | head -n 120 || true
    test_fail "phase29x_vm_route_observability_vm: missing vm-hako route tag"
    exit 1
fi
if echo "$OUT_VM_HAKO" | grep -q "^\[vm-route\] pre-dispatch"; then
    echo "[INFO] vm-hako output:"
    echo "$OUT_VM_HAKO" | head -n 120 || true
    test_fail "phase29x_vm_route_observability_vm: legacy pre-dispatch tag detected (vm-hako)"
    exit 1
fi

test_pass "phase29x_vm_route_observability_vm: PASS (pre-dispatch/select tags observed, legacy tag absent)"
