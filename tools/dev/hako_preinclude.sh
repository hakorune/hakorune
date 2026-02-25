#!/usr/bin/env bash
# hako_preinclude.sh — Expand include "path" directives into inlined content.
# Usage: tools/dev/hako_preinclude.sh <in.hako> <out.hako>
#
# Env (optional):
#   HAKO_PREINCLUDE_CACHE=1            # enable cache (default: 1)
#   HAKO_PREINCLUDE_CACHE_DIR=/tmp/... # cache dir (default: /tmp/hako_preinclude_cache)
#   HAKO_PREINCLUDE_MAX_DEPTH=10       # include nesting max (default: 12)
#   HAKO_PREINCLUDE_MAX_SIZE=1048576   # expanded size bytes (default: 1 MiB)

set -euo pipefail

if [[ $# -lt 2 ]]; then
  echo "Usage: $0 <in.hako> <out.hako>" >&2
  exit 2
fi

IN="$1"
OUT="$2"

ROOT="${NYASH_ROOT:-}"
if [[ -z "$ROOT" ]]; then
  if ROOT_GIT=$(git -C "$(dirname "$IN")" rev-parse --show-toplevel 2>/dev/null); then
    ROOT="$ROOT_GIT"
  else
    ROOT="$(pwd)"
  fi
fi

declare -A SEEN
declare -A IMPORT_SEEN

MAX_DEPTH="${HAKO_PREINCLUDE_MAX_DEPTH:-12}"
MAX_SIZE="${HAKO_PREINCLUDE_MAX_SIZE:-1048576}"
USE_CACHE="${HAKO_PREINCLUDE_CACHE:-1}"
CACHE_DIR="${HAKO_PREINCLUDE_CACHE_DIR:-/tmp/hako_preinclude_cache}"
mkdir -p "$CACHE_DIR" >/dev/null 2>&1 || true

sum_file() { sha256sum "$1" 2>/dev/null | awk '{print $1}' || echo ""; }
cache_key() {
  local key
  key="$(sum_file "$1")"
  echo "$key"
}

expand_file() {
  local file="$1"
  local abspath="$file"
  if [[ "$abspath" != /* ]]; then
    abspath="$ROOT/$abspath"
  fi
  if [[ -n "${SEEN[$abspath]:-}" ]]; then
    echo "// [preinclude] Skipping already included: $abspath"; return
  fi
  SEEN[$abspath]=1
  if [[ ! -f "$abspath" ]]; then
    echo "// [preinclude][ERROR] Not found: $abspath" >&2
    return 1
  fi
  local depth="${2:-0}"
  if (( depth > MAX_DEPTH )); then
    echo "// [preinclude][ERROR] max depth exceeded at: $abspath" >&2
    return 1
  fi
  local current_size=0
  while IFS='' read -r line || [[ -n "$line" ]]; do
    if [[ "$line" =~ ^[[:space:]]*include[[:space:]]+\"([^\"]+)\" ]]; then
      inc="${BASH_REMATCH[1]}"
      expand_file "$inc" $((depth+1))
    elif [[ "$line" =~ ^[[:space:]]*using[[:space:]].* ]]; then
      key="${line//[[:space:]]/ }" # normalize spaces a bit
      if [[ -n "${IMPORT_SEEN[$key]:-}" ]]; then
        echo "// [preinclude] Skip duplicate using: $line"
      else
        IMPORT_SEEN[$key]=1
        printf '%s\n' "$line"
      fi
    else
      printf '%s\n' "$line"
    fi
    # crude size guard
    current_size=$((current_size + ${#line} + 1))
    if (( current_size > MAX_SIZE )); then
      echo "// [preinclude][ERROR] expanded size exceeded at: $abspath" >&2
      return 1
    fi
  done <"$abspath"
}
if [[ "$USE_CACHE" = "1" ]]; then
  key=$(cache_key "$IN")
  if [[ -n "$key" && -f "$CACHE_DIR/$key.hako" ]]; then
    cp "$CACHE_DIR/$key.hako" "$OUT"
    echo "[preinclude/cache-hit] $IN" >&2
    exit 0
  fi
fi

expand_file "$IN" >"$OUT"
if [[ "$USE_CACHE" = "1" && -n "${key:-}" ]]; then
  cp "$OUT" "$CACHE_DIR/$key.hako" 2>/dev/null || true
fi
echo "[preinclude] Wrote: $OUT" >&2
