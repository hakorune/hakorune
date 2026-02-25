#!/usr/bin/env bash
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$ROOT"

FILE="src/mir/builder/builder_emit.rs"

require_signature() {
  local needle="$1"
  if ! rg -n --fixed-strings "$needle" "$FILE" >/dev/null; then
    echo "[builder-emit-visibility] ERROR: signature drift detected: $needle"
    exit 1
  fi
}

require_signature "pub(in crate::mir::builder) fn emit_instruction("
require_signature "pub(in crate::mir::builder) fn emit_extern_call_with_effects("
require_signature "pub(in crate::mir::builder) fn emit_extern_call("

tmpfile="$(mktemp)"
trap 'rm -f "$tmpfile"' EXIT

set +e
rg --type rust -n -S '\.emit_instruction\(' src \
  --glob '!src/mir/builder/**' \
  --glob '!src/**/tests/**' \
  --glob '!tests/**' >"$tmpfile"
rg_code=$?
set -e

if [[ $rg_code -ne 0 && $rg_code -ne 1 ]]; then
  echo "[builder-emit-visibility] ERROR: rg failed (code=$rg_code)."
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
      if (content ~ /^[[:space:]]*\*/) next;
      print line;
    }
  ' "$tmpfile" | sort -u
)

if [[ -n "$matches" ]]; then
  echo "[builder-emit-visibility] ERROR: direct emit_instruction calls outside src/mir/builder detected."
  echo "Fix: route through builder facade/SSOT entrypoints and keep raw emit inside builder layer only."
  echo
  echo "$matches"
  exit 1
fi

echo "[builder-emit-visibility] OK"
