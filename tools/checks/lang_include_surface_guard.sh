#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TARGET_DIR="$ROOT_DIR/lang/src"
TAG="lang-include-surface-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"
TMP_FILE="/tmp/hako_include_hits_$$.txt"
trap 'rm -f "$TMP_FILE"' EXIT

if [[ -e "$ROOT_DIR/tools/dev/check_lang_includes.sh" ]]; then
  guard_fail "$TAG" "lang include guard returned to active tools/dev"
fi

if [ ! -d "$TARGET_DIR" ]; then
  guard_fail "$TAG" "target dir not found: $TARGET_DIR"
fi

echo "[$TAG] scanning for include statements under lang/src ..." >&2
if rg -n '^\s*include\s+"' "$TARGET_DIR" >"$TMP_FILE"; then
  echo "[FAIL] include statements found (VM backend does not support include):" >&2
  cat "$TMP_FILE" >&2
  echo "[hint] Prefer 'using "alias" as Name' with nyash.toml [modules]." >&2
  echo "[hint] For dev/tests, set NYASH_PREINCLUDE=1 to expand includes temporarily." >&2
  exit 1
else
  echo "[$TAG] ok" >&2
fi
exit 0
