#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-370-ARRAY-SLOT-NATIVEDIRECT-SELECTED-METHOD-SEMANTIC-SMOKE.md"
STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"

require_line() {
  local file="$1"
  local needle="$2"
  if ! grep -Fq "$needle" "$file"; then
    echo "[row370-array-slot-nativedirect-semantic-smoke] missing line in ${file#$ROOT_DIR/}: $needle" >&2
    exit 1
  fi
}

require_line "$DOC" "output_contract=array-slot-nativedirect-selected-method-semantic-smoke-v0"
require_line "$DOC" "input_contract=array-slot-nativedirect-selected-method-lowering-implementation-v0"
require_line "$DOC" "python_lowering_smoke=ok"
require_line "$DOC" "rust_direct_array_substrate_smoke=ok"
require_line "$DOC" "direct_array_append_oob_storage_smoke=ok"
require_line "$DOC" "public_arraybox_handle_separation_smoke=ok"
require_line "$DOC" "legacy_retirement_now=0"
require_line "$DOC" "selected_next=array_slot_nativedirect_post_semantic_perf_owner_refresh"
require_line "$DOC" "summary=ok"

PYTHONPATH="$ROOT_DIR/src/llvm_py:$ROOT_DIR" \
  python3 -m unittest "$ROOT_DIR/src/llvm_py/tests/test_collection_method_call.py"

(
  cd "$ROOT_DIR"
  HAKO_ARRAY_SLOT_STORE=direct_array_i64_exact \
    cargo test -p nyash_kernel direct_array_i64 --lib -- --nocapture
)

require_line "$STATE" "latest_card = \"296x-370-ARRAY-SLOT-NATIVEDIRECT-SELECTED-METHOD-SEMANTIC-SMOKE\""
require_line "$STATE" "current_blocker_token = \"ARRAY-SLOT-NATIVEDIRECT-POST-SEMANTIC-PERF-OWNER-REFRESH-296X-001\""

echo "[row370-array-slot-nativedirect-semantic-smoke] ok"
