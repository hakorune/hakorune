#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-mimalloc-provider-package-hako-semantic-ping-pilot"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_33="docs/development/current/main/phases/phase-296x/296x-33-MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-PING-PILOT.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
CLI_ARGS="src/cli/args.rs"
CLI_MOD="src/cli/mod.rs"
CLI_IMPL="src/cli/provider_package_hako_derived_build.rs"
FIXTURE="apps/provider-package/hako-derived-allocator-fixture/main.hako"
NOOP_TOOL="tools/allocator/provider_package_noop_call_smoke.py"
METADATA_TOOL="tools/allocator/provider_package_metadata_preflight.py"
DESCRIPTOR_TOOL="tools/allocator/provider_package_descriptor_smoke.py"
API_BIND_TOOL="tools/allocator/provider_package_api_bind_smoke.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_provider_package_hako_semantic_ping_pilot_guard.sh"

echo "[$TAG] checking phase-296x .hako semantic ping pilot"

guard_require_files "$TAG" "$CARD_33" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$CLI_ARGS" "$CLI_MOD" "$CLI_IMPL" "$FIXTURE" "$NOOP_TOOL" "$METADATA_TOOL" "$DESCRIPTOR_TOOL" "$API_BIND_TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$NOOP_TOOL" "$METADATA_TOOL" "$DESCRIPTOR_TOOL" "$API_BIND_TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_33" "pilot card must be landed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-PING-PILOT-296X-001' "$CARD_33" "pilot card must identify blocker"
guard_expect_fixed_in_file "$TAG" '--provider-package-hako-semantic-codegen ping-literal-v0' "$CARD_33" "pilot card must document semantic CLI mode"
guard_expect_fixed_in_file "$TAG" 'hako_semantic_provider_codegen=ping-literal-v0' "$CARD_33" "pilot card must require semantic mode"
guard_expect_fixed_in_file "$TAG" 'hako_provider_ping_codegen=1' "$CARD_33" "pilot card must require ping codegen"
guard_expect_fixed_in_file "$TAG" 'hako_provider_ping_value=7' "$CARD_33" "pilot card must require ping value"
guard_expect_fixed_in_file "$TAG" 'provider_noop_call_result=7' "$CARD_33" "pilot card must require noop value"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$CARD_33" "pilot card must keep provider inactive"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$CARD_33" "pilot card must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_33" "pilot card must keep winners closed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-PING-CLOSEOUT-296X-001' "$CARD_33" "pilot card must select closeout"

guard_expect_fixed_in_file "$TAG" 'Arg::new("provider-package-hako-semantic-codegen")' "$CLI_ARGS" "CLI args must expose semantic codegen option"
guard_expect_fixed_in_file "$TAG" 'provider_package_hako_semantic_codegen: Option<String>' "$CLI_MOD" "CLI config must carry semantic codegen mode"
guard_expect_fixed_in_file "$TAG" 'extract_hako_provider_ping_literal' "$CLI_IMPL" "CLI impl must extract ping literal from MIR JSON"
guard_expect_fixed_in_file "$TAG" 'HakoProvider.ping/0' "$CLI_IMPL" "CLI impl must target HakoProvider.ping/0"
guard_expect_fixed_in_file "$TAG" 'hako_provider_ping_value' "$CLI_IMPL" "CLI impl must emit ping value"
guard_expect_fixed_in_file "$TAG" 'provider_call_executed=0' "$CLI_IMPL" "CLI package command must not call provider"

guard_expect_fixed_in_file "$TAG" 'static box HakoProvider' "$FIXTURE" "fixture must define provider box"
guard_expect_fixed_in_file "$TAG" 'ping()' "$FIXTURE" "fixture must define ping"
guard_expect_fixed_in_file "$TAG" 'return 7' "$FIXTURE" "fixture ping must return selected literal"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-33-MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-PING-PILOT"' "$CURRENT_STATE" "current state latest card must advance"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-PING-CLOSEOUT-296X-001"' "$CURRENT_STATE" "current state must select ping closeout"

guard_expect_fixed_in_file "$TAG" '| 33 | `MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-PING-PILOT-296X-001` | Landed |' "$TASKBOARD" "taskboard row 33 must be landed"
guard_expect_fixed_in_file "$TAG" '| 34 | `MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-PING-CLOSEOUT-296X-001` | Current |' "$TASKBOARD" "taskboard row 34 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list semantic ping pilot guard"

python3 -m py_compile "$NOOP_TOOL" "$METADATA_TOOL" "$DESCRIPTOR_TOOL" "$API_BIND_TOOL"
cargo build -q --bin hakorune

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_hako_semantic_ping.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
pkg="$tmp_dir/pkg"
build_out="$tmp_dir/build.out"
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

python3 "$NOOP_TOOL" --manifest "$pkg/hakorune_provider.json" --out "$noop_out"

guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-hako-derived-build-v0' "$build_out" "semantic build must emit package contract"
guard_expect_fixed_in_file "$TAG" 'hako_semantic_provider_codegen=ping-literal-v0' "$build_out" "semantic build must use ping mode"
guard_expect_fixed_in_file "$TAG" 'hako_provider_ping_codegen=1' "$build_out" "semantic build must codegen ping"
guard_expect_fixed_in_file "$TAG" 'hako_provider_ping_value=7' "$build_out" "semantic build must extract ping value"
guard_expect_fixed_in_file "$TAG" 'provider_call_executed=0' "$build_out" "package command must not call provider"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$build_out" "package command must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$build_out" "package command must keep winners closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$build_out" "semantic build must end ok"

guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-noop-call-smoke-v0' "$noop_out" "noop smoke must pass"
guard_expect_fixed_in_file "$TAG" 'provider_call_executed=1' "$noop_out" "noop smoke must call provider"
guard_expect_fixed_in_file "$TAG" 'provider_noop_call_executed=1' "$noop_out" "noop smoke must call ping"
guard_expect_fixed_in_file "$TAG" 'provider_noop_call_result=7' "$noop_out" "noop smoke must observe .hako ping value"
guard_expect_fixed_in_file "$TAG" 'allocator_entrypoint_called=0' "$noop_out" "noop smoke must not call allocator"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$noop_out" "noop smoke must keep provider inactive"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$noop_out" "noop smoke must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$noop_out" "noop smoke must keep winners closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$noop_out" "noop smoke must end ok"

cat "$build_out"
cat "$noop_out"
echo "[$TAG] ok"
