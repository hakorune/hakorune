#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

LOCK_DOC="docs/development/current/main/phases/phase-29cc/29cc-215-runtime-execution-path-observability-lock-ssot.md"
RUNTIME_LOCK="docs/development/current/main/phases/phase-29cc/29cc-214-runtime-rust-thin-to-zero-execution-path-ssot.md"
CUTOVER_SSOT="docs/development/current/main/design/hako-runtime-c-abi-cutover-order-ssot.md"
RUNNER_INIT="src/runner_plugin_init.rs"
PLUGIN_LOADER_ENABLED="src/runtime/plugin_loader_v2/enabled/mod.rs"
PLUGIN_LOADER_STUB="src/runtime/plugin_loader_v2/stub.rs"
ENV_FLAGS="src/config/env/box_factory_flags.rs"
DEV_GATE="tools/checks/dev_gate.sh"

for file in \
  "$LOCK_DOC" \
  "$RUNTIME_LOCK" \
  "$CUTOVER_SSOT" \
  "$RUNNER_INIT" \
  "$PLUGIN_LOADER_ENABLED" \
  "$PLUGIN_LOADER_STUB" \
  "$ENV_FLAGS" \
  "$DEV_GATE"; do
  if [ ! -f "$file" ]; then
    echo "[runtime-exec-zero-guard] missing file: $file" >&2
    exit 1
  fi
done

for keyword in \
  "execution-path-zero" \
  "Route drift observability lock" \
  "phase29cc_runtime_execution_path_zero_guard.sh" \
  "[runtime/exec-path]"; do
  if ! rg -F -q "$keyword" "$LOCK_DOC" "$RUNNER_INIT"; then
    echo "[runtime-exec-zero-guard] missing keyword: $keyword" >&2
    exit 1
  fi
done

if ! rg -q "plugin_exec_mode\(\)" "$RUNNER_INIT"; then
  echo "[runtime-exec-zero-guard] missing plugin_exec_mode observability hook" >&2
  exit 1
fi

if ! rg -q "box_factory_policy\(\)" "$RUNNER_INIT"; then
  echo "[runtime-exec-zero-guard] missing box_factory_policy observability hook" >&2
  exit 1
fi

if ! rg -q "backend_kind\(\)" "$PLUGIN_LOADER_ENABLED" "$PLUGIN_LOADER_STUB"; then
  echo "[runtime-exec-zero-guard] missing plugin_loader backend_kind contract" >&2
  exit 1
fi

if ! rg -q "\"module_first\"" "$ENV_FLAGS"; then
  echo "[runtime-exec-zero-guard] missing plugin exec mode contract (module_first)" >&2
  exit 1
fi

if ! rg -q "phase29cc_runtime_execution_path_zero_guard.sh" "$DEV_GATE"; then
  echo "[runtime-exec-zero-guard] dev_gate missing runtime-exec-zero guard wiring" >&2
  exit 1
fi

echo "[runtime-exec-zero-guard] ok"
