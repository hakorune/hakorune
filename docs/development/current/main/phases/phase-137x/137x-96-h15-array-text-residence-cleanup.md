# 137x-H15 Array/Text Residence Cleanup

Purpose:
- Keep the generic array/text residence lane owned by MIR.
- Keep the exact bridge quarantined until keeper-speed generic coverage is proven.
- Keep the current pointers short; put detailed cleanup history here.

Status:
- H15.1 and H15.2 are closed probes.
- H15.3a kept the line front keeper-fast by consuming `indexof_search_micro_seed_route` as text-state residence.
- H15.3b split the exact quarantine key from the generic residence key in MIR JSON.
- H15.3c cleaned backend-local naming away from the exact `micro_seed` vocabulary.
- H15.4 made `array_text_state_residence_route` a real `FunctionMetadata` field.

Current owners:
- MIR side next split: `array_text_state_residence_route` no longer aliases the exact JSON key, but still derives its payload from the exact search proof/action route.
- Backend delete candidate: `hako_llvmc_ffi_string_search_seed.inc`.
- Read-side owner: `array_text_observer_routes`.

Fixed order:
1. H15.4 make `array_text_state_residence_route` a real MIR-owned metadata field. Closed.
2. H15.5 split exact bridge proof fields from generic residence contract fields. Next.
3. H15.6 audit `.inc` consumers for raw `indexOf` window/liveness rediscovery.
4. H15.7 delete or fixture the exact search bridge only after exact and seed-off keeper gates stay green.

Acceptance:
- `cargo test indexof_search_micro_seed --lib`
- `cargo test array_text_observer --lib`
- `bash tools/perf/build_perf_release.sh`
- seed-off route trace shows `indexof_line_text_state_residence`
- exact and seed-off `kilo_micro_indexof_line` stay keeper-fast
- `tools/checks/current_state_pointer_guard.sh`

Decision notes:
- `array_text_observer_routes` remains the read-side owner.
- `hako_llvmc_ffi_string_search_seed.inc` stays the explicit delete candidate until the ledger gate is green.
- H15.4 keeps the exact proof/action payload reuse intentionally small; H15.5 is the structural split that removes exact bridge vocabulary from the generic residence contract.

Latest result:
- H15.4 adds `src/mir/array_text_state_residence_plan.rs` and stores the route in `FunctionMetadata.array_text_state_residence_route`.
- MIR JSON now emits `indexof_search_micro_seed_route` without residence fields and `array_text_state_residence_route` with `residence=loop_local_pointer_array`, `observer_kind=indexof`, and `result_repr=scalar_i64`.
- Seed-off route trace: `indexof_line_text_state_residence reason=text_state_residence`.
- Seed-off `kilo_micro_indexof_line`: `C 5 ms / Ny AOT 4 ms`.
- Exact `kilo_micro_indexof_line`: `C 4 ms / Ny AOT 3 ms`.
- Checks: targeted `rustfmt --check`, `git diff --check`, `cargo test indexof_search_micro_seed --lib`, `cargo test array_text_state_residence --lib`, `cargo test array_text_observer --lib`, `bash tools/perf/build_perf_release.sh`, `tools/checks/current_state_pointer_guard.sh`, and `tools/checks/dev_gate.sh quick` passed.
