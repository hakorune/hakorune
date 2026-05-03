#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT"

# `tools/hakorune_emit_mir.sh` is an internal compat-capsule implementation.
# New smoke/check/perf/dev scripts should use `tools/smokes/v2/lib/emit_mir_route.sh`.
allowed_files=(
  "tools/hakorune_emit_mir_compat.sh"
  "tools/hakorune_emit_mir_mainline.sh"
  "tools/selfhost/lib/selfhost_run_routes.sh"
)

is_allowed_file() {
  local candidate="$1"
  local allowed
  for allowed in "${allowed_files[@]}"; do
    if [ "$candidate" = "$allowed" ]; then
      return 0
    fi
  done
  return 1
}

tmp="$(mktemp)"
trap 'rm -f "$tmp"' EXIT

rg -n 'bash [^#]*hakorune_emit_mir\.sh' tools \
  -g '*.sh' \
  -g '!tools/hakorune_emit_mir.sh' \
  >"$tmp" || true

bad=0
while IFS=: read -r path line rest; do
  if [ -z "${path:-}" ]; then
    continue
  fi
  if is_allowed_file "$path"; then
    continue
  fi
  echo "[hakorune-emit-mir-direct-caller-guard] unexpected direct caller: $path:$line:$rest" >&2
  bad=1
done <"$tmp"

if [ "$bad" -ne 0 ]; then
  echo "[hakorune-emit-mir-direct-caller-guard] use tools/smokes/v2/lib/emit_mir_route.sh instead" >&2
  exit 1
fi

echo "[hakorune-emit-mir-direct-caller-guard] ok"
