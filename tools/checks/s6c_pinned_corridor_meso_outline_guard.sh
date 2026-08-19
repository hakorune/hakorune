#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="s6c-pinned-corridor-meso-outline-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

PROJECTOR="$ROOT_DIR/tools/perf/s6c_pinned_corridor_meso_outline.py"
SMOKE="$ROOT_DIR/tools/checks/s6c_pinned_corridor_meso_outline_smoke.sh"
DRIVER="$ROOT_DIR/lang/c-abi/tests/s6c_pinned_corridor_structural_zero_driver.c"
guard_require_command "$TAG" rg
guard_require_files "$TAG" "$PROJECTOR" "$SMOKE" "$DRIVER"

count_fixed() { (rg -F -o -- "$1" "${@:2}" || true) | wc -l | tr -d '[:space:]'; }

for needle in \
  'promotion-evidence-only' \
  '"blocks": 20' '"instructions": 92' '"edges": 35' '"phis": 5' '"returns": 2' \
  'retained scan depends on lifecycle/lane shell' \
  'outlined graph differs from retained real scan graph' \
  'retained scan digest drift'; do
  [[ "$(count_fixed "$needle" "$PROJECTOR")" -ge 1 ]] || \
    guard_fail "$TAG" "projector contract missing: $needle"
done
for negative in entry-extra root-offset lane-dependency removed-phi scan-drift finish-order; do
  [[ "$(count_fixed "$negative" "$SMOKE")" -ge 1 ]] || \
    guard_fail "$TAG" "outline negative missing: $negative"
done
[[ "$(count_fixed 'hako_llvmc_compile_json_pure_first(' "$DRIVER")" == 1 ]] || \
  guard_fail "$TAG" "outline must originate in one real final-module capture"
if rg -n 'LLVMRunPasses|opt -|mem2reg|fallback|retry|ny_main.*time|leaf.*plan.*loop' "$PROJECTOR" "$SMOKE"; then
  guard_fail "$TAG" "outline must not optimize, reconstruct, time, fallback, or retry"
fi
if rg -n 'hako_s6c_meso' lang/c-abi/shims src --glob '!README.md'; then
  guard_fail "$TAG" "outlined helper must remain absent from compiler and production sources"
fi
for file in "$PROJECTOR" "$SMOKE" "$DRIVER"; do
  lines="$(wc -l <"$file" | tr -d '[:space:]')"
  (( lines < 760 )) || guard_fail "$TAG" "source reached 760-line split trigger: ${file#"$ROOT_DIR/"}=$lines"
done

echo "[$TAG] ok (exact shell subtraction + retained graph digest; no second CFG authority)"
