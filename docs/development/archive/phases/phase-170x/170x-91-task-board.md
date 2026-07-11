# 170x-91: direct-kernel substring plan proof task board

## Board

- [x] `170xA` docs lock
  - phase README
  - SSOT
  - root/current/workstream pointers
- [x] `170xB` proof payload widening
  - add `left_value` / `right_value` to concat-triplet proof
  - keep existing source/window/shared-source fields
- [x] `170xC` MIR JSON refresh
  - emit the widened proof payload
  - pin JSON test coverage
- [x] `170xD` boundary route shrink
  - read concat-triplet piece carriers from `direct_kernel_entry.plan.proof`
  - lower helper-result `substring()` through `substring_concat3_hhhii`
  - keep remembered concat-chain as fallback only
- [x] `170xE` proof/verify
  - add `string_direct_kernel_plan_substring_window_min_v1.mir.json`
  - add `phase137x_boundary_string_direct_kernel_plan_substring_min.sh`
  - rerun len proof, live direct-emits, exact asm/perf, and `tools/checks/dev_gate.sh quick`

## Notes

- this is a bridge-shrink cut, not a new string sink family
- remaining concat backlog stays `return` / `store` / host-boundary publication
- if a future cut needs generic lifecycle/boundary extraction, reopen that as a separate contract phase
