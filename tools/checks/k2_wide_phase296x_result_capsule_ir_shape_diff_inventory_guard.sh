#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-255-RESULT-CAPSULE-IR-SHAPE-DIFF-INVENTORY.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-254-WEIGHTED-EXACT-SLOT-OWNER-SELECTION-AFTER-RECEIVER-FORWARDING.md"
APP="$ROOT_DIR/apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
TMP_DIR="$(mktemp -d /tmp/hakorune_row255_result_capsule.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

MIR="$TMP_DIR/app.mir.json"
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row255-result-capsule] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=result-capsule-ir-shape-diff-inventory-v0"
require_line "$DOC" "alloc_result_field_op_count=32"
require_line "$DOC" "release_result_field_op_count=25"
require_line "$DOC" "combined_result_field_op_count=57"
require_line "$DOC" "combined_result_call_count=0"
require_line "$DOC" "top_release_method=HakoAllocObjectLifecycleReleaseResult.birth/0"
require_line "$DOC" "selected_next=result_capsule_owner_selection"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "summary=ok"

NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 \
  "$ROOT_DIR/target/release/hakorune" \
    --backend mir \
    --emit-mir-json "$MIR" \
    "$APP" >/dev/null

python3 - "$MIR" >"$REPORT" <<'PY'
import collections
import json
import sys

data = json.load(open(sys.argv[1], encoding="utf-8"))

def collect(prefix: str):
    totals = collections.Counter()
    method_count = 0
    top = ("none", 0)
    for fn in data.get("functions", []):
        name = fn.get("name", "")
        if not name.startswith(prefix):
            continue
        method_count += 1
        counts = collections.Counter()
        for block in fn.get("blocks", []):
            for inst in block.get("instructions", []):
                counts[inst.get("op")] += 1
        totals.update(counts)
        field_ops = counts["field_get"] + counts["field_set"]
        if field_ops > top[1]:
            top = (name, field_ops)
    return method_count, totals, top

alloc_count, alloc, alloc_top = collect("HakoAllocObjectLifecycleAllocResult.")
release_count, release, release_top = collect("HakoAllocObjectLifecycleReleaseResult.")
combined_field_get = alloc["field_get"] + release["field_get"]
combined_field_set = alloc["field_set"] + release["field_set"]
combined_call = alloc["mir_call"] + alloc["call"] + alloc["boxcall"] + release["mir_call"] + release["call"] + release["boxcall"]

print("output_contract=result-capsule-ir-shape-diff-inventory-v0")
print("input_contract=weighted-exact-slot-owner-selection-after-receiver-forwarding-v0")
print("workload_id=representative-object-lifecycle-small-block-v0")
print(f"alloc_result_method_count={alloc_count}")
print(f"alloc_result_field_get_count={alloc['field_get']}")
print(f"alloc_result_field_set_count={alloc['field_set']}")
print(f"alloc_result_field_op_count={alloc['field_get'] + alloc['field_set']}")
print(f"alloc_result_copy_count={alloc['copy']}")
print(f"alloc_result_call_count={alloc['mir_call'] + alloc['call'] + alloc['boxcall']}")
print(f"alloc_result_phi_count={alloc['phi']}")
print(f"alloc_result_branch_count={alloc['branch']}")
print(f"release_result_method_count={release_count}")
print(f"release_result_field_get_count={release['field_get']}")
print(f"release_result_field_set_count={release['field_set']}")
print(f"release_result_field_op_count={release['field_get'] + release['field_set']}")
print(f"release_result_copy_count={release['copy']}")
print(f"release_result_call_count={release['mir_call'] + release['call'] + release['boxcall']}")
print(f"release_result_phi_count={release['phi']}")
print(f"release_result_branch_count={release['branch']}")
print(f"combined_result_field_get_count={combined_field_get}")
print(f"combined_result_field_set_count={combined_field_set}")
print(f"combined_result_field_op_count={combined_field_get + combined_field_set}")
print(f"combined_result_copy_count={alloc['copy'] + release['copy']}")
print(f"combined_result_call_count={combined_call}")
print(f"combined_result_phi_count={alloc['phi'] + release['phi']}")
print(f"combined_result_branch_count={alloc['branch'] + release['branch']}")
print(f"top_alloc_method={alloc_top[0]}")
print(f"top_alloc_method_field_op_count={alloc_top[1]}")
print(f"top_release_method={release_top[0]}")
print(f"top_release_method_field_op_count={release_top[1]}")
print("selected_next=result_capsule_owner_selection")
print("optimization_open=0")
print("winner_claim=0")
print("replacement_active=0")
print("hook_installed=0")
print("global_allocator=0")
print("summary=ok")
PY

require_line "$REPORT" "output_contract=result-capsule-ir-shape-diff-inventory-v0"
require_line "$REPORT" "alloc_result_method_count=13"
require_line "$REPORT" "release_result_method_count=11"
require_line "$REPORT" "combined_result_field_op_count=57"
require_line "$REPORT" "combined_result_call_count=0"
require_line "$REPORT" "selected_next=result_capsule_owner_selection"
require_line "$REPORT" "summary=ok"

cat "$REPORT"
