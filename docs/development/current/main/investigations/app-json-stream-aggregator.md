# APP-json-stream-aggregator

Status: JSON selected; common untyped-storage target accepted. Date: 2026-10-20
Related: [RULES](../../../RULES.md), [current state](../CURRENT_STATE.toml),
[untyped-slot D0](mirbuilder-untyped-object-storage-d0-2026-09-25.md), [pipeline](../design/mirbuilder-final-pipeline-ssot.md).

## Product evidence

- Latest fixed suite: 7PASS/4FAIL, actual EXE6 + empty marker1, no SKIP/timeout,
  runner494.034s. mimalloc157.354s, allocator180.018s, typed38–39s.
- Command: `tools/smokes/v2/run.sh --profile integration --owner-profile integration
  --suite real-apps-exe-boundary --jobs 1 --timeout 1900 --format json`.
- CLI91s; retirement rebuild29s keeps .text/.rodata, only diagnostic line/build-ID delta.
  Default/private C FFI agree; final EXE yes/1/Result0/exit0; runtime unchanged.
- Matching JSON smoke now reaches `borrowed-entry/source-use-completion-pending`,
  owner26 JsonStreamAggregator.ingestLine, called at line152 inside ingest Loop.
  The outer `source-loop-header-after` abort only wraps that real callee failure.
  Full use census still retains Body5 statsFor result, Body6 stats.add and Body9
  ok condition. No EXE/link/runtime success is claimed for JSON.
- UserStats.add line87 now passes signed source/write lowering. Dynamic execution
  still needs transactional replace/old-End and complete canonical publication.
- Read-only review: statsFor returns a Map-held nonowning object on BOTH branches;
  new stats moves Home at set commit. Precommit Fault keeps caller ownership;
  postcommit old-End Fault keeps Map ownership. Same-slot invalidation must stay
  absent through last borrow use, including callee effects; Map live alone is insufficient.
  Literal-key I64 Map terminal and Fresh/ForwardFormal mixed cannot grant this origin.
- Other product reds: untyped-field source activation, binary-trees source-not-i64,
  boxtorrent stored-child receiver. These are incomplete, not waived baselines.

## Connected source and next execution

- FieldWrite RHS retains the SAME all-writer ClosedNumeric/read and signed SUM.
  Explicit ExactSlot/DynamicI64Replace mode preserves pre/postcommit Fault law
  through writer, finishing, both final scans and JSON. ExactSlot cannot represent
  Dynamic; source-less/new-mode or foreign final instruction grants no permission.
- Checked Self read copies reuse original ExactLexicalRead without a received Call.
  Source image borrows full finalized read/write correspondence, not a new index.
- NonLoop single local Bool conditions reuse original Bool classification and the
  existing exact lexical read/final producer owner. No Bool CurrentScalar family.
- Same archive traversal imports unique optional field ABI; V3 session preserves
  V2 prefix and transports opaque outcome dimensions. C checks absence/version,
  C/LLVM target value/End layouts. Dynamic physical gate remains CLOSED.
- Existing Invoke42/RHS2/field7/write4/construction2/JSON33/runtimeABI8 plus selected
  actual archive1PASS. Scalar15/1FAIL reproduces prior frozen binary.
  Existing V4 execution suite reaches its malformed-add negative, accepted by BOTH
  old/new FFI; earlier execution/Fault checks passed. This red remains owed.
  No new test fn/file/hook/checker/guard/token. ABI malformed record mutations
  extend one existing case to protect the independent target-storage boundary.
- Signed values keep SAME point SUM, physical producer, aliases/read/Copy and
  producer forwarding; scalar aliases stay generic. Prior entry21/Static args2PASS.
- TextResidence Return keeps original packet/acquire/install/hold and Home rank
  through both consumers; Normal skips only that Home, Fault reverses original
  cleanup. Duplicate/stale holds and borrowing stay refused; plain exits Emitted.
- Ordinary Text Call holds its result below the exact argument End suffix.
  Call Fault acquires nothing; argument-End Fault cancels remaining args, result,
  then outer cleanup. SSA/Rust/C retain lifecycle dependencies apart from arguments.
- Text Local Call retains whole input, affine publication, install/hold and both
  consumers. Prior EXEs !ab/ab passed; Zero evidence and old I64 census stay intact.
- Fresh Text Return uses original ReturnValue/shared expression qualification;
  literal Materialize and Core acquisition join child cleanup, hold, original Home
  cleanup and Fault cancellation. No fake Local or second result registry.
- Static Text input ordering borrows SAME completed contract, input finish and
  affine final packet; unknown catalog stays unknown, replaced issuer test-only.
- Prior source96/Core11/input6/packet3/dispatch2/received4/Invoke60/final4/port13PASS.
  Initializer79/2, terminal6/2, callback0/1, plan5/1, admission2/2 failures remain;
  callback/plan/admission reproduce frozen earlier binaries. No baseline waiver.
- Next: source-owned Dynamic End/replace through both consumers, and Map holder
  liveness/retained object view plus both statsFor Return origins. Retain precommit
  no mutation, postcommit no rollback, reverse old-End and exactly-once Home release.
- Existing Loop Call/If and Function Return retain original reads, producers,
  Phi keys and root cleanup through both consumers. Plain scalar If requires
  whole leaf/read census, so it cannot steal Static Call/field ownership.
- CFG scope/order/bindings, related364PASS/36FAIL and stashed opaque draft remain.

## Shared remaining contracts

- Common storage D0 supersedes ParkedSealed. Dynamic runtime Empty/null/I64/Bool/
  Text/object/Array slots and complete End descriptors exist; source activation,
  canonical Dynamic12 publication and both final consumers remain incomplete.
- Required: checked Integer view/replace with Fault-without-mutation, original
  signed-only SUM and source/slot/transfer, complete detached End in reverse field
  order to actual HomeRelease/Fault. Runtime support grants no source permission.
  Retain Empty/null distinction, plain-reclaim/Null refusal and legacy mixed guards.
- Map placement/stored borrow/Shared/carrier remain unadmitted. Transfer requires
  the SAME complete object End descriptor; current Map residence is numeric-only.
- Array holder/contents/End, exact empty-transfer census and canonical writer must
  stay joined. Runtime empty End grants no source ownership or physical transfer.
- Earlier SSA/handoff/Array teardown/synthetic callee and field/count/composition reds
  remain open. Full history is in Git; do not create a parallel inventory.

## Measurement limits and completion

- mimalloc suite PASS includes removing an obsolete generic JSON all-Invoke refusal
  precheck; canonical EXE already worked. This was a harness correction, not new capability.
  Existing publication tests reject four object-lifecycle generic profiles; they do not
  reproduce the same mimalloc closure or every Invoke subtype. Preserve that distinction.
- JSON smoke must satisfy every artifact, execution, output/exit and negative contract.
- Membership, isolated physical execution and local green are not product completion.
  Complete common storage and both final consumers; retire replaced exclusive edges
  with caller/validation closure, retaining shared owners and finite-goal obligations.
