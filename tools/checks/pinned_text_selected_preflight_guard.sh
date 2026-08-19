#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="pinned-text-selected-preflight"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

PURE="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_pure_compile.inc"
PREFLIGHT="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_pinned_text_selected_preflight.inc"
LOWERING="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_pinned_text_selected_lowering.inc"
TARGET_SESSION="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_pinned_text_target_machine_session.inc"
FINAL_CLOSURE="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_pinned_text_final_module_closure.inc"
GENERIC="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering.inc"
DISPATCH="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering_op_dispatch.inc"
SELECTED_DISPATCH="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_pinned_text_selected_dispatch.inc"
CARRIER="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_pinned_text_residence_carrier.inc"
FRAME="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_pinned_text_backend_frame.inc"
RUST_TEST="$ROOT_DIR/src/mir/builder/resolved_lowering/common_v2_s6c_cursor_cfg_tests.rs"
SMOKE="$ROOT_DIR/tools/checks/pinned_text_selected_preflight_smoke.sh"
VERIFIER_TEST="$ROOT_DIR/lang/c-abi/tests/pinned_text_selected_verifier_test.c"
FINAL_CLOSURE_TEST="$ROOT_DIR/lang/c-abi/tests/pinned_text_final_module_closure_test.c"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$PURE" "$PREFLIGHT" "$LOWERING" "$TARGET_SESSION" "$FINAL_CLOSURE" "$GENERIC" "$DISPATCH" "$SELECTED_DISPATCH" "$CARRIER" "$FRAME" "$RUST_TEST" "$SMOKE" "$VERIFIER_TEST" "$FINAL_CLOSURE_TEST"

count_fixed() {
  local needle="$1"
  shift
  (rg -F -o -- "$needle" "$@" || true) | wc -l | tr -d '[:space:]'
}

if [[ "$(count_fixed '#include "hako_llvmc_ffi_pinned_text_selected_preflight.inc"' "$PURE")" != "1" ]]; then
  guard_fail "$TAG" "pure-first owner must include one private selected preflight"
fi
if [[ "$(count_fixed 'hako_llvmc_ptfc_preflight_selected_candidate(' "$PURE" "$PREFLIGHT")" != "2" ]]; then
  guard_fail "$TAG" "selected preflight must have one definition and one consumer"
fi
if [[ "$(count_fixed 'hako_llvmc_ptfc_parse_carrier(' "$CARRIER" "$PREFLIGHT")" != "3" ]]; then
  guard_fail "$TAG" "neutral carrier parser must have one definition plus fixture and selected consumers"
fi
if [[ "$(count_fixed 'struct HakoPtfCarrierFixture' "$CARRIER" "$PREFLIGHT")" != "0" ]]; then
  guard_fail "$TAG" "shared parsed carrier facts must not retain fixture-only identity"
fi
if [[ "$(count_fixed 'pinned_text_residence_trap' "$FRAME")" != "1" ]]; then
  guard_fail "$TAG" "module census must recognize the explicit Residence Trap"
fi
if [[ "$(count_fixed '[freeze:contract][ptfc/textual-lowering-verified-target-closed]' "$LOWERING" "$SMOKE")" != "0" ]]; then
  guard_fail "$TAG" "landed TargetMachine handoff must retire the textual closed tag"
fi
if [[ "$(count_fixed 'HAKO_PINNED_TEXT_REAL_CANDIDATE_JSON_OUT' "$RUST_TEST" "$SMOKE")" != "2" ]]; then
  guard_fail "$TAG" "smoke must consume one runtime-generated real-candidate witness"
fi

preflight_line="$(rg -n -m1 'hako_llvmc_ptfc_preflight_selected_candidate' "$PURE" | cut -d: -f1)"
pattern_line="$(rg -n -m1 'hako_llvmc_match_indexof_line_text_state_residence_fn' "$PURE" | cut -d: -f1)"
if [[ -z "$preflight_line" || -z "$pattern_line" || "$preflight_line" -ge "$pattern_line" ]]; then
  guard_fail "$TAG" "selected preflight must run before every pattern emitter"
fi

if rg -n 'fopen|EMIT\(|ptfb_session_(open|emit_object)|emit_pinned_text_residence_carrier_fixture' "$PREFLIGHT"; then
  guard_fail "$TAG" "effect-free preflight must not emit IR, open a session, or reuse the fixture"
fi
if rg -n 'fopen|ptfb_session_(open|emit_object)|emit_pinned_text_residence_carrier_fixture|memcmp\(' "$LOWERING"; then
  guard_fail "$TAG" "selected textual lowerer must stay private, exact-width, and session-free"
