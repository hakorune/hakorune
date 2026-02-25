#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
ALLOWLIST="$ROOT_DIR/tools/checks/phase29x_vm_route_pin_allowlist.txt"

cd "$ROOT_DIR"

echo "[vm-route-pin-guard] checking fixed pin inventory for NYASH_VM_HAKO_PREFER_STRICT_DEV=0"

if ! command -v rg >/dev/null 2>&1; then
  echo "[vm-route-pin-guard] ERROR: rg is required" >&2
  exit 2
fi

if [[ ! -f "$ALLOWLIST" ]]; then
  echo "[vm-route-pin-guard] ERROR: allowlist missing: $ALLOWLIST"
  exit 1
fi

tmp_actual="$(mktemp)"
tmp_expected="$(mktemp)"
cleanup() {
  rm -f "$tmp_actual" "$tmp_expected"
}
trap cleanup EXIT

HITS_DIRECT="$(rg -n --glob '*.sh' "NYASH_VM_HAKO_PREFER_STRICT_DEV=0" tools/smokes tools/selfhost -S || true)"
HITS_HELPER="$(rg -n --glob '*.sh' "\\b(run_with_vm_route_pin|run_hermetic_vm_with_route_pin|export_vm_route_pin)\\b" tools/smokes tools/selfhost -S || true)"
HITS="$(
  {
    printf "%s\n" "$HITS_DIRECT"
    printf "%s\n" "$HITS_HELPER"
  } \
  | rg -v '^tools/smokes/v2/lib/vm_route_pin.sh:' \
  | sed '/^$/d'
)"
if [[ -z "$HITS" ]]; then
  echo "[vm-route-pin-guard] ERROR: no pin callsite found in tools/smokes or tools/selfhost"
  exit 1
fi

printf "%s\n" "$HITS" | cut -d: -f1 | sort -u >"$tmp_actual"
awk 'NF && $1 !~ /^#/' "$ALLOWLIST" | sort -u >"$tmp_expected"

while IFS= read -r rel; do
  [[ -z "$rel" ]] && continue
  if [[ ! -f "$ROOT_DIR/$rel" ]]; then
    echo "[vm-route-pin-guard] ERROR: allowlist path missing: $rel"
    exit 1
  fi
done <"$tmp_expected"

unexpected="$(comm -13 "$tmp_expected" "$tmp_actual" || true)"
if [[ -n "$unexpected" ]]; then
  echo "[vm-route-pin-guard] ERROR: unexpected pin callsite(s):"
  printf "%s\n" "$unexpected"
  exit 1
fi

missing="$(comm -23 "$tmp_expected" "$tmp_actual" || true)"
if [[ -n "$missing" ]]; then
  echo "[vm-route-pin-guard] ERROR: allowlisted callsite missing pin assignment:"
  printf "%s\n" "$missing"
  exit 1
fi

SRC_HITS_RAW="$(rg -n --glob '*.rs' "NYASH_VM_HAKO_PREFER_STRICT_DEV=0" src -S || true)"
SRC_HITS="$(printf "%s\n" "$SRC_HITS_RAW" | awk -F: '
  NF >= 3 {
    text = $0
    sub("^[^:]*:[^:]*:", "", text)
    if (text !~ /^[[:space:]]*\/\//) {
      print $0
    }
  }
')"
if [[ -n "$SRC_HITS" ]]; then
  echo "[vm-route-pin-guard] ERROR: source lane contains hard pin assignment:"
  printf "%s\n" "$SRC_HITS"
  exit 1
fi

echo "[vm-route-pin-guard] ok"
