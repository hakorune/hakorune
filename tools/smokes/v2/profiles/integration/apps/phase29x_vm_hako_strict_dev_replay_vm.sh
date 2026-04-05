#!/bin/bash
# Phase 29x X49: vm-hako strict/dev replay gate
#
# Contract pin:
# 1) strict/dev + `--backend vm` chooses `lane=vm-hako-reference` without explicit route pin env override.
# 2) supported vm-hako fixture replays with `rc=42`.
# 3) unsupported fixture still enters `vm-hako-reference` first and exits non-zero.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"
SUPPORTED_FIXTURE="$NYASH_ROOT/apps/tests/phase29z_vm_hako_s0_const_add_return_min.hako"
REJECT_FIXTURE="$NYASH_ROOT/apps/tests/phase29z_vm_hako_s0_reject_compare_ne_min.hako"

if [ ! -f "$SUPPORTED_FIXTURE" ]; then
    test_fail "phase29x_vm_hako_strict_dev_replay_vm: fixture missing: $SUPPORTED_FIXTURE"
    exit 1
fi
if [ ! -f "$REJECT_FIXTURE" ]; then
    test_fail "phase29x_vm_hako_strict_dev_replay_vm: fixture missing: $REJECT_FIXTURE"
    exit 1
fi

run_vm_strict_dev() {
    local input="$1"
    set +e
    NYASH_VM_ROUTE_TRACE=1 NYASH_JOINIR_STRICT=1 NYASH_DISABLE_PLUGINS=1 \
        timeout "$RUN_TIMEOUT_SECS" "$NYASH_BIN" --backend vm "$input" 2>&1
    local rc=$?
    set -e
    return "$rc"
}

extract_first_line() {
    local prefix="$1"
    local text="$2"
    printf "%s\n" "$text" | rg "^${prefix}" | head -n 1 || true
}

set +e
OUT_SUPPORTED="$(run_vm_strict_dev "$SUPPORTED_FIXTURE")"
RC_SUPPORTED=$?
set -e

if [ "$RC_SUPPORTED" -eq 124 ]; then
    test_fail "phase29x_vm_hako_strict_dev_replay_vm: supported fixture timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi
if [ "$RC_SUPPORTED" -ne 42 ]; then
    echo "[INFO] supported output:"
    echo "$OUT_SUPPORTED" | tail -n 120 || true
    test_fail "phase29x_vm_hako_strict_dev_replay_vm: supported fixture rc mismatch (expected=42 got=$RC_SUPPORTED)"
    exit 1
fi

FIRST_SELECT_SUPPORTED="$(extract_first_line "\\[vm-route/select\\]" "$OUT_SUPPORTED")"
if [ "$FIRST_SELECT_SUPPORTED" != "[vm-route/select] backend=vm lane=vm-hako-reference reason=strict-dev-prefer" ]; then
    echo "[INFO] supported output:"
    echo "$OUT_SUPPORTED" | tail -n 120 || true
    test_fail "phase29x_vm_hako_strict_dev_replay_vm: first vm-route/select is not vm-hako strict-dev-prefer"
    exit 1
fi

FIRST_DERUST_SUPPORTED="$(extract_first_line "\\[derust-route/select\\]" "$OUT_SUPPORTED")"
if [ "$FIRST_DERUST_SUPPORTED" != "[derust-route/select] backend=vm lane=vm-hako-reference source=hako-skeleton reason=strict-dev-prefer" ]; then
    echo "[INFO] supported output:"
    echo "$OUT_SUPPORTED" | tail -n 120 || true
    test_fail "phase29x_vm_hako_strict_dev_replay_vm: first derust-route/select is not vm-hako hako-skeleton"
    exit 1
fi

set +e
OUT_REJECT="$(run_vm_strict_dev "$REJECT_FIXTURE")"
RC_REJECT=$?
set -e

if [ "$RC_REJECT" -eq 124 ]; then
    test_fail "phase29x_vm_hako_strict_dev_replay_vm: reject fixture timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi
if [ "$RC_REJECT" -eq 0 ]; then
    echo "[INFO] reject output:"
    echo "$OUT_REJECT" | tail -n 120 || true
    test_fail "phase29x_vm_hako_strict_dev_replay_vm: reject fixture expected non-zero"
    exit 1
fi

FIRST_SELECT_REJECT="$(extract_first_line "\\[vm-route/select\\]" "$OUT_REJECT")"
if [ "$FIRST_SELECT_REJECT" != "[vm-route/select] backend=vm lane=vm-hako-reference reason=strict-dev-prefer" ]; then
    echo "[INFO] reject output:"
    echo "$OUT_REJECT" | tail -n 120 || true
    test_fail "phase29x_vm_hako_strict_dev_replay_vm: reject first vm-route/select is not vm-hako strict-dev-prefer"
    exit 1
fi

FIRST_DERUST_REJECT="$(extract_first_line "\\[derust-route/select\\]" "$OUT_REJECT")"
if [ "$FIRST_DERUST_REJECT" != "[derust-route/select] backend=vm lane=vm-hako-reference source=hako-skeleton reason=strict-dev-prefer" ]; then
    echo "[INFO] reject output:"
    echo "$OUT_REJECT" | tail -n 120 || true
    test_fail "phase29x_vm_hako_strict_dev_replay_vm: reject first derust-route/select is not vm-hako-reference hako-skeleton"
    exit 1
fi

test_pass "phase29x_vm_hako_strict_dev_replay_vm: PASS (strict/dev vm-hako-reference replay locked)"
