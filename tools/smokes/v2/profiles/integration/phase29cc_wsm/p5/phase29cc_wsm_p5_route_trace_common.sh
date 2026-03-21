#!/bin/bash
# phase29cc_wsm_p5_route_trace_common.sh
# Shared route-trace contract runners for WSM-P5 min7/min8/min9 smoke scripts.

set -euo pipefail
_phase29cc_wsm_p5_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$_phase29cc_wsm_p5_dir/../../apps/phase29cc_wsm_cargo_test_common.sh"

run_wsm_p5_route_trace_contract_tests_full() {
  run_wsm_targeted_contract_test "wasm_route_policy_name_contract"
  run_wsm_targeted_contract_test "wasm_hako_default_lane_trace_"
  run_wsm_targeted_contract_test "wasm_demo_route_trace_reports_"
}

run_wsm_p5_route_trace_contract_tests_readiness() {
  run_wsm_targeted_contract_test "wasm_route_policy_name_contract"
  run_wsm_targeted_contract_test "wasm_demo_route_trace_reports_shape_id_for_native_default_contract"
  run_wsm_targeted_contract_test "wasm_demo_route_trace_reports_bridge_and_legacy_policy_rejected_contract"
}

run_wsm_p5_legacy_hard_remove_contract_tests() {
  run_wsm_targeted_contract_test "wasm_route_policy_name_contract"
  run_wsm_targeted_contract_test "wasm_demo_min_fixture_legacy_route_policy_rejected_contract"
  run_wsm_targeted_contract_test "wasm_demo_route_trace_reports_bridge_and_legacy_policy_rejected_contract"
}

run_wsm_p5_legacy_retire_execution_contract_tests() {
  run_wsm_p5_legacy_hard_remove_contract_tests
}

run_wsm_p6_route_policy_default_noop_contract_tests() {
  run_wsm_targeted_contract_test "wasm_route_policy_"
  run_wsm_targeted_contract_test "wasm_demo_min_fixture_route_policy_default_noop_contract"
}
