# APP-json-stream-aggregator

Status: selected after allocator-stress canonical EXE landing.
Date: 2026-10-19
Scope: unchanged JSON app, existing Loop/Text owners, canonical physical EXE.
Related: [RULES](../../../RULES.md), [current state](../CURRENT_STATE.toml),
[untyped-slot D0](mirbuilder-untyped-object-storage-d0-2026-09-25.md),
[final pipeline](../design/mirbuilder-final-pipeline-ssot.md).

## Current measurement

- Fixed `real-apps-exe-boundary`: 7PASS/4FAIL, actual EXE6 + empty marker1;
  mimalloc and allocator PASS, no SKIP/timeout, duration764.0s.
  Measured this Bool-equality landing tree using the quick CLI from `aef8645ec7`
  (Rust unchanged), explicitly selected lifecycle runtime and rebuilt production C FFI.
- Command: `tools/smokes/v2/run.sh --profile integration --owner-profile integration
  --suite real-apps-exe-boundary --timeout 1900`, build/run limits300s.
- JSON first stop: `[freeze:contract][source-loop/v2/source Loop has an unavailable path]`.
  MIR compilation fails before artifact generation; no link or runtime evidence.
  The aggregate error does not identify the original function/Loop; establish that first.
- Other reds remain explicit: untyped-field phase84 inference panic,
  binary-trees borrowed-result/source-not-i64, boxtorrent stored-child/receiver-unavailable.

## Next source connection

- Read-only worker recommends JSON over binary-trees: binary's `itemCheck` immediately
  reads untyped left/right/value; inferring their storage from birth writes would cross D0.
- JSON candidate sites are `JsonLine.find` line15 (Text comparison + Return), `intField`
  line49 (Text comparison + Break), and `ingest` line154 (Call + If rebind + substring).
  These are candidates, not proof of which loop failed; include using-library owners.
- Reuse `home_prefix_source_loop*` source header/control/expression inventory,
  `produce_source_loop_recipe_v2`, and existing execution/schedule and expression writers.
  Landed Text, Bool, BranchNormal and argument owners supply existing contracts.
- Determine original owner/site and the unavailable source path, then connect its SAME
  source authority through Facts/Recipe/Verify to the existing physical consumer.
  Do not add a parallel classifier, infer from MIR types, or rewrite the app.
- JSON's later untyped UserStats/aggregator slots and Map remain explicit dependencies.
  This selection does not accept dynamic-slot semantics or promise a D0-free whole-app PASS.

## Allocator landing and verification

- Unchanged `allocator_stress_exe.sh` passes alone and within the fixed suite:
  physical lifecycle V4 / LLVM C API, actual executable, no compat replay, exact output
  and exit0: requested_bytes=454, outstanding=12, rejects=4, summary=ok.
- Its final C refusal was Bool Eq for original `heap.release(small_handles.get(1)) == false`.
  Existing indexed admission now accepts only Bool/Bool Eq/Ne; the existing 0/1 carrier
  and Compare emitter are reused. Mixed Bool/I64 and Bool ordering still refuse.
- Existing C execution matrix adds only Bool Eq/Ne and three mixed/ordering refusals:
  this independent physical C boundary is not protected by Rust source checks.
  These cases and all preceding execution checks PASS; no new test function/file or hook.
- Full C matrix stops at existing tagged-add negative case13: expected refusal is absent
  with both the prior unchanged C library and the new one. Keep this red open and verify
  its source/physical contract; later artifact/failure tail checks were not reached.
- Matrix runtime selection now honors the same explicit lifecycle directory as the smoke.
  Default root archive lacks checked_residence_set; runtime semantics/ABI are unchanged.
- Earlier SSA/handoff/result-protocol, Array teardown and synthetic-callee reds remain open;
  their reproduction details and allocator owner connections remain in this path's Git history.
  Prior root test list9263 is unchanged; no Rust rebuild was required for the C-only landing.
- Worker reviewed the C admission/matrix change read-only, no Cargo; diff/pointer checks PASS.
  Production FFI is retained for later smokes. Owned debug/replay binaries were discarded;
  no production adapter, new checker, guard, refusal token or app workaround was introduced.

## Completion obligations

- Existing JSON smoke must satisfy every artifact/run/output/negative contract unchanged.
- Untyped-slot D0 stays ParkedSealed, implementation_permission=false until explicit acceptance.
- Retire replaced exclusive edges when caller/validation closure holds; retain shared owners.
- Fixed acceptance, final pipeline switching/retirement and the full goal remain incomplete.
