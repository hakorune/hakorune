#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT"

# Program(JSON)->MIR bridge stays live only as an explicit compat capsule.
# Archived engineering evidence is excluded; this guard locks active tools.
allowed_files=(
  "tools/selfhost/lib/program_json_mir_bridge.sh"
  "tools/selfhost_exe_stageb.sh"
  "tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh"
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

rg -n 'program_json_mir_bridge\.sh|program_json_mir_bridge_emit' tools \
  -g '*.sh' \
  -g '!tools/checks/program_json_mir_bridge_caller_guard.sh' \
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
  echo "[program-json-mir-bridge-caller-guard] unexpected active bridge caller: $path:$line:$rest" >&2
  bad=1
done <"$tmp"

if [ "$bad" -ne 0 ]; then
  echo "[program-json-mir-bridge-caller-guard] keep Program(JSON)->MIR bridge behind selfhost_exe_stageb.sh or phase29cg proof only" >&2
  exit 1
fi

echo "[program-json-mir-bridge-caller-guard] ok"
