#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
L3_TOOL="$ROOT_DIR/tools/cache/phase29x_l3_link_cache.sh"
FIXTURE="$ROOT_DIR/apps/tests/hello_simple_llvm.hako"

if [ ! -x "$L3_TOOL" ]; then
  echo "[l3-cache-guard] ERROR: l3 tool missing or not executable: $L3_TOOL"
  exit 1
fi

if [ ! -f "$FIXTURE" ]; then
  echo "[l3-cache-guard] ERROR: fixture missing: $FIXTURE"
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

echo "[l3-cache-guard] run #1 (expect miss)"
"$L3_TOOL" --input "$FIXTURE" --cache-root "$CACHE_ROOT" --profile strict-dev --backend llvm --target native >"$tmp1"
status1="$(extract_key cache_status "$tmp1")"
l1_status1="$(extract_key l1_cache_status "$tmp1")"
l2_status1="$(extract_key l2_cache_status "$tmp1")"
link_key1="$(extract_key link_key "$tmp1")"
manifest1="$(extract_key manifest_path "$tmp1")"
binary1="$(extract_key binary_path "$tmp1")"
obj_key1="$(extract_key object_key "$tmp1")"

if [ "$status1" != "miss" ]; then
  echo "[l3-cache-guard] ERROR: first run expected miss, got: $status1"
  exit 1
fi
if [ "$l1_status1" != "miss" ] && [ "$l1_status1" != "hit" ]; then
  echo "[l3-cache-guard] ERROR: unexpected l1 status on first run: $l1_status1"
  exit 1
fi
if [ "$l2_status1" != "miss" ] && [ "$l2_status1" != "hit" ]; then
  echo "[l3-cache-guard] ERROR: unexpected l2 status on first run: $l2_status1"
  exit 1
fi
if [ -z "$link_key1" ] || [ -z "$manifest1" ] || [ -z "$binary1" ]; then
  echo "[l3-cache-guard] ERROR: missing link outputs on first run"
  exit 1
fi
if [ ! -s "$manifest1" ] || [ ! -s "$binary1" ]; then
  echo "[l3-cache-guard] ERROR: first run did not materialize link artifacts"
  exit 1
fi

echo "[l3-cache-guard] run #2 (expect hit)"
"$L3_TOOL" --input "$FIXTURE" --cache-root "$CACHE_ROOT" --profile strict-dev --backend llvm --target native >"$tmp2"
status2="$(extract_key cache_status "$tmp2")"
l1_status2="$(extract_key l1_cache_status "$tmp2")"
l2_status2="$(extract_key l2_cache_status "$tmp2")"
link_key2="$(extract_key link_key "$tmp2")"
manifest2="$(extract_key manifest_path "$tmp2")"
binary2="$(extract_key binary_path "$tmp2")"

if [ "$status2" != "hit" ]; then
  echo "[l3-cache-guard] ERROR: second run expected hit, got: $status2"
  exit 1
fi
if [ "$l1_status2" != "hit" ] || [ "$l2_status2" != "hit" ]; then
  echo "[l3-cache-guard] ERROR: second run expected l1/l2 hit, got l1=$l1_status2 l2=$l2_status2"
  exit 1
fi
if [ "$link_key1" != "$link_key2" ] || [ "$manifest1" != "$manifest2" ] || [ "$binary1" != "$binary2" ]; then
  echo "[l3-cache-guard] ERROR: link key/path changed between miss/hit runs"
  exit 1
fi

echo "[l3-cache-guard] run #3 (runtime ABI diff expect miss + changed link key)"
"$L3_TOOL" --input "$FIXTURE" --cache-root "$CACHE_ROOT" --profile strict-dev --backend llvm --target native --runtime-abi-digest runtime-abi-override-v2 >"$tmp3"
status3="$(extract_key cache_status "$tmp3")"
l2_status3="$(extract_key l2_cache_status "$tmp3")"
link_key3="$(extract_key link_key "$tmp3")"
manifest3="$(extract_key manifest_path "$tmp3")"
binary3="$(extract_key binary_path "$tmp3")"
obj_key3="$(extract_key object_key "$tmp3")"

if [ "$status3" != "miss" ]; then
  echo "[l3-cache-guard] ERROR: runtime ABI diff run expected miss, got: $status3"
  exit 1
fi
if [ "$l2_status3" != "hit" ]; then
  echo "[l3-cache-guard] ERROR: runtime ABI diff run expected l2 hit, got: $l2_status3"
  exit 1
fi
if [ "$obj_key1" != "$obj_key3" ]; then
  echo "[l3-cache-guard] ERROR: runtime ABI diff should not mutate object key"
  exit 1
fi
if [ "$link_key1" = "$link_key3" ] || [ "$manifest1" = "$manifest3" ] || [ "$binary1" = "$binary3" ]; then
  echo "[l3-cache-guard] ERROR: link key/path unchanged across runtime ABI diff"
  exit 1
fi
if [ ! -s "$manifest3" ] || [ ! -s "$binary3" ]; then
  echo "[l3-cache-guard] ERROR: runtime ABI diff run did not materialize link artifacts"
  exit 1
fi

echo "[l3-cache-guard] stable link_key=$link_key1"
echo "[l3-cache-guard] runtime-abi-diff link_key=$link_key3"
echo "[l3-cache-guard] ok"
