#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-mimalloc-abandoned-heap-stress-nyrt-plugin-loadset-smaller-default-set-evidence"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-229-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-LOADSET-SMALLER-DEFAULT-SET-EVIDENCE.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-228-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-LOADSET-CLOSEOUT.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
RUNNER="tools/allocator/mimalloc_repeated_measurement_runner.py"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_mimalloc_abandoned_heap_stress_nyrt_plugin_loadset_smaller_default_set_evidence_guard.sh"

echo "[$TAG] checking phase-295x abandoned-heap stress smaller-default load-set evidence"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$RUNNER" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$RUNNER" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Current' "$CARD" "card must remain current while the evidence row is exercised"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-LOADSET-SMALLER-DEFAULT-SET-EVIDENCE-295X-002' "$CARD" "card must identify the current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-LOADSET-SMALLER-DEFAULT-SET-CLOSEOUT-295X-002' "$CARD" "card must select the closeout row"
guard_expect_in_file "$TAG" 'representative-small-block-v0' "$CARD" "card must keep the repeated comparison evidence"
guard_expect_in_file "$TAG" '3,588,096' "$CARD" "card must record the empty-default median"
guard_expect_in_file "$TAG" '9,457,664' "$CARD" "card must record the root median"
guard_expect_in_file "$TAG" 'representative-realloc-aligned-v0' "$CARD" "card must keep the realloc/aligned evidence"
guard_expect_in_file "$TAG" '3,657,728' "$CARD" "card must record the realloc/aligned empty-default median"
guard_expect_in_file "$TAG" '9,580,544' "$CARD" "card must record the realloc/aligned root median"
guard_expect_in_file "$TAG" 'representative-mixed-small-v0' "$CARD" "card must keep the mixed-small evidence"
guard_expect_in_file "$TAG" '3,641,344' "$CARD" "card must record the mixed-small empty-default median"
guard_expect_in_file "$TAG" '9,637,888' "$CARD" "card must record the mixed-small root median"
guard_expect_in_file "$TAG" 'representative-huge-ish-v0' "$CARD" "card must keep the huge-ish evidence"
guard_expect_in_file "$TAG" '3,612,672' "$CARD" "card must record the huge-ish empty-default median"
guard_expect_in_file "$TAG" '9,478,144' "$CARD" "card must record the huge-ish root median"
guard_expect_in_file "$TAG" 'hako_runtime_config_default=empty' "$CARD" "card must record the runner default"
guard_expect_in_file "$TAG" 'Status: Landed' "$PREV_CARD" "previous row must be landed before evidence runs"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-LOADSET-SMALLER-DEFAULT-SET-EVIDENCE-295X-002' "$TASKBOARD" "taskboard must expose the evidence row"
guard_expect_in_file "$TAG" '| 228 | `MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-LOADSET-CLOSEOUT-295X-002` | Landed |' "$TASKBOARD" "taskboard must mark the closeout row landed"
guard_expect_in_file "$TAG" '| 229 | `MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-LOADSET-SMALLER-DEFAULT-SET-EVIDENCE-295X-002` | Current |' "$TASKBOARD" "taskboard must expose the evidence row as current"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"
guard_expect_in_file "$TAG" 'default: empty' "$RUNNER" "runner must default to the empty hako runtime config"
guard_expect_in_file "$TAG" 'hako_runtime_config_default=empty' "$RUNNER" "runner must report the default runtime config"

tmp_dir="$(mktemp -d /tmp/hakorune_phase295x_abandoned_small_default.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
empty_out="$tmp_dir/empty.out"
root_out="$tmp_dir/root.out"

python3 "$RUNNER" \
  --out "$empty_out" \
  --sample-count 5 \
  --warmup-count 1 \
  --workload representative-small-block-v0 \
  --hako-runtime-config empty \
  --allow-ldconfig-discovery >/dev/null

python3 "$RUNNER" \
  --out "$root_out" \
  --sample-count 5 \
  --warmup-count 1 \
  --workload representative-small-block-v0 \
  --hako-runtime-config root \
  --allow-ldconfig-discovery >/dev/null

python3 - "$empty_out" "$root_out" <<'PY'
import sys

def read(path: str) -> dict[str, str]:
    values = {}
    with open(path, encoding="utf-8") as fh:
        for line in fh:
            line = line.strip()
            if "=" in line:
                key, value = line.split("=", 1)
                values[key] = value
    return values

empty = read(sys.argv[1])
root = read(sys.argv[2])

for values, label, expected_profile, expected_loadset in (
    (empty, "empty", "empty", "empty"),
    (root, "root", "root", "root"),
):
    if values.get("output_contract") != "mimalloc-comparison-repeated-measurement-v0":
        raise SystemExit(f"{label}: bad output contract")
    if values.get("summary") != "ok":
        raise SystemExit(f"{label}: summary must be ok")
    if values.get("winner_claim") != "0":
        raise SystemExit(f"{label}: winner claim must stay closed")
    if values.get("hako_runtime_config_profile") != expected_profile:
        raise SystemExit(f"{label}: runtime config profile mismatch")
    if values.get("hako_selected_loadset") != expected_loadset:
        raise SystemExit(f"{label}: selected loadset mismatch")
    if values.get("workload_0_id") != "representative-small-block-v0":
        raise SystemExit(f"{label}: workload mismatch")

empty_rss = int(empty["workload_0_hako_external_rss_median_bytes"])
root_rss = int(root["workload_0_hako_external_rss_median_bytes"])
if empty_rss <= 0 or root_rss <= 0:
    raise SystemExit("median RSS must be positive")
if empty_rss >= root_rss:
    raise SystemExit("empty default RSS must stay smaller than root compatibility RSS")

print("[phase295x-abandoned-heap-small-default-set-evidence] ok")
PY

echo "[$TAG] ok"
