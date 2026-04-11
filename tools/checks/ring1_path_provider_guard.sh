#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
FIXTURE="$ROOT_DIR/apps/tests/ring1_path_provider/path_join_exists_min.hako"
SMOKE="$ROOT_DIR/tools/smokes/v2/profiles/integration/ring1_providers/ring1_path_provider_vm.sh"
RING1_MOD="$ROOT_DIR/src/providers/ring1/mod.rs"
PROVIDER_LOCK="$ROOT_DIR/src/runtime/provider_lock/mod.rs"
PROVIDER_LOCK_PATH="$ROOT_DIR/src/runtime/provider_lock/path.rs"
PLUGIN_HOST="$ROOT_DIR/src/runtime/plugin_host.rs"
PATH_PROVIDER="$ROOT_DIR/src/providers/ring1/path/mod.rs"
PATH_BOX="$ROOT_DIR/src/boxes/path_box.rs"
BUILTIN_FACTORY="$ROOT_DIR/src/box_factory/builtin.rs"
BUILTIN_PATH_IMPL="$ROOT_DIR/src/box_factory/builtin_impls/path_box.rs"
VM_BOXCALL_HANDLER="$ROOT_DIR/src/backend/mir_interpreter/handlers/boxcall_dispatch.rs"
VM_PATH_HANDLER="$ROOT_DIR/src/backend/mir_interpreter/handlers/boxes_path.rs"
VM_METHOD_HANDLER="$ROOT_DIR/src/backend/mir_interpreter/handlers/calls/method.rs"
source "$(dirname "$0")/lib/guard_common.sh"

TAG="ring1-path-provider-guard"

cd "$ROOT_DIR"
echo "[$TAG] checking ring1 path provider wiring"

guard_require_command "$TAG" rg
guard_require_files "$TAG" \
  "$FIXTURE" \
  "$SMOKE" \
  "$RING1_MOD" \
  "$PROVIDER_LOCK" \
  "$PROVIDER_LOCK_PATH" \
  "$PLUGIN_HOST" \
  "$PATH_PROVIDER" \
  "$PATH_BOX" \
  "$BUILTIN_FACTORY" \
  "$BUILTIN_PATH_IMPL" \
  "$VM_BOXCALL_HANDLER" \
  "$VM_PATH_HANDLER" \
  "$VM_METHOD_HANDLER"
guard_require_exec_files "$TAG" "$SMOKE"

guard_expect_in_file "$TAG" '^pub mod path;' "$RING1_MOD" "ring1 mod must export path"
guard_expect_in_file "$TAG" 'PathService' "$PROVIDER_LOCK" "provider_lock must expose PathService"
guard_expect_in_file "$TAG" 'set_pathbox_provider' "$PROVIDER_LOCK" "provider_lock must expose set_pathbox_provider"
guard_expect_in_file "$TAG" 'get_pathbox_provider_instance' "$PROVIDER_LOCK" "provider_lock must expose get_pathbox_provider_instance"
guard_expect_in_file "$TAG" 'get_pathbox_provider_instance' "$PROVIDER_LOCK_PATH" "provider_lock/path must define get_pathbox_provider_instance"
guard_expect_in_file "$TAG" 'Ring1PathService' "$PLUGIN_HOST" "plugin_host must wire ring1 path provider"
guard_expect_in_file "$TAG" 'Ring1PathService' "$PATH_PROVIDER" "path provider implementation must define Ring1PathService"
guard_expect_in_file "$TAG" 'PathBox' "$BUILTIN_FACTORY" "builtin factory must advertise PathBox creation route"
guard_expect_in_file "$TAG" 'PathBox::try_new' "$BUILTIN_PATH_IMPL" "builtin PathBox impl must use provider-backed constructor"
guard_expect_in_file "$TAG" 'try_handle_path_box_boxcall' "$VM_BOXCALL_HANDLER" "boxcall path must wire PathBox handler"
guard_expect_in_file "$TAG" 'try_handle_path_box_methodcall' "$VM_METHOD_HANDLER" "methodcall path must wire PathBox handler"
guard_expect_in_file "$TAG" 'PathBox.join' "$VM_PATH_HANDLER" "PathBox handler must enforce join arg contract"

guard_expect_in_file "$TAG" 'ring1_path_provider/path_join_exists_min.hako' "$SMOKE" "smoke must run ring1 path fixture"
guard_expect_in_file "$TAG" 'PATH_PROVIDER_OK join=apps/tests norm=apps/tests' "$SMOKE" "smoke expected output contract is missing"
guard_expect_in_file "$TAG" 'new PathBox' "$FIXTURE" "fixture must instantiate PathBox directly"
guard_expect_in_file "$TAG" 'join\("apps", "tests"\)' "$FIXTURE" "fixture must verify provider-backed join behavior"

echo "[$TAG] ok"
