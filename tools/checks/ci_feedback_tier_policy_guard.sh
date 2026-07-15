#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
MIN_GATE="$ROOT_DIR/.github/workflows/min-gate.yml"
PORTABILITY="$ROOT_DIR/.github/workflows/portability-ci.yml"
README="$ROOT_DIR/.github/workflows/README.md"
TAG="ci-feedback-tier-policy-guard"

fail() {
  echo "[$TAG] ERROR: $*" >&2
  exit 1
}

require_text() {
  local file="$1"
  local text="$2"
  grep -F -q -- "$text" "$file" || fail "missing contract in ${file#$ROOT_DIR/}: $text"
}

trigger='types: [opened, synchronize, reopened, ready_for_review, converted_to_draft]'
full_gate_condition="if: \${{ github.event_name == 'pull_request' && github.event.pull_request.draft == false }}"
portability_condition="if: \${{ github.event_name == 'workflow_dispatch' || github.event.pull_request.draft == false }}"

require_text "$MIN_GATE" "$trigger"
require_text "$MIN_GATE" 'CI feedback tier policy guard'
require_text "$MIN_GATE" "$full_gate_condition"
require_text "$PORTABILITY" "$trigger"
require_text "$README" 'Draft pull requests run the fast `rust-check` steps on every update.'

portability_condition_count="$(grep -F -c -- "$portability_condition" "$PORTABILITY")"
[[ "$portability_condition_count" -eq 3 ]] \
  || fail "expected 3 draft-gated portability jobs, found $portability_condition_count"

echo "[$TAG] ok"
