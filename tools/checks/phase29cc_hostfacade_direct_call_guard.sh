#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

ALLOWLIST="tools/checks/phase29cc_hostfacade_direct_call_allowlist.txt"
if [ ! -f "$ALLOWLIST" ]; then
  echo "[hostfacade-direct-call-guard] missing allowlist: $ALLOWLIST" >&2
  exit 1
fi

mapfile -t ALLOWED < <(sed '/^\s*#/d;/^\s*$/d' "$ALLOWLIST")
if [ "${#ALLOWED[@]}" -eq 0 ]; then
  echo "[hostfacade-direct-call-guard] allowlist is empty" >&2
  exit 1
fi

TARGET_DIRS=(
  "lang/src/runtime"
  "lang/src/shared"
  "lang/src/vm"
)

tmp_all="$(mktemp)"
tmp_bad="$(mktemp)"
trap 'rm -f "$tmp_all" "$tmp_bad"' EXIT

for d in "${TARGET_DIRS[@]}"; do
  if [ ! -d "$d" ]; then
    continue
  fi
  rg -n -g '*.hako' 'hostbridge\.(extern_invoke|box_new|box_call)' "$d" \
    | awk -F: '{
        txt=$0;
        sub(/^[^:]+:[0-9]+:/, "", txt);
        if (txt ~ /^[[:space:]]*\/\//) next;
        print $0;
      }' \
    >>"$tmp_all" || true
done

if [ ! -s "$tmp_all" ]; then
  echo "[hostfacade-direct-call-guard] no direct hostbridge calls found" >&2
  exit 1
fi

while IFS= read -r line; do
  file="${line%%:*}"
  ok=0
  for allow in "${ALLOWED[@]}"; do
    if [ "$file" = "$allow" ]; then
      ok=1
      break
    fi
  done
  if [ "$ok" -eq 0 ]; then
    echo "$line" >>"$tmp_bad"
  fi
done <"$tmp_all"

if [ -s "$tmp_bad" ]; then
  echo "[hostfacade-direct-call-guard] direct hostbridge calls outside allowlist:" >&2
  cat "$tmp_bad" >&2
  exit 1
fi

echo "[hostfacade-direct-call-guard] ok"
