#!/bin/bash
# Phase 29cc PLG-05-min4: Encoding plugin wave-2 pilot smoke (VM)
# Contract:
# - EncodingBox is loaded via per-run nyash.toml.
# - base64Encode/hexEncode method routes work in VM.
# - Fixture prints enc_b64=aGVsbG8= and enc_hex=68656c6c6f and exits cleanly.

set -euo pipefail

source "$(dirname "$0")/../../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase29cc_plg05_encoding_pilot_vm"
FIXTURE="$NYASH_ROOT/apps/tests/phase29cc_plg05_encoding_pilot_min.hako"
WORK_DIR="$(mktemp -d -t phase29cc_plg05_encoding.XXXXXX)"
OUTPUT_FILE="$WORK_DIR/output.log"

cleanup() {
  rm -rf "$WORK_DIR"
}
trap cleanup EXIT

detect_lib_ext() {
  case "$(uname -s)" in
    Darwin) echo "dylib" ;;
    MINGW*|MSYS*|CYGWIN*|Windows_NT) echo "dll" ;;
    *) echo "so" ;;
  esac
}

lib_name_for() {
  local base="$1"
  local ext="$2"
  if [ "$ext" = "dll" ]; then
    echo "${base}.dll"
  else
    echo "lib${base}.${ext}"
  fi
}

if [ ! -f "$FIXTURE" ]; then
  test_fail "$SMOKE_NAME: fixture missing ($FIXTURE)"
  exit 1
fi

EXT="$(detect_lib_ext)"
ENCODING_LIB_NAME="$(lib_name_for nyash_encoding_plugin "$EXT")"
ENCODING_LIB_PATH="$NYASH_ROOT/target/release/$ENCODING_LIB_NAME"
STRING_LIB_NAME="$(lib_name_for nyash_string_plugin "$EXT")"
STRING_LIB_PATH="$NYASH_ROOT/target/release/$STRING_LIB_NAME"

log_info "$SMOKE_NAME: building encoding plugin release artifact"
(cd "$NYASH_ROOT" && cargo build -p nyash-encoding-plugin --release >/dev/null)

if [ ! -f "$ENCODING_LIB_PATH" ]; then
  test_fail "$SMOKE_NAME: encoding plugin artifact missing ($ENCODING_LIB_PATH)"
  exit 1
fi

log_info "$SMOKE_NAME: building string plugin release artifact"
(cd "$NYASH_ROOT/plugins/nyash-string-plugin" && cargo build --release >/dev/null)

if [ ! -f "$STRING_LIB_PATH" ]; then
  test_fail "$SMOKE_NAME: string plugin artifact missing ($STRING_LIB_PATH)"
  exit 1
fi

cat > "$WORK_DIR/nyash.toml" << EOF2
[libraries."$STRING_LIB_NAME"]
boxes = ["StringBox"]
path = "$STRING_LIB_PATH"

[libraries."$STRING_LIB_NAME".StringBox]
type_id = 10
abi_version = 1
singleton = false

[libraries."$STRING_LIB_NAME".StringBox.methods]
birth = { method_id = 0 }
length = { method_id = 1 }
len = { method_id = 1 }
isEmpty = { method_id = 2 }
charCodeAt = { method_id = 3 }
concat = { method_id = 4 }
toUtf8 = { method_id = 6 }
toString = { method_id = 6 }
fini = { method_id = 4294967295 }

[libraries."$ENCODING_LIB_NAME"]
boxes = ["EncodingBox"]
path = "$ENCODING_LIB_PATH"

[libraries."$ENCODING_LIB_NAME".EncodingBox]
type_id = 53
abi_version = 1
singleton = false

[libraries."$ENCODING_LIB_NAME".EncodingBox.methods]
birth = { method_id = 0 }
toUtf8Bytes = { method_id = 1 }
fromUtf8Bytes = { method_id = 2 }
base64Encode = { method_id = 3 }
base64Decode = { method_id = 4 }
hexEncode = { method_id = 5 }
hexDecode = { method_id = 6 }
fini = { method_id = 4294967295 }
EOF2

cp "$FIXTURE" "$WORK_DIR/main.hako"

set +e
(
  cd "$WORK_DIR"
  NYASH_DISABLE_PLUGINS=0 \
  NYASH_VM_HAKO_PREFER_STRICT_DEV=0 \
  NYASH_VM_USE_FALLBACK=0 \
  "$NYASH_BIN" --backend vm ./main.hako >"$OUTPUT_FILE" 2>&1
)
RC=$?
set -e

if [ "$RC" -ne 0 ]; then
  tail -n 120 "$OUTPUT_FILE" || true
  test_fail "$SMOKE_NAME: vm run failed rc=$RC"
  exit 1
fi

if ! grep -q 'enc_b64=aGVsbG8=' "$OUTPUT_FILE"; then
  tail -n 120 "$OUTPUT_FILE" || true
  test_fail "$SMOKE_NAME: expected output 'enc_b64=aGVsbG8=' not found"
  exit 1
fi

if ! grep -q 'enc_hex=68656c6c6f' "$OUTPUT_FILE"; then
  tail -n 120 "$OUTPUT_FILE" || true
  test_fail "$SMOKE_NAME: expected output 'enc_hex=68656c6c6f' not found"
  exit 1
fi

if grep -q 'Unknown Box type: EncodingBox' "$OUTPUT_FILE"; then
  tail -n 120 "$OUTPUT_FILE" || true
  test_fail "$SMOKE_NAME: stale unknown-box regression (EncodingBox)"
  exit 1
fi

test_pass "$SMOKE_NAME: PASS (Encoding plugin pilot locked)"
