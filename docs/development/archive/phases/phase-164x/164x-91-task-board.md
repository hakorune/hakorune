# 164x-91: repo-wide fmt drift cleanup task board

## Board

- [x] `164xA` inventory lock
  - the worker-confirmed inventory is fixed in `164x-90`
- [x] `164xB` string / array group cleanup
  - `crates/nyash_kernel/src/exports/string_helpers/concat.rs`
  - `crates/nyash_kernel/src/plugin/array.rs`
  - `crates/nyash_kernel/src/tests/string.rs`
  - `src/boxes/array/mod.rs`
- [x] `164xC` MIR / runtime / grammar group cleanup
  - `src/mir/instruction/methods.rs`
  - `src/mir/phi_query.rs`
  - `src/mir/string_corridor.rs`
  - `src/mir/sum_placement.rs`
  - `src/runner/mir_json_emit/emitters/sum.rs`
  - `src/core/instance_v2.rs`
  - `src/grammar/generated.rs`
- [x] `164xD` backend test cleanup
  - `src/backend/wasm/codegen/tests.rs`
- [x] `164xE` verification
  - `cargo fmt --check`
- [x] `164xF` closeout
  - sync `CURRENT_TASK.md` and `15-Workstream-Map.md`

## Notes

- This board is formatting-only.
- Do not mix optimization changes from `phase-163x` into this task series.
