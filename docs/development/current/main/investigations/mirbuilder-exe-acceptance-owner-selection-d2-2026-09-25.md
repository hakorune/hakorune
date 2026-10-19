# APP-allocator-stress

Status: selected; Bool condition passed; following scalar rebind after an If join remains open.
Date: 2026-10-19
Scope: unchanged allocator-stress, physical EXE, exact output/exit and retained handle safety.
Related: [RULES](../../../RULES.md), [current state](../CURRENT_STATE.toml),
[existing retained-handle obligation, row 7](mirbuilder-exe-acceptance-suite-red-disposition-d21-2026-09-27.md),
[parked untyped-slot D0](mirbuilder-untyped-object-storage-d0-2026-09-25.md).

## Selection and measured stop

- User authorized skipping parked untyped-field. Allocator reuses passing mimalloc's heap;
  binary-trees/JSON also need unresolved untyped slots, JSON additional Text/Loop connections.
- Fixed suite at revision `751dae566b`: 6PASS/5FAIL, actual EXE5 + empty marker1,
  SKIP/timeout0; mimalloc green. This is the landed baseline, not current WIP acceptance.
- Fresh source-only EXE passed `requestedBytes()` at main.hako:67; computed-argument
  CLI then passed original-actual selection and arithmetic frame qualification.
- Latest matching quick CLI and selected canonical smoke pass first Loop source coverage.
  Bool equality also passes main.hako:28 `heap.release(small_handles.get(1)) == false`.
  Current canonical smoke stops before artifacts at `array-home/append-prefix`,
  `PrefixNotCovered` Body11/IfThen0: line29 `rejects = rejects + 1`, after the
  prior conditional increment. Exact source If-join scalar continuation remains open.
- Existing smoke now selects canonical physical lifecycle EXE directly, as mimalloc
  does, retaining exact output/rejects=4, route/toolchain/artifact checks and no replay.
  Its earlier generic JSON route stopped at required-integer-source-unproved.
- Original demand: `apps/allocator-stress/main.hako:15`,
  `small_handles.push(heap.allocate(1 + (i * 4)))` in the first loop.

## Connected owners

- Same named-Array source selects original holder/Get/Loop-body append sites before Text
  issuance. Text refusals remain intact; Get is a direct parent-call argument.
- Same append Home observation, checked Loop action, Recipe effect slot and child-group
  packet retain receiver/candidate identities. Root prefixes do not duplicate Loop effects.
- Raw and Loop ports share the sole append writer. Normal transfer/Fault cancellation and
  holder teardown remain checked. Graph validation binds entry, acquisition and continuation.
- Same source proof lends required Phi tuples to repair/liveness. Natural-backedge admission
  checks exact resource state and consistent cached/actual CFG, without a termination claim.
- Current arithmetic work retains the SAME finite signed source SUM for the stable original
  formal use. Declared usize and legacy transport agreements are unchanged.
- Object input now uses the existing argument-purpose source/frame protocol, with original
  membership checks on re-observation and reconstruction. No second expression evaluator.
- Same computed-actual packet selects ScalarCompleted/NormalInteger for I64; Text retains
  OwnedTextDeferred/OwnedString and its cleanup. Exact Call operand coverage stays mandatory.
- Wire tag1 requires the same finalized computed descriptor/value and callee's signed-only
  complete source domain. Bare expression frames cannot issue transport.
- Add/Mul argument operations use the same canonical numeric envelope, expression
  writer and independent opcode/source matcher. No second arithmetic evaluator.
- Loop source census binds delegated frames to exact candidate Call/ordinal, original
  method Arc, plan/source-loan Rc and evaluation-time Homes. Same Call packet owns
  execution/final checks; Loop child-prefix count excludes these delegated frames.
- Bool equality borrows exact Bool/Bool Eq/Ne envelopes from the same
  operator owner. No Integer/truthiness view; existing Compare writer/final matcher reused.
- If state extends the existing IfForm observer across both arms and merge.
  Restore existing value/provenance snapshots before each arm, including implicit Else.
  Pre-If exact value/cache pairs bound merge publication; unchanged values retain origins.
  New joins deliberately gain no scalar proof until original BranchNormal + Phi closure.
  Prove all resolver ThenNormal/ElseNormal inputs, reuse the same IfForm condition/exits
  and indexed Phi, then validate via existing ExpressionReferenceRead; no new SSA writer.
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
- Current computed-actual/delegated coverage work: existing focused54 PASS;
  the same known Text Loop test remains failing. Test-name list9263 unchanged.
  Only the existing Copy-to-borrow test access changed; no cases or fixture calls added.
- Matching Bool quick CLI build PASS; unchanged-app smoke FAIL at Body11/IfThen0.
  If-state root build and existing focused48 PASS, test names9263 unchanged. The old
  If-driver oracle also failed on the preserved binary; its cached predecessor assertion
  now checks every live merge edge, including a single Normal arm. No cases were added.
  Next: original If-join scalar source and existing physical Phi owner;
  preserve all artifact/run/output obligations before app completion.
- Bool boundary: root build and focused32 PASS; test names9263 unchanged. Existing
  positive/negative operator matrices include the new Bool class because old classes
  cannot cover mixed Bool/Integer or Bool ordering refusal. No new test function/file.
- Fixed suite on committed computed-argument revision `3965b758ba`: 6PASS/5FAIL,
  actual EXE5 + empty marker1, no SKIP/timeout. Explicit outer1900s and build/run300s;
  same lifecycle runtime/C API. Later Bool/If changes are outside this measurement.
  Earlier120s timeout run is superseded for this snapshot, not counted as semantic red.
- Same If-state quick CLI + lifecycle runtime/C API: existing mimalloc smoke PASS,
  exact output/exit, no replay; allocator still stops at line29 before any artifact.
- Existing smoke: `bash tools/smokes/v2/profiles/integration/apps/allocator_stress_exe.sh`.
- Preserve expected accounting including rejects=4 and exit0. Retire only replaced exclusive
  edges after caller/validation closure. Untyped D0 stays ParkedSealed until accepted.
- Prior D2 selection history remains in this path's Git history; no new app card.
