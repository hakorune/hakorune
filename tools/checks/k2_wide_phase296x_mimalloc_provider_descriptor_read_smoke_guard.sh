#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-mimalloc-provider-descriptor-read-smoke"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

ABI_SSOT="docs/development/current/main/design/provider-abi-v1-ssot.md"
RUNTIME_SSOT="docs/development/current/main/design/provider-runtime-load-ssot.md"
ROADMAP="docs/development/current/main/design/mimalloc-benchmark-dll-roadmap-ssot.md"
CARD_10="docs/development/current/main/phases/phase-296x/296x-10-MIMALLOC-PROVIDER-SHARED-LIBRARY-LOAD-ONLY-SMOKE.md"
CARD_11="docs/development/current/main/phases/phase-296x/296x-11-MIMALLOC-PROVIDER-DESCRIPTOR-READ-SMOKE.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
METADATA_TOOL="tools/allocator/provider_package_metadata_preflight.py"
LOAD_TOOL="tools/allocator/provider_package_load_only_smoke.py"
DESCRIPTOR_TOOL="tools/allocator/provider_package_descriptor_smoke.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_provider_descriptor_read_smoke_guard.sh"

echo "[$TAG] checking phase-296x provider descriptor-read smoke"

guard_require_files "$TAG" "$ABI_SSOT" "$RUNTIME_SSOT" "$ROADMAP" "$CARD_10" "$CARD_11" "$TASKBOARD" "$INDEX" "$CURRENT_STATE" "$METADATA_TOOL" "$LOAD_TOOL" "$DESCRIPTOR_TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$METADATA_TOOL" "$LOAD_TOOL" "$DESCRIPTOR_TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'hakorune_provider_descriptor_v1' "$ABI_SSOT" "ABI SSOT must define descriptor export"
guard_expect_fixed_in_file "$TAG" 'magic=0x484B5250' "$ABI_SSOT" "ABI SSOT must define descriptor magic"
guard_expect_fixed_in_file "$TAG" 'descriptor-read-smoke' "$RUNTIME_SSOT" "runtime SSOT must define descriptor-read stage"
guard_expect_fixed_in_file "$TAG" 'required_export_resolved=1' "$RUNTIME_SSOT" "runtime SSOT must allow descriptor export resolution"
guard_expect_fixed_in_file "$TAG" 'descriptor_read_executed=1' "$RUNTIME_SSOT" "runtime SSOT must allow descriptor read"
guard_expect_fixed_in_file "$TAG" 'provider_call_executed=0' "$RUNTIME_SSOT" "runtime SSOT must keep provider calls closed"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-11-MIMALLOC-PROVIDER-DESCRIPTOR-READ-SMOKE"' "$CURRENT_STATE" "current state latest card must advance to descriptor-read smoke"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "MIMALLOC-PROVIDER-API-BIND-SMOKE-296X-001"' "$CURRENT_STATE" "current state must expose API bind blocker"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_10" "load-only smoke must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_11" "descriptor-read card must be landed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-DESCRIPTOR-READ-SMOKE-296X-001' "$CARD_11" "descriptor-read card must identify blocker"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-API-BIND-SMOKE-296X-001' "$CARD_11" "descriptor-read card must select API bind next"
guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-descriptor-smoke-v0' "$CARD_11" "descriptor-read card must define output contract"
guard_expect_fixed_in_file "$TAG" 'shared_library_load_executed=1' "$CARD_11" "descriptor-read card must load shared library"
guard_expect_fixed_in_file "$TAG" 'required_export_resolved=1' "$CARD_11" "descriptor-read card must resolve export"
guard_expect_fixed_in_file "$TAG" 'descriptor_read_executed=1' "$CARD_11" "descriptor-read card must read descriptor"
guard_expect_fixed_in_file "$TAG" 'provider_call_executed=0' "$CARD_11" "descriptor-read card must not call provider API"
guard_expect_fixed_in_file "$TAG" 'allocator_entrypoint_called=0' "$CARD_11" "descriptor-read card must not call allocator entrypoints"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$CARD_11" "descriptor-read card must keep provider inactive"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$CARD_11" "descriptor-read card must keep replacement inactive"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_11" "descriptor-read card must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'descriptor-read smoke with descriptor export only' "$ROADMAP" "roadmap must keep descriptor-read separate"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-API-BIND-SMOKE-296X-001' "$TASKBOARD" "taskboard must expose API bind row"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$DESCRIPTOR_TOOL" "$INDEX" "check index must list descriptor tool"

python3 -m py_compile "$METADATA_TOOL" "$LOAD_TOOL" "$DESCRIPTOR_TOOL"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_provider_descriptor.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
c_src="$tmp_dir/provider.c"
so="$tmp_dir/libhakorune_provider.so"
manifest="$tmp_dir/hakorune_provider.json"
descriptor_out="$tmp_dir/descriptor.out"

cat > "$c_src" <<'C'
#include <stdint.h>

