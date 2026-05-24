#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-minimal-config-repeated-pack-run"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-51-MIMALLOC-COMPARISON-MINIMAL-CONFIG-REPEATED-PACK-RUN.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-50-MIMALLOC-COMPARISON-RUNTIME-CONFIG-PROFILE-CONTRACT.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_minimal_config_repeated_pack_run_guard.sh"
RUNNER="tools/allocator/mimalloc_repeated_measurement_runner.py"
HAKO_RUNNER="tools/allocator/hako_exe_memory_runner.sh"

echo "[$TAG] checking phase-295x minimal-config repeated pack run"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT" "$RUNNER" "$HAKO_RUNNER"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$RUNNER" "$HAKO_RUNNER"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MINIMAL-CONFIG-REPEATED-PACK-RUN-295X-001' "$CARD" "card must identify current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MINIMAL-CONFIG-REPEATED-PACK-CLOSEOUT-295X-001' "$CARD" "card must select closeout follow-on"
guard_expect_in_file "$TAG" '--hako-runtime-config empty' "$CARD" "card must select empty runtime profile"
guard_expect_in_file "$TAG" 'hako_runtime_config_profile' "$CARD" "card must document output field"
guard_expect_in_file "$TAG" '--hako-runtime-config' "$RUNNER" "runner must expose hako runtime config profile option"
guard_expect_in_file "$TAG" 'hako_runtime_config_profile' "$RUNNER" "runner must emit hako runtime config profile"
guard_expect_in_file "$TAG" 'runtime_config_profile' "$RUNNER" "runner must validate hako evidence runtime profile"
guard_expect_in_file "$TAG" 'RUNTIME_CONFIG="root"' "$HAKO_RUNNER" "hako evidence runner default must remain root"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MINIMAL-CONFIG-REPEATED-PACK-CLOSEOUT-295X-001' "$TASKBOARD" "taskboard must expose selected follow-on"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

tmp_dir="$(mktemp -d /tmp/hakorune_phase295x_minimal_config_pack.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
out="$tmp_dir/minimal-config-pack.out"

python3 "$RUNNER" \
  --out "$out" \
  --sample-count 1 \
  --warmup-count 0 \
  --hako-runtime-config empty \
  --allow-ldconfig-discovery

rg -F -q 'mimalloc_repeated_measurement_runner=1' "$out"
rg -F -q 'output_contract=mimalloc-comparison-repeated-measurement-v0' "$out"
rg -F -q 'measurement_profile=phase295x-repeated-v0' "$out"
rg -F -q 'hako_runtime_config_profile=empty' "$out"
rg -F -q 'warmup_count=0' "$out"
rg -F -q 'sample_count=1' "$out"
rg -F -q 'workload_count=4' "$out"
rg -F -q 'winner_claim=0' "$out"
rg -F -q 'summary=ok' "$out"

python3 - "$out" <<'PY'
import sys

values = {}
with open(sys.argv[1], encoding="utf-8") as fh:
    for line in fh:
        line = line.strip()
        if "=" in line:
            key, value = line.split("=", 1)
            values[key] = value

if values.get("hako_runtime_config_profile") != "empty":
    raise SystemExit("hako runtime config profile must be empty")
for idx in range(4):
    if values.get(f"workload_{idx}_winner_claim") != "0":
        raise SystemExit(f"workload {idx} winner claim must remain closed")
    for side in ("hako", "c"):
        median = int(values.get(f"workload_{idx}_{side}_external_rss_median_bytes", "0"))
        if median <= 0:
            raise SystemExit(f"workload {idx} {side} median RSS must be positive")
print("[phase295x-minimal-config-repeated-pack-run] ok")
PY

cat "$out"
echo "[$TAG] ok"
