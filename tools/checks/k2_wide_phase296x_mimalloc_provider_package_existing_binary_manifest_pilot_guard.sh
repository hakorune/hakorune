#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-mimalloc-provider-package-existing-binary-manifest-pilot"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_19="docs/development/current/main/phases/phase-296x/296x-19-MIMALLOC-PROVIDER-EXPLICIT-COMPARISON-CLOSEOUT.md"
CARD_20="docs/development/current/main/phases/phase-296x/296x-20-MIMALLOC-PROVIDER-PACKAGE-EXISTING-BINARY-MANIFEST-PILOT.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
ARTIFACT_SSOT="docs/development/current/main/design/provider-package-artifact-ssot.md"
TOOL="tools/allocator/provider_package_existing_binary_manifest.py"
PREFLIGHT_TOOL="tools/allocator/provider_package_metadata_preflight.py"
PREV_GUARD="tools/checks/k2_wide_phase296x_mimalloc_provider_explicit_comparison_closeout_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_provider_package_existing_binary_manifest_pilot_guard.sh"

echo "[$TAG] checking phase-296x provider package existing-binary manifest pilot"

guard_require_files "$TAG" "$CARD_19" "$CARD_20" "$TASKBOARD" "$INDEX" "$ARTIFACT_SSOT" "$TOOL" "$PREFLIGHT_TOOL" "$PREV_GUARD" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$PREFLIGHT_TOOL" "$PREV_GUARD" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_19" "comparison closeout must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_20" "package pilot card must be landed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-EXISTING-BINARY-MANIFEST-PILOT-296X-001' "$CARD_20" "package card must identify blocker"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$CARD_20" "package card must name tool"
guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-existing-binary-manifest-v0' "$CARD_20" "package card must define output contract"
guard_expect_fixed_in_file "$TAG" 'package_mode=existing-binary-manifest' "$CARD_20" "package card must define package mode"
guard_expect_fixed_in_file "$TAG" 'schema_version=hakorune-provider-package-v1' "$CARD_20" "package card must define schema"
guard_expect_fixed_in_file "$TAG" 'shared_library_load_executed=0' "$CARD_20" "package card must keep loading closed"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$CARD_20" "package card must keep provider inactive"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$CARD_20" "package card must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$CARD_20" "package card must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$CARD_20" "package card must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_20" "package card must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-EXISTING-BINARY-MANIFEST-CLOSEOUT-296X-001' "$CARD_20" "package card must select closeout"
guard_expect_fixed_in_file "$TAG" 'Phase A: package existing binary + manifest' "$ARTIFACT_SSOT" "artifact SSOT must define Phase A"

guard_expect_fixed_in_file "$TAG" '| 20 | `MIMALLOC-PROVIDER-PACKAGE-EXISTING-BINARY-MANIFEST-PILOT-296X-001` | Landed |' "$TASKBOARD" "taskboard row 20 must be landed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-EXISTING-BINARY-MANIFEST-CLOSEOUT-296X-001' "$TASKBOARD" "taskboard must expose package closeout row"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list package guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list package tool"

python3 -m py_compile "$TOOL"
guard_expect_fixed_in_file "$TAG" 'OUTPUT_CONTRACT = "hakorune-provider-package-existing-binary-manifest-v0"' "$TOOL" "tool must own output contract as a constant"
guard_expect_fixed_in_file "$TAG" 'f"output_contract={OUTPUT_CONTRACT}"' "$TOOL" "tool report must use output contract constant"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_provider_package.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
c_src="$tmp_dir/provider.c"
so="$tmp_dir/libsource_provider.so"
pkg="$tmp_dir/pkg"
report="$tmp_dir/package.out"
preflight="$tmp_dir/preflight.out"

