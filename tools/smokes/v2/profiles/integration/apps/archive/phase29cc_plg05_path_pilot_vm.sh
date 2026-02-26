#!/bin/bash
# Phase 29cc PLG-05-min5: Path plugin wave-2 pilot smoke (VM)
# Contract:
# - PathBox is loaded via per-run nyash.toml.
# - join/extname/isAbs method routes work in VM.
# - Fixture prints path_join=a/b, path_ext=.txt, path_abs=false and exits cleanly.

set -euo pipefail

source "$(dirname "$0")/../../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../../lib/plugin_pilot_common.sh"
require_env || exit 2

SMOKE_NAME="phase29cc_plg05_path_pilot_vm"
FIXTURE="$NYASH_ROOT/apps/tests/phase29cc_plg05_path_pilot_min.hako"
WORK_DIR="$(mktemp -d -t phase29cc_plg05_path.XXXXXX)"
OUTPUT_FILE="$WORK_DIR/output.log"

cleanup() {
  rm -rf "$WORK_DIR"
}
trap cleanup EXIT

require_fixture_file "$SMOKE_NAME" "$FIXTURE" || exit 1

EXT="$(detect_lib_ext)"
PATH_LIB_NAME="$(lib_name_for nyash_path_plugin "$EXT")"
PATH_LIB_PATH="$NYASH_ROOT/target/release/$PATH_LIB_NAME"
STRING_LIB_NAME="$(lib_name_for nyash_string_plugin "$EXT")"
STRING_LIB_PATH="$NYASH_ROOT/target/release/$STRING_LIB_NAME"

build_plugin_release_checked "$SMOKE_NAME" "path" "$PATH_LIB_PATH" -p nyash-path-plugin || exit 1
build_string_plugin_release_checked "$SMOKE_NAME" "$STRING_LIB_PATH" || exit 1

cat > "$WORK_DIR/nyash.toml" << EOF2
$(append_stringbox_toml "$STRING_LIB_NAME" "$STRING_LIB_PATH")

[libraries."$PATH_LIB_NAME"]
boxes = ["PathBox"]
path = "$PATH_LIB_PATH"

[libraries."$PATH_LIB_NAME".PathBox]
type_id = 55
abi_version = 1
singleton = false

[libraries."$PATH_LIB_NAME".PathBox.methods]
birth = { method_id = 0 }
join = { method_id = 1 }
dirname = { method_id = 2 }
basename = { method_id = 3 }
extname = { method_id = 4 }
isAbs = { method_id = 5 }
normalize = { method_id = 6 }
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

if ! grep -q 'path_join=a/b' "$OUTPUT_FILE"; then
  tail -n 120 "$OUTPUT_FILE" || true
  test_fail "$SMOKE_NAME: expected output 'path_join=a/b' not found"
  exit 1
fi

if ! grep -q 'path_ext=.txt' "$OUTPUT_FILE"; then
  tail -n 120 "$OUTPUT_FILE" || true
  test_fail "$SMOKE_NAME: expected output 'path_ext=.txt' not found"
  exit 1
fi

if ! grep -q 'path_abs=false' "$OUTPUT_FILE"; then
  tail -n 120 "$OUTPUT_FILE" || true
  test_fail "$SMOKE_NAME: expected output 'path_abs=false' not found"
  exit 1
fi

if grep -q 'Unknown Box type: PathBox' "$OUTPUT_FILE"; then
  tail -n 120 "$OUTPUT_FILE" || true
  test_fail "$SMOKE_NAME: stale unknown-box regression (PathBox)"
  exit 1
fi

test_pass "$SMOKE_NAME: PASS (Path plugin pilot locked)"
