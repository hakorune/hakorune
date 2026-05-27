#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-mimalloc-provider-package-hako-semantic-alloc-free-pilot"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_36="docs/development/current/main/phases/phase-296x/296x-36-MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-ALLOC-FREE-PILOT.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
CLI_ARGS="src/cli/args.rs"
CLI_IMPL="src/cli/provider_package_hako_derived_build.rs"
FIXTURE="apps/provider-package/hako-derived-allocator-fixture/main.hako"
ALLOC_FREE_TOOL="tools/allocator/provider_package_alloc_free_smoke.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_provider_package_hako_semantic_alloc_free_pilot_guard.sh"

echo "[$TAG] checking phase-296x .hako semantic alloc/free pilot"

guard_require_files "$TAG" "$CARD_36" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$CLI_ARGS" "$CLI_IMPL" "$FIXTURE" "$ALLOC_FREE_TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$ALLOC_FREE_TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_36" "pilot card must be landed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-ALLOC-FREE-PILOT-296X-001' "$CARD_36" "pilot card must identify blocker"
guard_expect_fixed_in_file "$TAG" '--provider-package-hako-semantic-codegen alloc-free-owns-literal-v0' "$CARD_36" "pilot card must document semantic CLI mode"
guard_expect_fixed_in_file "$TAG" 'HakoProvider.ownsAllocated/0 -> i64 literal 1' "$CARD_36" "pilot card must document owns literal"
guard_expect_fixed_in_file "$TAG" 'hako_provider_owns_codegen=1' "$CARD_36" "pilot card must require owns codegen"
guard_expect_fixed_in_file "$TAG" 'hako_provider_owns_value=1' "$CARD_36" "pilot card must require owns value"
guard_expect_fixed_in_file "$TAG" 'provider_alloc_executed=1' "$CARD_36" "pilot card must require alloc call"
guard_expect_fixed_in_file "$TAG" 'provider_free_executed=1' "$CARD_36" "pilot card must require free call"
guard_expect_fixed_in_file "$TAG" 'provider_owns_result=1' "$CARD_36" "pilot card must require owns result"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$CARD_36" "pilot card must keep provider inactive"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$CARD_36" "pilot card must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_36" "pilot card must keep winners closed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-ALLOC-FREE-CLOSEOUT-296X-001' "$CARD_36" "pilot card must select closeout"

guard_expect_fixed_in_file "$TAG" 'alloc-free-owns-literal-v0' "$CLI_ARGS" "CLI args must expose alloc/free semantic mode"
guard_expect_fixed_in_file "$TAG" 'extract_hako_provider_owns_allocated_literal' "$CLI_IMPL" "CLI impl must extract owns literal"
guard_expect_fixed_in_file "$TAG" 'HakoProvider.ownsAllocated/0' "$CLI_IMPL" "CLI impl must target HakoProvider.ownsAllocated/0"
guard_expect_fixed_in_file "$TAG" 'hako_provider_owns_value' "$CLI_IMPL" "CLI impl must emit owns value"
guard_expect_fixed_in_file "$TAG" '__OWNS_VALUE__' "$CLI_IMPL" "wrapper source must substitute owns value"
guard_expect_fixed_in_file "$TAG" 'provider_call_executed=0' "$CLI_IMPL" "package command must not call provider"

guard_expect_fixed_in_file "$TAG" 'ownsAllocated()' "$FIXTURE" "fixture must define ownsAllocated"
guard_expect_fixed_in_file "$TAG" 'return 1' "$FIXTURE" "fixture ownsAllocated must return selected literal"

guard_expect_fixed_in_file "$TAG" '296x-36 Added alloc-free-owns-literal-v0' "$CURRENT_STATE" "current state landed tail must retain row 36"
guard_expect_fixed_in_file "$TAG" '| 36 | `MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-ALLOC-FREE-PILOT-296X-001` | Landed |' "$TASKBOARD" "taskboard row 36 must be landed"
guard_expect_fixed_in_file "$TAG" '| 37 | `MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-ALLOC-FREE-CLOSEOUT-296X-001` | Landed |' "$TASKBOARD" "taskboard row 37 must be landed after closeout"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list semantic alloc/free pilot guard"

python3 -m py_compile "$ALLOC_FREE_TOOL"
cargo build -q --bin hakorune

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_hako_semantic_alloc_free.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
pkg="$tmp_dir/pkg"
build_out="$tmp_dir/build.out"
alloc_out="$tmp_dir/alloc.out"

target/debug/hakorune \
  --provider-package-hako-derived-build-fixture "$FIXTURE" \
  --provider-package-hako-semantic-codegen alloc-free-owns-literal-v0 \
  --provider-package-out-dir "$pkg" \
  --provider-package-artifact-name libhakorune_provider.so \
  --provider-package-id org.hakorune.provider.hako.fixture \
  --provider-package-name hako-derived-fixture-provider \
  --provider-package-version 0.1.0 \
  --provider-package-target-triple x86_64-unknown-linux-gnu \
  --provider-package-platform linux \
  --provider-package-provider-call-allowed \
  --provider-package-force \
  > "$build_out"

python3 "$ALLOC_FREE_TOOL" --manifest "$pkg/hakorune_provider.json" --size 32 --align 8 --out "$alloc_out"

guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-hako-derived-build-v0' "$build_out" "semantic build must emit package contract"
guard_expect_fixed_in_file "$TAG" 'hako_semantic_provider_codegen=alloc-free-owns-literal-v0' "$build_out" "semantic build must use alloc/free mode"
guard_expect_fixed_in_file "$TAG" 'hako_provider_ping_codegen=1' "$build_out" "semantic build must keep ping codegen"
guard_expect_fixed_in_file "$TAG" 'hako_provider_ping_value=7' "$build_out" "semantic build must extract ping value"
guard_expect_fixed_in_file "$TAG" 'hako_provider_owns_codegen=1' "$build_out" "semantic build must codegen owns"
guard_expect_fixed_in_file "$TAG" 'hako_provider_owns_value=1' "$build_out" "semantic build must extract owns value"
guard_expect_fixed_in_file "$TAG" 'provider_call_executed=0' "$build_out" "package command must not call provider"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$build_out" "semantic build must end ok"

guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-alloc-free-smoke-v0' "$alloc_out" "alloc/free smoke must pass"
guard_expect_fixed_in_file "$TAG" 'dll_mode=provider-alloc-free' "$alloc_out" "alloc/free smoke mode must match"
guard_expect_fixed_in_file "$TAG" 'provider_call_executed=1' "$alloc_out" "alloc/free smoke must call provider"
guard_expect_fixed_in_file "$TAG" 'allocator_entrypoint_called=1' "$alloc_out" "alloc/free smoke must call allocator entrypoint"
guard_expect_fixed_in_file "$TAG" 'provider_alloc_executed=1' "$alloc_out" "alloc/free smoke must call alloc"
guard_expect_fixed_in_file "$TAG" 'provider_free_executed=1' "$alloc_out" "alloc/free smoke must call free"
guard_expect_fixed_in_file "$TAG" 'provider_owns_result=1' "$alloc_out" "alloc/free smoke must observe .hako owns value"
guard_expect_fixed_in_file "$TAG" 'allocated_pointer_nonzero=1' "$alloc_out" "alloc/free smoke must allocate a pointer"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$alloc_out" "alloc/free smoke must keep provider inactive"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$alloc_out" "alloc/free smoke must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$alloc_out" "alloc/free smoke must keep winners closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$alloc_out" "alloc/free smoke must end ok"

cat "$build_out"
cat "$alloc_out"
echo "[$TAG] ok"
