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
- H15.7 retired the exact leaf/line C dispatch bridge and backend env guard; all active line exact runs enter through `array_text_state_residence_route`.
- H15.8 renamed the remaining backend surface to `hako_llvmc_ffi_indexof_text_state_residence.inc`.
- H15.9 retired exported `indexof_search_micro_seed_route`; the residence route is the only backend metadata owner.

Current owners:
- MIR side: `array_text_state_residence_route` top-level is generic contract only; exact proof/action/literals live under `temporary_indexof_seed_payload`.
- MIR JSON no longer exports `indexof_search_micro_seed_route`.
- Backend side: active observer lowering consumes `array_text_observer_routes`; it does not call raw window/liveness analyzers or exact leaf/line dispatch wrappers.
- Backend remaining surface: `hako_llvmc_ffi_indexof_text_state_residence.inc` is a text-state residence temporary payload reader/emitter, not an exact route dispatcher.
- Read-side owner: `array_text_observer_routes`.

Fixed order:
1. H15.4 make `array_text_state_residence_route` a real MIR-owned metadata field. Closed.
2. H15.5 split exact bridge proof fields from generic residence contract fields. Closed.
3. H15.6 audit `.inc` consumers for raw `indexOf` window/liveness rediscovery. Closed.
4. H15.7 retire the exact search dispatch bridge only after exact and compatibility-skip keeper gates stay green. Closed.
5. H15.8 rename/quarantine the remaining text-state residence temporary emitter. Closed.
6. H15.9 lift the residence payload source away from exported `indexof_search_micro_seed_route`. Closed.
7. H15 closeout: keep `temporary_indexof_seed_payload` as the explicit, fixture-backed payload until a generic residence emitter replaces it. Next.

Acceptance:
- `cargo test indexof_search_micro_seed --lib`
- `cargo test array_text_observer --lib`
- `cargo test array_text_state_residence --lib`
- `bash tools/perf/build_perf_release.sh`
- route trace shows `indexof_line_text_state_residence`
- MIR JSON does not export `indexof_search_micro_seed_route`
- exact and retired-flag `kilo_micro_indexof_line` stay keeper-fast
- `tools/checks/current_state_pointer_guard.sh`

Decision notes:
- `array_text_observer_routes` remains the read-side owner.
- `hako_llvmc_ffi_indexof_text_state_residence.inc` stays quarantined until the temporary payload/emitter is replaced by a non-exact MIR payload.
- H15.4 keeps the exact proof/action payload reuse intentionally small; H15.5 is the structural split that removes exact bridge vocabulary from the generic residence contract.
- H15.9 keeps `temporary_indexof_seed_payload` explicit because the current emitter is still temporary; do not promote that payload to generic MIR truth.

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
- H15.7 result: removed `hako_llvmc_match_indexof_leaf_ascii_seed(...)`, `hako_llvmc_match_indexof_line_ascii_seed(...)`, their shared exact dispatch helper, and the backend env reader for `NYASH_LLVM_SKIP_INDEXOF_LINE_SEED`.
- H15.7 result: `--skip-indexof-line-seed` remains only as a tool compatibility flag; it no longer exports a backend env or changes compiler route selection.
- H15.7 trace: exact and retired-flag runs both emit `stage=indexof_line_text_state_residence reason=text_state_residence`.
- H15.7 perf: exact `kilo_micro_indexof_line = C 5 ms / Ny AOT 3 ms`; retired-env probe `kilo_micro_indexof_line = C 4 ms / Ny AOT 4 ms`.
- H15.7 checks: `cargo test indexof_search_micro_seed --lib`, `cargo test array_text_observer --lib`, `cargo test array_text_state_residence --lib`, `bash tools/perf/build_perf_release.sh`, `tools/checks/current_state_pointer_guard.sh`, `tools/checks/dev_gate.sh quick`, and `git diff --check` passed.
- H15.8 result: `hako_llvmc_ffi_string_search_seed.inc` is renamed to `hako_llvmc_ffi_indexof_text_state_residence.inc`; emitter/temp symbol names now use residence wording, while `temporary_indexof_seed_payload` remains the explicit MIR quarantine key.
- H15.8 trace: post-rename route trace still emits `stage=indexof_line_text_state_residence reason=text_state_residence`.
- H15.8 perf: `kilo_micro_indexof_line = C 5 ms / Ny AOT 4 ms`.
- H15.8 checks: `bash tools/perf/build_perf_release.sh`, `cargo test array_text_state_residence --lib`, `tools/checks/current_state_pointer_guard.sh`, `tools/checks/dev_gate.sh quick`, and `git diff --check` passed.
- H15.9 result: removed `FunctionMetadata.indexof_search_micro_seed_route`, its semantic refresh/export path, and the standalone MIR JSON key.
- H15.9 result: `array_text_state_residence_route` builds its temporary payload from an internal matcher and remains the only backend-consumable metadata route for this path.
- H15.9 trace: MIR JSON has no `indexof_search_micro_seed_route`; `array_text_state_residence_route` still exports `temporary_indexof_seed_payload`; route trace emits `stage=indexof_line_text_state_residence reason=text_state_residence`.
- H15.9 perf: `kilo_micro_indexof_line = C 4 ms / Ny AOT 3 ms`.
- H15.9 checks: `cargo test indexof_search_micro_seed --lib`, `cargo test array_text_state_residence --lib`, `bash tools/perf/build_perf_release.sh`, `tools/checks/current_state_pointer_guard.sh`, `tools/checks/dev_gate.sh quick`, and `git diff --check` passed.
