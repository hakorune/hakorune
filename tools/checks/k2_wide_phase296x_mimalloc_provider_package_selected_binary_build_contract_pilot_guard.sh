#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-mimalloc-provider-package-selected-binary-build-contract-pilot"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_27="docs/development/current/main/phases/phase-296x/296x-27-MIMALLOC-PROVIDER-PACKAGE-SELECTED-BINARY-BUILD-CONTRACT-PILOT.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
DOC="docs/reference/runtime/provider-package-v0.md"
INDEX="docs/tools/check-scripts-index.md"
CLI_ARGS="src/cli/args.rs"
CLI_MOD="src/cli/mod.rs"
CLI_IMPL="src/cli/provider_package_selected_binary_build.rs"
MAIN="src/main.rs"
PREFLIGHT_TOOL="tools/allocator/provider_package_metadata_preflight.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_provider_package_selected_binary_build_contract_pilot_guard.sh"

echo "[$TAG] checking phase-296x selected provider binary build/package pilot"

guard_require_files "$TAG" "$CARD_27" "$TASKBOARD" "$CURRENT_STATE" "$DOC" "$INDEX" "$CLI_ARGS" "$CLI_MOD" "$CLI_IMPL" "$MAIN" "$PREFLIGHT_TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$PREFLIGHT_TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-27-MIMALLOC-PROVIDER-PACKAGE-SELECTED-BINARY-BUILD-CONTRACT-PILOT"' "$CURRENT_STATE" "current state latest card must advance"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "MIMALLOC-PROVIDER-PACKAGE-SELECTED-BINARY-BUILD-CLOSEOUT-296X-001"' "$CURRENT_STATE" "current state must select closeout row"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_27" "pilot card must be landed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-SELECTED-BINARY-BUILD-CONTRACT-PILOT-296X-001' "$CARD_27" "pilot card must identify blocker"
guard_expect_fixed_in_file "$TAG" '--provider-package-selected-binary-build-fixture' "$CARD_27" "pilot card must document CLI flag"
guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-selected-binary-build-v0' "$CARD_27" "pilot card must define output contract"
guard_expect_fixed_in_file "$TAG" 'package_mode=selected-binary-build-package' "$CARD_27" "pilot card must define package mode"
guard_expect_fixed_in_file "$TAG" 'build_mode=selected-fixture' "$CARD_27" "pilot card must define build mode"
guard_expect_fixed_in_file "$TAG" 'hako_shared_library_generation=0' "$CARD_27" "pilot card must keep hako generation closed"
guard_expect_fixed_in_file "$TAG" 'arbitrary_shell_build_executed=0' "$CARD_27" "pilot card must forbid arbitrary shell build"
guard_expect_fixed_in_file "$TAG" 'shared_library_load_executed=0' "$CARD_27" "pilot card must keep package command no-load"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$CARD_27" "pilot card must keep provider inactive"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$CARD_27" "pilot card must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-SELECTED-BINARY-BUILD-CLOSEOUT-296X-001' "$CARD_27" "pilot card must select closeout"

guard_expect_fixed_in_file "$TAG" 'Arg::new("provider-package-selected-binary-build-fixture")' "$CLI_ARGS" "CLI args must expose selected build fixture flag"
guard_expect_fixed_in_file "$TAG" 'provider_package_selected_binary_build_fixture: bool' "$CLI_MOD" "CLI config must carry selected build fixture flag"
guard_expect_fixed_in_file "$TAG" 'maybe_run_provider_package_selected_binary_build' "$MAIN" "main must execute selected build package entry"
guard_expect_fixed_in_file "$TAG" 'OUTPUT_CONTRACT' "$CLI_IMPL" "CLI impl must own output contract"
guard_expect_fixed_in_file "$TAG" 'PACKAGE_MODE' "$CLI_IMPL" "CLI impl must own package mode"
guard_expect_fixed_in_file "$TAG" 'Command::new("cc")' "$CLI_IMPL" "CLI impl must use fixed compiler entry without shell"
guard_expect_fixed_in_file "$TAG" 'arbitrary_shell_build_executed=0' "$CLI_IMPL" "CLI impl must report no shell build"
guard_expect_fixed_in_file "$TAG" 'hako_shared_library_generation=0' "$CLI_IMPL" "CLI impl must keep hako generation closed"
guard_expect_fixed_in_file "$TAG" 'shared_library_load_executed=0' "$CLI_IMPL" "CLI impl must not load library"
guard_expect_fixed_in_file "$TAG" 'provider_call_executed=0' "$CLI_IMPL" "CLI impl must not call provider"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$CLI_IMPL" "CLI impl must keep replacement closed"

