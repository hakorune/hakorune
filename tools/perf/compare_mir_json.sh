#!/usr/bin/env bash
# compare_mir_json.sh — Structural diff for two MIR(JSON) files
# Usage: tools/perf/compare_mir_json.sh <a.json> <b.json>
# Prints sizes, sha1 (normalized), then unified diff (jq -S pretty) if available.

set -euo pipefail

if [[ $# -ne 2 ]]; then
  echo "Usage: $0 <a.json> <b.json>" >&2
  exit 2
fi
A="$1"; B="$2"
if [[ ! -f "$A" || ! -f "$B" ]]; then echo "error: file not found" >&2; exit 2; fi

sha1() {
  if command -v sha1sum >/dev/null 2>&1; then sha1sum | awk '{print $1}';
  elif command -v shasum >/dev/null 2>&1; then shasum -a 1 | awk '{print $1}';
  else openssl sha1 | awk '{print $2}'; fi
}

size_a=$(stat -c '%s' "$A" 2>/dev/null || stat -f '%z' "$A")
size_b=$(stat -c '%s' "$B" 2>/dev/null || stat -f '%z' "$B")

norm_a=$(jq -cS . "$A" 2>/dev/null || cat "$A")
norm_b=$(jq -cS . "$B" 2>/dev/null || cat "$B")

sha_a=$(printf '%s' "$norm_a" | sha1)
sha_b=$(printf '%s' "$norm_b" | sha1)

echo "A: $A (size=$size_a, sha1=$sha_a)"
echo "B: $B (size=$size_b, sha1=$sha_b)"

if [[ "$sha_a" == "$sha_b" ]]; then
  echo "= MIR JSON equal (normalized)"
  exit 0
fi

echo "- Diff (normalized, jq -S)"
tmpa=$(mktemp); tmpb=$(mktemp)
trap 'rm -f "$tmpa" "$tmpb" || true' EXIT
printf '%s\n' "$norm_a" | jq -S . >/dev/null 2>&1 && printf '%s\n' "$norm_a" | jq -S . >"$tmpa" || printf '%s\n' "$norm_a" >"$tmpa"
printf '%s\n' "$norm_b" | jq -S . >/dev/null 2>&1 && printf '%s\n' "$norm_b" | jq -S . >"$tmpb" || printf '%s\n' "$norm_b" >"$tmpb"
diff -u "$tmpa" "$tmpb" || true

exit 1

