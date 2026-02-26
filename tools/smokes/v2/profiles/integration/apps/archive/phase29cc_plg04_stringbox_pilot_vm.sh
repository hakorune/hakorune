#!/bin/bash
# Phase 29cc PLG-04-min4: StringBox wave-1 pilot smoke (VM)
# Contract:
# - StringBox is loaded via per-run nyash.toml.
# - StringBox method route (birth/length) works in VM.
# - Fixture prints string_len=3 and string_len2=5 and exits cleanly.

set -euo pipefail

source "$(dirname "$0")/../../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase29cc_plg04_stringbox_pilot_vm"
FIXTURE="$NYASH_ROOT/apps/tests/phase29cc_plg04_stringbox_pilot_min.hako"
WORK_DIR="$(mktemp -d -t phase29cc_plg04_stringbox.XXXXXX)"
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
STRING_LIB_NAME="$(lib_name_for nyash_string_plugin "$EXT")"
STRING_LIB_PATH="$NYASH_ROOT/plugins/nyash-string-plugin/$STRING_LIB_NAME"

if [ ! -f "$STRING_LIB_PATH" ]; then
  log_info "$SMOKE_NAME: string plugin not found, building release artifact"
  (cd "$NYASH_ROOT" && cargo build -p nyash-string-plugin --release >/dev/null)
fi

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
charCodeAt = { method_id = 3 }
toUtf8 = { method_id = 6 }
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
  tail -n 80 "$OUTPUT_FILE" || true
  test_fail "$SMOKE_NAME: vm run failed rc=$RC"
  exit 1
fi

if ! grep -q 'string_len=3' "$OUTPUT_FILE"; then
  tail -n 80 "$OUTPUT_FILE" || true
  test_fail "$SMOKE_NAME: expected output 'string_len=3' not found"
  exit 1
fi

if ! grep -q 'string_len2=5' "$OUTPUT_FILE"; then
  tail -n 80 "$OUTPUT_FILE" || true
  test_fail "$SMOKE_NAME: expected output 'string_len2=5' not found"
  exit 1
fi

if grep -q 'Unknown Box type: StringBox' "$OUTPUT_FILE"; then
  tail -n 80 "$OUTPUT_FILE" || true
  test_fail "$SMOKE_NAME: stale unknown-box regression"
  exit 1
fi

test_pass "$SMOKE_NAME: PASS (StringBox pilot locked)"