cat > "$c_src" <<'C'
#include <stdint.h>
typedef struct HakoProviderDescriptorV1 { uint32_t magic; uint16_t abi_major; uint16_t abi_minor; uint32_t descriptor_size; const char* provider_id; const char* provider_kind; const char* provider_version; uint64_t capability_bits; uint64_t safety_flags; const char* contract_hash; const char* function_table_hash; uint32_t api_table_size; uint32_t reserved; } HakoProviderDescriptorV1;
static const HakoProviderDescriptorV1 DESCRIPTOR = {0x484B5250u,1,0,sizeof(HakoProviderDescriptorV1),"org.hakorune.provider.package.fixture","allocator","0.1.0",1u,1u,"abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789","0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",0,0};
__attribute__((visibility("default"))) const HakoProviderDescriptorV1* hakorune_provider_descriptor_v1(void) { return &DESCRIPTOR; }
C
cc -shared -fPIC -o "$so" "$c_src"

python3 "$TOOL" \
  --binary "$so" \
  --out-dir "$pkg" \
  --artifact-name libhakorune_provider.so \
  --package-id org.hakorune.provider.package.fixture \
  --provider-kind allocator \
  --provider-name package-fixture \
  --provider-version 0.1.0 \
  --target-triple x86_64-unknown-linux-gnu \
  --platform linux \
  --profile speed \
  --report-out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-existing-binary-manifest-v0' "$report" "tool must emit package contract"
guard_expect_fixed_in_file "$TAG" 'package_mode=existing-binary-manifest' "$report" "tool must emit package mode"
guard_expect_fixed_in_file "$TAG" 'schema_version=hakorune-provider-package-v1' "$report" "tool must emit schema"
guard_expect_fixed_in_file "$TAG" 'provider_name=package-fixture' "$report" "tool must emit provider name"
guard_expect_fixed_in_file "$TAG" 'artifact_path=libhakorune_provider.so' "$report" "tool must emit artifact name"
guard_expect_fixed_in_file "$TAG" 'required_exports=hakorune_provider_descriptor_v1' "$report" "tool must emit descriptor export"
guard_expect_fixed_in_file "$TAG" 'capabilities=descriptor,explicit_allocator_api' "$report" "tool must emit capabilities"
guard_expect_fixed_in_file "$TAG" 'provider_call_allowed=0' "$report" "tool must keep provider call disabled by default"
guard_expect_fixed_in_file "$TAG" 'shared_library_load_executed=0' "$report" "tool must not load library"
guard_expect_fixed_in_file "$TAG" 'required_export_resolved=0' "$report" "tool must not resolve exports"
guard_expect_fixed_in_file "$TAG" 'descriptor_read_executed=0' "$report" "tool must not read descriptor"
guard_expect_fixed_in_file "$TAG" 'provider_call_executed=0' "$report" "tool must not call provider"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$report" "tool must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$report" "tool must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$report" "tool must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

guard_require_files "$TAG" "$pkg/hakorune_provider.json" "$pkg/hakorune_provider.sha256" "$pkg/libhakorune_provider.so"
python3 "$PREFLIGHT_TOOL" --manifest "$pkg/hakorune_provider.json" --out "$preflight"
guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-metadata-preflight-v0' "$preflight" "generated manifest must pass metadata preflight"
guard_expect_fixed_in_file "$TAG" 'dll_mode=metadata-preflight' "$preflight" "preflight must stay metadata-only"
guard_expect_fixed_in_file "$TAG" 'binary=libhakorune_provider.so' "$preflight" "preflight must consume packaged artifact"
guard_expect_fixed_in_file "$TAG" 'required_export=hakorune_provider_descriptor_v1' "$preflight" "preflight must preserve descriptor export"
guard_expect_fixed_in_file "$TAG" 'shared_library_load_executed=0' "$preflight" "preflight must not load library"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$preflight" "preflight must keep provider inactive"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$preflight" "preflight must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$preflight" "preflight must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$preflight" "preflight must end ok"

cat "$report"
cat "$preflight"
echo "[$TAG] ok"
