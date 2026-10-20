# APP-json-stream-aggregator

Status: JSON selected; common untyped-storage target accepted. Date: 2026-10-20
Related: [RULES](../../../RULES.md), [current state](../CURRENT_STATE.toml),
[untyped-slot D0](mirbuilder-untyped-object-storage-d0-2026-09-25.md), [pipeline](../design/mirbuilder-final-pipeline-ssot.md).

## Product evidence

- Last official fixed suite: this Self field-write change (base `11dd3230cb`),
  7PASS/4FAIL, actual EXE6 + empty marker1, no SKIP/timeout. Runner484.779s.
  Matching quick opt0 CLI1m28s, frozen quick runtime/private C FFI; build/run300s,
  no Cargo overlap. mimalloc PASS152.639s; allocator PASS177.583s; typed EXEs37–40s.
- Command: `tools/smokes/v2/run.sh --profile integration --owner-profile integration
  --suite real-apps-exe-boundary --timeout 1900 --format json`.
- Self writes now retain the exact entry receiver, whole RHS and all-writer source.
  Both final scans validate the same FieldSet packet, producers, pins and Normal/Fault
  paths; LOCAL keeps its New dominance, Self checks its actual signature lane.
- Signed RHS references borrow the existing SUM for the SAME uncaptured original
  flow/read point, then replay the existing physical producer verifier. Class::I64
  and MIR Integer alone grant no signed permission. Wide unsigned stays refused.
- Fresh receiver pins/Fault-frame prefixes no longer shift the observed RHS cursor;
  only the sole writer's exact new rows are excluded, never cached/RHS producers.
- Matching quick opt0 CLI1m28s: typed Counter `me.value = me.value + 1` now generates
  an EXE through pure-first/no replay and runs with Result8/exit8. This is a focused
  connection proof, not JSON acceptance or unannotated-storage activation.
- Unchanged JSON still fails MIR construction at owner26 `ingestLine` source-use
  completion; no EXE/link/runtime or old-route retry. Current missing census needs
  re-identification after the Self connection; do not reuse the old four-write claim.
- Existing field-write16, signed SUM1 and numeric capability1 PASS. Catalog65 PASS/
  10 FAIL: same names and normalized causes on prior binary. No new test cases/files.
- Worker review approved the shared receiver/signed source boundary. Alias point
  demands retain original anchors and the existing uncaptured-binding gate; signed
  reads are collected before the transactional plan borrow. Dynamic remains closed.
- Other product reds remain untyped-field source activation, binary-trees source-not-i64,
  boxtorrent stored-child receiver. Do not reinterpret them as completion.

## Connected source and next execution

- Self field writes retain their original entry receiver, declaration, complete
  RHS and prior Homes in the shared FieldWrite frame. Unannotated slots require
  the existing all-writer ClosedNumeric proof and keep their absent annotation.
- Source-use completion counts only the SAME qualified frame/RHS and original
  assignment/read census. Source membership alone never closes missing effects.
- Both final scans share FieldSet coordinates, RHS/pin producers and Normal/Fault
  packet validation. LOCAL still owes New/Normal dominance; Self owes its actual
  signature receiver lane/carrier and exact Me read, without LOCAL-only Home closure.
- Next source diagnosis: establish the current missing assignment/read sites. Worker
  identified `total_bytes + bytes` as a candidate: ResultExpression Integer basis
  does not yet borrow arbitrary original-point SUM for `bytes = JsonLine.intField`.
  Reuse that basis/solver; establish the actual refusal before treating it as the stop.
- Dynamic physical execution remains explicitly refused. Required next: checked
  Dynamic Integer view/replace preserving Fault-without-mutation, canonical layout
  publication, detached End/destruction and both final consumers. Runtime support
  alone or treating an unannotated slot as I64 layout is insufficient.
- Read-only worker approved the common receiver split. Structural Jumps use the
  existing checked contraction projection; effects remain mandatory.
- Existing source Loop Call/If and Function Return retain original reads, producers,
  Phi keys and root cleanup through both consumers. Plain scalar If requires the
  whole leaf/read census so it cannot steal Static Call/field ownership.
- Text Return retains its latest residence: Normal transfers only after cleanup;
  Fault retains the original Home. Ordinary Static String Call ABI remains owed.
- C borrowed-ingress Text/Integer mutation matrices passed; their physical ABI
  evidence does not activate Rust source entry or publication. Rust Integer view
  still requires SAME signed-only SUM, module/published replay and rejection of
  out-of-scope root/Copy/Phi uses. Keep legacy mixed guards and Null refusal.
- Original source liveness feeds CFG finishing. The existing substring tuple
  protects Loop Return, changed Return refusal and plain scalar If read purposes;
  prior related364 PASS/36 old failures remain. Existing opaque draft stays stashed
  until its intentional adoption/disposal. Detailed completed history is in Git.

## Shared remaining contracts

- [Common storage D0](mirbuilder-untyped-object-storage-d0-2026-09-25.md) supersedes
  ParkedSealed. Runtime Dynamic Empty/null/I64/Bool/Text/object/Array slots and complete
  End descriptors exist; source activation and both final consumers remain incomplete.
- Keep Empty distinct from null, exact source/slot/transfer and whole detached End.
  Map placement, stored borrow/Shared and carrier forwarding remain unadmitted.
- Array holder/contents/End, exact empty-transfer census and canonical writer must stay
  joined. Runtime empty End alone grants no source ownership or physical transfer.
- Earlier SSA/handoff/Array teardown/synthetic callee and field/count/composition reds
  remain open. Full implementation history is in Git; no new parallel inventory.

## Measurement limits and completion

- mimalloc suite PASS includes removal of an obsolete generic JSON all-Invoke refusal
  precheck; canonical EXE already worked. This was a harness correction, not new capability.
  Existing publication tests reject four object-lifecycle generic profiles; they do not
  reproduce the same mimalloc closure or every Invoke subtype. Preserve that distinction.
- Existing JSON smoke must satisfy every artifact, execution, output/exit and negative contract.
- Source membership, isolated physical execution and local green are not product completion.
  Complete common storage and both final consumers; retire replaced exclusive edges only
  with caller/validation closure, retaining shared owners and finite-goal obligations.
