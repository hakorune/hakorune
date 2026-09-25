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

# MAIN-IMPORT-ARITY-SCOPE-S0: the relation issue and the route diversion are
# both arity-0-scoped. An arity-bearing app main can never consume a row.
MODEL="$ROOT_DIR/src/mir/normal_callable_semantic_package/model.rs"
ARITY_PIN_TEST="$ROOT_DIR/src/mir/compiler/normal_default_pipeline_arity_scope_tests.rs"
ENTRY_PORT="$ROOT_DIR/src/mir/builder/normal_callable_binding_materialization_port.rs"
rg -q 'caller\.arity\(\) != 0' "$MODEL"

# MAIN-WRAPPER-PARAM-ENTRY-S0: declared source parameters adopt the
# injector's published `variable_map` locals via StaticInjectedLocals, and
# the diversion gate checks the DECLARED arity (wrapper physical formals
# are structurally 0, so a physical-formal check would be vacuous).
rg -q 'StaticInjectedLocals' "$ENTRY_PORT"
rg -q 'callable-entry/injected-local-missing' "$ENTRY_PORT"
rg -q 'declared_parameter_names_v1' "$MAIN_ROOT"
rg -q 'if arity0' "$MAIN_ROOT"

# Lexical-receiver and arity regression pins must stay in place.
rg -q 'lexical_receiver_method_call_main_stays_off_qualified_route' "$PIN_TEST"
rg -q 'arity_bearing_main_with_qualified_call_stays_off_canonical_route' "$ARITY_PIN_TEST"
rg -q 'injected_locals_snapshot_follows_declared_name_order' "$ENTRY_PORT"
rg -q 'injected_locals_missing_name_fails_named_boundary' "$ENTRY_PORT"

for file in "$MAIN_ROOT" "$PIN_TEST" "$ARITY_PIN_TEST" "$ENTRY_PORT"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    echo "[$TAG] source reached hard 800-line boundary: ${file#"$ROOT_DIR/"}=$lines" >&2
    exit 1
  fi
done

echo "[$TAG] ok"
