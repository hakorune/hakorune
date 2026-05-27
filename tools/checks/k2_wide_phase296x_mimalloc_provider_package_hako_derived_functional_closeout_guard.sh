#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-mimalloc-provider-package-hako-derived-functional-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_38="docs/development/current/main/phases/phase-296x/296x-38-MIMALLOC-PROVIDER-PACKAGE-HAKO-DERIVED-FUNCTIONAL-CLOSEOUT.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
REFERENCE="docs/reference/runtime/provider-package-v0.md"
SSOT="docs/development/current/main/design/provider-package-artifact-ssot.md"
CLI_IMPL="src/cli/provider_package_hako_derived_build.rs"
FIXTURE="apps/provider-package/hako-derived-allocator-fixture/main.hako"
METADATA_TOOL="tools/allocator/provider_package_metadata_preflight.py"
DESCRIPTOR_TOOL="tools/allocator/provider_package_descriptor_smoke.py"
API_BIND_TOOL="tools/allocator/provider_package_api_bind_smoke.py"
NOOP_TOOL="tools/allocator/provider_package_noop_call_smoke.py"
ALLOC_FREE_TOOL="tools/allocator/provider_package_alloc_free_smoke.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_provider_package_hako_derived_functional_closeout_guard.sh"

echo "[$TAG] checking phase-296x .hako-derived provider package functional closeout"

guard_require_files "$TAG" "$CARD_38" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$REFERENCE" "$SSOT" "$CLI_IMPL" "$FIXTURE" "$METADATA_TOOL" "$DESCRIPTOR_TOOL" "$API_BIND_TOOL" "$NOOP_TOOL" "$ALLOC_FREE_TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$METADATA_TOOL" "$DESCRIPTOR_TOOL" "$API_BIND_TOOL" "$NOOP_TOOL" "$ALLOC_FREE_TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_38" "functional closeout card must be landed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-HAKO-DERIVED-FUNCTIONAL-CLOSEOUT-296X-001' "$CARD_38" "functional closeout card must identify blocker"
guard_expect_fixed_in_file "$TAG" 'selected .hako source' "$CARD_38" "functional closeout card must start from .hako source"
guard_expect_fixed_in_file "$TAG" 'MIR JSON emission' "$CARD_38" "functional closeout card must include MIR emission"
guard_expect_fixed_in_file "$TAG" 'generated shared-library provider artifact' "$CARD_38" "functional closeout card must include shared library generation"
guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-alloc-free-smoke-v0' "$CARD_38" "functional closeout card must include alloc/free smoke"
guard_expect_fixed_in_file "$TAG" 'provider_noop_call_result=7' "$CARD_38" "functional closeout card must include ping evidence"
guard_expect_fixed_in_file "$TAG" 'provider_owns_result=1' "$CARD_38" "functional closeout card must include owns evidence"
guard_expect_fixed_in_file "$TAG" 'Native pointer allocation/free mechanics are still owned by the generated' "$CARD_38" "functional closeout card must keep native allocation mechanics separate"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-BENCHMARK-RETURN-SELECTION-296X-001' "$CARD_38" "functional closeout card must select benchmark return"

guard_expect_fixed_in_file "$TAG" 'alloc-free-owns-literal-v0' "$REFERENCE" "runtime docs must document alloc/free semantic mode"
guard_expect_fixed_in_file "$TAG" 'alloc-free-owns-literal-v0' "$SSOT" "artifact SSOT must document alloc/free semantic mode"
guard_expect_fixed_in_file "$TAG" 'alloc-free-owns-literal-v0' "$CLI_IMPL" "CLI impl must support alloc/free semantic mode"
guard_expect_fixed_in_file "$TAG" 'hako_provider_owns_value' "$CLI_IMPL" "CLI impl must emit owns semantic evidence"
guard_expect_fixed_in_file "$TAG" 'ownsAllocated()' "$FIXTURE" "fixture must retain ownsAllocated semantic source"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-38-MIMALLOC-PROVIDER-PACKAGE-HAKO-DERIVED-FUNCTIONAL-CLOSEOUT"' "$CURRENT_STATE" "current state latest card must advance"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "MIMALLOC-PROVIDER-PACKAGE-BENCHMARK-RETURN-SELECTION-296X-001"' "$CURRENT_STATE" "current state must select benchmark return"
guard_expect_fixed_in_file "$TAG" '| 38 | `MIMALLOC-PROVIDER-PACKAGE-HAKO-DERIVED-FUNCTIONAL-CLOSEOUT-296X-001` | Landed |' "$TASKBOARD" "taskboard row 38 must be landed"
guard_expect_fixed_in_file "$TAG" '| 39 | `MIMALLOC-PROVIDER-PACKAGE-BENCHMARK-RETURN-SELECTION-296X-001` | Current |' "$TASKBOARD" "taskboard row 39 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list functional closeout guard"

