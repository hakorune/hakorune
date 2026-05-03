#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT"

# Raw Program(JSON v0) compat emit stays live only as an explicit keeper seam.
# Archived engineering evidence is excluded; this guard locks active tools.
allowed_files=(
  "tools/lib/program_json_v0_compat.sh"
  "tools/selfhost/lib/stage1_contract.sh"
  "tools/smokes/v2/lib/stageb_helpers.sh"
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

rg -n 'program_json_v0_compat\.sh|program_json_v0_compat_emit_to_file|--emit-program-json-v0' tools \
  -g '*.sh' \
  -g '!tools/checks/program_json_v0_compat_caller_guard.sh' \
  -g '!tools/archive/**' \
  -g '!tools/smokes/v2/profiles/archive/**' \
  >"$tmp" || true

bad=0
while IFS=: read -r path line rest; do
  if [ -z "${path:-}" ]; then
    continue
  fi
  if is_allowed_file "$path"; then
    continue
  fi
  echo "[program-json-v0-compat-caller-guard] unexpected active compat caller: $path:$line:$rest" >&2
  bad=1
done <"$tmp"

if [ "$bad" -ne 0 ]; then
  echo "[program-json-v0-compat-caller-guard] keep raw Program(JSON v0) emit behind stage1_contract.sh or stageb_helpers.sh" >&2
  exit 1
fi

echo "[program-json-v0-compat-caller-guard] ok"
