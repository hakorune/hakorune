#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-mimalloc-provider-package-hako-derived-minimal-fixture-pilot"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_30="docs/development/current/main/phases/phase-296x/296x-30-MIMALLOC-PROVIDER-PACKAGE-HAKO-DERIVED-MINIMAL-FIXTURE-PILOT.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
DOC="docs/reference/runtime/provider-package-v0.md"
INDEX="docs/tools/check-scripts-index.md"
CLI_ARGS="src/cli/args.rs"
CLI_MOD="src/cli/mod.rs"
CLI_IMPL="src/cli/provider_package_hako_derived_build.rs"
MAIN="src/main.rs"
FIXTURE="apps/provider-package/hako-derived-allocator-fixture/main.hako"
METADATA_TOOL="tools/allocator/provider_package_metadata_preflight.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_provider_package_hako_derived_minimal_fixture_pilot_guard.sh"

echo "[$TAG] checking phase-296x .hako-derived minimal fixture provider package pilot"

guard_require_files "$TAG" "$CARD_30" "$TASKBOARD" "$CURRENT_STATE" "$DOC" "$INDEX" "$CLI_ARGS" "$CLI_MOD" "$CLI_IMPL" "$MAIN" "$FIXTURE" "$METADATA_TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$METADATA_TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_30" "pilot card must be landed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-HAKO-DERIVED-MINIMAL-FIXTURE-PILOT-296X-001' "$CARD_30" "pilot card must identify blocker"
guard_expect_fixed_in_file "$TAG" '--provider-package-hako-derived-build-fixture' "$CARD_30" "pilot card must document CLI flag"
guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-hako-derived-build-v0' "$CARD_30" "pilot card must define output contract"
guard_expect_fixed_in_file "$TAG" 'package_mode=hako-derived-provider-package' "$CARD_30" "pilot card must define package mode"
guard_expect_fixed_in_file "$TAG" 'build_mode=hako-derived-selected-fixture' "$CARD_30" "pilot card must define build mode"
guard_expect_fixed_in_file "$TAG" 'hako_source_checked=1' "$CARD_30" "pilot card must require hako source check"
guard_expect_fixed_in_file "$TAG" 'hako_mir_json_emitted=1' "$CARD_30" "pilot card must require MIR JSON emission"
guard_expect_fixed_in_file "$TAG" 'hako_semantic_provider_codegen=0' "$CARD_30" "pilot card must keep semantic codegen closed"
guard_expect_fixed_in_file "$TAG" 'shared_library_artifact_generated=1' "$CARD_30" "pilot card must generate package artifact"
guard_expect_fixed_in_file "$TAG" 'shared_library_load_executed=0' "$CARD_30" "pilot card must keep package command no-load"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$CARD_30" "pilot card must keep provider inactive"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$CARD_30" "pilot card must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$CARD_30" "pilot card must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$CARD_30" "pilot card must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_30" "pilot card must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-HAKO-DERIVED-CLOSEOUT-296X-001' "$CARD_30" "pilot card must select closeout"

guard_expect_fixed_in_file "$TAG" 'Arg::new("provider-package-hako-derived-build-fixture")' "$CLI_ARGS" "CLI args must expose hako-derived flag"
guard_expect_fixed_in_file "$TAG" 'provider_package_hako_derived_build_fixture: Option<String>' "$CLI_MOD" "CLI config must carry hako-derived source path"
guard_expect_fixed_in_file "$TAG" 'maybe_run_provider_package_hako_derived_build' "$MAIN" "main must execute hako-derived package entry"
guard_expect_fixed_in_file "$TAG" 'OUTPUT_CONTRACT: &str = "hakorune-provider-package-hako-derived-build-v0"' "$CLI_IMPL" "CLI impl must own output contract"
guard_expect_fixed_in_file "$TAG" 'emit_mir_json(&hako_source, &mir_json_path)' "$CLI_IMPL" "CLI impl must emit MIR JSON from hako source"
guard_expect_fixed_in_file "$TAG" 'hako_source_hash' "$CLI_IMPL" "CLI impl must record source hash"
guard_expect_fixed_in_file "$TAG" 'hako_mir_json_hash' "$CLI_IMPL" "CLI impl must record MIR JSON hash"
guard_expect_fixed_in_file "$TAG" 'hako_semantic_provider_codegen' "$CLI_IMPL" "CLI impl must report semantic codegen stop line"
guard_expect_fixed_in_file "$TAG" 'shared_library_load_executed=0' "$CLI_IMPL" "CLI impl must not load library"
guard_expect_fixed_in_file "$TAG" 'provider_call_executed=0' "$CLI_IMPL" "CLI impl must not call provider"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$CLI_IMPL" "CLI impl must keep replacement closed"