python3 -m py_compile "$METADATA_TOOL" "$DESCRIPTOR_TOOL" "$API_BIND_TOOL" "$NOOP_TOOL" "$ALLOC_FREE_TOOL"
cargo build -q --bin hakorune

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_hako_functional_closeout.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
pkg="$tmp_dir/pkg"
build_out="$tmp_dir/build.out"
metadata_out="$tmp_dir/metadata.out"
descriptor_out="$tmp_dir/descriptor.out"
api_bind_out="$tmp_dir/api_bind.out"
noop_out="$tmp_dir/noop.out"
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

python3 "$METADATA_TOOL" --manifest "$pkg/hakorune_provider.json" --out "$metadata_out"
python3 "$DESCRIPTOR_TOOL" --manifest "$pkg/hakorune_provider.json" --out "$descriptor_out"
python3 "$API_BIND_TOOL" --manifest "$pkg/hakorune_provider.json" --out "$api_bind_out"
python3 "$NOOP_TOOL" --manifest "$pkg/hakorune_provider.json" --out "$noop_out"
python3 "$ALLOC_FREE_TOOL" --manifest "$pkg/hakorune_provider.json" --size 32 --align 8 --out "$alloc_out"

guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-hako-derived-build-v0' "$build_out" "build must emit .hako-derived package contract"
guard_expect_fixed_in_file "$TAG" 'package_mode=hako-derived-provider-package' "$build_out" "build must identify hako package mode"
guard_expect_fixed_in_file "$TAG" 'hako_source_checked=1' "$build_out" "build must check .hako source"
guard_expect_fixed_in_file "$TAG" 'hako_mir_json_emitted=1' "$build_out" "build must emit MIR JSON"
guard_expect_fixed_in_file "$TAG" 'hako_semantic_provider_codegen=alloc-free-owns-literal-v0' "$build_out" "build must use final v0 semantic mode"
guard_expect_fixed_in_file "$TAG" 'hako_provider_ping_value=7' "$build_out" "build must extract ping value"
guard_expect_fixed_in_file "$TAG" 'hako_provider_owns_value=1' "$build_out" "build must extract owns value"
guard_expect_fixed_in_file "$TAG" 'shared_library_artifact_generated=1' "$build_out" "build must generate shared library"
guard_expect_fixed_in_file "$TAG" 'shared_library_load_executed=0' "$build_out" "build command must not load shared library"
guard_expect_fixed_in_file "$TAG" 'provider_call_executed=0' "$build_out" "build command must not call provider"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$build_out" "build must end ok"

guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-metadata-preflight-v0' "$metadata_out" "metadata preflight must pass"
guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-descriptor-smoke-v0' "$descriptor_out" "descriptor smoke must pass"
guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-api-bind-smoke-v0' "$api_bind_out" "API bind smoke must pass"
guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-noop-call-smoke-v0' "$noop_out" "noop smoke must pass"
guard_expect_fixed_in_file "$TAG" 'provider_noop_call_result=7' "$noop_out" "noop smoke must observe ping value"
guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-alloc-free-smoke-v0' "$alloc_out" "alloc/free smoke must pass"
guard_expect_fixed_in_file "$TAG" 'provider_owns_result=1' "$alloc_out" "alloc/free smoke must observe owns value"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$alloc_out" "provider must stay inactive"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$alloc_out" "replacement must stay closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$alloc_out" "hooks must stay closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$alloc_out" "global allocator must stay closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$alloc_out" "winner claims must stay closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$alloc_out" "alloc/free smoke must end ok"

cat "$build_out"
cat "$metadata_out"
cat "$descriptor_out"
cat "$api_bind_out"
cat "$noop_out"
cat "$alloc_out"
echo "[$TAG] ok"
