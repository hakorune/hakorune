#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-mimalloc-provider-api-bind-smoke"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

ABI_SSOT="docs/development/current/main/design/provider-abi-v1-ssot.md"
RUNTIME_SSOT="docs/development/current/main/design/provider-runtime-load-ssot.md"
CARD_11="docs/development/current/main/phases/phase-296x/296x-11-MIMALLOC-PROVIDER-DESCRIPTOR-READ-SMOKE.md"
CARD_12="docs/development/current/main/phases/phase-296x/296x-12-MIMALLOC-PROVIDER-API-BIND-SMOKE.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
TOOL="tools/allocator/provider_package_api_bind_smoke.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_provider_api_bind_smoke_guard.sh"

echo "[$TAG] checking phase-296x provider API bind smoke"

guard_require_files "$TAG" "$ABI_SSOT" "$RUNTIME_SSOT" "$CARD_11" "$CARD_12" "$TASKBOARD" "$INDEX" "$CURRENT_STATE" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'hakorune_provider_get_api_v1' "$ABI_SSOT" "ABI SSOT must define API bind export"
guard_expect_fixed_in_file "$TAG" 'api_magic=0x484B5241' "$ABI_SSOT" "ABI SSOT must define API magic"
guard_expect_fixed_in_file "$TAG" 'dll_mode=provider-api-bind' "$RUNTIME_SSOT" "runtime SSOT must define API bind mode"
guard_expect_fixed_in_file "$TAG" 'provider_api_bound=1' "$RUNTIME_SSOT" "runtime SSOT must mark API bound"
guard_expect_fixed_in_file "$TAG" 'provider_call_executed=0' "$RUNTIME_SSOT" "runtime SSOT must keep provider calls closed"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_11" "descriptor-read smoke must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_12" "API bind card must be landed"
guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-api-bind-smoke-v0' "$CARD_12" "card must define contract"
guard_expect_fixed_in_file "$TAG" 'provider_api_bound=1' "$CARD_12" "card must bind API"
guard_expect_fixed_in_file "$TAG" 'provider_call_executed=0' "$CARD_12" "card must not call provider functions"
guard_expect_fixed_in_file "$TAG" 'allocator_entrypoint_called=0' "$CARD_12" "card must not call allocator entrypoints"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-NOOP-CALL-SMOKE-296X-001' "$CARD_12" "card must select noop call next"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-NOOP-CALL-SMOKE-296X-001' "$TASKBOARD" "taskboard must expose noop call row"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "index must list guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "index must list tool"

python3 -m py_compile "$TOOL"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_provider_api_bind.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
c_src="$tmp_dir/provider.c"
so="$tmp_dir/libhakorune_provider.so"
manifest="$tmp_dir/hakorune_provider.json"
out="$tmp_dir/api.out"

cat > "$c_src" <<'C'
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct HakoProviderDescriptorV1 {
    uint32_t magic; uint16_t abi_major; uint16_t abi_minor; uint32_t descriptor_size;
    const char* provider_id; const char* provider_kind; const char* provider_version;
    uint64_t capability_bits; uint64_t safety_flags; const char* contract_hash;
    const char* function_table_hash; uint32_t api_table_size; uint32_t reserved;
} HakoProviderDescriptorV1;

typedef struct HakoProviderApiV1 {
    uint32_t magic; uint16_t abi_major; uint16_t abi_minor; uint32_t api_table_size;
    int (*ping)(void);
    void* (*alloc)(size_t size, size_t align);
    void (*free)(void* ptr);
    int (*owns)(void* ptr);
} HakoProviderApiV1;

static int provider_ping(void) { return 7; }
static void* provider_alloc(size_t size, size_t align) { (void)align; return malloc(size); }
static void provider_free(void* ptr) { free(ptr); }
static int provider_owns(void* ptr) { return ptr != NULL; }

static const HakoProviderDescriptorV1 DESCRIPTOR = {
    0x484B5250u, 1, 0, sizeof(HakoProviderDescriptorV1),
    "org.hakorune.provider.api-bind.fixture", "allocator", "0.1.0",
    3u, 1u,
    "abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789",
    "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
    sizeof(HakoProviderApiV1), 0
};
static const HakoProviderApiV1 API = {
    0x484B5241u, 1, 0, sizeof(HakoProviderApiV1),
    provider_ping, provider_alloc, provider_free, provider_owns
};

__attribute__((visibility("default")))
const HakoProviderDescriptorV1* hakorune_provider_descriptor_v1(void) { return &DESCRIPTOR; }

__attribute__((visibility("default")))
const HakoProviderApiV1* hakorune_provider_get_api_v1(void) { return &API; }
C

cc -shared -fPIC -o "$so" "$c_src"
sha="$(python3 - "$so" <<'PY'
import hashlib, pathlib, sys
print(hashlib.sha256(pathlib.Path(sys.argv[1]).read_bytes()).hexdigest())
PY
)"
size="$(wc -c < "$so" | tr -d ' ')"

cat > "$manifest" <<JSON
{
  "schema_version": "hakorune-provider-package-v1",
  "package_id": "org.hakorune.provider.api-bind.fixture",
  "provider_kind": "allocator",
  "provider_name": "api-bind-fixture",
  "provider_version": "0.1.0",
  "abi_version": "hakorune-provider-abi-v1",
  "target_triple": "x86_64-unknown-linux-gnu",
  "platform": "linux",
  "artifact": {"path": "libhakorune_provider.so", "sha256": "$sha", "size_bytes": $size},
  "contract_hash": "abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789",
  "required_exports": ["hakorune_provider_descriptor_v1", "hakorune_provider_get_api_v1"],
  "capabilities": ["descriptor", "explicit_allocator_api"],
  "activation": {
    "provider_call_allowed": false,
    "allocator_replacement_allowed": false,
    "hook_allowed": false,
    "global_allocator_allowed": false
  }
}
JSON

python3 "$TOOL" --manifest "$manifest" --out "$out"

guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-api-bind-smoke-v0' "$out" "tool must emit contract"
guard_expect_fixed_in_file "$TAG" 'dll_mode=provider-api-bind' "$out" "tool must emit mode"
guard_expect_fixed_in_file "$TAG" 'provider_name=api-bind-fixture' "$out" "tool must preserve provider name"
guard_expect_fixed_in_file "$TAG" 'provider_api_export=hakorune_provider_get_api_v1' "$out" "tool must resolve API export"
guard_expect_fixed_in_file "$TAG" 'provider_api_bound=1' "$out" "tool must bind API"
guard_expect_fixed_in_file "$TAG" 'provider_call_executed=0' "$out" "tool must not call provider function pointers"
guard_expect_fixed_in_file "$TAG" 'allocator_entrypoint_called=0' "$out" "tool must not call allocator entrypoints"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$out" "tool must keep provider inactive"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$out" "tool must keep replacement inactive"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$out" "tool must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$out" "tool must end ok"

cat "$out"
echo "[$TAG] ok"
