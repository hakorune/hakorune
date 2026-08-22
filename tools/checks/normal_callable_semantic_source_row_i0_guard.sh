#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="normal-callable-semantic-source-row-i0"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

LOAN_PORT="$ROOT_DIR/src/mir/builder/normal_callable_semantic_loan_port.rs"
LOWERING="$ROOT_DIR/src/mir/builder/normal_cataloged_box_method_lowering.rs"
LIFECYCLE_TEST="$ROOT_DIR/src/mir/builder/normal_default_root_catalog_lifecycle_tests.rs"
CARD="$ROOT_DIR/docs/development/current/main/investigations/mir-callable-semantic-nested-if-source-row-d0-2026-08-22.md"
STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$LOAN_PORT" "$LOWERING" "$LIFECYCLE_TEST" "$CARD" "$STATE"

guard_expect_fixed_in_file "$TAG" \
  "with_selected_source_scope(inner, lineage, selected, |inner, transport|" \
  "$LOAN_PORT" \
  "ordinary cataloged-static branch must retain the function-root transport"
guard_expect_fixed_in_file "$TAG" \
  "lower_normal_cataloged_static_box_method_with_source_v1" \
  "$LOAN_PORT" \
  "outside callable rows must retain the named source-aware seam"
if rg -F -q -- "lower_normal_cataloged_static_box_method_with_signature_v1" "$LOAN_PORT"; then
  guard_fail "$TAG" "old source-dropping signature seam must remain retired"
fi
if rg -F -q -- "lower_normal_cataloged_static_box_method_with_signature_and_source_v1" "$LOAN_PORT"; then
  guard_fail "$TAG" "resolved legacy signature seam must remain retired"
fi
if rg -F -q -- "|inner, _transport|" "$LOAN_PORT"; then
  guard_fail "$TAG" "ordinary static branch must not discard its transport"
fi

guard_expect_fixed_in_file "$TAG" \
  "pub(in crate::mir::builder) fn lower_normal_cataloged_static_box_method_with_source_v1" \
  "$LOWERING" \
  "outside source-aware lowering owner is missing"
guard_expect_fixed_in_file "$TAG" \
  "self.with_source_transport_v1(source, |port, ()|" \
  "$LOWERING" \
  "outside function session must run under the existing function-root source"
guard_expect_fixed_in_file "$TAG" \
  "source: RawInvocationSourceTransportV1<()>" \
  "$LOWERING" \
  "source transport must be required, not optional"

guard_expect_fixed_in_file "$TAG" \
  "fn parser_scan_package_passes_callable_source_handoff_without_fallback" \
  "$LIFECYCLE_TEST" \
  "focused parser-scan handoff evidence is missing"
guard_expect_fixed_in_file "$TAG" \
  "static-result-ingress/target-unavailable" \
  "$LIFECYCLE_TEST" \
  "focused test must record the next existing blocker"
guard_expect_fixed_in_file "$TAG" \
  "callable-semantic-lowering/missing-variable-site" \
  "$LIFECYCLE_TEST" \
  "focused test must assert the old source-row blocker is gone"

guard_expect_fixed_in_file "$TAG" \
  "ACCEPTED-SAFE-I0" \
  "$CARD" \
  "worker acceptance must be recorded in the active card"
guard_expect_fixed_in_file "$TAG" \
  "MIR-CALLABLE-SEMANTIC-NESTED-IF-SOURCE-ROW-I0" \
  "$CARD" \
  "bounded I0 task must be recorded in the active card"
if rg -F -q -- 'current_execution_row = "MIR-CALLABLE-SEMANTIC-NESTED-IF-SOURCE-ROW-I0"' "$STATE"; then
  : # active I0 state
elif rg -F -q -- 'current_execution_row = "CALLABLE-CANONICAL-TRIVIAL-ROW-I0"' "$STATE"; then
  : # the resolved-binding successor I0 is active
elif rg -F -q -- 'current_execution_row = "CALLABLE-RESOLVED-BINDING-AUTHORITY-HANDOFF-D0"' "$STATE" \
  && rg -F -q -- 'latest_card = "mir-callable-resolved-binding-authority-handoff-d0"' "$STATE" \
  && rg -F -q -- '95b65e4081' "$STATE"; then
  : # I0 is closed and the next design stop is selected
else
  guard_fail "$TAG" "CURRENT_STATE must select active I0 or its recorded next design stop"
fi

for file in "$LOAN_PORT" "$LOWERING"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 760 )); then
    guard_fail "$TAG" "production source reached the 760-line split boundary: ${file#"$ROOT_DIR/"}=$lines"
  fi
done

echo "[$TAG] ok"
