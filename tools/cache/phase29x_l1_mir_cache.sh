#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
KEY_TOOL="$ROOT_DIR/tools/cache/phase29x_cache_keys.sh"
EMIT_ROUTE_TOOL="$ROOT_DIR/tools/smokes/v2/lib/emit_mir_route.sh"

INPUT_PATH=""
PROFILE="strict-dev"
BACKEND="llvm"
TARGET="native"
CACHE_ROOT="$ROOT_DIR/target/hako-cache/v1"

usage() {
  cat <<'USAGE'
Usage:
  tools/cache/phase29x_l1_mir_cache.sh --input <module.hako> [options]

Options:
  --profile <name>       Profile label for cache path/key (default: strict-dev)
  --backend <name>       Backend label for key derivation (default: llvm)
  --target <name>        Target label for cache path/key (default: native)
  --cache-root <path>    Cache root (default: target/hako-cache/v1)
  -h, --help             Show help
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
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "phase29x_l1_mir_cache: unknown option: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if [ -z "$INPUT_PATH" ]; then
  echo "phase29x_l1_mir_cache: --input is required" >&2
  usage >&2
  exit 2
fi

if [ ! -f "$INPUT_PATH" ]; then
  echo "phase29x_l1_mir_cache: input not found: $INPUT_PATH" >&2
  exit 2
fi

if [ ! -f "$KEY_TOOL" ]; then
  echo "phase29x_l1_mir_cache: key tool missing: $KEY_TOOL" >&2
  exit 2
fi

if [ -n "${NYASH_BIN:-}" ] && [ -x "${NYASH_BIN:-}" ]; then
  NYASH_BIN_PATH="$NYASH_BIN"
elif [ -x "$ROOT_DIR/target/release/hakorune" ]; then
  NYASH_BIN_PATH="$ROOT_DIR/target/release/hakorune"
elif [ -x "$ROOT_DIR/target/release/nyash" ]; then
  NYASH_BIN_PATH="$ROOT_DIR/target/release/nyash"
else
  echo "phase29x_l1_mir_cache: nyash binary not found (target/release/hakorune|nyash)" >&2
  exit 2
fi

KEY_INFO="$(bash "$KEY_TOOL" --input "$INPUT_PATH" --profile "$PROFILE" --backend "$BACKEND" --target "$TARGET")"

key_of() {
  local name="$1"
  printf "%s\n" "$KEY_INFO" | awk -F= -v k="$name" '$1 == k { print $2 }'
}

MODULE_ID="$(key_of module_id)"
MODULE_KEY="$(key_of module_compile_key)"
SOURCE_DIGEST="$(key_of source_digest)"
RESOLVER_DIGEST="$(key_of resolver_digest)"
DEPS_INTERFACE_DIGEST="$(key_of deps_interface_digest)"

if [ -z "$MODULE_ID" ] || [ -z "$MODULE_KEY" ]; then
  echo "phase29x_l1_mir_cache: failed to derive module id/key" >&2
  exit 1
fi

CACHE_BASE="$CACHE_ROOT/$PROFILE/$TARGET"
MIR_DIR="$CACHE_BASE/mir/$MODULE_ID"
ABI_DIR="$CACHE_BASE/abi/$MODULE_ID"
MIR_PATH="$MIR_DIR/$MODULE_KEY.mir.json"
ABI_PATH="$ABI_DIR/$MODULE_KEY.abi.json"

mkdir -p "$MIR_DIR" "$ABI_DIR"

if [ -s "$MIR_PATH" ] && [ -s "$ABI_PATH" ]; then
  echo "[l1-cache] hit module=$MODULE_ID key=$MODULE_KEY"
  cat <<EOF
cache_status=hit
module_id=$MODULE_ID
module_compile_key=$MODULE_KEY
mir_path=$MIR_PATH
abi_path=$ABI_PATH
EOF
  exit 0
fi

tmp_mir="$(mktemp)"
tmp_abi="$(mktemp)"
cleanup() {
  rm -f "$tmp_mir" "$tmp_abi"
}
trap cleanup EXIT

set +e
env -i \
  PATH="$PATH" \
  HOME="${HOME:-}" \
  LD_LIBRARY_PATH="${LD_LIBRARY_PATH:-}" \
  NYASH_DISABLE_PLUGINS=1 \
  "$NYASH_BIN_PATH" --emit-mir-json "$tmp_mir" "$INPUT_PATH" >/dev/null 2>&1
emit_rc=$?
set -e

if [ "$emit_rc" -ne 0 ] && [ -f "$EMIT_ROUTE_TOOL" ]; then
  set +e
  env -i \
    PATH="$PATH" \
    HOME="${HOME:-}" \
    LD_LIBRARY_PATH="${LD_LIBRARY_PATH:-}" \
    NYASH_BIN="$NYASH_BIN_PATH" \
    bash "$EMIT_ROUTE_TOOL" --route hako-helper --timeout-secs 30 --out "$tmp_mir" --input "$INPUT_PATH" >/dev/null 2>&1
  emit_rc=$?
  set -e
fi

if [ "$emit_rc" -ne 0 ]; then
  echo "[l1-cache] ERROR: MIR emit failed for $INPUT_PATH" >&2
  exit 1
fi

if ! rg -q '"functions"' "$tmp_mir"; then
  echo "[l1-cache] ERROR: emitted payload is not MIR JSON: $tmp_mir" >&2
  exit 1
fi

cat >"$tmp_abi" <<EOF
{
  "schema": "phase29x-l1-abi-v1",
  "module_id": "$MODULE_ID",
  "module_compile_key": "$MODULE_KEY",
  "source_digest": "$SOURCE_DIGEST",
  "resolver_digest": "$RESOLVER_DIGEST",
  "deps_interface_digest": "$DEPS_INTERFACE_DIGEST"
}
EOF

mv "$tmp_mir" "$MIR_PATH"
mv "$tmp_abi" "$ABI_PATH"

echo "[l1-cache] miss module=$MODULE_ID key=$MODULE_KEY"
cat <<EOF
cache_status=miss
module_id=$MODULE_ID
module_compile_key=$MODULE_KEY
mir_path=$MIR_PATH
abi_path=$ABI_PATH
EOF
