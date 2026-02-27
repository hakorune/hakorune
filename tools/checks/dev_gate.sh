#!/usr/bin/env bash
set -euo pipefail

# dev_gate.sh
# Purpose: single-entry developer gate with tiered profiles.
#
# Usage:
#   tools/checks/dev_gate.sh [quick|hotpath|wasm-boundary-lite|wasm-demo-g2|wasm-demo-g3-core|wasm-demo-g3-full|wasm-demo-g3|portability|milestone|milestone-runtime|milestone-perf]
#   tools/checks/dev_gate.sh --list
#
# Profiles:
#   quick     : day-to-day lightweight checks (default)
#   hotpath   : quick + phase21.5 perf hotpath contract bundle
#   wasm-boundary-lite : quick + wasm-backend compile + boundary fast-fail unit locks
#   portability : cross-platform maintenance guards (Windows WSL/CMD + macOS readiness)
#   milestone-runtime : hotpath + runtime/selfhost milestone smoke
#   milestone-perf    : hotpath + phase21.5 full perf milestone checks
#   milestone         : milestone-runtime + milestone-perf (backward compatible)

PROFILE="${1:-quick}"
ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"

usage() {
  cat <<'USAGE'
Usage:
  tools/checks/dev_gate.sh [quick|hotpath|wasm-boundary-lite|wasm-demo-g2|wasm-demo-g3-core|wasm-demo-g3-full|wasm-demo-g3|portability|milestone|milestone-runtime|milestone-perf]
  tools/checks/dev_gate.sh --list
USAGE
}

