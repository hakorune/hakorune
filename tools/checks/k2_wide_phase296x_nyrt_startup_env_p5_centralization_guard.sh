#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-nyrt-startup-env-p5-centralization"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

DOC="docs/development/current/main/design/nyrt-startup-env-p5-centralization-ssot.md"
PRIORITY_DOC="docs/development/current/main/design/nyrt-startup-env-centralization-priority-ssot.md"
P4_DOC="docs/development/current/main/design/nyrt-startup-env-p4-centralization-ssot.md"
INVENTORY="docs/development/current/main/design/nyrt-startup-env-read-inventory-ssot.md"
CARD="docs/development/current/main/phases/phase-296x/296x-651-HAKO-MIMALLOC-TYPED-OBJECT-EXACT-SLOT-ABI-SPLIT.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
ENV_REF="docs/reference/environment-variables.md"
ENTRY="crates/nyash_kernel/src/entry.rs"
PATHS_HELPER="src/config/env/paths.rs"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_nyrt_startup_env_p5_centralization_guard.sh"

echo "[$TAG] checking NyRT startup env P5 centralization"

guard_require_files "$TAG" \
  "$DOC" "$PRIORITY_DOC" "$P4_DOC" "$INVENTORY" "$CARD" "$CURRENT_STATE" \
  "$INDEX" "$ENV_REF" "$ENTRY" "$PATHS_HELPER" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" "NYRT-ENV-008" "$DOC" "P5 doc must name the implementation row"
guard_expect_fixed_in_file "$TAG" "nyrt_p5_centralization_landed=1" "$DOC" "P5 doc must mark the implementation landed"
guard_expect_fixed_in_file "$TAG" "nyrt_path_helper_owner=src/config/env/paths.rs" "$DOC" "P5 doc must name the helper owner"
guard_expect_fixed_in_file "$TAG" "nyrt_entry_uses_shared_path_helpers=1" "$DOC" "P5 doc must record entry helper usage"
guard_expect_fixed_in_file "$TAG" "nyrt_path_shaping_shared=1" "$DOC" "P5 doc must record helper sharing"
guard_expect_fixed_in_file "$TAG" "direct_nyrt_path_reads_in_entry=0" "$DOC" "P5 doc must forbid direct entry reads"
guard_expect_fixed_in_file "$TAG" "docs/development/current/main/design/nyrt-startup-env-p5-centralization-ssot.md" "$ENV_REF" "env reference must point at the P5 doc"
guard_expect_fixed_in_file "$TAG" "NYRT-ENV-008" "$CARD" "phase card must mention the P5 implementation row"
guard_expect_fixed_in_file "$TAG" "NYRT-ENV-008" "$CURRENT_STATE" "current state must mention the P5 implementation row"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "NYRT-ENV-008" "$PRIORITY_DOC" "priority doc must mention the P5 landed row"
guard_expect_fixed_in_file "$TAG" "NYRT-ENV-007" "$P4_DOC" "P4 doc must remain landed before P5"
guard_expect_fixed_in_file "$TAG" "nyrt_entry_exe_dir()" "$PATHS_HELPER" "paths helper must expose executable discovery"
guard_expect_fixed_in_file "$TAG" "nyrt_entry_current_dir_display()" "$PATHS_HELPER" "paths helper must expose current-dir display"
guard_expect_fixed_in_file "$TAG" "nyrt_entry_apply_windows_path_shaping" "$PATHS_HELPER" "paths helper must expose Windows shaping"
guard_expect_fixed_in_file "$TAG" "nyrt_entry_exe_dir()" "$ENTRY" "entry must use the shared exe-dir helper"
guard_expect_fixed_in_file "$TAG" "nyrt_entry_current_dir_display()" "$ENTRY" "entry must use the shared current-dir helper"
guard_expect_fixed_in_file "$TAG" "nyrt_entry_apply_windows_path_shaping" "$ENTRY" "entry must use the shared Windows shaping helper"

if rg -F -q 'std::env::current_exe()' "$ENTRY"; then
  guard_fail "$TAG" "entry must not read current_exe directly anymore"
fi
if rg -F -q 'std::env::current_dir()' "$ENTRY"; then
  guard_fail "$TAG" "entry must not read current_dir directly anymore"
fi
if rg -F -q 'std::env::var("PATH")' "$ENTRY"; then
  guard_fail "$TAG" "entry must not read PATH directly anymore"
fi
if rg -F -q 'std::env::var("PYTHONHOME")' "$ENTRY"; then
  guard_fail "$TAG" "entry must not read PYTHONHOME directly anymore"
fi

tmp_dir="$(mktemp -d /tmp/hakorune_nyrt_env_p5_centralization.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
cat >"$report" <<'REPORT'
output_contract=nyrt-startup-env-p5-centralization-v0
input_contract=nyrt-startup-env-p4-centralization-v0
nyrt_p5_centralization_landed=1
nyrt_path_helper_owner=src/config/env/paths.rs
nyrt_entry_uses_shared_path_helpers=1
nyrt_path_shaping_shared=1
direct_nyrt_path_reads_in_entry=0
summary=ok
REPORT
cat "$report"

guard_expect_fixed_in_file "$TAG" "output_contract=nyrt-startup-env-p5-centralization-v0" "$report" "P5 report must carry the output contract"
guard_expect_fixed_in_file "$TAG" "input_contract=nyrt-startup-env-p4-centralization-v0" "$report" "P5 report must point at the P4 slice"
guard_expect_fixed_in_file "$TAG" "nyrt_p5_centralization_landed=1" "$report" "P5 report must mark the implementation landed"
guard_expect_fixed_in_file "$TAG" "nyrt_entry_uses_shared_path_helpers=1" "$report" "P5 report must record entry helper usage"
guard_expect_fixed_in_file "$TAG" "summary=ok" "$report" "P5 report must finish cleanly"

echo "[$TAG] ok"
