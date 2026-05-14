#!/usr/bin/env bash
set -euo pipefail

card='docs/development/current/main/phases/phase-293x/293x-324-PACKED-002-SOURCE-PACKED-ARRAY-AUTOUSE-PILOT.md'
ssot='docs/development/current/main/design/source-packed-array-autouse-pilot-ssot.md'

require_text() {
  local file="$1"
  local text="$2"
  if ! grep -Fq "$text" "$file"; then
    echo "[source-packed-array-autouse] missing '$text' in $file" >&2
    exit 1
  fi
}

require_text "$card" "293x-324 PACKED-002 source PackedArray auto-use pilot"
require_text "$ssot" "Source PackedArray Auto-Use Pilot SSOT"
require_text docs/development/current/main/design/record-and-packed-array-lowering-ssot.md "PACKED-002"
require_text docs/reference/language/EBNF.md "PACKED-002 emits metadata-only source PackedArray auto-use pilot rows"
require_text src/mir/function/types.rs "SourcePackedArrayAutoUsePilotPlan"
require_text src/mir/source_packed_array_autouse_pilot.rs "boxed_fallback_enabled: false"
require_text src/mir/source_packed_array_autouse_pilot.rs "backend_lowering_enabled: false"
require_text src/mir/semantic_refresh.rs "refresh_module_source_packed_array_autouse_pilot_plans"
require_text src/mir/builder/module_lifecycle.rs "refresh_module_source_packed_array_autouse_pilot_plans"
require_text src/runner/json_v0_bridge/lowering.rs "refresh_module_source_packed_array_autouse_pilot_plans"
require_text src/runner/mir_json_emit/root.rs "source_packed_array_autouse_pilot_plans"
require_text src/runner/mir_json_emit/decls.rs "collect_source_packed_array_autouse_pilot_plan_values"
require_text docs/tools/check-scripts-index.md "k2_wide_source_packed_array_autouse_pilot_guard.sh"

cargo test -q source_packed_array_autouse_pilot --lib
cargo test -q source_to_program_json_v0_accepts_packed_array_integer_record_eligibility --lib

if rg -n 'source_packed_array_autouse.*backend_lowering_enabled: true|source_packed_array_autouse.*boxed_fallback_enabled: true|source_packed_array_autouse.*public_array_get_materialization_enabled: true' \
  src/mir src/runner docs/development/current/main/phases/phase-293x >/tmp/source-packed-array-autouse.enabled 2>&1; then
  echo "[source-packed-array-autouse] ERROR: PACKED-002 stop-line flag enabled" >&2
  cat /tmp/source-packed-array-autouse.enabled >&2
  rm -f /tmp/source-packed-array-autouse.enabled
  exit 1
fi
rm -f /tmp/source-packed-array-autouse.enabled

echo "[source-packed-array-autouse] OK"
