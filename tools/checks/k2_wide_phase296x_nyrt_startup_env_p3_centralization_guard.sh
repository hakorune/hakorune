#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-nyrt-startup-env-p3-centralization"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

DOC="docs/development/current/main/design/nyrt-startup-env-p3-centralization-ssot.md"
PRIORITY_DOC="docs/development/current/main/design/nyrt-startup-env-centralization-priority-ssot.md"
P2_DOC="docs/development/current/main/design/nyrt-startup-env-p2-centralization-ssot.md"
INVENTORY="docs/development/current/main/design/nyrt-startup-env-read-inventory-ssot.md"
CARD="docs/development/current/main/phases/phase-296x/296x-651-HAKO-MIMALLOC-TYPED-OBJECT-EXACT-SLOT-ABI-SPLIT.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
ENV_REF="docs/reference/environment-variables.md"
ENTRY="crates/nyash_kernel/src/entry.rs"
STAGE1_HELPER="src/config/env/stage1.rs"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_nyrt_startup_env_p3_centralization_guard.sh"

echo "[$TAG] checking NyRT startup env P3 centralization"

guard_require_files "$TAG" \
  "$DOC" "$PRIORITY_DOC" "$P2_DOC" "$INVENTORY" "$CARD" "$CURRENT_STATE" \
  "$INDEX" "$ENV_REF" "$ENTRY" "$STAGE1_HELPER" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" "NYRT-ENV-006" "$DOC" "P3 doc must name the implementation row"
guard_expect_fixed_in_file "$TAG" "nyrt_p3_centralization_landed=1" "$DOC" "P3 doc must mark the implementation landed"
guard_expect_fixed_in_file "$TAG" "nyrt_minimal_startup_helper_owner=src/config/env/stage1.rs" "$DOC" "P3 doc must name the helper owner"
guard_expect_fixed_in_file "$TAG" "nyrt_entry_uses_shared_minimal_startup_helper=1" "$DOC" "P3 doc must record entry helper usage"
guard_expect_fixed_in_file "$TAG" "nyrt_minimal_startup_knob_shared=1" "$DOC" "P3 doc must record helper sharing"
guard_expect_fixed_in_file "$TAG" "direct_nyrt_minimal_startup_reads_in_entry=0" "$DOC" "P3 doc must forbid direct entry reads"
guard_expect_fixed_in_file "$TAG" "docs/development/current/main/design/nyrt-startup-env-p3-centralization-ssot.md" "$ENV_REF" "env reference must point at the P3 doc"
guard_expect_fixed_in_file "$TAG" "NYRT-ENV-006" "$CARD" "phase card must mention the P3 implementation row"
guard_expect_fixed_in_file "$TAG" "NYRT-ENV-006" "$CURRENT_STATE" "current state must mention the P3 implementation row"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "NYRT-ENV-006" "$PRIORITY_DOC" "priority doc must mention the P3 landed row"
guard_expect_fixed_in_file "$TAG" "NYRT-ENV-005" "$P2_DOC" "P2 doc must remain landed before P3"
guard_expect_fixed_in_file "$TAG" "nyrt_minimal_startup_enabled()" "$STAGE1_HELPER" "shared helper must expose the minimal-startup knob"
guard_expect_fixed_in_file "$TAG" "nyrt_minimal_startup_enabled()" "$ENTRY" "entry must use the shared minimal-startup helper"

if rg -F -q 'flag_on("NYASH_NYRT_MINIMAL_STARTUP")' "$ENTRY"; then
  guard_fail "$TAG" "entry must not read NYASH_NYRT_MINIMAL_STARTUP directly anymore"
fi

tmp_dir="$(mktemp -d /tmp/hakorune_nyrt_env_p3_centralization.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
cat >"$report" <<'REPORT'
output_contract=nyrt-startup-env-p3-centralization-v0
input_contract=nyrt-startup-env-p2-centralization-v0
nyrt_p3_centralization_landed=1
nyrt_minimal_startup_helper_owner=src/config/env/stage1.rs
nyrt_entry_uses_shared_minimal_startup_helper=1
nyrt_minimal_startup_knob_shared=1
direct_nyrt_minimal_startup_reads_in_entry=0
summary=ok
REPORT
cat "$report"

guard_expect_fixed_in_file "$TAG" "output_contract=nyrt-startup-env-p3-centralization-v0" "$report" "P3 report must carry the output contract"
guard_expect_fixed_in_file "$TAG" "input_contract=nyrt-startup-env-p2-centralization-v0" "$report" "P3 report must point at the P2 slice"
guard_expect_fixed_in_file "$TAG" "nyrt_p3_centralization_landed=1" "$report" "P3 report must mark the implementation landed"
guard_expect_fixed_in_file "$TAG" "nyrt_entry_uses_shared_minimal_startup_helper=1" "$report" "P3 report must record entry helper usage"
guard_expect_fixed_in_file "$TAG" "summary=ok" "$report" "P3 report must finish cleanly"

echo "[$TAG] ok"
