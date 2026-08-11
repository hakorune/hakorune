#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="a-prime-i64-parameter-lane"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

RUST_RECEIPT="$ROOT_DIR/src/mir/a_prime_i64_physical_receipt.rs"
PY_LOADER="$ROOT_DIR/src/llvm_py/builders/a_prime_i64_capability.py"
RUST_TESTS="$RUST_RECEIPT"
PY_TESTS="$ROOT_DIR/src/llvm_py/tests/test_a_prime_i64_capability.py"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$RUST_RECEIPT" "$PY_LOADER" "$PY_TESTS"

guard_expect_fixed_in_file "$TAG" '"pos" => 1' "$RUST_RECEIPT" \
  "Rust must bind pos to the second formal parameter"
guard_expect_fixed_in_file "$TAG" '"end" => 2' "$RUST_RECEIPT" \
  "Rust must bind end to the third formal parameter"
guard_expect_fixed_in_file "$TAG" 'expected_indices = {"pos": 1, "end": 2}' "$PY_LOADER" \
  "Python must bind roles to the canonical formal ordinals"
guard_expect_fixed_in_file "$TAG" '"params": [10, 11, 12, 13]' "$PY_TESTS" \
  "the fixture must retain src/pos/end/pred_chars order"
guard_expect_fixed_in_file "$TAG" 'rejects_swapped_parameter_role_indices' "$RUST_TESTS" \
  "Rust must reject swapped role/ordinal rows"
guard_expect_fixed_in_file "$TAG" 'test_parameter_role_index_mismatch_is_rejected' "$PY_TESTS" \
  "Python must reject swapped role/ordinal rows"
guard_expect_fixed_in_file "$TAG" '"substring/2"' "$RUST_RECEIPT" \
  "Rust must keep the source CallSlot arity for substring"
guard_expect_fixed_in_file "$TAG" '"indexOf/1"' "$RUST_RECEIPT" \
  "Rust must keep the source CallSlot arity for indexOf"
guard_expect_fixed_in_file "$TAG" 'expected_result_lane' "$PY_LOADER" \
  "Python must validate the role-specific result lane"
guard_expect_fixed_in_file "$TAG" 'result_lane: APrimeI64LaneV1::ImmediateI64' "$RUST_TESTS" \
  "Rust must model indexOf as an ImmediateI64 result"
guard_expect_fixed_in_file "$TAG" '"result_lane": "immediate_i64"' "$PY_TESTS" \
  "Python must model indexOf as an ImmediateI64 result"
guard_expect_fixed_in_file "$TAG" 'CallResultLaneMismatch' "$RUST_TESTS" \
  "Rust must reject a stale I7 opaque result lane"

for file in "$RUST_RECEIPT" "$PY_LOADER" "$PY_TESTS"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "A-prime parameter source reached hard 800-line boundary: ${file#"$ROOT_DIR/"} has $lines"
  fi
done

echo "[$TAG] ok"
