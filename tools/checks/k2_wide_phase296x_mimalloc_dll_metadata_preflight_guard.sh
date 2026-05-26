#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-mimalloc-dll-metadata-preflight"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_08="docs/development/current/main/phases/phase-296x/296x-08-MIMALLOC-DLL-LOAD-ONLY-SELECTION.md"
CARD_09="docs/development/current/main/phases/phase-296x/296x-09-MIMALLOC-DLL-LOAD-ONLY-METADATA-PREFLIGHT.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_dll_metadata_preflight_guard.sh"
TOOL="tools/allocator/provider_package_metadata_preflight.py"

echo "[$TAG] checking phase-296x DLL metadata preflight"

guard_require_files "$TAG" "$CARD_08" "$CARD_09" "$TASKBOARD" "$INDEX" "$CURRENT_STATE" "$SELF_SCRIPT" "$TOOL"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$TOOL"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-09-MIMALLOC-DLL-LOAD-ONLY-METADATA-PREFLIGHT"' "$CURRENT_STATE" "current state latest card must advance to metadata preflight"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "MIMALLOC-DLL-LOAD-ONLY-SHARED-LIBRARY-SMOKE-296X-001"' "$CURRENT_STATE" "current state must expose shared-library smoke blocker"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_08" "DLL load-only selection must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_09" "metadata preflight card must be landed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-DLL-LOAD-ONLY-METADATA-PREFLIGHT-296X-001' "$CARD_09" "metadata card must identify blocker"
guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-metadata-preflight-v0' "$CARD_09" "metadata card must define output contract"
guard_expect_fixed_in_file "$TAG" 'shared_library_load_executed=0' "$CARD_09" "metadata card must keep shared-library loading closed"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$CARD_09" "metadata card must keep provider inactive"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$CARD_09" "metadata card must keep replacement inactive"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_09" "metadata card must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-DLL-LOAD-ONLY-SHARED-LIBRARY-SMOKE-296X-001' "$CARD_09" "metadata card must select shared-library smoke next"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-DLL-LOAD-ONLY-METADATA-PREFLIGHT-296X-001' "$TASKBOARD" "taskboard must expose metadata preflight row"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"

python3 -m py_compile "$TOOL"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_dll_metadata.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
manifest="$tmp_dir/hakorune_provider.json"
out="$tmp_dir/report.out"

cat > "$manifest" <<'JSON'
{
  "provider_name": "hakorune-mimalloc-exp",
  "abi": "hakorune-provider-v1",
  "target": "x86_64-unknown-linux-gnu",
  "profile": "speed",
  "binary": "libhakorune_provider.so",
  "binary_sha256": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
  "contract_hash": "abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789",
  "features": {
    "diagnostic_stats": false,
    "timing_stats": false,
    "speed_lane": true
  },
  "exports": [
    "hakorune_provider_get_api_v1"
  ]
}
JSON

python3 "$TOOL" --manifest "$manifest" --out "$out"

guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-metadata-preflight-v0' "$out" "tool must emit metadata preflight contract"
guard_expect_fixed_in_file "$TAG" 'dll_mode=metadata-preflight' "$out" "tool must emit metadata mode"
guard_expect_fixed_in_file "$TAG" 'provider_name=hakorune-mimalloc-exp' "$out" "tool must preserve provider name"
guard_expect_fixed_in_file "$TAG" 'abi=hakorune-provider-v1' "$out" "tool must preserve ABI"
guard_expect_fixed_in_file "$TAG" 'binary=libhakorune_provider.so' "$out" "tool must preserve binary name"
guard_expect_fixed_in_file "$TAG" 'required_export=hakorune_provider_get_api_v1' "$out" "tool must require the single export"
guard_expect_fixed_in_file "$TAG" 'manifest_ready=1' "$out" "tool must mark manifest ready"
guard_expect_fixed_in_file "$TAG" 'binary_hash_ready=1' "$out" "tool must mark binary hash ready"
guard_expect_fixed_in_file "$TAG" 'shared_library_load_executed=0' "$out" "tool must not load a shared library"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$out" "tool must keep provider inactive"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$out" "tool must keep replacement inactive"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$out" "tool must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$out" "tool must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$out" "tool must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$out" "tool must end with summary"

cat "$out"
echo "[$TAG] ok"
