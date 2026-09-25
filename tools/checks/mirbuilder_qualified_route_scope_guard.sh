#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mirbuilder-qualified-route-scope"
MAIN_ROOT="$ROOT_DIR/src/mir/builder/normal_callable_semantic_loan_port/main_root.rs"
PIN_TEST="$ROOT_DIR/src/mir/compiler/normal_default_pipeline_tests.rs"

command -v rg >/dev/null
command -v wc >/dev/null

# The canonical qualified-methods diversion must key on QualifiedUnbound
# receivers only (MIRBUILDER-EXE-ACCEPTANCE-QUALIFIED-PREFLIGHT-ROUTE-SCOPE-S0).
rg -q 'ResolvedMethodCallReceiverSourceV1::QualifiedUnbound' "$MAIN_ROOT"
rg -q 'method_calls\(\)\.any' "$MAIN_ROOT"
if rg -n 'method_calls\(\)\.next\(\)\.is_some\(\)' "$MAIN_ROOT"; then
  echo "[$TAG] diversion regressed to any-method-call predicate" >&2
  exit 1
fi

# Lexical-receiver regression pin must stay in place.
rg -q 'lexical_receiver_method_call_main_stays_off_qualified_route' "$PIN_TEST"

for file in "$MAIN_ROOT" "$PIN_TEST"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    echo "[$TAG] source reached hard 800-line boundary: ${file#"$ROOT_DIR/"}=$lines" >&2
    exit 1
  fi
done

echo "[$TAG] ok"