guard_expect_fixed_in_file "$TAG" '## Phase B1 Selected Fixture Build' "$DOC" "reference docs must document Phase B1 selected build"
guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-selected-binary-build-v0' "$DOC" "reference docs must document selected build contract"
guard_expect_fixed_in_file "$TAG" 'hako_shared_library_generation=0' "$DOC" "reference docs must keep hako generation closed"

guard_expect_fixed_in_file "$TAG" '| 27 | `MIMALLOC-PROVIDER-PACKAGE-SELECTED-BINARY-BUILD-CONTRACT-PILOT-296X-001` | Landed |' "$TASKBOARD" "taskboard row 27 must be landed"
guard_expect_fixed_in_file "$TAG" '| 28 | `MIMALLOC-PROVIDER-PACKAGE-SELECTED-BINARY-BUILD-CLOSEOUT-296X-001` | Current |' "$TASKBOARD" "taskboard row 28 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list selected build guard"

cargo build -q --bin hakorune

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_selected_build.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
pkg="$tmp_dir/pkg"
report="$tmp_dir/package.out"
preflight="$tmp_dir/preflight.out"

target/debug/hakorune \
  --provider-package-selected-binary-build-fixture \
  --provider-package-out-dir "$pkg" \
  --provider-package-artifact-name libhakorune_provider.so \
  --provider-package-id org.hakorune.provider.selected.fixture \
  --provider-package-name selected-fixture-provider \
  --provider-package-version 0.1.0 \
  --provider-package-target-triple x86_64-unknown-linux-gnu \
  --provider-package-platform linux \
  --provider-package-force \
  > "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-selected-binary-build-v0' "$report" "CLI must emit selected build output contract"
guard_expect_fixed_in_file "$TAG" 'package_mode=selected-binary-build-package' "$report" "CLI must emit selected build package mode"
guard_expect_fixed_in_file "$TAG" 'build_mode=selected-fixture' "$report" "CLI must emit selected fixture build mode"
guard_expect_fixed_in_file "$TAG" 'build_command_executed=1' "$report" "CLI must execute selected build command"
guard_expect_fixed_in_file "$TAG" 'hako_shared_library_generation=0' "$report" "CLI must keep hako generation closed"
guard_expect_fixed_in_file "$TAG" 'arbitrary_shell_build_executed=0' "$report" "CLI must not use arbitrary shell build"
guard_expect_fixed_in_file "$TAG" 'provider_name=selected-fixture-provider' "$report" "CLI must preserve provider name"
guard_expect_fixed_in_file "$TAG" 'artifact_path=libhakorune_provider.so' "$report" "CLI must emit artifact path"
guard_expect_fixed_in_file "$TAG" 'shared_library_load_executed=0' "$report" "CLI must not load library"
guard_expect_fixed_in_file "$TAG" 'required_export_resolved=0' "$report" "CLI must not resolve exports"
guard_expect_fixed_in_file "$TAG" 'descriptor_read_executed=0' "$report" "CLI must not read descriptor"
guard_expect_fixed_in_file "$TAG" 'provider_call_executed=0' "$report" "CLI must not call provider"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$report" "CLI must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$report" "CLI must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$report" "CLI must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "CLI must keep winners closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "CLI must end ok"

guard_require_files "$TAG" "$pkg/hakorune_provider.json" "$pkg/hakorune_provider.sha256" "$pkg/libhakorune_provider.so"
python3 "$PREFLIGHT_TOOL" --manifest "$pkg/hakorune_provider.json" --out "$preflight"
guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-metadata-preflight-v0' "$preflight" "selected build manifest must pass metadata preflight"
guard_expect_fixed_in_file "$TAG" 'binary=libhakorune_provider.so' "$preflight" "preflight must consume selected build artifact"
guard_expect_fixed_in_file "$TAG" 'shared_library_load_executed=0' "$preflight" "preflight must not load library"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$preflight" "preflight must keep provider inactive"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$preflight" "preflight must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$preflight" "preflight must keep winners closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$preflight" "preflight must end ok"

cat "$report"
cat "$preflight"
echo "[$TAG] ok"
