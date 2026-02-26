#!/bin/bash
# Phase 29x X39: Rust lane opt-in only smoke
#
# Contract pin:
# 1) daily default lane remains LLVM-only (`phase29x_llvm_only_daily_gate.sh`).
# 2) Rust compatibility lane is callable only through `tools/compat/*`.
# 3) Rust lane requires explicit opt-in (`PHASE29X_ALLOW_RUST_LANE=1`).

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

COMPAT_GATE="$NYASH_ROOT/tools/compat/phase29x_rust_lane_gate.sh"
README_COMPAT="$NYASH_ROOT/tools/compat/README.md"
CHECKLIST="$NYASH_ROOT/docs/development/current/main/phases/phase-29x/29x-90-integration-checklist.md"
PHASE_README="$NYASH_ROOT/docs/development/current/main/phases/phase-29x/README.md"

for f in "$COMPAT_GATE" "$README_COMPAT" "$CHECKLIST" "$PHASE_README"; do
    if [ ! -f "$f" ]; then
        test_fail "phase29x_rust_lane_optin_only: required file missing: $f"
        exit 1
    fi
done

if ! rg -q "phase29x_llvm_only_daily_gate\\.sh" "$CHECKLIST"; then
    test_fail "phase29x_rust_lane_optin_only: checklist daily default is not LLVM-only gate"
    exit 1
fi

if ! rg -q "phase29x_llvm_only_daily_gate\\.sh" "$PHASE_README"; then
    test_fail "phase29x_rust_lane_optin_only: README milestone default is not LLVM-only gate"
    exit 1
fi

set +e
OUT_NO_OPTIN=$("$COMPAT_GATE" --dry-run 2>&1)
RC_NO_OPTIN=$?
set -e

if [ "$RC_NO_OPTIN" -eq 0 ]; then
    echo "[INFO] compat gate output (without opt-in):"
    echo "$OUT_NO_OPTIN" | tail -n 80 || true
    test_fail "phase29x_rust_lane_optin_only: compat gate unexpectedly succeeded without opt-in"
    exit 1
fi

if ! echo "$OUT_NO_OPTIN" | rg -q "^\[compat/optin-required\]"; then
    echo "[INFO] compat gate output (without opt-in):"
    echo "$OUT_NO_OPTIN" | tail -n 80 || true
    test_fail "phase29x_rust_lane_optin_only: missing opt-in required tag"
    exit 1
fi

set +e
OUT_OPTIN=$(PHASE29X_ALLOW_RUST_LANE=1 "$COMPAT_GATE" --dry-run 2>&1)
RC_OPTIN=$?
set -e

if [ "$RC_OPTIN" -ne 0 ]; then
    echo "[INFO] compat gate output (with opt-in):"
    echo "$OUT_OPTIN" | tail -n 80 || true
    test_fail "phase29x_rust_lane_optin_only: compat gate failed with explicit opt-in"
    exit 1
fi

if ! echo "$OUT_OPTIN" | rg -q "^\[compat/optin\] rust-lane gate dry-run"; then
    echo "[INFO] compat gate output (with opt-in):"
    echo "$OUT_OPTIN" | tail -n 80 || true
    test_fail "phase29x_rust_lane_optin_only: missing opt-in dry-run tag"
    exit 1
fi

test_pass "phase29x_rust_lane_optin_only: PASS (rust lane isolated to tools/compat + explicit opt-in)"
