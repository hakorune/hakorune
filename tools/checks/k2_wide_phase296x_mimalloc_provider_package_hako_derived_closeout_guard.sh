#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-mimalloc-provider-package-hako-derived-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_30="docs/development/current/main/phases/phase-296x/296x-30-MIMALLOC-PROVIDER-PACKAGE-HAKO-DERIVED-MINIMAL-FIXTURE-PILOT.md"
CARD_31="docs/development/current/main/phases/phase-296x/296x-31-MIMALLOC-PROVIDER-PACKAGE-HAKO-DERIVED-CLOSEOUT.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
CLI_IMPL="src/cli/provider_package_hako_derived_build.rs"
FIXTURE="apps/provider-package/hako-derived-allocator-fixture/main.hako"
METADATA_TOOL="tools/allocator/provider_package_metadata_preflight.py"
DESCRIPTOR_TOOL="tools/allocator/provider_package_descriptor_smoke.py"
API_BIND_TOOL="tools/allocator/provider_package_api_bind_smoke.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_provider_package_hako_derived_closeout_guard.sh"

echo "[$TAG] checking phase-296x .hako-derived provider package closeout"

guard_require_files "$TAG" "$CARD_30" "$CARD_31" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$CLI_IMPL" "$FIXTURE" "$METADATA_TOOL" "$DESCRIPTOR_TOOL" "$API_BIND_TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$METADATA_TOOL" "$DESCRIPTOR_TOOL" "$API_BIND_TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_30" "minimal fixture pilot must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_31" "closeout card must be landed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-HAKO-DERIVED-CLOSEOUT-296X-001' "$CARD_31" "closeout card must identify blocker"
guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-hako-derived-build-v0' "$CARD_31" "closeout must preserve hako-derived contract"
guard_expect_fixed_in_file "$TAG" 'hako_source_checked=1' "$CARD_31" "closeout must require hako source evidence"
guard_expect_fixed_in_file "$TAG" 'hako_mir_json_emitted=1' "$CARD_31" "closeout must require MIR evidence"
guard_expect_fixed_in_file "$TAG" 'hako_semantic_provider_codegen=0' "$CARD_31" "closeout must keep semantic codegen closed"
guard_expect_fixed_in_file "$TAG" 'metadata-preflight=ok' "$CARD_31" "closeout must record metadata preflight evidence"
guard_expect_fixed_in_file "$TAG" 'descriptor-smoke=ok' "$CARD_31" "closeout must record descriptor evidence"
guard_expect_fixed_in_file "$TAG" 'provider-api-bind=ok' "$CARD_31" "closeout must record API bind evidence"
guard_expect_fixed_in_file "$TAG" 'descriptor_ready=1' "$CARD_31" "closeout must require descriptor ready"
guard_expect_fixed_in_file "$TAG" 'provider_api_bound=1' "$CARD_31" "closeout must require API bind"
guard_expect_fixed_in_file "$TAG" 'provider_call_executed=0' "$CARD_31" "closeout must not call provider"
guard_expect_fixed_in_file "$TAG" 'allocator_entrypoint_called=0' "$CARD_31" "closeout must not call allocator entrypoints"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$CARD_31" "closeout must keep provider inactive"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$CARD_31" "closeout must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$CARD_31" "closeout must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$CARD_31" "closeout must keep globals closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_31" "closeout must keep winners closed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-CODEGEN-SELECTION-296X-001' "$CARD_31" "closeout must select semantic codegen selection"

guard_expect_fixed_in_file "$TAG" 'hako_semantic_provider_codegen' "$CLI_IMPL" "CLI impl must expose semantic codegen stop line"
guard_expect_fixed_in_file "$TAG" 'provider_call_executed=0' "$CLI_IMPL" "CLI package command must not call provider"
guard_expect_fixed_in_file "$TAG" 'descriptor_read_executed=0' "$CLI_IMPL" "CLI package command must not read descriptor"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-31-MIMALLOC-PROVIDER-PACKAGE-HAKO-DERIVED-CLOSEOUT"' "$CURRENT_STATE" "current state latest card must advance"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-CODEGEN-SELECTION-296X-001"' "$CURRENT_STATE" "current state must select semantic codegen selection"

