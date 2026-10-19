# APP-allocator-stress

Status: selected; original Array/Loop stop passed; scalar result source remains open.
Date: 2026-10-19
Scope: unchanged allocator-stress, physical EXE, exact output/exit and retained handle safety.
Related: [RULES](../../../RULES.md), [current state](../CURRENT_STATE.toml),
[existing retained-handle obligation, row 7](mirbuilder-exe-acceptance-suite-red-disposition-d21-2026-09-27.md),
[parked untyped-slot D0](mirbuilder-untyped-object-storage-d0-2026-09-25.md).

## Selection and measured stop

- User authorized skipping parked untyped-field. Allocator reuses passing mimalloc's heap;
  binary-trees/JSON also need unresolved untyped slots, JSON additional Text/Loop connections.
- Fixed suite at revision `751dae566b`: 6PASS/5FAIL, actual EXE5 + empty marker1,
  SKIP/timeout0; mimalloc green. No new whole-suite measurement or full-goal completion.
- Fresh direct EXE advances beyond `NamedArray(TextSourceMissing)` and stops before
  artifact generation at `ordinary-new/borrowed-result/source-not-i64`: main.hako:67
  summary `heap.requestedBytes()`, returning two declared child `requested_bytes` fields.
- Existing smoke separately stops in generic MIR emission at
  `borrowed-static/required-integer-source-unproved`; this is not the direct EXE route.
- Original demand: `apps/allocator-stress/main.hako:15`,
  `small_handles.push(heap.allocate(1 + (i * 4)))` in the first loop.

## Connected owners

- Same named-Array source issuer retains exact resolver-qualified Loop-body append sites.
  Original Get stays a direct parent-call argument; slot bounds/liveness stay runtime checks.
- Before Text contract issuance, the existing whole-holder source selects its exact handle
  append/candidate. No Text error is caught or relaxed; class/Home/ABI refusals remain downstream.
- Same append Home observation becomes a checked Loop action, Recipe unit-effect slot and
  verified schedule step. Receiver, candidate and prior Homes retain original identities.
- The existing raw and Loop ports share one append transaction and sole `emit_append` writer.
  Candidate CallNormal precedes transfer; cancellation precedes holder unwind on Fault.
- Same Loop child group tree owns append packets; the root does not duplicate conditional
  effects. Whole-holder final validation traverses that tree and keeps site cardinality.
- Independent graph checks bind step entry to candidate acquisition and Append, plus the
  actual Normal continuation. Both manual and whole-program verification retain their scopes.
- Same source proof lends required Phi tuples to always-on draft repair and finishing
  liveness. Strict correspondence is retained; no missing producer is silently accepted.
- Independent lifetime verification keeps exact-State joins/all terminal End checks. Only
  resource-stable natural backedges with a conditional normal exit are eligible; Fault,
  pending acquisitions, empty-holder and residual cycles retain refusal. Cached CFG and
  actual terminator edges must agree before backedge certification. No termination claim.
- No app rewrite, new test file/hook/checker/registry, or runtime/C ABI change.

## Verification and next action

- `cargo quick-test-fast --no-run --message-format=json` PASS.
- Existing whole-compiler/writer test1, named-array family9, source-loop13 and lifetime5 PASS.
  One existing full-compiler input was replaced by two iterations; the manual writer inputs
  remain straight-line. No additional test executions/cases in its four-variant loop.
- `callable_source_loop_exit_read_binds_after_phi_dst` remains red at Text source Loop
  unavailable; the same test fails identically with the preserved pre-change root binary.
  This is an open source boundary, not accepted product success.
- Test-name list9263 unchanged; builder layer/CallTarget guards and diff check PASS.
- Final quick CLI after CFG consistency hardening compiled; direct EXE and existing
  smoke both reproduce the stops above, exit1 before artifacts. Existing lifecycle
  runtime/C API reused; no runtime/C ABI change.
- Next: inspect requested_bytes writer `requested_size`: original
  Return leaf uses old Integer agreement, while Loop actual arithmetic has source SUM.
  Reuse that finite signed source closure; do not infer I64 from usize or widen transport.
  This diagnosis still needs direct validation; no app PASS from focused tests.
- Existing smoke: `bash tools/smokes/v2/profiles/integration/apps/allocator_stress_exe.sh`.
- Preserve expected accounting including rejects=4 and exit0. Retire only replaced exclusive
  edges after caller/validation closure. Untyped D0 stays ParkedSealed until accepted.
- Prior D2 selection history remains in this path's Git history; no new app card.
