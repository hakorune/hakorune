#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$ROOT/target/release"
SRC_DIR="$ROOT/lang/c-abi/shims"
LOCK_DIR="$ROOT/target/perf_state/locks"
LOCK_FILE="$LOCK_DIR/hako_llvmc_ffi.build.lock"

mkdir -p "$OUT_DIR"
mkdir -p "$LOCK_DIR"

if command -v flock >/dev/null 2>&1; then
  exec 9>"$LOCK_FILE"
  flock 9
fi

cc_cmd=${CC:-cc}
uname_s="$(uname -s)"
out_name="libhako_llvmc_ffi.so"
link_mode="-shared"
extra_linker_flag=""
if [[ "$uname_s" == "Darwin" ]]; then
  out_name="libhako_llvmc_ffi.dylib"
  link_mode="-dynamiclib"
  extra_linker_flag="-Wl,-install_name,@rpath/libhako_llvmc_ffi.dylib"
elif [[ "$uname_s" == MINGW* || "$uname_s" == MSYS* || "$uname_s" == CYGWIN* ]]; then
  out_name="hako_llvmc_ffi.dll"
fi
out_path="$OUT_DIR/$out_name"

echo "[build] cc=$cc_cmd"
echo "[build] target=$uname_s"
echo "[build] compiling $out_name ..."

YYJSON_DIR="$ROOT/plugins/nyash-json-plugin/c/yyjson"

"$cc_cmd" -fPIC "$link_mode" \
  -I"$YYJSON_DIR" \
  ${extra_linker_flag:+$extra_linker_flag} \
  -o "$out_path" \
  "$SRC_DIR/hako_llvmc_ffi.c" \
  "$SRC_DIR/hako_aot.c" \
  "$SRC_DIR/hako_json_v1.c" \
  "$YYJSON_DIR/yyjson.c"

echo "[build] done: $out_path"
