#!/bin/bash
# Phase 285 P2: "weak upgrade failure" VM smoke test
# Purpose: Verify weak_to_strong() fails (returns null) after explicit drop
# Uses phase285_p2_weak_upgrade_fail_min.hako
#
# Phase 285 P2.1: FIXED with KeepAlive instruction
# - KeepAlive[drop] in build_assignment() emits before variable overwrite
# - Ensures "alive until overwrite, then dropped" semantics

HAKO_FILE="apps/tests/phase285_p2_weak_upgrade_fail_min.hako"
BACKEND="vm"

# Test expects exit 1 (weak_to_strong returns null after x=null)
./target/release/hakorune --backend "$BACKEND" "$HAKO_FILE" >/dev/null 2>&1
RC=$?

if [[ "$RC" -eq 1 ]]; then
    echo "✅ PASS: phase285_p2_weak_upgrade_fail_vm"
    exit 0
else
    echo "❌ FAIL: phase285_p2_weak_upgrade_fail_vm"
    echo "Expected exit 1 (weak_to_strong null), got $RC"
    echo "---Full output (last 40 lines):---"
    ./target/release/hakorune --backend "$BACKEND" "$HAKO_FILE" 2>&1 | tail -n 40
    exit 1
fi