list_profiles() {
  cat <<'LIST'
[dev-gate] profiles:
  quick:
    - cargo check --bin hakorune
    - PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_strlen_fast.py
    - phase21_5_perf_chip8_kernel_crosslang_contract_vm.sh
  hotpath:
    - quick
    - tools/perf/run_phase21_5_perf_gate_bundle.sh hotpath
  wasm-boundary-lite:
    - quick
    - cargo check --features wasm-backend --bin hakorune
    - cargo test --features wasm-backend extern_contract_supported_name_maps_to_import -- --nocapture
    - cargo test --features wasm-backend test_unsupported_extern_call_fails_fast_with_supported_list -- --nocapture
    - cargo test --features wasm-backend test_unsupported_boxcall_method_fails_fast_with_supported_list -- --nocapture
    - phase29cc_wsm02d_demo_min_boundary_vm.sh
    - phase29cc_wsm02d_demo_unsupported_boundary_vm.sh
    - phase29cc_wsm_p1_emit_wat_cli_vm.sh
    - phase29cc_wsm_p1_parity_wat_vm.sh
    - phase29cc_wsm_p2_min1_bridge_lock_vm.sh
    - phase29cc_wsm_p3_min1_import_object_lock_vm.sh
    - phase29cc_wsm_p4_min1_docs_lock_vm.sh
    - phase29cc_wsm_p4_min2_binary_writer_lock_vm.sh
    - phase29cc_wsm_p4_min3_hako_writer_docs_lock_vm.sh
    - phase29cc_wsm_p4_min4_hako_writer_const_parity_vm.sh
    - phase29cc_wsm_p4_min5_hako_writer_neg_const_parity_vm.sh
    - phase29cc_wsm_p4_min6_shape_table_lock_vm.sh
    - phase29cc_wsm_p5_min1_default_cutover_docs_lock_vm.sh
    - phase29cc_wsm_p5_min2_route_policy_lock_vm.sh
    - phase29cc_wsm_p5_min3_default_hako_lane_lock_vm.sh
    - phase29cc_wsm_p5_min4_hako_lane_bridge_shrink_vm.sh
    - phase29cc_wsm_p5_min5_native_helper_lock_vm.sh
    - phase29cc_wsm_p5_min6_shape_expand_lock_vm.sh
    - phase29cc_wsm_p5_min7_shape_trace_lock_vm.sh
    - phase29cc_wsm_p5_min10_legacy_hard_remove_lock_vm.sh
    - phase29cc_wsm_p6_min1_route_policy_default_noop_lock_vm.sh
  wasm-demo-g2:
    - phase29cc_wsm_g2_min1_bridge_build_vm.sh
    - phase29cc_wsm_g2_browser_run_vm.sh
    - phase29cc_wsm_g4_min1_playground_console_baseline_vm.sh
    - phase29cc_wsm_g4_min2_playground_canvas_primer_vm.sh
    - phase29cc_wsm_g4_min3_webcanvas_fixture_parity_vm.sh
    - phase29cc_wsm_g4_min4_canvas_advanced_fixture_parity_vm.sh
    - phase29cc_wsm_g4_min5_headless_two_examples_vm.sh
    - phase29cc_wsm_g4_min6_gate_promotion_closeout_vm.sh
  wasm-demo-g3-core:
    - wasm-demo-g2
    - phase29cc_wsm_g3_canvas_clear_contract_vm.sh
    - phase29cc_wsm_g3_canvas_strokerect_contract_vm.sh
    - phase29cc_wsm_g3_canvas_beginpath_contract_vm.sh
  wasm-demo-g3-full:
    - wasm-demo-g3-core
    - phase29cc_wsm_g3_canvas_arc_contract_vm.sh
    - phase29cc_wsm_g3_canvas_fill_contract_vm.sh
    - phase29cc_wsm_g3_canvas_stroke_contract_vm.sh
    - phase29cc_wsm_g3_canvas_setfillstyle_contract_vm.sh
    - phase29cc_wsm_g3_canvas_setstrokestyle_contract_vm.sh
    - phase29cc_wsm_g3_canvas_setlinewidth_contract_vm.sh
    - phase29cc_wsm_g3_canvas_fillcircle_contract_vm.sh
    - phase29cc_wsm_g3_canvas_drawline_contract_vm.sh
  wasm-demo-g3:
    - wasm-demo-g3-full (backward compatible alias)
  portability:
    - tools/checks/windows_wsl_cmd_smoke.sh (preflight by default)
    - tools/checks/macos_portability_guard.sh
    - tools/checks/phase29cc_plg07_filebox_binary_retire_readiness_guard.sh
    - tools/checks/phase29cc_wsm_p7_default_hako_only_guard.sh
    - tools/checks/phase29cc_wsm_p8_bridge_retire_readiness_guard.sh
  milestone-runtime:
    - hotpath
    - phase29cc_wsm02d_milestone_gate_vm.sh
    - phase29y_no_compat_mainline_vm.sh
    - phase29x_derust_done_matrix_vm.sh
  milestone-perf:
    - hotpath
    - tools/perf/run_phase21_5_perf_gate_bundle.sh full
  milestone:
    - milestone-runtime
    - milestone-perf
LIST
}

run_step() {
  local label="$1"
  shift
  echo "[dev-gate] >>> ${label}"
  "$@"
}

run_quick() {
  run_step "cargo check" \
    cargo check --bin hakorune

  run_step "llvm_py unittest (strlen_fast)" \
    env PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_strlen_fast.py

  run_step "chip8 crosslang contract smoke" \
    env NYASH_LLVM_SKIP_BUILD="${NYASH_LLVM_SKIP_BUILD:-1}" \
      bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_chip8_kernel_crosslang_contract_vm.sh
}

run_hotpath() {
  run_quick
  run_step "phase21.5 perf gate bundle (hotpath)" \
    env NYASH_LLVM_SKIP_BUILD="${NYASH_LLVM_SKIP_BUILD:-1}" \
      tools/perf/run_phase21_5_perf_gate_bundle.sh hotpath
}

run_wasm_boundary_lite_p1_to_p3() {
  run_step "wasm p1 emit-wat CLI lock" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p1_emit_wat_cli_vm.sh
  run_step "wasm p1 fixture WAT parity lock" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p1_parity_wat_vm.sh
  run_step "wasm p2 wat2wasm bridge lock" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p2_min1_bridge_lock_vm.sh
  run_step "wasm p3 import object contract lock" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p3_min1_import_object_lock_vm.sh
}

