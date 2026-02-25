#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"

INPUT_PATH=""
MANIFEST_PATH=""
PROFILE="strict-dev"
BACKEND="llvm"
TARGET="native"
ENTRY_MODULE=""
DEPS_INTERFACE=""
OBJECT_DIGESTS=""
LINK_FLAGS=""
TOOLCHAIN_DIGEST=""
RESOLVER_DIGEST=""
ABI_BOUNDARY_DIGEST=""
RUNTIME_ABI_DIGEST=""
MIR_DIGEST=""

usage() {
  cat <<'USAGE'
Usage:
  tools/cache/phase29x_cache_keys.sh --input <module.hako> [options]

Options:
  --manifest <path>            Manifest path (default: hako.toml or nyash.toml)
  --profile <name>             Profile label (default: strict-dev)
  --backend <name>             Backend label (default: llvm)
  --target <name>              Target label (default: native)
  --entry-module <name>        Entry module name (default: derived from input)
  --deps-interface <csv>       Dependency interface digest list (default: empty)
  --object-digests <csv>       Ordered object digest list (default: object_key)
  --link-flags <text>          Link flags descriptor (default: empty)
  --toolchain-digest <digest>  Toolchain digest override (default: git HEAD)
  --resolver-digest <digest>   Resolver digest override
  --abi-boundary-digest <dig>  ABI boundary digest override
  --runtime-abi-digest <dig>   Runtime ABI digest override
  --mir-digest <dig>           MIR digest override (default: module_compile_key)
  -h, --help                   Show help
USAGE
}

hash_text() {
  local text="$1"
  if command -v sha256sum >/dev/null 2>&1; then
    printf "%s" "$text" | sha256sum | awk '{print $1}'
  elif command -v shasum >/dev/null 2>&1; then
    printf "%s" "$text" | shasum -a 256 | awk '{print $1}'
  else
    echo "phase29x_cache_keys: sha256 tool not found (sha256sum/shasum)" >&2
    exit 2
  fi
}

canon_csv() {
  local raw="${1:-}"
  if [ -z "$raw" ]; then
    echo ""
    return
  fi
  printf "%s" "$raw" \
    | tr ',' '\n' \
    | sed -e 's/^[[:space:]]*//' -e 's/[[:space:]]*$//' \
    | awk 'NF > 0' \
    | sort -u \
    | paste -sd',' -
}

to_abs_path() {
  local p="$1"
  if command -v realpath >/dev/null 2>&1; then
    realpath "$p"
  else
    readlink -f "$p"
  fi
}

module_name_from_path() {
  local abs="$1"
  local rel="$abs"
  if [[ "$abs" == "$ROOT_DIR/"* ]]; then
    rel="${abs#"$ROOT_DIR/"}"
  fi
  rel="${rel%.hako}"
  rel="${rel//\//.}"
  printf "%s" "$rel"
}

while [ $# -gt 0 ]; do
  case "$1" in
    --input)
      INPUT_PATH="${2:-}"
      shift 2
      ;;
    --manifest)
      MANIFEST_PATH="${2:-}"
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
    --entry-module)
      ENTRY_MODULE="${2:-}"
      shift 2
      ;;
    --deps-interface)
      DEPS_INTERFACE="${2:-}"
      shift 2
      ;;
    --object-digests)
      OBJECT_DIGESTS="${2:-}"
      shift 2
      ;;
    --link-flags)
      LINK_FLAGS="${2:-}"
      shift 2
      ;;
    --toolchain-digest)
      TOOLCHAIN_DIGEST="${2:-}"
      shift 2
      ;;
    --resolver-digest)
      RESOLVER_DIGEST="${2:-}"
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
    --mir-digest)
      MIR_DIGEST="${2:-}"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "phase29x_cache_keys: unknown option: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if [ -z "$INPUT_PATH" ]; then
  echo "phase29x_cache_keys: --input is required" >&2
  usage >&2
  exit 2
fi

if [ ! -f "$INPUT_PATH" ]; then
  echo "phase29x_cache_keys: input not found: $INPUT_PATH" >&2
  exit 2
fi

INPUT_ABS="$(to_abs_path "$INPUT_PATH")"
MODULE_ID="$(module_name_from_path "$INPUT_ABS")"
if [ -z "$ENTRY_MODULE" ]; then
  ENTRY_MODULE="$MODULE_ID"
fi

if [ -z "$MANIFEST_PATH" ]; then
  if [ -f "$ROOT_DIR/hako.toml" ]; then
    MANIFEST_PATH="$ROOT_DIR/hako.toml"
  elif [ -f "$ROOT_DIR/nyash.toml" ]; then
    MANIFEST_PATH="$ROOT_DIR/nyash.toml"
  else
    MANIFEST_PATH=""
  fi
fi

SOURCE_DIGEST="$(hash_text "$(cat "$INPUT_ABS")")"

