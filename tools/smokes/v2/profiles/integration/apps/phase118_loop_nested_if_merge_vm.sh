#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../../../lib/output_validator.sh"

FIXTURE="apps/tests/phase118_loop_nested_if_merge_min.hako"

echo "[phase118_loop_nested_if_merge_vm] Testing loop + if-else merge parity (VM)..."

# VM execution with STRICT mode
OUTPUT=$(NYASH_DISABLE_PLUGINS=1 HAKO_JOINIR_STRICT=1 ./target/release/hakorune --backend vm "$FIXTURE" 2>&1) || {
    echo "❌ VM execution failed"
    echo "$OUTPUT"
    exit 1
}

# Validate: expect 1 line with value 2
validate_numeric_output 1 "2" "$OUTPUT"

echo "✅ [phase118_loop_nested_if_merge_vm] PASS"
