#!/usr/bin/env bash
set -euo pipefail

ROOT=$(git rev-parse --show-toplevel 2>/dev/null || pwd)
cd "$ROOT"

# Keep in sync with docs/development/current/main/design/plan-lowering-entry-ssot.md
# Legacy physical path rationale is tracked in
# docs/development/current/main/design/route-physical-path-legacy-lane-ssot.md
ALLOW_GLOBS=(
  '!src/mir/builder/control_flow/joinir/route_entry/router.rs'
  '!src/mir/builder/control_flow/joinir/route_entry/registry/handlers.rs'
  '!src/mir/builder/control_flow/joinir/route_entry/registry/handlers/*.rs'
  '!src/mir/builder/stmts/return_stmt.rs'
  '!src/mir/builder/control_flow/plan/lowerer/mod.rs'
  '!tests/**'
  '!src/**/tests/**'
  '!crates/**/tests/**'
)

RG_ARGS=(--type rust -n -S)
for g in "${ALLOW_GLOBS[@]}"; do
  RG_ARGS+=(--glob "$g")
done

tmpfile="$(mktemp)"
trap 'rm -f "$tmpfile"' EXIT

set +e
rg "${RG_ARGS[@]}" 'PlanLowerer::lower[[:space:]]*\(' src >"$tmpfile"
rg_code=$?
set -e
if [[ $rg_code -ne 0 && $rg_code -ne 1 ]]; then
  echo "[plan-lower-entry] ERROR: rg failed (code=$rg_code)."
  exit 2
fi

# Filter comment-only hits.
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

if [[ -n "$matches" ]]; then
  echo "[plan-lower-entry] ERROR: unapproved PlanLowerer::lower entrypoint detected."
  echo "Fix: route through approved entry files or update allowlist + SSOT."
  echo
  echo "$matches"
  exit 1
fi

echo "[plan-lower-entry] OK"
