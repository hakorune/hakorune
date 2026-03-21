#!/usr/bin/env bash
set -euo pipefail

# dev_gate.sh
# Purpose: single-entry developer gate with tiered profiles.
#
# Usage:
#   tools/checks/dev_gate.sh [quick|hotpath|plugin-module-core8-light|plugin-module-core8|runtime-exec-zero|wasm-boundary-lite|wasm-demo-g2|wasm-demo-g3-core|wasm-demo-g3-full|wasm-demo-g3|wasm-freeze-core|wasm-freeze-parity|portability|milestone|milestone-runtime|milestone-perf]
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
  tools/checks/dev_gate.sh [quick|hotpath|plugin-module-core8-light|plugin-module-core8|runtime-exec-zero|wasm-boundary-lite|wasm-demo-g2|wasm-demo-g3-core|wasm-demo-g3-full|wasm-demo-g3|wasm-freeze-core|wasm-freeze-parity|portability|milestone|milestone-runtime|milestone-perf]
  tools/checks/dev_gate.sh --list
USAGE
}

list_profiles() {
  cat <<'LIST'
[dev-gate] profiles:
  quick:
    - tools/checks/route_no_fallback_guard.sh
    - cargo check --bin hakorune
    - PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_strlen_fast.py
    - phase21_5_perf_chip8_kernel_crosslang_contract_vm.sh
  hotpath:
    - quick
    - tools/perf/run_phase21_5_perf_gate_bundle.sh hotpath
  plugin-module-core8-light:
    - cargo check --bin hakorune
    - phase29cc_plg_hm1_contract_tests_vm.sh
  plugin-module-core8:
    - plugin-module-core8-light
    - tools/checks/phase29cc_plg_hm2_min2_core6_wave2_ceiling_guard.sh (HM2 fixed order: min2 -> min3)
    - tools/checks/phase29cc_plg_hm2_min3_route_policy_matrix_guard.sh (HM2 fixed order: min2 -> min3)
    - tools/checks/phase29cc_plg07_filebox_binary_retire_execution_guard.sh
    - tools/vm_plugin_smoke.sh
  runtime-exec-zero:
    - plugin-module-core8-light
    - cargo test -p nyash_kernel plugin::wiring_tests::b3_public_wiring_contract_compiles -- --nocapture
    - tools/checks/phase29cc_kernel_b3_compat_isolation_guard.sh
    - tools/checks/phase29cc_hako_forward_registry_guard.sh
    - tools/checks/phase29cc_runtime_execution_path_zero_guard.sh
    - tools/checks/phase29cc_hostfacade_direct_call_guard.sh
    - tools/checks/phase29cc_runtime_vm_aot_route_lock_guard.sh
    - tools/checks/phase29cc_runtime_v0_abi_slice_guard.sh
    - phase29cc_runtime_v0_adapter_fixtures_vm.sh
    - tools/checks/phase29cc_plg_hm2_min3_route_policy_matrix_guard.sh
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
    - phase29cc_wsm/g2_browser/phase29cc_wsm_g2_min1_bridge_build_vm.sh
    - phase29cc_wsm/g2_browser/phase29cc_wsm_g2_browser_run_vm.sh
    - phase29cc_wsm/g4/phase29cc_wsm_g4_min1_playground_console_baseline_vm.sh
    - phase29cc_wsm/g4/phase29cc_wsm_g4_min2_playground_canvas_primer_vm.sh
    - phase29cc_wsm/g4/phase29cc_wsm_g4_min9_webcanvas_wasmbox_repromotion_vm.sh
    - phase29cc_wsm/g4/phase29cc_wsm_g4_min10_canvas_advanced_wasmbox_repromotion_vm.sh
    - phase29cc_wsm/g4/phase29cc_wsm_g4_min5_headless_two_examples_vm.sh
    - phase29cc_wsm/g4/phase29cc_wsm_g4_min7_webdisplay_fixture_parity_vm.sh
    - phase29cc_wsm/g4/phase29cc_wsm_g4_min8_global_call_probe_vm.sh
    - phase29cc_wsm/g4/phase29cc_wsm_g4_min6_gate_promotion_closeout_vm.sh
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
  wasm-freeze-core:
    - cargo check --features wasm-backend --bin hakorune
    - phase29cc_wsm_freeze_min1_route_policy_rust_native_env_vm.sh
    - phase29cc_wsm_freeze_min2_route_trace_always_on_vm.sh
    - phase29cc_wsm_freeze_min3_route_policy_scope_emit_wat_vm.sh
    - phase29cc_wsm/g4/phase29cc_wsm_g4_min9_webcanvas_wasmbox_repromotion_vm.sh
    - phase29cc_wsm/g4/phase29cc_wsm_g4_min10_canvas_advanced_wasmbox_repromotion_vm.sh
    - phase29cc_wsm_p5_min10_legacy_hard_remove_lock_vm.sh
    - phase29cc_wsm_p6_min1_route_policy_default_noop_lock_vm.sh
  wasm-freeze-parity:
    - wasm-freeze-core
    - cargo test --features wasm-backend wasm_demo_route_trace_reports_rust_native_forced_contract -- --nocapture
  portability:
    - tools/checks/windows_wsl_cmd_smoke.sh (preflight by default)
    - tools/checks/macos_portability_guard.sh
    - tools/checks/phase29cc_plg_hm2_rust_recovery_line_guard.sh
    - tools/checks/phase29cc_plg_hm2_min3_route_policy_matrix_guard.sh
    - tools/checks/phase29cc_runtime_vm_aot_route_lock_guard.sh
    - tools/checks/phase29cc_plg07_filebox_binary_retire_execution_guard.sh
    - tools/checks/phase29cc_wsm_p7_default_hako_only_guard.sh
    - tools/checks/phase29cc_wsm_p8_bridge_retire_readiness_guard.sh
    - tools/checks/phase29cc_wsm_p9_non_native_inventory_guard.sh
    - tools/checks/phase29cc_wsm_p9_bridge_retire_refresh_guard.sh
    - tools/checks/phase29cc_wsm_p10_loop_extern_native_emit_design_guard.sh
    - tools/checks/phase29cc_wsm_p10_loop_extern_matcher_inventory_guard.sh
    - tools/checks/phase29cc_wsm_p10_loop_extern_writer_section_guard.sh
    - tools/checks/phase29cc_wsm_p10_single_fixture_native_promotion_guard.sh
    - tools/checks/phase29cc_wsm_p10_expansion_inventory_guard.sh
    - tools/checks/phase29cc_wsm_p10_warn_native_promotion_guard.sh
    - tools/checks/phase29cc_wsm_p10_info_native_promotion_guard.sh
    - tools/checks/phase29cc_wsm_p10_error_native_promotion_guard.sh
    - tools/checks/phase29cc_wsm_p10_debug_native_promotion_guard.sh
    - tools/checks/phase29cc_wsm_p10_native_promotion_closeout_guard.sh
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
  run_step "route no-fallback guard" \
    bash tools/checks/route_no_fallback_guard.sh

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

run_plugin_module_core8_light() {
  run_step "cargo check" \
    cargo check --bin hakorune
  run_step "PLG-HM1 consolidated contract lock (min1..min4)" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_plg_hm1_contract_tests_vm.sh
}

run_plugin_module_core8() {
  run_plugin_module_core8_light
  # HM2 closeout order is fixed: min2 route ceiling first, then min3 policy matrix.
  run_step "PLG-HM2 min2 core6/wave2 ceiling guard" \
    bash tools/checks/phase29cc_plg_hm2_min2_core6_wave2_ceiling_guard.sh
  run_step "PLG-HM2 min3 route policy matrix guard" \
    bash tools/checks/phase29cc_plg_hm2_min3_route_policy_matrix_guard.sh
  run_step "PLG-07 retire execution guard" \
    bash tools/checks/phase29cc_plg07_filebox_binary_retire_execution_guard.sh
  run_step "vm plugin smoke manifest" \
    bash tools/vm_plugin_smoke.sh
}

run_runtime_exec_zero() {
  run_plugin_module_core8_light
  run_step "kernel B3 public wiring contract lock" \
    cargo test -p nyash_kernel plugin::wiring_tests::b3_public_wiring_contract_compiles -- --nocapture
  run_step "kernel B3 compat isolation guard" \
    bash tools/checks/phase29cc_kernel_b3_compat_isolation_guard.sh
  run_step "hako forward C-registry guard" \
    bash tools/checks/phase29cc_hako_forward_registry_guard.sh
  run_step "runtime execution-path-zero observability guard" \
    bash tools/checks/phase29cc_runtime_execution_path_zero_guard.sh
  run_step "hostfacade direct-call allowlist guard" \
    bash tools/checks/phase29cc_hostfacade_direct_call_guard.sh
  run_step "runtime VM+AOT route lock guard (kilo contracts)" \
    bash tools/checks/phase29cc_runtime_vm_aot_route_lock_guard.sh
  run_step "runtime V0 ABI slice lock guard" \
    bash tools/checks/phase29cc_runtime_v0_abi_slice_guard.sh
  run_step "runtime V0 adapter fixture smoke" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_runtime_v0_adapter_fixtures_vm.sh
  run_step "PLG-HM2 min3 route policy matrix guard" \
    bash tools/checks/phase29cc_plg_hm2_min3_route_policy_matrix_guard.sh
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
    bash tools/smokes/v2/profiles/integration/phase29cc_wsm/p5/phase29cc_wsm_p5_min1_default_cutover_docs_lock_vm.sh
  run_step "wasm p5 route policy lock" \
    bash tools/smokes/v2/profiles/integration/phase29cc_wsm/p5/phase29cc_wsm_p5_min2_route_policy_lock_vm.sh
  run_step "wasm p5 default hako-lane lock" \
    bash tools/smokes/v2/profiles/integration/phase29cc_wsm/p5/phase29cc_wsm_p5_min3_default_hako_lane_lock_vm.sh
  run_step "wasm p5 hako-lane bridge shrink lock" \
    bash tools/smokes/v2/profiles/integration/phase29cc_wsm/p5/phase29cc_wsm_p5_min4_hako_lane_bridge_shrink_vm.sh
  run_step "wasm p5 native helper lock" \
    bash tools/smokes/v2/profiles/integration/phase29cc_wsm/p5/phase29cc_wsm_p5_min5_native_helper_lock_vm.sh
  run_step "wasm p5 shape expand lock" \
    bash tools/smokes/v2/profiles/integration/phase29cc_wsm/p5/phase29cc_wsm_p5_min6_shape_expand_lock_vm.sh
  run_step "wasm p5 shape trace lock" \
    bash tools/smokes/v2/profiles/integration/phase29cc_wsm/p5/phase29cc_wsm_p5_min7_shape_trace_lock_vm.sh
  run_step "wasm p5 legacy hard-remove lock" \
    bash tools/smokes/v2/profiles/integration/phase29cc_wsm/p5/phase29cc_wsm_p5_min10_legacy_hard_remove_lock_vm.sh
  run_step "wasm p6 route policy default-only no-op lock" \
    bash tools/smokes/v2/profiles/integration/phase29cc_wsm/p6/phase29cc_wsm_p6_min1_route_policy_default_noop_lock_vm.sh
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
    bash tools/smokes/v2/profiles/integration/phase29cc_wsm/g2_browser/phase29cc_wsm_g2_min1_bridge_build_vm.sh
  run_step "wasm g2 min2 headless run baseline" \
    bash tools/smokes/v2/profiles/integration/phase29cc_wsm/g2_browser/phase29cc_wsm_g2_browser_run_vm.sh
  run_step "wasm g4 min1 playground console baseline lock" \
    bash tools/smokes/v2/profiles/integration/phase29cc_wsm/g4/phase29cc_wsm_g4_min1_playground_console_baseline_vm.sh
  run_step "wasm g4 min2 playground canvas primer lock" \
    bash tools/smokes/v2/profiles/integration/phase29cc_wsm/g4/phase29cc_wsm_g4_min2_playground_canvas_primer_vm.sh
  run_step "wasm g4 min9 webcanvas WasmCanvasBox re-promotion lock" \
    bash tools/smokes/v2/profiles/integration/phase29cc_wsm/g4/phase29cc_wsm_g4_min9_webcanvas_wasmbox_repromotion_vm.sh
  run_step "wasm g4 min10 canvas_advanced WasmCanvasBox re-promotion lock" \
    bash tools/smokes/v2/profiles/integration/phase29cc_wsm/g4/phase29cc_wsm_g4_min10_canvas_advanced_wasmbox_repromotion_vm.sh
  run_step "wasm g4 min5 headless two-example parity lock" \
    bash tools/smokes/v2/profiles/integration/phase29cc_wsm/g4/phase29cc_wsm_g4_min5_headless_two_examples_vm.sh
  run_step "wasm g4 min7 webdisplay fixture parity lock" \
    bash tools/smokes/v2/profiles/integration/phase29cc_wsm/g4/phase29cc_wsm_g4_min7_webdisplay_fixture_parity_vm.sh
  run_step "wasm g4 min8 global call native box lock" \
    bash tools/smokes/v2/profiles/integration/phase29cc_wsm/g4/phase29cc_wsm_g4_min8_global_call_probe_vm.sh
  run_step "wasm g4 min6 gate promotion closeout lock" \
    bash tools/smokes/v2/profiles/integration/phase29cc_wsm/g4/phase29cc_wsm_g4_min6_gate_promotion_closeout_vm.sh
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

run_wasm_freeze_core() {
  run_step "cargo check (wasm-backend)" \
    cargo check --features wasm-backend --bin hakorune
  run_step "wasm freeze min1 route policy rust_native env lock" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_freeze_min1_route_policy_rust_native_env_vm.sh
  run_step "wasm freeze min2 route-trace always-on lock" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_freeze_min2_route_trace_always_on_vm.sh
  run_step "wasm freeze min3 rust_native compile-wasm-only scope lock" \
    bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_freeze_min3_route_policy_scope_emit_wat_vm.sh
  run_step "wasm g4 min9 webcanvas WasmCanvasBox re-promotion lock" \
    bash tools/smokes/v2/profiles/integration/phase29cc_wsm/g4/phase29cc_wsm_g4_min9_webcanvas_wasmbox_repromotion_vm.sh
  run_step "wasm g4 min10 canvas_advanced WasmCanvasBox re-promotion lock" \
    bash tools/smokes/v2/profiles/integration/phase29cc_wsm/g4/phase29cc_wsm_g4_min10_canvas_advanced_wasmbox_repromotion_vm.sh
  run_step "wasm p5 legacy hard-remove lock" \
    bash tools/smokes/v2/profiles/integration/phase29cc_wsm/p5/phase29cc_wsm_p5_min10_legacy_hard_remove_lock_vm.sh
  run_step "wasm p6 route policy freeze lock" \
    bash tools/smokes/v2/profiles/integration/phase29cc_wsm/p6/phase29cc_wsm_p6_min1_route_policy_default_noop_lock_vm.sh
}

run_wasm_freeze_parity() {
  run_wasm_freeze_core
  run_step "wasm rust_native parity route-trace contract" \
    cargo test --features wasm-backend wasm_demo_route_trace_reports_rust_native_forced_contract -- --nocapture
}

run_portability() {
  run_step "windows WSL/CMD smoke (preflight)" \
    bash tools/checks/windows_wsl_cmd_smoke.sh
  run_step "macOS portability guard" \
    bash tools/checks/macos_portability_guard.sh
  run_step "PLG-HM2 rust recovery line guard" \
    bash tools/checks/phase29cc_plg_hm2_rust_recovery_line_guard.sh
  run_step "PLG-HM2 min3 route policy matrix guard" \
    bash tools/checks/phase29cc_plg_hm2_min3_route_policy_matrix_guard.sh
  run_step "runtime execution-path-zero observability guard" \
    bash tools/checks/phase29cc_runtime_execution_path_zero_guard.sh
  run_step "runtime VM+AOT route lock guard (kilo contracts)" \
    bash tools/checks/phase29cc_runtime_vm_aot_route_lock_guard.sh
  run_step "runtime V0 ABI slice lock guard" \
    bash tools/checks/phase29cc_runtime_v0_abi_slice_guard.sh
  run_step "PLG-07 retire execution guard" \
    bash tools/checks/phase29cc_plg07_filebox_binary_retire_execution_guard.sh
  run_step "WSM-P7 default hako-only guard" \
    bash tools/checks/phase29cc_wsm_p7_default_hako_only_guard.sh
  run_step "WSM-P8 bridge retire readiness guard" \
    bash tools/checks/phase29cc_wsm_p8_bridge_retire_readiness_guard.sh
  run_step "WSM-P9 non-native inventory guard" \
    bash tools/checks/phase29cc_wsm_p9_non_native_inventory_guard.sh
  run_step "WSM-P9 bridge retire refresh guard" \
    bash tools/checks/phase29cc_wsm_p9_bridge_retire_refresh_guard.sh
  run_step "WSM-P10 loop/extern native emit design guard" \
    bash tools/checks/phase29cc_wsm_p10_loop_extern_native_emit_design_guard.sh
  run_step "WSM-P10 loop/extern matcher inventory guard" \
    bash tools/checks/phase29cc_wsm_p10_loop_extern_matcher_inventory_guard.sh
  run_step "WSM-P10 loop/extern writer section guard" \
    bash tools/checks/phase29cc_wsm_p10_loop_extern_writer_section_guard.sh
  run_step "WSM-P10 single fixture native promotion guard" \
    bash tools/checks/phase29cc_wsm_p10_single_fixture_native_promotion_guard.sh
  run_step "WSM-P10 expansion inventory guard" \
    bash tools/checks/phase29cc_wsm_p10_expansion_inventory_guard.sh
  run_step "WSM-P10 warn native promotion guard" \
    bash tools/checks/phase29cc_wsm_p10_warn_native_promotion_guard.sh
  run_step "WSM-P10 info native promotion guard" \
    bash tools/checks/phase29cc_wsm_p10_info_native_promotion_guard.sh
  run_step "WSM-P10 error native promotion guard" \
    bash tools/checks/phase29cc_wsm_p10_error_native_promotion_guard.sh
  run_step "WSM-P10 debug native promotion guard" \
    bash tools/checks/phase29cc_wsm_p10_debug_native_promotion_guard.sh
  run_step "WSM-P10 native promotion closeout guard" \
    bash tools/checks/phase29cc_wsm_p10_native_promotion_closeout_guard.sh
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
  plugin-module-core8-light)
    run_plugin_module_core8_light
    ;;
  plugin-module-core8)
    run_plugin_module_core8
    ;;
  runtime-exec-zero)
    run_runtime_exec_zero
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
  wasm-freeze-core)
    run_wasm_freeze_core
    ;;
  wasm-freeze-parity)
    run_wasm_freeze_parity
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
