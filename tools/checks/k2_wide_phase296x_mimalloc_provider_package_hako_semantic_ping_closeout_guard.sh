#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-mimalloc-provider-package-hako-semantic-ping-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_34="docs/development/current/main/phases/phase-296x/296x-34-MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-PING-CLOSEOUT.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
FIXTURE="apps/provider-package/hako-derived-allocator-fixture/main.hako"
METADATA_TOOL="tools/allocator/provider_package_metadata_preflight.py"
DESCRIPTOR_TOOL="tools/allocator/provider_package_descriptor_smoke.py"
API_BIND_TOOL="tools/allocator/provider_package_api_bind_smoke.py"
NOOP_TOOL="tools/allocator/provider_package_noop_call_smoke.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_provider_package_hako_semantic_ping_closeout_guard.sh"

echo "[$TAG] checking phase-296x .hako semantic ping closeout"

guard_require_files "$TAG" "$CARD_34" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$FIXTURE" "$METADATA_TOOL" "$DESCRIPTOR_TOOL" "$API_BIND_TOOL" "$NOOP_TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$METADATA_TOOL" "$DESCRIPTOR_TOOL" "$API_BIND_TOOL" "$NOOP_TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_34" "closeout card must be landed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-PING-CLOSEOUT-296X-001' "$CARD_34" "closeout card must identify blocker"
guard_expect_fixed_in_file "$TAG" 'hako_semantic_provider_codegen=ping-literal-v0' "$CARD_34" "closeout card must require semantic mode"
guard_expect_fixed_in_file "$TAG" 'hako_provider_ping_value=7' "$CARD_34" "closeout card must require ping value"
guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-metadata-preflight-v0' "$CARD_34" "closeout card must include metadata evidence"
guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-descriptor-smoke-v0' "$CARD_34" "closeout card must include descriptor evidence"
guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-api-bind-smoke-v0' "$CARD_34" "closeout card must include API bind evidence"
guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-noop-call-smoke-v0' "$CARD_34" "closeout card must include noop evidence"
guard_expect_fixed_in_file "$TAG" 'provider_noop_call_result=7' "$CARD_34" "closeout card must prove semantic ping value"
guard_expect_fixed_in_file "$TAG" 'allocator_entrypoint_called=0' "$CARD_34" "closeout card must keep allocator entrypoints closed"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$CARD_34" "closeout card must keep provider inactive"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$CARD_34" "closeout card must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_34" "closeout card must keep winners closed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-ALLOC-FREE-SELECTION-296X-001' "$CARD_34" "closeout card must select allocator semantic boundary"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-34-MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-PING-CLOSEOUT"' "$CURRENT_STATE" "current state latest card must advance"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-ALLOC-FREE-SELECTION-296X-001"' "$CURRENT_STATE" "current state must select alloc/free semantic selection"
guard_expect_fixed_in_file "$TAG" '| 34 | `MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-PING-CLOSEOUT-296X-001` | Landed |' "$TASKBOARD" "taskboard row 34 must be landed"
guard_expect_fixed_in_file "$TAG" '| 35 | `MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-ALLOC-FREE-SELECTION-296X-001` | Current |' "$TASKBOARD" "taskboard row 35 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list semantic ping closeout guard"

python3 -m py_compile "$METADATA_TOOL" "$DESCRIPTOR_TOOL" "$API_BIND_TOOL" "$NOOP_TOOL"
cargo build -q --bin hakorune

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_hako_semantic_ping_closeout.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
pkg="$tmp_dir/pkg"
build_out="$tmp_dir/build.out"
metadata_out="$tmp_dir/metadata.out"
descriptor_out="$tmp_dir/descriptor.out"
api_bind_out="$tmp_dir/api_bind.out"
noop_out="$tmp_dir/noop.out"

target/debug/hakorune \
  --provider-package-hako-derived-build-fixture "$FIXTURE" \
  --provider-package-hako-semantic-codegen ping-literal-v0 \
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

guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-hako-derived-build-v0' "$build_out" "semantic build must emit package contract"
guard_expect_fixed_in_file "$TAG" 'hako_semantic_provider_codegen=ping-literal-v0' "$build_out" "semantic build must use ping mode"
guard_expect_fixed_in_file "$TAG" 'hako_provider_ping_codegen=1' "$build_out" "semantic build must codegen ping"
guard_expect_fixed_in_file "$TAG" 'hako_provider_ping_value=7' "$build_out" "semantic build must extract ping value"
guard_expect_fixed_in_file "$TAG" 'shared_library_artifact_generated=1' "$build_out" "semantic build must generate shared library"
guard_expect_fixed_in_file "$TAG" 'provider_call_executed=0' "$build_out" "package command must not call provider"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$build_out" "semantic build must end ok"

guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-metadata-preflight-v0' "$metadata_out" "metadata preflight must pass"
guard_expect_fixed_in_file "$TAG" 'dll_mode=metadata-preflight' "$metadata_out" "metadata preflight mode must match"
guard_expect_fixed_in_file "$TAG" 'shared_library_load_executed=0' "$metadata_out" "metadata preflight must not load shared library"
guard_expect_fixed_in_file "$TAG" 'descriptor_ready=0' "$metadata_out" "metadata preflight must not read descriptor"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$metadata_out" "metadata preflight must end ok"

guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-descriptor-smoke-v0' "$descriptor_out" "descriptor smoke must pass"
guard_expect_fixed_in_file "$TAG" 'dll_mode=descriptor-smoke' "$descriptor_out" "descriptor mode must match"
guard_expect_fixed_in_file "$TAG" 'required_export_resolved=1' "$descriptor_out" "descriptor smoke must resolve export"
guard_expect_fixed_in_file "$TAG" 'descriptor_read_executed=1' "$descriptor_out" "descriptor smoke must read descriptor"
guard_expect_fixed_in_file "$TAG" 'provider_call_executed=0' "$descriptor_out" "descriptor smoke must not call provider"
guard_expect_fixed_in_file "$TAG" 'allocator_entrypoint_called=0' "$descriptor_out" "descriptor smoke must not call allocator"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$descriptor_out" "descriptor smoke must end ok"

guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-api-bind-smoke-v0' "$api_bind_out" "API bind smoke must pass"
guard_expect_fixed_in_file "$TAG" 'dll_mode=provider-api-bind' "$api_bind_out" "API bind mode must match"
guard_expect_fixed_in_file "$TAG" 'provider_api_bound=1' "$api_bind_out" "API bind smoke must bind API"
guard_expect_fixed_in_file "$TAG" 'provider_call_executed=0' "$api_bind_out" "API bind smoke must not call provider"
guard_expect_fixed_in_file "$TAG" 'allocator_entrypoint_called=0' "$api_bind_out" "API bind smoke must not call allocator"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$api_bind_out" "API bind smoke must end ok"

guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-noop-call-smoke-v0' "$noop_out" "noop smoke must pass"
guard_expect_fixed_in_file "$TAG" 'dll_mode=provider-noop-call' "$noop_out" "noop mode must match"
guard_expect_fixed_in_file "$TAG" 'provider_call_executed=1' "$noop_out" "noop smoke must call provider"
guard_expect_fixed_in_file "$TAG" 'provider_noop_call_executed=1' "$noop_out" "noop smoke must call ping"
guard_expect_fixed_in_file "$TAG" 'provider_noop_call_result=7' "$noop_out" "noop smoke must observe .hako ping value"
guard_expect_fixed_in_file "$TAG" 'allocator_entrypoint_called=0' "$noop_out" "noop smoke must not call allocator"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$noop_out" "noop smoke must keep provider inactive"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$noop_out" "noop smoke must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$noop_out" "noop smoke must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$noop_out" "noop smoke must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$noop_out" "noop smoke must keep winners closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$noop_out" "noop smoke must end ok"

cat "$build_out"
cat "$metadata_out"
cat "$descriptor_out"
cat "$api_bind_out"
cat "$noop_out"
echo "[$TAG] ok"