typedef struct HakoProviderDescriptorV1 {
    uint32_t magic;
    uint16_t abi_major;
    uint16_t abi_minor;
    uint32_t descriptor_size;
    const char* provider_id;
    const char* provider_kind;
    const char* provider_version;
    uint64_t capability_bits;
    uint64_t safety_flags;
    const char* contract_hash;
    const char* function_table_hash;
    uint32_t api_table_size;
    uint32_t reserved;
} HakoProviderDescriptorV1;

static const HakoProviderDescriptorV1 DESCRIPTOR = {
    0x484B5250u,
    1,
    0,
    sizeof(HakoProviderDescriptorV1),
    "org.hakorune.provider.descriptor.fixture",
    "allocator",
    "0.1.0",
    3u,
    1u,
    "abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789",
    "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
    0,
    0
};

__attribute__((visibility("default")))
const HakoProviderDescriptorV1* hakorune_provider_descriptor_v1(void) {
    return &DESCRIPTOR;
}
C

cc -shared -fPIC -o "$so" "$c_src"

sha="$(python3 - "$so" <<'PY'
import hashlib
import pathlib
import sys

path = pathlib.Path(sys.argv[1])
print(hashlib.sha256(path.read_bytes()).hexdigest())
PY
)"
size="$(wc -c < "$so" | tr -d ' ')"

cat > "$manifest" <<JSON
{
  "schema_version": "hakorune-provider-package-v1",
  "package_id": "org.hakorune.provider.descriptor.fixture",
  "provider_kind": "allocator",
  "provider_name": "descriptor-fixture",
  "provider_version": "0.1.0",
  "abi_version": "hakorune-provider-abi-v1",
  "target_triple": "x86_64-unknown-linux-gnu",
  "platform": "linux",
  "artifact": {
    "path": "libhakorune_provider.so",
    "sha256": "$sha",
    "size_bytes": $size
  },
  "contract_hash": "abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789",
  "required_exports": [
    "hakorune_provider_descriptor_v1"
  ],
  "capabilities": [
    "descriptor",
    "explicit_allocator_api"
  ],
  "activation": {
    "provider_call_allowed": false,
    "allocator_replacement_allowed": false,
    "hook_allowed": false,
    "global_allocator_allowed": false
  }
}
JSON

python3 "$DESCRIPTOR_TOOL" --manifest "$manifest" --out "$descriptor_out"

guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-descriptor-smoke-v0' "$descriptor_out" "descriptor tool must emit descriptor contract"
guard_expect_fixed_in_file "$TAG" 'dll_mode=descriptor-smoke' "$descriptor_out" "descriptor tool must emit descriptor mode"
guard_expect_fixed_in_file "$TAG" 'provider_name=descriptor-fixture' "$descriptor_out" "descriptor tool must preserve provider name"
guard_expect_fixed_in_file "$TAG" 'required_export=hakorune_provider_descriptor_v1' "$descriptor_out" "descriptor tool must resolve descriptor export"
guard_expect_fixed_in_file "$TAG" 'descriptor_provider_id=org.hakorune.provider.descriptor.fixture' "$descriptor_out" "descriptor tool must read provider id"
guard_expect_fixed_in_file "$TAG" 'descriptor_provider_kind=allocator' "$descriptor_out" "descriptor tool must read provider kind"
guard_expect_fixed_in_file "$TAG" 'descriptor_abi_major=1' "$descriptor_out" "descriptor tool must validate ABI major"
guard_expect_fixed_in_file "$TAG" 'descriptor_contract_hash=abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789' "$descriptor_out" "descriptor tool must validate contract hash"
guard_expect_fixed_in_file "$TAG" 'manifest_ready=1' "$descriptor_out" "descriptor tool must mark manifest ready"
guard_expect_fixed_in_file "$TAG" 'descriptor_ready=1' "$descriptor_out" "descriptor tool must mark descriptor ready"
guard_expect_fixed_in_file "$TAG" 'binary_hash_ready=1' "$descriptor_out" "descriptor tool must verify binary hash"
guard_expect_fixed_in_file "$TAG" 'shared_library_load_executed=1' "$descriptor_out" "descriptor tool must load shared library"
guard_expect_fixed_in_file "$TAG" 'required_export_resolved=1' "$descriptor_out" "descriptor tool must resolve export"
guard_expect_fixed_in_file "$TAG" 'descriptor_read_executed=1' "$descriptor_out" "descriptor tool must call descriptor"
guard_expect_fixed_in_file "$TAG" 'provider_call_executed=0' "$descriptor_out" "descriptor tool must not call provider API"
guard_expect_fixed_in_file "$TAG" 'allocator_entrypoint_called=0' "$descriptor_out" "descriptor tool must not call allocator entrypoints"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$descriptor_out" "descriptor tool must keep provider inactive"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$descriptor_out" "descriptor tool must keep replacement inactive"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$descriptor_out" "descriptor tool must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$descriptor_out" "descriptor tool must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$descriptor_out" "descriptor tool must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$descriptor_out" "descriptor tool must end with summary"

cat "$descriptor_out"
echo "[$TAG] ok"
