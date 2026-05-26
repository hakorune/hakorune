#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-mimalloc-provider-shared-library-load-only-smoke"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

ABI_SSOT="docs/development/current/main/design/provider-abi-v1-ssot.md"
ARTIFACT_SSOT="docs/development/current/main/design/provider-package-artifact-ssot.md"
RUNTIME_SSOT="docs/development/current/main/design/provider-runtime-load-ssot.md"
ROADMAP="docs/development/current/main/design/mimalloc-benchmark-dll-roadmap-ssot.md"
CARD_09="docs/development/current/main/phases/phase-296x/296x-09-MIMALLOC-DLL-LOAD-ONLY-METADATA-PREFLIGHT.md"
CARD_10="docs/development/current/main/phases/phase-296x/296x-10-MIMALLOC-PROVIDER-SHARED-LIBRARY-LOAD-ONLY-SMOKE.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
METADATA_TOOL="tools/allocator/provider_package_metadata_preflight.py"
LOAD_TOOL="tools/allocator/provider_package_load_only_smoke.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_provider_shared_library_load_only_smoke_guard.sh"

echo "[$TAG] checking phase-296x provider shared-library load-only smoke"

guard_require_files "$TAG" "$ABI_SSOT" "$ARTIFACT_SSOT" "$RUNTIME_SSOT" "$ROADMAP" "$CARD_09" "$CARD_10" "$TASKBOARD" "$INDEX" "$CURRENT_STATE" "$METADATA_TOOL" "$LOAD_TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$METADATA_TOOL" "$LOAD_TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'provider package artifact' "$ABI_SSOT" "ABI SSOT must make package artifact the subject"
guard_expect_fixed_in_file "$TAG" 'shared-library-load-only-smoke' "$RUNTIME_SSOT" "runtime SSOT must split load-only smoke"
guard_expect_fixed_in_file "$TAG" 'required_export_resolved=0' "$RUNTIME_SSOT" "runtime SSOT must forbid export resolution in load-only"
guard_expect_fixed_in_file "$TAG" 'descriptor_read_executed=0' "$RUNTIME_SSOT" "runtime SSOT must forbid descriptor reads in load-only"
guard_expect_fixed_in_file "$TAG" 'hakorune_provider_descriptor_v1' "$ABI_SSOT" "ABI SSOT must name descriptor export"
guard_expect_fixed_in_file "$TAG" 'hakorune-provider-package-v1' "$ARTIFACT_SSOT" "artifact SSOT must define manifest v1"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-10-MIMALLOC-PROVIDER-SHARED-LIBRARY-LOAD-ONLY-SMOKE"' "$CURRENT_STATE" "current state latest card must advance to load-only smoke"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "MIMALLOC-PROVIDER-DESCRIPTOR-READ-SMOKE-296X-001"' "$CURRENT_STATE" "current state must expose descriptor-read blocker"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_09" "metadata preflight must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_10" "load-only smoke card must be landed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-DLL-LOAD-ONLY-SHARED-LIBRARY-SMOKE-296X-001' "$CARD_10" "load-only card must close historical blocker"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-DESCRIPTOR-READ-SMOKE-296X-001' "$CARD_10" "load-only card must select descriptor-read next"
guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-load-only-smoke-v0' "$CARD_10" "load-only card must define output contract"
guard_expect_fixed_in_file "$TAG" 'shared_library_load_executed=1' "$CARD_10" "load-only card must load the shared library"
guard_expect_fixed_in_file "$TAG" 'required_export_resolved=0' "$CARD_10" "load-only card must not resolve exports"
guard_expect_fixed_in_file "$TAG" 'descriptor_read_executed=0' "$CARD_10" "load-only card must not read descriptor"
guard_expect_fixed_in_file "$TAG" 'provider_call_executed=0' "$CARD_10" "load-only card must not call provider APIs"
guard_expect_fixed_in_file "$TAG" 'allocator_entrypoint_called=0' "$CARD_10" "load-only card must not call allocator entrypoints"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$CARD_10" "load-only card must keep provider inactive"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$CARD_10" "load-only card must keep replacement inactive"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_10" "load-only card must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'shared-library-load-only smoke with no export resolution' "$ROADMAP" "roadmap must split load-only and descriptor-read"
guard_expect_fixed_in_file "$TAG" 'descriptor-read smoke with descriptor export only' "$ROADMAP" "roadmap must include descriptor-read after load-only"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-DESCRIPTOR-READ-SMOKE-296X-001' "$TASKBOARD" "taskboard must expose descriptor-read row"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$LOAD_TOOL" "$INDEX" "check index must list load-only tool"

