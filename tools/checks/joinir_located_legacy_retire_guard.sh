#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="joinir-located-legacy-retire-guard"
BUILDER="$ROOT_DIR/src/mir/builder.rs"
CARRIER="$ROOT_DIR/src/mir/callable_result_representation/located_legacy.rs"
LEDGER="$ROOT_DIR/src/mir/callable_result_representation/caller_ledger.rs"
CLAIM_BATCH="$ROOT_DIR/src/mir/callable_result_representation/loop_claim_batch.rs"

fail() {
  printf '[%s] ERROR: %s\n' "$TAG" "$1" >&2
  exit 1
}

for path in "$BUILDER" "$CARRIER" "$LEDGER" "$CLAIM_BATCH"; do
  [[ -f "$path" ]] || fail "missing retained file: ${path#"$ROOT_DIR/"}"
done

for deleted in \
  "$ROOT_DIR/src/mir/builder/located_legacy_lowering.rs" \
  "$ROOT_DIR/src/mir/builder/located_legacy_assignment.rs" \
  "$ROOT_DIR/src/mir/builder/located_legacy_if.rs" \
  "$ROOT_DIR/src/mir/builder/located_legacy_return.rs" \
  "$ROOT_DIR/src/mir/callable_result_representation/tests/located_legacy_lowering.rs" \
  "$ROOT_DIR/src/mir/callable_result_representation/tests/located_short_circuit_lowering.rs"; do
  [[ ! -e "$deleted" ]] || fail "retired located component still exists: ${deleted#"$ROOT_DIR/"}"
done

if rg -n -F 'LocatedLegacyLoweringSessionV1' "$ROOT_DIR/src" --glob '*.rs' >/dev/null 2>&1; then
  fail "retired LocatedLegacyLoweringSessionV1 symbol returned"
fi
if rg -n -F 'mod located_legacy_lowering;' "$BUILDER" >/dev/null 2>&1 || \
   rg -n -F 'use located_legacy_lowering::' "$BUILDER" >/dev/null 2>&1; then
  fail "retired builder module/re-export returned"
fi
if rg -n -F 'mod located_legacy_lowering;' "$ROOT_DIR/src/mir/callable_result_representation/tests/mod.rs" >/dev/null 2>&1 || \
   rg -n -F 'mod located_short_circuit_lowering;' "$ROOT_DIR/src/mir/callable_result_representation/tests/mod.rs" >/dev/null 2>&1; then
  fail "retired callable-result test module returned"
fi

# The source/claim carriers are intentionally retained for future canonical
# ingress; deletion of those semantic contracts belongs to a separate row.
rg -n -F 'LocatedLegacyBodySuffixV1' "$CARRIER" "$LEDGER" >/dev/null || \
  fail "located source carrier/ledger boundary drift"
rg -n -F 'CallableResultLoopClaimBatch' "$CLAIM_BATCH" >/dev/null || \
  fail "loop-claim carrier boundary drift"

for path in "$BUILDER" "$CARRIER" "$LEDGER" "$CLAIM_BATCH"; do
  lines="$(wc -l < "$path" | tr -d '[:space:]')"
  (( lines < 800 )) || fail "800-line boundary exceeded: ${path#"$ROOT_DIR/"}=$lines"
done

printf '[%s] ok: located legacy lowering is retired, carriers remain, and retained files stay below 800 lines\n' "$TAG"