guard_expect_fixed_in_file "$TAG" 'provider_fixture=allocator' "$FIXTURE" "fixture must identify provider fixture"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$FIXTURE" "fixture must be a simple MIR-emittable source"

guard_expect_fixed_in_file "$TAG" '## Phase C0 .hako-Derived Fixture Build' "$DOC" "reference docs must document Phase C0"
guard_expect_fixed_in_file "$TAG" '--provider-package-hako-derived-build-fixture' "$DOC" "reference docs must document hako-derived flag"
guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-hako-derived-build-v0' "$DOC" "reference docs must document hako-derived contract"
guard_expect_fixed_in_file "$TAG" 'hako_semantic_provider_codegen=0' "$DOC" "reference docs must document semantic codegen stop line"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-30-MIMALLOC-PROVIDER-PACKAGE-HAKO-DERIVED-MINIMAL-FIXTURE-PILOT"' "$CURRENT_STATE" "current state latest card must advance"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "MIMALLOC-PROVIDER-PACKAGE-HAKO-DERIVED-CLOSEOUT-296X-001"' "$CURRENT_STATE" "current state must select closeout row"

guard_expect_fixed_in_file "$TAG" '| 30 | `MIMALLOC-PROVIDER-PACKAGE-HAKO-DERIVED-MINIMAL-FIXTURE-PILOT-296X-001` | Landed |' "$TASKBOARD" "taskboard row 30 must be landed"
guard_expect_fixed_in_file "$TAG" '| 31 | `MIMALLOC-PROVIDER-PACKAGE-HAKO-DERIVED-CLOSEOUT-296X-001` | Current |' "$TASKBOARD" "taskboard row 31 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list hako-derived pilot guard"

cargo build -q --bin hakorune

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_hako_derived_pkg.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
pkg="$tmp_dir/pkg"
build_out="$tmp_dir/build.out"
metadata_out="$tmp_dir/metadata.out"

target/debug/hakorune \
  --provider-package-hako-derived-build-fixture "$FIXTURE" \
  --provider-package-out-dir "$pkg" \
  --provider-package-artifact-name libhakorune_provider.so \
  --provider-package-id org.hakorune.provider.hako.fixture \
  --provider-package-name hako-derived-fixture-provider \
  --provider-package-version 0.1.0 \
  --provider-package-target-triple x86_64-unknown-linux-gnu \
  --provider-package-platform linux \
  --provider-package-force \
  > "$build_out"

guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-hako-derived-build-v0' "$build_out" "CLI must emit hako-derived output contract"
guard_expect_fixed_in_file "$TAG" 'package_mode=hako-derived-provider-package' "$build_out" "CLI must emit hako-derived package mode"
guard_expect_fixed_in_file "$TAG" 'build_mode=hako-derived-selected-fixture' "$build_out" "CLI must emit hako-derived build mode"
guard_expect_fixed_in_file "$TAG" 'build_command_executed=1' "$build_out" "CLI must execute build command"
guard_expect_fixed_in_file "$TAG" 'hako_source_checked=1' "$build_out" "CLI must check source"
guard_expect_fixed_in_file "$TAG" 'hako_mir_json_emitted=1' "$build_out" "CLI must emit MIR JSON"
guard_expect_fixed_in_file "$TAG" 'hako_semantic_provider_codegen=0' "$build_out" "CLI must keep semantic codegen closed"
guard_expect_fixed_in_file "$TAG" 'shared_library_artifact_generated=1' "$build_out" "CLI must generate shared library artifact"
guard_expect_fixed_in_file "$TAG" 'shared_library_load_executed=0' "$build_out" "CLI must not load library"
guard_expect_fixed_in_file "$TAG" 'provider_call_executed=0' "$build_out" "CLI must not call provider"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$build_out" "CLI must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$build_out" "CLI must keep winners closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$build_out" "CLI must end ok"

guard_require_files "$TAG" "$pkg/hakorune_provider.json" "$pkg/hakorune_provider.sha256" "$pkg/libhakorune_provider.so" "$pkg/.hakorune_provider_build/hako_derived_fixture.mir.json"
python3 "$METADATA_TOOL" --manifest "$pkg/hakorune_provider.json" --out "$metadata_out"
guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-metadata-preflight-v0' "$metadata_out" "generated manifest must pass metadata preflight"
guard_expect_fixed_in_file "$TAG" 'binary=libhakorune_provider.so' "$metadata_out" "preflight must consume hako-derived artifact"
guard_expect_fixed_in_file "$TAG" 'shared_library_load_executed=0' "$metadata_out" "preflight must not load library"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$metadata_out" "preflight must keep provider inactive"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$metadata_out" "preflight must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$metadata_out" "preflight must keep winners closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$metadata_out" "preflight must end ok"

cat "$build_out"
cat "$metadata_out"
echo "[$TAG] ok"
