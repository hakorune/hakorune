#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
FIXTURE="$ROOT_DIR/apps/tests/ring1_array_provider/array_size_push_min.hako"
STRING_FIXTURE="$ROOT_DIR/apps/tests/ring1_array_provider/array_string_set_min.hako"
SHADOW_FIXTURE="$ROOT_DIR/apps/tests/ring1_array_provider/array_string_shadow_guard_min.hako"
SMOKE="$ROOT_DIR/tools/smokes/v2/profiles/integration/ring1_providers/ring1_array_provider_vm.sh"
STRING_SMOKE="$ROOT_DIR/tools/smokes/v2/profiles/integration/ring1_providers/ring1_array_string_provider_vm.sh"
SHADOW_SMOKE="$ROOT_DIR/tools/smokes/v2/profiles/integration/ring1_providers/ring1_array_string_shadow_guard_vm.sh"
RING1_MOD="$ROOT_DIR/src/providers/ring1/mod.rs"
PROVIDER_LOCK="$ROOT_DIR/src/runtime/provider_lock/mod.rs"
PLUGIN_HOST="$ROOT_DIR/src/runtime/plugin_host.rs"
source "$(dirname "$0")/lib/guard_common.sh"

TAG="ring1-array-provider-guard"

cd "$ROOT_DIR"
echo "[$TAG] checking ring1 array provider wiring"

guard_require_command "$TAG" rg
guard_require_files "$TAG" "$FIXTURE" "$STRING_FIXTURE" "$SHADOW_FIXTURE" "$SMOKE" "$STRING_SMOKE" "$SHADOW_SMOKE" "$RING1_MOD" "$PROVIDER_LOCK" "$PLUGIN_HOST"
guard_require_exec_files "$TAG" "$SMOKE" "$STRING_SMOKE" "$SHADOW_SMOKE"

guard_expect_in_file "$TAG" '^pub mod array;' "$RING1_MOD" "ring1 mod must export array"
guard_expect_in_file "$TAG" 'set_arraybox_provider' "$PROVIDER_LOCK" "provider_lock must expose set_arraybox_provider"
guard_expect_in_file "$TAG" 'new_arraybox_provider_instance' "$PROVIDER_LOCK" "provider_lock must expose new_arraybox_provider_instance"
guard_expect_in_file "$TAG" 'Ring1ArrayService' "$PLUGIN_HOST" "plugin_host must wire ring1 array provider"

guard_expect_in_file "$TAG" 'ring1_array_provider/array_size_push_min.hako' "$SMOKE" "smoke must run ring1 array fixture"
guard_expect_in_file "$TAG" 'ARRAY_PROVIDER_OK size=2 get0=11' "$SMOKE" "smoke expected output contract is missing"
guard_expect_in_file "$TAG" 'ring1_array_provider/array_string_set_min.hako' "$STRING_SMOKE" "string smoke must run ring1 array string fixture"
guard_expect_in_file "$TAG" 'ARRAY_STRING_SET_OK size=1' "$STRING_SMOKE" "string smoke expected output contract is missing"
guard_expect_in_file "$TAG" 'ring1_array_provider/array_string_shadow_guard_min.hako' "$SHADOW_SMOKE" "shadow guard smoke must run ring1 array shadow fixture"
guard_expect_in_file "$TAG" 'ARRAY_STRING_SHADOW_GUARD_OK handled=1 got=1' "$SHADOW_SMOKE" "shadow guard output contract is missing"

echo "[$TAG] ok"
