#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
FIXTURE_DIR="$ROOT/tools/hako_check/tests/fastmem_capability_inventory"
GOOD_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_good.XXXXXX")"
BAD_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_bad.XXXXXX")"
BAD_SAFE_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_bad_safe.XXXXXX")"
BAD_SHAPE_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_bad_shape.XXXXXX")"
trap 'rm -f "$GOOD_OUT" "$BAD_OUT" "$BAD_SAFE_OUT" "$BAD_SHAPE_OUT"' EXIT

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --report "$FIXTURE_DIR/report.kv" \
  --format kv \
  >"$GOOD_OUT"

grep -q '^output_contract=hako-check-fastmem-check-v0$' "$GOOD_OUT"
grep -q '^tool_surface=hako_check_fastmem_check$' "$GOOD_OUT"
grep -q '^failure_count=0$' "$GOOD_OUT"
grep -q '^summary=ok$' "$GOOD_OUT"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/bad_inventory.kv" \
  --format kv \
  >"$BAD_OUT"; then
  echo "[TEST/FAIL] fastmem-check accepted bad inventory" >&2
  exit 1
fi

grep -q '^output_contract=hako-check-fastmem-check-v0$' "$BAD_OUT"
grep -q '^failure_count=6$' "$BAD_OUT"
grep -q '^failure_0_reason=fastmem_escape_count$' "$BAD_OUT"
grep -q '^failure_1_reason=fastmem_contract_runtime_lookup_count$' "$BAD_OUT"
grep -q '^failure_2_reason=fastmem_memop_unclassified_count$' "$BAD_OUT"
grep -q '^failure_3_reason=fastmem_forbidden_call_count$' "$BAD_OUT"
grep -q '^failure_4_reason=type_abi_hot_path_lookup_count$' "$BAD_OUT"
grep -q '^failure_5_reason=provider_dispatch_hot_path$' "$BAD_OUT"
grep -q '^summary=failed$' "$BAD_OUT"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/bad_safe_wrapper_inventory.kv" \
  --format kv \
  >"$BAD_SAFE_OUT"; then
  echo "[TEST/FAIL] fastmem-check accepted bad safe wrapper inventory" >&2
  exit 1
fi

grep -q '^failure_count=2$' "$BAD_SAFE_OUT"
grep -q '^failure_0_reason=safe_capability_wrapper_route$' "$BAD_SAFE_OUT"
grep -q '^failure_1_reason=safe_capability_wrapper_memop_equivalence$' "$BAD_SAFE_OUT"
grep -q '^summary=failed$' "$BAD_SAFE_OUT"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/bad_shape_keeper_inventory.kv" \
  --format kv \
  >"$BAD_SHAPE_OUT"; then
  echo "[TEST/FAIL] fastmem-check accepted bad mimalloc shape keeper" >&2
  exit 1
fi

grep -q '^failure_count=3$' "$BAD_SHAPE_OUT"
grep -q '^failure_0_reason=mimalloc_shape_score$' "$BAD_SHAPE_OUT"
grep -q '^failure_1_reason=mimalloc_coverage_score$' "$BAD_SHAPE_OUT"
grep -q '^failure_2_reason=mimalloc_keeper_eligible$' "$BAD_SHAPE_OUT"
grep -q '^summary=failed$' "$BAD_SHAPE_OUT"

echo "[TEST/OK] fastmem_check"
