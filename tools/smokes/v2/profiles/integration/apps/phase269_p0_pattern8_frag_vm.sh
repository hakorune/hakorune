#!/bin/bash
set -e
cd "$(dirname "$0")/../../../../../.."
HAKORUNE_BIN="${HAKORUNE_BIN:-./target/release/hakorune}"

# Phase 269 P1: Pattern8 Frag lowering test
set +e
$HAKORUNE_BIN --backend vm apps/tests/phase269_p0_pattern8_frag_min.hako > /tmp/phase269_out.txt 2>&1
EXIT_CODE=$?
set -e

if [ $EXIT_CODE -eq 7 ]; then
    echo "[PASS] phase269_p0_pattern8_frag_vm"
    exit 0
else
    echo "[FAIL] phase269_p0_pattern8_frag_vm: expected exit 7, got $EXIT_CODE"
    cat /tmp/phase269_out.txt
    exit 1
fi
