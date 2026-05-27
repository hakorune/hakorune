#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-hakmem-ldpreload-shim-smoke"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_73="docs/development/current/main/phases/phase-296x/296x-73-HAKO-MIMALLOC-HAKMEM-LDPRELOAD-SHIM-SMOKE.md"
CARD_74="docs/development/current/main/phases/phase-296x/296x-74-HAKO-MIMALLOC-HAKMEM-LDPRELOAD-BENCH-PILOT.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_mimalloc_hakmem_ldpreload_shim_smoke.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_hakmem_ldpreload_shim_smoke_guard.sh"

echo "[$TAG] checking phase-296x hakmem LD_PRELOAD shim smoke"

guard_require_files "$TAG" "$CARD_73" "$CARD_74" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_73" "LD_PRELOAD smoke card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_74" "LD_PRELOAD bench pilot card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-hakmem-ldpreload-shim-smoke-v0' "$CARD_73" "card must record smoke contract"
guard_expect_fixed_in_file "$TAG" 'ld_preload_compatible=1' "$CARD_73" "card must mark LD_PRELOAD compatible"
guard_expect_fixed_in_file "$TAG" 'malloc_family_symbols_exported=1' "$CARD_73" "card must export malloc family"
guard_expect_fixed_in_file "$TAG" 'hakmem_script_compatible=probe-only' "$CARD_73" "card must keep probe-only scope"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$CARD_73" "card must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$CARD_73" "card must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$CARD_73" "card must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_73" "card must keep winner closed"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-73-HAKO-MIMALLOC-HAKMEM-LDPRELOAD-SHIM-SMOKE"' "$CURRENT_STATE" "current state latest card must advance to row 73"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-HAKMEM-LDPRELOAD-BENCH-PILOT-296X-001"' "$CURRENT_STATE" "current state must select bench pilot"
guard_expect_fixed_in_file "$TAG" '| 73 | `HAKO-MIMALLOC-HAKMEM-LDPRELOAD-SHIM-SMOKE-296X-001` | Landed |' "$TASKBOARD" "taskboard row 73 must be landed"
guard_expect_fixed_in_file "$TAG" '| 74 | `HAKO-MIMALLOC-HAKMEM-LDPRELOAD-BENCH-PILOT-296X-001` | Current |' "$TASKBOARD" "taskboard row 74 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list smoke tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_ldpreload_smoke.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
python3 "$TOOL" --out-dir "$tmp_dir/shim" --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-hakmem-ldpreload-shim-smoke-v0' "$report" "tool must emit smoke contract"
guard_expect_fixed_in_file "$TAG" 'ld_preload_compatible=1' "$report" "tool must mark compatible"
guard_expect_fixed_in_file "$TAG" 'shared_library_load_executed=1' "$report" "tool must load library"
guard_expect_fixed_in_file "$TAG" 'malloc_family_symbols_exported=1' "$report" "tool must export malloc family"
guard_expect_fixed_in_file "$TAG" 'malloc_family_symbols=malloc,free,calloc,realloc' "$report" "tool must list symbols"
guard_expect_fixed_in_file "$TAG" 'hakmem_script_compatible=probe-only' "$report" "tool must keep probe-only scope"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$report" "tool must keep provider inactive"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$report" "tool must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$report" "tool must keep hook closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$report" "tool must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'next_row=HAKO-MIMALLOC-HAKMEM-LDPRELOAD-BENCH-PILOT-296X-001' "$report" "tool must select bench pilot"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
