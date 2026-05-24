#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-repeated-measurement-policy"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-28-MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-POLICY.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-27-MIMALLOC-COMPARISON-HUGE-ISH-CLOSEOUT.md"
SSOT="docs/development/current/main/design/mimalloc-comparison-execution-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_repeated_measurement_policy_guard.sh"

echo "[$TAG] checking phase-295x repeated measurement policy"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$SSOT" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-POLICY-295X-001' "$CARD" "card must identify the current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-RUNNER-295X-001' "$CARD" "card must select runner follow-on"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-POLICY-295X-001' "$PREV_CARD" "previous row must select this policy"
guard_expect_in_file "$TAG" 'measurement_profile=phase295x-repeated-v0' "$CARD" "card must name the policy profile"
guard_expect_in_file "$TAG" 'sample_count=5' "$CARD" "card must fix sample count"
guard_expect_in_file "$TAG" 'warmup_count=1' "$CARD" "card must fix warmup count"
guard_expect_in_file "$TAG" 'canonical_rss_collector=external-time' "$CARD" "card must fix RSS collector"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-RUNNER-295X-001' "$TASKBOARD" "taskboard must expose runner follow-on"
guard_expect_in_file "$TAG" 'phase295x-repeated-v0' "$SSOT" "SSOT must define the repeated profile"
guard_expect_in_file "$TAG" 'canonical_rss_collector=external-time' "$SSOT" "SSOT must define canonical RSS collector"
guard_expect_in_file "$TAG" 'winner_claim=0' "$SSOT" "SSOT must keep winner claims closed"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

echo "[$TAG] ok"
