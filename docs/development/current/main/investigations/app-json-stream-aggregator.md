# APP-json-stream-aggregator
Status: selected app; Array carrier Runtime owner implemented, source-to-physical cleanup pending. Date: 2026-10-23
Related: [RULES](../../../RULES.md), [current state](../CURRENT_STATE.toml), [Dynamic invocation contract](../../../../reference/language/dynamic-invocation.md), [pipeline](../design/mirbuilder-final-pipeline-ssot.md).

## Current stop
- Fresh explicit `target/quick` smoke now reaches `ordinary-new/local-commit/lexical-call-prior-home-unavailable` in `StringHelpers.split_lines`; MIR emission fails and no EXE is produced.
- The exact prior Home is `arr = new ArrayBox()`. Its first Dynamic call needs a Fault unwind, and later pushes store `s.substring(...)` results from unannotated `s`.
- The resolver-proven Dynamic call contract returns `SelfContainedDynamicCarrier`; it cannot be narrowed to Text from the selector, runtime tag, caller, or MIR type.
- User decision accepted: Array push forwards an authorized opaque carrier into its slot once on successful commit. Pre-commit Fault retains the original responsibility; terminal Array End drains committed carriers in reverse install order. The [Array target](../../../../reference/language/ownership.md#named-array-opaque-carrier-destination-target) owns this law; compiler source-to-physical connection remains pending.
- The canonical callable-module path preflights every resolved function; `split_lines/1` remains in this app's imported compilation unit even though `main.hako` does not call it directly.
- The destination decision resolves this design stop. Connect its source, LocalCommit and physical runtime rather than treating the carrier as Text or an object Home. This append target grants no carrier duplication or borrowed Dynamic invocation result on get.
- Array storage now has a disjoint opaque-carrier profile. Successful append atomically changes the Dynamic token owner from caller to that exact Array residence; competing Array adoption and caller End are rejected. Array End detaches first, ends tokens outside storage locks, then retires the residence.
- Checked kernel claim/append/end entries and C ABI declarations are present. They are not yet selected by a MIR operation or emitted by the lifecycle C backend.
- Source-bound Dynamic expression emission now builds and validates the exact invoke/result pair. Local CallSlot publication/read and path-complete carrier End/forward remain incomplete.
- The existing `CheckedCallOut`/Dynamic V2 route is TextScan-specific and cannot represent the full trivial/owner-bearing/weak carrier contract. The existing bounded Dynamic Loop CallSlot is not a generic invocation owner.
- The accepted contract is `OpaqueObservable`, synchronous/non-detached, `Normal(SelfContainedDynamicCarrier) | Fault`, borrowed-no-escape inputs, and exactly-once forwarding or End.

## Verification
- `cargo quick-build-fast`: PASS after the runtime entry change.
- `cargo test --profile quick -p nyash_kernel --lib checked_dynamic_invocation_publishes_and_ends_one_opaque_carrier --jobs 4`: 1/1 PASS; checks Normal publication/End, Fault leaves output unchanged, and later End Fault is suppressed behind the primary Fault.
- The new header passes Clang C11 and C++17 syntax checks; both exported symbol names are present in the linked focused-test executable.
- `cargo quick-build-fast --jobs 4`: PASS after admitting `Less(I64, Dynamic)` in the operator issuer and source condition classifier. The focused `cargo quick-test-fast dynamic_less_and_mixed_string_comparisons_keep_complete_contracts --jobs 4` did not run its test: root lib-test compilation stopped at 68 existing errors in unrelated modules.
- `cargo quick-build-fast --jobs 4`: PASS on 2026-10-23 after adding the opaque carrier MIR representation and lifecycle operation shapes; later source and C connection results follow below.
- `cargo quick-build-fast --jobs 4`: PASS after adding the Array carrier storage and atomic owner transition.
- `cargo test --profile quick -p nyash_kernel --lib dynamic_carrier_array_wire_transfers_exactly_once_and_ends_by_owner --jobs 4`: 1/1 PASS; linked checked entries reject pre-claim append, competing Array adoption and caller End, then End both residences.
- Updated `nyrt_fault_v1.h` passes Clang C11 and C++17 syntax checks.
- `bash tools/build_hako_llvmc_ffi.sh`: PASS after connecting C V4 Dynamic invocation/carrier emission. Existing `published_lifecycle_v4_driver` accepted a temporary physical Dynamic invoke → integer read → End program and produced an LLVM-compiled object; no repository fixture or test was added, and the generated function was not runtime-executed.
- `cargo quick-build-fast --jobs 4`: PASS after connecting the exact Dynamic initializer result to the existing Local projection receipt; no new test or hook was added.
- Fresh `json_stream_aggregator_exe_runtime_boundary.sh` with `NYASH_BIN=target/quick/hakorune`: FAIL at `ordinary-new/emission/expression-call/disposition-missing`; no EXE parity result.
- After connecting the source-bound Dynamic expression path, `cargo quick-build-fast --jobs 4`: PASS; the same focused smoke now fails earlier at `ordinary-new/local-commit/lexical-call-prior-home-unavailable` for the push-only ArrayBox Home. No EXE is produced.
- Last full suite is historical: 7/11 PASS (6 EXE results plus one marker). The root `cargo quick-test-fast` target still has 68 unrelated compile errors; this does not affect the focused `nyash_kernel` test result.

## Next work
- Connect the accepted Array carrier destination from exact source coverage through LocalCommit, append commit, holder Return and lifecycle C emission. Retain complete source-use coverage, source order and every Normal/Fault obligation. Read/escape authority is a separate requirement if the source actually needs it.
- Complete Dynamic CallSlot publication/read and cleanup from the exact receiver/argument evaluation frame; retain tagged `(tag,payload)` input and exactly-once carrier disposition on Normal and Fault.
- Preserve the unchanged `split_lines` source, existing failure ordering, and all Array/text ownership contracts. No runtime-tag narrowing or helper rewrite.
- Re-run the same explicit EXE smoke only after the full source-to-physical path is connected. Do not widen the initializer gate, rewrite `.hako`, add an app/name branch, or use fallback/retry.

## Remaining app obligations
- Continue String, Map, ArrayBox, Birth, and loop work only as the focused smoke exposes it; preserve their canonical owners and lifecycle checks.
- App completion requires the existing smoke to generate and execute the EXE with expected output/exit. Fixed-suite status and full MirBuilder completion remain separate gates.
- [ADR-002](../../../adr/adr-002-explicit-ownership-and-compiler-migration.md) owns the queued explicit-ownership migration tasks. New move/copy syntax, static typing and wholesale End replacement are not selected by this app or added to its finite completion conditions.
