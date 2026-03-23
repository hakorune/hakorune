#!/usr/bin/env bash
set -euo pipefail
DIR="$(cd "$(dirname "$0")" && pwd)"

# Quick profile guard: this aggregator chains many heavier reps and can exceed
# the per-test timeout in quick. Skip under quick to avoid spurious timeouts.
if [[ "${SMOKES_CURRENT_PROFILE:-}" == "quick" ]]; then
  echo "[SKIP] phase2160/run_all is disabled under quick profile"
  exit 0
fi

run() { local f="$1"; [[ -x "$f" ]] || chmod +x "$f"; bash "$f"; }

run "$DIR/stageb_program_json_shape_canary_vm.sh" || true
run "$DIR/stageb_program_json_method_shape_canary_vm.sh" || true
run "$DIR/stageb_multi_method_shape_canary_vm.sh" || true
run "$DIR/program_to_mir_exe_return_canary_vm.sh" || true
run "$DIR/program_to_mir_exe_binop_canary_vm.sh" || true
run "$DIR/program_to_mir_exe_compare_canary_vm.sh" || true
run "$DIR/program_to_mir_exe_compare_lt_canary_vm.sh" || true
run "$DIR/registry_optin_canary_vm.sh" || true
run "$DIR/registry_optin_binop_intint_canary_vm.sh" || true
run "$DIR/registry_optin_compare_varint_canary_vm.sh" || true
run "$DIR/registry_optin_method_arraymap_canary_vm.sh" || true
run "$DIR/registry_optin_method_arraymap_push_canary_vm.sh" || true
run "$DIR/registry_optin_method_arraymap_get_canary_vm.sh" || true
run "$DIR/registry_optin_method_arraymap_set_canary_vm.sh" || true
run "$DIR/registry_optin_method_arraymap_len_canary_vm.sh" || true
run "$DIR/registry_optin_compare_varvar_canary_vm.sh" || true
run "$DIR/registry_optin_return_binop_varvar_canary_vm.sh" || true
run "$DIR/registry_optin_compare_fold_binints_canary_vm.sh" || true
run "$DIR/registry_optin_compare_fold_varint_canary_vm.sh" || true
run "$DIR/builder_min_method_arraymap_get_canary_vm.sh" || true
run "$DIR/builder_min_method_arraymap_push_canary_vm.sh" || true
run "$DIR/builder_min_method_arraymap_set_canary_vm.sh" || true
run "$DIR/builder_min_method_arraymap_len_canary_vm.sh" || true
run "$DIR/builder_min_return_binop_intint_canary_vm.sh" || true
run "$DIR/builder_min_if_compare_intint_canary_vm.sh" || true
run "$DIR/builder_min_return_binop_varvar_canary_vm.sh" || true
run "$DIR/builder_min_if_compare_varint_canary_vm.sh" || true
run "$DIR/builder_min_if_compare_varvar_canary_vm.sh" || true
run "$DIR/loop_scan_ne_else_break_canary_vm.sh" || true
run "$DIR/loop_scan_ne_else_continue_canary_vm.sh" || true
run "$DIR/selfhost_builder_first_return42_canary_vm.sh" || true
run "$DIR/hako_mainline_loop_undefined_block_vm.sh" || true

# Archived monitor-only probe:
# tools/smokes/v2/profiles/archive/core/phase2160/registry_optin_method_arraymap_direct_canary_vm.sh

echo "[phase2160] done"
