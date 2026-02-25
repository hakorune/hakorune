#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"

cd "$ROOT_DIR"

echo "[abi-lane-guard] checking that hako_abi_v1 symbols are not implemented in source lanes"

SYMBOL_PATTERN='\bhako_(str|arr|map)_(new|retain|release|size|length|get|set|push|slice|remove|keys|values|index_of|substring|to_string)\b'

if command -v rg >/dev/null 2>&1; then
  HITS="$(rg -n "$SYMBOL_PATTERN" src crates lang include -S || true)"
  LANE_HITS="$(rg -n "hako_abi_v1" src crates lang include -S || true)"
else
  echo "[abi-lane-guard] WARN: rg not found; falling back to grep" >&2
  SYMBOL_PATTERN_GREP='(^|[^[:alnum:]_])hako_(str|arr|map)_(new|retain|release|size|length|get|set|push|slice|remove|keys|values|index_of|substring|to_string)($|[^[:alnum:]_])'
  HITS="$(grep -RInE "$SYMBOL_PATTERN_GREP" src crates lang include || true)"
  LANE_HITS="$(grep -RIn "hako_abi_v1" src crates lang include || true)"
fi

if [[ -n "$HITS" ]]; then
  echo "[abi-lane-guard] ERROR: found non-canonical hako_abi_v1 symbol usage in source tree:"
  echo "$HITS"
  exit 1
fi

if [[ -n "$LANE_HITS" ]]; then
  echo "[abi-lane-guard] ERROR: found hako_abi_v1 lane reference in source tree:"
  echo "$LANE_HITS"
  exit 1
fi

echo "[abi-lane-guard] ok"
