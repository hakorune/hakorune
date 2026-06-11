#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-nyrt-startup-env-p0-centralization"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

DOC="docs/development/current/main/design/nyrt-startup-env-p0-centralization-ssot.md"
PRIORITY_DOC="docs/development/current/main/design/nyrt-startup-env-centralization-priority-ssot.md"
INVENTORY="docs/development/current/main/design/nyrt-startup-env-read-inventory-ssot.md"
CARD="docs/development/current/main/phases/phase-296x/296x-651-HAKO-MIMALLOC-TYPED-OBJECT-EXACT-SLOT-ABI-SPLIT.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
ENV_REF="docs/reference/environment-variables.md"
ENTRY="crates/nyash_kernel/src/entry.rs"
STAGE1_RUNTIME_DEFAULTS="src/runner/stage1_bridge/env/runtime_defaults.rs"
STAGE1_HELPER="src/config/env/stage1.rs"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_nyrt_startup_env_p0_centralization_guard.sh"

echo "[$TAG] checking NyRT startup env P0 centralization"

guard_require_files "$TAG" \
  "$DOC" "$PRIORITY_DOC" "$INVENTORY" "$CARD" "$CURRENT_STATE" "$INDEX" \
  "$ENV_REF" "$ENTRY" "$STAGE1_RUNTIME_DEFAULTS" "$STAGE1_HELPER" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" "NYRT-ENV-003" "$DOC" "P0 doc must name the implementation row"
guard_expect_fixed_in_file "$TAG" "nyrt_p0_centralization_landed=1" "$DOC" "P0 doc must mark the implementation landed"
guard_expect_fixed_in_file "$TAG" "nyrt_silent_result_helper_owner=src/config/env/stage1.rs" "$DOC" "P0 doc must name the helper owner"
guard_expect_fixed_in_file "$TAG" "nyrt_entry_uses_shared_helper=1" "$DOC" "P0 doc must record entry helper usage"
guard_expect_fixed_in_file "$TAG" "stage1_runtime_defaults_use_shared_helper=1" "$DOC" "P0 doc must record runtime defaults helper usage"
guard_expect_fixed_in_file "$TAG" "direct_nyrt_silent_result_reads_in_entry=0" "$DOC" "P0 doc must forbid direct entry reads"
guard_expect_fixed_in_file "$TAG" "direct_nyrt_silent_result_reads_in_runtime_defaults=0" "$DOC" "P0 doc must forbid direct runtime default reads"
guard_expect_fixed_in_file "$TAG" "src/config/env/stage1.rs" "$DOC" "P0 doc must cite the helper module"
guard_expect_fixed_in_file "$TAG" "docs/development/current/main/design/nyrt-startup-env-p0-centralization-ssot.md" "$ENV_REF" "env reference must point at the P0 doc"
guard_expect_fixed_in_file "$TAG" "NYRT-ENV-003" "$CARD" "phase card must mention the P0 implementation row"
guard_expect_fixed_in_file "$TAG" "NYRT-ENV-003" "$CURRENT_STATE" "current state must mention the P0 implementation row"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "NYASH_NYRT_SILENT_RESULT" "$STAGE1_HELPER" "shared helper must own the silent-result toggle"
guard_expect_fixed_in_file "$TAG" "nyrt_silent_result_enabled()" "$STAGE1_HELPER" "shared helper must expose the effective toggle"
guard_expect_fixed_in_file "$TAG" "nyrt_silent_result_present()" "$STAGE1_HELPER" "shared helper must expose presence detection"
guard_expect_fixed_in_file "$TAG" "nyrt_silent_result_enabled()" "$ENTRY" "entry must use the shared helper"
guard_expect_fixed_in_file "$TAG" "nyrt_silent_result_present()" "$STAGE1_RUNTIME_DEFAULTS" "runtime defaults must use the shared helper"

if rg -F -q 'flag_on("NYASH_NYRT_SILENT_RESULT")' "$ENTRY"; then
  guard_fail "$TAG" "entry must not read NYASH_NYRT_SILENT_RESULT directly anymore"
fi
if rg -F -q 'std::env::var("NYASH_NYRT_SILENT_RESULT")' "$STAGE1_RUNTIME_DEFAULTS"; then
  guard_fail "$TAG" "runtime defaults must not read NYASH_NYRT_SILENT_RESULT directly anymore"
fi

tmp_dir="$(mktemp -d /tmp/hakorune_nyrt_env_p0_centralization.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
cat >"$report" <<'REPORT'
output_contract=nyrt-startup-env-p0-centralization-v0
input_contract=nyrt-startup-env-centralization-priority-v0
nyrt_p0_centralization_landed=1
nyrt_silent_result_helper_owner=src/config/env/stage1.rs
nyrt_entry_uses_shared_helper=1
stage1_runtime_defaults_use_shared_helper=1
direct_nyrt_silent_result_reads_in_entry=0
direct_nyrt_silent_result_reads_in_runtime_defaults=0
summary=ok
REPORT
cat "$report"

guard_expect_fixed_in_file "$TAG" "output_contract=nyrt-startup-env-p0-centralization-v0" "$report" "P0 report must carry the output contract"
guard_expect_fixed_in_file "$TAG" "input_contract=nyrt-startup-env-centralization-priority-v0" "$report" "P0 report must point at the priority SSOT"
guard_expect_fixed_in_file "$TAG" "nyrt_p0_centralization_landed=1" "$report" "P0 report must mark the implementation landed"
guard_expect_fixed_in_file "$TAG" "nyrt_entry_uses_shared_helper=1" "$report" "P0 report must record entry helper usage"
guard_expect_fixed_in_file "$TAG" "stage1_runtime_defaults_use_shared_helper=1" "$report" "P0 report must record runtime defaults helper usage"
guard_expect_fixed_in_file "$TAG" "summary=ok" "$report" "P0 report must finish cleanly"

echo "[$TAG] ok"
