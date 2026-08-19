#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="s6c-pinned-corridor-structural-zero-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TARGET_SESSION="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_pinned_text_target_machine_session.inc"
DRIVER="$ROOT_DIR/lang/c-abi/tests/s6c_pinned_corridor_structural_zero_driver.c"
CHECKER="$ROOT_DIR/tools/perf/s6c_pinned_corridor_structural_zero.py"
SMOKE="$ROOT_DIR/tools/checks/s6c_pinned_corridor_structural_zero_smoke.sh"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$TARGET_SESSION" "$DRIVER" "$CHECKER" "$SMOKE"

count_fixed() {
  local needle="$1"
  shift
  (rg -F -o -- "$needle" "$@" || true) | wc -l | tr -d '[:space:]'
}

if [[ "$(count_fixed 'HAKO_LLVMC_PTFC_FINAL_MODULE_EVIDENCE_V1' "$TARGET_SESSION" "$DRIVER")" != "4" ]]; then
  guard_fail "$TAG" "one default-off final-module hook and one test override are required"
fi
closure_line="$(rg -n -m1 'hako_llvmc_ptfc_verify_final_module_v1\(' "$TARGET_SESSION" | tail -1 | cut -d: -f1)"
evidence_line="$(rg -n -m1 'HAKO_LLVMC_PTFC_FINAL_MODULE_EVIDENCE_V1\(session' "$TARGET_SESSION" | tail -1 | cut -d: -f1)"
emit_line="$(rg -n -m1 'session->emit_to_file\(' "$TARGET_SESSION" | cut -d: -f1)"
if [[ -z "$closure_line" || -z "$evidence_line" || -z "$emit_line" ]] ||
   (( closure_line >= evidence_line || evidence_line >= emit_line )); then
  guard_fail "$TAG" "test evidence borrow must follow final closure and precede sole emit"
fi
if rg -n 'getenv|setenv|putenv|LLVMRunPasses|LLVMCreateBinary|LLVMObjectFile|LLVMCreateDisasm' "$DRIVER"; then
  guard_fail "$TAG" "evidence driver must not add env authority, passes, or an object observer"
fi
if [[ "$(count_fixed 'LLVMPrintModuleToString' "$DRIVER")" != "1" ]] ||
   [[ "$(count_fixed 'hako_llvmc_compile_json_pure_first(' "$DRIVER")" != "1" ]]; then
  guard_fail "$TAG" "driver must borrow one final module from the real selected compile route"
fi
for needle in \
  'promotion-evidence-only' \
  'unexpected candidate IR call' \
  'wide or unaligned scalar read' \
  'indirect or PLT call in candidate' \
  'target datalayout' \
  'target triple'; do
  if [[ "$(count_fixed "$needle" "$CHECKER")" -lt "1" ]]; then
    guard_fail "$TAG" "checker contract is missing: $needle"
  fi
done
for needle in unexpected-call noalias wide-load indirect-call; do
  if [[ "$(count_fixed "$needle" "$SMOKE")" -lt "1" ]]; then
    guard_fail "$TAG" "focused negative is missing: $needle"
  fi
done
if rg -n 'section_count|relocation_count|ret_count|backward.branch|natural.loop' "$CHECKER" "$SMOKE"; then
  guard_fail "$TAG" "machine layout counts and inferred natural loops are non-authority"
fi
for file in "$TARGET_SESSION" "$DRIVER" "$CHECKER" "$SMOKE"; do
  lines="$(wc -l <"$file" | tr -d '[:space:]')"
  if (( lines >= 760 )); then
    guard_fail "$TAG" "structural evidence source reached the 760-line split trigger: ${file#"$ROOT_DIR/"}=$lines"
  fi
done

echo "[$TAG] ok (test-only final-module borrow + offline IR/linked-assembly evidence)"
