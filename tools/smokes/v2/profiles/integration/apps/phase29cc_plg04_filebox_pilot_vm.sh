#!/bin/bash
# Phase 29cc PLG-04-min6: FileBox wave-1 pilot smoke (VM)
# Contract:
# - FileBox is loaded via per-run nyash.toml.
# - FileBox method route (birth/open/read/close) works in VM.
# - Fixture prints file_read=FILEBOX_PILOT_OK and exits cleanly.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase29cc_plg04_filebox_pilot_vm"
FIXTURE="$NYASH_ROOT/apps/tests/phase29cc_plg04_filebox_pilot_min.hako"
WORK_DIR="$(mktemp -d -t phase29cc_plg04_filebox.XXXXXX)"
OUTPUT_FILE="$WORK_DIR/output.log"
INPUT_FILE="$WORK_DIR/phase29cc_filebox_input.txt"

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
FILEBOX_LIB_NAME="$(lib_name_for nyash_filebox_plugin "$EXT")"
FILEBOX_LIB_PATH="$NYASH_ROOT/plugins/nyash-filebox-plugin/$FILEBOX_LIB_NAME"

if [ ! -f "$FILEBOX_LIB_PATH" ]; then
  log_info "$SMOKE_NAME: filebox plugin not found, building release artifact"
  (cd "$NYASH_ROOT" && cargo build -p nyash-filebox-plugin --release >/dev/null)
fi

if [ ! -f "$FILEBOX_LIB_PATH" ]; then
  test_fail "$SMOKE_NAME: filebox plugin artifact missing ($FILEBOX_LIB_PATH)"
  exit 1
fi

cat > "$WORK_DIR/nyash.toml" << EOF2
[libraries."$FILEBOX_LIB_NAME"]
boxes = ["FileBox"]
path = "$FILEBOX_LIB_PATH"

[libraries."$FILEBOX_LIB_NAME".FileBox]
type_id = 6
abi_version = 1
singleton = false

[libraries."$FILEBOX_LIB_NAME".FileBox.methods]
birth = { method_id = 0 }
open = { method_id = 1 }
read = { method_id = 2 }
write = { method_id = 3 }
close = { method_id = 4 }
fini = { method_id = 4294967295 }
EOF2

printf 'FILEBOX_PILOT_OK' > "$INPUT_FILE"
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

if ! grep -q 'file_read=FILEBOX_PILOT_OK' "$OUTPUT_FILE"; then
  tail -n 80 "$OUTPUT_FILE" || true
  test_fail "$SMOKE_NAME: expected output 'file_read=FILEBOX_PILOT_OK' not found"
  exit 1
fi

if grep -q 'Unknown Box type: FileBox' "$OUTPUT_FILE"; then
  tail -n 80 "$OUTPUT_FILE" || true
  test_fail "$SMOKE_NAME: stale unknown-box regression"
  exit 1
fi

test_pass "$SMOKE_NAME: PASS (FileBox pilot locked)"
