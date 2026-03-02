#!/usr/bin/env bash
# route_no_fallback_guard.sh
# Fail-fast guard: fallback/helper toggle must stay disabled on daily routes.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PROBE="$ROOT/checks/route_env_probe.sh"

if [[ ! -x "$PROBE" ]]; then
  echo "[FAIL] route_no_fallback_guard: probe missing/executable: $PROBE" >&2
  exit 1
fi

if [[ ! -x "${NYASH_BIN:-$ROOT/../target/release/hakorune}" ]]; then
  echo "[WARN] route_no_fallback_guard: nyash binary missing; probe will still validate env" >&2
fi

for route in direct hako-mainline; do
  out="$("$PROBE" --route "$route" --require-no-fallback)"
  if ! printf '%s\n' "$out" | rg -q "\\[route_env_probe\\] route = ${route}$"; then
    echo "[FAIL] route_no_fallback_guard: route marker missing for $route" >&2
    printf '%s\n' "$out" >&2
    exit 1
  fi
done

echo "[PASS] route_no_fallback_guard"
exit 0
