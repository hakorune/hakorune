#!/usr/bin/env bash
# bootstrap_s1_s2_s3_repeat.sh — Run generator command 3 times and ensure normalized v1 hash matches
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
LIB_TR="$ROOT/tools/smokes/v2/lib/test_runner.sh"
if [ -f "$LIB_TR" ]; then source "$LIB_TR"; else echo "[FAIL] missing $LIB_TR" >&2; exit 2; fi

if [ $# -lt 1 ]; then
  echo "Usage: $0 '<generator_cmd>'" >&2
  exit 2
fi

GEN="$1"
tmp1="/tmp/hako_repeat_1_$$.json"
tmp2="/tmp/hako_repeat_2_$$.json"
tmp3="/tmp/hako_repeat_3_$$.json"
trap 'rm -f "$tmp1" "$tmp2" "$tmp3"' EXIT

eval "$GEN" > "$tmp1"
eval "$GEN" > "$tmp2"
eval "$GEN" > "$tmp3"

h1=$(v1_normalized_hash "$tmp1" || true)
h2=$(v1_normalized_hash "$tmp2" || true)
h3=$(v1_normalized_hash "$tmp3" || true)

echo "[H1] $h1" >&2
echo "[H2] $h2" >&2
echo "[H3] $h3" >&2

if [ -z "$h1" ] || [ -z "$h2" ] || [ -z "$h3" ]; then
  echo "[FAIL] invalid JSON(s) from generator" >&2
  exit 2
fi

if [ "$h1" = "$h2" ] && [ "$h2" = "$h3" ]; then
  echo "[PASS] repeat hashes match"
  exit 0
fi
echo "[FAIL] repeat hashes differ" >&2
exit 1