fi
if [[ "$(count_fixed 'tmpfile()' "$LOWERING")" != "1" ]] ||
   [[ "$(count_fixed 'hako_llvmc_ptfc_verify_and_take_selected_llvm(' "$LOWERING" "$GENERIC")" != "2" ]]; then
  guard_fail "$TAG" "one tmpfile owner and one private verify/take consumer are required"
fi
if [[ "$(count_fixed 'LLVMCreateMemoryBufferWithMemoryRangeCopy' "$TARGET_SESSION")" != "1" ]] ||
   [[ "$(count_fixed 'hako_llvmc_ptfb_session_emit_object_from_bytes(' "$TARGET_SESSION" "$GENERIC")" != "2" ]]; then
  guard_fail "$TAG" "verified bytes must enter the sole TargetMachine session once"
fi
if [[ "$(count_fixed '#include "hako_llvmc_ffi_pinned_text_final_module_closure.inc"' "$PURE")" != "1" ]] ||
   [[ "$(count_fixed 'hako_llvmc_ptfc_verify_final_module_v1(' "$TARGET_SESSION" "$FINAL_CLOSURE")" != "3" ]]; then
  guard_fail "$TAG" "one private final-module closure must consume the selected module"
fi
if [[ "$(count_fixed 'LLVMVerifyModule' "$FINAL_CLOSURE")" != "1" ]] ||
   rg -n 'LLVMRunPasses|LLVMCreateBinary|LLVMObjectFile|LLVMCreateDisasm' "$FINAL_CLOSURE"; then
  guard_fail "$TAG" "final closure must verify the parsed module without a pass or object observer"
fi
closure_line="$(rg -n -m1 'if \(selected_candidate &&' "$TARGET_SESSION" | cut -d: -f1)"
emit_line="$(rg -n -m1 'if \(session->emit_to_file\(' "$TARGET_SESSION" | cut -d: -f1)"
if [[ -z "$closure_line" || -z "$emit_line" || "$closure_line" -ge "$emit_line" ]]; then
  guard_fail "$TAG" "final module closure must be immediately upstream of the sole emit"
fi
if [[ "$(count_fixed 'typedef void (*hako_ptfb_set_target_fn)(void*, const char*);' "$TARGET_SESSION")" != "1" ]]; then
  guard_fail "$TAG" "LLVMSetTarget binding must retain the LLVM18 void signature"
fi
if rg -n '/proc/self/fd|hako_pure_gen_' "$LOWERING" "$TARGET_SESSION"; then
  guard_fail "$TAG" "selected memory ingress must not recreate a named LLVM path"
fi
if [[ "$(count_fixed '#include "hako_llvmc_ffi_pinned_text_selected_dispatch.inc"' "$DISPATCH")" != "1" ]] ||
   [[ "$(count_fixed 'hako_llvmc_ptfc_try_emit_selected_op(' "$SELECTED_DISPATCH")" != "1" ]]; then
  guard_fail "$TAG" "generic op dispatch must delegate through one selected child include"
fi
if [[ "$(count_fixed 'pinned_text_selected_verifier_test.c' "$SMOKE")" != "1" ]] ||
   [[ "$(count_fixed 'hako_llvmc_ptfc_verify_and_take_selected_llvm(' "$VERIFIER_TEST")" != "3" ]]; then
  guard_fail "$TAG" "private verifier positive plus ordering/effect negatives must stay in one test translation unit"
fi
if [[ "$(count_fixed 'pinned_text_final_module_closure_test.c' "$SMOKE")" != "1" ]] ||
   [[ "$(count_fixed 'missing_nounwind' "$FINAL_CLOSURE_TEST")" != "5" ]] ||
   [[ "$(count_fixed 'missing_finish' "$FINAL_CLOSURE_TEST")" != "5" ]] ||
   [[ "$(count_fixed 'extra_call' "$FINAL_CLOSURE_TEST")" != "5" ]] ||
   [[ "$(count_fixed 'eh_module' "$FINAL_CLOSURE_TEST")" != "7" ]]; then
  guard_fail "$TAG" "final-module positive and attribute/Finish/call/EH negatives must remain focused"
fi

for file in "$PURE" "$PREFLIGHT" "$LOWERING" "$TARGET_SESSION" "$FINAL_CLOSURE" "$GENERIC" "$DISPATCH" "$SELECTED_DISPATCH" "$CARRIER" "$FRAME" "$RUST_TEST"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 760 )); then
    guard_fail "$TAG" "selected preflight source reached the 760-line split trigger: ${file#"$ROOT_DIR/"}=$lines"
  fi
done

echo "[$TAG] ok (strict preflight + private textual lowering + TargetMachine memory ingress)"
