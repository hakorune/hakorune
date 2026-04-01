#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"
MANIFEST="$ROOT/tools/smokes/v2/profiles/integration/core/phase2044/llvmlite_monitor_keep.txt"

echo "[phase2044] llvmlite monitor-only keep"

if [[ ! -f "$MANIFEST" ]]; then
  echo "[phase2044] missing manifest: $MANIFEST" >&2
  exit 2
fi

while IFS= read -r filter; do
  [[ -z "$filter" || "${filter:0:1}" == "#" ]] && continue
  bash "$ROOT/tools/smokes/v2/run.sh" --profile integration --filter "$filter"
done < "$MANIFEST"

echo "[phase2044] llvmlite monitor-only keep done."
