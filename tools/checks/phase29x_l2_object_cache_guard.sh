#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
L2_TOOL="$ROOT_DIR/tools/cache/phase29x_l2_object_cache.sh"
FIXTURE="$ROOT_DIR/apps/tests/hello_simple_llvm.hako"

if [ ! -x "$L2_TOOL" ]; then
  echo "[l2-cache-guard] ERROR: l2 tool missing or not executable: $L2_TOOL"
  exit 1
fi

if [ ! -f "$FIXTURE" ]; then
  echo "[l2-cache-guard] ERROR: fixture missing: $FIXTURE"
  exit 1
fi

CACHE_ROOT="$(mktemp -d)"
tmp1="$(mktemp)"
tmp2="$(mktemp)"
tmp3="$(mktemp)"
cleanup() {
  rm -rf "$CACHE_ROOT"
  rm -f "$tmp1" "$tmp2" "$tmp3"
}
trap cleanup EXIT

extract_key() {
  local key_name="$1"
  local key_file="$2"
  awk -F= -v k="$key_name" '$1 == k { print $2 }' "$key_file"
}

echo "[l2-cache-guard] run #1 (expect miss)"
"$L2_TOOL" --input "$FIXTURE" --cache-root "$CACHE_ROOT" --profile strict-dev --backend llvm --target native >"$tmp1"
status1="$(extract_key cache_status "$tmp1")"
l1_status1="$(extract_key l1_cache_status "$tmp1")"
obj_key1="$(extract_key object_key "$tmp1")"
obj_path1="$(extract_key object_path "$tmp1")"

if [ "$status1" != "miss" ]; then
  echo "[l2-cache-guard] ERROR: first run expected miss, got: $status1"
  exit 1
fi
if [ -z "$obj_key1" ] || [ -z "$obj_path1" ] || [ ! -s "$obj_path1" ]; then
  echo "[l2-cache-guard] ERROR: first run did not materialize object artifact"
  exit 1
fi
if [ "$l1_status1" != "miss" ] && [ "$l1_status1" != "hit" ]; then
  echo "[l2-cache-guard] ERROR: unexpected l1 status on first run: $l1_status1"
  exit 1
fi

echo "[l2-cache-guard] run #2 (expect hit)"
"$L2_TOOL" --input "$FIXTURE" --cache-root "$CACHE_ROOT" --profile strict-dev --backend llvm --target native >"$tmp2"
status2="$(extract_key cache_status "$tmp2")"
l1_status2="$(extract_key l1_cache_status "$tmp2")"
obj_key2="$(extract_key object_key "$tmp2")"
obj_path2="$(extract_key object_path "$tmp2")"

if [ "$status2" != "hit" ]; then
  echo "[l2-cache-guard] ERROR: second run expected hit, got: $status2"
  exit 1
fi
if [ "$obj_key1" != "$obj_key2" ] || [ "$obj_path1" != "$obj_path2" ]; then
  echo "[l2-cache-guard] ERROR: object key/path changed between miss/hit runs"
  exit 1
fi
if [ "$l1_status2" != "hit" ]; then
  echo "[l2-cache-guard] ERROR: second run expected l1 hit, got: $l1_status2"
  exit 1
fi

echo "[l2-cache-guard] run #3 (ABI diff expect miss + changed object key)"
"$L2_TOOL" --input "$FIXTURE" --cache-root "$CACHE_ROOT" --profile strict-dev --backend llvm --target native --abi-boundary-digest abi-boundary-override-v2 >"$tmp3"
status3="$(extract_key cache_status "$tmp3")"
obj_key3="$(extract_key object_key "$tmp3")"
obj_path3="$(extract_key object_path "$tmp3")"

if [ "$status3" != "miss" ]; then
  echo "[l2-cache-guard] ERROR: ABI diff run expected miss, got: $status3"
  exit 1
fi
if [ "$obj_key1" = "$obj_key3" ] || [ "$obj_path1" = "$obj_path3" ]; then
  echo "[l2-cache-guard] ERROR: object key/path unchanged across ABI diff"
  exit 1
fi
if [ ! -s "$obj_path3" ]; then
  echo "[l2-cache-guard] ERROR: ABI diff run did not materialize object artifact"
  exit 1
fi

echo "[l2-cache-guard] stable object_key=$obj_key1"
echo "[l2-cache-guard] abi-diff object_key=$obj_key3"
echo "[l2-cache-guard] ok"
