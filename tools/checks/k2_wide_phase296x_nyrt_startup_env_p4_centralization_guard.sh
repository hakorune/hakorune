#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-nyrt-startup-env-p4-centralization"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

DOC="docs/development/current/main/design/nyrt-startup-env-p4-centralization-ssot.md"
PRIORITY_DOC="docs/development/current/main/design/nyrt-startup-env-centralization-priority-ssot.md"
P3_DOC="docs/development/current/main/design/nyrt-startup-env-p3-centralization-ssot.md"
INVENTORY="docs/development/current/main/design/nyrt-startup-env-read-inventory-ssot.md"
CARD="docs/development/current/main/phases/phase-296x/296x-651-HAKO-MIMALLOC-TYPED-OBJECT-EXACT-SLOT-ABI-SPLIT.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
ENV_REF="docs/reference/environment-variables.md"
ENTRY="crates/nyash_kernel/src/entry.rs"
STAGE1_HELPER="src/config/env/stage1.rs"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_nyrt_startup_env_p4_centralization_guard.sh"

echo "[$TAG] checking NyRT startup env P4 centralization"

guard_require_files "$TAG" \
  "$DOC" "$PRIORITY_DOC" "$P3_DOC" "$INVENTORY" "$CARD" "$CURRENT_STATE" \
  "$INDEX" "$ENV_REF" "$ENTRY" "$STAGE1_HELPER" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" "NYRT-ENV-007" "$DOC" "P4 doc must name the implementation row"
guard_expect_fixed_in_file "$TAG" "nyrt_p4_centralization_landed=1" "$DOC" "P4 doc must mark the implementation landed"
guard_expect_fixed_in_file "$TAG" "nyrt_startup_gate_helper_owner=src/config/env/stage1.rs" "$DOC" "P4 doc must name the helper owner"
guard_expect_fixed_in_file "$TAG" "nyrt_entry_uses_shared_startup_gate_helpers=1" "$DOC" "P4 doc must record entry helper usage"
guard_expect_fixed_in_file "$TAG" "nyrt_startup_gates_shared=1" "$DOC" "P4 doc must record gate sharing"
guard_expect_fixed_in_file "$TAG" "direct_nyrt_startup_gate_reads_in_entry=0" "$DOC" "P4 doc must forbid direct entry reads"
guard_expect_fixed_in_file "$TAG" "docs/development/current/main/design/nyrt-startup-env-p4-centralization-ssot.md" "$ENV_REF" "env reference must point at the P4 doc"
guard_expect_fixed_in_file "$TAG" "NYRT-ENV-007" "$CARD" "phase card must mention the P4 implementation row"
guard_expect_fixed_in_file "$TAG" "NYRT-ENV-007" "$CURRENT_STATE" "current state must mention the P4 implementation row"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "NYRT-ENV-007" "$PRIORITY_DOC" "priority doc must mention the P4 landed row"
guard_expect_fixed_in_file "$TAG" "NYRT-ENV-006" "$P3_DOC" "P3 doc must remain landed before P4"
guard_expect_fixed_in_file "$TAG" "nyrt_plugin_host_mode()" "$STAGE1_HELPER" "shared helper must expose the plugin-host gate"
guard_expect_fixed_in_file "$TAG" "nyrt_runtime_hooks_mode()" "$STAGE1_HELPER" "shared helper must expose the runtime-hooks gate"
guard_expect_fixed_in_file "$TAG" "nyrt_runtime_build_mode()" "$STAGE1_HELPER" "shared helper must expose the runtime-build gate"
guard_expect_fixed_in_file "$TAG" "nyrt_entry_path_prep_mode()" "$STAGE1_HELPER" "shared helper must expose the entry-path-prep gate"
guard_expect_fixed_in_file "$TAG" "nyrt_ring0_init_mode()" "$STAGE1_HELPER" "shared helper must expose the ring0 gate"
guard_expect_fixed_in_file "$TAG" "nyrt_plugin_host_mode()" "$ENTRY" "entry must use the shared plugin-host helper"
guard_expect_fixed_in_file "$TAG" "nyrt_runtime_hooks_mode()" "$ENTRY" "entry must use the shared runtime-hooks helper"
guard_expect_fixed_in_file "$TAG" "nyrt_runtime_build_mode()" "$ENTRY" "entry must use the shared runtime-build helper"
guard_expect_fixed_in_file "$TAG" "nyrt_entry_path_prep_mode()" "$ENTRY" "entry must use the shared entry-path-prep helper"
guard_expect_fixed_in_file "$TAG" "nyrt_ring0_init_mode()" "$ENTRY" "entry must use the shared ring0 helper"

if rg -F -q 'std::env::var("HAKO_NYRT_PLUGIN_HOST")' "$ENTRY"; then
  guard_fail "$TAG" "entry must not read HAKO_NYRT_PLUGIN_HOST directly anymore"
fi
if rg -F -q 'std::env::var("NYASH_NYRT_RUNTIME_HOOKS")' "$ENTRY"; then
  guard_fail "$TAG" "entry must not read NYASH_NYRT_RUNTIME_HOOKS directly anymore"
fi
if rg -F -q 'std::env::var("NYASH_NYRT_RUNTIME_BUILD")' "$ENTRY"; then
  guard_fail "$TAG" "entry must not read NYASH_NYRT_RUNTIME_BUILD directly anymore"
fi
if rg -F -q 'std::env::var("NYASH_NYRT_ENTRY_PATH_PREP")' "$ENTRY"; then
  guard_fail "$TAG" "entry must not read NYASH_NYRT_ENTRY_PATH_PREP directly anymore"
fi
if rg -F -q 'std::env::var("NYASH_NYRT_RING0_INIT")' "$ENTRY"; then
  guard_fail "$TAG" "entry must not read NYASH_NYRT_RING0_INIT directly anymore"
fi

tmp_dir="$(mktemp -d /tmp/hakorune_nyrt_env_p4_centralization.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
cat >"$report" <<'REPORT'
output_contract=nyrt-startup-env-p4-centralization-v0
input_contract=nyrt-startup-env-p3-centralization-v0
nyrt_p4_centralization_landed=1
nyrt_startup_gate_helper_owner=src/config/env/stage1.rs
nyrt_entry_uses_shared_startup_gate_helpers=1
nyrt_startup_gates_shared=1
direct_nyrt_startup_gate_reads_in_entry=0
summary=ok
REPORT
cat "$report"

guard_expect_fixed_in_file "$TAG" "output_contract=nyrt-startup-env-p4-centralization-v0" "$report" "P4 report must carry the output contract"
guard_expect_fixed_in_file "$TAG" "input_contract=nyrt-startup-env-p3-centralization-v0" "$report" "P4 report must point at the P3 slice"
guard_expect_fixed_in_file "$TAG" "nyrt_p4_centralization_landed=1" "$report" "P4 report must mark the implementation landed"
guard_expect_fixed_in_file "$TAG" "nyrt_entry_uses_shared_startup_gate_helpers=1" "$report" "P4 report must record entry helper usage"
guard_expect_fixed_in_file "$TAG" "summary=ok" "$report" "P4 report must finish cleanly"

echo "[$TAG] ok"