python3 -m py_compile "$METADATA_TOOL" "$LOAD_TOOL"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_provider_load_only.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
c_src="$tmp_dir/provider.c"
so="$tmp_dir/libhakorune_provider.so"
manifest="$tmp_dir/hakorune_provider.json"
metadata_out="$tmp_dir/metadata.out"
load_out="$tmp_dir/load.out"

cat > "$c_src" <<'C'
#include <stddef.h>

__attribute__((visibility("default")))
const void* hakorune_provider_descriptor_v1(void) {
    return NULL;
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
  "package_id": "org.hakorune.provider.load-only.fixture",
  "provider_kind": "allocator",
  "provider_name": "load-only-fixture",
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

python3 "$METADATA_TOOL" --manifest "$manifest" --out "$metadata_out"
python3 "$LOAD_TOOL" --manifest "$manifest" --out "$load_out"

guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-metadata-preflight-v0' "$metadata_out" "metadata tool must still accept manifest v1"
guard_expect_fixed_in_file "$TAG" 'dll_mode=metadata-preflight' "$metadata_out" "metadata tool must stay no-load"
guard_expect_fixed_in_file "$TAG" 'shared_library_load_executed=0' "$metadata_out" "metadata preflight must not load"
guard_expect_fixed_in_file "$TAG" 'required_export=hakorune_provider_descriptor_v1' "$metadata_out" "metadata preflight must preserve descriptor export"

guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-load-only-smoke-v0' "$load_out" "load tool must emit load-only contract"
guard_expect_fixed_in_file "$TAG" 'dll_mode=load-only' "$load_out" "load tool must emit load-only mode"
guard_expect_fixed_in_file "$TAG" 'schema_version=hakorune-provider-package-v1' "$load_out" "load tool must preserve schema version"
guard_expect_fixed_in_file "$TAG" 'provider_name=load-only-fixture' "$load_out" "load tool must preserve provider name"
guard_expect_fixed_in_file "$TAG" 'manifest_ready=1' "$load_out" "load tool must mark manifest ready"
guard_expect_fixed_in_file "$TAG" 'descriptor_ready=0' "$load_out" "load tool must keep descriptor unread"
guard_expect_fixed_in_file "$TAG" 'binary_hash_ready=1' "$load_out" "load tool must verify binary hash"
guard_expect_fixed_in_file "$TAG" 'shared_library_load_executed=1' "$load_out" "load tool must load shared library"
guard_expect_fixed_in_file "$TAG" 'required_export_resolved=0' "$load_out" "load tool must not resolve export"
guard_expect_fixed_in_file "$TAG" 'descriptor_read_executed=0' "$load_out" "load tool must not call descriptor"
guard_expect_fixed_in_file "$TAG" 'provider_call_executed=0' "$load_out" "load tool must not call provider API"
guard_expect_fixed_in_file "$TAG" 'allocator_entrypoint_called=0' "$load_out" "load tool must not call allocator entrypoints"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$load_out" "load tool must keep provider inactive"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$load_out" "load tool must keep replacement inactive"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$load_out" "load tool must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$load_out" "load tool must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$load_out" "load tool must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$load_out" "load tool must end with summary"

cat "$load_out"
echo "[$TAG] ok"
