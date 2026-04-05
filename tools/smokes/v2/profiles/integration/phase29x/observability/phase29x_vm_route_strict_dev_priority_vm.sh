#!/bin/bash
# Phase 29x X20: strict/dev vm-hako priority smoke
#
# Contract pin:
# 1) strict/dev gate (`NYASH_JOINIR_STRICT=1`) makes `--backend vm` choose `lane=vm-hako-reference`.
# 2) compat lane remains explicit-only (`NYASH_VM_USE_FALLBACK=1`) via `vm-compat-fallback`.
# 3) `[vm-route/pre-dispatch]` is emitted and legacy tag form is absent.

set -euo pipefail

source "$(dirname "$0")/../../../../lib/test_runner.sh"
require_env || exit 2

VM_INPUT="$NYASH_ROOT/apps/tests/phase285_leak_report.hako"
VM_HAKO_INPUT="$NYASH_ROOT/apps/tests/phase29z_vm_hako_s0_reject_compare_ne_min.hako"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"

if [ ! -f "$VM_INPUT" ]; then
    test_fail "phase29x_vm_route_strict_dev_priority_vm: fixture missing: $VM_INPUT"
    exit 1
fi
if [ ! -f "$VM_HAKO_INPUT" ]; then
    test_fail "phase29x_vm_route_strict_dev_priority_vm: fixture missing: $VM_HAKO_INPUT"
    exit 1
fi

set +e
OUT_STRICT=$(NYASH_VM_ROUTE_TRACE=1 NYASH_JOINIR_STRICT=1 timeout "$RUN_TIMEOUT_SECS" "$NYASH_BIN" --backend vm "$VM_HAKO_INPUT" 2>&1)
RC_STRICT=$?
set -e
if [ "$RC_STRICT" -eq 124 ]; then
    test_fail "phase29x_vm_route_strict_dev_priority_vm: strict vm run timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi
if [ "$RC_STRICT" -eq 0 ]; then
    echo "[INFO] strict vm output:"
    echo "$OUT_STRICT" | head -n 120 || true
    test_fail "phase29x_vm_route_strict_dev_priority_vm: expected non-zero (vm-hako path)"
    exit 1
fi
if ! echo "$OUT_STRICT" | grep -q "^\[vm-route/pre-dispatch\] backend=vm file=.*phase29z_vm_hako_s0_reject_compare_ne_min.hako$"; then
    echo "[INFO] strict vm output:"
    echo "$OUT_STRICT" | head -n 120 || true
    test_fail "phase29x_vm_route_strict_dev_priority_vm: missing strict pre-dispatch tag"
    exit 1
fi
if ! echo "$OUT_STRICT" | grep -q "^\[vm-route/select\] backend=vm lane=vm-hako-reference reason=strict-dev-prefer$"; then
    echo "[INFO] strict vm output:"
    echo "$OUT_STRICT" | head -n 120 || true
    test_fail "phase29x_vm_route_strict_dev_priority_vm: missing strict-dev vm-hako tag"
    exit 1
fi
if echo "$OUT_STRICT" | grep -q "^\[vm-route\] pre-dispatch"; then
    echo "[INFO] strict vm output:"
    echo "$OUT_STRICT" | head -n 120 || true
    test_fail "phase29x_vm_route_strict_dev_priority_vm: legacy pre-dispatch tag detected (strict)"
    exit 1
fi

set +e
OUT_FALLBACK=$(NYASH_VM_ROUTE_TRACE=1 NYASH_JOINIR_STRICT=1 NYASH_VM_USE_FALLBACK=1 NYASH_DISABLE_PLUGINS=1 timeout "$RUN_TIMEOUT_SECS" "$NYASH_BIN" --backend vm "$VM_INPUT" 2>&1)
RC_FALLBACK=$?
set -e
if [ "$RC_FALLBACK" -eq 124 ]; then
    test_fail "phase29x_vm_route_strict_dev_priority_vm: strict+fallback vm run timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi
if [ "$RC_FALLBACK" -ne 0 ]; then
    echo "[INFO] strict+fallback vm output:"
    echo "$OUT_FALLBACK" | head -n 120 || true
    test_fail "phase29x_vm_route_strict_dev_priority_vm: strict+fallback vm run failed (rc=$RC_FALLBACK)"
    exit 1
fi
if ! echo "$OUT_FALLBACK" | grep -q "^\[vm-route/pre-dispatch\] backend=vm file=.*phase285_leak_report.hako$"; then
    echo "[INFO] strict+fallback vm output:"
    echo "$OUT_FALLBACK" | head -n 120 || true
    test_fail "phase29x_vm_route_strict_dev_priority_vm: missing pre-dispatch tag (strict+fallback)"
    exit 1
fi
if ! echo "$OUT_FALLBACK" | grep -q "^\[vm-route/select\] backend=vm lane=vm-compat-fallback reason=env:NYASH_VM_USE_FALLBACK=1$"; then
    echo "[INFO] strict+fallback vm output:"
    echo "$OUT_FALLBACK" | head -n 120 || true
    test_fail "phase29x_vm_route_strict_dev_priority_vm: missing explicit compat fallback tag"
    exit 1
fi
if echo "$OUT_FALLBACK" | grep -q "^\[vm-route\] pre-dispatch"; then
    echo "[INFO] strict+fallback vm output:"
    echo "$OUT_FALLBACK" | head -n 120 || true
    test_fail "phase29x_vm_route_strict_dev_priority_vm: legacy pre-dispatch tag detected (strict+fallback)"
    exit 1
fi

test_pass "phase29x_vm_route_strict_dev_priority_vm: PASS (strict/dev prefers vm-hako-reference; compat explicit)"
