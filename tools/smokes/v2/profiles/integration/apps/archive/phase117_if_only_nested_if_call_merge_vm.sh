#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../../../lib/output_validator.sh"

FIXTURE="apps/tests/phase117_if_only_nested_if_call_merge_min.hako"

echo "[phase117_if_only_nested_if_call_merge_vm] Testing nested if + call merge parity (VM)..."

# VM execution with STRICT mode
OUTPUT=$(NYASH_DISABLE_PLUGINS=1 HAKO_JOINIR_STRICT=1 ./target/release/hakorune --backend vm "$FIXTURE" 2>&1) || {
    echo "❌ VM execution failed"
    echo "$OUTPUT"
    exit 1
}

# Validate: expect 3 lines with values 2, 3, 4
validate_numeric_output 3 "2
3
4" "$OUTPUT"

echo "✅ [phase117_if_only_nested_if_call_merge_vm] PASS"
