#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
FIXTURE="$ROOT_DIR/apps/tests/ring1_console_provider/console_warn_error_min.hako"
SMOKE="$ROOT_DIR/tools/smokes/v2/profiles/integration/ring1_providers/ring1_console_provider_vm.sh"
RING1_MOD="$ROOT_DIR/src/providers/ring1/mod.rs"
PROVIDER_LOCK="$ROOT_DIR/src/runtime/provider_lock/mod.rs"
PLUGIN_HOST="$ROOT_DIR/src/runtime/plugin_host.rs"
CONSOLE_PROVIDER="$ROOT_DIR/src/providers/ring1/console/mod.rs"
source "$(dirname "$0")/lib/guard_common.sh"

TAG="ring1-console-provider-guard"

cd "$ROOT_DIR"
echo "[$TAG] checking ring1 console provider wiring"

guard_require_command "$TAG" rg
guard_require_files "$TAG" \
  "$FIXTURE" \
  "$SMOKE" \
  "$RING1_MOD" \
  "$PROVIDER_LOCK" \
  "$PLUGIN_HOST" \
  "$CONSOLE_PROVIDER"
guard_require_exec_files "$TAG" "$SMOKE"

guard_expect_in_file "$TAG" '^pub mod console;' "$RING1_MOD" "ring1 mod must export console"
guard_expect_in_file "$TAG" 'set_consolebox_provider' "$PROVIDER_LOCK" "provider_lock must expose set_consolebox_provider"
guard_expect_in_file "$TAG" 'new_consolebox_provider_instance' "$PROVIDER_LOCK" "provider_lock must expose new_consolebox_provider_instance"
guard_expect_in_file "$TAG" 'Ring1ConsoleService' "$PLUGIN_HOST" "plugin_host must wire ring1 console provider"
guard_expect_in_file "$TAG" 'Ring1ConsoleService' "$CONSOLE_PROVIDER" "console provider implementation must define Ring1ConsoleService"

guard_expect_in_file "$TAG" 'ring1_console_provider/console_warn_error_min.hako' "$SMOKE" "smoke must run ring1 console fixture"
guard_expect_in_file "$TAG" 'CONSOLE_PROVIDER_OK warn=1 error=1' "$SMOKE" "smoke expected output contract is missing"

echo "[$TAG] ok"
