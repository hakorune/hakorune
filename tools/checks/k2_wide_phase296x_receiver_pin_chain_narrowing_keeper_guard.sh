#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

APP="apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
ATTR_TOOL="tools/allocator/mir_callsite_copy_attribution.py"
RECV_TOOL="tools/allocator/hako_mimalloc_receiver_materialization_attribution_probe.py"
CARD="docs/development/current/main/phases/phase-296x/296x-185-RECEIVER-PIN-CHAIN-NARROWING-KEEPER.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-184-RECEIVER-PIN-CHAIN-POLICY-SELECTION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
LOCAL="src/mir/builder/ssa/local.rs"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_receiver_pin_chain_narrowing_keeper_guard.sh"

[[ -f "$CARD" ]] || { echo "[row185-receiver-pin] missing card: $CARD" >&2; exit 1; }
[[ -f "$ATTR_TOOL" ]] || { echo "[row185-receiver-pin] missing attribution tool: $ATTR_TOOL" >&2; exit 1; }
[[ -f "$RECV_TOOL" ]] || { echo "[row185-receiver-pin] missing receiver tool: $RECV_TOOL" >&2; exit 1; }

grep -q '^Status: Current$' "$CARD" || { echo "[row185-receiver-pin] row185 card must be Current" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[row185-receiver-pin] row184 card must be Landed" >&2; exit 1; }
grep -q 'latest_card = "296x-185-RECEIVER-PIN-CHAIN-NARROWING-KEEPER"' "$STATE" || { echo "[row185-receiver-pin] CURRENT_STATE latest_card must point to row185" >&2; exit 1; }
grep -q 'current_blocker_token = "RECEIVER-PIN-CHAIN-NARROWING-KEEPER-296X-001"' "$STATE" || { echo "[row185-receiver-pin] CURRENT_STATE blocker must point to row185" >&2; exit 1; }
grep -q '| 184 | `RECEIVER-PIN-CHAIN-POLICY-SELECTION-296X-001` | Landed |' "$TASKBOARD" || { echo "[row185-receiver-pin] taskboard row184 must be Landed" >&2; exit 1; }
grep -q '| 185 | `RECEIVER-PIN-CHAIN-NARROWING-KEEPER-296X-001` | Current |' "$TASKBOARD" || { echo "[row185-receiver-pin] taskboard row185 must be Current" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[row185-receiver-pin] check index missing guard entry" >&2; exit 1; }

grep -q 'can_forward_same_block_copy_to_receiver' "$LOCAL" || { echo "[row185-receiver-pin] local.rs missing receiver copy forwarding policy" >&2; exit 1; }
grep -q 'matches!(self, LocalKind::Recv)' "$LOCAL" || { echo "[row185-receiver-pin] receiver copy forwarding must stay limited to Recv" >&2; exit 1; }
grep -q 'matches!(def_inst, Some(MirInstruction::Copy' "$LOCAL" || { echo "[row185-receiver-pin] receiver forwarding must stay limited to same-block Copy defs" >&2; exit 1; }

tmp_dir="$(mktemp -d /tmp/hakorune_row185_receiver_pin.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/app.mir.json"
attr_report="$tmp_dir/attr.out"
recv_report="$tmp_dir/recv.out"
proof_report="$tmp_dir/proof.out"

NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 \
  target/release/hakorune --backend mir --emit-mir-json "$mir_json" "$APP" >/dev/null
python3 "$ATTR_TOOL" --mir-json "$mir_json" --out "$attr_report"
python3 "$RECV_TOOL" --mir-json "$mir_json" --out "$recv_report"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row185-receiver-pin] missing report line: $expected" >&2
    cat "$file" >&2
    exit 1
  fi
}

require_line "$attr_report" "instruction_count=153"
require_line "$attr_report" "copy_count=61"
require_line "$attr_report" "receiver_copy_count=18"
require_line "$attr_report" "dominant_copy_owner=local_ssa_copy_materialization"
require_line "$recv_report" "receiver_attributed_copy_count=18"
require_line "$recv_report" "unique_receiver_copy_count=15"
require_line "$recv_report" "page_hotpath_receiver_copy_count=11"
require_line "$recv_report" "dominant_receiver_chain_len=1"
require_line "$recv_report" "summary=ok"

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

echo "[row185-receiver-pin] ok"
