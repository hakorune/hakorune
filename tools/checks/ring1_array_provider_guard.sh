#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
FIXTURE="$ROOT_DIR/apps/tests/ring1_array_provider/array_size_push_min.hako"
SMOKE="$ROOT_DIR/tools/smokes/v2/profiles/integration/ring1_providers/ring1_array_provider_vm.sh"
RING1_MOD="$ROOT_DIR/src/providers/ring1/mod.rs"
PROVIDER_LOCK="$ROOT_DIR/src/runtime/provider_lock/mod.rs"
PLUGIN_HOST="$ROOT_DIR/src/runtime/plugin_host.rs"
source "$(dirname "$0")/lib/guard_common.sh"

TAG="ring1-array-provider-guard"

cd "$ROOT_DIR"
echo "[$TAG] checking ring1 array provider wiring"

guard_require_command "$TAG" rg
guard_require_files "$TAG" "$FIXTURE" "$SMOKE" "$RING1_MOD" "$PROVIDER_LOCK" "$PLUGIN_HOST"
guard_require_exec_files "$TAG" "$SMOKE"

guard_expect_in_file "$TAG" '^pub mod array;' "$RING1_MOD" "ring1 mod must export array"
guard_expect_in_file "$TAG" 'set_arraybox_provider' "$PROVIDER_LOCK" "provider_lock must expose set_arraybox_provider"
guard_expect_in_file "$TAG" 'new_arraybox_provider_instance' "$PROVIDER_LOCK" "provider_lock must expose new_arraybox_provider_instance"
guard_expect_in_file "$TAG" 'Ring1ArrayService' "$PLUGIN_HOST" "plugin_host must wire ring1 array provider"

guard_expect_in_file "$TAG" 'ring1_array_provider/array_size_push_min.hako' "$SMOKE" "smoke must run ring1 array fixture"
guard_expect_in_file "$TAG" 'ARRAY_PROVIDER_OK size=2 get0=11' "$SMOKE" "smoke expected output contract is missing"

echo "[$TAG] ok"
