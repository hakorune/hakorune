#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mirbuilder-physical-receiver-identity-r0-guard"
STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
TEST="$ROOT_DIR/lang/c-abi/tests/published_lifecycle_v4_receiver_identity_test.c"
FLOW="$ROOT_DIR/lang/c-abi/shims/published_mir/hako_llvmc_ffi_lifecycle_v4_indexed_flow.inc"
ADMISSION="$ROOT_DIR/lang/c-abi/shims/published_mir/hako_llvmc_ffi_lifecycle_v4_admission.inc"

fail() {
  echo "[$TAG] result_class=current-change failure status=fail: $*" >&2
  exit 1
}

for file in "$STATE" "$TEST" "$FLOW" "$ADMISSION"; do
  [[ -f "$file" ]] || fail "required owner missing: ${file#$ROOT_DIR/}"
done

grep -Fq 'current_execution_row = "MIRBUILDER-PHYSICAL-CALL-RECEIVER-IDENTITY-COVERAGE-R0"' "$STATE" \
  || fail "receiver identity R0 is not the selected row"
grep -Fq 'json_template, 8u' "$TEST" || fail "valid receiver positive is missing"
grep -Fq 'json_template, 7u' "$TEST" || fail "receiver-only mutation is missing"
grep -Fq 'assert(rc == 0)' "$TEST" || fail "positive receiver compile assertion is missing"
grep -Fq 'receiver-object-mismatch' "$TEST" || fail "named receiver mismatch assertion is missing"
grep -Fq 'LV4_FLOW_RECEIVER_MISMATCH' "$FLOW" || fail "receiver mismatch flow code is missing"
grep -Fq 'receiver-object-mismatch' "$ADMISSION" || fail "receiver mismatch reason is not propagated"
grep -Fq 'home_release' "$TEST" || fail "fault cleanup base is missing"

for file in "$TEST" "$FLOW" "$ADMISSION"; do
  lines="$(wc -l <"$file")"
  (( lines < 800 )) || fail "owner crossed the 800-line hard stop: ${file#$ROOT_DIR/}=$lines"
done

echo "[$TAG] result_class=current-change failure status=pass row=MIRBUILDER-PHYSICAL-CALL-RECEIVER-IDENTITY-COVERAGE-R0 positive=present mutation=receiver_object_only named_reject=present"
