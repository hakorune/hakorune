#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT"

# Stage1 Program(JSON) compat execution is a probe-only route. Keep the exact
# helper callable only from the explicit phase29ch compatibility probe.
allowed_files=(
  "tools/selfhost/lib/stage1_contract.sh"
  "tools/dev/phase29ch_program_json_compat_route_probe.sh"
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

rg -n 'stage1_contract_exec_program_json_compat' tools \
  -g '*.sh' \
  -g '!tools/checks/stage1_program_json_compat_caller_guard.sh' \
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
  echo "[stage1-program-json-compat-caller-guard] unexpected active compat caller: $path:$line:$rest" >&2
  bad=1
done <"$tmp"

if [ "$bad" -ne 0 ]; then
  echo "[stage1-program-json-compat-caller-guard] keep Stage1 Program(JSON) compat execution behind phase29ch explicit probe only" >&2
  exit 1
fi

echo "[stage1-program-json-compat-caller-guard] ok"
