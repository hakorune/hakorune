#!/bin/bash
# phase29cc_wsm_p5_route_trace_common.sh
# Shared route-trace contract runners for WSM-P5 min7/min8/min9 smoke scripts.

set -euo pipefail

run_wsm_p5_route_trace_contract_tests_full() {
  cargo test --features wasm-backend wasm_route_policy_name_contract -- --nocapture
  cargo test --features wasm-backend wasm_hako_default_lane_trace_ -- --nocapture
  cargo test --features wasm-backend wasm_demo_route_trace_reports_ -- --nocapture
}

run_wsm_p5_route_trace_contract_tests_readiness() {
  cargo test --features wasm-backend wasm_route_policy_name_contract -- --nocapture
  cargo test --features wasm-backend wasm_demo_route_trace_reports_shape_id_for_native_default_contract -- --nocapture
  cargo test --features wasm-backend wasm_demo_route_trace_reports_bridge_and_legacy_retired_contract -- --nocapture
}

run_wsm_p5_legacy_retire_execution_contract_tests() {
  cargo test --features wasm-backend wasm_route_policy_name_contract -- --nocapture
  cargo test --features wasm-backend wasm_demo_min_fixture_legacy_route_retired_failfast_contract -- --nocapture
  cargo test --features wasm-backend wasm_demo_route_trace_reports_bridge_and_legacy_retired_contract -- --nocapture
}
