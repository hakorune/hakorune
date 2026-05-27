#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-hakmem-ldpreload-shim-decision"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_72="docs/development/current/main/phases/phase-296x/296x-72-HAKO-MIMALLOC-HAKMEM-LDPRELOAD-SHIM-DECISION.md"
CARD_73="docs/development/current/main/phases/phase-296x/296x-73-HAKO-MIMALLOC-HAKMEM-LDPRELOAD-SHIM-SMOKE.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_mimalloc_hakmem_ldpreload_shim_decision.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_hakmem_ldpreload_shim_decision_guard.sh"

echo "[$TAG] checking phase-296x hakmem LD_PRELOAD shim decision"

guard_require_files "$TAG" "$CARD_72" "$CARD_73" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_72" "LD_PRELOAD decision card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_73" "LD_PRELOAD smoke card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-hakmem-ldpreload-shim-decision-v0' "$CARD_72" "card must record decision contract"
guard_expect_fixed_in_file "$TAG" 'ld_preload_shim_decision=accepted' "$CARD_72" "card must accept probe-only shim"
guard_expect_fixed_in_file "$TAG" 'decision_scope=hakmem_compat_probe_only' "$CARD_72" "card must scope decision"
guard_expect_fixed_in_file "$TAG" 'provider_call_evidence_ready=1' "$CARD_72" "card must require provider evidence"
guard_expect_fixed_in_file "$TAG" 'ld_preload_shim_build_allowed=1' "$CARD_72" "card must allow shim build"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$CARD_72" "card must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$CARD_72" "card must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$CARD_72" "card must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_72" "card must keep winner closed"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-72-HAKO-MIMALLOC-HAKMEM-LDPRELOAD-SHIM-DECISION"' "$CURRENT_STATE" "current state latest card must advance to row 72"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-HAKMEM-LDPRELOAD-SHIM-SMOKE-296X-001"' "$CURRENT_STATE" "current state must select LD_PRELOAD smoke"
guard_expect_fixed_in_file "$TAG" '| 72 | `HAKO-MIMALLOC-HAKMEM-LDPRELOAD-SHIM-DECISION-296X-001` | Landed |' "$TASKBOARD" "taskboard row 72 must be landed"
guard_expect_fixed_in_file "$TAG" '| 73 | `HAKO-MIMALLOC-HAKMEM-LDPRELOAD-SHIM-SMOKE-296X-001` | Current |' "$TASKBOARD" "taskboard row 73 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list decision tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_ldpreload_decision.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
python3 "$TOOL" --repo-root "$ROOT_DIR" --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-hakmem-ldpreload-shim-decision-v0' "$report" "tool must emit decision contract"
guard_expect_fixed_in_file "$TAG" 'ld_preload_shim_decision=accepted' "$report" "tool must accept shim"
guard_expect_fixed_in_file "$TAG" 'decision_scope=hakmem_compat_probe_only' "$report" "tool must scope decision"
guard_expect_fixed_in_file "$TAG" 'provider_call_evidence_ready=1' "$report" "tool must require provider evidence"
guard_expect_fixed_in_file "$TAG" 'ld_preload_shim_build_allowed=1' "$report" "tool must allow shim build"
guard_expect_fixed_in_file "$TAG" 'ld_preload_shim_ready=0' "$report" "tool must not build shim"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$report" "tool must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$report" "tool must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$report" "tool must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'next_row=HAKO-MIMALLOC-HAKMEM-LDPRELOAD-SHIM-SMOKE-296X-001' "$report" "tool must select smoke row"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
