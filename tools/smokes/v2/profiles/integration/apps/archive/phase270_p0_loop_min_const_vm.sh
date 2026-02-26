#!/bin/bash
set -e
cd "$(dirname "$0")/../../../../../.."
HAKORUNE_BIN="${HAKORUNE_BIN:-./target/release/hakorune}"

# Phase 270 P0: No env vars, use existing JoinIR route
set +e
$HAKORUNE_BIN --backend vm apps/tests/phase270_p0_loop_min_const.hako > /tmp/phase270_out.txt 2>&1
EXIT_CODE=$?
set -e

if [ $EXIT_CODE -eq 3 ]; then
    echo "[PASS] phase270_p0_loop_min_const_vm"
    exit 0
else
    echo "[FAIL] phase270_p0_loop_min_const_vm: expected exit 3, got $EXIT_CODE"
    cat /tmp/phase270_out.txt
    exit 1
fi