if [ -n "$MANIFEST_PATH" ] && [ -f "$MANIFEST_PATH" ]; then
  MANIFEST_ABS="$(to_abs_path "$MANIFEST_PATH")"
  MANIFEST_DIGEST="$(hash_text "$(cat "$MANIFEST_ABS")")"
else
  MANIFEST_ABS="(missing)"
  MANIFEST_DIGEST="missing"
fi

if [ -z "$TOOLCHAIN_DIGEST" ]; then
  if git -C "$ROOT_DIR" rev-parse HEAD >/dev/null 2>&1; then
    TOOLCHAIN_DIGEST="$(git -C "$ROOT_DIR" rev-parse HEAD)"
  else
    TOOLCHAIN_DIGEST="nogit"
  fi
fi

PROFILE_SEED="profile=$PROFILE|NYASH_DEV=${NYASH_DEV:-}|NYASH_STAGE1_MODE=${NYASH_STAGE1_MODE:-}|NYASH_VM_HAKO_PREFER_STRICT_DEV=${NYASH_VM_HAKO_PREFER_STRICT_DEV:-}|NYASH_VM_USE_FALLBACK=${NYASH_VM_USE_FALLBACK:-}"
PROFILE_DIGEST="$(hash_text "$PROFILE_SEED")"

DEPS_INTERFACE_CANON="$(canon_csv "$DEPS_INTERFACE")"
if [ -z "$DEPS_INTERFACE_CANON" ]; then
  DEPS_INTERFACE_DIGEST="$(hash_text "none")"
else
  DEPS_INTERFACE_DIGEST="$(hash_text "$DEPS_INTERFACE_CANON")"
fi

if [ -z "$RESOLVER_DIGEST" ]; then
  RESOLVER_SEED="manifest=$MANIFEST_DIGEST|input=$INPUT_ABS|module=$MODULE_ID"
  RESOLVER_DIGEST="$(hash_text "$RESOLVER_SEED")"
fi

MODULE_COMPILE_KEY="$(hash_text "v1|source=$SOURCE_DIGEST|resolver=$RESOLVER_DIGEST|toolchain=$TOOLCHAIN_DIGEST|profile=$PROFILE_DIGEST|deps=$DEPS_INTERFACE_DIGEST")"

if [ -z "$MIR_DIGEST" ]; then
  MIR_DIGEST="$MODULE_COMPILE_KEY"
fi

BACKEND_DIGEST="$(hash_text "$BACKEND")"

if [ -z "$ABI_BOUNDARY_DIGEST" ]; then
  if [ -f "$ROOT_DIR/docs/reference/abi/ABI_BOUNDARY_MATRIX.md" ]; then
    ABI_BOUNDARY_DIGEST="$(hash_text "$(cat "$ROOT_DIR/docs/reference/abi/ABI_BOUNDARY_MATRIX.md")")"
  else
    ABI_BOUNDARY_DIGEST="$(hash_text "abi-boundary-missing")"
  fi
fi

TARGET_DIGEST="$(hash_text "$TARGET")"
OBJECT_KEY="$(hash_text "v1|mir=$MIR_DIGEST|backend=$BACKEND_DIGEST|abi=$ABI_BOUNDARY_DIGEST|target=$TARGET_DIGEST")"

if [ -z "$OBJECT_DIGESTS" ]; then
  ORDERED_OBJECT_LIST="$OBJECT_KEY"
else
  ORDERED_OBJECT_LIST="$(canon_csv "$OBJECT_DIGESTS")"
fi

LINK_FLAGS_DIGEST="$(hash_text "$LINK_FLAGS")"
if [ -z "$RUNTIME_ABI_DIGEST" ]; then
  RUNTIME_ABI_DIGEST="$ABI_BOUNDARY_DIGEST"
fi

LINK_KEY="$(hash_text "v1|entry=$ENTRY_MODULE|objects=$ORDERED_OBJECT_LIST|flags=$LINK_FLAGS_DIGEST|runtime_abi=$RUNTIME_ABI_DIGEST")"

cat <<EOF
module_id=$MODULE_ID
input_path=$INPUT_ABS
manifest_path=$MANIFEST_ABS
manifest_digest=$MANIFEST_DIGEST
source_digest=$SOURCE_DIGEST
resolver_digest=$RESOLVER_DIGEST
toolchain_digest=$TOOLCHAIN_DIGEST
profile_digest=$PROFILE_DIGEST
deps_interface_digest=$DEPS_INTERFACE_DIGEST
module_compile_key=$MODULE_COMPILE_KEY
mir_digest=$MIR_DIGEST
backend_digest=$BACKEND_DIGEST
abi_boundary_digest=$ABI_BOUNDARY_DIGEST
target_digest=$TARGET_DIGEST
object_key=$OBJECT_KEY
ordered_object_list=$ORDERED_OBJECT_LIST
link_flags_digest=$LINK_FLAGS_DIGEST
runtime_abi_digest=$RUNTIME_ABI_DIGEST
entry_module=$ENTRY_MODULE
link_key=$LINK_KEY
EOF
