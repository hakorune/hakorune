#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-mimalloc-provider-explicit-comparison-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_18="docs/development/current/main/phases/phase-296x/296x-18-MIMALLOC-PROVIDER-EXPLICIT-COMPARISON-ADAPTER-PILOT.md"
CARD_19="docs/development/current/main/phases/phase-296x/296x-19-MIMALLOC-PROVIDER-EXPLICIT-COMPARISON-CLOSEOUT.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
ARTIFACT_SSOT="docs/development/current/main/design/provider-package-artifact-ssot.md"
RUNTIME_SSOT="docs/development/current/main/design/provider-runtime-load-ssot.md"
PREV_GUARD="tools/checks/k2_wide_phase296x_mimalloc_provider_explicit_comparison_adapter_pilot_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_provider_explicit_comparison_closeout_guard.sh"

echo "[$TAG] checking phase-296x provider explicit comparison closeout"

guard_require_files "$TAG" "$CARD_18" "$CARD_19" "$TASKBOARD" "$INDEX" "$ARTIFACT_SSOT" "$RUNTIME_SSOT" "$PREV_GUARD" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$PREV_GUARD" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_18" "adapter pilot must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_19" "closeout card must be landed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-EXPLICIT-COMPARISON-CLOSEOUT-296X-001' "$CARD_19" "closeout card must identify blocker"
guard_expect_fixed_in_file "$TAG" 'output_contract=mimalloc-provider-explicit-comparison-adapter-v0' "$CARD_19" "closeout must preserve adapter contract"
guard_expect_fixed_in_file "$TAG" 'comparison_subjects=hako_exact_exe,c_mimalloc_explicit_runner,provider_package_explicit_alloc_free' "$CARD_19" "closeout must preserve subjects"
guard_expect_fixed_in_file "$TAG" 'subject_count=3' "$CARD_19" "closeout must preserve subject count"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_19" "closeout must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$CARD_19" "closeout must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$CARD_19" "closeout must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$CARD_19" "closeout must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-EXISTING-BINARY-MANIFEST-PILOT-296X-001' "$CARD_19" "closeout must select package artifact pilot"
guard_expect_fixed_in_file "$TAG" 'provider package artifact lane' "$CARD_19" "closeout must select Phase A package boundary"
guard_expect_fixed_in_file "$TAG" 'does not run benchmarks, build shared libraries, load provider' "$CARD_19" "closeout must keep stop line explicit"

guard_expect_fixed_in_file "$TAG" 'Phase A: package existing binary + manifest' "$ARTIFACT_SSOT" "artifact SSOT must define Phase A"
guard_expect_fixed_in_file "$TAG" 'Provider package presence alone never activates a provider.' "$RUNTIME_SSOT" "runtime SSOT must keep activation closed"

guard_expect_fixed_in_file "$TAG" '| 19 | `MIMALLOC-PROVIDER-EXPLICIT-COMPARISON-CLOSEOUT-296X-001` | Landed |' "$TASKBOARD" "taskboard row 19 must be landed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-EXISTING-BINARY-MANIFEST-PILOT-296X-001' "$TASKBOARD" "taskboard must expose package manifest pilot row"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list closeout guard"

echo "[$TAG] ok"
