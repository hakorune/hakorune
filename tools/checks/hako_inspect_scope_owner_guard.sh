#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="hako-inspect-scope-owner-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

ENTRY="$ROOT_DIR/tools/hako_check/inspect_scope_dump.py"
MODEL="$ROOT_DIR/tools/hako_check/inspect_scope_model.py"
IDENTITY="$ROOT_DIR/tools/hako_check/inspect_scope_identity.py"
SHAPE_MODEL="$ROOT_DIR/tools/hako_check/inspect_shape_model.py"
SHAPE_CLI="$ROOT_DIR/tools/hako_check/inspect_shape.py"
TEST="$ROOT_DIR/tools/hako_check/tests/test_inspect_scope_dump.py"
SHAPE_TEST="$ROOT_DIR/tools/hako_check/tests/test_inspect_shape.py"

guard_require_command "$TAG" rg
guard_require_files "$TAG" "$ENTRY" "$MODEL" "$IDENTITY" "$SHAPE_MODEL" \
  "$SHAPE_CLI" "$TEST" "$SHAPE_TEST"

for file in "$ENTRY" "$MODEL" "$IDENTITY" "$SHAPE_MODEL" "$SHAPE_CLI"; do
  lines="$(wc -l <"$file" | tr -d '[:space:]')"
  (( lines < 760 )) || \
    guard_fail "$TAG" "source reached 760-line split trigger: $file=$lines"
done

for symbol in mir_shape llvm_shape asm_shape build_shape_report; do
  [[ "$(rg -c "^def ${symbol}\\(" "$SHAPE_MODEL")" == "1" ]] || \
    guard_fail "$TAG" "shape model owner drift: $symbol"
  if rg -n "^def ${symbol}\\(" "$ENTRY" "$MODEL" "$IDENTITY" "$SHAPE_CLI"; then
    guard_fail "$TAG" "non-shape owner reintroduced shape model: $symbol"
  fi
done

for symbol in build_identity_contract validate_identity_contract \
  require_unique_mir_function require_unique_llvm_function \
  require_unique_asm_symbol; do
  [[ "$(rg -c "^def ${symbol}\\(" "$IDENTITY")" == "1" ]] || \
    guard_fail "$TAG" "identity owner drift: $symbol"
  if rg -n "^def ${symbol}\\(" "$ENTRY" "$MODEL"; then
    guard_fail "$TAG" "non-identity owner reintroduced seal logic: $symbol"
  fi
done

for symbol in bundle_report_rows format_report manifest_contract \
  read_bundle_report route_counts selected_route_rows; do
  [[ "$(rg -c "^def ${symbol}\\(" "$MODEL")" == "1" ]] || \
    guard_fail "$TAG" "model owner drift: $symbol"
  if rg -n "^def ${symbol}\\(" "$ENTRY"; then
    guard_fail "$TAG" "entry reintroduced model owner: $symbol"
  fi
done

if rg -n '^(import os|import subprocess|import tempfile|EMIT_ROUTE|TRACE_BUNDLE|def emit_mir_json\(|def emit_llvm_asm_bundle\()' "$MODEL" "$IDENTITY" "$SHAPE_MODEL"; then
  guard_fail "$TAG" "model acquired effect-bearing responsibility"
fi

python3 -m unittest tools.hako_check.tests.test_inspect_scope_dump \
  tools.hako_check.tests.test_inspect_shape >/dev/null
echo "[$TAG] ok (thin inspect entry + pure metadata/identity/shape models)"
