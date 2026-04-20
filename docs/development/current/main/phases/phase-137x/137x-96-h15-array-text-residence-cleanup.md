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

Current owners:
- MIR side leak: `array_text_state_residence_route` still aliases the exact proof payload.
- Backend delete candidate: `hako_llvmc_ffi_string_search_seed.inc`.
- Read-side owner: `array_text_observer_routes`.

Fixed order:
1. H15.4 make `array_text_state_residence_route` a real MIR-owned metadata field.
2. H15.5 split exact bridge proof fields from generic residence contract fields.
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
