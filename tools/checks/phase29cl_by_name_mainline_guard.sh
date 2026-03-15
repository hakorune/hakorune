#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
ALLOWLIST="$ROOT_DIR/tools/checks/phase29cl_by_name_mainline_allowlist.txt"

cd "$ROOT_DIR"

echo "[phase29cl-by-name-mainline-guard] checking no-new-mainline invoke_by_name owners"

if ! command -v rg >/dev/null 2>&1; then
  echo "[phase29cl-by-name-mainline-guard] ERROR: rg is required" >&2
  exit 2
fi

if [[ ! -f "$ALLOWLIST" ]]; then
  echo "[phase29cl-by-name-mainline-guard] ERROR: allowlist missing: $ALLOWLIST" >&2
  exit 1
fi

tmp_actual="$(mktemp)"
tmp_expected="$(mktemp)"
cleanup() {
  rm -f "$tmp_actual" "$tmp_expected"
}
trap cleanup EXIT

PATTERN='invoke_by_name_i64|nyash\.plugin\.invoke_by_name_i64'
HITS="$(
  rg -n "$PATTERN" \
    crates/nyash_kernel/src \
    src/llvm_py \
    lang/c-abi/shims \
    -g '!src/llvm_py/tests/**' \
    -S || true
)"

if [[ -z "$HITS" ]]; then
  echo "[phase29cl-by-name-mainline-guard] ERROR: no invoke_by_name owner found" >&2
  exit 1
fi

printf "%s\n" "$HITS" | cut -d: -f1 | sort -u >"$tmp_actual"
awk 'NF && $1 !~ /^#/' "$ALLOWLIST" | sort -u >"$tmp_expected"

while IFS= read -r rel; do
  [[ -z "$rel" ]] && continue
  if [[ ! -f "$ROOT_DIR/$rel" ]]; then
    echo "[phase29cl-by-name-mainline-guard] ERROR: allowlist path missing: $rel" >&2
    exit 1
  fi
done <"$tmp_expected"

unexpected="$(comm -13 "$tmp_expected" "$tmp_actual" || true)"
if [[ -n "$unexpected" ]]; then
  echo "[phase29cl-by-name-mainline-guard] ERROR: unexpected invoke_by_name caller(s):" >&2
  printf "%s\n" "$unexpected" >&2
  echo "[phase29cl-by-name-mainline-guard] offending hits:" >&2
  printf "%s\n" "$HITS" >&2
  exit 1
fi

missing="$(comm -23 "$tmp_expected" "$tmp_actual" || true)"
if [[ -n "$missing" ]]; then
  echo "[phase29cl-by-name-mainline-guard] ERROR: allowlisted owner missing symbol:" >&2
  printf "%s\n" "$missing" >&2
  exit 1
fi

echo "[phase29cl-by-name-mainline-guard] ok"
