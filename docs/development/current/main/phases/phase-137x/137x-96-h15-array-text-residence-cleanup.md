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
- H15.5 split the generic residence contract from the temporary exact emitter payload.
- H15.6 removed unused raw `.inc` `indexOf` window/liveness rediscovery analyzers from active compilation.

Current owners:
- MIR side: `array_text_state_residence_route` top-level is generic contract only; exact proof/action/literals live under `temporary_indexof_seed_payload`.
- Backend side: active observer lowering consumes `array_text_observer_routes`; it does not call raw window/liveness analyzers.
- Backend delete candidate: `hako_llvmc_ffi_string_search_seed.inc`.
- Read-side owner: `array_text_observer_routes`.

Fixed order:
1. H15.4 make `array_text_state_residence_route` a real MIR-owned metadata field. Closed.
2. H15.5 split exact bridge proof fields from generic residence contract fields. Closed.
3. H15.6 audit `.inc` consumers for raw `indexOf` window/liveness rediscovery. Closed.
4. H15.7 delete or fixture the exact search bridge only after exact and seed-off keeper gates stay green. Next.

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
- H15.5 keeps `temporary_indexof_seed_payload` explicit because the current emitter is still a temporary exact bridge; do not promote that payload to generic MIR truth.

Latest result:
- H15.4 adds `src/mir/array_text_state_residence_plan.rs` and stores the route in `FunctionMetadata.array_text_state_residence_route`.
- MIR JSON now emits `indexof_search_micro_seed_route` without residence fields and `array_text_state_residence_route` with `residence=loop_local_pointer_array`, `observer_kind=indexof`, and `result_repr=scalar_i64`.
- Seed-off route trace: `indexof_line_text_state_residence reason=text_state_residence`.
- Seed-off `kilo_micro_indexof_line`: `C 5 ms / Ny AOT 4 ms`.
- Exact `kilo_micro_indexof_line`: `C 4 ms / Ny AOT 3 ms`.
- Checks: targeted `rustfmt --check`, `git diff --check`, `cargo test indexof_search_micro_seed --lib`, `cargo test array_text_state_residence --lib`, `cargo test array_text_observer --lib`, `bash tools/perf/build_perf_release.sh`, `tools/checks/current_state_pointer_guard.sh`, and `tools/checks/dev_gate.sh quick` passed.
- H15.5 result: top-level `array_text_state_residence_route` keys are only `consumer_capability`, `observer_kind`, `publication_boundary`, `residence`, `result_repr`, and `temporary_indexof_seed_payload`.
- H15.5 result: `variant`, `proof`, `backend_action`, `result_use`, and candidate literals moved under `temporary_indexof_seed_payload`; `.inc` validates the top-level generic contract before reading the temporary payload.
- H15.5 perf: seed-off `kilo_micro_indexof_line = C 4 ms / Ny AOT 4 ms`; exact `kilo_micro_indexof_line = C 5 ms / Ny AOT 3 ms`.
- H15.6 result: removed unused raw observer analyzer/trace `.inc` files and trimmed `hako_llvmc_ffi_indexof_observer_state.inc` to metadata defer state only.
- H15.6 result: active pure lowering includes `hako_llvmc_ffi_indexof_observer_state.inc` and `hako_llvmc_ffi_indexof_observer_lowering.inc` only; remaining observer legality comes from MIR metadata.
- H15.6 verification: no references remain to `analyze_array_string_indexof_*`, `ArrayStringIndexof*WindowMatch`, or raw observer trace helpers.
- H15.6 perf: seed-off `kilo_micro_indexof_line = C 4 ms / Ny AOT 3 ms`; exact `kilo_micro_indexof_line = C 4 ms / Ny AOT 3 ms`.
