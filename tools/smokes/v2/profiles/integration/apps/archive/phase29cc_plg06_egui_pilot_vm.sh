#!/bin/bash
# Phase 29cc PLG-06-min4: Egui plugin wave-3 rollout pilot smoke (VM)
# Contract:
# - EguiBox is loaded via per-run nyash.toml.
# - open/uiLabel/pollEvent/run routes can be invoked in VM.
# - Fixture prints egui_ev= and egui_run=void then exits cleanly.

set -euo pipefail

source "$(dirname "$0")/../../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../../lib/plugin_pilot_common.sh"
require_env || exit 2

SMOKE_NAME="phase29cc_plg06_egui_pilot_vm"
FIXTURE="$NYASH_ROOT/apps/tests/phase29cc_plg06_egui_pilot_min.hako"
WORK_DIR="$(mktemp -d -t phase29cc_plg06_egui.XXXXXX)"
OUTPUT_FILE="$WORK_DIR/output.log"

cleanup() {
  rm -rf "$WORK_DIR"
}
trap cleanup EXIT

require_fixture_file "$SMOKE_NAME" "$FIXTURE" || exit 1

EXT="$(detect_lib_ext)"
EGUI_LIB_NAME="$(lib_name_for nyash_egui_plugin "$EXT")"
EGUI_LIB_PATH="$NYASH_ROOT/target/release/$EGUI_LIB_NAME"
STRING_LIB_NAME="$(lib_name_for nyash_string_plugin "$EXT")"
STRING_LIB_PATH="$NYASH_ROOT/target/release/$STRING_LIB_NAME"

build_plugin_release_checked "$SMOKE_NAME" "egui" "$EGUI_LIB_PATH" -p nyash-egui-plugin || exit 1
build_string_plugin_release_checked "$SMOKE_NAME" "$STRING_LIB_PATH" || exit 1

cat > "$WORK_DIR/nyash.toml" << EOF2
$(append_stringbox_toml "$STRING_LIB_NAME" "$STRING_LIB_PATH")

[libraries."$EGUI_LIB_NAME"]
boxes = ["EguiBox"]
path = "$EGUI_LIB_PATH"

[libraries."$EGUI_LIB_NAME".EguiBox]
type_id = 70
abi_version = 2
singleton = false

[libraries."$EGUI_LIB_NAME".EguiBox.methods]
birth = { method_id = 0 }
open = { method_id = 1, args = ["width", "height", "title"] }
uiLabel = { method_id = 2, args = ["text"] }
uiButton = { method_id = 3, args = ["text"] }
pollEvent = { method_id = 4, returns_result = true }
run = { method_id = 5, returns_result = true }
close = { method_id = 6 }
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

if ! grep -q 'egui_ev=' "$OUTPUT_FILE"; then
  tail -n 120 "$OUTPUT_FILE" || true
  test_fail "$SMOKE_NAME: expected output prefix 'egui_ev=' not found"
  exit 1
fi

if ! grep -q 'egui_run=void' "$OUTPUT_FILE"; then
  tail -n 120 "$OUTPUT_FILE" || true
  test_fail "$SMOKE_NAME: expected output 'egui_run=void' not found"
  exit 1
fi

if grep -q 'Unknown Box type: EguiBox' "$OUTPUT_FILE"; then
  tail -n 120 "$OUTPUT_FILE" || true
  test_fail "$SMOKE_NAME: stale unknown-box regression (EguiBox)"
  exit 1
fi

test_pass "$SMOKE_NAME: PASS (Egui plugin pilot locked)"
