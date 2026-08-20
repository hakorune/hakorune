#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

require_text() {
  local file="$1"
  local text="$2"
  grep -Fq "$text" "$file" || {
    echo "[script-direct-static-target] missing '$text' in $file" >&2
    exit 1
  }
}

MODULE=src/mir/source_call_target/script_direct_static.rs
TESTS=src/mir/source_call_target/script_direct_static_tests.rs
ADMISSION=src/mir/builder/normal_script_root_demand_window.rs
LIFECYCLE=src/mir/builder/normal_default_root_catalog_lifecycle.rs
CARD=docs/development/current/main/investigations/script-direct-static-call-target-d0.md

require_text "$MODULE" "VerifiedScriptDirectStaticCallTargetInventoryV1"
require_text "$MODULE" "observe_script_method_calls_shadow_view_v0"
require_text "$MODULE" "TargetOutsideCatalog"
require_text "$ADMISSION" "attach_script_direct_static_targets"
require_text "$LIFECYCLE" "VerifiedScriptDirectStaticCallTargetInventoryV1::issue"
require_text "$CARD" "SCRIPT-DIRECT-STATIC-CALL-TARGET-I0"
require_text "$CARD" "The target catalog is a source product only."

for file in "$MODULE" "$TESTS" "$ADMISSION" "$LIFECYCLE"; do
  lines="$(wc -l < "$file")"
  if (( lines >= 760 )); then
    echo "[script-direct-static-target] source split required: $file has $lines lines" >&2
    exit 1
  fi
done

if rg -n "raw_root_body_recipe|JoinSig|lower_.*physical|emit_.*call" "$MODULE"; then
  echo "[script-direct-static-target] observation module crossed the Recipe/physical boundary" >&2
  exit 1
fi

CARGO_BUILD_JOBS=4 cargo test --profile quick -q -p nyash-rust \
  mir::source_call_target::script_direct_static_tests --lib

echo "[script-direct-static-target] OK"
