#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

APP="apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
ATTR_TOOL="tools/allocator/mir_callsite_copy_attribution.py"
LOCAL_TOOL="tools/allocator/mir_local_ssa_copy_position_probe.py"
CARD="docs/development/current/main/phases/phase-296x/296x-182-FIELD-GET-DIRECT-CONSUMER-FORWARDING-KEEPER-DESIGN.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-181-FIELD-GET-DIRECT-CONSUMER-FORWARDING-CANDIDATE-PROBE.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
LOCAL="src/mir/builder/ssa/local.rs"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_field_get_direct_consumer_forwarding_keeper_guard.sh"

[[ -f "$CARD" ]] || { echo "[row182-field-get-forwarding] missing card: $CARD" >&2; exit 1; }
[[ -f "$ATTR_TOOL" ]] || { echo "[row182-field-get-forwarding] missing attribution tool: $ATTR_TOOL" >&2; exit 1; }
[[ -f "$LOCAL_TOOL" ]] || { echo "[row182-field-get-forwarding] missing local tool: $LOCAL_TOOL" >&2; exit 1; }

grep -q '^Status: Current$' "$CARD" || { echo "[row182-field-get-forwarding] row182 card must be Current" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[row182-field-get-forwarding] row181 card must be Landed" >&2; exit 1; }
grep -q 'latest_card = "296x-182-FIELD-GET-DIRECT-CONSUMER-FORWARDING-KEEPER-DESIGN"' "$STATE" || { echo "[row182-field-get-forwarding] CURRENT_STATE latest_card must point to row182" >&2; exit 1; }
grep -q 'current_blocker_token = "FIELD-GET-DIRECT-CONSUMER-FORWARDING-KEEPER-DESIGN-296X-001"' "$STATE" || { echo "[row182-field-get-forwarding] CURRENT_STATE blocker must point to row182" >&2; exit 1; }
grep -q '| 181 | `FIELD-GET-DIRECT-CONSUMER-FORWARDING-CANDIDATE-PROBE-296X-001` | Landed |' "$TASKBOARD" || { echo "[row182-field-get-forwarding] taskboard row181 must be Landed" >&2; exit 1; }
grep -q '| 182 | `FIELD-GET-DIRECT-CONSUMER-FORWARDING-KEEPER-DESIGN-296X-001` | Current |' "$TASKBOARD" || { echo "[row182-field-get-forwarding] taskboard row182 must be Current" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[row182-field-get-forwarding] check index missing guard entry" >&2; exit 1; }

grep -q 'can_forward_same_block_field_get_to_consumer' "$LOCAL" || { echo "[row182-field-get-forwarding] local.rs missing narrow forwarding policy" >&2; exit 1; }
grep -q 'matches!(self, LocalKind::Arg | LocalKind::CompareOperand)' "$LOCAL" || { echo "[row182-field-get-forwarding] forwarding must stay limited to Arg/CompareOperand" >&2; exit 1; }
grep -q 'matches!(def_inst, Some(MirInstruction::FieldGet' "$LOCAL" || { echo "[row182-field-get-forwarding] forwarding must stay limited to FieldGet defs" >&2; exit 1; }

tmp_dir="$(mktemp -d /tmp/hakorune_row182_field_get_forwarding.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/app.mir.json"
attr_report="$tmp_dir/attr.out"
local_report="$tmp_dir/local.out"
proof_report="$tmp_dir/proof.out"

NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 \
  target/release/hakorune --backend mir --emit-mir-json "$mir_json" "$APP" >/dev/null
python3 "$ATTR_TOOL" --mir-json "$mir_json" --out "$attr_report"
python3 "$LOCAL_TOOL" --mir-json "$mir_json" --out "$local_report"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row182-field-get-forwarding] missing report line: $expected" >&2
    cat "$file" >&2
    exit 1
  fi
}

require_line "$attr_report" "instruction_count=162"
require_line "$attr_report" "copy_count=70"
require_line "$attr_report" "local_ssa_copy_count=20"
require_line "$attr_report" "dominant_copy_owner=receiver_materialization"
require_line "$local_report" "expression_materialization_copy_count=9"
require_line "$local_report" "field_set_value_copy_count=0"
require_line "$local_report" "summary=ok"

timeout 180s bash tools/allocator/hako_exe_memory_runner.sh \
  --app "$APP" \
  --workload representative-object-lifecycle-small-block-v0 \
  --runtime-config empty \
  --operation-repeat 1 \
  --out "$proof_report" >/dev/null

require_line "$proof_report" "allocation_count=524288"
require_line "$proof_report" "free_count=524288"
require_line "$proof_report" "select_page_single_fast_path_count=524288"
require_line "$proof_report" "select_page_single_fallback_count=0"
require_line "$proof_report" "release_known_page_fast_path_count=524288"
require_line "$proof_report" "release_known_page_fallback_count=0"
require_line "$proof_report" "summary=ok"

echo "[row182-field-get-forwarding] ok"
