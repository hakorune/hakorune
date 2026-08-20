#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="script-static-source-lineage-witness"
TRANSPORT=src/mir/builder/raw_invocation_source_transport.rs
TRANSPORT_TESTS=src/mir/builder/raw_invocation_source_transport_tests.rs
WITNESS_TESTS=src/mir/builder/raw_invocation_source_lineage_witness_tests.rs
CLAIM_TRANSPORT=src/mir/builder/normal_script_direct_static_claim_transport.rs
CLASSIFIER=src/mir/builder/raw_invocation_source_statement_classification.rs
README=src/mir/builder/README.md
CARD=docs/development/current/main/investigations/script-static-result-publication-source-lineage-witness-d0-2026-08-21.md

require_text() {
  local file="$1"
  local text="$2"
  grep -Fq "$text" "$file" || {
    echo "[$TAG] missing '$text' in $file" >&2
    exit 1
  }
}

require_text "$TRANSPORT" "expected_lineage: Option<RawInvocationRootLineageV1>"
require_text "$TRANSPORT" "unlocated_with_expected_lineage"
require_text "$TRANSPORT" "Some(root) => RawInvocationSourceTransportV1::unlocated_with_expected_lineage"
require_text "$WITNESS_TESTS" "unlocated_source_loss_retains_the_root_lineage_witness"
require_text "$WITNESS_TESTS" "compatibility_unlocated_context_has_no_lineage_witness"
require_text "$CLAIM_TRANSPORT" "RawInvocationSourceContextV1::UnlocatedCompatibility { .. }"
require_text "$CLASSIFIER" "expected_lineage: Some(actual_root)"
require_text "$CLASSIFIER" "RawInvocationRootLineageV1::ScriptRoot"
require_text "$README" "Source-lineage witness for unlocated calls (P0)"
require_text "$CARD" "SCRIPT-STATIC-RESULT-PUBLICATION-SOURCE-LINEAGE-WITNESS-P0"
require_text "$CARD" "Located(Cataloged)"
require_text "$CARD" "Located(non-Cataloged)"
require_text "$CARD" "Unlocated(expected=Some(Cataloged))"
require_text "$CARD" "Unlocated(expected=None)"
require_text "$CARD" "Foreign/contradictory witness"
require_text "$CARD" "No wildcard, default, or empty witness is permitted"

for file in "$TRANSPORT" "$CLAIM_TRANSPORT" "$CLASSIFIER" "$WITNESS_TESTS"; do
  lines="$(wc -l < "$file")"
  if (( lines >= 760 )); then
    echo "[$TAG] source split required: $file has $lines lines" >&2
    exit 1
  fi
done

transport_test_lines="$(wc -l < "$TRANSPORT_TESTS")"
if (( transport_test_lines >= 800 )); then
  echo "[$TAG] test split required: $TRANSPORT_TESTS has $transport_test_lines lines" >&2
  exit 1
fi

if rg -n 'RawInvocationSourceContextV1::UnlocatedCompatibility\(' \
  src/mir/builder --glob '*.rs'; then
  echo "[$TAG] stale tuple-form UnlocatedCompatibility construction remains" >&2
  exit 1
fi

if rg -n 'RawInvocationSourceTransportV1::unlocated\(statement, reason\)' "$TRANSPORT"; then
  echo "[$TAG] source-backed body demotion dropped its lineage witness" >&2
  exit 1
fi

echo "[$TAG] OK"
