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

# Only documented front doors are direct-executable.  Helpers and smoke
# bodies intentionally stay 0644 and are launched through `bash`.
canonical_entrypoints=(
  "tools/selfhost/run.sh"
  "tools/selfhost/selfhost_build.sh"
  "tools/smokes/v2/run.sh"
  "tools/smokes/v2/lib/emit_mir_route.sh"
)

check_entrypoint_mode() {
  local path="$1"
  local records
  local index_mode
  records="$(git ls-files -s -- "$path")"
  if [ -z "$records" ]; then
    echo "[hakorune-emit-mir-direct-caller-guard] missing tracked entrypoint: $path" >&2
    return 1
  fi
  index_mode="$(printf '%s\n' "$records" | awk 'NR == 1 { print $1 }')"
  if [ "$index_mode" != "100755" ] || [ ! -x "$path" ]; then
    echo "[hakorune-emit-mir-direct-caller-guard] entrypoint must be executable (tracked 100755): $path" >&2
    return 1
  fi
}

mode_bad=0
for entrypoint in "${canonical_entrypoints[@]}"; do
  check_entrypoint_mode "$entrypoint" || mode_bad=1
done
if [ "$mode_bad" -ne 0 ]; then
  echo "[hakorune-emit-mir-direct-caller-guard] keep helper/test scripts 0644 and invoke them with bash; do not mass chmod" >&2
  exit 1
fi

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
