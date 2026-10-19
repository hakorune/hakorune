# APP-allocator-stress

Status: selected; first Loop source coverage passed; Bool call condition prefix remains open.
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
  They stop before artifacts at `array-home/append-prefix`, `PrefixNotCovered` Body11:
  main.hako:28, `heap.release(small_handles.get(1)) == false`. EXE remains ungenerated.
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
- Matching quick CLI build PASS; selected unchanged-app smoke FAIL at Body11 before
  artifacts. Next: connect the existing Bool-condition/ArrayGet call source owners;
  preserve all artifact/run/output obligations before app completion.
- Pre-selector WIP CLI whole-suite attempt: 3PASS/8FAIL, including three 120s timeouts
  and five semantic failures. Same-binary mimalloc retake PASS with 300s build allowance;
  typed method/birth retakes still hit their inner120s limits. These are unresolved
  timing results, not final-HEAD acceptance or evidence that the old6PASS remain green.
- Existing smoke: `bash tools/smokes/v2/profiles/integration/apps/allocator_stress_exe.sh`.
- Preserve expected accounting including rejects=4 and exit0. Retire only replaced exclusive
  edges after caller/validation closure. Untyped D0 stays ParkedSealed until accepted.
- Prior D2 selection history remains in this path's Git history; no new app card.
