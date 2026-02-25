#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
KEY_TOOL="$ROOT_DIR/tools/cache/phase29x_cache_keys.sh"
L1_TOOL="$ROOT_DIR/tools/cache/phase29x_l1_mir_cache.sh"

INPUT_PATH=""
PROFILE="strict-dev"
BACKEND="llvm"
TARGET="native"
CACHE_ROOT="$ROOT_DIR/target/hako-cache/v1"
ABI_BOUNDARY_DIGEST=""

usage() {
  cat <<'USAGE'
Usage:
  tools/cache/phase29x_l2_object_cache.sh --input <module.hako> [options]

Options:
  --profile <name>               Profile label (default: strict-dev)
  --backend <name>               Backend label (default: llvm)
  --target <name>                Target label (default: native)
  --cache-root <path>            Cache root (default: target/hako-cache/v1)
  --abi-boundary-digest <digest> ABI digest override for object key probe
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
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "phase29x_l2_object_cache: unknown option: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if [ -z "$INPUT_PATH" ]; then
  echo "phase29x_l2_object_cache: --input is required" >&2
  usage >&2
  exit 2
fi

if [ ! -f "$INPUT_PATH" ]; then
  echo "phase29x_l2_object_cache: input not found: $INPUT_PATH" >&2
  exit 2
fi

if [ ! -x "$KEY_TOOL" ]; then
  echo "phase29x_l2_object_cache: key tool missing: $KEY_TOOL" >&2
  exit 2
fi

if [ ! -x "$L1_TOOL" ]; then
  echo "phase29x_l2_object_cache: l1 tool missing: $L1_TOOL" >&2
  exit 2
fi

if [ -n "${NYASH_BIN:-}" ] && [ -x "${NYASH_BIN:-}" ]; then
  NYASH_BIN_PATH="$NYASH_BIN"
elif [ -x "$ROOT_DIR/target/release/hakorune" ]; then
  NYASH_BIN_PATH="$ROOT_DIR/target/release/hakorune"
elif [ -x "$ROOT_DIR/target/release/nyash" ]; then
  NYASH_BIN_PATH="$ROOT_DIR/target/release/nyash"
else
  echo "phase29x_l2_object_cache: nyash binary not found (target/release/hakorune|nyash)" >&2
  exit 2
fi

# L2 contract depends on L1 artifact existence.
L1_OUT="$("$L1_TOOL" --input "$INPUT_PATH" --profile "$PROFILE" --backend "$BACKEND" --target "$TARGET" --cache-root "$CACHE_ROOT")"
L1_STATUS="$(printf "%s\n" "$L1_OUT" | awk -F= '$1 == "cache_status" { print $2 }')"

KEY_ARGS=(--input "$INPUT_PATH" --profile "$PROFILE" --backend "$BACKEND" --target "$TARGET")
if [ -n "$ABI_BOUNDARY_DIGEST" ]; then
  KEY_ARGS+=(--abi-boundary-digest "$ABI_BOUNDARY_DIGEST")
fi
KEY_INFO="$("$KEY_TOOL" "${KEY_ARGS[@]}")"

key_of() {
  local name="$1"
  printf "%s\n" "$KEY_INFO" | awk -F= -v k="$name" '$1 == k { print $2 }'
}

MODULE_ID="$(key_of module_id)"
OBJECT_KEY="$(key_of object_key)"
MODULE_KEY="$(key_of module_compile_key)"

if [ -z "$MODULE_ID" ] || [ -z "$OBJECT_KEY" ] || [ -z "$MODULE_KEY" ]; then
  echo "phase29x_l2_object_cache: failed to derive module/object key" >&2
  exit 1
fi

CACHE_BASE="$CACHE_ROOT/$PROFILE/$TARGET"
OBJ_DIR="$CACHE_BASE/obj/$MODULE_ID"
OBJ_PATH="$OBJ_DIR/$OBJECT_KEY.o"
mkdir -p "$OBJ_DIR"

if [ -s "$OBJ_PATH" ]; then
  echo "[l2-cache] hit module=$MODULE_ID object_key=$OBJECT_KEY"
  cat <<EOF
cache_status=hit
l1_cache_status=$L1_STATUS
module_id=$MODULE_ID
module_compile_key=$MODULE_KEY
object_key=$OBJECT_KEY
object_path=$OBJ_PATH
EOF
  exit 0
fi

tmp_obj="$(mktemp)"
cleanup() {
  rm -f "$tmp_obj"
}
trap cleanup EXIT

set +e
env -i \
  PATH="$PATH" \
  HOME="${HOME:-}" \
  LD_LIBRARY_PATH="${LD_LIBRARY_PATH:-}" \
  NYASH_DISABLE_PLUGINS=1 \
  NYASH_LLVM_OBJ_OUT="$tmp_obj" \
  NYASH_LLVM_USE_HARNESS=1 \
  "$NYASH_BIN_PATH" --backend llvm "$INPUT_PATH" >/dev/null 2>&1
emit_rc=$?
set -e

if [ "$emit_rc" -ne 0 ] || [ ! -s "$tmp_obj" ]; then
  echo "[l2-cache] ERROR: object emit failed for $INPUT_PATH" >&2
  exit 1
fi

mv "$tmp_obj" "$OBJ_PATH"

echo "[l2-cache] miss module=$MODULE_ID object_key=$OBJECT_KEY"
cat <<EOF
cache_status=miss
l1_cache_status=$L1_STATUS
module_id=$MODULE_ID
module_compile_key=$MODULE_KEY
object_key=$OBJECT_KEY
object_path=$OBJ_PATH
EOF
