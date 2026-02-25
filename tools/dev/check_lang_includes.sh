#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TARGET_DIR="$ROOT_DIR/lang/src"

if [ ! -d "$TARGET_DIR" ]; then
  echo "[check] target dir not found: $TARGET_DIR" >&2
  exit 2
fi

echo "[check] scanning for include statements under lang/src ..." >&2
if rg -n '^\s*include\s+"' "$TARGET_DIR" >/tmp/hako_include_hits_$$.txt; then
  echo "[FAIL] include statements found (VM backend does not support include):" >&2
  cat /tmp/hako_include_hits_$$.txt >&2
  echo "[hint] Prefer 'using "alias" as Name' with nyash.toml [modules]." >&2
  echo "[hint] For dev/tests, set NYASH_PREINCLUDE=1 to expand includes temporarily." >&2
  rm -f /tmp/hako_include_hits_$$.txt
  exit 1
else
  echo "[OK] no include statements found under lang/src" >&2
fi
rm -f /tmp/hako_include_hits_$$.txt
exit 0

