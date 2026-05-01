#!/bin/bash
# Phase 29x X35: strict/dev default route cutover to `.hako` route contract
#
# Contract pin:
# 1) strict/dev default (`--backend vm`) emits `[derust-route/select] ... source=hako-skeleton`.
# 2) explicit rust-thin opt-in (`NYASH_VM_HAKO_PREFER_STRICT_DEV=0`) emits `source=rust-thin-explicit`.
# 3) explicit compat fallback in strict/dev still emits `source=hako-skeleton` with `lane=vm-compat-fallback`.

set -euo pipefail

source "$(dirname "$0")/../../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../../lib/vm_route_pin.sh"
require_env || exit 2

VM_INPUT="$NYASH_ROOT/apps/tests/phase285_leak_report.hako"
VM_HAKO_INPUT="$NYASH_ROOT/apps/tests/phase29z_vm_hako_s0_reject_compare_ne_min.hako"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"

if [ ! -f "$VM_INPUT" ]; then
    test_fail "phase29x_derust_strict_default_route_vm: fixture missing: $VM_INPUT"
    exit 1
fi
if [ ! -f "$VM_HAKO_INPUT" ]; then
    test_fail "phase29x_derust_strict_default_route_vm: fixture missing: $VM_HAKO_INPUT"
    exit 1
fi

set +e
OUT_STRICT_DEFAULT=$(NYASH_VM_ROUTE_TRACE=1 NYASH_JOINIR_STRICT=1 timeout "$RUN_TIMEOUT_SECS" "$NYASH_BIN" --backend vm "$VM_HAKO_INPUT" 2>&1)
RC_STRICT_DEFAULT=$?
set -e
if [ "$RC_STRICT_DEFAULT" -eq 124 ]; then
    test_fail "phase29x_derust_strict_default_route_vm: strict default run timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi
if [ "$RC_STRICT_DEFAULT" -eq 0 ]; then
    echo "[INFO] strict default output:"
    echo "$OUT_STRICT_DEFAULT" | head -n 120 || true
    test_fail "phase29x_derust_strict_default_route_vm: expected non-zero for vm-hako strict default fixture"
    exit 1
fi
if ! echo "$OUT_STRICT_DEFAULT" | rg -q "^\[derust-route/select\] backend=vm lane=vm-hako-reference source=hako-skeleton reason=strict-dev-prefer$"; then
    echo "[INFO] strict default output:"
    echo "$OUT_STRICT_DEFAULT" | head -n 120 || true
    test_fail "phase29x_derust_strict_default_route_vm: missing strict default hako route tag"
    exit 1
fi

set +e
OUT_STRICT_RUST_THIN=$(run_with_vm_route_pin env NYASH_VM_ROUTE_TRACE=1 NYASH_JOINIR_STRICT=1 NYASH_DISABLE_PLUGINS=1 timeout "$RUN_TIMEOUT_SECS" "$NYASH_BIN" --backend vm "$VM_INPUT" 2>&1)
RC_STRICT_RUST_THIN=$?
set -e
if [ "$RC_STRICT_RUST_THIN" -eq 124 ]; then
    test_fail "phase29x_derust_strict_default_route_vm: strict explicit rust-thin run timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi
if [ "$RC_STRICT_RUST_THIN" -ne 0 ]; then
    echo "[INFO] strict explicit rust-thin output:"
    echo "$OUT_STRICT_RUST_THIN" | head -n 120 || true
    test_fail "phase29x_derust_strict_default_route_vm: strict explicit rust-thin run failed (rc=$RC_STRICT_RUST_THIN)"
    exit 1
fi
if ! echo "$OUT_STRICT_RUST_THIN" | rg -q "^\[derust-route/select\] backend=vm lane=bootstrap-rust-vm-keep source=rust-thin-explicit reason=explicit-deprecated-bootstrap-keep-not-daily$"; then
    echo "[INFO] strict explicit rust-thin output:"
    echo "$OUT_STRICT_RUST_THIN" | head -n 120 || true
    test_fail "phase29x_derust_strict_default_route_vm: missing explicit rust-thin tag"
    exit 1
fi

set +e
OUT_STRICT_FALLBACK=$(NYASH_VM_ROUTE_TRACE=1 NYASH_JOINIR_STRICT=1 NYASH_VM_USE_FALLBACK=1 NYASH_DISABLE_PLUGINS=1 timeout "$RUN_TIMEOUT_SECS" "$NYASH_BIN" --backend vm "$VM_INPUT" 2>&1)
RC_STRICT_FALLBACK=$?
set -e
if [ "$RC_STRICT_FALLBACK" -eq 124 ]; then
    test_fail "phase29x_derust_strict_default_route_vm: strict fallback run timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi
if [ "$RC_STRICT_FALLBACK" -ne 0 ]; then
    echo "[INFO] strict fallback output:"
    echo "$OUT_STRICT_FALLBACK" | head -n 120 || true
    test_fail "phase29x_derust_strict_default_route_vm: strict fallback run failed (rc=$RC_STRICT_FALLBACK)"
    exit 1
fi
if ! echo "$OUT_STRICT_FALLBACK" | rg -q "^\[derust-route/select\] backend=vm lane=vm-compat-fallback source=hako-skeleton reason=env:NYASH_VM_USE_FALLBACK=1$"; then
    echo "[INFO] strict fallback output:"
    echo "$OUT_STRICT_FALLBACK" | head -n 120 || true
    test_fail "phase29x_derust_strict_default_route_vm: missing strict fallback hako route tag"
    exit 1
fi

test_pass "phase29x_derust_strict_default_route_vm: PASS (strict/dev default hako route + rust-thin explicit opt-in)"
