#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
L1_TOOL="$ROOT_DIR/tools/cache/phase29x_l1_mir_cache.sh"
FIXTURE="$ROOT_DIR/apps/tests/hello_simple_llvm.hako"

if [ ! -x "$L1_TOOL" ]; then
  echo "[l1-cache-guard] ERROR: l1 tool missing or not executable: $L1_TOOL"
  exit 1
fi

if [ ! -f "$FIXTURE" ]; then
  echo "[l1-cache-guard] ERROR: fixture missing: $FIXTURE"
  exit 1
fi

CACHE_ROOT="$(mktemp -d)"
tmp1="$(mktemp)"
tmp2="$(mktemp)"
cleanup() {
  rm -rf "$CACHE_ROOT"
  rm -f "$tmp1" "$tmp2"
}
trap cleanup EXIT

extract_key() {
  local key_name="$1"
  local key_file="$2"
  awk -F= -v k="$key_name" '$1 == k { print $2 }' "$key_file"
}

echo "[l1-cache-guard] run #1 (expect miss)"
"$L1_TOOL" --input "$FIXTURE" --cache-root "$CACHE_ROOT" --profile strict-dev --backend llvm --target native >"$tmp1"

status1="$(extract_key cache_status "$tmp1")"
key1="$(extract_key module_compile_key "$tmp1")"
mir1="$(extract_key mir_path "$tmp1")"
abi1="$(extract_key abi_path "$tmp1")"

if [ "$status1" != "miss" ]; then
  echo "[l1-cache-guard] ERROR: first run expected miss, got: $status1"
  exit 1
fi
if [ -z "$key1" ] || [ -z "$mir1" ] || [ -z "$abi1" ]; then
  echo "[l1-cache-guard] ERROR: first run output missing key/path"
  exit 1
fi
if [ ! -s "$mir1" ] || [ ! -s "$abi1" ]; then
  echo "[l1-cache-guard] ERROR: first run did not materialize artifacts"
  exit 1
fi

echo "[l1-cache-guard] run #2 (expect hit)"
"$L1_TOOL" --input "$FIXTURE" --cache-root "$CACHE_ROOT" --profile strict-dev --backend llvm --target native >"$tmp2"

status2="$(extract_key cache_status "$tmp2")"
key2="$(extract_key module_compile_key "$tmp2")"
mir2="$(extract_key mir_path "$tmp2")"
abi2="$(extract_key abi_path "$tmp2")"

if [ "$status2" != "hit" ]; then
  echo "[l1-cache-guard] ERROR: second run expected hit, got: $status2"
  exit 1
fi
if [ "$key1" != "$key2" ]; then
  echo "[l1-cache-guard] ERROR: module key changed between miss/hit runs"
  exit 1
fi
if [ "$mir1" != "$mir2" ] || [ "$abi1" != "$abi2" ]; then
  echo "[l1-cache-guard] ERROR: artifact paths changed between miss/hit runs"
  exit 1
fi

echo "[l1-cache-guard] stable module_compile_key=$key1"
echo "[l1-cache-guard] ok"
