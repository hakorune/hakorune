#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mir-builder-layer-dependency-guard"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

viol=0

echo "[$TAG] checking builder layer dependencies (origin -> observe -> rewrite)"

# Rule 1: origin/* must NOT import observe or rewrite
while IFS= read -r -d '' f; do
  if rg -n "use\\s+crate::mir::builder::(observe|rewrite)" "$f" >/dev/null; then
    echo "[ERROR] origin-layer import violation: $f"
    rg -n "use\\s+crate::mir::builder::(observe|rewrite)" "$f" || true
    viol=$((viol+1))
  fi
done < <(find src/mir/builder/origin -type f -name '*.rs' -print0 2>/dev/null || true)

# Rule 2: observe/* must NOT import rewrite or origin
while IFS= read -r -d '' f; do
  if rg -n "use\\s+crate::mir::builder::(rewrite|origin)" "$f" >/dev/null; then
    echo "[ERROR] observe-layer import violation: $f"
    rg -n "use\\s+crate::mir::builder::(rewrite|origin)" "$f" || true
    viol=$((viol+1))
  fi
done < <(find src/mir/builder/observe -type f -name '*.rs' -print0 2>/dev/null || true)

# Rule 3: rewrite/* must NOT import origin (observe is allowed)
while IFS= read -r -d '' f; do
  if rg -n "use\\s+crate::mir::builder::origin" "$f" >/dev/null; then
    echo "[ERROR] rewrite-layer import violation: $f"
    rg -n "use\\s+crate::mir::builder::origin" "$f" || true
    viol=$((viol+1))
  fi
done < <(find src/mir/builder/rewrite -type f -name '*.rs' -print0 2>/dev/null || true)

if [[ $viol -gt 0 ]]; then
  guard_fail "$TAG" "$viol violation(s) detected"
else
  echo "[$TAG] ok"
fi
