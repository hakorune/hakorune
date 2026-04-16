#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
ALLOWLIST="$ROOT_DIR/tools/checks/phase29x_vm_hako_s6_vocab_allowlist.txt"
TARGET="$ROOT_DIR/src/runner/reference/vm_hako/subset_check/mod.rs"

cd "$ROOT_DIR"

echo "[vm-hako-s6-vocab-guard] checking vm-hako subset vocabulary inventory"

if ! command -v rg >/dev/null 2>&1; then
  echo "[vm-hako-s6-vocab-guard] ERROR: rg is required" >&2
  exit 2
fi

if [[ ! -f "$ALLOWLIST" ]]; then
  echo "[vm-hako-s6-vocab-guard] ERROR: allowlist missing: $ALLOWLIST"
  exit 1
fi

if [[ ! -f "$TARGET" ]]; then
  echo "[vm-hako-s6-vocab-guard] ERROR: target missing: $TARGET"
  exit 1
fi

tmp_expected="$(mktemp)"
tmp_actual="$(mktemp)"
cleanup() {
  rm -f "$tmp_expected" "$tmp_actual"
}
trap cleanup EXIT

awk 'NF && $1 !~ /^#/' "$ALLOWLIST" | sort -u >"$tmp_expected"

awk '
  # Depth-aware extraction:
  # - enter `match op {` inside `check_vm_hako_subset_json`
  # - only collect top-level arms (depth==1)
  # - tolerate multiline patterns and optional `if` guards
  function flush_arm(arm,    flat, pat, tok) {
    flat = arm
    gsub(/\r/, "", flat)
    gsub(/\n/, " ", flat)
    if (flat ~ /^[[:space:]]*$/) {
      return
    }
    if (flat ~ /^[[:space:]]*other[[:space:]]*=>/) {
      done = 1
      return
    }
    pat = flat
    sub(/=>.*/, "", pat)
    sub(/[[:space:]]+if[[:space:]].*$/, "", pat)
    while (match(pat, /"[a-z_]+"/)) {
      tok = substr(pat, RSTART + 1, RLENGTH - 2)
      print tok
      pat = substr(pat, RSTART + RLENGTH)
    }
  }

  /^[[:space:]]*(pub\([[:alnum:]_]+\)[[:space:]]+)?fn[[:space:]]+check_vm_hako_subset_json[[:space:]]*\(/ {
    in_fn = 1
  }

  {
    if (!in_fn) {
      next
    }

    if (!in_match && $0 ~ /match[[:space:]]+op[[:space:]]*\{/) {
      in_match = 1
      match_depth = 1
      arm = ""
      next
    }

    if (!in_match) {
      next
    }

    depth_before = match_depth
    if (depth_before == 1) {
      arm = arm $0 "\n"
      if (index($0, "=>") > 0) {
        flush_arm(arm)
        arm = ""
        if (done) {
          in_match = 0
          in_fn = 0
          next
        }
      }
    }

    tmp = $0
    opens = gsub(/\{/, "", tmp)
    closes = gsub(/\}/, "", tmp)
    match_depth += (opens - closes)
    if (match_depth <= 0) {
      in_match = 0
      in_fn = 0
    }
  }
' "$TARGET" | sort -u >"$tmp_actual"

if [[ ! -s "$tmp_actual" ]]; then
  echo "[vm-hako-s6-vocab-guard] ERROR: failed to collect subset op inventory from $TARGET"
  exit 1
fi

missing="$(comm -23 "$tmp_expected" "$tmp_actual" || true)"
if [[ -n "$missing" ]]; then
  echo "[vm-hako-s6-vocab-guard] ERROR: allowlisted op missing in subset inventory:"
  printf "%s\n" "$missing"
  exit 1
fi

unexpected="$(comm -13 "$tmp_expected" "$tmp_actual" || true)"
if [[ -n "$unexpected" ]]; then
  echo "[vm-hako-s6-vocab-guard] ERROR: subset inventory has unexpected op(s):"
  printf "%s\n" "$unexpected"
  exit 1
fi

for forbidden in debug_log; do
  if rg -q "^${forbidden}$" "$tmp_actual"; then
    echo "[vm-hako-s6-vocab-guard] ERROR: forbidden legacy op is still tracked: $forbidden"
    exit 1
  fi
done

echo "[vm-hako-s6-vocab-guard] ok"