run_wasm_boundary_lite_p4() {
  run_step "wasm p4 binary-writer docs lock" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p4_min1_docs_lock_vm.sh
  run_step "wasm p4 binary-writer skeleton lock" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p4_min2_binary_writer_lock_vm.sh
  run_step "wasm p4 .hako writer docs lock" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p4_min3_hako_writer_docs_lock_vm.sh
  run_step "wasm p4 .hako writer const parity lock" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p4_min4_hako_writer_const_parity_vm.sh
  run_step "wasm p4 .hako writer neg-const parity lock" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p4_min5_hako_writer_neg_const_parity_vm.sh
  run_step "wasm p4 shape table lock" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p4_min6_shape_table_lock_vm.sh
}

run_wasm_boundary_lite_p5() {
  run_step "wasm p5 default cutover docs lock" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p5_min1_default_cutover_docs_lock_vm.sh
  run_step "wasm p5 route policy lock" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p5_min2_route_policy_lock_vm.sh
  run_step "wasm p5 default hako-lane lock" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p5_min3_default_hako_lane_lock_vm.sh
  run_step "wasm p5 hako-lane bridge shrink lock" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p5_min4_hako_lane_bridge_shrink_vm.sh
  run_step "wasm p5 native helper lock" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p5_min5_native_helper_lock_vm.sh
  run_step "wasm p5 shape expand lock" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p5_min6_shape_expand_lock_vm.sh
    run_step "wasm p5 shape trace lock" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p5_min7_shape_trace_lock_vm.sh
    run_step "wasm p5 legacy hard-remove lock" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p5_min10_legacy_hard_remove_lock_vm.sh
    run_step "wasm p6 route policy default-only no-op lock" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p6_min1_route_policy_default_noop_lock_vm.sh
}

run_wasm_boundary_lite() {
  run_quick
  run_step "cargo check (wasm-backend)" \
    cargo check --features wasm-backend --bin hakorune
  run_step "wasm extern contract lock" \
    cargo test --features wasm-backend extern_contract_supported_name_maps_to_import -- --nocapture
  run_step "wasm extern unsupported boundary lock" \
    cargo test --features wasm-backend test_unsupported_extern_call_fails_fast_with_supported_list -- --nocapture
  run_step "wasm boxcall unsupported boundary lock" \
    cargo test --features wasm-backend test_unsupported_boxcall_method_fails_fast_with_supported_list -- --nocapture
  run_step "wasm demo-min fixture boundary lock" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm02d_demo_min_boundary_vm.sh
  run_step "wasm demo unsupported boundary lock" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm02d_demo_unsupported_boundary_vm.sh
  run_wasm_boundary_lite_p1_to_p3
  run_wasm_boundary_lite_p4
  run_wasm_boundary_lite_p5
}

run_wasm_demo_g2() {
  run_step "wasm g2 min1 bridge build baseline" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g2_min1_bridge_build_vm.sh
  run_step "wasm g2 min2 headless run baseline" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g2_browser_run_vm.sh
  run_step "wasm g4 min1 playground console baseline lock" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g4_min1_playground_console_baseline_vm.sh
  run_step "wasm g4 min2 playground canvas primer lock" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g4_min2_playground_canvas_primer_vm.sh
  run_step "wasm g4 min3 webcanvas fixture parity lock" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g4_min3_webcanvas_fixture_parity_vm.sh
  run_step "wasm g4 min4 canvas_advanced fixture parity lock" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g4_min4_canvas_advanced_fixture_parity_vm.sh
  run_step "wasm g4 min5 headless two-example parity lock" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g4_min5_headless_two_examples_vm.sh
  run_step "wasm g4 min6 gate promotion closeout lock" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g4_min6_gate_promotion_closeout_vm.sh
}

