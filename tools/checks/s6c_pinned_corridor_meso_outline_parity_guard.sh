#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="s6c-pinned-corridor-meso-outline-parity-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

DRIVER="$ROOT_DIR/lang/c-abi/tests/s6c_pinned_corridor_meso_object_driver.c"
RUNNER="$ROOT_DIR/lang/c-abi/tests/s6c_pinned_corridor_meso_outline_parity.c"
SMOKE="$ROOT_DIR/tools/checks/s6c_pinned_corridor_meso_outline_parity_smoke.sh"
guard_require_command "$TAG" rg
guard_require_files "$TAG" "$DRIVER" "$RUNNER" "$SMOKE"

count_fixed() { (rg -F -o -- "$1" "${@:2}" || true) | wc -l | tr -d '[:space:]'; }

[[ "$(count_fixed 'hako_llvmc_ptfb_session_emit_owned_buffer(' "$DRIVER")" == 1 ]] || \
  guard_fail "$TAG" "outlined module must use one retained LLVM18 session emission"
[[ "$(count_fixed 'hako_llvmc_compile_json_pure_first(' "$DRIVER")" == 1 ]] || \
  guard_fail "$TAG" "object driver must be entered by one real selected compile"
for needle in hako_s6c_candidate hako_s6c_meso hako_text_formal_residence_enter_v1 hako_text_formal_residence_finish_or_abort_v1 oracle alias-one alias-multi; do
  [[ "$(count_fixed "$needle" "$RUNNER")" -ge 1 ]] || guard_fail "$TAG" "parity contract missing: $needle"
done
for family in w1-first w1-middle w1-last w1-miss w2-first w2-middle w2-last w2-miss w3-first w3-middle w3-last w3-miss w4-first w4-middle w4-last w4-miss mixed-first mixed-middle mixed-last mixed-miss; do
  [[ "$(count_fixed "$family" "$RUNNER")" == 1 ]] || guard_fail "$TAG" "parity corpus drift: $family"
done
for needle in wrong.ll 'ret i64 999' promotion-test-support; do
  [[ "$(count_fixed "$needle" "$SMOKE")" -ge 1 ]] || guard_fail "$TAG" "parity negative/control missing: $needle"
done
if rg -n 'CLOCK_|benchmark|threshold|fallback|retry|LLVMRunPasses|memcmp' "$DRIVER" "$SMOKE"; then
  guard_fail "$TAG" "parity cell must not time, optimize, fallback, or retry"
fi
for file in "$DRIVER" "$RUNNER" "$SMOKE"; do
  lines="$(wc -l <"$file" | tr -d '[:space:]')"
  (( lines < 760 )) || guard_fail "$TAG" "source reached 760-line split trigger: ${file#"$ROOT_DIR/"}=$lines"
done

echo "[$TAG] ok (same runtime roots + whole/outline/oracle parity; no benchmark authority)"
