#!/usr/bin/env bash
set -euo pipefail

ROOT=$(git rev-parse --show-toplevel 2>/dev/null || pwd)
cd "$ROOT"

# Keep in sync with docs/development/current/main/design/recipe-tree-and-parts-ssot.md (M6-final-1).
ALLOWLIST_RE='^src/mir/builder/control_flow/plan/core.rs:[0-9]+:[[:space:]]*Seq\(Vec<CorePlan>\),$'
BOUNDARY_PATTERN='->\s*CorePlan\b|\[CorePlan\]|&CorePlan\b|Vec<CorePlan>|Result<.*CorePlan|Option<CorePlan>|Option<Vec<CorePlan>>|pub core_plan:\s*CorePlan'

ALLOW_GLOBS=(
  '!**/*tests.rs'
  '!**/tests.rs'
  '!**/*.md'
)

RG_ARGS=(--type rust -n -S)
for g in "${ALLOW_GLOBS[@]}"; do
  RG_ARGS+=(--glob "$g")
done

tmpfile="$(mktemp)"
trap 'rm -f "$tmpfile"' EXIT

set +e
rg "${RG_ARGS[@]}" -- "$BOUNDARY_PATTERN" src/mir/builder/control_flow/plan >"$tmpfile"
rg_code=$?
set -e
if [[ $rg_code -ne 0 && $rg_code -ne 1 ]]; then
  echo "[coreplan-boundary] ERROR: rg failed (code=$rg_code)."
  exit 2
fi

# Ignore comment-only lines (including //! and /// docs).
matches=$(
  awk -F: '
    {
      line=$0;
      content=line;
      sub(/^[^:]*:[0-9]+:/, "", content);
      if (content ~ /^[[:space:]]*\/\//) next;
      print line;
    }
  ' "$tmpfile" | sort -u
)

disallowed="$(echo "$matches" | grep -Ev "$ALLOWLIST_RE" || true)"
if [[ -n "$disallowed" ]]; then
  echo "[coreplan-boundary] ERROR: unapproved CorePlan type boundary detected."
  echo "Fix: migrate boundary to LoweredRecipe or update allowlist + SSOT."
  echo
  echo "$disallowed"
  exit 1
fi

echo "[coreplan-boundary] OK"