run_wasm_demo_g3_core() {
  run_wasm_demo_g2
  run_step "wasm g3 canvas.clear contract lock" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g3_canvas_clear_contract_vm.sh
  run_step "wasm g3 canvas.strokeRect contract lock" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g3_canvas_strokerect_contract_vm.sh
  run_step "wasm g3 canvas.beginPath contract lock" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g3_canvas_beginpath_contract_vm.sh
}

run_wasm_demo_g3_full() {
  run_wasm_demo_g3_core
  run_step "wasm g3 canvas.arc contract lock" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g3_canvas_arc_contract_vm.sh
  run_step "wasm g3 canvas.fill contract lock" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g3_canvas_fill_contract_vm.sh
  run_step "wasm g3 canvas.stroke contract lock" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g3_canvas_stroke_contract_vm.sh
  run_step "wasm g3 canvas.setFillStyle contract lock" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g3_canvas_setfillstyle_contract_vm.sh
  run_step "wasm g3 canvas.setStrokeStyle contract lock" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g3_canvas_setstrokestyle_contract_vm.sh
  run_step "wasm g3 canvas.setLineWidth contract lock" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g3_canvas_setlinewidth_contract_vm.sh
  run_step "wasm g3 canvas.fillCircle contract lock" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g3_canvas_fillcircle_contract_vm.sh
  run_step "wasm g3 canvas.drawLine contract lock" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g3_canvas_drawline_contract_vm.sh
}

run_portability() {
  run_step "windows WSL/CMD smoke (preflight)" \
    bash tools/checks/windows_wsl_cmd_smoke.sh
  run_step "macOS portability guard" \
    bash tools/checks/macos_portability_guard.sh
  run_step "PLG-07 retire readiness guard" \
    bash tools/checks/phase29cc_plg07_filebox_binary_retire_readiness_guard.sh
  run_step "WSM-P7 default hako-only guard" \
    bash tools/checks/phase29cc_wsm_p7_default_hako_only_guard.sh
  run_step "WSM-P8 bridge retire readiness guard" \
    bash tools/checks/phase29cc_wsm_p8_bridge_retire_readiness_guard.sh
}

run_milestone_runtime() {
  run_hotpath
  run_step "phase29cc wasm WSM-02d milestone gate" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm02d_milestone_gate_vm.sh
  run_step "phase29y no-compat mainline smoke" \
    bash tools/smokes/v2/profiles/integration/apps/phase29y_no_compat_mainline_vm.sh
  run_step "phase29x de-rust done matrix smoke" \
    bash tools/smokes/v2/profiles/integration/apps/phase29x_derust_done_matrix_vm.sh
}

run_milestone_perf() {
  run_hotpath
  run_step "phase21.5 perf gate bundle (full)" \
    env NYASH_LLVM_SKIP_BUILD="${NYASH_LLVM_SKIP_BUILD:-1}" \
      tools/perf/run_phase21_5_perf_gate_bundle.sh full
}

run_milestone() {
  run_milestone_runtime
  run_milestone_perf
}

cd "${ROOT_DIR}"

case "${PROFILE}" in
  -h|--help)
    usage
    exit 0
    ;;
  --list)
    list_profiles
    exit 0
    ;;
  quick)
    run_quick
    ;;
  hotpath)
    run_hotpath
    ;;
  wasm-boundary-lite)
    run_wasm_boundary_lite
    ;;
  wasm-demo-g2)
    run_wasm_demo_g2
    ;;
  wasm-demo-g3-core)
    run_wasm_demo_g3_core
    ;;
  wasm-demo-g3-full|wasm-demo-g3)
    run_wasm_demo_g3_full
    ;;
  portability)
    run_portability
    ;;
  milestone-runtime)
    run_milestone_runtime
    ;;
  milestone-perf)
    run_milestone_perf
    ;;
  milestone)
    run_milestone
    ;;
  *)
    usage >&2
    exit 2
    ;;
esac

echo "[dev-gate] profile=${PROFILE} ok"
