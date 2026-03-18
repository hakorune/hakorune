#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

COMPAT_FILE="crates/nyash_kernel/src/hako_forward.rs"
BRIDGE_FILE="crates/nyash_kernel/src/hako_forward_bridge.rs"

if rg -n "AtomicUsize|HAKO_PLUGIN_INVOKE_BY_NAME|HAKO_FUTURE_SPAWN_INSTANCE|HAKO_STRING_DISPATCH" "$COMPAT_FILE" >/dev/null; then
  echo "[phase29cc-hako-forward-registry-guard] violation: Rust-local registry state leaked back into hako_forward.rs" >&2
  exit 1
fi

if rg -n "call_plugin_invoke_by_name|register_plugin_invoke_by_name|nyrt_hako_try_plugin_invoke_by_name|pub mod string_ops|nyrt_hako_try_" "$COMPAT_FILE" >/dev/null; then
  echo "[phase29cc-hako-forward-registry-guard] violation: hako_forward.rs must stay compat-export only" >&2
  exit 1
fi

if ! rg -n "export_name = \"nyrt\\.hako\\.register_future_spawn_instance\"" "$COMPAT_FILE" >/dev/null; then
  echo "[phase29cc-hako-forward-registry-guard] violation: dot-name compatibility export missing (future spawn)" >&2
  exit 1
fi

if ! rg -n "export_name = \"nyrt\\.hako\\.register_string_dispatch\"" "$COMPAT_FILE" >/dev/null; then
  echo "[phase29cc-hako-forward-registry-guard] violation: dot-name compatibility export missing (string dispatch)" >&2
  exit 1
fi

if ! rg -n "nyrt_hako_try_future_spawn_instance|nyrt_hako_try_string_dispatch" "$BRIDGE_FILE" >/dev/null; then
  echo "[phase29cc-hako-forward-registry-guard] violation: C registry try API call path missing in bridge" >&2
  exit 1
fi

if rg -n "register_plugin_invoke_by_name|call_plugin_invoke_by_name|nyrt_hako_try_plugin_invoke_by_name|nyrt_hako_register_plugin_invoke_by_name|HakoPluginInvokeByNameFn" "$BRIDGE_FILE" >/dev/null; then
  echo "[phase29cc-hako-forward-registry-guard] violation: by-name registry residue reappeared in hako_forward_bridge.rs" >&2
  exit 1
fi

if ! rg -n "call_future_spawn_instance|call_string_dispatch" "$BRIDGE_FILE" >/dev/null; then
  echo "[phase29cc-hako-forward-registry-guard] violation: bridge call entrypoints missing" >&2
  exit 1
fi

echo "[phase29cc-hako-forward-registry-guard] ok"
