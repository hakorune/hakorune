#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="docs-slim-027-phase-row-writer-pilot"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

TOOL="tools/docs/phase_row.py"
README="tools/docs/README.md"
POLICY="docs/development/current/main/design/current-docs-update-policy-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/docs_slim_027_phase_row_writer_pilot_guard.sh"

echo "[$TAG] checking phase row writer pilot"

guard_require_files "$TAG" "$TOOL" "$README" "$POLICY" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"
guard_require_command "$TAG" python3

guard_expect_fixed_in_file "$TAG" "Phase Row Writer" "$README" "tools docs README must describe the phase row writer"
guard_expect_fixed_in_file "$TAG" "tools/docs/phase_row.py" "$POLICY" "current docs policy must name the row writer"
guard_expect_fixed_in_file "$TAG" "dry-run" "$POLICY" "policy must require dry-run first"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

help_out="$(mktemp /tmp/${TAG}.help.XXXXXX)"
dry_out="$(mktemp /tmp/${TAG}.dry.XXXXXX)"
trap 'rm -f "$help_out" "$dry_out"' EXIT

python3 "$TOOL" --help >"$help_out"
guard_expect_fixed_in_file "$TAG" "create" "$help_out" "tool help must expose create command"

python3 "$TOOL" create \
  --row 295x-999 \
  --row-number 999 \
  --slug DOCS-ROW-WRITER-PILOT-DRY-RUN \
  --title "Docs Row Writer Pilot Dry Run" \
  --scope "dry-run contract only" \
  --blocker DOCS-ROW-WRITER-PILOT-DRY-RUN-295X-001 \
  --summary "dry-run row writer output without modifying files" \
  --previous-card docs/development/current/main/phases/phase-295x/295x-199-MIMALLOC-COMPARISON-NYRT-PLUGIN-HOST-BASELINE-SELECTION.md \
  --selected-row DOCS-ROW-WRITER-PILOT-FOLLOW-ON-295X-001 \
  --queue-boundary "Dry-run only; do not write files." \
  --land-row 199 \
  >"$dry_out"

guard_expect_fixed_in_file "$TAG" "[phase-row] dry-run only" "$dry_out" "tool must default to dry-run"
if [[ -e docs/development/current/main/phases/phase-295x/295x-999-DOCS-ROW-WRITER-PILOT-DRY-RUN.md ]]; then
  guard_fail "$TAG" "dry-run created a phase card"
fi

echo "[$TAG] ok"
