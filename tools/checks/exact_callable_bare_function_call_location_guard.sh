#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

require() {
  grep -Fq "$2" "$1" || {
    echo "[exact-callable-bare-call] missing '$2' in $1" >&2
    exit 1
  }
}

CLASSIFIER=src/mir/builder/raw_invocation_source_statement_classification.rs
TRANSPORT=src/mir/builder/raw_invocation_source_transport.rs
PREFLIGHT=src/mir/builder/calls/function_call_preflight_route.rs
CARD=docs/development/current/main/investigations/brand-instance-constructor-source-relation-d0.md
README=src/mir/builder/README.md

require "$CLASSIFIER" 'is_bare_function_call_statement'
require "$TRANSPORT" 'allows_bare_function_call_location'
require "$TRANSPORT" 'Self::Cataloged(_) | Self::TopLevel(_) | Self::InstanceConstructor(_)'
require "$TRANSPORT" 'is_bare_function_call_statement(&statement)'
require "$TRANSPORT" 'is_bare_function_call_statement(statement)'
require "$PREFLIGHT" 'is_brand_declared'
require "$CARD" 'EXACT-CALLABLE-BARE-FUNCTION-CALL-LOCATION-P0'
require "$CARD" 'Raw-only, Compatibility, Deferred, nested/Main, and unlocated-only paths'
require "$README" 'Exact callable bare-call location (P0)'

if rg -n 'ASTNode::MethodCall.*is_bare_function_call_statement|ASTNode::Call.*is_bare_function_call_statement' \
  "$CLASSIFIER" "$TRANSPORT"; then
  echo "[exact-callable-bare-call] call shape widened beyond bare FunctionCall" >&2
  exit 1
fi

for file in "$CLASSIFIER" "$TRANSPORT"; do
  lines="$(wc -l < "$file")"
  if (( lines >= 760 )); then
    echo "[exact-callable-bare-call] source split required: $file has $lines lines" >&2
    exit 1
  fi
done

test_lines="$(wc -l < src/mir/builder/raw_invocation_source_transport_tests.rs)"
if (( test_lines >= 800 )); then
  echo "[exact-callable-bare-call] transport test hard stop exceeded: $test_lines" >&2
  exit 1
fi

echo "[exact-callable-bare-call] OK"
