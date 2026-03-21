# Integration phase29cc_wsm p5

This subfamily holds the active WSM-P5 route-trace / default-lane progression.

## Active Cases

- `phase29cc_wsm_p5_min1_default_cutover_docs_lock_vm.sh`
- `phase29cc_wsm_p5_min2_route_policy_lock_vm.sh`
- `phase29cc_wsm_p5_min3_default_hako_lane_lock_vm.sh`
- `phase29cc_wsm_p5_min4_hako_lane_bridge_shrink_vm.sh`
- `phase29cc_wsm_p5_min5_native_helper_lock_vm.sh`
- `phase29cc_wsm_p5_min6_shape_expand_lock_vm.sh`
- `phase29cc_wsm_p5_min7_shape_trace_lock_vm.sh`
- `phase29cc_wsm_p5_min8_legacy_retire_readiness_lock_vm.sh`
- `phase29cc_wsm_p5_min9_legacy_retire_execution_lock_vm.sh`
- `phase29cc_wsm_p5_min10_legacy_hard_remove_lock_vm.sh`

## Migration Note

- `phase29cc_wsm_p5_route_trace_common.sh` lives here as the shared helper.
- Keep new WSM-P5 cases under this directory.
- The next subfamily to inspect is `p6/`.
