#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="joinir-located-legacy-retire-guard"
SESSION="$ROOT_DIR/src/mir/builder/located_legacy_lowering.rs"
BUILDER="$ROOT_DIR/src/mir/builder.rs"
CARRIER="$ROOT_DIR/src/mir/callable_result_representation/located_legacy.rs"
LEDGER="$ROOT_DIR/src/mir/callable_result_representation/caller_ledger.rs"
CLAIM_BATCH="$ROOT_DIR/src/mir/callable_result_representation/loop_claim_batch.rs"

fail() {
  printf '[%s] ERROR: %s\n' "$TAG" "$1" >&2
  exit 1
}

for path in "$SESSION" "$BUILDER" "$CARRIER" "$LEDGER" "$CLAIM_BATCH"; do
  [[ -f "$path" ]] || fail "missing file: ${path#"$ROOT_DIR/"}"
done

for fixture in \
  "$ROOT_DIR/src/mir/callable_result_representation/tests/located_legacy_lowering.rs" \
  "$ROOT_DIR/src/mir/callable_result_representation/tests/located_short_circuit_lowering.rs"; do
  [[ -f "$fixture" ]] || fail "missing retained test oracle: ${fixture#"$ROOT_DIR/"}"
done

rg -n -F 'LocatedLegacyBodySuffixV1' "$CARRIER" "$LEDGER" >/dev/null || \
  fail "located source carrier/ledger boundary drift"
rg -n -F 'CallableResultLoopClaimBatch' "$CLAIM_BATCH" >/dev/null || \
  fail "loop-claim carrier boundary drift"

# The session is retained only long enough to move its test oracle. There must
# be no constructor/verify call from a production root or a second module.
verify_refs="$(rg -n -F 'LocatedLegacyLoweringSessionV1::verify(' "$ROOT_DIR/src" --glob '*.rs' || true)"
total="$(printf '%s\n' "$verify_refs" | sed '/^$/d' | wc -l | tr -d '[:space:]')"
[[ "$total" == "48" ]] || fail "verify-call census drift: $total != 48"
if printf '%s\n' "$verify_refs" | awk -F: '
  $1 !~ /_tests?\.rs$/ && $1 !~ /\/tests\// && $1 !~ /located_legacy_lowering\.rs$/ { found = 1 }
  END { exit found }
'; then
  :
else
  fail "LocatedLegacyLoweringSessionV1::verify escaped test/session boundary"
fi

all_refs="$(rg -n -F 'LocatedLegacyLoweringSessionV1' "$ROOT_DIR/src" --glob '*.rs' || true)"
if printf '%s\n' "$all_refs" | awk -F: '
  $1 !~ /_tests?\.rs$/ && $1 !~ /\/tests\// &&
  $1 !~ /located_legacy_lowering\.rs$/ &&
  $1 !~ /located_legacy_(assignment|if|return)\.rs$/ &&
  $1 !~ /\/builder\.rs$/ { found = 1 }
  END { exit found }
'; then
  :
else
  fail "LocatedLegacyLoweringSessionV1 has an unexpected non-test production reference"
fi

# The only non-test symbols are the disconnected session and its internal
# adapters. Keep the 800-line rule explicit while the retirement task is open.
for path in \
  "$SESSION" \
  "$ROOT_DIR/src/mir/builder/located_legacy_assignment.rs" \
  "$ROOT_DIR/src/mir/builder/located_legacy_if.rs" \
  "$ROOT_DIR/src/mir/builder/located_legacy_return.rs"; do
  [[ -f "$path" ]] || fail "missing located legacy component: ${path#"$ROOT_DIR/"}"
  lines="$(wc -l < "$path" | tr -d '[:space:]')"
  (( lines < 800 )) || fail "800-line boundary exceeded: ${path#"$ROOT_DIR/"}=$lines"
done

while IFS= read -r path; do
  lines="$(wc -l < "$path" | tr -d '[:space:]')"
  (( lines < 800 )) || fail "800-line boundary exceeded: ${path#"$ROOT_DIR/"}=$lines"
done < <(find "$ROOT_DIR/src/mir/builder" -maxdepth 1 -type f -name 'located_legacy_*.rs' -print)

rg -n -F 'mod located_legacy_lowering;' "$BUILDER" >/dev/null || \
  fail "builder module registration drift"
rg -n -F 'pub(in crate::mir) use located_legacy_lowering::' "$BUILDER" >/dev/null || \
  fail "builder test-oracle reexport drift"

printf '[%s] ok: 48 verify calls are test-only, no production root caller, and located components stay below 800 lines\n' "$TAG"
