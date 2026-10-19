# APP-json-stream-aggregator

Status: common untyped-storage target accepted; implementation first; JSON remains selected.
Date: 2026-10-19
Scope: unchanged JSON app, existing Loop/Text owners, canonical physical EXE.
Related: [RULES](../../../RULES.md), [current state](../CURRENT_STATE.toml),
[untyped-slot D0](mirbuilder-untyped-object-storage-d0-2026-09-25.md),
[final pipeline](../design/mirbuilder-final-pipeline-ssot.md).

## Current measurement

- Fixed `real-apps-exe-boundary`: 7PASS/4FAIL, actual EXE6 + empty marker1;
  mimalloc and allocator PASS, no SKIP/timeout, duration783.522s.
  Measured literal-source admission `23ff510a8b` with its matching quick CLI,
  explicitly selected lifecycle runtime and production Bool-equality C FFI.
- Command: `tools/smokes/v2/run.sh --profile integration --owner-profile integration
  --suite real-apps-exe-boundary --timeout 1900`, build/run limits300s.
- JSON first stop: `[freeze:contract][source-loop/v2/source Loop has an unavailable path]`.
  MIR compilation fails before artifact generation; no link or runtime evidence.
  Original owner is `ingest`, Loop Body(2), source line145; first uncovered statement
  is line151 `me.ingestLine(stream.substring(start, end))`. No artifact is generated.
  On the current source-admission tree, incoming and Mul gates are cleared; debug
  reports `source expression has no checked execution root`, path=None, in this Loop.
  The scanner also lacks a Call action for line146 `local end = JsonLine.find(...)`;
  retain its real I64 result as well as the later discarded Bool result.
- Other reds remain explicit: untyped-field phase84 inference panic,
  binary-trees borrowed-result/source-not-i64, boxtorrent stored-child/receiver-unavailable.

## Current priority and measurement interpretation

- [Common storage D0](mirbuilder-untyped-object-storage-d0-2026-09-25.md) is now the
  accepted target for three real apps plus the untyped sentinel. Tagged dynamic
  preserves existing EBNF; EmptyField read-Fault, source residence, replacement,
  reads and cleanup are now specified. Implementation is open; production coverage is not.
- `mimalloc-lite` suite PASS includes a harness correction: an obsolete generic JSON
  all-Invoke refusal precheck was removed; production compiler/app behavior was unchanged.
  The canonical EXE already worked. Route/toolchain/artifact/no-replay/output/exit checks remain.
- Existing publication tests cover generic profile rejection of four object-lifecycle
  programs and positive Static I64 Invoke transport. They do not reproduce the same
  mimalloc import closure or assert every unsupported Invoke subtype. Do not describe
  this as a new runtime capability or an identical negative test transfer.
- Own unverified Loop Call source edits are saved in Git stash under
  `JSON Loop Call source connection pending common storage decision`; not built/tested.

## Next source connection

- Source declaration now issues Dynamic layout for bare strong slots; no writer/MIR
  inference. Existing complete fixed-Array specialization remains a bounded profile.
- Shared storage excludes Dynamic from numeric lane. Destruction/exact-slot/final
  consumer fences reject unsupported execution; layout availability grants no ABI.
- Existing vocabulary2, plan inference19 (including layout4), declaration6 and
  referenced physical-layout rejection1 PASS. Existing cases add Dynamic layout/
  noninteger assertions for that independent boundary; no new test functions/files.
- Runtime TypedSlotValue/TypedSlot/TypedSlotObject are now move-only. Existing
  residence checks observe payload/outcome without cloning a storage owner.
  Kernel lifecycle-core `typed_object`35 PASS (build105s, execution<0.01s);
  checked C operations pass on SafeMutex/SingleThread, and indexed profile checks
  retain rejection on PinnedArena/DirectSlot. No runtime representation changed.
- Next extend the SAME TypedSlot value/residence and atomic store owner with known
  EmptyField, checked tagged reads, replacement/detached-old cleanup and exact End.
  Existing Text lease pairs and source-selected object/Array/Map End owners retain
  authority; no generic handle clone, tag-to-destructor switch or alternate heap.
- Both final consumers must recheck the same slot/residence/commit/Fault relation
  before any C/runtime profile is admitted. The unchanged sentinel is the common
  field-boundary acceptance, not a replacement for the selected JSON smoke.
- JSON Loop Call source edits stay stashed until its physical half is implemented.
  Connect BOTH line146 Local I64 Call and line151 discarded Bool Call through the
  existing source tree/schedule and SAME Static/Instance child packet. Its substring
  argument stays inside that packet; retain original Homes and argument cleanup.
- Existing Loop normalization uses SAME child groups; source_loop13PASS/1FAIL matches
  the prior binary, source walk1/Array lifetime5 PASS; count_at remains open.
- Literal I64/Bool Return census and SAME Core Text argument frames are connected.
  Static22/Instance4/pending7 PASS; composition9PASS/2FAIL/result15PASS/2FAIL and
  Unit diagnostic failure match prior evidence. Root test functions remain9263.
- JSON's later UserStats/aggregator slots and Map remain implementation dependencies.
  Accepted semantics do not claim physical storage/read/write/cleanup coverage.

## Retained verification obligations

- Allocator and mimalloc physical EXEs pass unchanged. Allocator's Bool Eq/Ne C admission
  retains mixed Bool/I64 and Bool-order refusals; existing matrix tail is still blocked
  by the pre-existing tagged-add negative. Earlier SSA/handoff/Array teardown/synthetic
  callee reds remain open; implementation details are retained in this file's Git history.
- No app workaround, silent retry, new checker/test file or runtime adapter was introduced.

## Completion obligations

- Existing JSON smoke must satisfy every artifact/run/output/negative contract unchanged.
- Common untyped-slot D0 is accepted; source/layout/runtime/final-consumer implementation remains.
- Retire replaced exclusive edges when caller/validation closure holds; retain shared owners.
- Fixed acceptance, final pipeline switching/retirement and the full goal remain incomplete.
