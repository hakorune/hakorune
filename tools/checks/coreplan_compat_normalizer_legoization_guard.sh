#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="coreplan-compat-normalizer-legoization"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

SSOT="docs/development/current/main/design/coreplan-compat-normalizer-legoization-ssot.md"
TASKBOARD="docs/development/current/main/workstreams/compiler-foundation-current.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1005-COREPLAN-FOUND-000-001.md"
ROADMAP="docs/development/current/main/design/coreplan-migration-roadmap-ssot.md"
REGISTRY="src/mir/builder/control_flow/plan/REGISTRY.md"
NORMALIZER_README="src/mir/builder/control_flow/plan/normalizer/README.md"
FEATURES_README="src/mir/builder/control_flow/plan/features/README.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/coreplan_compat_normalizer_legoization_guard.sh"

echo "[$TAG] checking CorePlan compatibility normalizer lego-ization boundary"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$TASKBOARD" \
  "$CARD" \
  "$ROADMAP" \
  "$REGISTRY" \
  "$NORMALIZER_README" \
  "$FEATURES_README" \
  "$INDEX" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" \
  "selected_family=B1_remaining_compatibility_normalizer_legoization" \
  "$SSOT" \
  "SSOT must select B1"
guard_expect_fixed_in_file "$TAG" \
  "selected_family_ssot_exists=1" \
  "$SSOT" \
  "SSOT must record FOUND-001 acceptance"
guard_expect_fixed_in_file "$TAG" \
  "planner_required_failfast_preserved=1" \
  "$SSOT" \
  "SSOT must preserve planner_required fail-fast"
guard_expect_fixed_in_file "$TAG" \
  "bash tools/checks/coreplan_compat_normalizer_legoization_guard.sh" \
  "$SSOT" \
  "SSOT must name this guard"

guard_expect_fixed_in_file "$TAG" \
  "B1_remaining_compatibility_normalizer_legoization" \
  "$TASKBOARD" \
  "taskboard must list B1 candidate"
guard_expect_fixed_in_file "$TAG" \
  "COREPLAN-FOUND-001: selected family SSOT / fixture / gate" \
  "$TASKBOARD" \
  "taskboard must keep FOUND-001 row"
guard_expect_fixed_in_file "$TAG" \
  "docs/development/current/main/design/coreplan-compat-normalizer-legoization-ssot.md" \
  "$TASKBOARD" \
  "taskboard must point to selected family SSOT"

guard_expect_fixed_in_file "$TAG" \
  "selected_family=B1_remaining_compatibility_normalizer_legoization" \
  "$CARD" \
  "active card must select B1"
guard_expect_fixed_in_file "$TAG" \
  "boxcount_boxshape_mixed=0" \
  "$CARD" \
  "active card must keep BoxCount/BoxShape separate"

guard_expect_fixed_in_file "$TAG" \
  "B1. Remaining compatibility-lane normalizers" \
  "$ROADMAP" \
  "roadmap must keep B1 row"
guard_expect_fixed_in_file "$TAG" \
  "Remaining legacy normalizers / vocabulary hotspots" \
  "$REGISTRY" \
  "registry must keep remaining normalizer hotspot table"
guard_expect_fixed_in_file "$TAG" \
  "legacy normalizer に分岐を足さない" \
  "$REGISTRY" \
  "registry must forbid adding legacy normalizer branches"

guard_expect_fixed_in_file "$TAG" \
  "Composer/entry 経路では使わない" \
  "$NORMALIZER_README" \
  "normalizer README must keep legacy/analysis boundary"
guard_expect_fixed_in_file "$TAG" \
  "Do not re-run facts/canon analysis" \
  "$NORMALIZER_README" \
  "normalizer README must forbid re-running facts/canon"
guard_expect_fixed_in_file "$TAG" \
  "Do not list retired route files as active modules" \
  "$NORMALIZER_README" \
  "normalizer README must prevent stale route resurrection"
guard_expect_fixed_in_file "$TAG" \
  'plan/<kind>/normalizer/*` should be a thin adapter that calls a pipeline' \
  "$FEATURES_README" \
  "features README must keep normalizer adapters thin"

for retired in \
  "src/mir/builder/control_flow/plan/normalizer/simple_while_coreloop_builder.rs" \
  "src/mir/builder/control_flow/plan/normalizer/loop_break.rs"
do
  if [[ -e "$retired" ]]; then
    guard_fail "$TAG" "retired normalizer file reintroduced: $retired"
  fi
done

if find src/mir/builder/control_flow/plan/normalizer \
  -maxdepth 1 \
  -type f \
  -name 'pattern_*.rs' \
  | rg -q .; then
  guard_fail "$TAG" "route-specific pattern_*.rs normalizer files reintroduced"
fi

guard_expect_fixed_in_file "$TAG" \
  "$SELF_SCRIPT" \
  "$INDEX" \
  "check index must list this guard"

echo "[$TAG] ok"
