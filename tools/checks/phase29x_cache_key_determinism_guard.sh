#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
KEY_TOOL="$ROOT_DIR/tools/cache/phase29x_cache_keys.sh"
FIXTURE="$ROOT_DIR/apps/tests/hello_simple_llvm.hako"

if [ ! -x "$KEY_TOOL" ]; then
  echo "[cache-key-guard] ERROR: key tool missing or not executable: $KEY_TOOL"
  exit 1
fi

if [ ! -f "$FIXTURE" ]; then
  echo "[cache-key-guard] ERROR: fixture missing: $FIXTURE"
  exit 1
fi

tmp1="$(mktemp)"
tmp2="$(mktemp)"
tmp3="$(mktemp)"
tmp4="$(mktemp)"
cleanup() {
  rm -f "$tmp1" "$tmp2" "$tmp3" "$tmp4"
}
trap cleanup EXIT

extract_key() {
  local key_name="$1"
  local key_file="$2"
  awk -F= -v k="$key_name" '$1 == k { print $2 }' "$key_file"
}

run_keygen() {
  local out_file="$1"
  shift
  "$KEY_TOOL" --input "$FIXTURE" "$@" >"$out_file"
}

echo "[cache-key-guard] run baseline #1"
run_keygen "$tmp1" \
  --profile strict-dev \
  --backend llvm \
  --target native \
  --entry-module apps.tests.hello_simple_llvm

echo "[cache-key-guard] run baseline #2"
run_keygen "$tmp2" \
  --profile strict-dev \
  --backend llvm \
  --target native \
  --entry-module apps.tests.hello_simple_llvm

for key in module_compile_key object_key link_key; do
  v1="$(extract_key "$key" "$tmp1")"
  v2="$(extract_key "$key" "$tmp2")"
  if [ -z "$v1" ] || [ -z "$v2" ]; then
    echo "[cache-key-guard] ERROR: missing key '$key' in baseline output"
    exit 1
  fi
  if [ "$v1" != "$v2" ]; then
    echo "[cache-key-guard] ERROR: nondeterministic key '$key'"
    echo "  #1: $v1"
    echo "  #2: $v2"
    exit 1
  fi
  echo "[cache-key-guard] stable $key=$v1"
done

echo "[cache-key-guard] run profile-diff probe"
run_keygen "$tmp3" \
  --profile release \
  --backend llvm \
  --target native \
  --entry-module apps.tests.hello_simple_llvm

base_module="$(extract_key module_compile_key "$tmp1")"
profile_module="$(extract_key module_compile_key "$tmp3")"
if [ "$base_module" = "$profile_module" ]; then
  echo "[cache-key-guard] ERROR: module_compile_key unchanged across profile change"
  exit 1
fi
echo "[cache-key-guard] profile-diff module_compile_key changed"

echo "[cache-key-guard] run abi-diff probe"
run_keygen "$tmp4" \
  --profile strict-dev \
  --backend llvm \
  --target native \
  --entry-module apps.tests.hello_simple_llvm \
  --abi-boundary-digest abi-boundary-override-v2

base_object="$(extract_key object_key "$tmp1")"
base_link="$(extract_key link_key "$tmp1")"
abi_object="$(extract_key object_key "$tmp4")"
abi_link="$(extract_key link_key "$tmp4")"

if [ "$base_object" = "$abi_object" ]; then
  echo "[cache-key-guard] ERROR: object_key unchanged across ABI change"
  exit 1
fi
if [ "$base_link" = "$abi_link" ]; then
  echo "[cache-key-guard] ERROR: link_key unchanged across ABI change"
  exit 1
fi
echo "[cache-key-guard] abi-diff object_key/link_key changed"

echo "[cache-key-guard] ok"
