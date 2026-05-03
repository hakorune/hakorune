#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT"

# Stage-B Program(JSON) stdout capture remains an explicit compat/debug
# capsule. Archived engineering evidence is excluded; active callers stay
# behind the known MIR emit / Stage-B helper surfaces only.
allowed_files=(
  "tools/selfhost/lib/stageb_program_json_capture.sh"
  "tools/hakorune_emit_mir.sh"
  "tools/selfhost_exe_stageb.sh"
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

rg -n 'stageb_program_json_capture\.sh|stageb_program_json_extract_from_stdin|stageb_source_program_json_capture' tools \
  -g '*.sh' \
  -g '!tools/checks/stageb_program_json_capture_caller_guard.sh' \
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
  echo "[stageb-program-json-capture-caller-guard] unexpected active capture caller: $path:$line:$rest" >&2
  bad=1
done <"$tmp"

if [ "$bad" -ne 0 ]; then
  echo "[stageb-program-json-capture-caller-guard] keep Stage-B Program(JSON) capture behind hakorune_emit_mir.sh or Stage-B helper surfaces only" >&2
  exit 1
fi

echo "[stageb-program-json-capture-caller-guard] ok"
