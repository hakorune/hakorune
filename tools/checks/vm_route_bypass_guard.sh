#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

echo "[vm-route-bypass-guard] checking fallback callsite ownership"

if command -v rg >/dev/null 2>&1; then
  hits="$(rg -n "execute_vm_fallback_interpreter\\(" src/runner -S || true)"
else
  echo "[vm-route-bypass-guard] ERROR: rg is required" >&2
  exit 2
fi

if [[ -z "$hits" ]]; then
  echo "[vm-route-bypass-guard] ERROR: fallback callsite not found"
  exit 1
fi

disallowed="$(printf "%s\n" "$hits" | rg -v "src/runner/route_orchestrator.rs|src/runner/modes/vm_fallback.rs" || true)"
if [[ -n "$disallowed" ]]; then
  echo "[vm-route-bypass-guard] ERROR: disallowed fallback callsite detected"
  echo "$disallowed"
  exit 1
fi

if ! rg -q "enforce_vm_compat_fallback_guard_or_exit" src/runner/modes/vm_fallback.rs; then
  echo "[vm-route-bypass-guard] ERROR: vm_fallback guard hook missing"
  exit 1
fi

echo "[vm-route-bypass-guard] ok"
