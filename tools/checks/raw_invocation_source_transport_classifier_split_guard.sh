#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

PARENT=src/mir/builder/raw_invocation_source_transport.rs
CHILD=src/mir/builder/raw_invocation_source_statement_classification.rs

require_text() {
  local file="$1"
  local text="$2"
  grep -Fq "$text" "$file" || {
    echo "[raw-source-classifier-split] missing '$text' in $file" >&2
    exit 1
  }
}

reject_text() {
  local file="$1"
  local text="$2"
  if grep -Fq "$text" "$file"; then
    echo "[raw-source-classifier-split] forbidden '$text' in $file" >&2
    exit 1
  fi
}

require_text "$PARENT" "raw_invocation_source_statement_classification"
require_text "$CHILD" "ASTNode::FunctionCall"
require_text "$CHILD" "ASTNode::MethodCall"
require_text "$CHILD" "RawUnlocatedPortalV1::CallObject"
require_text "$CHILD" "is_located_scalar_statement"
require_text "$CHILD" "is_located_control_or_diagnostic_terminal"
reject_text "$PARENT" "fn reason_for_non_box_statement"
reject_text "$PARENT" "fn is_located_scalar_statement"

parent_lines="$(wc -l < "$PARENT")"
child_lines="$(wc -l < "$CHILD")"
if (( parent_lines >= 760 || child_lines >= 760 )); then
  echo "[raw-source-classifier-split] split boundary failed parent=$parent_lines child=$child_lines" >&2
  exit 1
fi

CARGO_BUILD_JOBS=4 cargo test --profile quick -q -p nyash-rust \
  mir::builder::raw_invocation_source_statement_classification::tests --lib

echo "[raw-source-classifier-split] OK parent=$parent_lines child=$child_lines"
