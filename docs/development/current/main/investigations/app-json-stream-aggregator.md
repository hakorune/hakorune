# APP-json-stream-aggregator

Status: JSON selected; common untyped-storage target accepted. Date: 2026-10-20
Related: [RULES](../../../RULES.md), [current state](../CURRENT_STATE.toml),
[untyped-slot D0](mirbuilder-untyped-object-storage-d0-2026-09-25.md), [pipeline](../design/mirbuilder-final-pipeline-ssot.md).

## Product evidence

- Last official fixed suite: source Loop If/SSA commit, 7PASS/4FAIL, actual EXE6 +
  empty marker1, no SKIP/timeout, duration452.253s. Frozen quick opt0 CLI34.70s,
  quick runtime/private C FFI; build/run300s, no Cargo overlap.
- mimalloc PASS139.093s; allocator PASS160.642s; four typed EXEs PASS37–39s each.
  Earlier allocator standalone267.063→160.247s after same-operation input validation sharing.
  Those totals include compile/link/run; individual phases were not timed.
- Command: `tools/smokes/v2/run.sh --profile integration --owner-profile integration
  --suite real-apps-exe-boundary --timeout 1900`. Later source/Text changes are outside this measurement.
- Current matching quick CLI passes the Main.main argument frame for
  `agg.ingest(Main.sampleStream())`, then fails during MIR construction:
  `phi_lifecycle/txn_abort`, source-loop-header-after, `static-input/cohort-unfinished`.
  Matching CLI35.23s identifies ingest line146 `JsonLine.find(stream, "\n", start)`
  (callee JsonLine.find/3). No EXE/link/runtime.
- Caller Loop `JsonLine.find` argument2 (`start`) now retains its checked scalar read
  in the original argument frame; matching smoke no longer reports ingest's uncovered read.
  Callee JsonLine.find still lacks its Header basis; the same input-cohort stop remains.
- Other product reds remain untyped-field source activation, binary-trees source-not-i64,
  boxtorrent stored-child receiver. Do not reinterpret them as completion.

## Connected source and next execution

- Existing source Loop Call/If retains original condition, argument subtrees, both
  Normal arms, branch incoming producers and Phi keys through schedule and sole writer.
  Both final consumers independently validate the same source and cleanup correspondence.
- Text Return retains its latest initializer/rebind acquisition. Original exit Homes
  keep it for Fault cleanup; Normal excludes exactly that residence, with transfer
  committed only after all Normal cleanup succeeds. No ordinary result ABI is granted.
- Local Call observations now share one result-source enum: scalar loan or SAME
  Static Text source Rc. Existing numeric getter lends only scalar, preserving its gates.
- Text source demand validates original result/target evidence, whole incoming census,
  complete callee formal contract and SAME source snapshot/ordered arguments. A selected
  refusal is terminal. Qualification admits Text only in the existing Argument purpose.
- Existing focused249 PASS on the normalized source generation; test function count unchanged.
  Minimal existing tuple extension protects ordinary-call ownership versus Core results.
  Prior Text Return focused33 PASS; qualified-return2/Raw Loop residual1 reproduce pre-Text.
- Next: finish Static inputs and connect the SAME source/Completion/return residence to
  existing Static packet selection, publication handoff, Invoke and final result consumers.
  Retain selected-Static-Loop I64 restrictions; source membership is not execution permission.
- C pair Return reuses `{handle, End token}`, per-Invoke String output slot,
  Normal projection and String End. Existing borrowed-ingress `--text-only` matrix
  PASS with quick runtime/private C FFI and `CC="clang -fuse-ld=lld"`.
  SafeMutex/SingleThread Return cover callee Fault, failed Add without result,
  exact caller End and result-kind/spent-result/Fault-omission refusals.
  Runtime backend matches each explicit wire profile. The same test now honors CC.
  These minimal case extensions protect the new affine Call boundary; no new test
  function/file, runtime function or checker. Physical execution is not source cutover.
- Opaque scalar argument/Return draft remains in Git stash; it requires source-issued
  finite-entry reads and original Return use closure, never a binding-to-I64 guess.
- Entry projection decision from read-only review: retain Opaque declaration and
  tagged call ABI; a SAME signed-only source domain can authorize an Integer view.
  C must check kind1 before body effects and independently reject every non-I64
  incoming. Copy/Phi then use the checked I64 view; forwarding must publish tag1,
  not a tagged Copy whose kind variable no longer exists. Legacy mixed guards stay.
  C Static/Instance Copy/Phi, Normal/Fault and domain/producer mutation matrix PASS.
  Minimal existing matrix extensions protect this new input boundary; no new test function.
  Rust activation still requires module/published sequence replay plus rejection of
  projected root/Copy/Phi uses outside the sealed scope; a domain-only skip is forbidden.
- Existing compiler tuple extension checks the separate caller boundary: a Loop
  scalar enters an unannotated, unused callee formal without losing its source read.
  It adds no test function; declared-formal/Return coverage remains in its original tuple.
  Current focused37 PASS. Selected Static forwarded-candidate drift test fails during
  package setup identically in the pre-change saved binary; refusal is not weakened.

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
