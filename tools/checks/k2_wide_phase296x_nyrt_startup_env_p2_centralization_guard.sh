#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-nyrt-startup-env-p2-centralization"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

DOC="docs/development/current/main/design/nyrt-startup-env-p2-centralization-ssot.md"
PRIORITY_DOC="docs/development/current/main/design/nyrt-startup-env-centralization-priority-ssot.md"
P1_DOC="docs/development/current/main/design/nyrt-startup-env-p1-centralization-ssot.md"
INVENTORY="docs/development/current/main/design/nyrt-startup-env-read-inventory-ssot.md"
CARD="docs/development/current/main/phases/phase-296x/296x-651-HAKO-MIMALLOC-TYPED-OBJECT-EXACT-SLOT-ABI-SPLIT.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
ENV_REF="docs/reference/environment-variables.md"
ENTRY="crates/nyash_kernel/src/entry.rs"
STAGE1_HELPER="src/config/env/stage1.rs"
MIR_FLAGS="src/config/env/mir_flags.rs"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_nyrt_startup_env_p2_centralization_guard.sh"

echo "[$TAG] checking NyRT startup env P2 centralization"

guard_require_files "$TAG" \
  "$DOC" "$PRIORITY_DOC" "$P1_DOC" "$INVENTORY" "$CARD" "$CURRENT_STATE" \
  "$INDEX" "$ENV_REF" "$ENTRY" "$STAGE1_HELPER" "$MIR_FLAGS" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" "NYRT-ENV-005" "$DOC" "P2 doc must name the implementation row"
guard_expect_fixed_in_file "$TAG" "nyrt_p2_centralization_landed=1" "$DOC" "P2 doc must mark the implementation landed"
guard_expect_fixed_in_file "$TAG" "nyrt_gc_telemetry_helper_owner=src/config/env/stage1.rs" "$DOC" "P2 doc must name the helper owner"
guard_expect_fixed_in_file "$TAG" "nyrt_entry_uses_shared_gc_telemetry_helper=1" "$DOC" "P2 doc must record entry helper usage"
guard_expect_fixed_in_file "$TAG" "nyrt_gc_telemetry_cluster_shared=1" "$DOC" "P2 doc must record cluster sharing"
guard_expect_fixed_in_file "$TAG" "nyrt_gc_warning_threshold_shared=1" "$DOC" "P2 doc must record warning threshold sharing"
guard_expect_fixed_in_file "$TAG" "direct_nyrt_gc_telemetry_reads_in_entry=0" "$DOC" "P2 doc must forbid direct entry reads"
guard_expect_fixed_in_file "$TAG" "docs/development/current/main/design/nyrt-startup-env-p2-centralization-ssot.md" "$ENV_REF" "env reference must point at the P2 doc"
guard_expect_fixed_in_file "$TAG" "NYRT-ENV-005" "$CARD" "phase card must mention the P2 implementation row"
guard_expect_fixed_in_file "$TAG" "NYRT-ENV-005" "$CURRENT_STATE" "current state must mention the P2 implementation row"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "NYRT-ENV-005" "$PRIORITY_DOC" "priority doc must mention the P2 landed row"
guard_expect_fixed_in_file "$TAG" "NYRT-ENV-004" "$P1_DOC" "P1 doc must remain landed before P2"
guard_expect_fixed_in_file "$TAG" "nyrt_gc_collect_sp_interval()" "$STAGE1_HELPER" "shared helper must expose the SP interval"
guard_expect_fixed_in_file "$TAG" "nyrt_gc_collect_alloc_bytes()" "$STAGE1_HELPER" "shared helper must expose the alloc interval"
guard_expect_fixed_in_file "$TAG" "nyrt_llvm_auto_safepoint_enabled()" "$STAGE1_HELPER" "shared helper must expose auto safepoint"
guard_expect_fixed_in_file "$TAG" "nyrt_gc_alloc_threshold_bytes()" "$STAGE1_HELPER" "shared helper must expose GC threshold"
guard_expect_fixed_in_file "$TAG" "nyrt_gc_collect_sp_interval()" "$ENTRY" "entry must use the shared SP helper"
guard_expect_fixed_in_file "$TAG" "nyrt_gc_collect_alloc_bytes()" "$ENTRY" "entry must use the shared alloc helper"
guard_expect_fixed_in_file "$TAG" "nyrt_llvm_auto_safepoint_enabled()" "$ENTRY" "entry must use the shared auto safepoint helper"
guard_expect_fixed_in_file "$TAG" "nyrt_gc_alloc_threshold_bytes()" "$ENTRY" "entry must use the shared threshold helper"

if rg -F -q 'flag_on("NYASH_LLVM_AUTO_SAFEPOINT")' "$ENTRY"; then
  guard_fail "$TAG" "entry must not read NYASH_LLVM_AUTO_SAFEPOINT directly anymore"
fi
if rg -F -q 'u64_or("NYASH_GC_COLLECT_SP"' "$ENTRY"; then
  guard_fail "$TAG" "entry must not read NYASH_GC_COLLECT_SP directly anymore"
fi
if rg -F -q 'u64_or("NYASH_GC_COLLECT_ALLOC"' "$ENTRY"; then
  guard_fail "$TAG" "entry must not read NYASH_GC_COLLECT_ALLOC directly anymore"
fi
if rg -F -q 'u64_or("NYASH_GC_ALLOC_THRESHOLD"' "$ENTRY"; then
  guard_fail "$TAG" "entry must not read NYASH_GC_ALLOC_THRESHOLD directly anymore"
fi

tmp_dir="$(mktemp -d /tmp/hakorune_nyrt_env_p2_centralization.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
cat >"$report" <<'REPORT'
output_contract=nyrt-startup-env-p2-centralization-v0
input_contract=nyrt-startup-env-p1-centralization-v0
nyrt_p2_centralization_landed=1
nyrt_gc_telemetry_helper_owner=src/config/env/stage1.rs
nyrt_entry_uses_shared_gc_telemetry_helper=1
nyrt_gc_telemetry_cluster_shared=1
nyrt_gc_warning_threshold_shared=1
direct_nyrt_gc_telemetry_reads_in_entry=0
summary=ok
REPORT
cat "$report"

guard_expect_fixed_in_file "$TAG" "output_contract=nyrt-startup-env-p2-centralization-v0" "$report" "P2 report must carry the output contract"
guard_expect_fixed_in_file "$TAG" "input_contract=nyrt-startup-env-p1-centralization-v0" "$report" "P2 report must point at the P1 slice"
guard_expect_fixed_in_file "$TAG" "nyrt_p2_centralization_landed=1" "$report" "P2 report must mark the implementation landed"
guard_expect_fixed_in_file "$TAG" "nyrt_entry_uses_shared_gc_telemetry_helper=1" "$report" "P2 report must record entry helper usage"
guard_expect_fixed_in_file "$TAG" "summary=ok" "$report" "P2 report must finish cleanly"

echo "[$TAG] ok"
