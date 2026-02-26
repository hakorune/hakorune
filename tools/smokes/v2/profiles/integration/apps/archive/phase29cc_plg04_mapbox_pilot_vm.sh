#!/bin/bash
# Phase 29cc PLG-04-min3: MapBox wave-1 pilot smoke (VM)
# Contract:
# - MapBox is loaded via per-run nyash.toml.
# - MapBox method route (birth/set/get/size) works in VM.
# - Fixture prints map_size=2 and map_g1=4 and exits cleanly.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase29cc_plg04_mapbox_pilot_vm"
FIXTURE="$NYASH_ROOT/apps/tests/phase29cc_plg04_mapbox_pilot_min.hako"
WORK_DIR="$(mktemp -d -t phase29cc_plg04_mapbox.XXXXXX)"
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
MAP_LIB_NAME="$(lib_name_for nyash_map_plugin "$EXT")"
MAP_LIB_PATH="$NYASH_ROOT/target/release/$MAP_LIB_NAME"

log_info "$SMOKE_NAME: building map plugin release artifact"
(cd "$NYASH_ROOT" && cargo build -p nyash-map-plugin --release >/dev/null)

if [ ! -f "$MAP_LIB_PATH" ]; then
  test_fail "$SMOKE_NAME: map plugin artifact missing ($MAP_LIB_PATH)"
  exit 1
fi

cat > "$WORK_DIR/nyash.toml" << EOF
[libraries."$MAP_LIB_NAME"]
boxes = ["MapBox"]
path = "$MAP_LIB_PATH"

[libraries."$MAP_LIB_NAME".MapBox]
type_id = 11
abi_version = 1
singleton = false

[libraries."$MAP_LIB_NAME".MapBox.methods]
birth = { method_id = 0 }
size = { method_id = 1 }
get = { method_id = 2 }
has = { method_id = 3 }
set = { method_id = 4 }
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

if ! grep -q 'map_size=2' "$OUTPUT_FILE"; then
  tail -n 80 "$OUTPUT_FILE" || true
  test_fail "$SMOKE_NAME: expected output 'map_size=2' not found"
  exit 1
fi

if ! grep -q 'map_g1=4' "$OUTPUT_FILE"; then
  tail -n 80 "$OUTPUT_FILE" || true
  test_fail "$SMOKE_NAME: expected output 'map_g1=4' not found"
  exit 1
fi

if grep -q 'Unknown Box type: MapBox' "$OUTPUT_FILE"; then
  tail -n 80 "$OUTPUT_FILE" || true
  test_fail "$SMOKE_NAME: stale unknown-box regression"
  exit 1
fi

test_pass "$SMOKE_NAME: PASS (MapBox pilot locked)"
