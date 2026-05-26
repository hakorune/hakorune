#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-mimalloc-provider-package-cli-package-pilot"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_21="docs/development/current/main/phases/phase-296x/296x-21-MIMALLOC-PROVIDER-PACKAGE-EXISTING-BINARY-MANIFEST-CLOSEOUT.md"
CARD_22="docs/development/current/main/phases/phase-296x/296x-22-MIMALLOC-PROVIDER-PACKAGE-CLI-PACKAGE-PILOT.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
CLI_ARGS="src/cli/args.rs"
CLI_MOD="src/cli/mod.rs"
CLI_IMPL="src/cli/provider_package_existing_binary.rs"
MAIN="src/main.rs"
PREFLIGHT_TOOL="tools/allocator/provider_package_metadata_preflight.py"
PREV_GUARD="tools/checks/k2_wide_phase296x_mimalloc_provider_package_existing_binary_manifest_closeout_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_provider_package_cli_package_pilot_guard.sh"

echo "[$TAG] checking phase-296x provider package CLI package pilot"

guard_require_files "$TAG" "$CARD_21" "$CARD_22" "$TASKBOARD" "$INDEX" "$CLI_ARGS" "$CLI_MOD" "$CLI_IMPL" "$MAIN" "$PREFLIGHT_TOOL" "$PREV_GUARD" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$PREFLIGHT_TOOL" "$PREV_GUARD" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_21" "CLI selection closeout must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_22" "CLI package card must be landed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-CLI-PACKAGE-PILOT-296X-001' "$CARD_22" "CLI package card must identify blocker"
guard_expect_fixed_in_file "$TAG" '--provider-package-existing-binary' "$CARD_22" "card must document CLI entry"
guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-existing-binary-manifest-v0' "$CARD_22" "card must define output contract"
guard_expect_fixed_in_file "$TAG" 'shared_library_load_executed=0' "$CARD_22" "card must keep loading closed"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$CARD_22" "card must keep provider inactive"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$CARD_22" "card must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$CARD_22" "card must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$CARD_22" "card must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_22" "card must keep winners closed"

guard_expect_fixed_in_file "$TAG" 'Arg::new("provider-package-existing-binary")' "$CLI_ARGS" "CLI args must expose package binary flag"
guard_expect_fixed_in_file "$TAG" 'provider_package_existing_binary: Option<String>' "$CLI_MOD" "CLI config must carry package binary"
guard_expect_fixed_in_file "$TAG" 'maybe_run_provider_package_existing_binary' "$MAIN" "main must execute package CLI entry"
guard_expect_fixed_in_file "$TAG" 'OUTPUT_CONTRACT' "$CLI_IMPL" "CLI impl must own output contract"
guard_expect_fixed_in_file "$TAG" 'shared_library_load_executed=0' "$CLI_IMPL" "CLI impl must keep loading closed"
guard_expect_fixed_in_file "$TAG" 'provider_call_executed=0' "$CLI_IMPL" "CLI impl must not call provider"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$CLI_IMPL" "CLI impl must keep replacement closed"

guard_expect_fixed_in_file "$TAG" '| 22 | `MIMALLOC-PROVIDER-PACKAGE-CLI-PACKAGE-PILOT-296X-001` | Landed |' "$TASKBOARD" "taskboard row 22 must be landed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-CLI-PACKAGE-CLOSEOUT-296X-001' "$TASKBOARD" "taskboard must expose CLI package closeout"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list CLI package guard"

cargo build -q --bin nyash

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_provider_package_cli.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
c_src="$tmp_dir/provider.c"
so="$tmp_dir/libsource_provider.so"
pkg="$tmp_dir/pkg"
report="$tmp_dir/package.out"
preflight="$tmp_dir/preflight.out"

cat > "$c_src" <<'C'
#include <stdint.h>
typedef struct HakoProviderDescriptorV1 { uint32_t magic; uint16_t abi_major; uint16_t abi_minor; uint32_t descriptor_size; const char* provider_id; const char* provider_kind; const char* provider_version; uint64_t capability_bits; uint64_t safety_flags; const char* contract_hash; const char* function_table_hash; uint32_t api_table_size; uint32_t reserved; } HakoProviderDescriptorV1;
static const HakoProviderDescriptorV1 DESCRIPTOR = {0x484B5250u,1,0,sizeof(HakoProviderDescriptorV1),"org.hakorune.provider.cli.fixture","allocator","0.1.0",1u,1u,"abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789","0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",0,0};
__attribute__((visibility("default"))) const HakoProviderDescriptorV1* hakorune_provider_descriptor_v1(void) { return &DESCRIPTOR; }
C
cc -shared -fPIC -o "$so" "$c_src"

HAKO_ALLOW_NYASH=1 target/debug/nyash \
  --provider-package-existing-binary "$so" \
  --provider-package-out-dir "$pkg" \
  --provider-package-artifact-name libhakorune_provider.so \
  --provider-package-id org.hakorune.provider.cli.fixture \
  --provider-package-name cli-package-fixture \
  --provider-package-version 0.1.0 \
  --provider-package-target-triple x86_64-unknown-linux-gnu \
  --provider-package-platform linux \
  --provider-package-force \
  > "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-existing-binary-manifest-v0' "$report" "CLI must emit package contract"
guard_expect_fixed_in_file "$TAG" 'package_mode=existing-binary-manifest' "$report" "CLI must emit package mode"
guard_expect_fixed_in_file "$TAG" 'provider_name=cli-package-fixture' "$report" "CLI must preserve provider name"
guard_expect_fixed_in_file "$TAG" 'artifact_path=libhakorune_provider.so' "$report" "CLI must emit artifact path"
guard_expect_fixed_in_file "$TAG" 'required_exports=hakorune_provider_descriptor_v1' "$report" "CLI must emit descriptor export"
guard_expect_fixed_in_file "$TAG" 'provider_call_allowed=0' "$report" "CLI must keep provider calls disabled by default"
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
guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-metadata-preflight-v0' "$preflight" "CLI generated manifest must pass metadata preflight"
guard_expect_fixed_in_file "$TAG" 'binary=libhakorune_provider.so' "$preflight" "preflight must consume CLI packaged artifact"
guard_expect_fixed_in_file "$TAG" 'shared_library_load_executed=0' "$preflight" "preflight must not load library"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$preflight" "preflight must keep provider inactive"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$preflight" "preflight must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$preflight" "preflight must keep winners closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$preflight" "preflight must end ok"

cat "$report"
cat "$preflight"
echo "[$TAG] ok"
