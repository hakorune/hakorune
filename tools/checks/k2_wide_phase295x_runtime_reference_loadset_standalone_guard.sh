#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-runtime-reference-loadset-standalone"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-62-MIMALLOC-COMPARISON-RUNTIME-REFERENCE-LOADSET-STANDALONE.md"
LOADSETS="docs/reference/runtime/plugin-loadsets.md"
STANDALONE="docs/reference/runtime/standalone-exe-routes.md"
REF_INDEX="docs/reference/README.md"
PLUGIN_INDEX="docs/reference/plugin-system/README.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_runtime_reference_loadset_standalone_guard.sh"

echo "[$TAG] checking phase-295x runtime reference loadset/standalone docs"

guard_require_files "$TAG" "$CARD" "$LOADSETS" "$STANDALONE" "$REF_INDEX" "$PLUGIN_INDEX" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-RUNTIME-REFERENCE-LOADSET-STANDALONE-295X-001' "$CARD" "card must identify current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-RUNTIME-REFERENCE-LOADSET-STANDALONE-CLOSEOUT-295X-001' "$CARD" "card must select closeout"

guard_expect_in_file "$TAG" 'plugin_load_policy=eager_selected' "$LOADSETS" "loadset reference must document eager selected policy"
guard_expect_in_file "$TAG" 'no implicit lazy loading' "$LOADSETS" "loadset reference must forbid implicit lazy"
guard_expect_in_file "$TAG" 'output_contract=hako-plugin-loadset-plan-v0' "$LOADSETS" "loadset reference must document preflight contract"
guard_expect_in_file "$TAG" 'hako_selected_loadset' "$LOADSETS" "loadset reference must document comparison evidence"
guard_expect_in_file "$TAG" 'provider_activation=0' "$LOADSETS" "loadset reference must keep provider activation closed"

guard_expect_in_file "$TAG" 'standalone-minimal' "$STANDALONE" "standalone reference must define minimal route"
guard_expect_in_file "$TAG" 'standalone-root' "$STANDALONE" "standalone reference must define root route"
guard_expect_in_file "$TAG" 'standalone-diagnostic' "$STANDALONE" "standalone reference must define diagnostic route"
guard_expect_in_file "$TAG" 'link_policy=exact-mir-exe' "$STANDALONE" "standalone reference must define exact MIR link policy"
guard_expect_in_file "$TAG" 'standalone_packaging_generated=<0|1>' "$STANDALONE" "standalone reference must define packaging evidence"

guard_expect_in_file "$TAG" 'plugin-loadsets.md' "$REF_INDEX" "reference index must link plugin loadsets"
guard_expect_in_file "$TAG" 'standalone-exe-routes.md' "$REF_INDEX" "reference index must link standalone routes"
guard_expect_in_file "$TAG" 'plugin-loadsets.md' "$PLUGIN_INDEX" "plugin-system index must link plugin loadsets"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-RUNTIME-REFERENCE-LOADSET-STANDALONE-CLOSEOUT-295X-001' "$TASKBOARD" "taskboard must expose selected follow-on"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

echo "[$TAG] ok"
