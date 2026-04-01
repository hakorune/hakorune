#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
KEY_TOOL="$ROOT_DIR/tools/cache/phase29x_cache_keys.sh"
L2_TOOL="$ROOT_DIR/tools/cache/phase29x_l2_object_cache.sh"
BUILD_LLVM_TOOL="$ROOT_DIR/tools/build_llvm.sh"

INPUT_PATH=""
PROFILE="strict-dev"
BACKEND="llvm"
TARGET="native"
CACHE_ROOT="$ROOT_DIR/target/hako-cache/v1"
ABI_BOUNDARY_DIGEST=""
RUNTIME_ABI_DIGEST=""

usage() {
  cat <<'USAGE'
Usage:
  tools/cache/phase29x_l3_link_cache.sh --input <module.hako> [options]

Options:
  --profile <name>               Profile label (default: strict-dev)
  --backend <name>               Backend label (default: llvm)
  --target <name>                Target label (default: native)
  --cache-root <path>            Cache root (default: target/hako-cache/v1)
  --abi-boundary-digest <digest> ABI digest override
  --runtime-abi-digest <digest>  Runtime ABI digest override (link key)
  -h, --help                     Show help
USAGE
}

while [ $# -gt 0 ]; do
  case "$1" in
    --input)
      INPUT_PATH="${2:-}"
      shift 2
      ;;
    --profile)
      PROFILE="${2:-}"
      shift 2
      ;;
    --backend)
      BACKEND="${2:-}"
      shift 2
      ;;
    --target)
      TARGET="${2:-}"
      shift 2
      ;;
    --cache-root)
      CACHE_ROOT="${2:-}"
      shift 2
      ;;
    --abi-boundary-digest)
      ABI_BOUNDARY_DIGEST="${2:-}"
      shift 2
      ;;
    --runtime-abi-digest)
      RUNTIME_ABI_DIGEST="${2:-}"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "phase29x_l3_link_cache: unknown option: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if [ -z "$INPUT_PATH" ]; then
  echo "phase29x_l3_link_cache: --input is required" >&2
  usage >&2
  exit 2
fi

if [ ! -f "$INPUT_PATH" ]; then
  echo "phase29x_l3_link_cache: input not found: $INPUT_PATH" >&2
  exit 2
fi

if [ ! -f "$KEY_TOOL" ]; then
  echo "phase29x_l3_link_cache: key tool missing: $KEY_TOOL" >&2
  exit 2
fi

if [ ! -f "$L2_TOOL" ]; then
  echo "phase29x_l3_link_cache: l2 tool missing: $L2_TOOL" >&2
  exit 2
fi

if [ ! -f "$BUILD_LLVM_TOOL" ]; then
  echo "phase29x_l3_link_cache: build tool missing: $BUILD_LLVM_TOOL" >&2
  exit 2
fi

L2_ARGS=(--input "$INPUT_PATH" --profile "$PROFILE" --backend "$BACKEND" --target "$TARGET" --cache-root "$CACHE_ROOT")
if [ -n "$ABI_BOUNDARY_DIGEST" ]; then
  L2_ARGS+=(--abi-boundary-digest "$ABI_BOUNDARY_DIGEST")
fi
L2_OUT="$(bash "$L2_TOOL" "${L2_ARGS[@]}")"

l2_of() {
  local name="$1"
  printf "%s\n" "$L2_OUT" | awk -F= -v k="$name" '$1 == k { print $2 }'
}

L2_STATUS="$(l2_of cache_status)"
L1_STATUS="$(l2_of l1_cache_status)"
MODULE_ID="$(l2_of module_id)"
MODULE_KEY="$(l2_of module_compile_key)"
OBJECT_KEY="$(l2_of object_key)"
OBJECT_PATH="$(l2_of object_path)"

if [ -z "$MODULE_ID" ] || [ -z "$OBJECT_KEY" ] || [ -z "$OBJECT_PATH" ]; then
  echo "phase29x_l3_link_cache: failed to derive module/object from l2 output" >&2
  exit 1
