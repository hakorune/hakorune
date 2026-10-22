# APP-json-stream-aggregator
Status: selected app; source-bound Dynamic expression emission builds, then fresh MIR emission stops at prior-Home cleanup before EXE generation. Date: 2026-10-23
Related: [RULES](../../../RULES.md), [current state](../CURRENT_STATE.toml), [Dynamic invocation contract](../../../../reference/language/dynamic-invocation.md), [pipeline](../design/mirbuilder-final-pipeline-ssot.md).

## Current stop
- Fresh explicit `target/quick` smoke now reaches `ordinary-new/local-commit/lexical-call-prior-home-unavailable` in `StringHelpers.split_lines`; MIR emission fails and no EXE is produced.
- The exact prior Home is the local `arr = new ArrayBox()`. The selected owned-array lifecycle is currently anchored by an `ArrayGet`-derived element recipe, so this push-only holder has no `LocalCommitV1::OwnedArray` row for the required Fault unwind. Keep the Home and connect its construction/append source to the existing Array local owner; do not skip its unwind.
- The next Array path needs a semantic choice: whether `ArrayBox.push` owns a `SelfContainedDynamicCarrier`. The accepted named Array append contract is typed-object-only and does not grant generic carrier storage; decide before wiring slot transfer or End.
- The exact Dynamic initializer frame now carries its `DynamicCarrier` through the existing Local projection receipt without copying. This is an intermediate handoff; carrier cleanup and the source-bound call issuer are still missing.
- A distinct opaque carrier MIR type, ordered Dynamic invocation operation, and physical JSON representation compile. C V4 now validates and emits the checked invocation, integer projection, and consuming End operations with Normal/Fault carrier state; the source-bound caller and emitted local publication/read are still absent.
- The unchanged helper also needs `i < n` where `n` is Dynamic. The operator contract previously admitted only `Less(Dynamic, I64)`; this app requires `Less(I64, Dynamic)`. The same-card decision is: compare the exact signed i64 with a Dynamic Integer, TypeError for another well-formed Dynamic kind, borrow both inputs, publish TrivialBool, and create no carrier lifecycle. The source order stays unchanged.
- The checked runtime C entry now invokes the immutable provider table and issues/ends an opaque carrier, but no selected compiler physical caller, source publication, or cleanup path is connected.
- Provider selection uses exact Rust runtime type identity, source selector, and arity. Keep the source tag; never infer String from `length`, caller behavior, or MIR type.
- The existing `CheckedCallOut`/Dynamic V2 route is TextScan-specific and cannot represent the full trivial/owner-bearing/weak carrier contract. The existing bounded Dynamic Loop CallSlot is not a generic invocation owner.
- The accepted contract is `OpaqueObservable`, synchronous/non-detached, `Normal(SelfContainedDynamicCarrier) | Fault`, borrowed-no-escape inputs, and exactly-once forwarding or End.

## Verification
- `cargo quick-build-fast`: PASS after the runtime entry change.
- `cargo test --profile quick -p nyash_kernel --lib checked_dynamic_invocation_publishes_and_ends_one_opaque_carrier --jobs 4`: 1/1 PASS; checks Normal publication/End, Fault leaves output unchanged, and later End Fault is suppressed behind the primary Fault.
- The new header passes Clang C11 and C++17 syntax checks; both exported symbol names are present in the linked focused-test executable.
- `cargo quick-build-fast --jobs 4`: PASS after admitting `Less(I64, Dynamic)` in the operator issuer and source condition classifier. The focused `cargo quick-test-fast dynamic_less_and_mixed_string_comparisons_keep_complete_contracts --jobs 4` did not run its test: root lib-test compilation stopped at 68 existing errors in unrelated modules.
- `cargo quick-build-fast --jobs 4`: PASS on 2026-10-23 after adding the opaque carrier MIR representation and lifecycle operation shapes; source lowering and C consumption remain pending.
- `bash tools/build_hako_llvmc_ffi.sh`: PASS after connecting C V4 Dynamic invocation/carrier emission. Existing `published_lifecycle_v4_driver` accepted a temporary physical Dynamic invoke → integer read → End program and produced an LLVM-compiled object; no repository fixture or test was added, and the generated function was not runtime-executed.
- `cargo quick-build-fast --jobs 4`: PASS after connecting the exact Dynamic initializer result to the existing Local projection receipt; no new test or hook was added.
- Fresh `json_stream_aggregator_exe_runtime_boundary.sh` with `NYASH_BIN=target/quick/hakorune`: FAIL at `ordinary-new/emission/expression-call/disposition-missing`; no EXE parity result.
- After connecting the source-bound Dynamic expression path, `cargo quick-build-fast --jobs 4`: PASS; the same focused smoke now fails earlier at `ordinary-new/local-commit/lexical-call-prior-home-unavailable` for the push-only ArrayBox Home. No EXE is produced.
- Last full suite is historical: 7/11 PASS (6 EXE results plus one marker). The root `cargo quick-test-fast` target still has 68 unrelated compile errors; this does not affect the focused `nyash_kernel` test result.

## Next work
- Connect push-only ArrayBox construction and append source to the existing `OwnedArrayLocalProgress` and root-cleanup End owner, preserving the live Home on the call's Fault path.
- Finish the source-bound Dynamic CallSlot from the exact receiver/argument evaluation frame, then publish/read its opaque carrier in the existing local lifecycle; `StringInvokeOperand::Dynamic` remains tagged `(tag,payload)` input. Add path-complete carrier cleanup before treating the handoff as complete.
- Connect carrier-aware i64/Dynamic, Dynamic/String comparison, and String + Dynamic consumers; preserve ordered cleanup and exactly-once forwarding/End on Normal and Fault.
- Re-run the same explicit EXE smoke only after the full source-to-physical path is connected. Do not widen the initializer gate, rewrite `.hako`, add an app/name branch, or use fallback/retry.

## Remaining app obligations
- Continue String, Map, ArrayBox, Birth, and loop work only as the focused smoke exposes it; preserve their canonical owners and lifecycle checks.
- App completion requires the existing smoke to generate and execute the EXE with expected output/exit. Fixed-suite status and full MirBuilder completion remain separate gates.
