# APP-json-stream-aggregator

Status: JSON selected; common untyped-storage target accepted. Date: 2026-10-20
Related: [RULES](../../../RULES.md), [current state](../CURRENT_STATE.toml),
[untyped-slot D0](mirbuilder-untyped-object-storage-d0-2026-09-25.md), [pipeline](../design/mirbuilder-final-pipeline-ssot.md).

## Product evidence

- Last official fixed suite: plain scalar If commit, 7PASS/4FAIL, actual EXE6 +
  empty marker1, no SKIP/timeout. Runner481.943s/wall483.997s. Matching quick opt0
  CLI36.96s, frozen quick runtime/private C FFI; build/run300s, no Cargo overlap.
- mimalloc PASS154.321s; allocator PASS178.179s; four typed EXEs PASS36–38s.
- Command: `tools/smokes/v2/run.sh --profile integration --owner-profile integration
  --suite real-apps-exe-boundary --timeout 1900`. Later entry diagnostics are outside this measurement.
- Matching scalar-If CLI passes find's original `if m == 0` read. Diagnostic
  CLI42.21s identifies owner26 `ingestLine`: four original Self field assignments
  (Body7/8 and Body9 Then/Else) are absent from its retained assignment census.
  JSON fails during MIR construction; no EXE/link/runtime or old-route retry.
- Common FieldWrite source now owns canonical field/name and qualified RHS access
  for the sole writer and packet validation. Receiver-specific LOCAL New/readiness
  checks stay sealed; missing non-formal origins refuse rather than gain permission.
  Self source/RHS and Dynamic leases remain unconnected. Matching CLI42.39s and
  unchanged JSON smoke confirm the same owner26 MIR stop; no app advancement.
- Worker review: reuse entry receiver/root source and all-writer value grounding;
  separate LOCAL-only New dominance from common packet validation. Unannotated
  declarations, initial `0`, or source inventory membership grant no I64 lease.
  Temporary probes removed; owner error retained. Existing field-write16 PASS
  (RHS2 included); catalog65 PASS/10 FAIL, same ten names/causes on prior binary.
- Other product reds remain untyped-field source activation, binary-trees source-not-i64,
  boxtorrent stored-child receiver. Do not reinterpret them as completion.

## Connected source and next execution

- Existing source Loop Call/If retains original condition, argument subtrees, both
  Normal arms, branch incoming producers and Phi keys through schedule and sole writer.
  Both final consumers independently validate the same source and cleanup correspondence.
- Text Return retains its latest initializer/rebind acquisition. Original exit Homes
  keep it for Fault cleanup; Normal excludes exactly that residence, with transfer
  committed only after all Normal cleanup succeeds. No ordinary result ABI is granted.
- Local Call shares scalar/Text result-source identity. Text Argument demand keeps
  original result/target, whole incoming census and ordered inputs; refusal is terminal.
- Bool String comparisons share the existing expression body writer and final
  projection, retaining original Core/read proofs and per-comparison acquisition,
  Normal End, Fault prefix and remaining End suffix. Logical RHS stays conditional.
  Source SUM now demands original If/Loop conditions, without new formal incoming.
  Threading removes only the same orphaned empty Jump, keeping other entries alive.
- The existing substring tuple now covers Loop Return/root cleanup and changed
  Return operand refusal; this independent FunctionExit boundary needs the minimal
  tuple/mutation extension. Its minimal scalar If addition protects a separate read purpose.
  Prior related364 PASS/36 old failures remain; no new test function/file/hook/checker.
- Plain scalar comparison selection requires the whole original leaf/read census.
  Calls/fields keep their existing owner; scalar siblings cannot preempt a Static Call.
- Core coverage borrows the SAME checked frame loans and exact resolver metadata;
  full source census/duplicates remain checked. Loop Return is not Header/After
  incoming. Shared lexical read and existing root cleanup emit once and validate
  independently. Source liveness is shared with production CFG finishing; Rust
  Integer entry and Static String Call ABI remain owed.
- C pair Return reuses `{handle, End token}`, per-Invoke String output slot,
  Normal projection and String End. Existing borrowed-ingress `--text-only` matrix
  PASS with quick runtime/private C FFI and `CC="clang -fuse-ld=lld"`.
  SafeMutex/SingleThread Return cover callee Fault, failed Add without result,
  exact caller End and result-kind/spent-result/Fault-omission refusals.
  These minimal case extensions protect the new affine Call boundary; no new test
  function/file, runtime function or checker. Physical execution is not source cutover.
- FiniteEntryRead now retains the original SUM/flow/read/plan across initializer
  and direct Return purposes. Signed-only root initializers keep their checked frame;
  terminal coverage retains its separate original read, never a binding-to-I64 guess.
  The same finite proof now serves genuine If/Loop conditions; the existing reference
  variant is purpose-neutral, with original source/alias census and Null refusal kept.
  The older opaque draft remains in Git stash pending full adoption/disposal.
- Entry projection decision from read-only review: retain Opaque declaration and
  tagged call ABI; a SAME signed-only source domain can authorize an Integer view.
  C must check kind1 before body effects and independently reject every non-I64
  incoming. Copy/Phi then use the checked I64 view; forwarding must publish tag1,
  not a tagged Copy whose kind variable no longer exists. Legacy mixed guards stay.
  C Static/Instance mutation matrix PASS. Rust requires module/published replay
  and rejection of root/Copy/Phi uses outside the sealed scope; no domain-only skip.

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