guard_expect_fixed_in_file "$TAG" '| 31 | `MIMALLOC-PROVIDER-PACKAGE-HAKO-DERIVED-CLOSEOUT-296X-001` | Landed |' "$TASKBOARD" "taskboard row 31 must be landed"
guard_expect_fixed_in_file "$TAG" '| 32 | `MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-CODEGEN-SELECTION-296X-001` | Current |' "$TASKBOARD" "taskboard row 32 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list hako-derived closeout guard"

python3 -m py_compile "$METADATA_TOOL" "$DESCRIPTOR_TOOL" "$API_BIND_TOOL"
cargo build -q --bin hakorune

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_hako_derived_closeout.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
pkg="$tmp_dir/pkg"
build_out="$tmp_dir/build.out"
metadata_out="$tmp_dir/metadata.out"
descriptor_out="$tmp_dir/descriptor.out"
api_bind_out="$tmp_dir/api-bind.out"

target/debug/hakorune \
  --provider-package-hako-derived-build-fixture "$FIXTURE" \
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

guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-hako-derived-build-v0' "$build_out" "hako-derived build must emit contract"
guard_expect_fixed_in_file "$TAG" 'hako_source_checked=1' "$build_out" "hako-derived build must check source"
guard_expect_fixed_in_file "$TAG" 'hako_mir_json_emitted=1' "$build_out" "hako-derived build must emit MIR JSON"
guard_expect_fixed_in_file "$TAG" 'hako_semantic_provider_codegen=0' "$build_out" "hako-derived build must keep semantic codegen closed"
guard_expect_fixed_in_file "$TAG" 'shared_library_artifact_generated=1' "$build_out" "hako-derived build must generate artifact"
guard_expect_fixed_in_file "$TAG" 'shared_library_load_executed=0' "$build_out" "hako-derived package command must stay no-load"
guard_expect_fixed_in_file "$TAG" 'provider_call_executed=0' "$build_out" "hako-derived package command must not call provider"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$build_out" "hako-derived package command must keep winners closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$build_out" "hako-derived build must end ok"

guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-metadata-preflight-v0' "$metadata_out" "metadata preflight must pass"
guard_expect_fixed_in_file "$TAG" 'shared_library_load_executed=0' "$metadata_out" "metadata preflight must not load"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$metadata_out" "metadata preflight must end ok"

guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-descriptor-smoke-v0' "$descriptor_out" "descriptor smoke must pass"
guard_expect_fixed_in_file "$TAG" 'descriptor_ready=1' "$descriptor_out" "descriptor smoke must read descriptor"
guard_expect_fixed_in_file "$TAG" 'provider_call_executed=0' "$descriptor_out" "descriptor smoke must not call provider"
guard_expect_fixed_in_file "$TAG" 'allocator_entrypoint_called=0' "$descriptor_out" "descriptor smoke must not call allocator"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$descriptor_out" "descriptor smoke must keep provider inactive"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$descriptor_out" "descriptor smoke must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$descriptor_out" "descriptor smoke must keep winners closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$descriptor_out" "descriptor smoke must end ok"

guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-api-bind-smoke-v0' "$api_bind_out" "API bind smoke must pass"
guard_expect_fixed_in_file "$TAG" 'provider_api_bound=1' "$api_bind_out" "API bind smoke must bind API table"
guard_expect_fixed_in_file "$TAG" 'provider_call_executed=0' "$api_bind_out" "API bind smoke must not call provider"
guard_expect_fixed_in_file "$TAG" 'allocator_entrypoint_called=0' "$api_bind_out" "API bind smoke must not call allocator"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$api_bind_out" "API bind smoke must keep provider inactive"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$api_bind_out" "API bind smoke must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$api_bind_out" "API bind smoke must keep winners closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$api_bind_out" "API bind smoke must end ok"

cat "$build_out"
cat "$metadata_out"
cat "$descriptor_out"
cat "$api_bind_out"
echo "[$TAG] ok"
