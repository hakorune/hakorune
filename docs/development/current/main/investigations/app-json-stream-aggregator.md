# APP-json-stream-aggregator

Status: JSON selected; common untyped-storage target accepted. Date: 2026-10-20
Related: [RULES](../../../RULES.md), [current state](../CURRENT_STATE.toml),
[untyped-slot D0](mirbuilder-untyped-object-storage-d0-2026-09-25.md), [pipeline](../design/mirbuilder-final-pipeline-ssot.md).

## Product evidence

- Last official fixed suite: Loop Function Return commit, 7PASS/4FAIL, actual EXE6 +
  empty marker1, no SKIP/timeout. Runner448.163s/wall450.249s. Matching quick opt0
  CLI59.01s, frozen quick runtime/private C FFI; build/run300s, no Cargo overlap.
- mimalloc PASS140.112s; allocator PASS159.797s; four typed EXEs PASS36–38s.
  Totals include compile/link/run; individual phases were not timed.
- Command: `tools/smokes/v2/run.sh --profile integration --owner-profile integration
  --suite real-apps-exe-boundary --timeout 1900`. Scalar If WIP is outside this measurement.
- Matching scalar-If CLI36.96s passes find's original `if m == 0` read (line11).
  JSON still fails during MIR construction: `phi_lifecycle/txn_abort`,
  source-loop-header-after, `borrowed-entry/source-use-completion-pending`.
  The next step is to identify the demanded entry owner and retain its SAME source
  use/completion through ingress. No EXE/link/runtime; no old-route retry.
- Scalar If retains the whole original scalar read census through both final
  consumers. Related364 PASS; 36 previous-case failures remain. A matching fixed
  suite is pending; the last completed suite above is the Return generation.
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
  Current related364 PASS; 36 failures reproduce prior cases/causes (root12, Loop21,
  Static2, source SUM1). New test function/file/hook/checker count remains zero.
- Plain scalar comparison selection requires the whole original leaf/read census.
  Calls/fields keep their existing owner; scalar siblings cannot preempt a Static Call.
- Core coverage borrows the SAME checked frame loans and exact resolver metadata;
  full source census/duplicates remain checked. Loop Return is not Header/After
  incoming. Shared lexical read and existing root cleanup emit once and validate
  independently. The existing document fixture now lends source liveness to CFG
  finishing exactly as production does; focused build58.42s. Rust Integer entry
  and Static String Call ABI remain owed.
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
  Minimal existing tuple extension protects independent condition/Return read purposes.
  The older opaque draft remains in Git stash pending full adoption/disposal.
- Entry projection decision from read-only review: retain Opaque declaration and
  tagged call ABI; a SAME signed-only source domain can authorize an Integer view.
  C must check kind1 before body effects and independently reject every non-I64
  incoming. Copy/Phi then use the checked I64 view; forwarding must publish tag1,
  not a tagged Copy whose kind variable no longer exists. Legacy mixed guards stay.
  C Static/Instance mutation matrix PASS. Rust requires module/published replay
  and rejection of root/Copy/Phi uses outside the sealed scope; no domain-only skip.
- Existing Loop caller tuple preserves its read into an unused unannotated formal.

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
