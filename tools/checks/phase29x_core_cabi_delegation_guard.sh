#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
ALLOWLIST="$ROOT_DIR/tools/checks/phase29x_core_cabi_delegation_allowlist.txt"

cd "$ROOT_DIR"

echo "[core-cabi-delegation-guard] checking canonical Core C ABI delegation ownership"

if ! command -v rg >/dev/null 2>&1; then
  echo "[core-cabi-delegation-guard] ERROR: rg is required" >&2
  exit 2
fi

if [[ ! -f "$ALLOWLIST" ]]; then
  echo "[core-cabi-delegation-guard] ERROR: allowlist missing: $ALLOWLIST"
  exit 1
fi

if ! bash "$ROOT_DIR/tools/checks/abi_lane_guard.sh" >/dev/null; then
  echo "[core-cabi-delegation-guard] ERROR: abi lane guard failed"
  exit 1
fi
if ! bash "$ROOT_DIR/tools/checks/nyrt_core_cabi_surface_guard.sh" >/dev/null; then
  echo "[core-cabi-delegation-guard] ERROR: core cabi surface guard failed"
  exit 1
fi

tmp_allow="$(mktemp)"
tmp_files="$(mktemp)"
cleanup() {
  rm -f "$tmp_allow" "$tmp_files"
}
trap cleanup EXIT

awk 'NF && $1 !~ /^#/' "$ALLOWLIST" | sort -u >"$tmp_allow"
while IFS= read -r rel; do
  [[ -z "$rel" ]] && continue
  if [[ ! -f "$ROOT_DIR/$rel" ]]; then
    echo "[core-cabi-delegation-guard] ERROR: allowlist path missing: $rel"
    exit 1
  fi
done <"$tmp_allow"

SYMS=(
  nyrt_load_mir_json
  nyrt_exec_main
  nyrt_verify_mir_json
  nyrt_safety_check_mir_json
  nyrt_handle_retain_h
  nyrt_handle_release_h
)

for sym in "${SYMS[@]}"; do
  hits="$(rg -n "\\b${sym}\\b" src include -S || true)"
  if [[ -z "$hits" ]]; then
    echo "[core-cabi-delegation-guard] ERROR: symbol not found in src/include: $sym"
    exit 1
  fi

  printf "%s\n" "$hits" | cut -d: -f1 | sort -u >"$tmp_files"
  unexpected="$(comm -13 "$tmp_allow" "$tmp_files" || true)"
  if [[ -n "$unexpected" ]]; then
    echo "[core-cabi-delegation-guard] ERROR: non-canonical delegation owner for $sym:"
    printf "%s\n" "$unexpected"
    echo "[core-cabi-delegation-guard] offending hits:"
    printf "%s\n" "$hits"
    exit 1
  fi
done

echo "[core-cabi-delegation-guard] ok"
