#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-mimalloc-provider-alloc-free-smoke"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

RUNTIME_SSOT="docs/development/current/main/design/provider-runtime-load-ssot.md"
CARD_14="docs/development/current/main/phases/phase-296x/296x-14-MIMALLOC-PROVIDER-ALLOC-FREE-SMOKE.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
TOOL="tools/allocator/provider_package_alloc_free_smoke.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_provider_alloc_free_smoke_guard.sh"

echo "[$TAG] checking phase-296x provider alloc/free smoke"

guard_require_files "$TAG" "$RUNTIME_SSOT" "$CARD_14" "$TASKBOARD" "$INDEX" "$CURRENT_STATE" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'dll_mode=provider-alloc-free' "$RUNTIME_SSOT" "runtime SSOT must define alloc/free mode"
guard_expect_fixed_in_file "$TAG" 'provider_alloc_executed=1' "$RUNTIME_SSOT" "runtime SSOT must allow alloc"
guard_expect_fixed_in_file "$TAG" 'provider_free_executed=1' "$RUNTIME_SSOT" "runtime SSOT must allow free"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$RUNTIME_SSOT" "runtime SSOT must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_14" "alloc/free card must be landed"
guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-alloc-free-smoke-v0' "$CARD_14" "card must define contract"
guard_expect_fixed_in_file "$TAG" 'allocator_entrypoint_called=1' "$CARD_14" "card must call allocator entrypoint"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$CARD_14" "card must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_14" "card must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-EXPLICIT-REPEATED-MEASUREMENT-296X-001' "$TASKBOARD" "taskboard must expose repeated measurement row"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "index must list guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "index must list tool"

python3 -m py_compile "$TOOL"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_provider_alloc_free.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
c_src="$tmp_dir/provider.c"
so="$tmp_dir/libhakorune_provider.so"
manifest="$tmp_dir/hakorune_provider.json"
out="$tmp_dir/alloc_free.out"

cat > "$c_src" <<'C'
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
typedef struct HakoProviderDescriptorV1 { uint32_t magic; uint16_t abi_major; uint16_t abi_minor; uint32_t descriptor_size; const char* provider_id; const char* provider_kind; const char* provider_version; uint64_t capability_bits; uint64_t safety_flags; const char* contract_hash; const char* function_table_hash; uint32_t api_table_size; uint32_t reserved; } HakoProviderDescriptorV1;
typedef struct HakoProviderApiV1 { uint32_t magic; uint16_t abi_major; uint16_t abi_minor; uint32_t api_table_size; int (*ping)(void); void* (*alloc)(size_t,size_t); void (*free)(void*); int (*owns)(void*); } HakoProviderApiV1;
static int provider_ping(void) { return 7; }
static void* provider_alloc(size_t size, size_t align) { (void)align; return malloc(size); }
static void provider_free(void* ptr) { free(ptr); }
static int provider_owns(void* ptr) { return ptr != NULL; }
static const HakoProviderDescriptorV1 DESCRIPTOR = {0x484B5250u,1,0,sizeof(HakoProviderDescriptorV1),"org.hakorune.provider.allocfree.fixture","allocator","0.1.0",3u,1u,"abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789","0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",sizeof(HakoProviderApiV1),0};
static const HakoProviderApiV1 API = {0x484B5241u,1,0,sizeof(HakoProviderApiV1),provider_ping,provider_alloc,provider_free,provider_owns};
__attribute__((visibility("default"))) const HakoProviderDescriptorV1* hakorune_provider_descriptor_v1(void) { return &DESCRIPTOR; }
__attribute__((visibility("default"))) const HakoProviderApiV1* hakorune_provider_get_api_v1(void) { return &API; }
C
cc -shared -fPIC -o "$so" "$c_src"
sha="$(python3 - "$so" <<'PY'
import hashlib, pathlib, sys
print(hashlib.sha256(pathlib.Path(sys.argv[1]).read_bytes()).hexdigest())
PY
)"
size="$(wc -c < "$so" | tr -d ' ')"
cat > "$manifest" <<JSON
{"schema_version":"hakorune-provider-package-v1","package_id":"org.hakorune.provider.allocfree.fixture","provider_kind":"allocator","provider_name":"allocfree-fixture","provider_version":"0.1.0","abi_version":"hakorune-provider-abi-v1","target_triple":"x86_64-unknown-linux-gnu","platform":"linux","artifact":{"path":"libhakorune_provider.so","sha256":"$sha","size_bytes":$size},"contract_hash":"abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789","required_exports":["hakorune_provider_descriptor_v1","hakorune_provider_get_api_v1"],"capabilities":["descriptor","explicit_allocator_api"],"activation":{"provider_call_allowed":true,"allocator_replacement_allowed":false,"hook_allowed":false,"global_allocator_allowed":false}}
JSON

python3 "$TOOL" --manifest "$manifest" --out "$out" --size 64 --align 8
guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-alloc-free-smoke-v0' "$out" "tool must emit contract"
guard_expect_fixed_in_file "$TAG" 'dll_mode=provider-alloc-free' "$out" "tool must emit mode"
guard_expect_fixed_in_file "$TAG" 'provider_call_executed=1' "$out" "tool must call provider"
guard_expect_fixed_in_file "$TAG" 'allocator_entrypoint_called=1' "$out" "tool must call allocator"
guard_expect_fixed_in_file "$TAG" 'provider_alloc_executed=1' "$out" "tool must call alloc"
guard_expect_fixed_in_file "$TAG" 'provider_free_executed=1' "$out" "tool must call free"
guard_expect_fixed_in_file "$TAG" 'allocation_count=1' "$out" "tool must count allocation"
guard_expect_fixed_in_file "$TAG" 'free_count=1' "$out" "tool must count free"
guard_expect_fixed_in_file "$TAG" 'requested_bytes=64' "$out" "tool must preserve requested bytes"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$out" "tool must keep provider inactive"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$out" "tool must keep replacement inactive"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$out" "tool must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$out" "tool must end ok"
cat "$out"
echo "[$TAG] ok"
