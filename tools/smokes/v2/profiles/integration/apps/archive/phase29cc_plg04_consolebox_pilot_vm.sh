#!/bin/bash
# Phase 29cc PLG-04-min5: ConsoleBox wave-1 pilot smoke (VM)
# Contract:
# - ConsoleBox is loaded via per-run nyash.toml.
# - ConsoleBox method route (birth/println) works in VM.
# - Fixture prints consolebox_pilot_ok=1 and exits cleanly.

set -euo pipefail

source "$(dirname "$0")/../../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase29cc_plg04_consolebox_pilot_vm"
FIXTURE="$NYASH_ROOT/apps/tests/phase29cc_plg04_consolebox_pilot_min.hako"
WORK_DIR="$(mktemp -d -t phase29cc_plg04_consolebox.XXXXXX)"
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
CONSOLE_LIB_NAME="$(lib_name_for nyash_console_plugin "$EXT")"
CONSOLE_LIB_PATH="$NYASH_ROOT/plugins/nyash-console-plugin/$CONSOLE_LIB_NAME"

if [ ! -f "$CONSOLE_LIB_PATH" ]; then
  log_info "$SMOKE_NAME: console plugin not found, building release artifact"
  (cd "$NYASH_ROOT" && cargo build -p nyash-console-plugin --release >/dev/null)
fi

if [ ! -f "$CONSOLE_LIB_PATH" ]; then
  test_fail "$SMOKE_NAME: console plugin artifact missing ($CONSOLE_LIB_PATH)"
  exit 1
fi

cat > "$WORK_DIR/nyash.toml" << EOF
[libraries."$CONSOLE_LIB_NAME"]
boxes = ["ConsoleBox"]
path = "$CONSOLE_LIB_PATH"

[libraries."$CONSOLE_LIB_NAME".ConsoleBox]
type_id = 5
abi_version = 1
singleton = false

[libraries."$CONSOLE_LIB_NAME".ConsoleBox.methods]
birth = { method_id = 0 }
log = { method_id = 1 }
println = { method_id = 2 }
fini = { method_id = 4294967295 }
EOF

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

if ! grep -q 'consolebox_pilot_ok=1' "$OUTPUT_FILE"; then
  tail -n 80 "$OUTPUT_FILE" || true
  test_fail "$SMOKE_NAME: expected output 'consolebox_pilot_ok=1' not found"
  exit 1
fi

if grep -q 'Unknown Box type: ConsoleBox' "$OUTPUT_FILE"; then
  tail -n 80 "$OUTPUT_FILE" || true
  test_fail "$SMOKE_NAME: stale unknown-box regression"
  exit 1
fi

test_pass "$SMOKE_NAME: PASS (ConsoleBox pilot locked)"
