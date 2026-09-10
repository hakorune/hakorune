#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mirbuilder-physical-negative-test-proof-r0-guard"
STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
RECEIVER="$ROOT_DIR/lang/c-abi/tests/published_lifecycle_v4_receiver_identity_test.c"
MAP="$ROOT_DIR/lang/c-abi/tests/published_map_physical_execution_test.py"
PARSER="$ROOT_DIR/lang/c-abi/shims/published_mir/hako_llvmc_ffi_published_lifecycle_physical_v2.inc"

fail() {
  echo "[$TAG] result_class=current-change failure status=fail: $*" >&2
  exit 1
}

for file in "$STATE" "$RECEIVER" "$MAP" "$PARSER"; do
  [[ -f "$file" ]] || fail "required owner missing: ${file#$ROOT_DIR/}"
done

grep -Fq 'current_execution_row = "MIRBUILDER-PHYSICAL-NEGATIVE-TEST-PROOF-R0"' "$STATE" \
  || fail "negative-test proof row is not selected"
grep -Fq 'json_template, 8u' "$RECEIVER" || fail "valid receiver base is missing"
grep -Fq 'json_template, 7u' "$RECEIVER" || fail "receiver-only mutation is missing"
grep -Fq 'receiver-object-mismatch' "$RECEIVER" || fail "receiver named rejection is missing"
grep -Fq 'assert_named_reject' "$MAP" || fail "reusable named-reject helper is missing"
grep -Fq 'published-lifecycle-physical-parser/abi-layout' "$MAP" \
  || fail "layout mutation does not assert the parser-owned named rejection"
grep -Fq 'hako_physical_fail(err_out, "abi-layout")' "$PARSER" \
  || fail "parser-owned abi-layout terminal is missing"

for file in "$RECEIVER" "$MAP"; do
  lines="$(wc -l <"$file")"
  (( lines < 800 )) || fail "test owner crossed the 800-line hard stop: ${file#$ROOT_DIR/}=$lines"
done

echo "[$TAG] result_class=current-change failure status=pass base=valid_normal_and_fault_cleanup mutation=single_field named_rejects=receiver_and_abi_layout"
