#!/bin/bash
# Phase 29cc PLG-06-min3: PythonParser plugin wave-3 rollout pilot smoke (VM)
# Contract:
# - PythonParserBox is loaded via per-run nyash.toml.
# - parse method route can be invoked in VM.
# - Fixture currently prints pyparser_out=void (known bridge contract) and exits cleanly.
# Debt tag:
# - [plg06/pyparser:return-bridge] parse result is not bridged to caller yet.
#   When fixed, replace this contract with non-void payload assertion.

set -euo pipefail

source "$(dirname "$0")/../../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../../lib/plugin_pilot_common.sh"
require_env || exit 2

SMOKE_NAME="phase29cc_plg06_pyparser_pilot_vm"
FIXTURE="$NYASH_ROOT/apps/tests/phase29cc_plg06_pyparser_pilot_min.hako"
WORK_DIR="$(mktemp -d -t phase29cc_plg06_pyparser.XXXXXX)"
OUTPUT_FILE="$WORK_DIR/output.log"

cleanup() {
  rm -rf "$WORK_DIR"
}
trap cleanup EXIT

require_fixture_file "$SMOKE_NAME" "$FIXTURE" || exit 1

EXT="$(detect_lib_ext)"
PYPARSER_LIB_NAME="$(lib_name_for nyash_python_parser_plugin "$EXT")"
PYPARSER_LIB_PATH="$NYASH_ROOT/target/release/$PYPARSER_LIB_NAME"
STRING_LIB_NAME="$(lib_name_for nyash_string_plugin "$EXT")"
STRING_LIB_PATH="$NYASH_ROOT/target/release/$STRING_LIB_NAME"

build_plugin_release_checked "$SMOKE_NAME" "python-parser" "$PYPARSER_LIB_PATH" -p nyash-python-parser-plugin || exit 1
build_string_plugin_release_checked "$SMOKE_NAME" "$STRING_LIB_PATH" || exit 1

cat > "$WORK_DIR/nyash.toml" << EOF2
$(append_stringbox_toml "$STRING_LIB_NAME" "$STRING_LIB_PATH")

[libraries."$PYPARSER_LIB_NAME"]
boxes = ["PythonParserBox"]
path = "$PYPARSER_LIB_PATH"

[libraries."$PYPARSER_LIB_NAME".PythonParserBox]
type_id = 60
abi_version = 2
singleton = false

[libraries."$PYPARSER_LIB_NAME".PythonParserBox.methods]
birth = { method_id = 0 }
parse = { method_id = 1, args = ["code"], returns_result = true }
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

if ! grep -q 'pyparser_out=void' "$OUTPUT_FILE"; then
  tail -n 120 "$OUTPUT_FILE" || true
  test_fail "$SMOKE_NAME: expected output 'pyparser_out=void' not found"
  exit 1
fi

if grep -q 'Unknown Box type: PythonParserBox' "$OUTPUT_FILE"; then
  tail -n 120 "$OUTPUT_FILE" || true
  test_fail "$SMOKE_NAME: stale unknown-box regression (PythonParserBox)"
  exit 1
fi

test_pass "$SMOKE_NAME: PASS (PythonParser plugin pilot locked)"