fi

KEY_ARGS=(--input "$INPUT_PATH" --profile "$PROFILE" --backend "$BACKEND" --target "$TARGET" --entry-module "$MODULE_ID" --object-digests "$OBJECT_KEY")
if [ -n "$ABI_BOUNDARY_DIGEST" ]; then
  KEY_ARGS+=(--abi-boundary-digest "$ABI_BOUNDARY_DIGEST")
fi
if [ -n "$RUNTIME_ABI_DIGEST" ]; then
  KEY_ARGS+=(--runtime-abi-digest "$RUNTIME_ABI_DIGEST")
fi
KEY_INFO="$("$KEY_TOOL" "${KEY_ARGS[@]}")"

key_of() {
  local name="$1"
  printf "%s\n" "$KEY_INFO" | awk -F= -v k="$name" '$1 == k { print $2 }'
}

LINK_KEY="$(key_of link_key)"
RUNTIME_ABI_DIGEST_EFFECTIVE="$(key_of runtime_abi_digest)"
if [ -z "$LINK_KEY" ]; then
  echo "phase29x_l3_link_cache: failed to derive link key" >&2
  exit 1
fi

CACHE_BASE="$CACHE_ROOT/$PROFILE/$TARGET"
LINK_DIR="$CACHE_BASE/link/$MODULE_ID"
BIN_DIR="$CACHE_BASE/bin/$MODULE_ID/$LINK_KEY"
MANIFEST_PATH="$LINK_DIR/$LINK_KEY.manifest.json"
BINARY_PATH="$BIN_DIR/app"

mkdir -p "$LINK_DIR" "$BIN_DIR"

if [ -s "$BINARY_PATH" ] && [ -s "$MANIFEST_PATH" ]; then
  echo "[l3-cache] hit module=$MODULE_ID link_key=$LINK_KEY"
  cat <<EOF
cache_status=hit
l1_cache_status=$L1_STATUS
l2_cache_status=$L2_STATUS
module_id=$MODULE_ID
module_compile_key=$MODULE_KEY
object_key=$OBJECT_KEY
link_key=$LINK_KEY
manifest_path=$MANIFEST_PATH
binary_path=$BINARY_PATH
EOF
  exit 0
fi

tmp_bin="$(mktemp)"
tmp_manifest="$(mktemp)"
cleanup() {
  rm -f "$tmp_bin" "$tmp_manifest"
}
trap cleanup EXIT

set +e
NYASH_LLVM_SKIP_EMIT=1 NYASH_LLVM_OBJ_OUT="$OBJECT_PATH" \
  bash "$BUILD_LLVM_TOOL" "$INPUT_PATH" -o "$tmp_bin" >/dev/null 2>&1
link_rc=$?
set -e

if [ "$link_rc" -ne 0 ] || [ ! -s "$tmp_bin" ]; then
  echo "[l3-cache] ERROR: link failed for $INPUT_PATH" >&2
  exit 1
fi

cat >"$tmp_manifest" <<EOF
{
  "schema": "phase29x-l3-link-v1",
  "module_id": "$MODULE_ID",
  "module_compile_key": "$MODULE_KEY",
  "object_key": "$OBJECT_KEY",
  "object_path": "$OBJECT_PATH",
  "link_key": "$LINK_KEY",
  "runtime_abi_digest": "$RUNTIME_ABI_DIGEST_EFFECTIVE"
}
EOF

mv "$tmp_bin" "$BINARY_PATH"
mv "$tmp_manifest" "$MANIFEST_PATH"

echo "[l3-cache] miss module=$MODULE_ID link_key=$LINK_KEY"
cat <<EOF
cache_status=miss
l1_cache_status=$L1_STATUS
l2_cache_status=$L2_STATUS
module_id=$MODULE_ID
module_compile_key=$MODULE_KEY
object_key=$OBJECT_KEY
link_key=$LINK_KEY
manifest_path=$MANIFEST_PATH
binary_path=$BINARY_PATH
EOF
