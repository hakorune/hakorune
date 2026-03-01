#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

FILE="crates/nyash_kernel/src/hako_forward.rs"

if rg -n "AtomicUsize|HAKO_PLUGIN_INVOKE_BY_NAME|HAKO_FUTURE_SPAWN_INSTANCE|HAKO_STRING_DISPATCH" "$FILE" >/dev/null; then
  echo "[phase29cc-hako-forward-registry-guard] violation: Rust-local registry state leaked back into hako_forward.rs" >&2
  exit 1
fi

if ! rg -n "nyrt_hako_try_plugin_invoke_by_name|nyrt_hako_try_future_spawn_instance|nyrt_hako_try_string_dispatch" "$FILE" >/dev/null; then
  echo "[phase29cc-hako-forward-registry-guard] violation: C registry try API call path missing" >&2
  exit 1
fi

if ! rg -n "export_name = \"nyrt\\.hako\\.register_plugin_invoke_by_name\"" "$FILE" >/dev/null; then
  echo "[phase29cc-hako-forward-registry-guard] violation: dot-name compatibility export missing (plugin invoke)" >&2
  exit 1
fi

if ! rg -n "export_name = \"nyrt\\.hako\\.register_future_spawn_instance\"" "$FILE" >/dev/null; then
  echo "[phase29cc-hako-forward-registry-guard] violation: dot-name compatibility export missing (future spawn)" >&2
  exit 1
fi

if ! rg -n "export_name = \"nyrt\\.hako\\.register_string_dispatch\"" "$FILE" >/dev/null; then
  echo "[phase29cc-hako-forward-registry-guard] violation: dot-name compatibility export missing (string dispatch)" >&2
  exit 1
fi

echo "[phase29cc-hako-forward-registry-guard] ok"
